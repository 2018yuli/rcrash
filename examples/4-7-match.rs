enum Alphabet {
    A,
    B,
}

fn main() {
    let letter = Alphabet::A;

    match letter {
        Alphabet::A => {
            println!("It's A");
        }
        Alphabet::B => {
            println!("It's B");
        }
    }

    let opcode: u8 = 42;

    match opcode {
        42 => {
            println!("bingo!");
        }
        _ => {
            println!("{}", opcode);
        }
    }
}