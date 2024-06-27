use ::simplerandom::bitcolumnmatrix::BitColumnMatrix;

use num_traits::{One, Pow, Zero};

type BitColumnMatrix32 = BitColumnMatrix<u32, 32>;

#[test]
fn test_zero() {
    let zero = BitColumnMatrix32::zero();
    let mut x = 0x8000000_u32;
    while x != 0 {
        let mul_result = zero.dot_vec(x);
        assert_eq!(mul_result, 0);
        x = x >> 1;
    }
}

#[test]
fn test_one() {
    let one = BitColumnMatrix32::one();
    let mut x = 0x8000000_u32;
    while x != 0 {
        let mul_result = one.dot_vec(x);
        assert_eq!(mul_result, x);
        x = x >> 1;
    }
}

#[test]
fn test_shl() {
    // Test that left-shifting the unity matrix by n is equivalent to calling shift(n).
    let shl_1 = BitColumnMatrix32::one() << 13;
    let shift_1 = BitColumnMatrix32::shift(13);
    debug_assert_eq!(shl_1, shift_1);

    // Test left-shifting a more complex matrix. Compare to multiplying by shift(n).
    const SHR3_MATRIX_ARRAY: [u32; 32] = [
        0x00042021, 0x00084042, 0x00108084, 0x00210108, 0x00420231, 0x00840462, 0x010808C4,
        0x02101188, 0x04202310, 0x08404620, 0x10808C40, 0x21011880, 0x42023100, 0x84046200,
        0x0808C400, 0x10118800, 0x20231000, 0x40462021, 0x808C4042, 0x01080084, 0x02100108,
        0x04200210, 0x08400420, 0x10800840, 0x21001080, 0x42002100, 0x84004200, 0x08008400,
        0x10010800, 0x20021000, 0x40042000, 0x80084000,
    ];
    let shr3_matrix = BitColumnMatrix32::new(&SHR3_MATRIX_ARRAY);
    let shl_1 = shr3_matrix.clone() << 13;
    let shift_1 = BitColumnMatrix32::shift(13) * shr3_matrix;
    debug_assert_eq!(shl_1, shift_1);
}

#[test]
fn test_shr() {
    // Test that right-shifting the unity matrix by n is equivalent to calling shift(-n).
    let shl_1 = BitColumnMatrix32::one() >> 13;
    let shift_1 = BitColumnMatrix32::shift(-13);
    debug_assert_eq!(shl_1, shift_1);

    // Test right-shifting a more complex matrix. Compare to multiplying by shift(-n).
    const SHR3_MATRIX_ARRAY: [u32; 32] = [
        0x00042021, 0x00084042, 0x00108084, 0x00210108, 0x00420231, 0x00840462, 0x010808C4,
        0x02101188, 0x04202310, 0x08404620, 0x10808C40, 0x21011880, 0x42023100, 0x84046200,
        0x0808C400, 0x10118800, 0x20231000, 0x40462021, 0x808C4042, 0x01080084, 0x02100108,
        0x04200210, 0x08400420, 0x10800840, 0x21001080, 0x42002100, 0x84004200, 0x08008400,
        0x10010800, 0x20021000, 0x40042000, 0x80084000,
    ];
    let shr3_matrix = BitColumnMatrix32::new(&SHR3_MATRIX_ARRAY);
    let shl_1 = shr3_matrix.clone() >> 13;
    let shift_1 = BitColumnMatrix32::shift(-13) * shr3_matrix;
    debug_assert_eq!(shl_1, shift_1);
}

#[test]
fn test_shift() {
    let mask = simplerandom::maths::bit_width_mask::<u32>(32);
    assert_eq!(mask, 0xFFFFFFFF);
    for shift_by in -31..31 {
        let shift = BitColumnMatrix32::shift(shift_by);
        let mut x = 0x8000000_u32;
        while x != 0 {
            let mul_result = shift.dot_vec(x);
            if shift_by >= 0 {
                assert_eq!(mul_result, (x << shift_by) & mask);
            } else {
                let shift_right_by = -shift_by;
                assert_eq!(mul_result, x >> shift_right_by);
            }
            x = x >> 1;
        }
    }
}

#[test]
fn test_bitand() {
    let mask = 0xA5F01248_u32;
    let mask_matrix = BitColumnMatrix32::one() & mask;

    let mul_result = mask_matrix.dot_vec(0xFFFFFFFF_u32);
    assert_eq!(mul_result, mask);

    let mut x = 0x8000000_u32;
    while x != 0 {
        let mul_result = mask_matrix.dot_vec(x);
        assert_eq!(mul_result, x & mask);
        x = x >> 1;
    }
}

