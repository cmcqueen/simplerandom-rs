//! Square matrix of bits (Galois(2))
//!
//! The bits in the matrix are represented by an array of unsigned integers,
//! of a bit width that is suitable for the desired matrix dimensions.
//! Eg for a 32×32 matrix, an array of u32 of length 32 is used.

use num_traits::{One, Pow, PrimInt, Unsigned, Zero};
use std::ops::{BitAnd, Shl, Shr};

/// Shorthand for traits needed in `BitColumnMatrix`.
pub trait BitColumnMatrixInt: PrimInt + Unsigned + One + std::ops::BitXorAssign {}
impl<T: PrimInt + Unsigned + One + std::ops::BitXorAssign> BitColumnMatrixInt for T {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitColumnMatrix<T, const WIDTH: usize>
where
    T: BitColumnMatrixInt,
{
    columns: [T; WIDTH],
}

impl<T, const WIDTH: usize> BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    fn width_mask() -> T {
        if WIDTH < (T::one().count_zeros() as usize) {
            (T::one() << WIDTH) - T::one()
        } else {
            !(T::zero())
        }
    }

    pub fn new(init_data: &[T; WIDTH]) -> BitColumnMatrix<T, WIDTH> {
        BitColumnMatrix::<T, WIDTH> {
            columns: *init_data,
        }
    }

    pub fn shift(shift_value: i8) -> BitColumnMatrix<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH> {
            columns: [T::zero(); WIDTH],
        };
        let mut value: T = if shift_value >= 0 {
            T::one() << shift_value as usize
        } else {
            T::zero()
        };
        let mut shift_temp = shift_value;
        for i in 0..WIDTH {
            result.columns[i] = value;
            if shift_temp < 0 {
                shift_temp += 1;
                if shift_temp == 0 {
                    value = T::one();
                }
            } else {
                value = value << 1;
            }
        }
        result
    }

    pub fn dot_vec(&self, b: T) -> T {
        let mut result: T = T::zero();
        let mut b_temp = b;
        for i in 0..WIDTH {
            if b_temp & T::one() != T::zero() {
                result ^= self.columns[i];
            }
            b_temp = b_temp >> 1;
        }
        result
    }

    pub fn dot(&self, b: &BitColumnMatrix<T, WIDTH>) -> BitColumnMatrix<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = self.dot_vec(b.columns[i]);
        }
        result
    }

    pub fn dot_equ(&mut self, b: &BitColumnMatrix<T, WIDTH>) {
        let a = BitColumnMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            self.columns[i] = a.dot_vec(b.columns[i]);
        }
    }
}

impl<T, const WIDTH: usize> Zero for BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    /// Create a zero-matrix.
    fn zero() -> BitColumnMatrix<T, WIDTH> {
        BitColumnMatrix::<T, WIDTH> {
            columns: [T::zero(); WIDTH],
        }
    }

    fn is_zero(&self) -> bool {
        for i in 0..WIDTH {
            if self.columns[i] != T::zero() {
                return false;
            }
        }
        true
    }
}

impl<T, const WIDTH: usize> One for BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    /// Create a unity-matrix. That is, ones on the diagonal, zeros elsewhere.
    fn one() -> BitColumnMatrix<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH> {
            columns: [T::zero(); WIDTH],
        };
        let mut value: T = T::one();
        for i in 0..WIDTH {
            result.columns[i] = value;
            value = value << 1;
        }
        result
    }
}

impl<N, T, const WIDTH: usize> Pow<N> for BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
    N: Unsigned + PrimInt + BitAnd + One + Zero,
{
    type Output = Self;

    /// Raise a matrix to a power. Efficient matrix exponentiation.
    fn pow(self, n: N) -> BitColumnMatrix<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH>::one();
        let mut temp_exp = BitColumnMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        let mut n_work: N = n;

        loop {
            if n_work & N::one() != N::zero() {
                result.dot_equ(&temp_exp);
            }
            n_work = n_work >> 1;
            if n_work == N::zero() {
                break;
            }
            let temp_exp2 = temp_exp.clone();
            temp_exp.dot_equ(&temp_exp2);
        }
        result
    }
}

impl<T, const WIDTH: usize> BitAnd<T> for BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    type Output = Self;

    /// Bitwise-and matrix with an integer value.
    /// The meaning of this is, this modifies a matrix so that its matrix-multiplication with an
    /// integer represents a bit operation that has been masked with the given integer bit value.
    fn bitand(self, rhs: T) -> BitColumnMatrix<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = self.columns[i] & rhs;
        }
        result
    }
}

impl<T, const WIDTH: usize> Shl<usize> for BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    type Output = Self;

    /// Shift a matrix left.
    /// The meaning of this is, this modifies a matrix so that its matrix-multiplication with an
    /// integer represents a bit operation that is shifted left.
    fn shl(self, n: usize) -> BitColumnMatrix<T, WIDTH> {
        let mask = Self::width_mask();
        let mut result = BitColumnMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = (self.columns[i] << n) & mask;
        }
        result
    }
}

impl<T, const WIDTH: usize> Shr<usize> for BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    type Output = Self;

    /// Shift a matrix right. The meaning of this is, this modifies a matrix so that its
    /// matrix-multiplication with an integer represents a bit operation that is shifted right.
    fn shr(self, n: usize) -> BitColumnMatrix<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = self.columns[i] >> n;
        }
        result
    }
}

impl<T, const WIDTH: usize> core::ops::Add for BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    type Output = Self;

    /// Add two matrices.
    fn add(self, b: BitColumnMatrix<T, WIDTH>) -> BitColumnMatrix<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            result.columns[i] ^= b.columns[i];
        }
        result
    }
}

impl<'a, 'b, T, const WIDTH: usize> core::ops::Add<&'b BitColumnMatrix<T, WIDTH>>
    for &'a BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    type Output = BitColumnMatrix<T, WIDTH>;

    /// Add two matrices (by reference).
    fn add(self, b: &'b BitColumnMatrix<T, WIDTH>) -> BitColumnMatrix<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            result.columns[i] ^= b.columns[i];
        }
        result
    }
}

impl<T, const WIDTH: usize> core::ops::Mul for BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    type Output = Self;

    /// Multiply two matrices.
    fn mul(self, b: BitColumnMatrix<T, WIDTH>) -> BitColumnMatrix<T, WIDTH> {
        self.dot(&b)
    }
}

impl<'a, 'b, T, const WIDTH: usize> core::ops::Mul<&'b BitColumnMatrix<T, WIDTH>>
    for &'a BitColumnMatrix<T, WIDTH>
where
    T: BitColumnMatrixInt,
{
    type Output = BitColumnMatrix<T, WIDTH>;

    /// Multiply two matrices (by reference).
    fn mul(self, b: &'b BitColumnMatrix<T, WIDTH>) -> BitColumnMatrix<T, WIDTH> {
        self.dot(b)
    }
}
