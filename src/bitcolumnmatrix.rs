use num_traits::{PrimInt, Unsigned, Zero, One};
use std::ops::{BitAnd};

#[derive(Clone)]
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

    pub fn zero() -> BitColumnMatrix::<T, WIDTH> {
        BitColumnMatrix::<T, WIDTH> {
            columns: [T::zero(); WIDTH],
        }
    }

    pub fn unity() -> BitColumnMatrix::<T, WIDTH> {
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

    pub fn add(&self, b: &BitColumnMatrix::<T, WIDTH>) -> BitColumnMatrix::<T, WIDTH> {
        let mut result = BitColumnMatrix::<T, WIDTH> {
            columns: self.columns,
        };
        for i in 0..WIDTH {
            result.columns[i] ^= b.columns[i];
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

    pub fn pow<N>(&self, n: N) -> BitColumnMatrix::<T, WIDTH>
        where N: Unsigned + PrimInt + BitAnd + One + Zero
    {
        let mut result = BitColumnMatrix::<T, WIDTH>::unity();
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
