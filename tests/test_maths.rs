use ::simplerandom::maths;


#[test]
fn test_mul_mod() {
    let result = maths::mul_mod(1_u32, 1, 0x9068FFFF);
    assert_eq!(result, 1_u32);
    let result = maths::mul_mod(2_u32, 0x40000000, 0x9068FFFF);
    assert_eq!(result, 0x80000000_u32);
    let result = maths::mul_mod(4_u32, 0x40000000, 0x9068FFFF);
    assert_eq!(result, 0x6F970001_u32);
    let result = maths::mul_mod(123456789_u32, 3111222333, 0x9068FFFF);
    assert_eq!(result, 1473911797_u32);

    let result = maths::mul_mod(1_u64, 1, 0x29A65EACFFFFFFFF);
    assert_eq!(result, 1_u64);
    let result = maths::mul_mod(12345678901234567890_u64, 10888777666555444333, 0x29A65EACFFFFFFFF);
    assert_eq!(result, 1426802886101663366_u64);
}
