pub struct GpioGroup<'a> {
    pub name: &'a str,
    pub count: u8,
}

pub struct GpioCommunity<'a> {
    pub id: u8,
    pub step: usize,
    pub groups: &'a [GpioGroup<'a>]
}

impl<'a> GpioCommunity<'a> {
    pub fn skylake() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0xAF,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_B",
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAE,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_D",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_E",
                        count: 13
                    },
                    GpioGroup {
                        name: "GPP_F",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_G",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_H",
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAD,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        count: 12
                    },
                ]
            },
            GpioCommunity {
                id: 0xAC,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_I",
                        count: 11
                    },
                ]
            }
        ]
    }

    pub fn skylake_lp() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0xAF,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_B",
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAE,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_D",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_E",
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAD,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        count: 12
                    },
                ]
            },
            GpioCommunity {
                id: 0xAC,
                step: 1,
                groups: &[
                    GpioGroup {
                        name: "GPP_F",
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_G",
                        count: 8
                    },
                ]
            }
        ]
    }

    pub fn cannonlake() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0x6E,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B",
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6D,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G",
                        count: 8,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        count: 12,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6B,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_K",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_H",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E",
                        count: 13,
                    },
                    GpioGroup {
                        name: "GPP_F",
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6A,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_I",
                        count: 15,
                    },
                    GpioGroup {
                        name: "GPP_J",
                        count: 12,
                    },
                ],
            },
        ]
    }

    pub fn cannonlake_lp() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0x6E,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G",
                        count: 8,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6D,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_D",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_F",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_H",
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        count: 12,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6A,
                step: 2,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E",
                        count: 24,
                    },
                ],
            },
        ]
    }
}
