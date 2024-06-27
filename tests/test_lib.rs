use rand_core::RngCore;
use simplerandom::RngJumpAhead;

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
