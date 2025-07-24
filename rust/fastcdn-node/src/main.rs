use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// 一个强大的文件处理工具
#[derive(Parser, Debug)]
#[command(
    author = "midoks <midoks@163.com>",
    version = "0.0.1",
    about = "fastcdn-node",
    long_about = "这个工具提供了多种文件操作功能，包括复制、移动、删除和查看文件信息"
)]

struct Cli {
    /// 启用详细输出
    #[arg(short, long, global = true)]
    verbose: bool,

    /// 操作模式
    #[command(subcommand)]
    command: Commands,
}

/// 文件信息显示格式
#[derive(ValueEnum, Clone, Debug)]
enum InfoFormat {
    /// 纯文本格式
    Text,
    /// JSON格式
    Json,
    /// YAML格式
    Yaml,
}

/// 搜索选项
#[derive(Parser, Debug)]
#[group(required = false, multiple = false)]
struct SearchOptions {
    /// 区分大小写搜索
    #[arg(short = 'c', long)]
    case_sensitive: bool,

    /// 全词匹配
    #[arg(short = 'w', long)]
    whole_word: bool,

    /// 使用正则表达式
    #[arg(short = 'r', long)]
    regex: bool,
}

/// 支持的操作命令
#[derive(Subcommand, Debug)]
enum Commands {
    /// 复制文件或目录
    Copy {
        /// 源文件路径
        source: PathBuf,

        /// 目标路径
        destination: PathBuf,

        /// 覆盖已存在的文件
        #[arg(short, long)]
        force: bool,

        /// 递归复制目录
        #[arg(short, long)]
        recursive: bool,
    },

    /// 移动或重命名文件
    Move {
        /// 源文件路径
        source: PathBuf,

        /// 目标路径
        destination: PathBuf,

        /// 覆盖已存在的文件
        #[arg(short, long)]
        force: bool,
    },

    /// 删除文件或目录
    Delete {
        /// 目标路径
        target: PathBuf,

        /// 递归删除目录
        #[arg(short, long)]
        recursive: bool,

        /// 不显示确认提示
        #[arg(short = 'y', long)]
        no_confirm: bool,
    },

    /// 查看文件信息
    Info {
        /// 目标文件路径
        file: PathBuf,

        /// 信息显示格式
        #[arg(short, long, value_enum, default_value_t = InfoFormat::Text)]
        format: InfoFormat,
    },

    /// 搜索文件内容
    Search {
        /// 搜索模式 (正则表达式)
        pattern: String,

        /// 搜索路径 (默认为当前目录)
        path: Option<PathBuf>,

        /// 搜索选项
        #[command(flatten)]
        options: SearchOptions,
    },
}

fn main() {
    let args = Cli::parse();

    println!("命令行参数解析结果:");
    println!("{:#?}", args);

    // 在实际应用中，这里会根据解析的参数执行相应的操作
    match &args.command {
        Commands::Copy {
            source,
            destination,
            force,
            recursive,
        } => {
            println!(
                "执行复制操作: {} -> {} (force: {}, recursive: {})",
                source.display(),
                destination.display(),
                force,
                recursive
            );
        }
        Commands::Move {
            source,
            destination,
            force,
        } => {
            println!(
                "执行移动操作: {} -> {} (force: {})",
                source.display(),
                destination.display(),
                force
            );
        }
        Commands::Delete {
            target,
            recursive,
            no_confirm,
        } => {
            println!(
                "执行删除操作: {} (recursive: {}, no_confirm: {})",
                target.display(),
                recursive,
                no_confirm
            );
        }
        Commands::Info { file, format } => {
            println!("查看文件信息: {} (格式: {:?})", file.display(), format);
        }
        Commands::Search {
            pattern,
            path,
            options,
        } => {
            let search_path = path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "当前目录".to_string());

            println!("执行搜索操作: '{}' 在 {}", pattern, search_path);

            if options.case_sensitive {
                println!(" - 区分大小写");
            } else if options.whole_word {
                println!(" - 全词匹配");
            } else if options.regex {
                println!(" - 使用正则表达式");
            }
        }
    }

    if args.verbose {
        println!("详细模式已启用");
    }
}
