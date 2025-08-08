use clap::Args;
use colored::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::env;
use std::fs;

use crate::utils::{cat_face, load_waifu_lines, rainbow, speak, VoiceLine};

/// 🐾 `waifu ps` 子命令参数（只负责撒娇&下发参数）
#[derive(Args)]
pub struct PsArgs {
    /// 展示前 N 个进程（默认 10）
    #[arg(short, long, default_value_t = 10)]
    pub top: usize,

    /// 排序字段：cpu 或 mem（默认 cpu）
    #[arg(short, long, default_value = "cpu")]
    pub sort: String,

    /// 撒娇语音播报（日语）
    #[arg(long)]
    pub miao: bool,
}

pub fn run_ps(args: PsArgs) {
    let lines = load_waifu_lines();
    let offset = rand::thread_rng().gen_range(0..13);

    // 开场文案（来自 waifu.json）
    if let Some(VoiceLine { cn, jp }) = lines.ps.success.choose(&mut rand::thread_rng()) {
        let cn = cn.replace("{top}", &args.top.to_string())
                   .replace("{sort}", &args.sort);
        let jp = jp.replace("{top}", &args.top.to_string())
                   .replace("{sort}", &args.sort);

        println!(
            "{} {}",
            rainbow(cat_face(&lines), offset),
            rainbow(&cn, offset)
        );
        if args.miao { speak(&jp); }
    }

    // ✅ 把参数写进临时文件（给 PowerShell 用）
    let tdir = env::temp_dir();
    let top_file  = tdir.join("waifu_ps_top.txt");
    let sort_file = tdir.join("waifu_ps_sort.txt");

    if let Err(e) = fs::write(&top_file, args.top.to_string()) {
        eprintln!("写入 waifu_ps_top.txt 失败：{e}");
    }
    if let Err(e) = fs::write(&sort_file, args.sort.trim()) {
        eprintln!("写入 waifu_ps_sort.txt 失败：{e}");
    }

    // 尾巴（可选）
    // if let Some(foot) = lines.ps.footer.as_ref().and_then(|v| v.choose(&mut rand::thread_rng())) {
    //     println!(
    //         "{} {}",
    //         rainbow(cat_face(&lines), (offset+3)%13),
    //         rainbow(&foot.cn, (offset+3)%13)
    //     );
    //     if args.miao { speak(&foot.jp); }
    // }
}
