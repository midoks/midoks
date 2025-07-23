use argh::FromArgs;

#[derive(FromArgs)]
/// 网络工具
struct Args {
    #[argh(option, short = 'p')]
    /// 端口号
    port: u16,

    #[argh(switch)]
    /// 启用调试模式
    debug: bool,
}

fn main() {
    let args: Args = argh::from_env();
    println!("端口: {}, 调试模式: {}", args.port, args.debug);
}
