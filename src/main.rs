extern crate coreboot_collector;
extern crate libc;
extern crate sysfs_class;
extern crate system76_power;

use coreboot_collector::gpio::GpioCommunity;
use std::{io, process};
use sysfs_class::{PciDevice, SysClass};
use system76_power::sideband::Sideband;

fn gpio_communities() -> io::Result<&'static [GpioCommunity<'static>]> {
    let mut devs = PciDevice::all()?;
    devs.sort_by(|a, b| {
        a.id().cmp(&b.id())
    });
    for dev in devs {
        if dev.class()? == 0x00060100 && dev.vendor()? == 0x8086 {
            println!(
                "PCI Device: {}: Class {:>08X}, Vendor {:>04X}, Device {:>04X}, Revision {}",
                dev.id(),
                dev.class()?,
                dev.vendor()?,
                dev.device()?,
                dev.revision()?
            );

            match dev.device()? & 0xFF80 {
                // 100 Series PCH (Sky Lake)
                0xA100 => return Ok(GpioCommunity::skylake()),
                // 200 Series PCH (Kaby Lake, compatible with Sky Lake)
                0xA280 => return Ok(GpioCommunity::skylake()),
                // Cannon Lake PCH
                0xA300 => return Ok(GpioCommunity::cannonlake()),
                // Unknown PCH
                unknown => eprintln!("Unknown PCH: {:#>04X}", unknown),
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Failed to find compatible Intel PCH"
    ))
}

fn inner() -> io::Result<()> {
    let communities = gpio_communities()?;

    let sideband = unsafe {
        Sideband::new(0xFD00_0000).map_err(|err| {
            io::Error::new(
                io::ErrorKind::Other,
                err
            )
        })?
    };

    for community in communities.iter() {
        let mut pad = 0;
        for group in community.groups.iter() {
            for i in 0..group.count {
                let data = unsafe { sideband.gpio(community.id, pad) };
                let low = data as u32;
                let high = (data >> 32) as u32;
                println!("{}{} = {:#>08x} {:#>08x}", group.name, i, low, high);
                pad += 1;
            }
        }
    }


    Ok(())
}

fn main() {
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("coreboot_collector: must be run as root");
        process::exit(1);
    }

    if let Err(err) = inner() {
        eprintln!("coreboot-collector: {:?}", err);
        process::exit(1);
    }
}
