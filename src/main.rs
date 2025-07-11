use std::time::Instant;

use deno_core::{v8, JsRuntime};

fn main() -> anyhow::Result<()> {
    run_fibonacci_test2(30, 1000);
    Ok(())
}

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
