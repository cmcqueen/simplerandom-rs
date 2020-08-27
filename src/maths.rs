
use num_traits::{PrimInt, Signed, Unsigned, WrappingMul, WrappingAdd, WrappingSub, WrappingNeg, Zero, One, NumCast};
use std::ops::{AddAssign, BitAnd, MulAssign};

pub trait UIntTypes: PrimInt + Unsigned + WrappingAdd + WrappingSub + Zero + One + Copy
{
    fn mul_mod(a: Self, b: Self, m: Self) -> Self;
}

pub fn mul_mod<T>(a: T, b: T, m: T) -> T
    where T: UIntTypes
{
    let mut a_work: T = a;
    let mut b_work: T = b;
    let mut result: T = T::zero();

    if b_work >= m {
        if m > T::max_value() / (T::one() + T::one()) {
            b_work = b_work.wrapping_sub(&m);
        } else {
            b_work = b_work % m;
        }
    }

    while a_work != T::zero() {
        if a_work & T::one() != T::zero() {
            if b_work >= m - result {
                result = result.wrapping_sub(&m);
            }
            result = result.wrapping_add(&b_work);
        }
        a_work = a_work >> 1;

        let mut temp_b = b_work;
        if b_work >= m - temp_b {
            temp_b = temp_b.wrapping_sub(&m);
        }
        b_work = b_work.wrapping_add(&temp_b);
    }
    result
}

impl UIntTypes for u8 {
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        ((a as u16) * (b as u16) % (m as u16)) as u8
    }
}
impl UIntTypes for u16 {
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        ((a as u32) * (b as u32) % (m as u32)) as u16
    }
}
impl UIntTypes for u32 {
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        ((a as u64) * (b as u64) % (m as u64)) as u32
    }
}
impl UIntTypes for u64 {
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        ((a as u128) * (b as u128) % (m as u128)) as u64
    }
}
impl UIntTypes for u128 {
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        mul_mod::<Self>(a, b, m)
    }
}

pub trait IntTypes: PrimInt + NumCast + Zero + WrappingAdd + WrappingNeg + Copy
{
    type SignedType: PrimInt + Signed + Zero + One + Copy + NumCast;
    type UnsignedType: PrimInt + Unsigned + Zero + One + Copy + NumCast + WrappingNeg;
    type OtherSignType: PrimInt + Zero + One + Copy + NumCast;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType;
}

pub fn abs_as_unsigned<T>(a: T) -> T::UnsignedType
    where T: IntTypes
{
    if a < T::zero() {
        // Negative input. Negate it.
        let result: Option<T::UnsignedType> = NumCast::from(a.wrapping_neg());
        if result.is_some() {
            // Normal case.
            result.unwrap_or(T::UnsignedType::zero())
        } else {
            // The exceptional case: the lowest negative number,
            let result_minus_1: Option<T::UnsignedType> = NumCast::from((a + T::one()).wrapping_neg());
            result_minus_1.unwrap_or(T::UnsignedType::zero()) + T::UnsignedType::one()
        }
    } else {
        let result: Option<T::UnsignedType> = NumCast::from(a);
        result.unwrap_or(T::UnsignedType::zero())
    }
}

impl IntTypes for i8 {
    type SignedType = i8;
    type UnsignedType = u8;
    type OtherSignType = u8;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        abs_as_unsigned::<Self>(a)
    }
}
impl IntTypes for i16 {
    type SignedType = i16;
    type UnsignedType = u16;
    type OtherSignType = u16;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        abs_as_unsigned::<Self>(a)
    }
}
impl IntTypes for i32 {
    type SignedType = i32;
    type UnsignedType = u32;
    type OtherSignType = u32;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        abs_as_unsigned::<Self>(a)
    }
}
impl IntTypes for i64 {
    type SignedType = i64;
    type UnsignedType = u64;
    type OtherSignType = u64;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        abs_as_unsigned::<Self>(a)
    }
}
impl IntTypes for i128 {
    type SignedType = i128;
    type UnsignedType = u128;
    type OtherSignType = u128;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        abs_as_unsigned::<Self>(a)
    }
}
impl IntTypes for isize {
    type SignedType = isize;
    type UnsignedType = usize;
    type OtherSignType = usize;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        abs_as_unsigned::<Self>(a)
    }
}
impl IntTypes for u8 {
    type SignedType = i8;
    type UnsignedType = u8;
    type OtherSignType = i8;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        a
    }
}
impl IntTypes for u16 {
    type SignedType = i16;
    type UnsignedType = u16;
    type OtherSignType = i16;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        a
    }
}
impl IntTypes for u32 {
    type SignedType = i32;
    type UnsignedType = u32;
    type OtherSignType = i32;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        a
    }
}
impl IntTypes for u64 {
    type SignedType = i64;
    type UnsignedType = u64;
    type OtherSignType = i64;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        a
    }
}
impl IntTypes for u128 {
    type SignedType = i128;
    type UnsignedType = u128;
    type OtherSignType = i128;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        a
    }
}
impl IntTypes for usize {
    type SignedType = isize;
    type UnsignedType = usize;
    type OtherSignType = isize;
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType {
        a
    }
}

