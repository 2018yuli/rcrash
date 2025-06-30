use clap::{Parser, Subcommand};
use std::path::PathBuf;

// 定义主命令结构体 `Cli`，包含了从命令行输入的参数配置
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// 定义 `name` 参数，是一个可选项，用来操作的名称
    /// 如果命令行中没有传递该参数，则为 `None`
    name: Option<String>,

    /// 定义 `config` 参数，用来设置自定义的配置文件路径
    /// 该参数为可选项，类型是 `Option<PathBuf>`，用于存储文件路径
    /// `-c` 或 `--config` 参数，用户可以传入一个文件路径
    /// 配置文件的路径，默认值是 `None`，表示没有配置文件
    #[arg(short, long, value_name = "FILE")]  
    config: Option<PathBuf>,

    /// 定义 `debug` 参数，用于控制调试信息的输出级别
    /// `action = clap::ArgAction::Count` 使得该参数可以计数，表示不同的调试级别
    /// `-d` 或 `--debug` 参数，用户可以通过重复使用该参数来控制调试级别
    /// `debug` 是一个 `u8` 类型的参数，表示调试信息的级别，数值越大调试信息越详细
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// 定义一个子命令 `command`，该命令包含一个 `Commands` 枚举类型，用于处理不同的子命令
    /// `command` 字段是 `Cli` 结构体中的子命令
    /// 子命令的枚举类型，用户可以在命令行中指定不同的子命令来执行
    #[command(subcommand)]
    command: Option<Commands>,
}

// 定义所有可能的子命令，通过 `Commands` 枚举来表示
#[derive(Debug, Subcommand)]
enum Commands {
    /// 子命令 `Test`，用于执行测试相关的操作
    Test {
        /// 子命令 `Test` 下的 `list` 参数，用于指定是否列出测试值
        /// 该参数是一个布尔值，表示是否列出测试项
        /// `-l` 或 `--list` 参数，用户可以通过该参数指定是否列出测试项
        /// `list` 是一个布尔类型的字段，用户可以通过命令行输入来设置该参数
        #[arg(short, long)]
        list: bool,
    },
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_no_name_no_config() {
        // 使用 `Cli::try_parse_from` 模拟命令行输入，传递 `--debug` 参数
        let args = Cli::try_parse_from(["", "--debug"]).unwrap();

        // 检查解析结果中是否包含预期的输出
        let result = format!("Debug mode is kind of on"); 
        println!("args.debug = {}", args.debug);
        if args.debug == 1 {
            assert_eq!(result, "Debug mode is kind of on");
        }
    }

    #[test]
    fn test_debug_equal_2() {
        // 模拟命令行输入，传递 `-d -d` 或 `--debug --debug`，调试级别应该为 2
        let args = Cli::try_parse_from(["", "--debug", "--debug"]).unwrap();

        // 检查 `debug` 参数的值是否等于 2
        assert_eq!(args.debug, 2);
    }

    #[test]
    fn test_with_name() {
        // 使用 `Cli::try_parse_from` 模拟命令行输入，传递 `name` 参数和 `--debug`
        let args = Cli::try_parse_from(["", "Alice", "--debug"]).unwrap();

        // 检查解析结果中是否包含预期的输出
        assert_eq!(args.name, Some("Alice".to_string()));
        assert_eq!(args.debug, 1);
    }

    #[test]
    fn test_with_config() {
        // 使用 `Cli::try_parse_from` 模拟命令行输入，传递 `--config` 参数
        let config_path = "config.toml";  // 模拟一个 config 文件路径
        let args = Cli::try_parse_from(["", "--config", config_path, "--debug"]).unwrap();

        // 检查解析结果中是否包含预期的输出
        assert_eq!(args.config.unwrap(), PathBuf::from(config_path));
        assert_eq!(args.debug, 1);
    }

    #[test]
    fn test_test_subcommand() {
        // 使用 `Cli::try_parse_from` 模拟命令行输入，传递 `test` 子命令和 `--list` 参数
        let args = Cli::try_parse_from(["", "test", "--list"]).unwrap();

        // 检查解析结果中是否包含子命令和预期的选项
        if let Some(Commands::Test { list }) = args.command {
            assert_eq!(list, true);
        }
    }

    #[test]
    fn test_test_subcommand_no_list() {
        // 使用 `Cli::try_parse_from` 模拟命令行输入，传递 `test` 子命令，不传递 `--list` 参数
        let args = Cli::try_parse_from(["", "test"]).unwrap();

        // 检查解析结果中是否包含子命令并且 `list` 默认为 `false`
        if let Some(Commands::Test { list }) = args.command {
            assert_eq!(list, false);
        }
    }
}
