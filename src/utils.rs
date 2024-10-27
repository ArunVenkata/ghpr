use git2::Repository;
use std::fs;

pub fn get_current_branch_name() -> Option<String> {
    match Repository::discover(".") {
        Ok(repo) => {
            // try to get the head reference
            match repo.head() {
                Ok(head) => {
                    if let Some(branch) = head.shorthand() {
                        return Some(branch.to_string());
                    } else {
                    }
                }
                Err(_e) => {}
            }

            let head_path = repo.path().join("HEAD");

            if let Ok(head_content) = fs::read_to_string(head_path) {
                if head_content.starts_with("ref: refs/heads/") {
                    let branch_name = head_content
                        .trim_start_matches("ref: refs/heads/")
                        .trim()
                        .to_string();
                    return Some(branch_name);
                }
            }
            None
        }
        Err(_e) => None,
    }
}
