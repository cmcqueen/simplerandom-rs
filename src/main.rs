use simplerandom;

fn main() {
    let mut s = simplerandom::Shr3::new();
    s.next();
    s.next();
    println!("simplerandom {:?}", s);
}
