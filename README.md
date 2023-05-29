<p align="center">
    <h1 align="center">
        ✍️ writeme
    </h1>
    <p align="center"> Cross-platform Auto-generate README.md for dev projects</p>
</p>

**writeme** is a project designed to simplify the process of creating a comprehensive **README.md** file for software development projects.
The primary purpose of writeme is to automatically extract relevant information from your project and generate for you a well-formatted README.md file that includes important details such as project name, description, repository name, usage and configuration steps, funding details, and collaborators.

writeme primarily extracts information from the project's configuration file. The specific type of configuration file depends on the project, but popular examples include **Cargo.toml**, **package.json**, **composer.json**, and others. To get the best from writeme, take a look at your configuration files

# Table of Contents
- [Table of Contents](#table-of-contents)
- [⚙️ Configuration ](#️-configuration-)
- [⬇️ Installation ](#️-installation-)
    - [Cargo](#cargo)
    - [Homebrew](#homebrew)
- [🎈 Usage ](#-usage-)

- [🔭 Contributing ](#-contributing-)
  - [🐛 Issues ](#-issues-)
  - [🤝 Pull Requests ](#-pull-requests-)
  - [🎨 Style Guide](#-style-guide)
- [📄 License ](#-license-)
- [✍️ Authors ](#️-authors-)

## ⚙️ Configuration <a name="configuration"></a>
If you choose to install the application using cargo, make sure you have the rust toolchain installed on your machine. You can find the installation instructions [here](https://www.rust-lang.org/tools/install).


## ⬇️ Installation <a name="installation"></a>
### Cargo
```bash
cargo install writeme
```

### Homebrew
```bash
brew tap BabelDev0/writeme && brew install writeme
```

## 🎈 Usage <a name="usage"></a>
As simple as writing:

```bash
writeme 
```
or to select a different path
```bash
writeme --path 'path/to/your/project'
```

Use `writeme --help` to see all the available options.

# 🔭 Contributing <a name = "contributing"></a>

🎉 Thank you for being interested in contributing to the project! 🎉 

Feel welcome and read the following sections in order to know how to ask questions and how to work on something.

Please make sure you are welcoming and friendly in all of our spaces. 👍

## 🐛 Issues <a name = "issues"></a>

The best way to contribute to our projects is by opening a new issue or tackling one of the issues that are already open.

## 🤝 Pull Requests <a name = "pull-requests"></a>

Pull requests are great if you want to add a feature or fix a bug. Here's a quick guide:
1. Fork the repo.
documentation changes require no new tests.
1. Make sure to check out the [Style Guide](#style-guide) and ensure that your code complies with the rules.
2. Commit your changes.
3. Push to your fork and submit a pull request. Please provide us with some explanation of why you made the changes you made.

## 🎨 Style Guide<a name="style-guide"></a>

### Commits rules<a name="commits-rules"></a>

For commits it is recommended to use [Conventional Commits](https://www.conventionalcommits.org).

#### Type<a name="commit-type"></a>

The type must be one of the following:

-   feat: A new feature
-   fix: A bug fix
-   docs: Documentation only changes
-   style: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
-   refactor: A code change that neither fixes a bug nor adds a feature (improvements of the code structure)
-   perf: A code change that improves the performance
-   test: Adding missing or correcting existing tests
-   build: Changes that affect the build system or external dependencies (example scopes: gulp, npm)
-   ci: Changes to CI configuration files and scripts (example scopes: travis, circle)
-   chore: Other changes that don't modify src or test files
-   revert: Reverts a previous commit

#### Scope<a name="commit-scope"></a>

The scope should be the name of the piece of the project affected.

#### Subject<a name="commit-subject"></a>

The subject contains a succinct description of the change:

-   Use the imperative, present tense: "change" not "changed" nor "changes"
-   Don't capitalize the first letter
-   No dot (.) at the end

# 📄 License <a name = "license"></a>
GPLv3

# ✍️ Authors <a name = "authors"></a>
- [BabelDev0](https://github.com/BabelDev0)
- [Luca Corsetti](https://github.com/ilcors-dev)

<p align="center">
<br>
<a href="https://github.com/BabelDev0" target='_blank'>
<img height='32' style='border:0px;height:32px;border-radius:.5rem' src='https://img.shields.io/badge/GitHub-100000?style&#x3D;for-the-badge&amp;logo&#x3D;github&amp;logoColor&#x3D;white' border='0' alt='Buy Me a Coffee' />
</a>
 <a href="https://github.com/ilcors-dev" target='_blank'>
<img height='32' style='border:0px;height:32px;border-radius:.5rem' src='https://img.shields.io/badge/GitHub-100000?style&#x3D;for-the-badge&amp;logo&#x3D;github&amp;logoColor&#x3D;white' border='0' alt='Buy Me a Coffee' />
</a>

<p align="center">
auto-generated by <a href="https://github.com/BabelDev0/writeme">writeme</a>
</p>
