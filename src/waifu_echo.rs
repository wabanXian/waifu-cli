use clap::Args;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};

use crate::utils::{cat_face, load_waifu_lines, rainbow, speak};

/// 🗣 `waifu echo` 子命令参数
#[derive(Args)]
pub struct EchoArgs {
    /// 要输出的文字（多词会自动拼接）
    pub message: Vec<String>,

    /// 撒娇语音播报（日语）
    #[arg(long)]
    pub miao: bool,
}

pub fn run_echo(args: EchoArgs) {
    let lines = load_waifu_lines();
    let text = args.message.join(" ").trim().to_string();

    // 随机彩虹起点（建议用你公开的 stops 长度；没有就给个宽松范围）
    let offset = rand::thread_rng().gen_range(0..13);

    let (cn_line, jp_line) = if text.is_empty() {
        (lines.echo.empty.cn.clone(), lines.echo.empty.jp.clone())
    } else {
        let tpl = lines
            .echo
            .sayings
            .choose(&mut rand::thread_rng())
            .expect("echo.sayings 为空");
        (
            tpl.cn.replace("{text}", &text),
            tpl.jp.replace("{text}", &text),
        )
    };

    // 彩色输出（直接写 stdout，避免被宿主降级处理）
    let mut stdout = io::stdout();
    let cat = rainbow(cat_face(&lines), offset);
    let msg = rainbow(&cn_line, offset);
    writeln!(stdout, "{} {}", cat, msg).ok();

    // 语音（有就播，无就算了）
    if args.miao && !jp_line.trim().is_empty() {
        speak(&jp_line);
    }
}
