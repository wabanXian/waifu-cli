mod rainbow_mod;
mod utils;
mod waifu_cd;
mod waifu_ls;
mod waifu_ping;
mod waifu_cat;
mod waifu_echo;
mod waifu_clear;
mod waifu_ps;

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
    Cat(waifu_cat::CatArgs), 
    Echo(waifu_echo::EchoArgs),
    Clear(waifu_clear::ClearArgs),
    Ps(waifu_ps::PsArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ping(args) => waifu_ping::run_ping(args),
        Commands::Cd(args) => {
            if let Err(e) = waifu_cd::run_cd(args) {
                eprintln!("cd 执行失败：{e}");
            }
        }
        Commands::Ls(args) => waifu_ls::run_ls(args),
        Commands::Cat(args)  => waifu_cat::run_cat(args),
        Commands::Echo(args)  => waifu_echo::run_echo(args),
        Commands::Clear(args)  => waifu_clear::run_clear(args),
        Commands::Ps(args)  => waifu_ps::run_ps(args),
    }
}
