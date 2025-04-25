
fn largest1(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

// 总是使用 T 做为泛型的类型，代表 Type
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", largest1(10, 20));
    println!("{}", largest(10, 30));
    println!("{}", largest(10.0, 30.0));
    let a: f64 = 3.14;
    let b: f64 = 2.17;
    println!("{}", largest(a, b));
}