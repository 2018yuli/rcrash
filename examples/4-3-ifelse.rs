
fn main() {
    let n = 5;

    if n < 0 {
        println!("n is negative.")
    } else if n == 0 {
        println!("n is zero.")
    } else {
        println!("n is positive.")
    }

    // 所有分支必须返回相同的类型
    let m = if n <0 {
        2.0
    } else {
        3.0 
    };
    println!("m = {}", m)
    
}