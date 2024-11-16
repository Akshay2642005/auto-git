use names::Generator;
use std::process::{exit, Command};

//Update the git repository
pub fn git_update_repo() {
    let mut add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .spawn()
        .expect("failed to execute process");
    let status = add_command.wait().expect("failed to wait on child");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
    let mut commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .spawn()
        .expect("failed to execute process");
    let status = commit_command.wait().expect("failed to wait on child");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
    let mut push_command = Command::new("git")
        .arg("push")
        .spawn()
        .expect("failed to execute process");
    let status = push_command.wait().expect("failed to wait on child");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}

// Generate a random name for the commit message
fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}
