
fn main() {
    //let mut s = simplerandom::Shr3::new();
    let mut s = simplerandom::Cong::new();
    //let mut s = simplerandom::MWC2::new();
    s.next();
    let s_result = s.next();
    println!("simplerandom {}, {:?}", s_result, s);
}
