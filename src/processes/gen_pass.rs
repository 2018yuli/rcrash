use anyhow::Ok;
use rand::seq::{IndexedRandom, SliceRandom};

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*_";

pub fn process_generate_password(
    length: u8,
    upper: bool,
    lower: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    // Vec<u8> 常用于处理字节流，比如序列化、文件内容、网络数据等。
    let mut chars = Vec::new();

    if upper {
        // b"..." 是 字节字符串，extend_from_slice(...) 是把一段字节数组追加到 Vec<u8> 里。
        // b"ABC" 表示一个 字节串（byte string literal），它的类型是 &[u8; N]
        // b"..." 字面量不能包含非 ASCII 字符（比如中文），因为一个字节无法表示多字节字符。
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if numbers {
        chars.extend_from_slice(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if symbols {
        chars.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("UPPER won't be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        // let idx = rng.random_range(0..chars.len());
        // password.push(chars[idx] as char);
        let c = chars.choose(&mut rng).expect("chars won't be empty");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    // from_utf8_lossy: 将 &[u8] 类型的字节数组转换成 String，如果有非法 UTF-8 字节，就用 �（U+FFFD）代替
    let password = String::from_utf8(password)?;

    Ok(password)
}
