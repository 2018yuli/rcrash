use deno_core::futures::future::poll_fn;
use deno_core::futures::FutureExt;
use deno_core::{v8, JsRuntime, PollEventLoopOptions};
use std::task::Poll;

async fn execute_js_with_manual_event_loop() -> Result<String, Box<dyn std::error::Error>> {
    let mut runtime = JsRuntime::new(Default::default());

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

    // 执行脚本
    let value_global = runtime.execute_script("hello.js", js_code)?;
    let mut resolve = runtime.resolve(value_global);

    // 手动轮询事件循环直到 Promise 解决
    let result = poll_fn(|cx| {
        // 首先检查 Promise 是否已经解决
        if let Poll::Ready(result) = resolve.poll_unpin(cx) {
            return Poll::Ready(result);
        }

        // 轮询事件循环
        match runtime.poll_event_loop(cx, PollEventLoopOptions::default()) {
            Poll::Ready(Ok(())) => {
                // 事件循环完成，再次检查 Promise
                if let Poll::Ready(result) = resolve.poll_unpin(cx) {
                    Poll::Ready(result)
                } else {
                    Poll::Ready(Err(deno_core::error::CoreError::PendingPromiseResolution))
                }
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    })
    .await?;

    // 获取结果值
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
        let result = execute_js_with_manual_event_loop().await.unwrap();
        assert_eq!(result, "hello");
    }
}
