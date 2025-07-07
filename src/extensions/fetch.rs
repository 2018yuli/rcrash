use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub fn fetch(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    println!("fetching.....................");
    let url: String = serde_v8::from_v8(scope, args.get(0)).unwrap();
    let result = reqwest::blocking::get(url).unwrap().text().unwrap();

    rv.set(serde_v8::to_v8(scope, result).unwrap());
}
