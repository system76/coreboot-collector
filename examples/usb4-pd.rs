extern crate coreboot_collector;
extern crate libc;
extern crate sysfs_class;

use coreboot_collector::gpio::GpioCommunity;
use coreboot_collector::sideband::Sideband;
use std::{fs, io, process};
use std::io::{Read, Seek};
use sysfs_class::{PciDevice, SysClass};

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

                    // 500 Series PCH-LP (Tiger Lake LP)
                    0xA080 => {
                        println!("500 Series PCH-LP");
                        return Ok((GpioVendor::Intel, GpioCommunity::tigerlake_lp()));
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
                    for i in group.start..group.start + group.count {
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
                    for i in group.start..group.start + group.count {
                        for j in 0..community.step {
                            if group.name == "GPP_E" && i == 1 && j == 0 {
                                print!("{}{}", group.name, i);
                                let mut data = unsafe { sideband.gpio(community.id, pad) };
                                print!(" 0x{:>08x}", data as u32);
                                print!(" 0x{:>08x}", (data >> 32) as u32);
                                print!(" =>");

                                data |= 1;
                                print!(" 0x{:>08x}", data as u32);
                                print!(" 0x{:>08x}", (data >> 32) as u32);
                                unsafe { sideband.set_gpio(community.id, pad, data) };

                                println!();
                            }
                            pad += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("usb4-pd: must be run as root");
        process::exit(1);
    }

    if let Err(err) = gpio() {
        eprintln!("usb4-pd: {:?}", err);
        process::exit(1);
    }
}
