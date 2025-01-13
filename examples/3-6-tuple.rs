
fn main() {
    let a: i32 = 10;
    let b: char = 'A';

    let mytuple:(i32, char) = (a, b);

    let (c, d) = mytuple;
    println!("c={},d={}", c, d);
    println!("mytuple.0={},mytuple.1={}",mytuple.0, mytuple.1);

    let (result, is_overflow) = a.overflowing_add(10);
    println!("{} {}", result, is_overflow);
}