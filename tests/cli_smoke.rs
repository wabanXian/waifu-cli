use std::process::{Command, Output};

fn run_waifu(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_waifu"))
        .args(args)
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
    let output = run_waifu(&["cat", "__waifu_missing_test_file__"]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("cat 执行失败"));
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
