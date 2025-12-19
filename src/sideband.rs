use libc::{
    c_int, c_void, close, mmap, munmap, open, MAP_FAILED, MAP_SHARED, O_RDWR, PROT_READ, PROT_WRITE,
};

use std::{ffi::CString, io, ptr};
use thiserror::Error;

// P2SB private registers.
const P2SB_PORTID_SHIFT: u32 = 16;

// GPIO sideband registers.
const REG_PCH_GPIO_PADBAR: u32 = 0xc;

// Size of sideband mapping (16 MB).
const SBREG_SIZE: usize = 1 << 24;

#[derive(Debug, Error)]
pub enum SidebandError {
    #[error("failed to open /dev/mem: {0}")]
    DevMemOpen(io::Error),
    #[error(
        "failed to map sideband memory at 0x{address:X}: {source}. \
        This may be caused by kernel security restrictions (CONFIG_STRICT_DEVMEM or \
        CONFIG_IO_STRICT_DEVMEM). Try booting with 'iomem=relaxed' kernel parameter, \
        or disable CONFIG_STRICT_DEVMEM in your kernel config."
    )]
    MapFailed {
        address: usize,
        source: io::Error,
    },
}

pub struct Sideband {
    pub addr: u64,
    size: usize,
}

impl Sideband {
    /// Creates a new Sideband instance by mapping the sideband registers.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It maps physical memory via /dev/mem which requires root privileges
    /// - The caller must ensure `sbreg_phys` is a valid sideband base address
    ///   for the current chipset (e.g., 0xFD000000 for older PCH, 0xE0000000 for newer)
    /// - Incorrect addresses may cause system instability or crashes
    /// - The mapped memory must not be accessed after the Sideband is dropped
    pub unsafe fn new(sbreg_phys: usize) -> Result<Sideband, SidebandError> {
        let mem_str = CString::new("/dev/mem").expect("/dev/mem path is valid");
        let memfd: c_int = open(mem_str.as_ptr(), O_RDWR);
        if memfd == -1 {
            return Err(SidebandError::DevMemOpen(io::Error::last_os_error()));
        }

        let sbreg_virt = mmap(
            ptr::null_mut(),
            SBREG_SIZE,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            memfd,
            sbreg_phys as i64,
        );

        close(memfd);

        if sbreg_virt == MAP_FAILED {
            return Err(SidebandError::MapFailed {
                address: sbreg_phys,
                source: io::Error::last_os_error(),
            });
        }

        Ok(Sideband {
            addr: sbreg_virt as u64,
            size: SBREG_SIZE,
        })
    }

    /// Returns a pointer to the sideband register at the given port and register offset.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - The returned pointer is only valid while the Sideband instance exists
    /// - Reading/writing through this pointer accesses hardware registers directly
    /// - The caller must ensure the port and register are valid for the chipset
    pub unsafe fn ptr(&self, port: u8, reg: u32) -> Option<*mut u32> {
        let offset = (u64::from(port) << P2SB_PORTID_SHIFT) + u64::from(reg);
        if offset < SBREG_SIZE as u64 {
            let addr = self.addr + offset;
            Some(addr as *mut u32)
        } else {
            None
        }
    }

    /// Reads a 32-bit value from the sideband register at the given port and offset.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It performs a volatile read from a hardware register
    /// - The caller must ensure the port and register are valid for the chipset
    /// - Returns 0 if the offset is out of bounds (silent failure)
    pub unsafe fn read(&self, port: u8, reg: u32) -> u32 {
        if let Some(ptr) = self.ptr(port, reg) {
            ptr::read_volatile(ptr)
        } else {
            0
        }
    }

    /// Writes a 32-bit value to the sideband register at the given port and offset.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It performs a volatile write to a hardware register
    /// - Writing incorrect values can cause system instability or hardware damage
    /// - The caller must ensure the port and register are valid for the chipset
    pub unsafe fn write(&self, port: u8, reg: u32, value: u32) {
        if let Some(ptr) = self.ptr(port, reg) {
            ptr::write_volatile(ptr, value)
        }
    }

    /// Returns a pointer to the GPIO pad register at the given port and pad number.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It reads the PADBAR register to calculate the pad address
    /// - The returned pointer is only valid while the Sideband instance exists
    /// - The caller must ensure the port and pad are valid for the chipset
    pub unsafe fn gpio_ptr(&self, port: u8, pad: u8) -> Option<*mut u32> {
        let padbar: u32 = self.read(port, REG_PCH_GPIO_PADBAR);
        self.ptr(port, padbar + u32::from(pad) * 8)
    }

    /// Reads the GPIO pad configuration (DW0 and DW1) as a 64-bit value.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It performs volatile reads from hardware registers
    /// - The caller must ensure the port and pad are valid for the chipset
    /// - Returns 0 if the pad offset is out of bounds (silent failure)
    pub unsafe fn gpio(&self, port: u8, pad: u8) -> u64 {
        if let Some(ptr) = self.gpio_ptr(port, pad) {
            let dw1: u32 = ptr::read_volatile(ptr.offset(1));
            let dw0: u32 = ptr::read_volatile(ptr);
            u64::from(dw0) | u64::from(dw1) << 32
        } else {
            0
        }
    }

    /// Writes the GPIO pad configuration (DW0 and DW1) from a 64-bit value.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It performs volatile writes to hardware registers
    /// - Writing incorrect values can change GPIO behavior and cause hardware issues
    /// - The caller must ensure the port and pad are valid for the chipset
    pub unsafe fn set_gpio(&self, port: u8, pad: u8, value: u64) {
        if let Some(ptr) = self.gpio_ptr(port, pad) {
            ptr::write_volatile(ptr.add(1), (value >> 32) as u32);
            ptr::write_volatile(ptr, value as u32);
        }
    }
}

impl Drop for Sideband {
    fn drop(&mut self) {
        // SAFETY: self.addr was returned by mmap in new(), and self.size is SBREG_SIZE
        unsafe {
            munmap(self.addr as *mut c_void, self.size);
        }
    }
}
