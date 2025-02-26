use std::thread;

fn main() {
    let times3 = |n:u32| -> u32 {n * 3};

    println!("{}", times3(10));

    // move 将环境中的值，移到闭包内部
    // 使用场景：多线程，从主线程当中移动一个值到子线程
    let hell0_message = "hello world!";
    let _ = thread::spawn(move || {
        println!("{}", hell0_message);
    }).join();
}