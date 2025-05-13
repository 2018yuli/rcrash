
fn main() {
    /*
    Rust 中每个值都绑定有一个变量,称为该值的所有者。
    每个值只有一个所有者,而且每个值都有它的作用域 （即大括号）。
    一旦当这个值离开作用域,这个值占用的内存将被回收
    */

    // 1. 作用域的范围
    // let value1 = 1;
    // println!("{}", value1);
    // {
    //     let value2 = 2;
    // }
    // // cannot find value `value2` in this scope
    // println!("{}", value2);

    // 2. 所有权转移
    // let s1 = String::from("Hello World!");
    // let s2 = s1;
    // println!("{}", s2);

    // 2.1 所有权转移
    // let s1 = String::from("Hello World!");
    // let s2 = s1;
    // // borrow of moved value: `s1`
    // println!("{}", s1);

    // 2.2 不可变引用
    // let mut  s3 = "Hello World";
    // let mut s4 = s3;
    // // cannot use `+=` on type `&str`
    // s4 += "!";
    // println!("{}", s4);
    // println!("{}", s3);
    // println!("{}", s4);

    // 2.3 离开了s1 的子作用域，"Hello World" 因为发生了所有权转移，所以不会被销毁
    let s2: String;
    {
        let s1 = String::from("Hello World");
        s2 = s1;
        // borrow of moved value: `s1`
        // println!("{}", s1);
    }
    println!("{}", s2);
    
}