#[test]
fn test_shr3_matrix() {
    const SHR3_MATRIX_ARRAY: [u32; 32] = [
        0x00042021, 0x00084042, 0x00108084, 0x00210108, 0x00420231, 0x00840462, 0x010808C4,
        0x02101188, 0x04202310, 0x08404620, 0x10808C40, 0x21011880, 0x42023100, 0x84046200,
        0x0808C400, 0x10118800, 0x20231000, 0x40462021, 0x808C4042, 0x01080084, 0x02100108,
        0x04200210, 0x08400420, 0x10800840, 0x21001080, 0x42002100, 0x84004200, 0x08008400,
        0x10010800, 0x20021000, 0x40042000, 0x80084000,
    ];
    let shr3_matrix = BitColumnMatrix32::new(&SHR3_MATRIX_ARRAY);

    let shr3_matrix_a = BitColumnMatrix32::one() + (BitColumnMatrix32::one() << 13);
    let shr3_matrix_b = BitColumnMatrix32::one() + (BitColumnMatrix32::one() >> 17);
    let shr3_matrix_c = BitColumnMatrix32::one() + (BitColumnMatrix32::one() << 5);
    let built_shr3_matrix = shr3_matrix_c * shr3_matrix_b * shr3_matrix_a;

    assert_eq!(shr3_matrix, built_shr3_matrix);
}

#[test]
fn test_pow_using_shr3() {
    const SHR3_MATRIX_ARRAY: [u32; 32] = [
        0x00042021, 0x00084042, 0x00108084, 0x00210108, 0x00420231, 0x00840462, 0x010808C4,
        0x02101188, 0x04202310, 0x08404620, 0x10808C40, 0x21011880, 0x42023100, 0x84046200,
        0x0808C400, 0x10118800, 0x20231000, 0x40462021, 0x808C4042, 0x01080084, 0x02100108,
        0x04200210, 0x08400420, 0x10800840, 0x21001080, 0x42002100, 0x84004200, 0x08008400,
        0x10010800, 0x20021000, 0x40042000, 0x80084000,
    ];
    let shr3_matrix = BitColumnMatrix32::new(&SHR3_MATRIX_ARRAY);

    const SHR3_MATRIX_POW_BILLION_ARRAY: [u32; 32] = [
        0x363ED7AC, 0xF891F4FD, 0xD1F74339, 0xA7DAB3E4, 0x77AE86B9, 0x0489CBC8, 0xC5DF9FF8,
        0x878F08E3, 0x4F8A70E5, 0x5DBE9A6A, 0xFECF0847, 0x77EB376E, 0xE2C97CF1, 0x878C7D68,
        0xB949B585, 0x4E643902, 0xAA197C6D, 0xE42F09A2, 0xC09479E5, 0x83CF163A, 0x1383F309,
        0x872692BB, 0xB4CF5CB0, 0x8476A25F, 0x95B3EC9E, 0x2A6D6AF0, 0x567C560B, 0xFAFE8FA3,
        0x61D228A8, 0x1CDED1C2, 0x833D6334, 0xF99D2B11,
    ];
    let shr3_pow_billion_matrix = BitColumnMatrix32::new(&SHR3_MATRIX_POW_BILLION_ARRAY);

    let result = shr3_matrix.pow(1_000_000_000_u32);

    assert_eq!(result, shr3_pow_billion_matrix);
}

#[test]
fn test_lfsr113_1_matrix() {
    const LFSR88_Z1_MATRIX_ARRAY: [u32; 32] = [
        0x00000000, 0x00002000, 0x00004000, 0x00008000, 0x00010000, 0x00020000, 0x00040001,
        0x00080002, 0x00100004, 0x00200008, 0x00400010, 0x00800020, 0x01000040, 0x02000080,
        0x04000100, 0x08000200, 0x10000400, 0x20000800, 0x40001000, 0x80000001, 0x00000002,
        0x00000004, 0x00000008, 0x00000010, 0x00000020, 0x00000040, 0x00000080, 0x00000100,
        0x00000200, 0x00000400, 0x00000800, 0x00001000,
    ];
    let lfsr88_z1_matrix = BitColumnMatrix32::new(&LFSR88_Z1_MATRIX_ARRAY);

    let lfsr88_z1_matrix_a = BitColumnMatrix32::one() + (BitColumnMatrix32::one() << 13);
    let lfsr88_z1_matrix_b = BitColumnMatrix32::one() >> 19;
    let lfsr88_z1_matrix_c = BitColumnMatrix32::one() & 0xFFFFFFFE;
    let lfsr88_z1_matrix_d = BitColumnMatrix32::one() << 12;
    let built_lfsr88_z1_matrix =
        lfsr88_z1_matrix_d * lfsr88_z1_matrix_c + lfsr88_z1_matrix_b * lfsr88_z1_matrix_a;

    assert_eq!(lfsr88_z1_matrix, built_lfsr88_z1_matrix);
}
