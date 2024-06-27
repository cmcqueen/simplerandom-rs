//! Simple pseudo-random number generators
//!
//! ## Features
//!
//! * Main API functions:
//!     * Seed
//!     * Generate "next" random value
//!     * "Jump-ahead" (also known as "discard" in C++) to skip the generator
//!       ahead by 'n' samples.
//! * Simple algorithms that are easily ported to different languages.
//! * Safe seeding. Many generators have some "bad" state values that must be avoided. The seed
//!   functions for all generators ensure that any "bad" state values are avoided, and replaced by
//!   a suitable alternative initial state.
//! * These random number generators have been implemented in the following languages:
//!     * C
//!     * Python
//!     * Rust
//! * Same numeric output in each supported language. It can be useful to be able to implement the
//!   identical algorithm on muliple platforms and/or languages.
//! * Simple algorithms and state size appropriate for limited RAM and ROM (e.g. in embedded
//!   systems).
//! * Decent cross-platform support.
//!     * Various OS.
//!     * Various processors, 8- to 64-bit.
//! * Implement target language's API idioms and/or existing random number generator API.
//! * Reasonable statistical properties of pseudo-random output (though not for all generators
//!   provided).
//!
//! ## Algorithms
//!
//! Most algorithms were obtained from two newsgroup posts by George Marsaglia [[mars1]](#mars1)
//! [[mars2]](#mars2). However, some modifications have been made. From [[rose1]](#rose1), it seems
//! that the SHR3 algorithm defined in [[mars1]](#mars1) is flawed and should not be used. It
//! doesn't actually have a period of 2<sup>32</sup>-1 as expected, but has 64 different cycles,
//! some with very short periods. The SHR3 in the 2003 post is very similar, but with two shift
//! values swapped. It has a period of 2<sup>32</sup>-1 as expected.
//!
//! We still find KISS from [[mars1]](#mars1) useful mainly because it uses 32-bit calculations for
//! MWC, which can be more suitable for small embedded systems. So we define KISS that uses a MWC
//! based on [[mars1]](#mars1), but the Cong and SHR3 from [[mars2]](#mars2).
//!
//! From Pierre L'Ecuyer [[lecuyer1]](#lecuyer1) [[lecuyer2]](#lecuyer2), the Combined LFSR
//! (Tausworthe) LFSR113 algorithm [[lecuyer3]](#lecuyer3) and LFSR88 (aka Taus88) have been
//! implemented.
//!
//! ### Random Number Generators Provided
//!
//! The following pseudo-random number generators are provided:
//!
//! | Generator   | Notes
//! | ----------- | --------------------------------------------------------------------------------------------------------------
//! | `MWC1`      | Two 32-bit MWCs combined. From [[mars1]](#mars1).
//! | `MWC2`      | Very similar to `MWC1`, but slightly modified to improve its statistical properties.
//! | `Cong`      | From [[mars2]](#mars2).
//! | `SHR3`      | From [[mars2]](#mars2).
//! | `MWC64`     | A single 64-bit multiply-with-carry calculation. From [[mars2]](#mars2).
//! | `KISS`      | Combination of MWC2, Cong and SHR3. Based on [[mars1]](#mars1) but using Cong and SHR3 from [[mars2]](#mars2), and the modified MWC.
//! | `KISS2`     | Combination of MWC64, Cong and SHR3. From [[mars2]](#mars2).
//! | `LFSR113`   | Combined LFSR (Tausworthe) random number generator by L'Ecuyer. From [[lecuyer1]](#lecuyer1) [[lecuyer3]](#lecuyer3).
//! | `LFSR88`    | Combined LFSR (Tausworthe) random number generator by L'Ecuyer. From [[lecuyer2]](#lecuyer2).
//!
//! ## License
//!
//! The code is released under the MIT license. See LICENSE.txt for details.
//!
//! ## References
//!
//! <a name="mars1">\[mars1\]</a>  
//! [Random Numbers for C: End, at last?](http://www.cse.yorku.ca/~oz/marsaglia-rng.html)  
//! George Marsaglia  
//! Newsgroup post, sci.stat.math and others, Thu, 21 Jan 1999
//!
//! <a name="mars2">\[mars2\]</a>  
//! [RNGs](http://groups.google.com/group/sci.math/msg/9959175f66dd138f)  
//! George Marsaglia  
//! Newsgroup post, sci.math, 26 Feb 2003
//!
//! <a name="rose1">\[rose1\]</a>  
//! [KISS: A Bit Too Simple](http://eprint.iacr.org/2011/007.pdf)  
//! Greg Rose  
//! Qualcomm Inc.
//!
//! <a name="lecuyer1">\[lecuyer1\]</a>  
//! [Tables of Maximally-Equidistributed Combined LFSR Generators](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.43.3639)  
//! Pierre L'Ecuyer  
//! Mathematics of Computation, 68, 225 (1999), 261–269.
//!
//! <a name="lecuyer2">\[lecuyer2\]</a>  
//! [Maximally Equidistributed Combined Tausworthe Generators](http://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.43.4155)  
//! P. L'Ecuyer  
//! Mathematics of Computation, 65, 213 (1996), 203–213.
//!
//! <a name="lecuyer3">\[lecuyer3\]</a>  
//! [LFSR113 C double implementation](http://www.iro.umontreal.ca/~simardr/rng/lfsr113.c)  
//! Pierre L'Ecuyer

