use clap::Args;
use rand::{seq::SliceRandom, Rng};
use std::process::Command;

use crate::utils::{cat_face, load_waifu_lines, rainbow, speak, AppResult};

/// 🧹 `waifu clear` 子命令
#[derive(Args)]
pub struct ClearArgs {
    /// 撒娇语音播报（日语）
    #[arg(long)]
    pub miao: bool,
}

pub fn run_clear(args: ClearArgs) -> AppResult {
    // 先清屏
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()
    } else {
        Command::new("clear").status()
    }
    .map_err(|e| format!("启动清屏程序失败：{e}"))?;

    if !status.success() {
        return Err(format!("清屏程序退出异常：{status}"));
    }

    // 加载台词
    let lines = load_waifu_lines()?;
    let pool = lines.clear.success.choose(&mut rand::thread_rng())
        .ok_or_else(|| "clear.success 文案为空".to_string())?;

    let offset = rand::thread_rng().gen_range(0..6);

    println!(
        "{} {}",
        rainbow(cat_face(&lines), offset),
        rainbow(&pool.cn, offset)
    );

    if args.miao {
        speak(&pool.jp);
    }

    Ok(())
}
