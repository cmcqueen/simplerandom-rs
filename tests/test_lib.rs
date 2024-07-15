use rand_core::RngCore;
use simplerandom::{RngJumpAhead, Seedable32Rng};

#[test]
fn test_kiss_million() {
    let mut rng = simplerandom::KISS::new(2247183469, 99545079, 3269400377, 3950144837);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 2100752872);

    let mut rng_ja = simplerandom::KISS::new(2247183469, 99545079, 3269400377, 3950144837);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_kiss_from_seed32() {
    let rng = simplerandom::KISS::new(2247183469, 99545079, 3269400377, 3950144837);
    let seed32 = [2247183469_u32, 99545079, 3269400377, 3950144837];
    let rng_from_seed32 = simplerandom::KISS::from_seed32(seed32);
    assert_eq!(rng, rng_from_seed32);
}

#[test]
fn test_cong_million() {
    let mut rng = simplerandom::Cong::new(2051391225);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 2416584377);

    let mut rng_ja = simplerandom::Cong::new(2051391225);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_cong_from_seed32() {
    let rng = simplerandom::Cong::new(2051391225);
    let seed32 = [2051391225_u32];
    let rng_from_seed32 = simplerandom::Cong::from_seed32(seed32);
    assert_eq!(rng, rng_from_seed32);
}

#[test]
fn test_shr3_million() {
    let mut rng = simplerandom::SHR3::new(3360276411);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 1153302609);

    let mut rng_ja = simplerandom::SHR3::new(3360276411);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_shr3_from_seed32() {
    let rng = simplerandom::SHR3::new(3360276411);
    let seed32 = [3360276411_u32];
    let rng_from_seed32 = simplerandom::SHR3::from_seed32(seed32);
    assert_eq!(rng, rng_from_seed32);
}

#[test]
fn test_mwc1_million() {
    let mut rng = simplerandom::MWC1::new(2374144069, 1046675282);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 904977562);

    let mut rng_ja = simplerandom::MWC1::new(2374144069, 1046675282);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_mwc1_from_seed32() {
    let rng = simplerandom::MWC1::new(2374144069, 1046675282);
    let seed32 = [2374144069_u32, 1046675282];
    let rng_from_seed32 = simplerandom::MWC1::from_seed32(seed32);
    assert_eq!(rng, rng_from_seed32);
}

#[test]
fn test_mwc2_million() {
    let mut rng = simplerandom::MWC2::new(0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 767834450);

    let mut rng_ja = simplerandom::MWC2::new(0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_mwc2_from_seed32() {
    let rng = simplerandom::MWC2::new(1661913791, 1937402980);
    let seed32 = [1661913791_u32, 1937402980];
    let rng_from_seed32 = simplerandom::MWC2::from_seed32(seed32);
    assert_eq!(rng, rng_from_seed32);
}

#[test]
fn test_mwc64_million() {
    let mut rng = simplerandom::MWC64::new(0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 2191957470);

    let mut rng_ja = simplerandom::MWC64::new(0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_mwc64_from_seed32() {
    let rng = simplerandom::MWC64::new(2144782570, 2495795596);
    let seed32 = [2144782570_u32, 2495795596];
    let rng_from_seed32 = simplerandom::MWC64::from_seed32(seed32);
    assert_eq!(rng, rng_from_seed32);
}

#[test]
fn test_kiss2_million() {
    let mut rng = simplerandom::KISS2::new(0, 0, 0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 4044786495);

    let mut rng_ja = simplerandom::KISS2::new(0, 0, 0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_kiss2_from_seed32() {
    let rng = simplerandom::KISS2::new(3291055097, 2366213450, 1454267445, 2119750303);
    let seed32 = [3291055097_u32, 2366213450, 1454267445, 2119750303];
    let rng_from_seed32 = simplerandom::KISS2::from_seed32(seed32);
    assert_eq!(rng, rng_from_seed32);
}

#[test]
fn test_lfsr113_million() {
    let mut rng = simplerandom::LFSR113::new(0, 0, 0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 300959510);

    let mut rng_ja = simplerandom::LFSR113::new(0, 0, 0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_lfsr113_from_seed32() {
    let rng = simplerandom::LFSR113::new(1978567657, 1566108429, 3559516695, 1376077178);
    let seed32 = [1978567657_u32, 1566108429, 3559516695, 1376077178];
    let rng_from_seed32 = simplerandom::LFSR113::from_seed32(seed32);
    assert_eq!(rng, rng_from_seed32);
}

#[test]
fn test_lfsr88_million() {
    let mut rng = simplerandom::LFSR88::new(0, 0, 0);
    let mut k: u32 = 0;
    for _ in 0..1_000_000 {
        k = rng.next_u32();
    }
    assert_eq!(k, 3774296834);

    let mut rng_ja = simplerandom::LFSR88::new(0, 0, 0);
    rng_ja.jumpahead(1_000_000);
    assert_eq!(rng_ja, rng);
    assert_eq!(rng_ja.next_u32(), rng.next_u32());
}

#[test]
fn test_lfsr88_from_seed32() {
    let rng = simplerandom::LFSR88::new(3004756978, 2083959123, 3430364291);
    let seed32 = [3004756978_u32, 2083959123, 3430364291];
    let rng_from_seed32 = simplerandom::LFSR88::from_seed32(seed32);
    assert_eq!(rng, rng_from_seed32);
}
