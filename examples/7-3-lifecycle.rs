
// -> &str missing lifetime specifier
// this function's return type contains a borrowed value, 
// but the signature does not say whether it is borrowed from `str1` or `str2`
// fn bigger(str1: &str, str2: &str) -> &str {
//     if str1 > str2 {
//         str1
//     } else {
//         str2
//     }
// }

fn another(str1: &str, str2: &str) -> &'static str {
    "hello"
}

// 显式的注明函数的返回值的声明周期
// 一般使用`'+短小写字母` 来定义生命周期注解，一般使用 `'a`
// `'a` 并不会改变变量原本的生命周期
// 拥有相同生命周期注解的变量，他们的生命周期是一样的
fn bigger2<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1 > str2 {
        str1
    } else {
        str2
    }
}

// 声明周期结构体
// struct Person {
//     // missing lifetime specifier
//     name: &str,
// }

struct Person<'a> {
    name: &'a str,
}

struct People<'a> {
    name: &'a str,
}

fn main() {
    // error[E0106]: missing lifetime specifier
    // println!("{}", bigger("a", "b"));

    println!("{}", another("a", "b"));

    println!("{}", bigger2("a", "b"));

    let p = Person{name: "Jack"};
    println!("{}", p.name);

    let p = People{name: "Jackson"};
    println!("{}", p.name);

}