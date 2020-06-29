use rand_core::{RngCore, Error, impls};

/* Cong ----------------------------------------------------------------------*/

#[derive(Debug)]
pub struct Cong {
    cong: u32,
}

impl Cong {
    pub fn new() -> Cong {
        Cong {
            cong: 0,
        }
    }
}

impl RngCore for Cong {
    fn next_u32(&mut self) -> u32 {
        self.cong = self.cong.wrapping_mul(69069).wrapping_add(12345);
        self.cong
    }
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}


/* SHR3 ----------------------------------------------------------------------*/

#[derive(Debug)]
pub struct SHR3 {
    shr3: u32,
}

impl SHR3 {
    pub fn new() -> SHR3 {
        SHR3 {
            shr3: 1,
        }
    }
    fn sanitise(&mut self) {
        if self.shr3 == 0 {
            self.shr3 = 1;
        }
    }
}

impl RngCore for SHR3 {
    fn next_u32(&mut self) -> u32 {
        self.sanitise();
        let mut shr3 = self.shr3;
    
        shr3 ^= shr3 << 13;
        shr3 ^= shr3 >> 17;
        shr3 ^= shr3 << 5;
        self.shr3 = shr3;
        
        shr3
    }
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}


/* MWC2 ----------------------------------------------------------------------*/

#[derive(Debug)]
pub struct MWC2 {
    upper: u32,
    lower: u32,
}

impl MWC2 {
    pub fn new() -> MWC2 {
        MWC2 {
            upper: 1,
            lower: 1,
        }
    }
    fn sanitise_upper(&mut self) {
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
    fn sanitise_lower(&mut self) {
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
    fn next_upper(&mut self) {
        self.upper = (self.upper & 0xFFFF).wrapping_mul(36969).wrapping_add(self.upper >> 16);
    }
    fn next_lower(&mut self) {
        self.lower = (self.lower & 0xFFFF).wrapping_mul(18000).wrapping_add(self.lower >> 16);
    }
    fn current(&self) -> u32 {
        self.lower.wrapping_add(self.upper << 16).wrapping_add(self.upper >> 16)
    }
}
impl RngCore for MWC2 {
    fn next_u32(&mut self) -> u32 {
        self.sanitise_upper();
        self.sanitise_lower();

        self.next_upper();
        self.next_lower();

        self.current()
    }
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}


/* KISS ----------------------------------------------------------------------*/

#[derive(Debug)]
pub struct KISS {
    mwc2: MWC2,
    cong: Cong,
    shr3: SHR3,
}

impl KISS {
    pub fn new() -> KISS {
        KISS {
            mwc2: MWC2::new(),
            cong: Cong::new(),
            shr3: SHR3::new(),
        }
    }
    fn current(&self) -> u32 {
        (self.mwc2.current() ^ self.cong.cong).wrapping_add(self.shr3.shr3)
    }
}
impl RngCore for KISS {
    fn next_u32(&mut self) -> u32 {
        self.mwc2.next_u32();
        self.cong.next_u32();
        self.shr3.next_u32();

        self.current()
    }
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}


/* LFSR ----------------------------------------------------------------------*/

fn lfsr_next_z(z: u32, a: u8, b: u8, c: u8, mask: u32) -> u32 {
    let b = ((z << a) ^ z) >> b;
    ((z & mask) << c) ^ b
}

/* LFSR88 --------------------------------------------------------------------*/

#[derive(Debug)]
pub struct LFSR88 {
    z1: u32,
    z2: u32,
    z3: u32,
}

impl LFSR88 {
    //const Z1_MIN: u32 = 2;
    //const Z2_MIN: u32 = 8;
    //const Z3_MIN: u32 = 16;

    pub fn new() -> LFSR88 {
        LFSR88 {
            z1: 0xFFFFFFFF,
            z2: 0xFFFFFFFF,
            z3: 0xFFFFFFFF,
        }
    }
    fn next_z1(&mut self) {
        self.z1 = lfsr_next_z(self.z1, 13, 19, 12, 0xFFFFFFFE);
    }
    fn next_z2(&mut self) {
        self.z2 = lfsr_next_z(self.z2, 2, 25, 4, 0xFFFFFFF8);
    }
    fn next_z3(&mut self) {
        self.z3 = lfsr_next_z(self.z3, 3, 11, 17, 0xFFFFFFF0);
    }
    fn current(&self) -> u32 {
        self.z1 ^ self.z2 ^ self.z3
    }
}
impl RngCore for LFSR88 {
    fn next_u32(&mut self) -> u32 {
        self.next_z1();
        self.next_z2();
        self.next_z3();

        self.current()
    }
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}


/* LFSR113 -------------------------------------------------------------------*/

#[derive(Debug)]
pub struct LFSR113 {
    z1: u32,
    z2: u32,
    z3: u32,
    z4: u32,
}

impl LFSR113 {
    //const Z1_MIN: u32 = 2;
    //const Z2_MIN: u32 = 8;
    //const Z3_MIN: u32 = 16;
    //const Z4_MIN: u32 = 128;

    pub fn new() -> LFSR113 {
        LFSR113 {
            z1: 0xFFFFFFFF,
            z2: 0xFFFFFFFF,
            z3: 0xFFFFFFFF,
            z4: 0xFFFFFFFF,
        }
    }
    fn next_z1(&mut self) {
        self.z1 = lfsr_next_z(self.z1, 6, 13, 18, 0xFFFFFFFE);
    }
    fn next_z2(&mut self) {
        self.z2 = lfsr_next_z(self.z2, 2, 27, 2, 0xFFFFFFF8);
    }
    fn next_z3(&mut self) {
        self.z3 = lfsr_next_z(self.z3, 13, 21, 7, 0xFFFFFFF0);
    }
    fn next_z4(&mut self) {
        self.z4 = lfsr_next_z(self.z4, 3, 12, 13, 0xFFFFFF80);
    }
    fn current(&self) -> u32 {
        self.z1 ^ self.z2 ^ self.z3 ^ self.z4
    }
}
impl RngCore for LFSR113 {
    fn next_u32(&mut self) -> u32 {
        self.next_z1();
        self.next_z2();
        self.next_z3();
        self.next_z4();

        self.current()
    }
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
