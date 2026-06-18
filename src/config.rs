use crate::utils::AppResult;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
    pub success: Vec<VoiceLine>,
    pub fail: Vec<VoiceLine>,
}

#[derive(Debug, Deserialize)]
pub struct PsLines {
    pub success: Vec<VoiceLine>,
    #[allow(dead_code)]
    pub fail: Vec<VoiceLine>,
    #[allow(dead_code)]
    pub footer: Option<Vec<VoiceLine>>,
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

pub fn load_waifu_lines() -> AppResult<WaifuLines> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("无法定位当前程序：{e}"))?;
    let json_path: PathBuf = exe_path.with_file_name("waifu.json");
    let raw = fs::read_to_string(&json_path)
        .map_err(|e| format!("读取 {} 失败：{e}", json_path.display()))?;
    let lines: WaifuLines = serde_json::from_str(&raw)
        .map_err(|e| format!("解析 {} 失败：{e}", json_path.display()))?;
    validate_waifu_lines(&lines)?;
    Ok(lines)
}

fn validate_waifu_lines(lines: &WaifuLines) -> AppResult {
    validate_voice_map("ping.success", &lines.ping.success)?;
    validate_voice_map("ping.fail", &lines.ping.fail)?;
    require_non_empty("cd.success", &lines.cd.success)?;
    require_non_empty("cd.fail", &lines.cd.fail)?;
    require_non_empty("ls.count", &lines.ls.count)?;
    require_non_empty("clear.success", &lines.clear.success)?;
    require_non_empty("echo.sayings", &lines.echo.sayings)?;
    require_non_empty("cat.success", &lines.cat.success)?;
    require_non_empty("cat.fail", &lines.cat.fail)?;
    require_non_empty("ps.success", &lines.ps.success)?;
    Ok(())
}

fn validate_voice_map(name: &str, map: &HashMap<String, Vec<VoiceLine>>) -> AppResult {
    if map.is_empty() {
        return Err(format!("{name} 文案为空"));
    }
    if let Some((period, _)) = map.iter().find(|(_, lines)| lines.is_empty()) {
        return Err(format!("{name}.{period} 文案为空"));
    }
    Ok(())
}

fn require_non_empty<T>(name: &str, values: &[T]) -> AppResult {
    if values.is_empty() {
        Err(format!("{name} 文案为空"))
    } else {
        Ok(())
    }
}