use rand_core::{RngCore, Error, impls};
use num_traits::{PrimInt, Unsigned, WrappingAdd, WrappingMul, Pow};
use std::ops::SubAssign;

pub mod maths;
pub mod bitcolumnmatrix;

pub trait RngJumpAhead {
    fn jumpahead<N>(&mut self, n: N)
        where N: maths::IntTypes;
}

type BitColumnMatrix32 = bitcolumnmatrix::BitColumnMatrix::<u32, 32>;


/* Cong ----------------------------------------------------------------------*/

/// Cong -- Congruential random number generator
///
/// This is a congruential generator with the widely used 69069 multiplier:
/// x[n]=69069x[n-1]+12345.
/// It has period 2^32.
///
/// Marsaglia says: The leading half of its 32 bits seem to pass tests, but bits in the last half
/// are too regular. It fails tests for which those bits play a significant role. But keep in mind
/// that it is a rare application for which the trailing bits play a significant role. Cong is one
/// of the most widely used generators of the last 30 years, as it was the system generator for VAX
/// and was incorporated in several popular software packages, all seemingly without complaint.
///
///     use rand_core::RngCore;
///     use simplerandom::RngJumpAhead;
///     let mut s = simplerandom::Cong::new(1);
///     let r = s.next_u32();
///     assert_eq!(r, 81414);
///     let r = s.next_u32();
///     assert_eq!(r, 1328228615);
///     let r = s.next_u32();
///     assert_eq!(r, 3215746516);
///     let r = s.next_u32();
///     assert_eq!(r, 2752347901);
///     s.jumpahead(1_000_000);
///     let r = s.next_u32();
///     assert_eq!(r, 2250891922);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cong {
    cong: u32,
}

impl Cong {
    const M: u32 = 69069;
    const C: u32 = 12345;
    const CYCLE_LEN: u64 = 1 << 32;

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
        where N: maths::IntTypes
    {
        let n_mod = maths::modulo(n, Cong::CYCLE_LEN);
        let mult_exp = maths::wrapping_pow(Cong::M, n_mod);
        let add_const = maths::wrapping_geom_series(Cong::M, n_mod).wrapping_mul(Cong::C);
        let cong = mult_exp.wrapping_mul(self.cong).wrapping_add(add_const);
        self.cong = cong;
    }
}


/* SHR3 ----------------------------------------------------------------------*/

