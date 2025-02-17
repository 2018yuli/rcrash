enum Alphabet {
    A,
    B,
}
enum Symbol {
    Char(char),
    Number,
}

fn main() {
    // If let 简化了 match 的语法，当你仅仅需要匹配其中一个分支时
    let letter = Alphabet::A;

    match letter {
        Alphabet::A => {
            println!("It's A");
        }
       _ => {}
    }

    // 使用 if let
    if let Alphabet::A = letter {
        println!("It's A");
    }

    // if let 匹配带参数的 Enum，尽量使用 if let over match
    let letter = Symbol::Char('A');
    if let Symbol::Char(x) = letter {
        println!("It's {}", x);
    }
}