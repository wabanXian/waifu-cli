use clap::Args;
use rand::seq::SliceRandom;
use std::path::Path;
use rand::Rng;
use crate::rainbow_mod::RAINBOW_STOPS;
use crate::utils::{cat_face, load_waifu_lines, rainbow, speak, AppResult};

/// 🏠 `waifu cd` 子命令参数
#[derive(Args)]
pub struct CdArgs {
    /// 要切换的目标路径
    pub target: String,

    /// 撒娇语音播报（日语）
    #[arg(long)]
    pub miao: bool,
}

pub fn run_cd(args: CdArgs) -> AppResult {
    let lines = load_waifu_lines()?;
    let path = Path::new(&args.target);
    let path_str = args.target.clone();
    let offset = rand::thread_rng().gen_range(0..RAINBOW_STOPS.len() as u8);

    if path.exists() && path.is_dir() {
        // ✅ 成功：撒娇输出 + 输出目标路径
        let pool = lines
            .cd
            .success
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| "cd.success 文案为空".to_string())?;

        let line_cn = pool.cn.replace("{path}", &path_str);
        let line_jp = pool.jp.replace("{path}", &path_str);

        // 猫脸固定黄色，后半段做渐变
        eprintln!(
            "{} {}",
            // cat_face(&lines).bright_yellow(),
            rainbow(cat_face(&lines), offset),
            rainbow(&line_cn, offset)
        );

        if args.miao {
            speak(&line_jp);
        }

        println!("{path_str}");
    } else {
        // ❌ 失败：撒娇输出
        let pool = lines
            .cd
            .fail
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| "cd.fail 文案为空".to_string())?;

        let line_cn = pool.cn.replace("{path}", &path_str);
        let line_jp = pool.jp.replace("{path}", &path_str);

        eprintln!("{} {}", rainbow(cat_face(&lines),0), rainbow(&line_cn, 0));

        if args.miao {
            speak(&line_jp);
        }

        return Err(format!("目标目录不存在：{path_str}"));
    }

    Ok(())
}
