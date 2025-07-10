use std::time::Instant;

use deno_core::{v8, JsRuntime};

fn main() -> anyhow::Result<()> {
    run_fibonacci_test(30, 1000);
    Ok(())
}

#[allow(unused)]
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

#[allow(unused)]
fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[allow(unused)]
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
