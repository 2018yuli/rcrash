use super::state::JsRuntimeState;
use crate::{extensions::Extensions, utils::execute_script};
use once_cell::sync::OnceCell;
use v8::{CreateParams, HandleScope, Isolate, OwnedIsolate, V8};

#[derive(Debug)]
pub struct JsRuntime {
    isolate: OwnedIsolate,
}

///
/// 结构体，包含一个字段，字段类型是 CreateParams
///     结构体中的 CreateParams 并没有显式地命名
///     所以它实际上是一个元组结构体
///
#[derive(Debug, Default)]
pub struct JsRuntimeOptions(CreateParams);

impl JsRuntimeOptions {
    pub fn new(_snapshot: Option<Vec<u8>>) -> Self {
        JsRuntimeOptions(CreateParams::default())
    }
    // 用于获取结构体中的内部数据
    pub fn into_inner(self) -> CreateParams {
        self.0
    }
}

impl JsRuntime {
    pub fn init() {
        static V8_INSTANCE: OnceCell<()> = OnceCell::new();
        V8_INSTANCE.get_or_init(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            V8::initialize_platform(platform);
            V8::initialize();
        });
    }
    pub fn new(params: JsRuntimeOptions) -> Self {
        let isolate = Isolate::new(params.into_inner());
        JsRuntime::init_isolate(isolate)
    }
    pub fn execute_script(
        &mut self,
        script: impl AsRef<str>,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let context = JsRuntimeState::get_context(&mut self.isolate);
        let handle_scope = &mut HandleScope::with_context(&mut self.isolate, context);
        match execute_script(handle_scope, script) {
            Ok(v) => Ok(serde_v8::from_v8(handle_scope, v).unwrap()),
            Err(e) => Err(serde_v8::from_v8(handle_scope, e).unwrap()),
        }
    }
    pub fn create_snapshot(&self) -> Vec<u8> {
        todo!();
    }
    fn init_isolate(mut isolate: OwnedIsolate) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);
        {
            let context = JsRuntimeState::get_context(&mut isolate);
            let scope = &mut HandleScope::with_context(&mut isolate, context);
            Extensions::install(scope);
        }

        JsRuntime { isolate }
    }
}
