use rand::Rng;
use std::io;

fn main() {
    let secret_num = rand::rng().random_range(1..101);

    loop {
        // 获取用户的输入
        println!("Please input u guess:");
        // 创建一个可变的字符串变量 `guess`，用于存储用户输入的数据。
        // `String::new()` 是 Rust 提供的一个方法，它会创建一个新的、空的 `String` 类型的字符串。
        let mut guess = String::new();

        // 从标准输入（键盘）读取用户输入的内容，并存储到 `guess` 变量中。
        // `io::stdin().read_line(&mut guess)` 用于从标准输入读取一行字符串，并存入 `guess` 变量。
        // `.unwrap()` 用于解包 `read_line()` 返回的 `Result<T, E>`，如果读取失败，程序会直接崩溃。
        io::stdin().read_line(&mut guess).unwrap();

        // 将用户输入的字符串 `guess` 转换为整数类型 `u32`，以便后续进行比较。
        // `trim()` 方法用于去除字符串两端的空格和换行符，防止解析出错。
        // `parse()` 方法尝试将字符串转换为指定的数值类型，在这里是 `u32`。
        // 由于 `parse()` 可能会失败，所以我们使用 `match` 进行错误处理：
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,  // 如果解析成功，将数值赋值给 `guess_number`
            Err(_) => continue,  // 如果解析失败，跳过本次循环，重新让用户输入
        };


        // 判断大小
        if guess_number > secret_num {
            println!("Too big");
        } else if guess_number < secret_num {
            println!("Too small");
        } else {
            println!("u win!");
            break;
        }  

        // 如果正确的话，程序退出
    }
}