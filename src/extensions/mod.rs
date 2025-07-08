mod fetch;
mod print;

use lazy_static::lazy_static;
use v8::{ExternalReference, HandleScope, MapFnTo};

use crate::execute_script;

const GLUE: &str = include_str!("glue.js");

pub struct Extensions;

pub struct MyExternalRefs {
    pub refs: &'static [ExternalReference],
}

// ⚠️ 确保不在多个线程写裸指针！
unsafe impl Sync for MyExternalRefs {}

lazy_static! {
    pub static ref EXTERNAL_REFERENCE: MyExternalRefs = MyExternalRefs {
        refs: Box::leak(Box::new([
            ExternalReference {
                function: MapFnTo::map_fn_to(print::print),
            },
            ExternalReference {
                function: MapFnTo::map_fn_to(fetch::fetch),
            },
        ])),
    };
}

impl Extensions {
    pub fn install(scope: &mut HandleScope) {
        let bindings = v8::Object::new(scope);
        let name = v8::String::new(scope, "print").unwrap();
        let func = v8::Function::new(scope, print::print).unwrap();
        bindings.set(scope, name.into(), func.into());

        let name = v8::String::new(scope, "fetch").unwrap();
        let func = v8::Function::new(scope, fetch::fetch).unwrap();
        bindings.set(scope, name.into(), func.into());

        // ????
        if let Ok(result) = execute_script(scope, GLUE) {
            let func = v8::Local::<v8::Function>::try_from(result).unwrap();
            let v = v8::undefined(scope).into();
            let args = [bindings.into()];
            func.call(scope, v, &args).unwrap();
        }
    }
}
