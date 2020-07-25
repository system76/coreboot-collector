extern crate coreboot_collector;
extern crate libc;
extern crate sysfs_class;
extern crate system76_power;

use coreboot_collector::gpio::GpioCommunity;
use std::{fs, io, path, process};
use std::io::{Read, Seek};
use sysfs_class::{PciDevice, SysClass};
use system76_power::sideband::Sideband;

fn pci() -> io::Result<()> {
    let mut devs = PciDevice::all()?;
    devs.sort_by(|a, b| {
        a.id().cmp(&b.id())
    });

    for dev in devs {
        println!(
            "PCI Device: {}: Class 0x{:>08X}, Vendor 0x{:>04X}, Device 0x{:>04X}, Revision 0x{:>02X}",
            dev.id(),
            dev.class()?,
            dev.vendor()?,
            dev.device()?,
            dev.revision()?
        );
    }

    Ok(())
}

enum GpioVendor {
    Amd,
    Intel,
}

fn gpio_communities() -> io::Result<(GpioVendor, &'static [GpioCommunity<'static>])> {
    let mut devs = PciDevice::all()?;
    devs.sort_by(|a, b| {
        a.id().cmp(&b.id())
    });
    for dev in devs {
        if dev.class()? == 0x00060100 {
            match dev.vendor()? {
                // AMD
                0x1022 => match dev.device()? {
                    // B450
                    0x790E => {
                        println!("B450 FCH");
                        return Ok((GpioVendor::Amd, GpioCommunity::b450()));
                    },

                    // Unknown PCH
                    unknown => {
                        eprintln!("Unknown FCH: {:#>04X}", unknown);
                    },
                },

                // Intel
                0x8086 => match dev.device()? & 0xFF80 {
                    // 100 Series PCH (Sky Lake)
                    0xA100 => {
                        println!("100 Series PCH");
                        return Ok((GpioVendor::Intel, GpioCommunity::skylake()));
                    },
                    // 100 Series PCH-LP (Sky Lake LP)
                    0x9D00 => {
                        println!("100 Series PCH-LP");
                        return Ok((GpioVendor::Intel, GpioCommunity::skylake_lp()));
                    }

                    // 200 Series PCH (Compatible with Sky Lake)
                    0xA280 => {
                        println!("200 Series PCH");
                        return Ok((GpioVendor::Intel, GpioCommunity::skylake()));
                    },

                    // 300 Series PCH (Cannon Lake)
                    0xA300 => {
                        println!("300 Series PCH");
                        return Ok((GpioVendor::Intel, GpioCommunity::cannonlake()));
                    },
                    // 300 Series PCH-LP (Cannon Lake LP)
                    0x9D80 => {
                        println!("300 Series PCH-LP");
                        return Ok((GpioVendor::Intel, GpioCommunity::cannonlake_lp()));
                    },

                    // 400 Series PCH (Comet Lake, compatible with Cannon Lake)
                    0x0680 => {
                        println!("400 Series PCH");
                        return Ok((GpioVendor::Intel, GpioCommunity::cannonlake()));
                    },
                    // 400 Series PCH-LP (Comet Lake LP, compatible with Cannon Lake LP)
                    0x0280 => {
                        println!("400 Series PCH-LP");
                        return Ok((GpioVendor::Intel, GpioCommunity::cannonlake_lp()));
                    },

                    // Unknown PCH
                    unknown => {
                        eprintln!("Unknown PCH: {:#>04X}", unknown);
                    },
                },

                // Unknown vendor
                unknown => {
                    eprintln!("Unknown chipset vendor: {:#>04X}", unknown);
                },
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Failed to find compatible chipset"
    ))
}

fn gpio() -> io::Result<()> {
    let (vendor, communities) = gpio_communities()?;

    match vendor {
        GpioVendor::Amd => {
            let mut mem = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/mem")?;

            for community in communities.iter() {
                for group in community.groups.iter() {
                    for i in 0..group.count {
                        print!("{}{}", group.name, i);

                        let function_offset = 0xFED8_0D00 + i;
                        mem.seek(io::SeekFrom::Start(function_offset as u64))?;
                        let mut function = [0; 1];
                        mem.read(&mut function)?;

                        let control_offset = 0xFED8_1500 + i * 4;
                        mem.seek(io::SeekFrom::Start(control_offset as u64))?;
                        let mut control = [0; 4];
                        mem.read(&mut control)?;

                        println!(" 0x{:>02x} 0x{:>08x}", function[0], u32::from_ne_bytes(control));
                    }
                }
            }
        },
        GpioVendor::Intel => {
            let sideband = unsafe {
                Sideband::new(0xFD00_0000).map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        err
                    )
                })?
            };

            for community in communities.iter() {
                for group in community.groups.iter() {
                    let mut pad = ((group.offset - community.offset) / 8) as u8;
                    for i in 0..group.count {
                        print!("{}{}", group.name, i);
                        print!(" (0x{:>02X},0x{:>02X})", community.id, pad);
                        for _j in 0..community.step {
                            let data = unsafe { sideband.gpio(community.id, pad) };
                            print!(" 0x{:>08x}", data as u32);
                            print!(" 0x{:>08x}", (data >> 32) as u32);
                            pad += 1;
                        }
                        println!();
                    }
                }
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
    let mut codecs = Vec::new();
    for entry_res in fs::read_dir("/sys/bus/hdaudio/devices")? {
        let entry = entry_res?;
        codecs.push(entry.path());
    }

    codecs.sort();

    for path in codecs {
        println!("{}", path.file_name().unwrap().to_str().unwrap());
        println!("  vendor_name: {}", read_trimmed(path.join("vendor_name"))?);
        println!("  chip_name: {}", read_trimmed(path.join("chip_name"))?);
        println!("  vendor_id: {}", read_trimmed(path.join("vendor_id"))?);
        println!("  subsystem_id: {}", read_trimmed(path.join("subsystem_id"))?);
        println!("  revision_id: {}", read_trimmed(path.join("revision_id"))?);

        let mut widgets = Vec::new();
        for entry_res in fs::read_dir(path.join("widgets"))? {
            let entry = entry_res?;
            widgets.push(entry.path());
        }

        widgets.sort();

        for path in widgets {
            if let Ok(pin_cfg) = read_trimmed(path.join("pin_cfg")) {
                if ! pin_cfg.is_empty() {
                    println!("  0x{}: {}", path.file_name().unwrap().to_str().unwrap(), pin_cfg);
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
