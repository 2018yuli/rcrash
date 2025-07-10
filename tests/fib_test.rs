// main.rs
use deno_core::JsRuntime;
use std::time::Instant;

fn run_fibonacci_test(test_number: u32, iterations: u32) {
    let mut runtime = JsRuntime::new(deno_core::RuntimeOptions {
        ..Default::default()
    });

    // JavaScript 斐波那契函数
    let js_script = r#"
        function fib(n) {
            if (n === 0) return 0;
            else if (n === 1) return 1;
            else return fib(n - 1) + fib(n - 2);
        }
    "#;

    // 将 JavaScript 函数加载到运行时
    runtime.execute_script("<init>", js_script).unwrap();

    let mut total_time = 0;
    let start_time = Instant::now();

    for _ in 0..iterations {
        let start = Instant::now();
        let result = runtime
            .execute_script("<fib>", format!("fib({})", test_number))
            .unwrap();
        let elapsed = start.elapsed();
        total_time += elapsed.as_nanos();

        // 打印单次执行的结果和时间
        println!(
            "Result of fib({}) is {:?}, Time taken: {} ms",
            test_number,
            result,
            elapsed.as_millis()
        );
    }

    let total_elapsed = start_time.elapsed();
    let avg_time = total_time as f64 / iterations as f64 / 1_000_000.0;

    println!("Average time taken: {} ms", avg_time);
    println!("Total: {} ms", total_elapsed.as_millis());
}

fn run_fibonacci(test_number: u32, iterations: u32) {
    let mut total_time = 0;
    let start_time = Instant::now();

    for _ in 0..iterations {
        let start = Instant::now();
        // 纯 Rust 斐波那契
        let result = fib(test_number);
        let elapsed = start.elapsed();
        total_time += elapsed.as_nanos();

        // 打印单次执行的结果和时间
        println!(
            "Result of fib({}) is {}, Time taken: {} ms",
            test_number,
            result,
            elapsed.as_millis()
        );
    }
    let total_elapsed = start_time.elapsed();
    let avg_time = total_time as f64 / iterations as f64 / 1_000_000.0;

    println!("Average time taken: {} ms", avg_time);
    println!("Total: {} ms", total_elapsed.as_millis());
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java 调用内嵌 Js 引擎： Average time taken: 134 ms Total: 185774 ms
    // Rust 调用 V8 引擎：Average time taken: 16.45747533 ms Total 16466 ms
    #[test]
    fn test_fibonacci_performance() {
        let test_number = 30; // 计算第30个斐波那契数
        let iterations = 1000; // 重复测试次数

        // 调用性能测试函数
        run_fibonacci_test(test_number, iterations);
    }

    // Java Average time taken: 0 ms Total: 653 ms
    // Rust Average time taken: 15.811717924 ms Total 15819 ms
    #[test]
    fn test_rust_fibonacci_performance() {
        let test_number = 30; // 计算第30个斐波那契数
        let iterations = 1000; // 重复测试次数

        // 纯粹 Rust 调用 fib 计算性能测试函数
        run_fibonacci(test_number, iterations);
    }
}
