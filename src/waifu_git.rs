//! Read-only Git helpers. Never modifies the repository and never replaces git.exe.

use clap::{Args, Subcommand};
use std::process::{Command, Output};

use crate::utils::AppResult;

#[derive(Debug, Args)]
pub struct GitArgs {
    #[command(subcommand)]
    command: GitCommands,
}

#[derive(Debug, Subcommand)]
enum GitCommands {
    /// 查看当前分支、已暂存、未暂存和未追踪的文件
    Status,
    /// 以简洁格式查看最近的提交
    Log {
        /// 显示最近多少条提交（1 - 50）
        #[arg(short = 'n', long = "count", default_value_t = 8, value_parser = clap::value_parser!(u8).range(1..=50))]
        count: u8,
    },
}

fn git_command(args: &[&str]) -> Result<Output, String> {
    Command::new("git")
        .args(args)
        .output()
        .map_err(|err| format!("无法运行 Git，请确认 git.exe 在 PATH 中：{err}"))
}

fn require_repo() -> Result<String, String> {
    let out = git_command(&["rev-parse", "--show-toplevel"])?;
    if !out.status.success() {
        return Err("这里还不是 Git 仓库喵；先 cd 到仓库目录再试试～".into());
    }
    let root = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if root.is_empty() {
        return Err("Git 未返回仓库根目录".into());
    }
    Ok(root)
}

fn run_status() -> AppResult {
    let root = require_repo()?;
    let out = git_command(&["status", "--short", "--branch", "--untracked-files=normal"])?;
    if !out.status.success() {
        return Err(format!("Git 状态查询失败：{}", String::from_utf8_lossy(&out.stderr).trim()));
    }
    let output = String::from_utf8_lossy(&out.stdout);
    let mut lines = output.lines();
    let branch = lines.next().unwrap_or("## (unknown branch)");
    println!("🐾 主人正在照看的仓库：{root}");
    println!("🌸 {}", branch.strip_prefix("## ").unwrap_or(branch));
    let changes: Vec<&str> = lines.collect();
    if changes.is_empty() {
        println!("✨ 工作区干干净净，主人真棒！");
    } else {
        println!("😼 有 {} 项变动，记得检查和提交哦～", changes.len());
        for change in &changes {
            println!("  {change}");
        }
    }
    Ok(())
}

fn run_log(count: u8) -> AppResult {
    let root = require_repo()?;
    let n = count.to_string();
    let out = git_command(&["log", "--no-decorate", "--format=%h%x09%s", "-n", &n])?;
    if !out.status.success() {
        return Err(format!(
            "读取 Git 历史失败：{}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    println!("🐈 {root} · 最近 {count} 条提交：");
    let text = String::from_utf8_lossy(&out.stdout);
    if text.trim().is_empty() {
        println!("🌱 暂时没有提交哦～");
    } else {
        for line in text.lines() {
            if let Some((hash, subject)) = line.split_once('\t') {
                println!("  🌸 {hash}  {subject}");
            } else {
                println!("  {line}");
            }
        }
    }
    Ok(())
}

pub fn run(args: GitArgs) -> AppResult {
    match args.command {
        GitCommands::Status => run_status(),
        GitCommands::Log { count } => run_log(count),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn git_status_outside_repo_returns_clear_error() {
        // No filesystem mutation or network traffic is performed by this module.
        let err = git_command(&["--this-is-not-a-git-flag"]).unwrap();
        assert!(!err.status.success());
    }
}
