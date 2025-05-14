
fn echo(s: String) {
    println!("{}", s);
}

fn echo_borrow(s: &String) {
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(" Changed!");
}

fn main() {
    // rust 借用带来的开发的不便
    // let s = String::from("Hello world");
    // // echo 并不想拥有 s 的所有权，它只是想临时使用
    // echo(s);
    // // borrow of moved value: `s`
    // // println!("{}", s);

    /*
    借用的引入
    */
    let s = String::from("Hello world");
    echo_borrow(&s);
    println!("{}", s);

    /*
    可变引用和不可变引用
    */
    let mut s1 = String::from("mut Hello World");
    change(&mut s1);
    println!("{}", s1);

    /*
    同一时间内至多只能有一个可变引用
    */
    // let mut s2 = String::from("mut Hello World2");
    // let s2_ref = &mut s2;
    // // 防止两个线程同时拥有同一块内存的修改权限
    // // error[E0499]: cannot borrow `s2` as mutable more than once at a time
    // let s2_ref2 = &mut s2;
    // println!("{}", s2_ref);
    // println!("{}", s2_ref2);

    let mut s2 = String::from("mut Hello World2");
    let s2_ref = &mut s2;
    println!("{}", s2_ref);
    let s2_ref2 = &mut s2;
    println!("{}", s2_ref2);

}