pub fn modulo<A, M>(a: A, m: M) -> M
    where A: IntTypes,
        M: PrimInt + Unsigned + Zero + Copy + NumCast
{
    if a >= A::zero() {
        // Unsigned input.
        let a_opt: Option<M> = NumCast::from(a);
        if a_opt.is_some() {
            // a fits into type M. Easy.
            a_opt.unwrap() % m
        } else {
            // a doesn't fit into type M. m should fit into type A.
            let m_opt: Option<A> = NumCast::from(m);
            let result_a = a % m_opt.unwrap();
            let result_m: Option<M> = NumCast::from(result_a);
            result_m.unwrap()
        }
    } else {
        // Signed input.
        let a_abs = IntTypes::abs_as_unsigned(a);
        let a_abs_opt: Option<M> = NumCast::from(a_abs);
        if a_abs_opt.is_some() {
            // a_abs fits into type M.
            m - (a_abs_opt.unwrap() % m)
        } else {
            // a_abs doesn't fit into type M. m should fit into the corresponding unsigned type of A.
            let m_opt: Option<A::UnsignedType> = NumCast::from(m);
            let m_s = m_opt.unwrap();
            let result_a = m_s - (a_abs % m_s);
            let result_m: Option<M> = NumCast::from(result_a);
            result_m.unwrap()
        }
    }
}

/*
 * Exponentiation with wrapping.
 *
 * Calculation of 'base' to the power of an unsigned integer 'n', with the
 * natural modulo of the unsigned integer type T (ie, with wrapping).
 */
pub fn wrapping_pow<T, N>(base: T, n: N) -> T
    where T: Unsigned + PrimInt + WrappingMul + WrappingSub + One,
          N: Unsigned + PrimInt + BitAnd + One + Zero,
{
    let mut result: T = T::one();
    let mut temp_exp = base;
    let mut n_work: N = n;

    loop {
        if n_work & N::one() != N::zero() {
            result = result.wrapping_mul(&temp_exp);
        }
        n_work = n_work >> 1;
        if n_work == N::zero() {
            break;
        }
        temp_exp = temp_exp.wrapping_mul(&temp_exp);
    }
    result
}

/*
 * Modular exponentiation.
 *
 * Calculation of 'base' to the power of an unsigned integer 'n',
 * modulo a value 'm'.
 */
pub fn pow_mod<T, N>(base: T, n: N, m: T) -> T
    where T: UIntTypes,
          N: Unsigned + PrimInt + BitAnd + One + Zero,
{
    let mut result: T = T::one();
    let mut temp_exp = base;
    let mut n_work: N = n;

    loop {
        if n_work & N::one() != N::zero() {
            result = T::mul_mod(result, temp_exp, m);
        }
        n_work = n_work >> 1;
        if n_work == N::zero() {
            break;
        }
        temp_exp = T::mul_mod(temp_exp, temp_exp, m);
    }
    result
}

/* Calculate geometric series:
 *     1 + r + r^2 + r^3 + ... r^(n-1)
 * summed to n terms, modulo 2^32.
 *
 * It makes use of the fact that the series can pair up terms:
 *     (1 + r) + (1 + r) r^2 + (1 + r) r^4 + ... + (1 + r) (r^2)^(n/2-1) + [ r^(n-1) if n is odd ]
 *     (1 + r) (1 + r^2 + r^4 + ... + (r^2)^(n/2-1)) + [ r^(n-1) if n is odd ]
 *
 * Which can easily be calculated by recursion, with time order O(log n), and
 * also stack depth O(log n). However that stack depth isn't good, so a
 * non-recursive implementation is preferable.
 * This implementation is by a loop, not recursion, with time order
 * O(log n) and stack depth O(1).
 */
pub fn wrapping_geom_series<T, N>(r: T, n: N) -> T
    where T: Unsigned + PrimInt + WrappingMul + WrappingAdd + WrappingSub + AddAssign + MulAssign + One,
          N: Unsigned + PrimInt + BitAnd + One + Zero,
{
    let mut temp_r = r;
    let mut mult = T::one();
    let mut result = T::zero();

    if n == N::zero() {
        return T::zero();
    }

    let mut n_work = n;
    while n_work > N::one() {
        if n_work & N::one() != N::zero() {
            result = wrapping_pow(temp_r, n_work - N::one()).wrapping_mul(&mult).wrapping_add(&result);
        }
        mult = (T::one().wrapping_add(&temp_r)).wrapping_mul(&mult);
        temp_r = temp_r.wrapping_mul(&temp_r);
        n_work = n_work >> 1;
    }
    result = result.wrapping_add(&mult);
    result
}
