use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn print(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let result: String = serde_v8::from_v8(scope, args.get(0)).unwrap();
    println!("Rust says: {result}");
    rv.set(serde_v8::to_v8(scope, result).unwrap());
}
