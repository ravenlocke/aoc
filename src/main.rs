use std::ops::Neg;

fn main() {
    let mut x = -0i64;
    x = x.neg();
    let y = 0i64;
    println!("{:?}, {:?}, {}", x.to_le_bytes(), x.is_negative(), x == y);
}
