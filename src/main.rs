use rand_core::RngCore;

fn main() {
    //let mut s = simplerandom::Cong::new();
    //let mut s = simplerandom::SHR3::new();
    //let mut s = simplerandom::MWC2::new();
    //let mut s = simplerandom::KISS::new();
    //let mut s = simplerandom::LFSR88::new();
    let mut s = simplerandom::LFSR113::new();
    let s_result = s.next_u32();
    println!("{}, {:?}", s_result, s);
    let s_result = s.next_u32();
    println!("{}, {:?}", s_result, s);
    let s_result = s.next_u32();
    println!("{}, {:?}", s_result, s);
}
