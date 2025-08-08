use clap::Args;
use colored::*;
use rand::{seq::SliceRandom, Rng};
use std::process::Command;

use crate::utils::{cat_face, load_waifu_lines, rainbow, speak};

/// 🧹 `waifu clear` 子命令
#[derive(Args)]
pub struct ClearArgs {
    /// 撒娇语音播报（日语）
    #[arg(long)]
    pub miao: bool,
}

pub fn run_clear(args: ClearArgs) {
    // 先清屏
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }

    // 加载台词
    let lines = load_waifu_lines();
    let pool = lines.clear.success.choose(&mut rand::thread_rng()).expect("clear.success 文案为空");

    let offset = rand::thread_rng().gen_range(0..6);

    println!(
        "{} {}",
        rainbow(cat_face(&lines), offset),
        rainbow(&pool.cn, offset)
    );

    if args.miao {
        speak(&pool.jp);
    }
}