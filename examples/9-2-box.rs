
/*
box 没有速度上的性能损失：主要用于：
1. 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
3. 当希望拥有一个值并只关心它的类型是否实现了特定trait 而不是其具体类型的时候
*/

// ---- recursive without indirection
// enum List {
//     Cons(i32, List),
//     Nil
// }

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

fn test() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::read("/temp/not_exist")?;
    Ok(())
}

fn main() {
    // rust 中的数组也是栈上面的
    // Box 堆中的内容在其变量声明周期结束后，会自动释放
    let b = Box::new(5);
    println!("b = {}", b);

    // 1. 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
    let list = List::Cons(0, Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))));
    println!("list = {:?}", list);

    // 2. 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
    //      存储在栈中的数据，在转移所有权的时候，使用 Box 指向堆，可以防止大量的内存拷贝

    // a 被存储在栈上（大量数据）
    let a = [0; 1024 * 512];
    println!("a length {}", a.len());

    // a 被拷贝到堆上，即便使用 Box::new([0; 1024 * 512]); 也是会先在栈上分配，然后被拷贝到堆上
    let a_box_with_copy = Box::new(a);
    println!("a_box_with_copy length {}", a_box_with_copy.len());

    // 3. 当希望拥有一个值并只关心它的类型是否实现了特定trait 而不是其具体类型的时候
    let r = test();
    r.unwrap_or_else(|err| {
        eprintln!("Error occurred: {}", err);
    });
    // match r {
    //     Ok(_) => (),
    //     Err(_) => (),
    // }
}