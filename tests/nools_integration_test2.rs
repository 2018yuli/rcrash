// main.rs
use deno_core::{JsRuntime, RuntimeOptions};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nools() {
        let _ = run_nools_test();
    }
}

fn run_nools_test() -> anyhow::Result<()> {
    let mut runtime = JsRuntime::new(RuntimeOptions {
        ..Default::default()
    });

    // 加载 Nools 脚本
    let nools_src = include_str!("../assets/nools2.js");
    runtime.execute_script("nools.js", nools_src)?;

    // 加载你的 basic.js 脚本
    let js_script = include_str!("../assets/basic.js");
    runtime.execute_script("basic.js", js_script)?;

    Ok(())
}
