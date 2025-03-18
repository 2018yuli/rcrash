mod mod1 {
    pub const MESSAGE: &str = "Hello World!";

    // pub(self)，对本模块以及子模块都可访问
    // （不加也是对本模块以及子模块都可访问）
    // 主要用于代码上的标记
    pub(self) const NUMBER: u32 = 42;

    pub mod mod2 {
        pub const MESSAGE: &str = "Hello World!";

        pub fn say42() {
            println!("{}", super::NUMBER);
        }
    }

    // 成员在整个 crate 范围内可以被访问
    pub(crate) enum CrateEnum {
        Item = 4
    }

    
}

fn main() {
    // constant `MESSAGE` is private
    // println!("{}", mod1::MESSAGE)
    println!("{}", mod1::MESSAGE);

    // module `mod2` is private
    // println!("{}", mod1::mod2::MESSAGE);

    println!("{}", mod1::mod2::MESSAGE);
    println!("{}", mod1::CrateEnum::Item as u32);
    mod1::mod2::say42();
}