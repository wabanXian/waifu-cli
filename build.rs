use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/waifu.json");
    println!("cargo:rerun-if-changed=speak.py");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not set"));
    let executable_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("unexpected Cargo OUT_DIR layout");

    copy_resource("src/waifu.json", executable_dir);
    copy_resource("speak.py", executable_dir);
}

fn copy_resource(source: &str, destination_dir: &Path) {
    let file_name = Path::new(source)
        .file_name()
        .expect("resource path has no file name");
    let destination = destination_dir.join(file_name);

    fs::copy(source, &destination).unwrap_or_else(|error| {
        panic!(
            "failed to copy {source} to {}: {error}",
            destination.display()
        )
    });
}
