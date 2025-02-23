mod engine;

use anyhow::Result;
use engine::JsEngine;

fn main() -> Result<()> {
    let mut engine = JsEngine::new();

    // 执行一段 JS 脚本，并返回字符串
    let hello: String = engine.eval(r#"
        (function() {
            return "Hello from JS!"
        })()
    "#)?;
    println!("String result => {}", hello);

    // 执行计算并返回数字
    let number: f64 = engine.eval("123.456 * 2")?;
    println!("Number result => {}", number);

    // 返回一个对象（转为 Rust 中的 serde_json::Value）
    let obj: serde_json::Value = engine.eval(r#"
        ({
            name: "Deno",
            year: 2023,
            nested: { foo: 42 }
        })
    "#)?;
    println!("Object result => {}", obj);

    Ok(())
}
