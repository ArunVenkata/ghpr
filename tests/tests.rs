use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_generate_pr_url_with_all_parameters() {
    let mut cmd = Command::cargo_bin("ghpr").unwrap();
    cmd.arg("--repo")
        .arg("owner/repo")
        .arg("--src")
        .arg("feature-branch")
        .arg("--dest")
        .arg("main")
        .arg("--title")
        .arg("Fix bug")
        .arg("--body")
        .arg("This PR fixes the bug.")
        .arg("--labels")
        .arg("bug,urgent")
        .arg("--milestone")
        .arg("v1.0")
        .arg("--assignees")
        .arg("user1,user2")
        .arg("--projects")
        .arg("project1,project2")
        .arg("--template")
        .arg("default");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Click on this link to generate your PR"));
}

#[test]
fn test_generate_pr_url_with_minimum_parameters() {
    let mut cmd = Command::cargo_bin("ghpr").unwrap();
    cmd.arg("--repo")
        .arg("owner/repo")
        .arg("--src")
        .arg("feature-branch")
        .arg("--dest")
        .arg("main");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Click on this link to generate your PR"));
}

#[test]
fn test_generate_pr_url_with_config_file() {
    // Create a temporary .ghprrc file
    let config_content = r#"
    [defaults]
    repo = "owner/repo"
    dest = "main"
    "#;
    fs::write(".ghprrc", config_content).unwrap();

    let mut cmd = Command::cargo_bin("ghpr").unwrap();
    cmd.arg("--src")
        .arg("feature-branch")
        .arg("--title")
        .arg("Add new feature");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Click on this link to generate your PR"));

    // Clean up the temporary .ghprrc file
    fs::remove_file(".ghprrc").unwrap();
}

