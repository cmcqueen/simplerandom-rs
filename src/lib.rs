
use rand_core::{RngCore, Error, impls};
use num_traits::{PrimInt, Unsigned, WrappingAdd, WrappingMul};
use std::ops::SubAssign;

pub mod maths;
pub mod bitcolumnmatrix;

pub trait RngJumpAhead {
    fn jumpahead<N>(&mut self, n: N)
        where N: PrimInt + Unsigned;
}


/* Cong ----------------------------------------------------------------------*/

#[derive(Debug)]
pub struct Cong {
    cong: u32,
}

impl Cong {
    const M: u32 = 69069;
    const C: u32 = 12345;

    pub fn new(seed1: u32) -> Cong {
        Cong {
            cong: seed1,
        }
    }
}

impl RngCore for Cong {

    fn next_u32(&mut self) -> u32 {
        self.cong = self.cong.wrapping_mul(Cong::M).wrapping_add(Cong::C);
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

impl RngJumpAhead for Cong {
    fn jumpahead<N>(&mut self, n: N)
        where N: Unsigned + PrimInt
    {
        let mult_exp = maths::wrapping_pow(Cong::M, n);
        let add_const = maths::wrapping_geom_series(Cong::M, n).wrapping_mul(Cong::C);
        let cong = mult_exp.wrapping_mul(self.cong).wrapping_add(add_const);
        self.cong = cong;
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

impl RngJumpAhead for SHR3 {
    fn jumpahead<N>(&mut self, n: N)
        where N: Unsigned + PrimInt
    {
        const SHR3_MATRIX_ARRAY: [u32; 32] = [
            0x00042021, 0x00084042, 0x00108084, 0x00210108, 0x00420231, 0x00840462, 0x010808C4, 0x02101188,
            0x04202310, 0x08404620, 0x10808C40, 0x21011880, 0x42023100, 0x84046200, 0x0808C400, 0x10118800,
            0x20231000, 0x40462021, 0x808C4042, 0x01080084, 0x02100108, 0x04200210, 0x08400420, 0x10800840,
            0x21001080, 0x42002100, 0x84004200, 0x08008400, 0x10010800, 0x20021000, 0x40042000, 0x80084000,
        ];
        self.sanitise();
        let shr3_matrix = bitcolumnmatrix::BitColumnMatrix32::new(&SHR3_MATRIX_ARRAY);
        let shr3_mult = shr3_matrix.pow(n);
        self.shr3 = shr3_mult.dot_vec(self.shr3);
    }
}


/* MWC2 ----------------------------------------------------------------------*/

#[derive(Debug)]
pub struct MWC2 {
    upper: u32,
    lower: u32,
}

fn mwc_next<T>(x: T, multiplier: T) -> T
    where T: PrimInt + Unsigned + WrappingAdd + WrappingMul
{
    let width_bits = T::zero().count_zeros() as usize;
    let half_width_bits = width_bits / 2;
    let half_width_mask: T = T::max_value() >> half_width_bits;
    (x & half_width_mask).wrapping_mul(&multiplier).wrapping_add(&(x >> half_width_bits))
}

fn mwc_sanitise<T>(x: T, limit: T) -> T
    where T: PrimInt + Unsigned + SubAssign
{
    let mut temp = x;
    if temp >= limit {
        temp -= limit;
    }
    if temp == T::zero() {
        temp = x ^ T::max_value();
        if temp >= limit {
            temp -= limit;
        }
    }
    temp
}

impl MWC2 {
    const UPPER_M: u32 = 36969;
    const LOWER_M: u32 = 18000;
    const UPPER_MOD: u32 = (MWC2::UPPER_M << 16) - 1;
    const LOWER_MOD: u32 = (MWC2::LOWER_M << 16) - 1;

    pub fn new(seed1: u32, seed2: u32) -> MWC2 {
        MWC2 {
            upper: seed1,
            lower: seed2,
        }
    }
    fn sanitise(&mut self) {
        self.upper = mwc_sanitise(self.upper, MWC2::UPPER_MOD);
        self.lower = mwc_sanitise(self.lower, MWC2::LOWER_MOD);
    }
    fn current(&self) -> u32 {
        self.lower.wrapping_add(self.upper << 16).wrapping_add(self.upper >> 16)
    }
}
impl RngCore for MWC2 {
    fn next_u32(&mut self) -> u32 {
        self.sanitise();
        self.upper = mwc_next(self.upper, MWC2::UPPER_M);
        self.lower = mwc_next(self.lower, MWC2::LOWER_M);

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
impl RngJumpAhead for MWC2 {
    fn jumpahead<N>(&mut self, n: N)
        where N: Unsigned + PrimInt
    {
        self.sanitise();
        self.upper = maths::mul_mod(maths::pow_mod(MWC2::UPPER_M, n, MWC2::UPPER_MOD), self.upper, MWC2::UPPER_MOD);
        self.lower = maths::mul_mod(maths::pow_mod(MWC2::LOWER_M, n, MWC2::LOWER_MOD), self.lower, MWC2::LOWER_MOD);
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
impl RngJumpAhead for MWC1 {
    fn jumpahead<N>(&mut self, n: N)
        where N: Unsigned + PrimInt
    {
        self.mwc.jumpahead(n);
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
impl RngJumpAhead for KISS {
    fn jumpahead<N>(&mut self, n: N)
        where N: Unsigned + PrimInt
    {
        self.mwc.jumpahead(n);
        self.cong.jumpahead(n);
        self.shr3.jumpahead(n);
    }
}


/* MWC64 ---------------------------------------------------------------------*/

#[derive(Debug)]
pub struct MWC64 {
    mwc: u64,
}

impl MWC64 {
    const M: u64 = 698769069;
    const MOD: u64 = (MWC64::M << 32) - 1;

    pub fn new(seed1: u32, seed2: u32) -> MWC64 {
        MWC64 {
            mwc: (((seed1 as u64) << 32) ^ (seed2 as u64)),
        }
    }
    fn sanitise(&mut self) {
        self.mwc = mwc_sanitise(self.mwc, MWC64::MOD);
    }
    fn next_mwc(&mut self) {
        self.mwc = mwc_next(self.mwc, MWC64::M);
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
impl RngJumpAhead for MWC64 {
    fn jumpahead<N>(&mut self, n: N)
        where N: Unsigned + PrimInt
    {
        self.sanitise();
        self.mwc = maths::mul_mod(maths::pow_mod(MWC64::M, n, MWC64::MOD), self.mwc, MWC64::MOD);
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
impl RngJumpAhead for KISS2 {
    fn jumpahead<N>(&mut self, n: N)
        where N: Unsigned + PrimInt
    {
        self.mwc.jumpahead(n);
        self.cong.jumpahead(n);
        self.shr3.jumpahead(n);
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

fn lfsr_next_z(z: u32, a: u8, b: u8, c: u8, min_value: u32) -> u32 {
    let mask = 0xFFFFFFFF - (min_value - 1);
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
        self.z1 = lfsr_next_z(self.z1, 13, 19, 12, LFSR88::Z1_MIN);
    }
    fn next_z2(&mut self) {
        self.z2 = lfsr_next_z(self.z2, 2, 25, 4, LFSR88::Z2_MIN);
    }
    fn next_z3(&mut self) {
        self.z3 = lfsr_next_z(self.z3, 3, 11, 17, LFSR88::Z3_MIN);
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
impl RngJumpAhead for LFSR88 {
    fn jumpahead<N>(&mut self, n: N)
        where N: Unsigned + PrimInt
    {
        const LFSR88_Z1_MATRIX_ARRAY: [u32; 32] = [
            0x00000000, 0x00002000, 0x00004000, 0x00008000, 0x00010000, 0x00020000, 0x00040001, 0x00080002,
            0x00100004, 0x00200008, 0x00400010, 0x00800020, 0x01000040, 0x02000080, 0x04000100, 0x08000200,
            0x10000400, 0x20000800, 0x40001000, 0x80000001, 0x00000002, 0x00000004, 0x00000008, 0x00000010,
            0x00000020, 0x00000040, 0x00000080, 0x00000100, 0x00000200, 0x00000400, 0x00000800, 0x00001000
        ];
        const LFSR88_Z2_MATRIX_ARRAY: [u32; 32] = [
            0x00000000, 0x00000000, 0x00000000, 0x00000080, 0x00000100, 0x00000200, 0x00000400, 0x00000800,
            0x00001000, 0x00002000, 0x00004000, 0x00008000, 0x00010000, 0x00020000, 0x00040000, 0x00080000,
            0x00100000, 0x00200000, 0x00400000, 0x00800000, 0x01000000, 0x02000000, 0x04000000, 0x08000001,
            0x10000002, 0x20000005, 0x4000000A, 0x80000014, 0x00000028, 0x00000050, 0x00000020, 0x00000040
        ];
        const LFSR88_Z3_MATRIX_ARRAY: [u32; 32] = [
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00200000, 0x00400000, 0x00800000, 0x01000000,
            0x02000001, 0x04000002, 0x08000004, 0x10000009, 0x20000012, 0x40000024, 0x80000048, 0x00000090,
            0x00000120, 0x00000240, 0x00000480, 0x00000900, 0x00001200, 0x00002400, 0x00004800, 0x00009000,
            0x00012000, 0x00024000, 0x00048000, 0x00090000, 0x00120000, 0x00040000, 0x00080000, 0x00100000
        ];

        self.sanitise_z1();
        let lfsr88_matrix = bitcolumnmatrix::BitColumnMatrix32::new(&LFSR88_Z1_MATRIX_ARRAY);
        let lfsr88_mult = lfsr88_matrix.pow(n);
        self.z1 = lfsr88_mult.dot_vec(self.z1);

        self.sanitise_z2();
        let lfsr88_matrix = bitcolumnmatrix::BitColumnMatrix32::new(&LFSR88_Z2_MATRIX_ARRAY);
        let lfsr88_mult = lfsr88_matrix.pow(n);
        self.z2 = lfsr88_mult.dot_vec(self.z2);

        self.sanitise_z3();
        let lfsr88_matrix = bitcolumnmatrix::BitColumnMatrix32::new(&LFSR88_Z3_MATRIX_ARRAY);
        let lfsr88_mult = lfsr88_matrix.pow(n);
        self.z3 = lfsr88_mult.dot_vec(self.z3);
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
        self.z1 = lfsr_next_z(self.z1, 6, 13, 18, LFSR113::Z1_MIN);
    }
    fn next_z2(&mut self) {
        self.z2 = lfsr_next_z(self.z2, 2, 27, 2, LFSR113::Z2_MIN);
    }
    fn next_z3(&mut self) {
        self.z3 = lfsr_next_z(self.z3, 13, 21, 7, LFSR113::Z3_MIN);
    }
    fn next_z4(&mut self) {
        self.z4 = lfsr_next_z(self.z4, 3, 12, 13, LFSR113::Z4_MIN);
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
impl RngJumpAhead for LFSR113 {
    fn jumpahead<N>(&mut self, n: N)
        where N: Unsigned + PrimInt
    {
        const LFSR113_Z1_MATRIX_ARRAY: [u32; 32] = [
            0x00000000, 0x00080000, 0x00100000, 0x00200000, 0x00400000, 0x00800000, 0x01000000, 0x02000001,
            0x04000002, 0x08000004, 0x10000008, 0x20000010, 0x40000020, 0x80000041, 0x00000082, 0x00000104,
            0x00000208, 0x00000410, 0x00000820, 0x00001040, 0x00002080, 0x00004100, 0x00008200, 0x00010400,
            0x00020800, 0x00041000, 0x00002000, 0x00004000, 0x00008000, 0x00010000, 0x00020000, 0x00040000
        ];
        const LFSR113_Z2_MATRIX_ARRAY: [u32; 32] = [
            0x00000000, 0x00000000, 0x00000000, 0x00000020, 0x00000040, 0x00000080, 0x00000100, 0x00000200,
            0x00000400, 0x00000800, 0x00001000, 0x00002000, 0x00004000, 0x00008000, 0x00010000, 0x00020000,
            0x00040000, 0x00080000, 0x00100000, 0x00200000, 0x00400000, 0x00800000, 0x01000000, 0x02000000,
            0x04000000, 0x08000001, 0x10000002, 0x20000005, 0x4000000A, 0x80000014, 0x00000008, 0x00000010
        ];
        const LFSR113_Z3_MATRIX_ARRAY: [u32; 32] = [
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000800, 0x00001000, 0x00002000, 0x00004000,
            0x00008001, 0x00010002, 0x00020004, 0x00040008, 0x00080010, 0x00100020, 0x00200040, 0x00400080,
            0x00800100, 0x01000200, 0x02000400, 0x04000000, 0x08000000, 0x10000001, 0x20000002, 0x40000004,
            0x80000008, 0x00000010, 0x00000020, 0x00000040, 0x00000080, 0x00000100, 0x00000200, 0x00000400
        ];
        const LFSR113_Z4_MATRIX_ARRAY: [u32; 32] = [
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00100000,
            0x00200000, 0x00400001, 0x00800002, 0x01000004, 0x02000009, 0x04000012, 0x08000024, 0x10000048,
            0x20000090, 0x40000120, 0x80000240, 0x00000480, 0x00000900, 0x00001200, 0x00002400, 0x00004800,
            0x00009000, 0x00012000, 0x00024000, 0x00048000, 0x00090000, 0x00020000, 0x00040000, 0x00080000
        ];

        self.sanitise_z1();
        let lfsr113_matrix = bitcolumnmatrix::BitColumnMatrix32::new(&LFSR113_Z1_MATRIX_ARRAY);
        let lfsr113_mult = lfsr113_matrix.pow(n);
        self.z1 = lfsr113_mult.dot_vec(self.z1);

        self.sanitise_z2();
        let lfsr113_matrix = bitcolumnmatrix::BitColumnMatrix32::new(&LFSR113_Z2_MATRIX_ARRAY);
        let lfsr113_mult = lfsr113_matrix.pow(n);
        self.z2 = lfsr113_mult.dot_vec(self.z2);

        self.sanitise_z3();
        let lfsr113_matrix = bitcolumnmatrix::BitColumnMatrix32::new(&LFSR113_Z3_MATRIX_ARRAY);
        let lfsr113_mult = lfsr113_matrix.pow(n);
        self.z3 = lfsr113_mult.dot_vec(self.z3);

        self.sanitise_z4();
        let lfsr113_matrix = bitcolumnmatrix::BitColumnMatrix32::new(&LFSR113_Z4_MATRIX_ARRAY);
        let lfsr113_mult = lfsr113_matrix.pow(n);
        self.z4 = lfsr113_mult.dot_vec(self.z4);
    }
}
