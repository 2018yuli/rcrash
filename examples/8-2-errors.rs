// 不可恢复错误
fn un_recoverable() {
    panic!("error!");
    println!("here");
}

// 断言（不可恢复错误）
fn assert_error() {
    assert!(1==2);
    assert_eq!(1, 2);
    println!("here");
}

// 未实现代码 not implemented: todo busy now!
fn unimplement_now() {
    unimplemented!("todo busy now!")
}

// 不应当被访问到的代码 internal error: entered unreachable code:
fn divide_by_three(x: u32) -> u32 {
    // 枚举求除法 （￣︶￣）↗　
    // for i in 0.. {
    //     if 3 * i < i {
    //         panic!("u32 overflow");
    //     }
    //     if x < 3 * i {
    //         return i - 1;
    //     }
    // }
    unreachable!("")
}

// 异常

fn main() {
    unimplement_now();
}