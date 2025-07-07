use std::{cell::RefCell, rc::Rc};

mod runtime;
mod state;

pub use runtime::{JsRuntime, JsRuntimeOptions};
pub use state::JsRuntimeState;

// type aliases
type GlobalContext = v8::Global<v8::Context>;
type JsRuntimeStateRef = Rc<RefCell<JsRuntimeState>>;
pub type LocalValue<'s> = v8::Local<'s, v8::Value>;
