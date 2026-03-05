use std::ops::{Bound, RangeBounds};

#[derive(Debug, Eq, PartialEq)]
pub struct GpioGroup {
    pub name: String,
    pub offset: usize,
    pub start: usize,
    pub count: usize,
}

impl GpioGroup {
    pub fn new<R: RangeBounds<usize>>(name: impl Into<String>, offset: usize, range: R) -> Self {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i + 1,
        };
        let end = match range.end_bound() {
            Bound::Unbounded => panic!("GpioGroup::new cannot handle unbounded range"),
            Bound::Included(i) => *i,
            Bound::Excluded(i) => *i - 1,
        };
        Self {
            name: name.into(),
            offset,
            start,
            count: end + 1 - start,
        }
    }
}

pub struct GpioCommunity {
    pub id: u8,
    pub step: usize,
    pub offset: usize,
    pub groups: Vec<GpioGroup>,
}

impl GpioCommunity {
    pub fn b450() -> Vec<GpioCommunity> {
        vec![GpioCommunity {
            id: 0,
            step: 1,
            offset: 0,
            groups: vec![GpioGroup {
                name: "GPIO".into(),
                offset: 0,
                start: 0,
                count: 256,
            }],
        }]
    }

    pub fn skylake() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0xAF,
                step: 1,
                offset: 0x400,
                groups: vec![
                    GpioGroup {
                        name: "GPP_A".into(),
                        offset: 0x400,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B".into(),
                        offset: 0x4C0,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0xAE,
                step: 1,
                offset: 0x400,
                groups: vec![
                    GpioGroup {
                        name: "GPP_C".into(),
                        offset: 0x400,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D".into(),
                        offset: 0x4C0,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E".into(),
                        offset: 0x580,
                        start: 0,
                        count: 13,
                    },
                    GpioGroup {
                        name: "GPP_F".into(),
                        offset: 0x5E8,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G".into(),
                        offset: 0x6A8,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_H".into(),
                        offset: 0x768,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0xAD,
                step: 1,
                offset: 0x400,
                groups: vec![GpioGroup {
                    name: "GPD".into(),
                    offset: 0x400,
                    start: 0,
                    count: 12,
                }],
            },
            GpioCommunity {
                id: 0xAC,
                step: 1,
                offset: 0x400,
                groups: vec![GpioGroup {
                    name: "GPP_I".into(),
                    offset: 0x400,
                    start: 0,
                    count: 11,
                }],
            },
        ]
    }

    pub fn skylake_lp() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0xAF,
                step: 1,
                offset: 0x400,
                groups: vec![
                    GpioGroup {
                        name: "GPP_A".into(),
                        offset: 0x400,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B".into(),
                        offset: 0x4C0,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0xAE,
                step: 1,
                offset: 0x400,
                groups: vec![
                    GpioGroup {
                        name: "GPP_C".into(),
                        offset: 0x400,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D".into(),
                        offset: 0x4C0,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E".into(),
                        offset: 0x580,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0xAD,
                step: 1,
                offset: 0x400,
                groups: vec![GpioGroup {
                    name: "GPD".into(),
                    offset: 0x400,
                    start: 0,
                    count: 12,
                }],
            },
            GpioCommunity {
                id: 0xAC,
                step: 1,
                offset: 0x400,
                groups: vec![
                    GpioGroup {
                        name: "GPP_F".into(),
                        offset: 0x400,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G".into(),
                        offset: 0x4C0,
                        start: 0,
                        count: 8,
                    },
                ],
            },
        ]
    }

    pub fn cannonlake() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0x6E,
                step: 2,
                offset: 0x600,
                groups: vec![
                    GpioGroup {
                        name: "GPP_A".into(),
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B".into(),
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
                groups: vec![
                    GpioGroup {
                        name: "GPP_C".into(),
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D".into(),
                        offset: 0x780,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G".into(),
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
                groups: vec![GpioGroup {
                    name: "GPD".into(),
                    offset: 0x600,
                    start: 0,
                    count: 12,
                }],
            },
            GpioCommunity {
                id: 0x6B,
                step: 2,
                offset: 0x600,
                groups: vec![
                    GpioGroup {
                        name: "GPP_K".into(),
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_H".into(),
                        offset: 0x780,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E".into(),
                        offset: 0x900,
                        start: 0,
                        count: 13,
                    },
                    GpioGroup {
                        name: "GPP_F".into(),
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
                groups: vec![
                    GpioGroup {
                        name: "GPP_I".into(),
                        offset: 0x740,
                        start: 0,
                        count: 15,
                    },
                    GpioGroup {
                        name: "GPP_J".into(),
                        offset: 0x860,
                        start: 0,
                        count: 12,
                    },
                ],
            },
        ]
    }

    pub fn cannonlake_lp() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0x6E,
                step: 2,
                offset: 0x600,
                groups: vec![
                    GpioGroup {
                        name: "GPP_A".into(),
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_B".into(),
                        offset: 0x790,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G".into(),
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
                groups: vec![
                    GpioGroup {
                        name: "GPP_D".into(),
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_F".into(),
                        offset: 0x790,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_H".into(),
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
                groups: vec![GpioGroup {
                    name: "GPD".into(),
                    offset: 0x600,
                    start: 0,
                    count: 12,
                }],
            },
            GpioCommunity {
                id: 0x6A,
                step: 2,
                offset: 0x600,
                groups: vec![
                    GpioGroup {
                        name: "GPP_C".into(),
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E".into(),
                        offset: 0x780,
                        start: 0,
                        count: 24,
                    },
                ],
            },
        ]
    }

    pub fn tigerlake() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0x6E,
                step: 2,
                offset: 0x600,
                groups: vec![
                    GpioGroup {
                        name: "GPP_A".into(),
                        offset: 0x680,
                        start: 0,
                        count: 15,
                    },
                    GpioGroup {
                        name: "GPP_R".into(),
                        offset: 0x790,
                        start: 0,
                        count: 20,
                    },
                    GpioGroup {
                        name: "GPP_B".into(),
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
                groups: vec![
                    GpioGroup {
                        name: "GPP_D".into(),
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_C".into(),
                        offset: 0x7A0,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_S".into(),
                        offset: 0x920,
                        start: 0,
                        count: 8,
                    },
                    GpioGroup {
                        name: "GPP_G".into(),
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
                groups: vec![GpioGroup {
                    name: "GPD".into(),
                    offset: 0x600,
                    start: 0,
                    count: 13,
                }],
            },
            GpioCommunity {
                id: 0x6B,
                step: 2,
                offset: 0x600,
                groups: vec![
                    GpioGroup {
                        name: "GPP_E".into(),
                        offset: 0x600,
                        start: 0,
                        count: 13,
                    },
                    GpioGroup {
                        name: "GPP_F".into(),
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
                groups: vec![
                    GpioGroup {
                        name: "GPP_H".into(),
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_J".into(),
                        offset: 0x780,
                        start: 0,
                        count: 10,
                    },
                    GpioGroup {
                        name: "GPP_K".into(),
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
                groups: vec![GpioGroup {
                    name: "GPP_I".into(),
                    offset: 0x600,
                    start: 0,
                    count: 15,
                }],
            },
        ]
    }

    pub fn tigerlake_lp() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0x6E,
                step: 2,
                offset: 0x700,
                groups: vec![
                    GpioGroup {
                        name: "GPP_B".into(),
                        offset: 0x700,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_T".into(),
                        offset: 0x8C0,
                        start: 2,
                        count: 2,
                    },
                    GpioGroup {
                        name: "GPP_A".into(),
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
                groups: vec![
                    GpioGroup {
                        name: "GPP_S".into(),
                        offset: 0x700,
                        start: 0,
                        count: 8,
                    },
                    GpioGroup {
                        name: "GPP_H".into(),
                        offset: 0x780,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D".into(),
                        offset: 0x900,
                        start: 0,
                        count: 20,
                    },
                    GpioGroup {
                        name: "GPP_U".into(),
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
                groups: vec![GpioGroup {
                    name: "GPD".into(),
                    offset: 0x700,
                    start: 0,
                    count: 12,
                }],
            },
            // Community 3 (0x6B) is not used
            GpioCommunity {
                id: 0x6A,
                step: 2,
                offset: 0x700,
                groups: vec![
                    GpioGroup {
                        name: "GPP_C".into(),
                        offset: 0x700,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_F".into(),
                        offset: 0x880,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E".into(),
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
                groups: vec![GpioGroup {
                    name: "GPP_R".into(),
                    offset: 0x700,
                    start: 0,
                    count: 8,
                }],
            },
        ]
    }

    pub fn alderlake_lp() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0x6E,
                step: 2,
                offset: 0x700,
                groups: vec![
                    GpioGroup {
                        name: "GPP_B".into(),
                        offset: 0x700,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_T".into(),
                        offset: 0x8C0,
                        start: 2,
                        count: 2,
                    },
                    GpioGroup {
                        name: "GPP_A".into(),
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
                groups: vec![
                    GpioGroup {
                        name: "GPP_S".into(),
                        offset: 0x700,
                        start: 0,
                        count: 8,
                    },
                    GpioGroup {
                        name: "GPP_H".into(),
                        offset: 0x780,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D".into(),
                        offset: 0x900,
                        start: 0,
                        count: 20,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                offset: 0x700,
                groups: vec![GpioGroup {
                    name: "GPD".into(),
                    offset: 0x700,
                    start: 0,
                    count: 12,
                }],
            },
            // Community 3 (0x6B) is not used
            GpioCommunity {
                id: 0x6A,
                step: 2,
                offset: 0x700,
                groups: vec![
                    GpioGroup {
                        name: "GPP_C".into(),
                        offset: 0x700,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_F".into(),
                        offset: 0x880,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_E".into(),
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
                groups: vec![GpioGroup {
                    name: "GPP_R".into(),
                    offset: 0x700,
                    start: 0,
                    count: 8,
                }],
            },
        ]
    }

    pub fn alderlake() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0x6E,
                step: 2,
                offset: 0x700,
                groups: vec![
                    GpioGroup {
                        name: "GPP_I".into(),
                        offset: 0x700,
                        start: 0,
                        count: 23,
                    },
                    GpioGroup {
                        name: "GPP_R".into(),
                        offset: 0x890,
                        start: 0,
                        count: 22,
                    },
                    GpioGroup {
                        name: "GPP_J".into(),
                        offset: 0xA00,
                        start: 0,
                        count: 12,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6D,
                step: 2,
                offset: 0x700,
                groups: vec![
                    GpioGroup {
                        name: "GPP_B".into(),
                        offset: 0x700,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_G".into(),
                        offset: 0x880,
                        start: 0,
                        count: 8,
                    },
                    GpioGroup {
                        name: "GPP_H".into(),
                        offset: 0x900,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6C,
                step: 2,
                offset: 0x700,
                groups: vec![GpioGroup {
                    name: "GPD".into(),
                    offset: 0x700,
                    start: 0,
                    count: 13,
                }],
            },
            GpioCommunity {
                id: 0x6B,
                step: 2,
                offset: 0x700,
                groups: vec![
                    GpioGroup {
                        name: "GPP_A".into(),
                        offset: 0x790,
                        start: 0,
                        count: 15,
                    },
                    GpioGroup {
                        name: "GPP_C".into(),
                        offset: 0x890,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x6A,
                step: 2,
                offset: 0x700,
                groups: vec![
                    GpioGroup {
                        name: "GPP_S".into(),
                        offset: 0x700,
                        start: 0,
                        count: 8,
                    },
                    GpioGroup {
                        name: "GPP_E".into(),
                        offset: 0x780,
                        start: 0,
                        count: 22,
                    },
                    GpioGroup {
                        name: "GPP_K".into(),
                        offset: 0x8F0,
                        start: 0,
                        count: 12,
                    },
                    GpioGroup {
                        name: "GPP_F".into(),
                        offset: 0x9E0,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0x69,
                step: 2,
                offset: 0x600,
                groups: vec![GpioGroup {
                    name: "GPP_D".into(),
                    offset: 0x700,
                    start: 0,
                    count: 24,
                }],
            },
        ]
    }

    pub fn meteorlake_hu() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0xD1,
                step: 2,
                offset: 0x650,
                groups: vec![
                    GpioGroup {
                        name: "GPP_V".into(),
                        offset: 0x650,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_C".into(),
                        offset: 0x7D0,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0xD2,
                step: 2,
                offset: 0x600,
                groups: vec![
                    GpioGroup {
                        name: "GPP_A".into(),
                        offset: 0x600,
                        start: 0,
                        count: 22,
                    },
                    GpioGroup {
                        name: "GPP_E".into(),
                        offset: 0x790,
                        start: 0,
                        count: 23,
                    },
                ],
            },
            GpioCommunity {
                id: 0xD3,
                step: 2,
                offset: 0x600,
                groups: vec![
                    GpioGroup {
                        name: "GPP_H".into(),
                        offset: 0x600,
                        start: 0,
                        count: 23,
                    },
                    GpioGroup {
                        name: "GPP_F".into(),
                        offset: 0x7A0,
                        start: 0,
                        count: 24,
                    },
                ],
            },
            GpioCommunity {
                id: 0xD4,
                step: 2,
                offset: 0x600,
                groups: vec![GpioGroup {
                    name: "GPP_S".into(),
                    offset: 0x600,
                    start: 0,
                    count: 8,
                }],
            },
            GpioCommunity {
                id: 0xD5,
                step: 2,
                offset: 0x600,
                groups: vec![
                    GpioGroup {
                        name: "GPP_B".into(),
                        offset: 0x600,
                        start: 0,
                        count: 24,
                    },
                    GpioGroup {
                        name: "GPP_D".into(),
                        offset: 0x790,
                        start: 0,
                        count: 24,
                    },
                ],
            },
        ]
    }

    pub fn pantherlake_hu() -> Vec<GpioCommunity> {
        vec![
            GpioCommunity {
                id: 0xD1,
                step: 2,
                offset: 0x800,
                groups: vec![
                    GpioGroup::new("GPP_V", 0x800, 0..=17),
                    GpioGroup::new("GPP_C", 0x980, 0..=23),
                ],
            },
            GpioCommunity {
                id: 0xD2,
                step: 2,
                offset: 0x800,
                groups: vec![
                    GpioGroup::new("GPP_F", 0x800, 0..=23),
                    GpioGroup::new("GPP_E", 0x9B0, 1..=22),
                ],
            },
            GpioCommunity {
                id: 0xD3,
                step: 2,
                offset: 0x8F0,
                groups: vec![
                    GpioGroup::new("GPP_H", 0x8F0, 0..=24),
                    GpioGroup::new("GPP_A", 0xAB0, 0..=17),
                ],
            },
            GpioCommunity {
                id: 0xD4,
                step: 2,
                offset: 0x800,
                groups: vec![GpioGroup::new("GPP_S", 0x800, 0..=7)],
            },
            GpioCommunity {
                id: 0xD5,
                step: 2,
                offset: 0x800,
                groups: vec![
                    GpioGroup::new("GPP_B", 0x800, 0..=25),
                    GpioGroup::new("GPP_D", 0x9B0, 0..=25),
                ],
            },
        ]
    }
}

#[test]
fn test_range() {
    assert_eq!(
        GpioGroup::new("GPP_V", 0x800, 0..=17),
        GpioGroup {
            name: "GPP_V".into(),
            offset: 0x800,
            start: 0,
            count: 18,
        }
    );
    assert_eq!(
        GpioGroup::new("GPP_A", 0x600, 1..=22),
        GpioGroup {
            name: "GPP_A".into(),
            offset: 0x600,
            start: 1,
            count: 22,
        }
    );
    assert_eq!(
        GpioGroup::new("GPP_A", 0x600, 0..=23),
        GpioGroup {
            name: "GPP_A".into(),
            offset: 0x600,
            start: 0,
            count: 24,
        }
    );
    assert_eq!(
        GpioGroup::new("GPP_A", 0x600, ..24),
        GpioGroup {
            name: "GPP_A".into(),
            offset: 0x600,
            start: 0,
            count: 24,
        }
    );
}
