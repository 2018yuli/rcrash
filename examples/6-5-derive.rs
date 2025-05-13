

struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/**
 * Rust 编译起自动的帮我们结构体实现 traits
 *  - 结构体中的所有字段都已经实现了这个 traits 结构体才能添加这个 traints
 */
#[derive(Debug, PartialEq, Default)]
struct Point2<T> {
    x: T,
    y: T,
}


fn main() {
    // traits 类似于其他语言的接口
    let point = Point{x: 10, y: 20};
    println!("{}", point);
    let point2 = Point2{x: 10, y: 20};
    println!("{:?}", point2);

    let p1 = Point2{x: 10, y: 20};
    let p2 = Point2{x: 10, y: 20};
    if (p1.x == p2.x && p1.y == p2.y) {
        println!("p1 == p2");
    }
    println!("p1 == p2 over PartialEq: {}", p1 == p2);
    let p: Point2<i32> = Point2::default();
    println!("Point2::default = {:?}", p);
   
}