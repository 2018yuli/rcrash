use std::time::Instant;

fn fib(n: u64) -> u64 {
    if n < 2 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}

fn fib2(n: u64) -> u64 {
    let mut m = 1;
    let mut a = 1;
    let mut b = 1;
    
    while m < n {
        let temp = a;
        a = b;
        b = temp + b;
        m += 1;
    }
    
    b
}

fn main() {

    // 记录当前时间
    let start = Instant::now();

    // 计算fib(80)
    println!("fib(40) = {}", fib(40));

    // 打印计算所用的时间
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);

    // 记录当前时间
    let start = Instant::now();

    // 计算fib(80)
    println!("fib2(40) = {}", fib2(40));

    // 打印计算所用的时间
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}