mod waifu_ping;
mod utils;
mod waifu_cd;
mod waifu_ls;
mod rainbow_mod;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "waifu", version, about = "会撒娇的终端工具箱 ✨")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 会撒娇的 ping 工具 ✨
    Ping(waifu_ping::PingArgs),
    Cd(waifu_cd::CdArgs),
    Ls(waifu_ls::LsArgs),
}

#[tokio::main] // 用 Tokio 作为异步入口
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ping(args) => waifu_ping::run_ping(args), // 同步，照常调用
        Commands::Cd(args) => {
            // 异步：等待动画完成；出错就友好提示
            if let Err(e) = waifu_cd::run_cd(args).await {
                eprintln!("cd 执行失败：{e}");
            }
        }
        Commands::Ls(args) => waifu_ls::run_ls(args), // 同步，照常调用
    }
}
