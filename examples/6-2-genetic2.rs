
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Line<T> {
    a: Point<T>,
    b: Point<T>,
}

fn main() {

    let point1: Point<u32> = Point { x: 0, y: 0 };
    let line = Line{a: point1, b: Point { x: 1, y: 2 }};
    println!("{:?}", line);
}