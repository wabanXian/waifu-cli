pub use crate::config::{load_waifu_lines, VoiceLine, WaifuLines};
pub use crate::style::{rainbow, RAINBOW_STOPS};
pub use crate::voice::speak;
use chrono::{Local, Timelike};
use rand::seq::SliceRandom;

pub type AppError = String;
pub type AppResult<T = ()> = Result<T, AppError>;

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
