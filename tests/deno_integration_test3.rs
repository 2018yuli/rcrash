use deno_core::futures::future::poll_fn;
use deno_core::JsRuntime;

async fn simple_manual_event_loop() -> Result<(), Box<dyn std::error::Error>> {
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

    runtime.execute_script("hello.js", js_code)?;

    // 手动轮询直到事件循环完成
    poll_fn(|cx| runtime.poll_event_loop(cx, Default::default())).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_js_async() {
        // 调用 execute_js_async 函数并验证返回的结果
        simple_manual_event_loop().await.unwrap();
    }
}
