use std::borrow::Cow;

use super::state::JsRuntimeState;
use crate::{
    extensions::{Extensions, EXTERNAL_REFERENCE},
    utils::execute_script,
};
use once_cell::sync::OnceCell;
use v8::{CreateParams, FunctionCodeHandling, HandleScope, Isolate, OwnedIsolate, V8};

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
pub struct JsRuntimeOptions {
    pub params: CreateParams,
    pub snapshot_support: bool,
}

impl JsRuntimeOptions {
    pub fn new(snapshot: Option<Vec<u8>>) -> Self {
        let (params, support) = match snapshot {
            Some(snapshot_data) => (
                CreateParams::default()
                    .snapshot_blob(snapshot_data.into())
                    .external_references(Cow::Borrowed(EXTERNAL_REFERENCE.refs)),
                true,
            ),
            None => (CreateParams::default(), false),
        };

        JsRuntimeOptions {
            params,
            snapshot_support: support,
        }
    }
    // 用于获取结构体中的内部数据
    pub fn into_inner(self) -> CreateParams {
        self.params
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
        // let (isolate, initialized) = if params.snapshot_support {
        //     (
        //         Isolate::snapshot_creator(
        //             Some(Cow::Borrowed(EXTERNAL_REFERENCE.refs)),
        //             Some(params.into_inner()),
        //         ),
        //         true,
        //     )
        // } else {
        //     (Isolate::new(params.into_inner()), false)
        // };
        let initialized = params.snapshot_support;
        let isolate = Isolate::new(params.into_inner());
        JsRuntime::init_isolate(isolate, initialized, false)
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
    pub fn create_snapshot() -> Vec<u8> {
        JsRuntime::init();
        let isolate = Isolate::snapshot_creator(
            Some(Cow::Borrowed(EXTERNAL_REFERENCE.refs)),
            Some(CreateParams::default()),
        );
        let mut runtime = JsRuntime::init_isolate(isolate, false, true);
        JsRuntimeState::drop_context(&mut runtime.isolate);

        let startup_data = runtime
            .isolate
            .create_blob(FunctionCodeHandling::Clear)
            .expect("快照创建失败");
        startup_data.to_vec()
    }
    fn init_isolate(mut isolate: OwnedIsolate, initialized: bool, snapshot: bool) -> Self {
        let state = JsRuntimeState::new(&mut isolate, snapshot);
        isolate.set_slot(state);

        if !initialized {
            let context = JsRuntimeState::get_context(&mut isolate);
            let scope = &mut HandleScope::with_context(&mut isolate, context);
            Extensions::install(scope);
        }

        JsRuntime { isolate }
    }
}
