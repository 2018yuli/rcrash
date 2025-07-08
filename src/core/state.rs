use std::{cell::RefCell, rc::Rc};

use super::{GlobalContext, JsRuntimeStateRef};
use v8::{ContextOptions, Isolate};

#[derive(Clone, Debug)]
pub struct JsRuntimeState {
    context: Option<v8::Global<v8::Context>>,
}

impl JsRuntimeState {
    pub fn new(isolate: &mut Isolate, snapshot: bool) -> JsRuntimeStateRef {
        // 创建默认 Context
        let context = {
            let handle_scope = &mut v8::HandleScope::new(isolate);
            let context = v8::Context::new(handle_scope, ContextOptions::default());
            if snapshot {
                handle_scope.set_default_context(context);
            }
            let global = v8::Global::new(handle_scope, context);
            global
        };
        // context 逃出了 HandleScope 的生命周期
        Rc::new(RefCell::new(JsRuntimeState {
            context: Some(context),
        }))
    }

    pub fn get_context(isolate: &mut Isolate) -> GlobalContext {
        let state = isolate.get_slot::<JsRuntimeStateRef>().unwrap().clone();
        let ctx = &state.borrow().context;
        ctx.as_ref().unwrap().clone()
    }

    pub fn drop_context(isolate: &mut Isolate) {
        let state = isolate.get_slot::<JsRuntimeStateRef>().unwrap().clone();
        //
        // 把 context 设置为 None（相当于把 context 从结构中取走、丢弃）
        //
        state.borrow_mut().context.take();
    }
}
