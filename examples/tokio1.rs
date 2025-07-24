// cargo expand --example tokio1
/*
fn main() {
    let body = async {
        let a = 10;
        let b = 10;
        {
            ::std::io::_print(format_args!("{0} + {1} = {2}\n", a, b, a + b));
        };
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
*/

#[tokio::main]
async fn main() {
    let a = 10;
    let b = 10;
    println!("{} + {} = {}", a, b, a + b);
}
