use clap::Args;
use colored::*;
use rand::seq::SliceRandom;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

use crate::utils::{cat_face, load_waifu_lines, rainbow, speak};

/// 🏠 `waifu cd` 子命令参数
#[derive(Args)]
pub struct CdArgs {
    /// 要切换的目标路径
    pub target: String,

    /// 撒娇语音播报（日语）
    #[arg(long)]
    pub miao: bool,
}

pub fn run_cd(args: CdArgs) -> io::Result<()> {
    let lines = load_waifu_lines();
    let path = Path::new(&args.target);
    let path_str = args.target.clone();

    if path.exists() && path.is_dir() {
        // ✅ 成功：撒娇输出 + 输出目标路径
        let pool = lines
            .cd
            .success
            .choose(&mut rand::thread_rng())
            .expect("cd.success 文案为空");

        let line_cn = pool.cn.replace("{path}", &path_str);
        let line_jp = pool.jp.replace("{path}", &path_str);

        // 猫脸固定黄色，后半段做渐变

        println!("{} {}", cat_face(&lines).bright_yellow(), rainbow(&line_cn, 0));


        if args.miao {
            speak(&line_jp);
        }

        // ✅ 输出路径写入临时文件（shell 去读）
        let tmp_path = env::temp_dir().join("waifu_cd_path.txt");
        if let Err(e) = fs::write(&tmp_path, &path_str) {
            eprintln!("写入临时文件失败：{}", e);
        }
    } else {
        // ❌ 失败：撒娇输出
        let pool = lines
            .cd
            .fail
            .choose(&mut rand::thread_rng())
            .expect("cd.fail 文案为空");

        let line_cn = pool.cn.replace("{path}", &path_str);
        let line_jp = pool.jp.replace("{path}", &path_str);

        println!("{} {}", cat_face(&lines).bright_red(), rainbow(&line_cn, 0));


        if args.miao {
            speak(&line_jp);
        }
    }

    Ok(())
}
