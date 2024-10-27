use crate::{config_reader::CommandArgs, utils::get_current_branch_name};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
pub struct GithubPRURLBuilder {
    pub url: String,
}

impl GithubPRURLBuilder {
    pub(crate) fn new() -> Self {
        GithubPRURLBuilder {
            url: String::from("https://www.github.com/"),
        }
    }

    pub(crate) fn build_url(&mut self, args: &mut CommandArgs) -> &String {
        if args.src.is_empty() {
            args.src = get_current_branch_name().unwrap_or_else(|| String::new());
        }

        self.url.push_str(&format!(
            "{}/compare/{}...{}?quick_pull=1",
            args.repo, args.dest, args.src
        ));

        if !args.title.is_empty() {
            let encoded_title = utf8_percent_encode(&args.title, NON_ALPHANUMERIC).to_string();
            self.url.push_str(&format!("&title={}", encoded_title));
        }
        if !args.body.is_empty() {
            let encoded_body = utf8_percent_encode(&args.body, NON_ALPHANUMERIC).to_string();
            self.url.push_str(&format!("&body={}", encoded_body));
        }
        if !args.labels.is_empty() {
            self.url.push_str(&format!("&labels={}", args.labels));
        }
        if !args.milestone.is_empty() {
            self.url.push_str(&format!("&milestone={}", args.milestone));
        }
        if !args.assignees.is_empty() {
            self.url.push_str(&format!("&assignees={}", args.assignees));
        }
        if !args.projects.is_empty() {
            self.url.push_str(&format!("&projects={}", args.projects));
        }
        if !args.template.is_empty() {
            self.url.push_str(&format!("&template={}", args.template));
        }

        return &self.url;
    }
}
