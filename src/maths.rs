
use num_traits::{PrimInt, Unsigned, WrappingMul, WrappingAdd, WrappingSub, Zero, One};
use std::ops::{AddAssign, BitAnd, MulAssign};

pub fn mul_mod<T>(a: T, b: T, m: T) -> T
    where T: Unsigned + PrimInt + WrappingAdd + WrappingSub + One + Zero
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
    where T: Unsigned + PrimInt + WrappingAdd + WrappingSub + One,
          N: Unsigned + PrimInt + BitAnd + One + Zero,
{
    let mut result: T = T::one();
    let mut temp_exp = base;
    let mut n_work: N = n;

    loop {
        if n_work & N::one() != N::zero() {
            result = mul_mod::<T>(result, temp_exp, m);
        }
        n_work = n_work >> 1;
        if n_work == N::zero() {
            break;
        }
        temp_exp = mul_mod::<T>(temp_exp, temp_exp, m);
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
