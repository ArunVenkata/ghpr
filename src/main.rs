#![allow(unused)]

mod config_reader;
mod consts;
mod model_types;
mod url_builder;
mod utils;
use clap::ArgMatches;
use config_reader::{CommandArgs, ConfigReader};
use url_builder::GithubPRURLBuilder;
use webbrowser;

fn main() {
    let mut url_builder: GithubPRURLBuilder = GithubPRURLBuilder::new();

    let mut config_reader: ConfigReader = ConfigReader::new();

    let (matches, mut args): (ArgMatches, CommandArgs) = config_reader.read_cli_args();

    let url: &String = url_builder.build_url(&mut args);

    let open_in_browser: bool = matches.get_flag("open");

    if open_in_browser && webbrowser::open(&url).is_ok() {
        println!("Opened the URL in the default browser: {}", url);
    } else if !open_in_browser {
        println!("Click on this link to generate your PR: {}", url);
    }
}
