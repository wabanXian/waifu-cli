use std::process::Command;

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
