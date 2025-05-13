
struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// 使用 traits 做为函数的参数
fn show<T: std::fmt::Display>(a: T) {
    println!("show: {}", a)
}

fn show_simple(a: impl std::fmt::Display) {
    println!("show: {}", a)
}

fn main() {
    // traits 类似于其他语言的接口
    let point = Point{x: 10, y: 20};
    println!("{}", point);
    show(point);
    let point2 = Point{x: 10, y: 20};
    show_simple(point2);
}