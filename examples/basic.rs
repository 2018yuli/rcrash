use deno_core::{anyhow::Result, v8, JsRuntime, PollEventLoopOptions, RuntimeOptions};

// cargo run --package rcrash --example basic

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions::default();
    let mut rt = JsRuntime::new(options);
    let source_code = include_str!("../assets/basic.js");
    // rt.execute_script("<anon1>", source_code)?;

    let value_global = rt.execute_script("<anon1>", source_code)?;

    // deal with async results
    let resolve = rt.resolve(value_global);
    let result = rt
        .with_event_loop_promise(resolve, PollEventLoopOptions::default())
        .await;

    let scope = &mut rt.handle_scope();
    let result_value = v8::Local::new(scope, result.unwrap());
    let result_string = result_value
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    println!("Result: {}", result_string);

    Ok(())
}
