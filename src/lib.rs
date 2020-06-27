use std::num::Wrapping;

#[derive(Debug)]
pub struct Cong {
    cong: u32,
}

#[derive(Debug)]
pub struct Shr3 {
    shr3: u32,
}

#[derive(Debug)]
pub struct MWC2 {
    upper: u32,
    lower: u32,
}

#[derive(Debug)]
pub struct KISS {
    mwc2: MWC2,
    cong: Cong,
    shr3: Shr3,
}

#[derive(Debug)]
pub struct LFSR88 {
    z1: u32,
    z2: u32,
    z3: u32,
}

#[derive(Debug)]
pub struct LFSR113 {
    z1: u32,
    z2: u32,
    z3: u32,
    z4: u32,
}


impl Cong {
    pub fn new() -> Cong {
        Cong {
            cong: 1,
        }
    }
    pub fn next(&mut self) -> u32 {
        let mut cong = Wrapping::<u32>(self.cong);

        cong = Wrapping::<u32>(69069) * cong + Wrapping::<u32>(12345);
        self.cong = cong.0;

        self.cong
    }
}

impl Shr3 {
    pub fn new() -> Shr3 {
        Shr3 {
            shr3: 1,
        }
    }
    pub fn sanitise(&mut self) {
        if self.shr3 == 0 {
            self.shr3 = 1;
        }
    }
    pub fn next(&mut self) -> u32 {
        self.sanitise();
        let mut shr3 = self.shr3;
    
        shr3 ^= shr3 << 13;
        shr3 ^= shr3 >> 17;
        shr3 ^= shr3 << 5;
        self.shr3 = shr3;
        
        shr3
    }
}

impl MWC2 {
    pub fn new() -> MWC2 {
        MWC2 {
            upper: 1,
            lower: 1,
        }
    }
    pub fn sanitise_upper(&mut self) {
        if self.upper > 0x9068FFFF {
            self.upper -= 0x9068FFFF;
        }
        if self.upper == 0 {
            self.upper = 0xFFFFFFFF;
            if self.upper > 0x9068FFFF {
                self.upper -= 0x9068FFFF;
            }
        }
    }
    pub fn sanitise_lower(&mut self) {
        if self.lower > 0x464FFFFF {
            self.lower -= 0x464FFFFF;
        }
        if self.lower == 0 {
            self.lower = 0xFFFFFFFF;
            if self.lower > 0x464FFFFF {
                self.lower -= 0x464FFFFF;
            }
        }
    }
    pub fn next_upper(&mut self) {
        let new_upper = Wrapping::<u32>(36969) * Wrapping::<u32>(self.upper & 0xFFFF) + Wrapping::<u32>(self.upper >> 16);
        self.upper = new_upper.0
    }
    pub fn next_lower(&mut self) {
        let new_lower = Wrapping::<u32>(18000) * Wrapping::<u32>(self.lower & 0xFFFF) + Wrapping::<u32>(self.lower >> 16);
        self.lower = new_lower.0
    }
    pub fn current(&self) -> u32 {
        let new_value = Wrapping::<u32>(self.upper << 16) + Wrapping::<u32>(self.upper >> 16) + Wrapping::<u32>(self.lower);
        new_value.0
    }
    pub fn next(&mut self) -> u32 {
        self.sanitise_upper();
        self.sanitise_lower();

        self.next_upper();
        self.next_lower();

        self.current()
    }
}

impl KISS {
    pub fn new() -> KISS {
        KISS {
            mwc2: MWC2::new(),
            cong: Cong::new(),
            shr3: Shr3::new(),
        }
    }
    pub fn current(&self) -> u32 {
        let new_value = Wrapping::<u32>(self.mwc2.current() ^ self.cong.cong) + Wrapping::<u32>(self.shr3.shr3);
        new_value.0
    }
    pub fn next(&mut self) -> u32 {
        self.mwc2.next();
        self.cong.next();
        self.shr3.next();

        self.current()
    }
}

fn next_z(z: u32, a: u8, b: u8, c: u8, mask: u32) -> u32 {
    let b = ((z << a) ^ z) >> b;
    ((z & mask) << c) ^ b
}

impl LFSR88 {
    const Z1_MIN: u32 = 2;
    const Z2_MIN: u32 = 8;
    const Z3_MIN: u32 = 16;

    pub fn new() -> LFSR88 {
        LFSR88 {
            z1: 0xFFFFFFFF,
            z2: 0xFFFFFFFF,
            z3: 0xFFFFFFFF,
        }
    }
    pub fn next_z1(&mut self) {
        self.z1 = next_z(self.z1, 13, 19, 12, 0xFFFFFFFE);
    }
    pub fn next_z2(&mut self) {
        self.z2 = next_z(self.z2, 2, 25, 4, 0xFFFFFFF8);
    }
    pub fn next_z3(&mut self) {
        self.z3 = next_z(self.z3, 3, 11, 17, 0xFFFFFFF0);
    }
    pub fn current(&self) -> u32 {
        self.z1 ^ self.z2 ^ self.z3
    }
    pub fn next(&mut self) -> u32 {
        self.next_z1();
        self.next_z2();
        self.next_z3();

        self.current()
    }
}

impl LFSR113 {
    const Z1_MIN: u32 = 2;
    const Z2_MIN: u32 = 8;
    const Z3_MIN: u32 = 16;
    const Z4_MIN: u32 = 128;

    pub fn new() -> LFSR113 {
        LFSR113 {
            z1: 0xFFFFFFFF,
            z2: 0xFFFFFFFF,
            z3: 0xFFFFFFFF,
            z4: 0xFFFFFFFF,
        }
    }
    pub fn next_z1(&mut self) {
        self.z1 = next_z(self.z1, 6, 13, 18, 0xFFFFFFFE);
    }
    pub fn next_z2(&mut self) {
        self.z2 = next_z(self.z2, 2, 27, 2, 0xFFFFFFF8);
    }
    pub fn next_z3(&mut self) {
        self.z3 = next_z(self.z3, 13, 21, 7, 0xFFFFFFF0);
    }
    pub fn next_z4(&mut self) {
        self.z4 = next_z(self.z4, 3, 12, 13, 0xFFFFFF80);
    }
    pub fn current(&self) -> u32 {
        self.z1 ^ self.z2 ^ self.z3 ^ self.z4
    }
    pub fn next(&mut self) -> u32 {
        self.next_z1();
        self.next_z2();
        self.next_z3();
        self.next_z4();

        self.current()
    }
}
