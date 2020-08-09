use num_traits::{PrimInt, Unsigned, Zero, One};
use std::ops::{BitAnd};

#[derive(Clone)]
pub struct BitColumnMatrix32 {
    columns: [u32; 32],
}

impl BitColumnMatrix32 {
    const WIDTH: usize = 32;

    pub fn new(init_data: &[u32; 32]) -> BitColumnMatrix32 {
        BitColumnMatrix32 {
            columns: *init_data,
        }
    }

    pub fn zero() -> BitColumnMatrix32 {
        BitColumnMatrix32 {
            columns: [0; 32],
        }
    }

    pub fn unity() -> BitColumnMatrix32 {
        let mut result = BitColumnMatrix32 {
            columns: [0; 32],
        };
        let mut value: u32 = 1;
        for i in 0..BitColumnMatrix32::WIDTH {
            result.columns[i] = value;
            value <<= 1;
        }
        result
    }

    pub fn shift(shift_value: i8) -> BitColumnMatrix32 {
        let mut result = BitColumnMatrix32 {
            columns: [0; 32],
        };
        let mut value: u32 =
            if shift_value >= 0 {
                1 << shift_value
            } else {
                0
            };
        let mut shift_temp = shift_value;
        for i in 0..BitColumnMatrix32::WIDTH {
            result.columns[i] = value;
            if shift_temp < 0 {
                shift_temp += 1;
                if shift_temp == 0 {
                    value = 1;
                }
            } else {
                value <<= 1;
            }
        }
        result
    }

    pub fn add(&self, b: &BitColumnMatrix32) -> BitColumnMatrix32 {
        let mut result = BitColumnMatrix32 {
            columns: self.columns,
        };
        for i in 0..BitColumnMatrix32::WIDTH {
            result.columns[i] ^= b.columns[i];
        }
        result
    }

    pub fn dot_vec(&self, b: u32) -> u32 {
        let mut result: u32 = 0;
        let mut b_temp = b;
        for i in 0..BitColumnMatrix32::WIDTH {
            if b_temp & 1 != 0 {
                result ^= self.columns[i];
            }
            b_temp >>= 1;
        }
        result
    }

    pub fn dot(&self, b: &BitColumnMatrix32) -> BitColumnMatrix32 {
        let mut result = BitColumnMatrix32::zero();
        for i in 0..BitColumnMatrix32::WIDTH {
            result.columns[i] = self.dot_vec(b.columns[i]);
        }
        result
    }

    pub fn dot_equ(&mut self, b: &BitColumnMatrix32) {
        let a = BitColumnMatrix32 {
            columns: self.columns,
        };
        for i in 0..BitColumnMatrix32::WIDTH {
            self.columns[i] = a.dot_vec(b.columns[i]);
        };
    }

    pub fn pow<N>(&self, n: N) -> BitColumnMatrix32
        where N: Unsigned + PrimInt + BitAnd + One + Zero
    {
        let mut result = BitColumnMatrix32::unity();
        let mut temp_exp = BitColumnMatrix32 {
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
