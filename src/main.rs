use rcrash::{JsRuntime, JsRuntimeOptions};

fn main() {
    JsRuntime::init();
    let mut runtime = JsRuntime::new(JsRuntimeOptions::default());
    let script = r#"
        print("hello");
        function hello(a, b) {
            return fetch("https://www.rust-lang.org/");
        }
        hello();
    "#;
    let result = runtime.execute_script(script).unwrap();
    println!("Result is {result:?}");
}
