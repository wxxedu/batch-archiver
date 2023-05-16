use std::{fmt::Display, process::Command};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Repo {
    name_with_owner: String,
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name_with_owner)
    }
}

fn get_repos() -> Vec<Repo> {
    let res = Command::new("gh")
        .arg("repo")
        .arg("list")
        .arg("--json")
        .arg("nameWithOwner")
        .arg("--limit")
        .arg("100")
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8(res.stdout).unwrap();
    serde_json::from_str(&stdout).unwrap()
}

fn archive_repo(repo: &Repo) {
    Command::new("gh")
        .arg("repo")
        .arg("archive")
        .arg("--yes")
        .arg(&repo.name_with_owner)
        .output()
        .expect("failed to execute process");
}

const OPTIONS: [&str; 2] = ["Skip", "Archive"];

fn ask_lib(repo: &Repo) {
    let selected = dialoguer::Select::new()
        .with_prompt(format!("What to do with {}", repo))
        .items(&OPTIONS)
        .default(0)
        .interact()
        .unwrap();
    match selected {
        0 => {
            println!("Skipping {}", repo);
        }
        1 => {
            println!("Archiving {}", repo);
            archive_repo(repo);
        }
        _ => panic!("Invalid option"),
    }
}

fn main() {
    let repos = get_repos();
    for repo in repos {
        ask_lib(&repo);
    }
}
