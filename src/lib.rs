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
