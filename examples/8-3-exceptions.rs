
fn main() {
    let a: Result<u32, &'static str> = Result::Ok(1);
    println!("{:?}", a);    // Ok(1)

    let b: Result<u32, &'static str> = Result::Err("result error");
    println!("{:?}", b);    // Err("result error")

    let r = std::fs::read("./examples/a.txt");
    match r {
        // unwrap 不用处理 result，如果程序出错直接退出
        Ok(data) => println!("{:?}", std::str::from_utf8(&data).unwrap()),
        Err(err) => println!("{:?}", err),
    }
}