use crate::utils::{cat_face, load_waifu_lines, rainbow, speak};
use clap::Args;
use colored::*;
use std::fs;
use std::path::Path;

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
pub fn run_ls(args: LsArgs) {
    let lines = load_waifu_lines();
    let path = Path::new(&args.path);

    if !path.exists() || !path.is_dir() {
        println!("{} {}", "呜呜……这个地方找不到哟 >_<".red(), args.path);

        if args.miao {
            speak("うぅ……このフォルダ、見つからないよ〜");
        }

        return;
    }

    // 解析路径
    let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let mut display_path = abs_path.display().to_string();
    if display_path.starts_with(r"\\?\") {
        display_path = display_path.trim_start_matches(r"\\?\").to_string();
    }

    // 输出头部撒娇语句
    println!(
        "{}\n{}",
        format!("{} {}", cat_face(&lines), lines.ls.header.cn)
            .bright_magenta()
            .bold(),
        lines.ls.path.replace("{path}", &display_path).bold()
    );

    // 🔍（可选）统计文件数（你可以不做，用 powershell 自己统计）
    let count = match fs::read_dir(path) {
        Ok(entries) => entries.count(),
        Err(_) => 0,
    };

    // 输出文件数段的彩色提示
    let msg = lines
        .ls
        .count
        .iter()
        .find(|range| count >= range.min && count <= range.max)
        .map(|range| range.cn.as_str())
        .unwrap_or("哼，主人不给我设定语句喵！");

    println!("\n{}", rainbow(msg, 2));

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

    // ✅ 写入路径到临时文件，供 powershell 调用 ls
    use std::env;
    let tmp_path = env::temp_dir().join("waifu_ls_path.txt");
    if let Err(e) = fs::write(&tmp_path, &display_path) {
        eprintln!("写入 waifu_ls_path.txt 失败：{}", e);
    }
}
