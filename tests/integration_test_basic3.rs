use v8::{ContextOptions, CreateParams, HandleScope, Isolate, Script, V8};

pub fn demo3() -> anyhow::Result<()> {
    // create an isolate
    // let create_params = v8::Isolate::create_params().heap_limits(min, max);
    let params = CreateParams::default();
    let params = params.heap_limits(0, 1024 * 1024 * 10);
    let isolate = &mut Isolate::new(params);

    // create a handle scope
    // HandleScope （用于管理 V8 句柄的生命周期）的操作是会修改内部状态的
    // 因此在 Rust 中你需要借用它的可变引用来创建新的句柄
    let handle_scope = &mut HandleScope::new(isolate);
    // create a context
    let context = v8::Context::new(handle_scope, ContextOptions::default());
    // create context scope
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    // javascript code
    let source = r#"
        function hello(a, b) {
            return {
                status: 200,
                message: "Hello from Rust",
            };
        }
        hello();
    "#;
    let source = v8::String::new(context_scope, source).unwrap();
    // compile the code
    let script = Script::compile(context_scope, source, None).unwrap();
    let result = script.run(context_scope).unwrap();
    let value: serde_json::Value = serde_v8::from_v8(context_scope, result)?;
    println!("Result is {value:?}");

    Ok(())
}
pub fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    V8::initialize_platform(platform);
    V8::initialize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo3() -> anyhow::Result<()> {
        init();
        demo3()?;
        Ok(())
    }
}
