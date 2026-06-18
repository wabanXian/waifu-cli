use clap::Args;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs;
use std::path::Path;

use crate::utils::{cat_face, load_waifu_lines, rainbow, speak, AppResult};

/// 🐱 `waifu cat` 子命令参数
#[derive(Args)]
pub struct CatArgs {
    /// 要查看的文件路径
    pub path: String,

    /// 撒娇语音播报（日语）
    #[arg(long)]
    pub miao: bool,
}

/// 🏁 waifu cat 入口
pub fn run_cat(args: CatArgs) -> AppResult {
    let lines = load_waifu_lines()?;
    let p = Path::new(&args.path);
    let offset = rand::thread_rng().gen_range(0..13);

    if p.exists() && p.is_file() {
        // ✅ 成功撒娇文案（来自 waifu.json）
        let pool = lines.cat.success.choose(&mut rand::thread_rng())
            .ok_or_else(|| "cat.success 文案为空".to_string())?;
        let cn = pool.cn.replace("{path}", &args.path);
        let jp = pool.jp.replace("{path}", &args.path);

        println!(
            "{} {}",
            rainbow(cat_face(&lines), offset),
            rainbow(&cn, offset)
        );
        if args.miao { speak(&jp); }

        // ✅ 实际读文件
        match fs::read_to_string(p) {
            Ok(content) => {
                // 你要整段染色就这样；不想整段彩色就直接 println!("{}", content)
                println!("{}", rainbow(&content, offset));
            }
            Err(e) => {
                let pool = lines.cat.fail.choose(&mut rand::thread_rng())
                    .ok_or_else(|| "cat.fail 文案为空".to_string())?;
                let cn = pool.cn.replace("{path}", &args.path);
                let jp = pool.jp.replace("{path}", &args.path);
                println!("{} {}", rainbow(cat_face(&lines), 2), rainbow(&cn, 2));
                if args.miao { speak(&jp); }
                return Err(format!("读取文件 {} 失败：{e}", args.path));
            }
        }
    } else {
        // ❌ 找不到文件
        let pool = lines.cat.fail.choose(&mut rand::thread_rng())
            .ok_or_else(|| "cat.fail 文案为空".to_string())?;
        let cn = pool.cn.replace("{path}", &args.path);
        let jp = pool.jp.replace("{path}", &args.path);

        println!(
            "{} {}",
            rainbow(cat_face(&lines), offset),
            rainbow(&cn, offset)
        );
        if args.miao { speak(&jp); }
        let reason = if p.exists() {
            "目标路径不是文件"
        } else {
            "目标文件不存在"
        };
        return Err(format!("{reason}：{}", args.path));
    }

    Ok(())
}
