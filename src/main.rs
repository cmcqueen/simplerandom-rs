use rand_core::RngCore;

fn main() {
    //let mut s = simplerandom::Cong::new(1);
    //let mut s = simplerandom::SHR3::new(1);
    //let mut s = simplerandom::MWC1::new(1, 2);
    //let mut s = simplerandom::MWC2::new(1, 2);
    //let mut s = simplerandom::KISS::new(1, 2, 3, 4);
    //let mut s = simplerandom::MWC64::new(1, 2);
    let mut s = simplerandom::KISS2::new(1, 2, 3, 4);
    //let mut s = simplerandom::LFSR88::new(1, 2, 3);
    //let mut s = simplerandom::LFSR113::new(1, 2, 3, 4);
    for _ in 0..4 {
        let s_result = s.next_u32();
        println!("{}, {:?}", s_result, s);
    }
}
