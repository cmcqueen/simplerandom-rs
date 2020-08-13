#![allow(dead_code)]

use rand_core::RngCore;
use simplerandom::RngJumpAhead;
//use rand::Rng;

fn test_new_and_next_u32() {
    //let mut s = simplerandom::Cong::new(1);
    //let mut s = simplerandom::SHR3::new(1);
    //let mut s = simplerandom::MWC1::new(1, 2);
    //let mut s = simplerandom::MWC2::new(1, 2);
    let mut s = simplerandom::KISS::new(1, 2, 3, 4);
    //let mut s = simplerandom::MWC64::new(1, 2);
    //let mut s = simplerandom::KISS2::new(1, 2, 3, 4);
    //let mut s = simplerandom::LFSR88::new(1, 2, 3);
    //let mut s = simplerandom::LFSR113::new(1, 2, 3, 4);
    for _ in 0..4 {
        println!("{}, {:?}", s.next_u32(), s);
        //println!("{}, {:?}", s.gen::<u32>(), s);
        //println!("{:?}", s.gen::<(f64)>());
    }

    let jumpahead_n = 1_000_000_000_000_000_000_u64;
    s.jumpahead(jumpahead_n);
    println!("jumpahead by {}", jumpahead_n);
    println!("{}, {:?}", s.next_u32(), s);
}

fn test_maths() {
    println!("wrapping_pow {}", simplerandom::maths::wrapping_pow(123456789_u32, 123456789_u32));
    println!("pow_mod {}", simplerandom::maths::pow_mod(123456789_u32, 123456789_u32, 99887766_u32));
    println!("wrapping_geom_series {}", simplerandom::maths::wrapping_geom_series(12345_u32, 12345_u32));
}

fn main() {
    test_new_and_next_u32();
    //test_maths();
}