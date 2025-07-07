use std::{cell::RefCell, rc::Rc};

use super::{GlobalContext, JsRuntimeStateRef};
use v8::{ContextOptions, Isolate};

#[derive(Clone, Debug)]
pub struct JsRuntimeState {
    context: Option<v8::Global<v8::Context>>,
}

impl JsRuntimeState {
    pub fn new(isolate: &mut Isolate) -> JsRuntimeStateRef {
        let context = {
            let handle_scope = &mut v8::HandleScope::new(isolate);
            let context = v8::Context::new(handle_scope, ContextOptions::default());
            v8::Global::new(handle_scope, context)
        };

        Rc::new(RefCell::new(JsRuntimeState {
            context: Some(context),
        }))
    }

    pub fn get_context(isolate: &mut Isolate) -> GlobalContext {
        let state = isolate.get_slot::<JsRuntimeStateRef>().unwrap().clone();
        let ctx = &state.borrow().context;
        ctx.as_ref().unwrap().clone()
    }
}
