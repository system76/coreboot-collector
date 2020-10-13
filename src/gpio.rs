pub struct GpioGroup<'a> {
    pub name: &'a str,
    pub offset: usize,
    pub start: usize,
    pub count: usize,
}

pub struct GpioCommunity<'a> {
    pub id: u8,
    pub step: usize,
    pub offset: usize,
    pub groups: &'a [GpioGroup<'a>]
}

impl<'a> GpioCommunity<'a> {
    pub fn b450() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0,
                step: 1,
                offset: 0,
                groups: &[
                    GpioGroup {
                        name: "GPIO",
                        offset: 0,
                        start: 0,
                        count: 256,
                    },
                ],
            },
        ]
    }

    pub fn skylake() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0xAF,
                step: 1,
                offset: 0x400,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        offset: 0x400,
                        start: 0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_B",
                        offset: 0x4C0,
                        start: 0,
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAE,
                step: 1,
                offset: 0x400,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        offset: 0x400,
                        start: 0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_D",
                        offset: 0x4C0,
                        start: 0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_E",
                        offset: 0x580,
                        start: 0,
                        count: 13
                    },
                    GpioGroup {
                        name: "GPP_F",
                        offset: 0x5E8,
                        start: 0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_G",
                        offset: 0x6A8,
                        start: 0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_H",
                        offset: 0x768,
                        start: 0,
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAD,
                step: 1,
                offset: 0x400,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        offset: 0x400,
                        start: 0,
                        count: 12
                    },
                ]
            },
            GpioCommunity {
                id: 0xAC,
                step: 1,
                offset: 0x400,
                groups: &[
                    GpioGroup {
                        name: "GPP_I",
                        offset: 0x400,
                        start: 0,
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
                offset: 0x400,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        offset: 0x400,
                        start: 0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_B",
                        offset: 0x4C0,
                        start: 0,
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAE,
                step: 1,
                offset: 0x400,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        offset: 0x400,
                        start: 0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_D",
                        offset: 0x4C0,
                        start: 0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_E",
                        offset: 0x580,
                        start: 0,
                        count: 24
                    },
                ],
            },
            GpioCommunity {
                id: 0xAD,
                step: 1,
                offset: 0x400,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        offset: 0x400,
                        start: 0,
                        count: 12
                    },
                ]
            },
            GpioCommunity {
                id: 0xAC,
                step: 1,
                offset: 0x400,
                groups: &[
                    GpioGroup {
                        name: "GPP_F",
                        offset: 0x400,
                        start: 0,
                        count: 24
                    },
                    GpioGroup {
                        name: "GPP_G",
                        offset: 0x4C0,
                        start: 0,
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
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B",
                        offset: 0x790,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6D,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D",
                        offset: 0x780,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G",
                        offset: 0x900,
                        start: 0,
                        count: 8,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        offset: 0x600,
                        start: 0,
                        count: 12,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6B,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_K",
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_H",
                        offset: 0x780,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E",
                        offset: 0x900,
                        start: 0,
                        count: 13,
                    },
                    GpioGroup {
                        name: "GPP_F",
                        offset: 0x9D0,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6A,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_I",
                        offset: 0x740,
                        start: 0,
                        count: 15,
                    },
                    GpioGroup {
                        name: "GPP_J",
                        offset: 0x860,
                        start: 0,
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
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B",
                        offset: 0x790,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G",
                        offset: 0x930,
                        start: 0,
                        count: 8,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6D,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_D",
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_F",
                        offset: 0x790,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_H",
                        offset: 0x910,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        offset: 0x600,
                        start: 0,
                        count: 12,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6A,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E",
                        offset: 0x780,
                        start: 0,
                        count: 24,
                    },
                ],
            },
        ]
    }

    pub fn tigerlake() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0x6E,
                step: 2,
                offset: 0x680,
                groups: &[
                    GpioGroup {
                        name: "GPP_A",
                        offset: 0x680,
                        start: 0,
                        count: 15,
                    },
                    GpioGroup {
                        name: "GPP_R",
                        offset: 0x790,
                        start: 0,
                        count: 20,
                    },
                    GpioGroup {
                        name: "GPP_B",
                        offset: 0x8D0,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6D,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_D",
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_C",
                        offset: 0x7A0,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_S",
                        offset: 0x920,
                        start: 0,
                        count: 8,
                    },
                    GpioGroup {
                        name: "GPP_G",
                        offset: 0x9A0,
                        start: 0,
                        count: 16,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        offset: 0x600,
                        start: 0,
                        count: 13,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6B,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_E",
                        offset: 0x600,
                        start: 0,
                        count: 13,
                    },
                    GpioGroup {
                        name: "GPP_F",
                        offset: 0x6D0,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6A,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_H",
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_J",
                        offset: 0x780,
                        start: 0,
                        count: 10,
                    },
                    GpioGroup {
                        name: "GPP_K",
                        offset: 0x820,
                        start: 0,
                        count: 12,
                    },
                ],
            },
            GpioCommunity {
                id: 0x69,
                step: 2,
                offset: 0x600,
                groups: &[
                    GpioGroup {
                        name: "GPP_I",
                        offset: 0x600,
                        start: 0,
                        count: 15,
                    },
                ],
            },
        ]
    }

    pub fn tigerlake_lp() -> &'static [GpioCommunity<'static>] {
        &[
            GpioCommunity {
                id: 0x6E,
                step: 2,
                offset: 0x700,
                groups: &[
                    GpioGroup {
                        name: "GPP_B",
                        offset: 0x700,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_T",
                        offset: 0x8C0,
                        start: 2,
                        count: 2,
                    },
                    GpioGroup {
                        name: "GPP_A",
                        offset: 0x9A0,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6D,
                step: 2,
                offset: 0x700,
                groups: &[
                    GpioGroup {
                        name: "GPP_S",
                        offset: 0x700,
                        start: 0,
                        count: 8,
                    },
                    GpioGroup {
                        name: "GPP_H",
                        offset: 0x780,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D",
                        offset: 0x900,
                        start: 0,
                        count: 20,
                    },
                    GpioGroup {
                        name: "GPP_U",
                        offset: 0xA90,
                        start: 4,
                        count: 2,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                offset: 0x700,
                groups: &[
                    GpioGroup {
                        name: "GPD",
                        offset: 0x700,
                        start: 0,
                        count: 12,
                    },
                ],
            },
            // Community 3 (0x6B) is not used
            GpioCommunity {
                id: 0x6A,
                step: 2,
                offset: 0x700,
                groups: &[
                    GpioGroup {
                        name: "GPP_C",
                        offset: 0x700,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_F",
                        offset: 0x880,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E",
                        offset: 0xA70,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x69,
                step: 2,
                offset: 0x700,
                groups: &[
                    GpioGroup {
                        name: "GPP_R",
                        offset: 0x700,
                        start: 0,
                        count: 8,
                    },
                ],
            },
        ]
    }
}
