pub use crate::style::{rainbow, RAINBOW_STOPS};
use chrono::{Local, Timelike};
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub type AppResult<T = ()> = Result<T, String>;

/// ⏰ 获取当前时间段（morning / afternoon / evening）
pub fn get_time_period() -> &'static str {
    let hour = Local::now().hour();
    match hour {
        5..=10 => "morning",
        11..=17 => "afternoon",
        _ => "evening",
    }
}

/// 🐾 从 JSON 加载的猫猫颜文字池中抽一个
pub fn cat_face(lines: &WaifuLines) -> &str {
    lines
        .cat_faces
        .choose(&mut rand::thread_rng())
        .map(|s| s.as_str())
        .unwrap_or("😿")
}

/// 🗣 播报日语台词（调用 speak.py）
pub fn speak(japanese: &str) {
    let exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("🐍💥 无法定位语音脚本：{e}");
            return;
        }
    };
    let script_path = exe_path.with_file_name("speak.py");

    match Command::new("python")
        .arg(script_path)
        .arg(japanese)
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("🌀 播报脚本退出异常：{}", output.status);
            }
        }
        Err(e) => {
            eprintln!("🐍💥 无法调用 Python 播报：{e:?}");
        }
    }
}

//
// 🎒 JSON 映射结构体
//

#[derive(Debug, Deserialize)]
pub struct VoiceLine {
    pub cn: String,
    pub jp: String,
}

#[derive(Debug, Deserialize)]
pub struct PingLines {
    pub success: HashMap<String, Vec<VoiceLine>>,
    pub fail: HashMap<String, Vec<VoiceLine>>,
}

#[derive(Debug, Deserialize)]
pub struct CdLines {
    pub success: Vec<VoiceLine>,
    pub fail: Vec<VoiceLine>,
}

#[derive(Debug, Deserialize)]
pub struct LsLines {
    pub header: VoiceLine,
    pub path: String,
    pub count: Vec<LsCountLine>,
}

#[derive(Debug, Deserialize)]
pub struct LsCountLine {
    pub min: usize,
    pub max: usize,
    pub cn: String,
    pub jp: String,
}

#[derive(Debug, Deserialize)]
pub struct ClearLines {
    pub success: Vec<VoiceLine>,
}

#[derive(Debug, Deserialize)]
pub struct EchoLines {
    pub sayings: Vec<VoiceLine>,
    pub empty: VoiceLine,
}

#[derive(Debug, Deserialize)]
pub struct CatLines {
    pub success: Vec<VoiceLine>, // 读取成功时的撒娇文案
    pub fail: Vec<VoiceLine>,    // 找不到/读取失败时
}

#[derive(Debug, Deserialize)]
pub struct PsLines {
    pub success: Vec<VoiceLine>,        // 开场
    #[allow(dead_code)]
    pub fail: Vec<VoiceLine>,           // 异常/空
    #[allow(dead_code)]
    pub footer: Option<Vec<VoiceLine>>, // 结尾（可选）
}

#[derive(Debug, Deserialize)]
pub struct WaifuLines {
    pub cat_faces: Vec<String>,
    pub ping: PingLines,
    pub cd: CdLines,
    pub ls: LsLines,
    pub clear: ClearLines,
    pub echo: EchoLines,
    pub cat: CatLines,
    pub ps: PsLines,
}

/// 📦 加载 JSON 文件为 WaifuLines
pub fn load_waifu_lines() -> AppResult<WaifuLines> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("无法定位当前程序：{e}"))?;
    let json_path: PathBuf = exe_path.with_file_name("waifu.json");
    let raw = fs::read_to_string(&json_path)
        .map_err(|e| format!("读取 {} 失败：{e}", json_path.display()))?;
    serde_json::from_str(&raw)
        .map_err(|e| format!("解析 {} 失败：{e}", json_path.display()))
}
