//! Audio output is optional. VOICEVOX uses the native C API on Windows,
//! and the old CeVIO/Python bridge remains available as an explicit fallback.

#[cfg(windows)]
mod voicevox;

#[cfg(windows)]
pub(crate) fn inspect_voicevox() -> Vec<(String, String, bool)> {
    voicevox::inspect()
}

#[cfg(windows)]
pub(crate) fn probe_voicevox_gpu() -> Result<bool, String> {
    voicevox::probe_gpu_mode()
}

#[cfg(windows)]
pub fn speak(japanese: &str) {
    let backend = std::env::var("WAIFU_TTS_BACKEND")
        .unwrap_or_else(|_| "voicevox".into())
        .to_ascii_lowercase();

    let result = match backend.as_str() {
        "voicevox" => voicevox::speak(japanese),
        "cevio" => speak_cevio(japanese),
        other => Err(format!("未知语音后端 {other:?}，请使用 voicevox 或 cevio")),
    };
    // Synthesis is an optional side effect: no speech backend may break CLI commands.
    if let Err(err) = result {
        eprintln!("😿 语音播放失败：{err}");
    }
}

#[cfg(windows)]
fn speak_cevio(japanese: &str) -> Result<(), String> {
    use std::process::Command;

    let exe = std::env::current_exe()
        .map_err(|e| format!("无法定位 CeVIO 脚本：{e}"))?;
    let script = exe.with_file_name("speak.py");
    if !script.is_file() {
        return Err(format!("找不到 CeVIO 兼容脚本：{}", script.display()));
    }
    let status = Command::new("python")
        .arg(script)
        .arg(japanese)
        .env("WAIFU_TTS_BACKEND", "cevio")
        .status()
        .map_err(|e| format!("无法启动 CeVIO Python 脚本：{e}"))?;
    if !status.success() {
        return Err(format!("CeVIO 播放脚本异常退出：{status}"));
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn speak(_japanese: &str) {
    eprintln!("😿 当前仅支持 Windows x64 的 VOICEVOX CORE C API");
}
