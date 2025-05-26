// 0 -> 1 \
//         | -> 4
// 2 -> 3 /

// 不像 Box，Rc 需要单独引入
use std::rc::Rc;


#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}

fn main() {
    // 带引入计数器的只能指针

    // RC: 实现一个值可以有多个所有者，Rc::new, Rc::clone（底层的计数+1）
    let four = Rc::new(List::Cons(4, Rc::new(List::Nil)));
    // let zero_one = List::Cons(0, Rc::new(List::Cons(1, four.clone())));
    // 一种更为推荐的，直观的做法，防止和 数据的 clone 混淆
    let zero_one = List::Cons(0, Rc::new(List::Cons(1, Rc::clone(&four))));
    let two_three = List::Cons(2, Rc::new(List::Cons(3, four)));
    println!("{:?}", zero_one);
    println!("{:?}", two_three);
}