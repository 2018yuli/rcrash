use rand::Rng;
use std::io;

fn main() {
    let secret_num = rand::rng().random_range(1..101);

    loop {
        // 获取用户的输入
        println!("Please input u guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
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