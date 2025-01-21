
fn main() {
    // rust 除了下列代码，都有返回值
    // 变量声明
    // 模块声明
    // 函数声明
    // 结构体声明
    // 枚举声明
    // ......  

    let a: i32 = "12".parse().unwrap();
    let b = if a > 10 {
        1
    } else {
        0
    };
    println!("a = {}, b = {}", a, b);

    let mut sum = 0;
    let mut num = 10;
    let a = loop {
        if num < 0 {
            break sum;
        }
        sum += num;
        num -= 1;
    };
    println!("{:?}", a)
}