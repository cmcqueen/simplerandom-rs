use num_traits::{PrimInt, Unsigned, One, Pow, Zero};
use std::ops::{BitAnd, Shl, Shr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitColumnMatrix<T, const WIDTH: usize>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign
{
    columns: [T; WIDTH],
}

impl<T, const WIDTH: usize> BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign
{
    pub fn new(init_data: &[T; WIDTH]) -> BitColumnMatrix::<T, WIDTH> {
        BitColumnMatrix::<T, WIDTH> {
            columns: *init_data,
        }
    }

    pub fn shift(shift_value: i8) -> BitColumnMatrix::<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH> {
            columns: [T::zero(); WIDTH],
        };
        let mut value: T =
            if shift_value >= 0 {
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

    pub fn dot(&self, b: &BitColumnMatrix::<T, WIDTH>) -> BitColumnMatrix::<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = self.dot_vec(b.columns[i]);
        }
        result
    }

    pub fn dot_equ(&mut self, b: &BitColumnMatrix::<T, WIDTH>) {
        let a = BitColumnMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            self.columns[i] = a.dot_vec(b.columns[i]);
        };
    }
}

impl<T, const WIDTH: usize> Zero for BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign
{

    fn zero() -> BitColumnMatrix::<T, WIDTH> {
        BitColumnMatrix::<T, WIDTH> {
            columns: [T::zero(); WIDTH],
        }
    }

    fn is_zero(&self) -> bool {
        for i in 0..WIDTH {
            if self.columns[i] != T::zero()
            {
                return false;
            }
        }
        true
    }
}

impl<T, const WIDTH: usize> One for BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign
{
    fn one() -> BitColumnMatrix::<T, WIDTH> {
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

impl<N, T, const WIDTH: usize> Pow<N> for BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign,
    N: Unsigned + PrimInt + BitAnd + One + Zero
{
    type Output = Self;

    fn pow(self, n: N) -> BitColumnMatrix::<T, WIDTH> {
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

impl<T, const WIDTH: usize> Shl<usize> for BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign,
{
    type Output = Self;

    fn shl(self, n: usize) -> BitColumnMatrix::<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = self.columns[i] << n;
        }
        result
    }
}

impl<T, const WIDTH: usize> Shr<usize> for BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign,
{
    type Output = Self;

    fn shr(self, n: usize) -> BitColumnMatrix::<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH>::zero();
        for i in 0..WIDTH {
            result.columns[i] = self.columns[i] >> n;
        }
        result
    }
}

impl<T, const WIDTH: usize> core::ops::Add for BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign
{
    type Output = Self;

    fn add(self, b: BitColumnMatrix::<T, WIDTH>) -> BitColumnMatrix::<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            result.columns[i] ^= b.columns[i];
        }
        result
    }
}

impl<'a, 'b, T, const WIDTH: usize> core::ops::Add<&'b BitColumnMatrix::<T, WIDTH>> for &'a BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign
{
    type Output = BitColumnMatrix::<T, WIDTH>;

    fn add(self, b: &'b BitColumnMatrix::<T, WIDTH>) -> BitColumnMatrix::<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            result.columns[i] ^= b.columns[i];
        }
        result
    }
}

impl<T, const WIDTH: usize> core::ops::Mul for BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign
{
    type Output = Self;

    fn mul(self, b: BitColumnMatrix::<T, WIDTH>) -> BitColumnMatrix::<T, WIDTH> {
        self.dot(&b)
    }
}

impl<'a, 'b, T, const WIDTH: usize> core::ops::Mul<&'b BitColumnMatrix::<T, WIDTH>> for &'a BitColumnMatrix::<T, WIDTH>
    where T: PrimInt + Unsigned + std::ops::BitXorAssign
{
    type Output = BitColumnMatrix::<T, WIDTH>;

    fn mul(self, b: &'b BitColumnMatrix::<T, WIDTH>) -> BitColumnMatrix::<T, WIDTH> {
        self.dot(b)
    }
}
