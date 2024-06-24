use ::simplerandom::maths;

#[test]
fn test_mul_mod_generic() {
    // u8
    let result = maths::mul_mod_generic(129_u8, 35, 251);
    assert_eq!(result, 248_u8);

    // u16
    let result = maths::mul_mod_generic(12785_u16, 35067, 36969);
    assert_eq!(result, 8532_u16);
    let result = maths::mul_mod_generic(0xFFFE_u16, 0xFFFE, 0xFFFF);
    assert_eq!(result, 1_u16);

    // u32
    let result = maths::mul_mod_generic(1_u32, 1, 0x9068FFFF);
    assert_eq!(result, 1_u32);
    let result = maths::mul_mod_generic(2_u32, 0x40000000, 0x9068FFFF);
    assert_eq!(result, 0x80000000_u32);
    let result = maths::mul_mod_generic(4_u32, 0x40000000, 0x9068FFFF);
    assert_eq!(result, 0x6F970001_u32);
    let result = maths::mul_mod_generic(123456789_u32, 3111222333, 0x9068FFFF);
    assert_eq!(result, 1473911797_u32);
    let result = maths::mul_mod_generic(0xFFFFFFFE_u32, 0xFFFFFFFE, 0xFFFFFFFF);
    assert_eq!(result, 1_u32);

    // u64
    let result = maths::mul_mod_generic(1_u64, 1, 0x29A65EACFFFFFFFF);
    assert_eq!(result, 1_u64);
    let result = maths::mul_mod_generic(
        12345678901234567890_u64,
        10888777666555444333,
        0x29A65EACFFFFFFFF,
    );
    assert_eq!(result, 1426802886101663366_u64);

    // u128
    let result = maths::mul_mod_generic(
        155718596121766287585345689834087598824_u128,
        246971076467827994563959167304353476447,
        95808270567529502542106925837186468775,
    );
    assert_eq!(result, 57525510646347994906099177489276808553_u128);
}

#[test]
fn test_mul_mod() {
    // u8
    let result = maths::mul_mod(129_u8, 35, 251);
    assert_eq!(result, 248_u8);

    // u16
    let result = maths::mul_mod(12785_u16, 35067, 36969);
    assert_eq!(result, 8532_u16);
    let result = maths::mul_mod(0xFFFE_u16, 0xFFFE, 0xFFFF);
    assert_eq!(result, 1_u16);

    // u32
    let result = maths::mul_mod(1_u32, 1, 0x9068FFFF);
    assert_eq!(result, 1_u32);
    let result = maths::mul_mod(2_u32, 0x40000000, 0x9068FFFF);
    assert_eq!(result, 0x80000000_u32);
    let result = maths::mul_mod(4_u32, 0x40000000, 0x9068FFFF);
    assert_eq!(result, 0x6F970001_u32);
    let result = maths::mul_mod(123456789_u32, 3111222333, 0x9068FFFF);
    assert_eq!(result, 1473911797_u32);
    let result = maths::mul_mod(0xFFFFFFFE_u32, 0xFFFFFFFE, 0xFFFFFFFF);
    assert_eq!(result, 1_u32);

    // u64
    let result = maths::mul_mod(1_u64, 1, 0x29A65EACFFFFFFFF);
    assert_eq!(result, 1_u64);
    let result = maths::mul_mod(
        12345678901234567890_u64,
        10888777666555444333,
        0x29A65EACFFFFFFFF,
    );
    assert_eq!(result, 1426802886101663366_u64);

    // u128
    let result = maths::mul_mod(
        155718596121766287585345689834087598824_u128,
        246971076467827994563959167304353476447,
        95808270567529502542106925837186468775,
    );
    assert_eq!(result, 57525510646347994906099177489276808553_u128);
}

#[test]
fn test_modulo() {
    let result = maths::modulo(217_u8, 103_u8);
    assert_eq!(result, 11_u8);

    let result = maths::modulo(3_000_000_000_u32, 103_u8);
    assert_eq!(result, 61_u8);

    let result = maths::modulo(-1_000_000_000_i32, 207_u8);
    assert_eq!(result, 26_u8);

    let result = maths::modulo(-1_000_000_000_i32, 40239_u16);
    assert_eq!(result, 19628_u16);

    let result = maths::modulo(-1_000_000_000_i32, 3442199977_u32);
    assert_eq!(result, 2442199977_u32);

    let result = maths::modulo(-1_000_000_000_000_000_000_i64, 3442199977_u32);
    assert_eq!(result, 423026668_u32);

    let result = maths::modulo(1_000_000_000_000_000_000_i64, 3442199977_u32);
    assert_eq!(result, 3019173309_u32);
}

#[test]
fn test_wrapping_pow() {
    let result = maths::wrapping_pow(87_u8, 123_u8);
    assert_eq!(result, 135_u8);

    let result = maths::wrapping_pow(0xFD_u8, 0xFF_u8);
    assert_eq!(result, 0x55_u8);

    let result = maths::wrapping_pow(87_u8, 12345_u16);
    assert_eq!(result, 151_u8);

    let result = maths::wrapping_pow(0xFFFD_u16, 0xFFFF_u16);
    assert_eq!(result, 0x5555_u16);

    let result = maths::wrapping_pow(0xFFFFFFFD_u32, 0xFFFFFFFF_u32);
    assert_eq!(result, 0x55555555_u32);

    let result = maths::wrapping_pow(0xFFFFFFFFFFFFFFFD_u64, 0xFFFFFFFFFFFFFFFF_u64);
    assert_eq!(result, 0x5555555555555555_u64);
}

#[test]
fn test_pow_mod() {
    let result = maths::pow_mod(87_u8, 12345_u16, 251_u8);
    assert_eq!(result, 188_u8);

    let result = maths::pow_mod(252_u8, 255_u8, 255_u8);
    assert_eq!(result, 198_u8);

    let result = maths::pow_mod(10235_u16, 12345_u16, 63644_u16);
    assert_eq!(result, 45795_u16);

    let result = maths::pow_mod(648518821_u32, 12345_u32, 3288555137_u32);
    assert_eq!(result, 2953876344_u32);

    let result = maths::pow_mod(0xFFFFFFFC_u32, 0xFFFFFFFF_u32, 0xFFFFFFFF_u32);
    assert_eq!(result, 0x71C71C71_u32);

    let result = maths::pow_mod(
        0xFFFFFFFFFFFFFFFC_u64,
        0xFFFFFFFFFFFFFFFF_u64,
        0xFFFFFFFFFFFFFFFF_u64,
    );
    assert_eq!(result, 0x7C4A71C0F57CAAB0_u64);
}

#[test]
fn test_wrapping_geom_series() {
    let result = maths::wrapping_geom_series(21345_u32, 12345_u16);
    assert_eq!(result, 2573576889_u32);

    let result = maths::wrapping_geom_series(0xFFFFFFFD_u32, 12345_u16);
    assert_eq!(result, 0xB7068D89_u32);

    let result = maths::wrapping_geom_series(0xFFFFFFFD_u32, 123456789_u32);
    assert_eq!(result, 0xBA21CFAD_u32);

    let result = maths::wrapping_geom_series(69069_u32, 1_000_000_000_000_000_000_u64);
    assert_eq!(result, 629932032_u32);
}