/// SHR3 -- 3-shift-register random number generator
///
/// Reading between the lines, I believe the SHR3 defined in Marsaglia's 1999 post actually has a
/// typo: the shift values defined don't actually produce a period of 2^32-1, but have 64 possible
/// cycles, some extremely short. But the swapped values from Marsaglia's 2003 post produce the
/// full 2^32-1 period. So we use that definition of SHR3.
///
/// SHR3 is a 3-shift-register generator with period 2^32-1. It uses
/// y[n]=y[n-1](I+L^13)(I+R^17)(I+L^5),
/// with the y's viewed as binary vectors, L the 32x32 binary matrix that shifts a vector left 1,
/// and R its transpose.
///
/// Marsaglia says: SHR3 seems to pass all except those related to the binary rank test, since 32
/// successive values, as binary vectors, must be linearly independent, while 32 successive truly
/// random 32-bit integers, viewed as binary vectors, will be linearly independent only about 29%
/// of the time.
///
///     use rand_core::RngCore;
///     use simplerandom::RngJumpAhead;
///     let mut s = simplerandom::SHR3::new(1);
///     let r = s.next_u32();
///     assert_eq!(r, 270369);
///     let r = s.next_u32();
///     assert_eq!(r, 67634689);
///     let r = s.next_u32();
///     assert_eq!(r, 2647435461);
///     let r = s.next_u32();
///     assert_eq!(r, 307599695);
///     s.jumpahead(1_000_000);
///     let r = s.next_u32();
///     assert_eq!(r, 1105614340);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SHR3 {
    shr3: u32,
}

impl SHR3 {
    const CYCLE_LEN: u32 = 0xFFFFFFFF;

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
        where N: maths::IntTypes
    {
        const SHR3_MATRIX_ARRAY: [u32; 32] = [
            0x00042021, 0x00084042, 0x00108084, 0x00210108, 0x00420231, 0x00840462, 0x010808C4, 0x02101188,
            0x04202310, 0x08404620, 0x10808C40, 0x21011880, 0x42023100, 0x84046200, 0x0808C400, 0x10118800,
            0x20231000, 0x40462021, 0x808C4042, 0x01080084, 0x02100108, 0x04200210, 0x08400420, 0x10800840,
            0x21001080, 0x42002100, 0x84004200, 0x08008400, 0x10010800, 0x20021000, 0x40042000, 0x80084000,
        ];
        let n_mod = maths::modulo(n, SHR3::CYCLE_LEN);
        self.sanitise();
        let shr3_matrix = BitColumnMatrix32::new(&SHR3_MATRIX_ARRAY);
        let shr3_mult = shr3_matrix.pow(n_mod);
        self.shr3 = shr3_mult.dot_vec(self.shr3);
    }
}


/* MWC2 ----------------------------------------------------------------------*/

/// MWC2 -- "Multiply-with-carry" random number generator
///
/// Very similar to MWC1, except that it concatenates the two 16-bit MWC generators differently.
/// The 'x' generator is rotated 16 bits instead of just shifted 16 bits.
///
/// This gets much better test results than MWC1 in L'Ecuyer's TestU01 test suite, so it should
/// probably be preferred.
///
///     use rand_core::RngCore;
///     use simplerandom::RngJumpAhead;
///     let mut s = simplerandom::MWC2::new(1, 2);
///     let r = s.next_u32();
///     assert_eq!(r, 2422836384);
///     let r = s.next_u32();
///     assert_eq!(r, 1907426166);
///     let r = s.next_u32();
///     assert_eq!(r, 3696423159);
///     let r = s.next_u32();
///     assert_eq!(r, 560799047);
///     s.jumpahead(1_000_000_000_000_i64);
///     let r = s.next_u32();
///     assert_eq!(r, 1656033328);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MWC2 {
    upper: u32,
    lower: u32,
}

