pub struct GpioGroup<'a> {
    pub name: &'a str,
    pub count: u8,
}

pub struct GpioCommunity<'a> {
    pub id: u8,
    pub groups: &'a [GpioGroup<'a>]
}

impl<'a> GpioCommunity<'a> {
    pub fn skylake() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0xAF,
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
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        count: 12
                    }
                ]
            },
            GpioCommunity {
                id: 0xAC,
                groups: &[
                    GpioGroup {
                        name: "GPP_I",
                        count: 11
                    }
                ]
            }
        ]
    }

    pub fn cannonlake() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0x6E,
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
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        count: 12,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6A,
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
