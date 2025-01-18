//! A main project provides fibonacci function
//! 这是模块级别的文档注释, 一般用于模块文件的头部

/// 这是文档注释, 一般用于函数或结构体的说明, 置于说明对象的上方.
struct Person;

/*
也可以使用 /* */ 注释多行, 这一点与 C 语言是一样的
*/

struct Cat;

/// In mathematics, the Fibonacci numbers, commonly denoted Fn form a sequence, called the Fibonacci sequence, such that
/// each number is the sum of the two preceding ones, starting from 0 and 1. That is
/// ```
/// F(0) = 0
/// F(1) = 1
/// F(n) = F(n − 1) + F(n − 2)
/// ```
fn fibo(n: u32) -> u32 {
    if n== 0 || n == 1 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    // Calculate fibo(10)
    println!("fibo(10) = {}", fibo(10));
}