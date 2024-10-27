use clap::{Arg, ArgMatches, Command};
use config::{Config, File, FileFormat};

use crate::{
    consts::{ABOUT, AUTHOR, COMMAND_NAME, CONFIG_FILE_PATH, USAGE, VERSION},
    model_types::{Defaults, RcFile},
};
use std::collections::HashMap;

pub struct CommandArgs {
    pub repo: String,
    pub dest: String,
    pub src: String,
    pub title: String,
    pub body: String,
    pub labels: String,
    pub milestone: String,
    pub assignees: String,
    pub projects: String,
    pub template: String,
}

pub struct ConfigReader {
    config_file_data: HashMap<String, String>,
}

impl ConfigReader {
    pub fn new() -> Self {
        let mut instance = Self {
            config_file_data: HashMap::new(),
        };

        instance.read_config_file();

        return instance;
    }

    pub fn read_cli_args(&mut self) -> (ArgMatches, CommandArgs) {
        let matches: ArgMatches = Command::new(COMMAND_NAME)
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
                    .num_args(0),
            )
            .get_matches();

        let args = CommandArgs {
            repo: self.get_config_value(&matches, "repo"),
            dest: self.get_config_value(&matches, "dest"),
            src: self.get_config_value(&matches, "src"),
            title: self.get_config_value(&matches, "title"),
            body: self.get_config_value(&matches, "body"),
            labels: self.get_config_value(&matches, "labels"),
            milestone: self.get_config_value(&matches, "milestone"),
            assignees: self.get_config_value(&matches, "assignees"),
            projects: self.get_config_value(&matches, "projects"),
            template: self.get_config_value(&matches, "template"),
        };

        return (matches, args);
    }

    fn read_config_file(&mut self) -> Option<RcFile> {
        let builder: config::ConfigBuilder<config::builder::DefaultState> =
            Config::builder().add_source(File::new(CONFIG_FILE_PATH, FileFormat::Ini));

        match builder.build() {
            Ok(config) => {
                let defaults: Option<HashMap<String, String>> = config.get("defaults").ok();
                let defaults: Option<Defaults> = defaults.map(|values: HashMap<String, String>| {
                    self.config_file_data = values.clone();

                    Defaults { values }
                });
                Some(RcFile { defaults })
            }
            Err(_e) => {
                // eprintln!("Error reading config: {}", e);
                None
            }
        }
    }

    fn get_file_config(&mut self) -> &HashMap<String, String> {
        if self.config_file_data.is_empty() {
            self.read_config_file();
        }

        return &self.config_file_data;
    }

    pub fn get_config_value(&mut self, matches: &clap::ArgMatches, key: &str) -> String {
        // Check matches first
        let value_from_matches: Option<&str> =
            matches.get_one::<String>(key).map(|s: &String| s.as_str());

        let config = self.get_file_config();

        let value_from_config: Option<&str> = config.get(key).map(|s: &String| s.as_str());

        // Combine both options
        value_from_matches
            .or(value_from_config)
            .map(|s: &str| s.to_string())
            .unwrap_or_else(|| {
                String::new() // or provide a default string if needed
            })
    }
}
