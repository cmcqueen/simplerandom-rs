use rand_core::{RngCore, Error, impls};

/* Cong ----------------------------------------------------------------------*/

#[derive(Debug)]
pub struct Cong {
    cong: u32,
}

impl Cong {
    pub fn new(seed1: u32) -> Cong {
        Cong {
            cong: seed1,
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
    pub fn new(seed1: u32) -> SHR3 {
        SHR3 {
            shr3: seed1,
        }
    }
    fn sanitise(&mut self) {
        if self.shr3 == 0 {
            self.shr3 = 0xFFFFFFFF;
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

fn mwc32_sanitise(x: u32, limit: u32) -> u32 {
    let mut temp = x;
    if temp >= limit {
        temp -= limit;
    }
    if temp == 0 {
        temp = x ^ 0xFFFFFFFF;
        if temp >= limit {
            temp -= limit;
        }
    }
    temp
}

fn mwc32_next(x: u32, multiplier: u32) -> u32 {
    (x & 0xFFFF).wrapping_mul(multiplier).wrapping_add(x >> 16)
}

impl MWC2 {
    pub fn new(seed1: u32, seed2: u32) -> MWC2 {
        MWC2 {
            upper: seed1,
            lower: seed2,
        }
    }
    fn current(&self) -> u32 {
        self.lower.wrapping_add(self.upper << 16).wrapping_add(self.upper >> 16)
    }
}
impl RngCore for MWC2 {
    fn next_u32(&mut self) -> u32 {
        self.upper = mwc32_sanitise(self.upper, 0x9068FFFF);
        self.lower = mwc32_sanitise(self.lower, 0x464FFFFF);

        self.upper = mwc32_next(self.upper, 36969);
        self.lower = mwc32_next(self.lower, 18000);

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


/* MWC1 ----------------------------------------------------------------------*/

#[derive(Debug)]
pub struct MWC1 {
    mwc: MWC2,
}

impl MWC1 {
    pub fn new(seed1: u32, seed2: u32) -> MWC1 {
        MWC1 {
            mwc: MWC2::new(seed1, seed2),
        }
    }
    fn current(&self) -> u32 {
        self.mwc.lower.wrapping_add(self.mwc.upper << 16)
    }
}
impl RngCore for MWC1 {
    fn next_u32(&mut self) -> u32 {
        self.mwc.next_u32();
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
    mwc: MWC2,
    cong: Cong,
    shr3: SHR3,
}

impl KISS {
    pub fn new(seed1: u32, seed2: u32, seed3: u32, seed4: u32) -> KISS {
        KISS {
            mwc: MWC2::new(seed1, seed2),
            cong: Cong::new(seed3),
            shr3: SHR3::new(seed4),
        }
    }
    fn current(&self) -> u32 {
        (self.mwc.current() ^ self.cong.cong).wrapping_add(self.shr3.shr3)
    }
}
impl RngCore for KISS {
    fn next_u32(&mut self) -> u32 {
        self.mwc.next_u32();
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


/* MWC64 ---------------------------------------------------------------------*/

#[derive(Debug)]
pub struct MWC64 {
    mwc: u64,
}

fn mwc64_sanitise(x: u64, limit: u64) -> u64 {
    let mut temp = x;
    while temp >= limit {
        temp -= limit;
    }
    if temp == 0 {
        temp = x ^ 0xFFFFFFFFFFFFFFFF;
        while temp >= limit {
            temp -= limit;
        }
    }
    temp
}

fn mwc64_next(x: u64, multiplier: u64) -> u64 {
    (x & 0xFFFFFFFF).wrapping_mul(multiplier).wrapping_add(x >> 32)
}

impl MWC64 {
    pub fn new(seed1: u32, seed2: u32) -> MWC64 {
        MWC64 {
            mwc: (((seed1 as u64) << 32) ^ (seed2 as u64)),
        }
    }
    fn sanitise(&mut self) {
        self.mwc = mwc64_sanitise(self.mwc, 0x29A65EACFFFFFFFF);
    }
    fn next_mwc(&mut self) {
        self.mwc = mwc64_next(self.mwc, 698769069);
    }
    fn current(&self) -> u32 {
        self.mwc as u32
    }
}
impl RngCore for MWC64 {
    fn next_u32(&mut self) -> u32 {
        self.sanitise();

        self.next_mwc();

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


/* KISS2 ---------------------------------------------------------------------*/

#[derive(Debug)]
pub struct KISS2 {
    mwc: MWC64,
    cong: Cong,
    shr3: SHR3,
}

impl KISS2 {
    pub fn new(seed1: u32, seed2: u32, seed3: u32, seed4: u32) -> KISS2 {
        KISS2 {
            mwc: MWC64::new(seed1, seed2),
            cong: Cong::new(seed3),
            shr3: SHR3::new(seed4),
        }
    }
    fn current(&self) -> u32 {
        self.mwc.current().wrapping_add(self.cong.cong).wrapping_add(self.shr3.shr3)
    }
}
impl RngCore for KISS2 {
    fn next_u32(&mut self) -> u32 {
        self.mwc.next_u32();
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

fn lfsr_seed_z(seed: u32) -> u32 {
    seed ^ (seed << 16)
}

fn lfsr_sanitise_z(z: u32, min_value: u32) -> u32 {
    if z < min_value {
        z ^ 0xFFFFFFFF
    } else {
        z
    }
}

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
    const Z1_MIN: u32 = 2;
    const Z2_MIN: u32 = 8;
    const Z3_MIN: u32 = 16;

    pub fn new(seed1: u32, seed2: u32, seed3: u32) -> LFSR88 {
        LFSR88 {
            z1: lfsr_seed_z(seed1),
            z2: lfsr_seed_z(seed2),
            z3: lfsr_seed_z(seed3),
        }
    }
    fn sanitise_z1(&mut self) {
        self.z1 = lfsr_sanitise_z(self.z1, LFSR88::Z1_MIN);
    }
    fn sanitise_z2(&mut self) {
        self.z2 = lfsr_sanitise_z(self.z2, LFSR88::Z2_MIN);
    }
    fn sanitise_z3(&mut self) {
        self.z3 = lfsr_sanitise_z(self.z3, LFSR88::Z3_MIN);
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
        self.sanitise_z1();
        self.next_z1();
        self.sanitise_z2();
        self.next_z2();
        self.sanitise_z3();
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
    const Z1_MIN: u32 = 2;
    const Z2_MIN: u32 = 8;
    const Z3_MIN: u32 = 16;
    const Z4_MIN: u32 = 128;

    pub fn new(seed1: u32, seed2: u32, seed3: u32, seed4: u32) -> LFSR113 {
        LFSR113 {
            z1: lfsr_seed_z(seed1),
            z2: lfsr_seed_z(seed2),
            z3: lfsr_seed_z(seed3),
            z4: lfsr_seed_z(seed4),
        }
    }
    fn sanitise_z1(&mut self) {
        self.z1 = lfsr_sanitise_z(self.z1, LFSR113::Z1_MIN);
    }
    fn sanitise_z2(&mut self) {
        self.z2 = lfsr_sanitise_z(self.z2, LFSR113::Z2_MIN);
    }
    fn sanitise_z3(&mut self) {
        self.z3 = lfsr_sanitise_z(self.z3, LFSR113::Z3_MIN);
    }
    fn sanitise_z4(&mut self) {
        self.z4 = lfsr_sanitise_z(self.z4, LFSR113::Z4_MIN);
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
        self.sanitise_z1();
        self.next_z1();
        self.sanitise_z2();
        self.next_z2();
        self.sanitise_z3();
        self.next_z3();
        self.sanitise_z4();
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
