
fn main() {
    // 1 + 2 + ... + 100
    let mut sum = 0;
    let mut num = 1;
    let result = loop {
        sum += num;
        num += 1;
        if num > 100 {
            break sum;
        }
    };
    println!("1 + 2 + ... + 100 = {}", result);
}