#![allow(dead_code)]

use rand_core::RngCore;
use simplerandom::{RngJumpAhead, SeedableSimpleRandom};
//use rand::Rng;

fn test_new_and_next_u32() {
    //let mut s = simplerandom::Cong::new(1);
    //let mut s = simplerandom::SHR3::new(1);
    //let mut s = simplerandom::MWC1::new(1, 2);
    //let mut s = simplerandom::MWC2::new(1, 2);
    //let mut s = simplerandom::KISS::new(1, 2, 3, 4);
    //let mut s = simplerandom::MWC64::new(1, 2);
    //let mut s = simplerandom::KISS2::new(1, 2, 3, 4);
    //let mut s = simplerandom::LFSR88::new(1, 2, 3);
    let mut s = simplerandom::LFSR113::new(1, 2, 3, 4);
    for _ in 0..4 {
        println!("{}, {:?}", s.next_u32(), s);
        //println!("{}, {:?}", s.gen::<u32>(), s);
        //println!("{:?}", s.gen::<(f64)>());
    }

    let jumpahead_n = 1_000_000_000_000_000_000_i64;
    s.jumpahead(jumpahead_n);
    println!("jumpahead by {}", jumpahead_n);
    println!("{}, {:?}", s.next_u32(), s);
}

fn test_maths() {
    println!(
        "wrapping_pow {}",
        simplerandom::maths::wrapping_pow(123456789_u32, 123456789_u32)
    );
    println!(
        "pow_mod {:#X}",
        simplerandom::maths::pow_mod(0xDC28D76F_usize, 0x732E73C3_usize, 0xEC327D45_usize)
    );
    println!(
        "pow_mod {:#X}",
        simplerandom::maths::pow_mod(
            0xDC28D76FFD9338E9D868AF566191DE10_u128,
            0x732E73C316878E244FDFDE4EE623CDCC_u128,
            0xEC327D45470669CC56B547B6FE6888A2_u128
        )
    );
    println!(
        "wrapping_geom_series {}",
        simplerandom::maths::wrapping_geom_series(12345_u32, 12345_u32)
    );
}

fn test_seed() {
    let s = [2_u32; 1];

    let mut rng = simplerandom::Cong::from_seed(&s);
    //let mut rng = simplerandom::SHR3::from_seed(&s);
    //let mut rng = simplerandom::MWC1::from_seed(&s);
    //let mut rng = simplerandom::MWC2::from_seed(&s);
    //let mut rng = simplerandom::KISS::from_seed(&s);
    //let mut rng = simplerandom::MWC64::from_seed(&s);
    //let mut rng = simplerandom::KISS2::from_seed(&s);
    //let mut rng = simplerandom::LFSR88::from_seed(&s);
    //let mut rng = simplerandom::LFSR113::from_seed(&s);

    println!("{:?}", rng);
    for _ in 0..4 {
        let rng_result = rng.next_u32();
        println!("{}, {:?}", rng_result, rng);
    }
}

fn main() {
    // test_new_and_next_u32();
    test_seed();

    println!("");
    test_maths();
}
