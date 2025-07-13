use std::{path::PathBuf, rc::Rc};

use deno_core::{FsModuleLoader, JsRuntime, PollEventLoopOptions, RuntimeOptions};

#[tokio::test]
async fn test_execute_js_date_format() -> anyhow::Result<()> {
    let date_file_path = std::fs::canonicalize("./assets/extend/date-extended.js")?;
    let date_file_url =
        deno_core::resolve_path(&date_file_path.to_string_lossy(), &PathBuf::from("."))?;

    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    });

    // 加载 ES 模块
    let mod_id = js_runtime.load_main_es_module(&date_file_url).await?;
    let _eval = js_runtime.mod_evaluate(mod_id);
    js_runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await?;

    // 调用格式化逻辑（格式化 new Date(2006, 7, 11, 0, 55, 12, 345) 为 M/dd/yy）
    js_runtime.execute_script(
        "<test>",
        r#"
            globalThis._result = dateExtended(new Date(2006, 7, 11, 0, 55, 12, 345)).format("M/dd/yy");
        "#,
    ).unwrap();

    // 从 V8 获取结果
    let scope = &mut js_runtime.handle_scope();
    let global = scope.get_current_context().global(scope);
    let result_key = deno_core::v8::String::new(scope, "_result").unwrap();
    let val = global.get(scope, result_key.into()).unwrap();
    let result = val.to_rust_string_lossy(scope);

    assert_eq!(result, "8/11/06");
    Ok(())
}
