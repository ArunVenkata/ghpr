pub const ABOUT: &str = r###"

Generates GitHub pull request URLs"###;

pub const USAGE: &str = r###"

Generate a pull request URL with a specified repository, source branch, and destination branch:
>> ghpr --repo owner/repo --src my-feature-branch --dest main

Generate a pull request URL with additional parameters like title and body:
>> ghpr --repo owner/repo --src main --dest feature-branch --title "Fix bug" --body "This PR fixes the bug."

Generate a pull request URL using values from a .ghprrc file (dest is picked up from the .ghprrc file):
>> ghpr --src feature-branch --title "Add new feature"
"###;

pub const AUTHOR: &str = "Arun V <arunvv.dev@gmail.com>";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CONFIG_FILE_PATH: &str = ".ghprrc";
pub const COMMAND_NAME: &str = "PR URL Generator";
