use clap::Args;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::utils::{cat_face, load_waifu_lines, rainbow, speak, AppResult, VoiceLine};

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

pub fn run_ps(args: PsArgs) -> AppResult {
    let lines = load_waifu_lines()?;
    let offset = rand::thread_rng().gen_range(0..13);
    let sort = args.sort.trim().to_lowercase();

    if !matches!(sort.as_str(), "cpu" | "mem" | "memory") {
        return Err(format!("不支持的排序字段：{}", args.sort));
    }

    // 开场文案（来自 waifu.json）
    let VoiceLine { cn, jp } = lines.ps.success.choose(&mut rand::thread_rng())
        .ok_or_else(|| "ps.success 文案为空".to_string())?;
    let cn = cn.replace("{top}", &args.top.to_string())
               .replace("{sort}", &args.sort);
    let jp = jp.replace("{top}", &args.top.to_string())
               .replace("{sort}", &args.sort);

    eprintln!(
        "{} {}",
        rainbow(cat_face(&lines), offset),
        rainbow(&cn, offset)
    );
    if args.miao { speak(&jp); }

    println!(
        "{}",
        serde_json::json!({ "top": args.top, "sort": sort })
    );

    // 尾巴（可选）
    // if let Some(foot) = lines.ps.footer.as_ref().and_then(|v| v.choose(&mut rand::thread_rng())) {
    //     println!(
    //         "{} {}",
    //         rainbow(cat_face(&lines), (offset+3)%13),
    //         rainbow(&foot.cn, (offset+3)%13)
    //     );
    //     if args.miao { speak(&foot.jp); }
    // }

    Ok(())
}
