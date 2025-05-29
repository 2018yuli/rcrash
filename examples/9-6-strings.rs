// 如果你正在定义一个函数，这个函数的参数是字符串类型，使用 str 的引用类型 &str
// 如果你在定义一个结构体，这个结构体的成员是字符串类型，使用 String 类型
// 其他
// 从本地打开一个文件.........

// 方法参数使用 &str 同时支持堆和字面量
fn echo(s: &str) {
    println!("{:?}", s);
}

struct Foo {
    // 成员变量避免使用 &str，因为 &str 是一个引用，导致需要给结构体写生命周期
    name: String,
}

fn main() {
    // "Hello World！" 编译到二进制中，保存在数据段的地方。叫做字符串的字面亮
    // str 类型几乎永远不会被使用
    // 我们总是使用 &str
    //   |_   str 代表的是内存中（数据段，不允许修改）的字符串数据
    // &str 可以引用数据段中的内容，也可以是堆里面的内容....
    let s = "Hello World!"; // s 是具有静态生命周期的 str 的引用 &'static str

    // String 类型是存储在堆中的，它拥有自己的数据，而不只是一个引用
    //      |_ 所以 String 是可以修改的
    let mut t = String::from(s);
    t.push_str("!!");
    println!("{:?}", t);

    echo(s);
    echo(&t);

    let foo = Foo {name: t};
    println!("{:?}", foo.name);

    let foo2 = Foo {name: String::from(s)};
    println!("{:?}", foo2.name);

}