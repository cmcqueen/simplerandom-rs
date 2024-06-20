
use num_traits::{PrimInt, Signed, Unsigned, WrappingMul, WrappingAdd, WrappingSub, WrappingNeg, Zero, One, NumCast};
use std::ops::{AddAssign, BitAnd, MulAssign};

/// Unsigned integer types
///
pub trait UIntTypes: PrimInt + Unsigned + WrappingAdd + WrappingSub + Zero + One + Copy
{
    /// Multiply unsigned `a` and `b`, modulo `m`
    ///
    /// It can be specialised for each integer type. See the comments on
    /// the generic mul_mod() implementation below.
    ///
    fn mul_mod(a: Self, b: Self, m: Self) -> Self;
}

/// Multiply unsigned `a` and `b`, modulo `m`
///
/// This is a generic implementation that should work for any unsigned
/// primitive integer type.
///
/// Note that for most integer types for which a larger integer type is
/// available for intermediate calculations, it may be faster to implement it
/// by doing a simple multiplication in the larger integer type, calculating
/// the modulo, then casting back to the result type. That can be done for
/// u8 through u64, but not u128 and perhaps not usize. See
/// UIntTypes::mul_mod() which can be specialised for each type.
///
/// # Arguments
///
/// Multiplicands `a` and `b` can be any unsigned primitive integer.
/// Modulus `m` can be any unsigned primitive integer.
///
/// # Return
///
/// The result is the multiplication of `a` and `b`, modulo `m`.
///
///     use simplerandom::maths::mul_mod;
///     let result = mul_mod(123456789_u32, 3111222333, 0x9068FFFF);
///     assert_eq!(1473911797_u32, result);
///     let result = mul_mod(12345678901234567890_u64, 10222333444555666777, 0x29A65EACFFFFFFFF);
///     assert_eq!(1000040008665797219_u64, result);
///
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
    /// Simple specialisation using the next larger integer type
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        ((a as u16) * (b as u16) % (m as u16)) as u8
    }
}
impl UIntTypes for u16 {
    /// Simple specialisation using the next larger integer type
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        ((a as u32) * (b as u32) % (m as u32)) as u16
    }
}
impl UIntTypes for u32 {
    /// Simple specialisation using the next larger integer type
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        ((a as u64) * (b as u64) % (m as u64)) as u32
    }
}
impl UIntTypes for u64 {
    /// Simple specialisation using the next larger integer type
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        ((a as u128) * (b as u128) % (m as u128)) as u64
    }
}
impl UIntTypes for u128 {
    /// Use the generic implementation
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        mul_mod::<Self>(a, b, m)
    }
}
impl UIntTypes for usize {
    /// Use the generic implementation
    fn mul_mod(a: Self, b: Self, m: Self) -> Self {
        mul_mod::<Self>(a, b, m)
    }
}

/// Primitive integer types
///
/// Mappings to associated signed and unsigned types with the same bit width.
///
pub trait IntTypes: PrimInt + NumCast + Zero + WrappingAdd + WrappingNeg + Copy
{
    type SignedType: PrimInt + Signed + Zero + One + Copy + NumCast;
    type UnsignedType: PrimInt + Unsigned + Zero + One + Copy + NumCast + WrappingNeg;
    type OtherSignType: PrimInt + Zero + One + Copy + NumCast;

    /// abs() function which returns a corresponding unsigned type
    ///
    /// For unsigned input types, just return the same value.
    /// For signed types, return the unsigned type of the same bit width.
    ///
    fn abs_as_unsigned(a: Self) -> Self::UnsignedType;
}

/// abs() function which returns a corresponding unsigned type
///
/// For unsigned input types, just return the same value.
/// For signed types, return the unsigned type of the same bit width.
///
/// This is a generic implementation which should work for all primitive
/// integers, both signed and unsigned.
pub fn abs_as_unsigned<T>(a: T) -> T::UnsignedType
    where T: IntTypes
{
    if a < T::zero() {
        // Negative input. Negate it.
        let result: Option<T::UnsignedType> = NumCast::from(a.wrapping_neg());
        if result.is_some() {
            // The vast majority of values.
            result.unwrap()
        } else {
            // The exceptional case: in two's complement form, the lowest
            // negative number's negation doesn't fit into the signed type.
            let result_minus_1: Option<T::UnsignedType> = NumCast::from((a + T::one()).wrapping_neg());
            result_minus_1.unwrap_or(T::UnsignedType::zero()) + T::UnsignedType::one()
        }
    } else {
        // Positive input. Return it as-is.
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

/// Calculate `a` modulo `m`
///
/// # Arguments
///
/// `a` can be any primitive integer, signed or unsigned.
/// `m` can be any unsigned primitive integer.
///
/// # Return
///
/// The result is the same unsigned type as that of parameter `m`.
/// The result is in the range [0..m] even when `a` is negative.
///
///     use simplerandom::maths::modulo;
///     let result = modulo(12345_u32, 7_u32);
///     assert_eq!(result, 4_u32);
///     let result = modulo(-12345_i32, 7_u32);
///     assert_eq!(result, 3_u32);
///
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

/// Exponentiation with wrapping
///
/// Calculation of `base` to the power of an unsigned integer `n`, with the
/// natural modulo of the unsigned integer type T (ie, with wrapping).
///
///     use simplerandom::maths::wrapping_pow;
///     let result = wrapping_pow(12345_u32, 1500000_u32);
///     assert_eq!(result, 2764689665_u32);
///
pub fn wrapping_pow<T, N>(base: T, n: N) -> T
    where T: PrimInt + Unsigned + WrappingMul + WrappingSub + One,
          N: PrimInt + Unsigned + BitAnd + One + Zero,
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

/// Modular exponentiation
///
/// Calculation of `base` to the power of an unsigned integer `n`,
/// modulo a value `m`.
///
///     use simplerandom::maths::pow_mod;
///     let result = pow_mod(12345_u32, 1500000_u32, 1211400191_u32);
///     assert_eq!(result, 348133782_u32);
///
pub fn pow_mod<T, N>(base: T, n: N, m: T) -> T
    where T: UIntTypes,
          N: PrimInt + Unsigned + BitAnd + One + Zero,
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

/// Calculate geometric series
///
/// That is, calculate the geometric series:
///
/// 1 + r + r^2 + r^3 + ... r^(n-1)
///
/// summed to `n` terms, with the natural modulo of the unsigned integer
/// type T (ie, with wrapping).
///
/// It makes use of the fact that the series can pair up terms:
///
/// (1 + r) + (1 + r) r^2 + (1 + r) r^4 + ... + (1 + r) (r^2)^(n/2-1) + [ r^(n-1) if n is odd ]
/// (1 + r) (1 + r^2 + r^4 + ... + (r^2)^(n/2-1)) + [ r^(n-1) if n is odd ]
///
/// Which can easily be calculated by recursion, with time order `O(log n)`, and
/// stack depth `O(log n)`. However that stack depth isn't good, so a
/// non-recursive implementation is preferable.
/// This implementation is by a loop, not recursion, with time order
/// `O(log n)` and stack depth `O(1)`.
///
///     use simplerandom::maths::wrapping_geom_series;
///     let result = wrapping_geom_series(12345_u32, 1500000_u32);
///     assert_eq!(result, 57634016_u32);
///
pub fn wrapping_geom_series<T, N>(r: T, n: N) -> T
    where T: PrimInt + Unsigned + WrappingMul + WrappingAdd + WrappingSub + AddAssign + MulAssign + One,
          N: PrimInt + Unsigned + BitAnd + One + Zero,
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
