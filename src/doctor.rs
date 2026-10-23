//! Fast, read-only environment checks for waifu-cli.
//! Voice model inference is opt-in; a regular doctor run never starts TTS.

use clap::Args;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Args)]
pub struct DoctorArgs {
    /// Run one uncached VOICEVOX synthesis to verify the real CPU/GPU mode.
    /// Does not play sound or write to the WAV cache.
    #[arg(long)]
    pub voice_test: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Ok,
    Warning,
    Error,
    Info,
}

struct Check {
    status: Status,
    name: String,
    detail: String,
}

fn check(status: Status, name: impl Into<String>, detail: impl Into<String>) -> Check {
    Check { status, name: name.into(), detail: detail.into() }
}

fn executable_on_path(name: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|path| {
        env::split_paths(&path)
            .map(|dir| dir.join(name))
            .find(|candidate| candidate.is_file())
    })
}

fn find_script(exe: &Path, script: &str) -> Option<PathBuf> {
    exe.parent()?
        .ancestors()
        .take(5)
        .map(|dir| dir.join(script))
        .find(|path| path.is_file())
}

fn show(checks: &[Check]) {
    for item in checks {
        let icon = match item.status {
            Status::Ok => "✓",
            Status::Warning => "!",
            Status::Error => "×",
            Status::Info => "i",
        };
        println!("  {icon} {:<18} {}", item.name, item.detail);
    }
}

pub fn run(args: DoctorArgs) -> crate::utils::AppResult {
    println!("🐾 waifu doctor · 环境诊断");
    println!("（默认只检查文件与配置；不会启动语音合成）\n");

    let mut checks = Vec::new();
    let exe = env::current_exe();
    match &exe {
        Ok(path) => checks.push(check(Status::Ok, "waifu 可执行文件", path.display().to_string())),
        Err(err) => checks.push(check(Status::Error, "waifu 可执行文件", err.to_string())),
    }

    let config_error = match crate::config::load_waifu_lines() {
        Ok(_) => {
            checks.push(check(Status::Ok, "waifu.json", "已读取并通过结构检查"));
            false
        }
        Err(err) => {
            checks.push(check(Status::Error, "waifu.json", err));
            true
        }
    };

    if let Ok(exe) = &exe {
        if let Some(script) = find_script(exe, "waifu-shell.ps1") {
            checks.push(check(Status::Ok, "Shell 包装脚本", script.display().to_string()));
        } else {
            checks.push(check(Status::Warning, "Shell 包装脚本",
                "未在可执行文件上级目录中找到 waifu-shell.ps1（可选）"));
        }
    }
    let ps = executable_on_path("pwsh.exe")
        .or_else(|| executable_on_path("powershell.exe"));
    if let Some(path) = ps {
        checks.push(check(Status::Ok, "PowerShell", format!("PATH 中找到 {}", path.display())));
    } else {
        checks.push(check(Status::Warning, "PowerShell",
            "PATH 中未发现 pwsh.exe / powershell.exe；不影响 waifu.exe 独立运行"));
    }
    checks.push(check(Status::Info, "PowerShell Profile",
        "子进程无法确定当前会话是否已加载 waifu-shell.ps1；请在 PowerShell 内检查 Get-Command ls"));

    let backend = env::var("WAIFU_TTS_BACKEND")
        .unwrap_or_else(|_| "voicevox".to_string())
        .to_ascii_lowercase();
    checks.push(check(Status::Info, "语音后端", backend.clone()));

    let mut voice_error = false;

    #[cfg(windows)]
    match backend.as_str() {
        "voicevox" => {
            for (name, detail, success) in crate::voice::inspect_voicevox() {
                checks.push(check(
                    if success { Status::Ok } else { Status::Warning },
                    name, detail,
                ));
            }
            if args.voice_test {
                println!("  … 执行未缓存的 VOICEVOX 语音合成测试（不会播放声音）");
                match crate::voice::probe_voicevox_gpu() {
                    Ok(true) => checks.push(check(Status::Ok, "语音合成测试",
                        "合成成功 · 官方 C API 确认使用 GPU 模式")),
                    Ok(false) => checks.push(check(Status::Warning, "语音合成测试",
                        "合成成功 · 官方 C API 确认使用 CPU 模式（GPU 未启用或不受支持）")),
                    Err(err) => {
                        voice_error = true;
                        checks.push(check(Status::Error, "语音合成测试", err));
                    }
                }
            }
        }
        "cevio" => {
            let script = exe.as_ref().ok()
                .and_then(|path| path.parent())
                .map(|dir| dir.join("speak.py"));
            if script.as_ref().is_some_and(|p| p.is_file()) {
                checks.push(check(Status::Ok, "CeVIO 兼容脚本", "exe 同级存在 speak.py"));
            } else {
                checks.push(check(Status::Warning, "CeVIO 兼容脚本", "exe 同级缺少 speak.py"));
            }
            if executable_on_path("python.exe").is_some() {
                checks.push(check(Status::Ok, "Python", "PATH 中找到 python.exe"));
            } else {
                checks.push(check(Status::Warning, "Python", "PATH 中找不到 python.exe"));
            }
            checks.push(check(Status::Info, "CeVIO Host",
                "未探测实际运行状态；需要调用时由 Python 兼容脚本连接"));
            if args.voice_test {
                voice_error = true;
                checks.push(check(Status::Error, "--voice-test",
                    "此选项仅测试 VOICEVOX C API；请切换 WAIFU_TTS_BACKEND=voicevox"));
            }
        }
        other => {
            voice_error = args.voice_test;
            checks.push(check(Status::Warning, "语音后端设置",
                format!("未知后端 {other:?}，可使用 voicevox 或 cevio")));
        }
    }

    #[cfg(not(windows))]
    {
        checks.push(check(Status::Warning, "VOICEVOX",
            "原生 C API 接口当前仅支持 Windows x64"));
        if args.voice_test {
            voice_error = true;
            checks.push(check(Status::Error, "--voice-test",
                "当前平台不支持 VOICEVOX C API 语音测试"));
        }
    }

    println!();
    show(&checks);
    let warnings = checks.iter().filter(|c| c.status == Status::Warning).count();
    let errors = checks.iter().filter(|c| c.status == Status::Error).count();
    println!("\n诊断完成：{} 项通过，{warnings} 项提醒，{errors} 项错误。",
        checks.iter().filter(|c| c.status == Status::Ok).count());

    if config_error || exe.is_err() || voice_error {
        Err("doctor 检查存在需要处理的错误".into())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_script_in_project_ancestor() {
        let temp = std::env::temp_dir().join(format!("waifu-doctor-{}", std::process::id()));
        let path = temp.join("target").join("release");
        std::fs::create_dir_all(&path).unwrap();
        let script = temp.join("waifu-shell.ps1");
        std::fs::write(&script, "# shell").unwrap();
        assert_eq!(find_script(&path.join("waifu.exe"), "waifu-shell.ps1"), Some(script.clone()));
        let _ = std::fs::remove_dir_all(temp);
    }
}
