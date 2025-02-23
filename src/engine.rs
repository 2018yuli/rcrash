use anyhow::Result;
use deno_core::{
    serde_v8,  // 用于在 JS 和 Rust 间序列化/反序列化
    JsRuntime, RuntimeOptions,
};
use serde::de::DeserializeOwned;

/// 一个简易的 JS 引擎封装
pub struct JsEngine {
    runtime: JsRuntime,
}

impl JsEngine {
    /// 创建新的引擎实例
    pub fn new() -> Self {
        // 若需要注册自定义函数，可以在这里通过 Extension 来做
        let runtime = JsRuntime::new(RuntimeOptions::default());
        Self { runtime }
    }

    /// 执行一段 JavaScript 代码，将返回值转换为 T
    pub fn eval<T>(&mut self, script: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // 1. 执行脚本
        let value = self.runtime.execute_script("<eval>", script)?;

        // 2. 在当前 HandleScope 中将返回值转换成 Rust 数据
        let scope = &mut self.runtime.handle_scope();
        let local = v8::Local::new(scope, value);

        // 3. 利用 serde_v8::from_v8 转换
        let result = serde_v8::from_v8::<T>(scope, local)?;
        Ok(result)
    }
}