fn mwc_next<T>(x: T, multiplier: T) -> T
    where T: PrimInt + Unsigned + WrappingAdd + WrappingMul
{
    let width_bits = maths::size_of_bits::<T>();
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
    const UPPER_CYCLE_LEN: u32 = (MWC2::UPPER_M << 16) / 2 - 1;
    const LOWER_CYCLE_LEN: u32 = (MWC2::LOWER_M << 16) / 2 - 1;

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
        where N: maths::IntTypes
    {
        let n_upper = maths::modulo(n, MWC2::UPPER_CYCLE_LEN);
        let n_lower = maths::modulo(n, MWC2::LOWER_CYCLE_LEN);

        self.sanitise();
        self.upper = maths::mul_mod(maths::pow_mod(MWC2::UPPER_M, n_upper, MWC2::UPPER_MOD), self.upper, MWC2::UPPER_MOD);
        self.lower = maths::mul_mod(maths::pow_mod(MWC2::LOWER_M, n_lower, MWC2::LOWER_MOD), self.lower, MWC2::LOWER_MOD);
    }
}


/* MWC1 ----------------------------------------------------------------------*/

/// MWC1 -- "Multiply-with-carry" random number generator
///
/// This is the MWC as defined in Marsaglia's 1999 newsgroup post.
///
/// It uses two MWC generators to generate high and low 16-bit parts, which are then combined to
/// make a 32-bit value.
///
/// The MWC generator concatenates two 16-bit multiply-with-carry generators:
///
/// x[n]=36969x[n-1]+carry,
/// y[n]=18000y[n-1]+carry mod 2^16,
///
/// It has a period about 2^60.
///
/// This seems to pass all Marsaglia's Diehard tests. However, it fails many of L'Ecuyer's TestU01
/// tests. The modified MWC2 generator passes many more tests in TestU01, and should probably be
/// preferred, unless backwards compatibility is required.
///
///     use rand_core::RngCore;
///     use simplerandom::RngJumpAhead;
///     let mut s = simplerandom::MWC1::new(1, 2);
///     let r = s.next_u32();
///     assert_eq!(r, 2422836384);
///     let r = s.next_u32();
///     assert_eq!(r, 1907405312);
///     let r = s.next_u32();
///     assert_eq!(r, 3696412319);
///     let r = s.next_u32();
///     assert_eq!(r, 560774291);
///     s.jumpahead(1_000_000_000_000_u64);
///     let r = s.next_u32();
///     assert_eq!(r, 1656029560);
#[derive(Debug, Clone, Eq, PartialEq)]
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
        where N: maths::IntTypes
    {
        self.mwc.jumpahead(n);
    }
}


/* KISS ----------------------------------------------------------------------*/

/// KISS -- "Keep It Simple Stupid" random number generator
///
/// It combines the MWC2, Cong, SHR3 generators. Period is about 2^123.
///
/// This is based on, but not identical to, Marsaglia's KISS generator as defined in his 1999
/// newsgroup post. That generator most significantly has problems with its SHR3 component (see
/// notes on SHR3). Since we are not keeping compatibility with the 1999 KISS generator for that
/// reason, we take the opportunity to slightly update the MWC and Cong generators too.
///
///     use rand_core::RngCore;
///     use simplerandom::RngJumpAhead;
///     let mut s = simplerandom::KISS::new(1, 2, 3, 4);
///     let r = s.next_u32();
///     assert_eq!(r, 2424001924);
///     let r = s.next_u32();
///     assert_eq!(r, 107884083);
///     let r = s.next_u32();
///     assert_eq!(r, 623570573);
///     let r = s.next_u32();
///     assert_eq!(r, 538815332);
///     s.jumpahead(1_000_000_000_000_000_000_i64);
///     let r = s.next_u32();
///     assert_eq!(r, 101779707);
#[derive(Debug, Clone, Eq, PartialEq)]
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
        where N: maths::IntTypes
    {
        self.mwc.jumpahead(n);
        self.cong.jumpahead(n);
        self.shr3.jumpahead(n);
    }
}


/* MWC64 ---------------------------------------------------------------------*/

/// MWC64 -- "Multiply-with-carry" random number generator
///
/// This is a different MWC generator design, from the newsgroup post in 2003.
///
/// This uses a single MWC generator with a 64-bit calculation to generate a 32-bit value. The
/// seeds should still be 32-bit values.
///
///     use rand_core::RngCore;
///     use simplerandom::RngJumpAhead;
///     let mut s = simplerandom::MWC64::new(1, 2);
///     let r = s.next_u32();
///     assert_eq!(r, 1397538139);
///     let r = s.next_u32();
///     assert_eq!(r, 3563413631);
///     let r = s.next_u32();
///     assert_eq!(r, 3101181111);
///     let r = s.next_u32();
///     assert_eq!(r, 1402594920);
///     s.jumpahead(1_000_000_000_000_000_000_i64);
///     let r = s.next_u32();
///     assert_eq!(r, 655025777);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MWC64 {
    mwc: u64,
}

