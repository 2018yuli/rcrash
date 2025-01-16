// 元组结构
struct Pair(i32, f32);
// 标准的 c 结构

#[derive(Debug)] // 派生属性
pub struct Persion {
    name: String,
    age: u32,
}
// 单元结构
#[derive(Debug)]
struct Unit;

fn main() {
    
    let pair = Pair(18, 4.2);
    println!("pair.0 = {}", pair.0);

    let jack = Persion {
        name: String::from("jack"),
        age: 6,
    };
    println!("jack.name = {}, jack.age = {}, jack = {:?}", jack.name, jack.age, jack);

    let unit = Unit;
    println!("unit = {:?}", unit);
}