use clap::Args;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use crate::utils::{cat_face, load_waifu_lines, rainbow, speak, AppResult};

const MAX_FILE_SIZE: u64 = 16 * 1024 * 1024;
const MAX_LINE_SIZE: usize = 1024 * 1024;

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
        let file = File::open(p)
            .map_err(|e| format!("打开文件 {} 失败：{e}", args.path))?;
        let metadata = file
            .metadata()
            .map_err(|e| format!("读取文件 {} 元数据失败：{e}", args.path))?;
        if metadata.len() > MAX_FILE_SIZE {
            return Err(format!(
                "文件过大：{}（最大允许 16 MiB）",
                args.path
            ));
        }

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

        let mut reader = BufReader::new(file);
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = match reader.read_line(&mut line) {
                Ok(bytes_read) => bytes_read,
                Err(e) if e.kind() == io::ErrorKind::InvalidData => {
                    return Err(format!("文件不是有效的 UTF-8 文本：{}", args.path));
                }
                Err(e) => return Err(format!("读取文件 {} 失败：{e}", args.path)),
            };
            if bytes_read == 0 {
                break;
            }

            let content = line.strip_suffix('\n').unwrap_or(&line);
            let content = content.strip_suffix('\r').unwrap_or(content);
            if content.len() > MAX_LINE_SIZE {
                return Err(format!(
                    "单行内容过长：{}（最大允许 1 MiB）",
                    args.path
                ));
            }

            write!(stdout, "{}", rainbow(&line, offset))
                .map_err(|e| format!("输出文件 {} 失败：{e}", args.path))?;
        }

        writeln!(stdout).map_err(|e| format!("输出文件 {} 失败：{e}", args.path))?;
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