impl MWC64 {
    const M: u64 = 698769069;
    const MOD: u64 = (MWC64::M << 32) - 1;
    const CYCLE_LEN: u64 = (MWC64::M << 32) / 2 - 1;

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
        where N: maths::IntTypes
    {
        let n_mod = maths::modulo(n, MWC64::CYCLE_LEN);
        self.sanitise();
        self.mwc = maths::mul_mod(maths::pow_mod(MWC64::M, n_mod, MWC64::MOD), self.mwc, MWC64::MOD);
    }
}


/* KISS2 ---------------------------------------------------------------------*/

/// KISS2 -- "Keep It Simple Stupid" random number generator
///
/// It combines the MWC64, Cong, SHR3 generators. Period is about 2^123.
///
/// This is a slightly updated KISS generator design, from the newsgroup post in 2003. The MWC
/// component uses a single 64-bit calculation, instead of two 32-bit calculations that are
/// combined. The seeds should still be 32-bit values.
///
///     use rand_core::RngCore;
///     use simplerandom::RngJumpAhead;
///     let mut s = simplerandom::KISS2::new(1, 2, 3, 4);
///     let r = s.next_u32();
///     assert_eq!(r, 1398839167);
///     let r = s.next_u32();
///     assert_eq!(r, 1816301020);
///     let r = s.next_u32();
///     assert_eq!(r, 431342393);
///     let r = s.next_u32();
///     assert_eq!(r, 2488040595);
///     s.jumpahead(1_000_000_000_000_000_000_i64);
///     let r = s.next_u32();
///     assert_eq!(r, 666798061);
#[derive(Debug, Clone, Eq, PartialEq)]
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
        where N: maths::IntTypes
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

/// LFSR88 -- Combined LFSR random number generator by L'Ecuyer
///
/// It combines 3 LFSR generators. The generators have been chosen for maximal equidistribution.
///
/// The period is approximately 2^88.
///
/// "Tables of Maximally-Equidistributed Combined Lfsr Generators"
/// P. L'Ecuyer
/// Mathematics of Computation, 68, 225 (1999), 261–269.
///
///     use rand_core::RngCore;
///     use simplerandom::RngJumpAhead;
///     let mut s = simplerandom::LFSR88::new(1, 2, 3);
///     let r = s.next_u32();
///     assert_eq!(r, 270534496);
///     let r = s.next_u32();
///     assert_eq!(r, 75498003);
///     let r = s.next_u32();
///     assert_eq!(r, 539432965);
///     let r = s.next_u32();
///     assert_eq!(r, 536871567);
///     s.jumpahead(1_000_000_000_000_000_000_i64);
///     let r = s.next_u32();
///     assert_eq!(r, 1799734280);
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LFSR88 {
    z1: u32,
    z2: u32,
    z3: u32,
}

impl LFSR88 {
    const Z1_MIN: u32 = 2;
    const Z2_MIN: u32 = 8;
    const Z3_MIN: u32 = 16;
    const Z1_CYCLE_LEN: u32 = (1 << (32 - 1)) - 1;
    const Z2_CYCLE_LEN: u32 = (1 << (32 - 3)) - 1;
    const Z3_CYCLE_LEN: u32 = (1 << (32 - 4)) - 1;

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
        where N: maths::IntTypes
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

        let n_z1 = maths::modulo(n, LFSR88::Z1_CYCLE_LEN);
        self.sanitise_z1();
        let lfsr88_matrix = BitColumnMatrix32::new(&LFSR88_Z1_MATRIX_ARRAY);
        let lfsr88_mult = lfsr88_matrix.pow(n_z1);
        self.z1 = lfsr88_mult.dot_vec(self.z1);

