use libc::{
    c_int, c_void, close, mmap, open, MAP_FAILED, MAP_SHARED, O_RDWR, PROT_READ, PROT_WRITE,
};

use std::{ffi::CString, io, ptr};

// P2SB private registers.
const P2SB_PORTID_SHIFT: u32 = 16;

// GPIO sideband registers.
const REG_PCH_GPIO_PADBAR: u32 = 0xc;

#[derive(Debug, Error)]
pub enum SidebandError {
    #[error(display = "failed to open /dev/mem: {}", _0)]
    DevMemOpen(io::Error),
    #[error(display = "failed to map sideband memory: {}", _0)]
    MapFailed(io::Error),
}

pub struct Sideband {
    pub addr: u64,
}

impl Sideband {
    pub unsafe fn new(sbreg_phys: usize) -> Result<Sideband, SidebandError> {
        let mem_str = CString::new("/dev/mem").unwrap();
        let memfd: c_int = open(mem_str.as_ptr(), O_RDWR);
        if memfd == -1 {
            return Err(SidebandError::DevMemOpen(io::Error::last_os_error()));
        }

        let sbreg_virt = mmap(
            sbreg_phys as *mut c_void,
            1 << 24,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            memfd,
            sbreg_phys as i64,
        );

        close(memfd);

        if sbreg_virt == MAP_FAILED {
            return Err(SidebandError::MapFailed(io::Error::last_os_error()));
        }

        Ok(Sideband { addr: sbreg_virt as u64 })
    }

    pub unsafe fn ptr(&self, port: u8, reg: u32) -> Option<*mut u32> {
        let offset = (u64::from(port) << P2SB_PORTID_SHIFT) + u64::from(reg);
        if offset < 1 << 24 {
            let addr = self.addr + offset;
            Some(addr as *mut u32)
        } else {
            None
        }
    }

    pub unsafe fn read(&self, port: u8, reg: u32) -> u32 {
        if let Some(ptr) = self.ptr(port, reg) {
            ptr::read_volatile(ptr)
        } else {
            0
        }
    }

    pub unsafe fn write(&self, port: u8, reg: u32, value: u32) {
        if let Some(ptr) = self.ptr(port, reg) {
            ptr::write_volatile(ptr, value)
        }
    }

    pub unsafe fn gpio_ptr(&self, port: u8, pad: u8) -> Option<*mut u32> {
        let padbar: u32 = self.read(port, REG_PCH_GPIO_PADBAR);
        self.ptr(port, padbar + u32::from(pad) * 8)
    }

    pub unsafe fn gpio(&self, port: u8, pad: u8) -> u64 {
        if let Some(ptr) = self.gpio_ptr(port, pad) {
            let dw1: u32 = ptr::read_volatile(ptr.offset(1));
            let dw0: u32 = ptr::read_volatile(ptr);
            u64::from(dw0) | u64::from(dw1) << 32
        } else {
            0
        }
    }

    pub unsafe fn set_gpio(&self, port: u8, pad: u8, value: u64) {
        if let Some(ptr) = self.gpio_ptr(port, pad) {
            ptr::write_volatile(ptr.add(1), (value >> 32) as u32);
            ptr::write_volatile(ptr, value as u32);
        }
    }
}
