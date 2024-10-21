use crate::model_types::{RcFile, Defaults};

use config::{Config, File, FileFormat};
use git2::Repository;
use std::{collections::HashMap, fs};

pub fn get_config_value<'a>(matches: &clap::ArgMatches, config: Option<&RcFile>, key: &str) -> String {
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

pub fn read_config(file_path: &str) -> Option<RcFile> {
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
