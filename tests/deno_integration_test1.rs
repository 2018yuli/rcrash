use deno_core::{v8, JsRuntime, PollEventLoopOptions, RuntimeOptions};
use std::error::Error;

async fn execute_js_async() -> Result<String, Box<dyn Error>> {
    // 创建一个 JsRuntime 实例
    let mut runtime = JsRuntime::new(RuntimeOptions::default());

    // JavaScript 代码，包含一个异步函数
    let js_code = r#"
        async function hello() {
            return new Promise((res, _rej) => {
                Deno.core.print("Hello world!!!!!\n");
                console.log('1111');
                res("hello");
            });
        }
        hello();
    "#;

    // 执行 JavaScript 代码
    let value_global = runtime.execute_script("hello.js", js_code)?;

    // 获取并解析 Promise 结果
    let resolve = runtime.resolve(value_global);

    // 等待 Promise 解决并获取结果
    let result = runtime
        .with_event_loop_promise(resolve, PollEventLoopOptions::default())
        .await?;

    // 获取执行结果并转换为 Rust 字符串
    let scope = &mut runtime.handle_scope();
    let result_value = v8::Local::new(scope, result);
    let result_string = result_value
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    Ok(result_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_js_async() {
        // 调用 execute_js_async 函数并验证返回的结果
        let result = execute_js_async().await.unwrap();
        assert_eq!(result, "hello");
    }
}
