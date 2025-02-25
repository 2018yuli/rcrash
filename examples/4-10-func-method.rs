
// 函数
fn fibonatic(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        fibonatic(n - 1) + fibonatic(n - 2)
    }
}

// 方法
#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    // 构造方法
    fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }

    fn get_x(&self) -> u64 {
        self.x
    }

    fn set_x(&mut self, x: u64) {
        self.x = x
    }
}

fn main() {
    println!("fibonatic(10) = {}", fibonatic(10));

    let mut p = Point::new(10, 20);
    println!("{:?}, Point.x = {}, p.get_x() = {}", p, p.x, p.get_x());

    p.set_x(12);
    println!("p = {:?}", p)
}