fn avg(a: u32, b: u32) -> u32 {
    // 补充你的代码
    let a1: u64 = a.into();
    let b1: u64 = b.into();
    ((a1 + b1)/2).try_into().unwrap()
}

fn main() {
    let a: u32 = "4294967295".parse::<u32>().unwrap();
    let b: u32 = 1;

    let (sum, is_overflow) = a.overflowing_add(b);
    println!("sum = {}, is_overflow = {}", sum, is_overflow);

    assert_eq!(avg(4294967295, 4294967295), 4294967295);
    assert_eq!(avg(0, 0), 0);
    assert_eq!(avg(10, 20), 15);
    assert_eq!(avg(4294967295, 1), 2147483648);
    println!("passed") 
}