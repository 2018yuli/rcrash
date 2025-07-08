// cargo run --bin snapshot build
// cargo run --bin snapshot run

use std::fs;

use clap::{Parser, Subcommand};
use rcrash::{JsRuntime, JsRuntimeOptions};

const SS_FILE: &str = "snapshot.bin";

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug, Clone)]
enum Action {
    Build,
    Run,
}

fn main() {
    let args = Args::parse();
    match &args.action {
        Action::Build => build_snapshot(),
        Action::Run => run_snapshot(),
    }
}

fn build_snapshot() {
    let blob = JsRuntime::create_snapshot();
    fs::write(SS_FILE, blob).unwrap();
}

fn run_snapshot() {
    let blob = fs::read(SS_FILE).unwrap();
    let mut runtime = JsRuntime::new(JsRuntimeOptions::new(Some(blob)));
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
