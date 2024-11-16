use std::process::Command;

pub fn git_status() {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout).unwrap();

    if output.is_empty() {
        println!("🌳 Clean, No Changes");
    } else {
        println!("🌳 Changes Detected:{}", output);
    }
}
