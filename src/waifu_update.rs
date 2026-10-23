//! Start the signed-checksum update script in a separate PowerShell process.
//! Windows cannot replace a running waifu.exe; the updater waits for our exit.

use crate::utils::AppResult;

#[cfg(windows)]
pub fn run() -> AppResult {
    use std::env;
    use std::process::{Command, Stdio};

    let exe = env::current_exe().map_err(|e| format!("无法定位 waifu.exe：{e}"))?;
    let script = exe.with_file_name("update.ps1");
    if !script.is_file() {
        return Err(format!(
            "找不到安装目录中的 update.ps1：{}。请先运行 install.ps1，或手动更新 GitHub Release。",
            script.display()
        ));
    }
    let updater = Command::new("powershell.exe")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File"])
        .arg(&script)
        .arg("-WaitForProcessId")
        .arg(std::process::id().to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| format!("启动更新脚本失败：{e}"))?;
    println!("🐾 已启动独立更新程序 (PID {})，它将在 waifu.exe 退出后执行。", updater.id());
    Ok(())
}

#[cfg(not(windows))]
pub fn run() -> AppResult {
    Err("当前仅支持 Windows PowerShell 自动更新".into())
}
