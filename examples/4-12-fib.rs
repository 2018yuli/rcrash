fn fib(n: u64) -> u64 {
    if n <= 2 {
        return n
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("fib(10) = {}", fib(10))
}