use std::rc::Rc;

// main.rs
use deno_core::{FsModuleLoader, JsRuntime, PollEventLoopOptions, RuntimeOptions};

async fn run_nools_test() -> anyhow::Result<()> {
    let mut runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    });

    let main_module = deno_core::resolve_path("./assets/nools.js", &std::env::current_dir()?)?;
    let mod_id = runtime.load_main_es_module(&main_module).await?;
    let _ = runtime.mod_evaluate(mod_id);
    runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await?;

    let js_script = include_str!("../assets/basic.js");

    runtime.execute_script("basic", js_script).unwrap();
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_nools() {
//         run_nools_test().await.unwrap();
//     }
// }
