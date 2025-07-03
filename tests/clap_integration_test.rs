// cargo test --test clap_integration_test

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[cfg(test)]
mod tests {
    use super::*; // 引入主函数中的所有代码
                  // use clap::CommandFactory; // 用于验证命令行解析

    // 测试命令行参数解析
    #[test]
    fn test_args_parsing() {
        // 模拟命令行参数
        let args = Args::try_parse_from(["", "--name", "Alice", "--count", "3"]).unwrap();

        // 验证解析后的参数
        assert_eq!(args.name, "Alice");
        assert_eq!(args.count, 3);
    }

    // 测试默认值
    #[test]
    fn test_default_count() {
        let args = Args::try_parse_from(["", "--name", "Bob"]).unwrap();

        // 验证默认值（count 默认是 1）
        assert_eq!(args.name, "Bob");
        assert_eq!(args.count, 1);
    }

    // 测试解析错误
    #[test]
    #[should_panic]
    fn test_invalid_args() {
        // 传递一个无效的参数，程序应该 panic
        Args::try_parse_from(["", "--invalid", "value"]).unwrap();
    }
}
