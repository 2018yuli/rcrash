// main.rs
use deno_core::{v8, JsRuntime};
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

        let scope = &mut runtime.handle_scope();
        let result_value = v8::Local::new(scope, result);
        let result_string = result_value
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        let elapsed = start.elapsed();
        total_time += elapsed.as_nanos();

        // 打印单次执行的结果和时间
        println!(
            "Result of fib({}) is {:?}, Time taken: {} ms",
            test_number,
            result_string,
            elapsed.as_millis()
        );
    }

    let total_elapsed = start_time.elapsed();
    let avg_time = total_time as f64 / iterations as f64 / 1_000_000.0;

    println!("Average time taken: {} ms", avg_time);
    println!("Total: {} ms", total_elapsed.as_millis());
}

fn run_fibonacci_unre_test(test_number: u32, iterations: u32) {
    let mut runtime = JsRuntime::new(deno_core::RuntimeOptions {
        ..Default::default()
    });

    // JavaScript 斐波那契函数
    let js_script = r#"
        function fib(n) {
            let a = 0, b = 1;
            for (let i = 2; i <= n; i++) {
                let temp = a + b;
                a = b;
                b = temp;
            }
            return n === 0 ? a : b;
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

        let scope = &mut runtime.handle_scope();
        let result_value = v8::Local::new(scope, result);
        let result_string = result_value
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        let elapsed = start.elapsed();
        total_time += elapsed.as_nanos();

        // 打印单次执行的结果和时间
        println!(
            "Result of fib({}) is {:?}, Time taken: {} ms",
            test_number,
            result_string,
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

    // Java 调用内嵌 Js 引擎： Average time taken: 350 ms Total: 348955 ms (20倍性能差)
    // Rust 调用 V8 引擎：Average time taken: 16.072226189 ms Total 16076 ms
    // Rust-release 调用 V8 引擎：Average time taken: 16.204571959 ms Total 16239 ms
    #[test]
    fn test_fibonacci_performance() {
        let test_number = 30; // 计算第30个斐波那契数
        let iterations = 1000; // 重复测试次数

        // 调用性能测试函数，通过脚本，计算 fib(30)
        run_fibonacci_test(test_number, iterations);
    }

    // Rust 调用 V8 引擎：Average time taken: 15.859703261 ms Total 15863 ms
    #[test]
    fn run_fibonacci_unre_test() {
        let test_number = 30; // 计算第30个斐波那契数
        let iterations = 1000; // 重复测试次数

        // 调用性能测试函数，通过脚本，计算 fib(30)
        run_fibonacci_test(test_number, iterations);
    }

    // Java 原生代码 Average time taken: 5 ms Total: 5741 ms
    // Rust-debug 原生代码 Average time taken: 12.260116272 ms Total 12262 ms
    // Rust-release 原生代码 Average time taken: 3.0295954789999997 ms Total 3048 ms
    #[test]
    fn test_rust_fibonacci_performance() {
        let test_number = 30; // 计算第30个斐波那契数
        let iterations = 1000; // 重复测试次数

        // 纯粹 Rust 调用 fib 计算性能测试函数
        run_fibonacci(test_number, iterations);
    }
}
