use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct TestDir(PathBuf);

impl TestDir {
    fn new() -> Self {
        let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("waifu-cli-test-{}-{id}", std::process::id()));
        fs::create_dir_all(&path).expect("failed to create test directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn write(&self, name: &str, contents: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, contents).expect("failed to write test file");
        path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run_waifu(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_waifu"))
        .args(args)
        .env("NO_COLOR", "1")
        .output()
        .expect("failed to run waifu")
}

#[test]
fn echo_accepts_separate_arguments() {
    let output = run_waifu(&["echo", "hello", "world"]);

    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}

#[test]
fn missing_cat_target_returns_failure() {
    let dir = TestDir::new();
    let missing = dir.path().join("missing.txt");
    let output = run_waifu(&["cat", missing.to_str().unwrap()]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("cat 执行失败"));
}

#[test]
fn small_utf8_cat_file_succeeds() {
    let dir = TestDir::new();
    let path = dir.write("small.txt", b"small utf-8 text");
    let output = run_waifu(&["cat", path.to_str().unwrap()]);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("small utf-8 text"));
}

#[test]
fn multiline_utf8_cat_file_preserves_each_line() {
    let dir = TestDir::new();
    let path = dir.write("multiline.txt", "第一行\nsecond line\n第三行".as_bytes());
    let output = run_waifu(&["cat", path.to_str().unwrap()]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(stdout.contains("第一行"));
    assert!(stdout.contains("second line"));
    assert!(stdout.contains("第三行"));
}

#[test]
fn non_utf8_cat_file_returns_failure() {
    let dir = TestDir::new();
    let path = dir.write("invalid.bin", &[0xff, 0xfe, 0xff]);
    let output = run_waifu(&["cat", path.to_str().unwrap()]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("UTF-8"));
}

#[test]
fn cat_directory_returns_failure() {
    let dir = TestDir::new();
    let output = run_waifu(&["cat", dir.path().to_str().unwrap()]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("不是文件"));
}

#[test]
fn oversized_cat_file_returns_failure() {
    let dir = TestDir::new();
    let path = dir.path().join("oversized.txt");
    let file = File::create(&path).expect("failed to create oversized file");
    file.set_len(16 * 1024 * 1024 + 1)
        .expect("failed to size oversized file");
    let output = run_waifu(&["cat", path.to_str().unwrap()]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("文件过大"));
}

#[test]
fn overlong_cat_line_returns_failure() {
    let dir = TestDir::new();
    let path = dir.path().join("long-line.txt");
    let mut file = File::create(&path).expect("failed to create long-line file");
    file.write_all(&vec![b'a'; 1024 * 1024 + 1])
        .expect("failed to write long line");
    let output = run_waifu(&["cat", path.to_str().unwrap()]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("单行内容过长"));
}

#[test]
fn shell_bridge_commands_keep_machine_stdout() {
    let cd = run_waifu(&["cd", "."]);
    assert!(cd.status.success());
    assert_eq!(String::from_utf8_lossy(&cd.stdout).trim(), ".");

    let ls = run_waifu(&["ls", "."]);
    assert!(ls.status.success());
    assert_eq!(String::from_utf8_lossy(&ls.stdout).lines().count(), 1);

    let ps = run_waifu(&["ps", "--top", "1", "--sort", "cpu"]);
    assert!(ps.status.success());
    let result: serde_json::Value =
        serde_json::from_slice(&ps.stdout).expect("ps stdout should be JSON");
    assert_eq!(result["top"], 1);
    assert_eq!(result["sort"], "cpu");
}