        let n_z2 = maths::modulo(n, LFSR88::Z2_CYCLE_LEN);
        self.sanitise_z2();
        let lfsr88_matrix = BitColumnMatrix32::new(&LFSR88_Z2_MATRIX_ARRAY);
        let lfsr88_mult = lfsr88_matrix.pow(n_z2);
        self.z2 = lfsr88_mult.dot_vec(self.z2);

        let n_z3 = maths::modulo(n, LFSR88::Z3_CYCLE_LEN);
        self.sanitise_z3();
        let lfsr88_matrix = BitColumnMatrix32::new(&LFSR88_Z3_MATRIX_ARRAY);
        let lfsr88_mult = lfsr88_matrix.pow(n_z3);
        self.z3 = lfsr88_mult.dot_vec(self.z3);
    }
}


/* LFSR113 -------------------------------------------------------------------*/

/// LFSR113 -- Combined LFSR random number generator by L'Ecuyer
///
/// It combines 4 LFSR generators. The generators have been chosen for maximal equidistribution.
///
/// The period is approximately 2^113.
///
/// "Tables of Maximally-Equidistributed Combined Lfsr Generators"
/// P. L'Ecuyer
/// Mathematics of Computation, 68, 225 (1999), 261–269.
///
///     use rand_core::RngCore;
///     use simplerandom::RngJumpAhead;
///     let mut s = simplerandom::LFSR113::new(1, 2, 3, 4);
///     let r = s.next_u32();
///     assert_eq!(r, 2173174600);
///     let r = s.next_u32();
///     assert_eq!(r, 3360260106);
///     let r = s.next_u32();
///     assert_eq!(r, 5252608);
///     let r = s.next_u32();
///     assert_eq!(r, 1645469841);
///     s.jumpahead(1_000_000_000_000_000_000_i64);
///     let r = s.next_u32();
///     assert_eq!(r, 2384440028);
#[derive(Debug, Clone, Eq, PartialEq)]
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
    const Z1_CYCLE_LEN: u32 = (1 << (32 - 1)) - 1;
    const Z2_CYCLE_LEN: u32 = (1 << (32 - 3)) - 1;
    const Z3_CYCLE_LEN: u32 = (1 << (32 - 4)) - 1;
    const Z4_CYCLE_LEN: u32 = (1 << (32 - 7)) - 1;

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
        where N: maths::IntTypes
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

        let n_z1 = maths::modulo(n, LFSR113::Z1_CYCLE_LEN);
        self.sanitise_z1();
        let lfsr113_matrix = BitColumnMatrix32::new(&LFSR113_Z1_MATRIX_ARRAY);
        let lfsr113_mult = lfsr113_matrix.pow(n_z1);
        self.z1 = lfsr113_mult.dot_vec(self.z1);

        let n_z2 = maths::modulo(n, LFSR113::Z2_CYCLE_LEN);
        self.sanitise_z2();
        let lfsr113_matrix = BitColumnMatrix32::new(&LFSR113_Z2_MATRIX_ARRAY);
        let lfsr113_mult = lfsr113_matrix.pow(n_z2);
        self.z2 = lfsr113_mult.dot_vec(self.z2);

        let n_z3 = maths::modulo(n, LFSR113::Z3_CYCLE_LEN);
        self.sanitise_z3();
        let lfsr113_matrix = BitColumnMatrix32::new(&LFSR113_Z3_MATRIX_ARRAY);
        let lfsr113_mult = lfsr113_matrix.pow(n_z3);
        self.z3 = lfsr113_mult.dot_vec(self.z3);

        let n_z4 = maths::modulo(n, LFSR113::Z4_CYCLE_LEN);
        self.sanitise_z4();
        let lfsr113_matrix = BitColumnMatrix32::new(&LFSR113_Z4_MATRIX_ARRAY);
        let lfsr113_mult = lfsr113_matrix.pow(n_z4);
        self.z4 = lfsr113_mult.dot_vec(self.z4);
    }
}
