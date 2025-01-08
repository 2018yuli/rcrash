// 创建变量 let 关键字
// 变量默认是不可变的（运行期间求值）
// 可变变量：变量名称前加 mut
// 常量：const 关键字（编译期间进行求值）

const A_CONST: i32 = 1 * 42;

// const B_CONST: i32 = get_number();
fn get_number() -> i32 {
    42
}

// const C_CONST : i32 = (||-> i32 { 42 })();

fn main() {
    let mut x = 5;
    println!("x = {}", x);

    x = 6;
    println!("now x = {}", x);

    println!("The value of a const is {}", A_CONST);

    // println!("The value of c const is {}", C_CONST);

    let r = get_number();
    println!("The value of r is {}", r);

    // shadowing
    let x = x + 1;
    // 日常更推荐使用 shadowing over mut
    println!("The new shadowing value of x is {}", x);
}