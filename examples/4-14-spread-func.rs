fn foo() -> ! {
    panic!("This call never returns. ");
}

fn some_fn() {
    ()
}

fn main() {
    let a:() = some_fn();
    println!("this function returns and you can see the line.");

    // 最大的作用，用于通过类型检查
    let a = if true {
        10
    } else {
        foo()
    };
    println!("a = {}", a)
}