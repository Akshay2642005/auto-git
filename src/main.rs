mod init;
mod status;
mod update;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use init::git_init;
use status::git_status;
use std::process::exit;
use update::git_update_repo;

//main function
fn main() {
    let options = vec![
        " Initialize a new git repository",
        " Update the git repository",
        " Check the status of the git repository",
        " Exit",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    match options[selection] {
        " Initialize a new git repository" => git_init(),
        " Update the git repository" => git_update_repo(),
        " Check the status of the git repository" => git_status(),
        " Exit" => exit(0),
        _ => println!("Invalid option selected."),
    }
}
