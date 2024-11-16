use std::fs::File;
use std::io::{self, Write};
use std::process::{exit, Command};
pub fn git_init() {
    // Step 1: Create the README.md file and write initial content to it
    let mut readme = File::create("README.md").expect("Failed to create README.md");
    write!(
        readme,
        "# auto_git\n\nThis is an automatically generated README.md file."
    )
    .expect("Failed to write to README.md");

    // Initialize git repository
    let mut init_command = Command::new("git")
        .arg("init")
        .spawn()
        .expect("failed to execute process");

    let status = init_command.wait().expect("failed to wait on child");
    if !status.success() {
        eprintln!(
            "Git init failed with exit code: {}",
            status.code().unwrap_or(1)
        );
        exit(status.code().unwrap_or(1));
    }

    // Initial add files
    let mut add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .spawn()
        .expect("failed to execute process");

    let status = add_command.wait().expect("failed to wait on child");
    if !status.success() {
        eprintln!(
            "Git add failed with exit code: {}",
            status.code().unwrap_or(1)
        );
        exit(status.code().unwrap_or(1));
    }

    // Initial commit
    let mut commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("initial commit")
        .spawn()
        .expect("failed to execute process");

    let status = commit_command.wait().expect("failed to wait on child");
    if !status.success() {
        eprintln!(
            "Git commit failed with exit code: {}",
            status.code().unwrap_or(1)
        );
        exit(status.code().unwrap_or(1));
    }

    // Create and switch to the 'main' branch
    let mut new_branch = String::new();
    io::stdin()
        .read_line(&mut new_branch)
        .expect("failed to read line");
    print!("  Specify the branch:");
    io::stdout().flush().expect("failed to flush stdout");
    let new_branch = new_branch.trim();

    let mut branch_command = Command::new("git")
        .arg("branch")
        .arg("-M")
        .arg(new_branch)
        .spawn()
        .expect("failed to execute process");

    let status = branch_command.wait().expect("failed to wait on child");
    if !status.success() {
        eprintln!(
            "Git branch -M main failed with exit code: {}",
            status.code().unwrap_or(1)
        );
        exit(status.code().unwrap_or(1));
    }

    // Ask for remote repository URL
    println!("=>Initialized git repository in the current directory.");
    print!("  Enter the remote repository URL: ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut remote_url = String::new();
    io::stdin()
        .read_line(&mut remote_url)
        .expect("failed to read line");

    // Trim the input URL to remove extra spaces or newlines
    let remote_url = remote_url.trim();

    if remote_url.is_empty() {
        eprintln!("Remote repository URL is empty. Exiting.");
        exit(1);
    }

    // Add the remote repository
    let mut remote_command = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(remote_url)
        .spawn()
        .expect("failed to execute process");

    let status = remote_command.wait().expect("failed to wait on child");
    if !status.success() {
        eprintln!(
            "Git remote add failed with exit code: {}",
            status.code().unwrap_or(1)
        );
        exit(status.code().unwrap_or(1));
    }

    println!("=>Remote repository added successfully.");

    // Push changes to the remote repository
    println!("=>Pushing changes to remote repository...");
    let mut push_command = Command::new("git")
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg(new_branch)
        .spawn()
        .expect("failed to execute process");

    let status = push_command.wait().expect("failed to wait on child");
    if !status.success() {
        eprintln!(
            "Git push failed with exit code: {}",
            status.code().unwrap_or(1)
        );
        exit(status.code().unwrap_or(1));
    }

    println!("=>Pushed changes successfully to the remote repository.");
}
