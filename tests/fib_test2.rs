// main.rs
use deno_core::{v8, JsRuntime};
use std::time::Instant;

fn run_fibonacci_test2(_test_number: u32, iterations: u32) {
    let mut runtime = JsRuntime::new(deno_core::RuntimeOptions {
        ..Default::default()
    });

    // JavaScript 斐波那契函数和循环
    let js_script = r#"
        var fib = function(n) {
            let a = 0, b = 1;
            for (let i = 2; i <= n; i++) {
                let temp = a + b;
                a = b;
                b = temp;
            }
            return n === 0 ? 0 : b;
        }
    "#;
    let js_call = r#"
        var results = [];
        for (let i = 0; i < 1000; i++) {
            results.push(fib(30));
        }
        results;
    "#;

    // 将 JavaScript 函数加载到运行时
    runtime.execute_script("<init>", js_script).unwrap();

    let mut total_time = 0;
    let start_time = Instant::now();

    // 执行脚本，运行 1000 次 fib(30)
    let start = Instant::now();
    let result = runtime.execute_script("<fib_1000>", js_call).unwrap();

    let scope = &mut runtime.handle_scope();
    let result_value = v8::Local::new(scope, result);
    let result_string = result_value
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    println!("Result: {}", result_string);

    let elapsed = start.elapsed();
    total_time += elapsed.as_nanos();

    // 计算总时间和平均时间
    let total_elapsed = start_time.elapsed();
    let avg_time = total_time as f64 / iterations as f64 / 1_000_000.0;

    println!("Average time taken: {} ms", avg_time);
    println!("Total: {} ms", total_elapsed.as_millis());
}

fn run_fibonacci_test3(_test_number: u32, iterations: u32) {
    let mut runtime = JsRuntime::new(deno_core::RuntimeOptions {
        ..Default::default()
    });

    // JavaScript 斐波那契函数和循环
    let js_script = r#"
        var fib = function(n) {
            if (n === 0) return 0;
            else if (n === 1) return 1;
            else return fib(n - 1) + fib(n - 2);
        }
    "#;
    let js_call = r#"
        var results = [];
        for (let i = 0; i < 1000; i++) {
            results.push(fib(30));
        }
        results;
    "#;

    // 将 JavaScript 函数加载到运行时
    runtime.execute_script("<init>", js_script).unwrap();

    let mut total_time = 0;
    let start_time = Instant::now();

    // 执行脚本，运行 1000 次 fib(30)
    let start = Instant::now();
    let result = runtime.execute_script("<fib_1000>", js_call).unwrap();

    let scope = &mut runtime.handle_scope();
    let result_value = v8::Local::new(scope, result);
    let result_string = result_value
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    println!("Result: {}", result_string);

    let elapsed = start.elapsed();
    total_time += elapsed.as_nanos();

    // 计算总时间和平均时间
    let total_elapsed = start_time.elapsed();
    let avg_time = total_time as f64 / iterations as f64 / 1_000_000.0;

    println!("Average time taken: {} ms", avg_time);
    println!("Total: {} ms", total_elapsed.as_millis());
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
        run_fibonacci_test2(test_number, iterations);
    }

    #[test]
    fn test_fibonacci_performance2() {
        let test_number = 30; // 计算第30个斐波那契数
        let iterations = 1000; // 重复测试次数

        // 调用性能测试函数，通过脚本，计算 fib(30)
        run_fibonacci_test3(test_number, iterations);
    }
}
