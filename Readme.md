# 🦀 Github PR URL Generator

Welcome to the **GitHub PR URL Generator**! This command-line tool simplifies the creation of pull request for your GitHub repositories.

To create a PR quickly, run the command and click the generated link.



[](https://github.com/user-attachments/assets/50ccbc19-e587-4820-9e7a-d02704d99add)



## 🚀 Examples of Usage

Here are some examples of how to use the `ghpr` command:

1. Generate a pull request URL with a specified repository, source branch, and destination branch: 
```bash
ghpr --repo owner/repo --src my-feature-branch --dest main
```

2. Generate a pull request URL with additional parameters like title and body:
```bash
ghpr --repo owner/repo --src main --dest feature-branch --title "Fix bug" --body "This PR fixes the bug."
```


3. Generate a pull request URL using values from a `.ghprrc` file (destination is picked up from the file):
```bash
ghpr --src feature-branch --title "Add new feature"
```

.ghprrc file:
```ini
[defaults]
dest="master"
repo="ArunVenkata/ghpr"
```
Generated URL: https://github.com/ArunVenkata/ghpr/compare/feature-branch...master?quick_pull=1&title=Add%20new%20feature

## 📦 Installation Instructions

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

2. Run the following command 
```bash
cargo install ghpr
```

## 🖥️ Install from Source

1. Clone the repository:
```bash
git clone https://github.com/ArunVenkata/ghpr.git
cd ghpr
```
2. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

3. Build the project:
```bash
cargo build --release
```

4. Run the tool:
```bash
./target/release/ghpr --help
```


## ⚙️ About the .ghprrc File

The .ghprrc file allows you to set default values for various parameters used in the ghpr command. This file should be located in your project directory and formatted as follows:

```ini
[defaults]
repo = "owner/repo"
src = "feature-branch"
dest = "main"
title = "Your PR Title"
body = "Description of the pull request."
labels = "bug,enhancement"
milestone = "1.0"
assignees = "username1,username2"
projects = "project1,project2"
template = "template_name"
```
> _Note: If `src` is not specified, it automatically refers to your current git branch (provided your current directory is a git repository)_


Run `ghpr --help` for more information on the supported parameters.


## 🤝 Contributing Guidelines

If you would like to contribute to the project, please follow these guidelines:
1. Fork the repository and create your feature branch:

```bash
git checkout -b feature/new-feature
```

2. Commit your changes:

```bash
git commit -m "Add some feature"
```

3. Push to the branch:
```bash
git push origin feature/new-feature
```

4. Open a pull request and describe your changes.



## 🎉 License

This project is licensed under the MIT License. See the [LICENSE](https://www.github.com/ArunVenkata/ghpr/blob/master/LICENSE) file for details.

