mod config;
mod doctor;
mod waifu_git;
mod waifu_update;
mod style;
mod utils;
mod voice;
mod waifu_cd;
mod waifu_ls;
mod waifu_ping;
mod waifu_cat;
mod waifu_echo;
mod waifu_clear;
mod waifu_ps;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

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
    /// 检查项目资源、PowerShell 环境和可选语音功能
    Doctor(doctor::DoctorArgs),
    /// 只读的 Git 猫娘助手
    Git(waifu_git::GitArgs),
    /// 更新已安装的 waifu-cli (通过 update.ps1)
    Update,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let (command, result) = match cli.command {
        Commands::Ping(args) => ("ping", waifu_ping::run_ping(args)),
        Commands::Cd(args) => ("cd", waifu_cd::run_cd(args)),
        Commands::Ls(args) => ("ls", waifu_ls::run_ls(args)),
        Commands::Cat(args) => ("cat", waifu_cat::run_cat(args)),
        Commands::Echo(args) => ("echo", waifu_echo::run_echo(args)),
        Commands::Clear(args) => ("clear", waifu_clear::run_clear(args)),
        Commands::Ps(args) => ("ps", waifu_ps::run_ps(args)),
        Commands::Doctor(args) => ("doctor", doctor::run(args)),
        Commands::Git(args) => ("git", waifu_git::run(args)),
        Commands::Update => ("update", waifu_update::run()),
    };

    if let Err(error) = result {
        eprintln!("{command} 执行失败：{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
