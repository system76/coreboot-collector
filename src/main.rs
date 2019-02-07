extern crate coreboot_collector;
extern crate libc;
extern crate sysfs_class;
extern crate system76_power;

use coreboot_collector::gpio::GpioCommunity;
use std::{fs, io, path, process};
use sysfs_class::{PciDevice, SysClass};
use system76_power::sideband::Sideband;

fn pci() -> io::Result<()> {
    let mut devs = PciDevice::all()?;
    devs.sort_by(|a, b| {
        a.id().cmp(&b.id())
    });

    for dev in devs {
        println!(
            "PCI Device: {}: Class {:>08X}, Vendor {:>04X}, Device {:>04X}, Revision {}",
            dev.id(),
            dev.class()?,
            dev.vendor()?,
            dev.device()?,
            dev.revision()?
        );
    }

    Ok(())
}

fn gpio_communities() -> io::Result<&'static [GpioCommunity<'static>]> {
    let mut devs = PciDevice::all()?;
    devs.sort_by(|a, b| {
        a.id().cmp(&b.id())
    });
    for dev in devs {
        if dev.class()? == 0x00060100 && dev.vendor()? == 0x8086 {
            match dev.device()? & 0xFF80 {
                // 100 Series PCH (Sky Lake)
                0xA100 => {
                    println!("Sky Lake PCH");
                    return Ok(GpioCommunity::skylake());
                },
                // 200 Series PCH (Kaby Lake, compatible with Sky Lake)
                0xA280 => {
                    println!("Kaby Lake PCH");
                    return Ok(GpioCommunity::skylake());
                },
                // Cannon Lake PCH
                0xA300 => {
                    println!("Cannon Lake PCH");
                    return Ok(GpioCommunity::cannonlake());
                },
                // Unknown PCH
                unknown => {
                    eprintln!("Unknown PCH: {:#>04X}", unknown);
                },
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Failed to find compatible Intel PCH"
    ))
}

fn gpio() -> io::Result<()> {
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

fn read_trimmed<P: AsRef<path::Path>>(p: P) -> io::Result<String> {
    Ok(
        fs::read_to_string(p)?
            .trim()
            .to_string()
    )
}

fn hdaudio() -> io::Result<()> {
    for entry_res in fs::read_dir("/sys/bus/hdaudio/devices")? {
        let entry = entry_res?;
        let path = entry.path();
        println!("{}", path.display());
        println!("  vendor_name: {}", read_trimmed(path.join("vendor_name"))?);
        println!("  chip_name: {}", read_trimmed(path.join("chip_name"))?);
        println!("  vendor_id: {}", read_trimmed(path.join("vendor_id"))?);
        println!("  subsystem_id: {}", read_trimmed(path.join("subsystem_id"))?);
        println!("  revision_id: {}", read_trimmed(path.join("revision_id"))?);

        for entry_res in fs::read_dir(path.join("widgets"))? {
            let entry = entry_res?;
            let path = entry.path();
            if let Ok(pin_cfg) = read_trimmed(path.join("pin_cfg")) {
                if ! pin_cfg.is_empty() {
                    println!("  {}: {}", path.display(), pin_cfg);
                }
            }
        }
    }

    Ok(())
}

fn inner() -> io::Result<()> {
    println!("## PCI ##");
    pci()?;

    println!("## GPIO ##");
    gpio()?;

    println!("## HDAUDIO ##");
    hdaudio()?;

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
