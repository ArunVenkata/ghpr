mod utils;
mod model_types;
mod consts;
use consts::{ABOUT, AUTHOR, USAGE, VERSION};
use clap::{Arg, Command};
use utils::{get_config_value, get_current_branch_name, read_config};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use webbrowser;

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
        )
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
