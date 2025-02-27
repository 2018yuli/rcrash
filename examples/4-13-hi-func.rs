fn calc(method: fn(u32, u32) ->u32, a: u32, b: u32) -> u32 {
    method(a, b)
}

type Method = fn(u32, u32) -> u32;

fn calculate(method: Method, a: u32, b: u32) -> u32 {
    method(a, b)
}

fn cal_factory(method: &str) -> Method {
    match method {
        "add" => add,
        "sub" => sub,
        _ => unimplemented!(),
    }
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    a - b
}

fn main() {
    println!("calc(add, 10, 20) = {}", calc(add, 10, 20));
    println!("calculate(sub, 20, 10) = {}", calculate(sub, 20, 10));
    println!("cal_factory('add')(10, 20) = {}", cal_factory("add")(10, 20));
}