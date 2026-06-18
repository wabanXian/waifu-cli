use crate::utils::{cat_face, load_waifu_lines, rainbow, speak, AppResult, RAINBOW_STOPS};
use clap::Args;
use colored::*;
use std::fs;
use std::path::Path;
use  rand::Rng;

/// 📂 `waifu ls` 子命令参数
#[derive(Args)]
pub struct LsArgs {
    /// 要查看的目录路径（默认为当前目录）
    #[arg(default_value = ".")]
    pub path: String,

    /// 撒娇语音播报（日语）
    #[arg(long)]
    pub miao: bool,
}

/// 🏁 waifu ls 入口
pub fn run_ls(args: LsArgs) -> AppResult {
    let lines = load_waifu_lines()?;
    let path = Path::new(&args.path);

    if !path.exists() || !path.is_dir() {
        eprintln!("{} {}", "呜呜……这个地方找不到哟 >_<".red(), args.path);

        if args.miao {
            speak("うぅ……このフォルダ、見つからないよ〜");
        }

        return Err(format!("目标目录不存在：{}", args.path));
    }

    // 解析路径
    let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let mut display_path = abs_path.display().to_string();
    let offset = rand::thread_rng().gen_range(0..RAINBOW_STOPS.len() as u8);
    if display_path.starts_with(r"\\?\") {
        display_path = display_path.trim_start_matches(r"\\?\").to_string();
    }

    // 输出头部撒娇语句
    // println!(
    //     "{}\n{}",
    //     format!("{} {}", cat_face(&lines), lines.ls.header.cn)
    //         .bright_magenta()
    //         .bold(),
    //     lines.ls.path.replace("{path}", &display_path).bold()
    // );
    eprintln!(
        "{}\n{}",
        format!(
            "{} {}",
            rainbow(cat_face(&lines), offset),
            rainbow(&lines.ls.header.cn, offset)
        )
        .bold(),
        rainbow(&lines.ls.path.replace("{path}", &display_path), 0).bold()
    );
    

    // 🔍（可选）统计文件数（你可以不做，用 powershell 自己统计）
    let count = fs::read_dir(path)
        .map_err(|e| format!("读取目录 {} 失败：{e}", args.path))?
        .count();

    // 输出文件数段的彩色提示
    let msg = lines
        .ls
        .count
        .iter()
        .find(|range| count >= range.min && count <= range.max)
        .map(|range| range.cn.as_str())
        .unwrap_or("哼，主人不给我设定语句喵！");

    // println!("\n{}", rainbow(msg, 0));
    eprintln!("\n{} {}", rainbow(cat_face(&lines), offset), rainbow(msg, offset));

    if args.miao {
        if let Some(jp) = lines
            .ls
            .count
            .iter()
            .find(|range| count >= range.min && count <= range.max)
            .map(|range| range.jp.as_str())
        {
            speak(jp);
        }
    }

    println!("{display_path}");

    Ok(())
}
