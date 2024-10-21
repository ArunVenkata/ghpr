use clap::{Arg, Command};
use config::{Config, File, FileFormat};
use git2::Repository;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::{collections::HashMap, fs};
use webbrowser;


const ABOUT: &str = r###"

Generates GitHub pull request URLs"###;

const USAGE: &str = r###"

Generate a pull request URL with a specified repository, source branch, and destination branch:
>> ghpr --repo owner/repo --src my-feature-branch --dest main

Generate a pull request URL with additional parameters like title and body:
>> ghpr --repo owner/repo --src main --dest feature-branch --title "Fix bug" --body "This PR fixes the bug."

Generate a pull request URL using values from a .ghprrc file (dest is picked up from the .ghprrc file):
>> ghpr --src feature-branch --title "Add new feature"
"###;

const AUTHOR: &str = "Arun V <arunvv.dev@gmail.com>";
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Defaults {
    values: HashMap<String, String>,
}

struct RcFile {
    defaults: Option<Defaults>,
}

fn main() {
    let matches = Command::new("PR URL Generator")
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .override_usage(USAGE)
        .arg(
            Arg::new("repo")
                .short('r')
                .long("repo")
                .value_name("REPO")
                .help("The GitHub repository in the format owner/repo")
                .required(false)
                .num_args(1),
        )
        .arg(
            Arg::new("src")
                .short('b')
                .long("src")
                .value_name("BASE")
                .help("The src branch")
                .required(false)
                .num_args(1),
        )
        .arg(
            Arg::new("dest")
                .short('d')
                .long("dest")
                .value_name("HEAD")
                .help("The dest branch")
                .required(false)
                .num_args(1),
        )
        .arg(
            Arg::new("title")
                .short('t')
                .long("title")
                .value_name("TITLE")
                .help("The title of the pull request")
                .num_args(1),
        )
        .arg(
            Arg::new("body")
                .short('p')
                .long("body")
                .value_name("BODY")
                .help("The body of the pull request")
                .num_args(1),
        )
        .arg(
            Arg::new("labels")
                .short('l')
                .long("labels")
                .value_name("LABELS")
                .help("Comma-separated labels for the pull request")
                .num_args(1),
        )
        .arg(
            Arg::new("milestone")
                .short('m')
                .long("milestone")
                .value_name("MILESTONE")
                .help("The milestone for the pull request")
                .num_args(1),
        )
        .arg(
            Arg::new("assignees")
                .short('a')
                .long("assignees")
                .value_name("ASSIGNEES")
                .help("Comma-separated assignees for the pull request")
                .num_args(1),
        )
        .arg(
            Arg::new("projects")
                .short('j')
                .long("projects")
                .value_name("PROJECTS")
                .help("Comma-separated projects for the pull request")
                .num_args(1),
        )
        .arg(
            Arg::new("template")
                .short('e')
                .long("template")
                .value_name("TEMPLATE")
                .help("The template for the pull request")
                .num_args(1),
        )
        .arg(
            Arg::new("open")
                .short('o')
                .long("open")
                .help("Open the generated URL in the default browser")
                .num_args(0)
        ) // Add this block to define the -o flag
        .get_matches();
    let config = read_config(".ghprrc");

    let repo: String = get_config_value(&matches, config.as_ref(), "repo");
    let dest: String = get_config_value(&matches, config.as_ref(), "dest");
    let mut src: String = get_config_value(&matches, config.as_ref(), "src");
    let title: String = get_config_value(&matches, config.as_ref(), "title");
    let body: String = get_config_value(&matches, config.as_ref(), "body");
    let labels: String = get_config_value(&matches, config.as_ref(), "labels");
    let milestone: String = get_config_value(&matches, config.as_ref(), "milestone");
    let assignees: String = get_config_value(&matches, config.as_ref(), "assignees");
    let projects: String = get_config_value(&matches, config.as_ref(), "projects");
    let template: String = get_config_value(&matches, config.as_ref(), "template");
    let open_in_browser: bool = matches.get_flag("open");
    
    
    if src.is_empty() {
        src = get_current_branch_name().unwrap_or_else(|| String::new());
    }

    let mut url = format!(
        "https://github.com/{}/compare/{}...{}?quick_pull=1",
        repo, dest, src
    );

    if !title.is_empty() {
        let encoded_title = utf8_percent_encode(&title, NON_ALPHANUMERIC).to_string();
        url.push_str(&format!("&title={}", encoded_title));
    }
    if !body.is_empty() {
        let encoded_body = utf8_percent_encode(&body, NON_ALPHANUMERIC).to_string();
        url.push_str(&format!("&body={}", encoded_body));
    }
    if !labels.is_empty() {
        url.push_str(&format!("&labels={}", labels));
    }
    if !milestone.is_empty() {
        url.push_str(&format!("&milestone={}", milestone));
    }
    if !assignees.is_empty() {
        url.push_str(&format!("&assignees={}", assignees));
    }
    if !projects.is_empty() {
        url.push_str(&format!("&projects={}", projects));
    }
    if !template.is_empty() {
        url.push_str(&format!("&template={}", template));
    }

    if open_in_browser && webbrowser::open(&url).is_ok() {
        println!("Opened the URL in the default browser: {}", url);
    } else if !open_in_browser{
        println!("Click on this link to generate your PR: {}", url);
    }
}

fn get_config_value<'a>(matches: &clap::ArgMatches, config: Option<&RcFile>, key: &str) -> String {
    // Check matches first
    let value_from_matches: Option<&str> =
        matches.get_one::<String>(key).map(|s: &String| s.as_str());

    // If no value found, check the config
    let value_from_config: Option<&str> = config.and_then(|cfg: &RcFile| {
        cfg.defaults
            .as_ref()
            .and_then(|d: &Defaults| d.values.get(key))
            .map(|s: &String| s.as_str())
    });

    // Combine both options
    value_from_matches
        .or(value_from_config)
        .map(|s: &str| s.to_string())
        .unwrap_or_else(|| {
            String::new() // or provide a default string if needed
        })
}

fn read_config(file_path: &str) -> Option<RcFile> {
    let builder: config::ConfigBuilder<config::builder::DefaultState> =
        Config::builder().add_source(File::new(file_path, FileFormat::Ini));

    match builder.build() {
        Ok(config) => {
            let defaults: Option<HashMap<String, String>> = config.get("defaults").ok();
            let defaults: Option<Defaults> =
                defaults.map(|values: HashMap<String, String>| Defaults { values });

            Some(RcFile { defaults })
        }
        Err(_e) => {
            // eprintln!("Error reading config: {}", e);
            None
        }
    }
}

fn get_current_branch_name() -> Option<String> {
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
