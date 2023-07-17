use std::{collections::HashMap, str::FromStr};

use anyhow::Error;
use git2::Repository as GitRepository;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use strum::EnumIter;

use crate::{
    converter::{Contributor, Contributors, ConverterOutput},
    dialoguer,
    utils::{paths, trim, GenMarkdown},
};

/// The repository information of the project retrieved from scanning .git folder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub url: String,
    /// Represents the name of the repository
    ///
    /// e.g. https://github.com/writeme-project/writeme.git -> **writeme**
    pub name: Option<String>,
    /// Represents the user+repo name in the form user/repo
    ///
    /// e.g. https://github.com/writeme-project/writeme.git -> **writeme-project/writeme**
    pub sign: Option<String>,
    pub platform: RepositoryPlatform,
}

impl Repository {
    pub fn new(url: String) -> Self {
        let url = trim(url).unwrap();
        // check that the url is in the https|http://platform/user/name form
        let regex =
            regex::Regex::new(r"^(git@[^:/]+:[^/]+/[^.]+\.git|https?://[^/]+/[^/]+/[^/]+)$")
                .unwrap();
        if !regex.is_match(&url) {
            return Self {
                url,
                name: None,
                sign: None,
                platform: RepositoryPlatform::Unknown,
            };
        }

        if url.starts_with("git") {
            let url_split = url.split(':').collect::<Vec<&str>>();
            let sign = url_split[1..]
                .join("/")
                .split(".git")
                .collect::<Vec<&str>>()[0]
                .to_string();

            let name = sign
                .split('/')
                .collect::<Vec<&str>>()
                .last()
                .unwrap_or(&"")
                .to_string();

            let platform_str = url_split[0].split('@').collect::<Vec<&str>>()[1];
            let platform = RepositoryPlatform::from_str(platform_str).unwrap();

            Self {
                url: format!("https://{}.com/{}", platform.to_string(), sign),
                sign: Some(sign),
                name: Some(name),
                platform,
            }
        } else {
            let url_split = url.split('/').collect::<Vec<&str>>();
            let sign = url_split[3..]
                .join("/")
                .split(".git")
                .collect::<Vec<&str>>()[0]
                .to_string();
            let name = sign
                .split('/')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string();

            let platform_str = url_split[2];
            let platform = RepositoryPlatform::from_str(platform_str).unwrap();

            Self {
                url: url.split(".git").collect::<Vec<&str>>()[0].to_string(),
                sign: Some(sign),
                name: Some(name),
                platform,
            }
        }
    }

    /// Returns a ConverterOutput struct with the data found in the .git folder
    pub fn scan(project_location: &str) -> Result<ConverterOutput, Error> {
        let mut git_converter = ConverterOutput::empty();

        git_converter.source_config_file_path = format!("{}.git", project_location);

        // Open the repository
        let repo: GitRepository = match GitRepository::open(project_location) {
            Ok(repo) => repo,
            Err(e) => {
                dialoguer::error("Failed to open repository: {}", &e);
                return Ok(git_converter);
            }
        };

        let url: String = repo
            .find_remote("origin")
            .unwrap()
            .url()
            .unwrap_or(&"".to_string())
            .to_string();

        let project_repository = Repository::new(url);
        git_converter.repository = Option::from(project_repository.clone());

        git_converter.name = project_repository.name.clone();

        // check if the repo is a github repo
        // if so not need to continue
        if project_repository.platform == RepositoryPlatform::Github {
            return Ok(git_converter);
        }

        // Get the head commit
        let head = match repo.head() {
            Ok(head) => head,
            Err(e) => {
                dialoguer::error("Failed to get repository head: {}", &e);
                return Ok(git_converter);
            }
        };

        let head_commit = match head.peel_to_commit() {
            Ok(commit) => commit,
            Err(e) => {
                dialoguer::error("Failed to peel to commit of the repository: {}", &e);
                return Ok(git_converter);
            }
        };

        // Iterate over the commits in the repository
        let mut revwalk = match repo.revwalk() {
            Ok(revwalk) => revwalk,
            Err(e) => {
                dialoguer::error("Failed to get revwalk of the repository: {}", &e);
                return Ok(git_converter);
            }
        };

        revwalk.push(head_commit.id()).unwrap();

        let mut contributors: HashMap<Contributor, i32> = std::collections::HashMap::new();

        // fill contributors hashmap counting the number of commits for each contributor
        for oid in revwalk {
            let oid = match oid {
                Ok(oid) => oid,
                Err(e) => {
                    dialoguer::error("Failed to get oid of the repository: {}", &e);
                    return Ok(git_converter);
                }
            };

            let commit = match repo.find_commit(oid) {
                Ok(commit) => commit,
                Err(e) => {
                    dialoguer::error("Failed to find commit: {}", &e);
                    return Ok(git_converter);
                }
            };

            let author = commit.author();
            let name = author.name().unwrap_or(&"");
            let email = author.email().unwrap_or(&"");

            let contributor = Contributor {
                name: Some(name.to_string()),
                email: Some(email.to_string()),
                url: None,
            };

            let count = contributors.entry(contributor).or_insert(0);
            *count += 1;
        }

        // sort contributors by number of commits
        let contributors: Contributors = contributors
            .iter()
            .sorted_by(|a, b| b.1.cmp(a.1))
            .map(|(contributor, _)| contributor.clone())
            .collect();

        git_converter.contributors = Option::from(contributors);

        Ok(git_converter)
    }
}

impl GenMarkdown for Repository {
    fn gen_md(&self) -> Result<String, Error> {
        let contrib_rocks_tpl =
            paths::read_util_file_contents(paths::UtilityPath::ContribRocksReadme);
        let mut handlebars = handlebars::Handlebars::new();
        handlebars
            .register_template_string("contrib_rocks_tpl", contrib_rocks_tpl)
            .unwrap();

        let repo_url = self.url.clone();
        let repo_contrib_url = format!(
            "{}/graphs/contributors",
            repo_url.split(".git").collect::<Vec<&str>>()[0]
        );

        let data: Value = json!({
            "repository_contrib_url": repo_contrib_url,
            "repository_sign": self.sign.clone().unwrap(),
        });

        Ok(handlebars.render("contrib_rocks_tpl", &data).unwrap())
    }
}

// possible repository platforms
#[derive(Debug, Clone, EnumIter, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepositoryPlatform {
    Github,
    Gitlab,
    Bitbucket,
    SelfHosted,
    Unknown,
}

impl RepositoryPlatform {
    fn to_string(&self) -> &'static str {
        match self {
            RepositoryPlatform::Github => "github",
            RepositoryPlatform::Gitlab => "gitlab",
            RepositoryPlatform::Bitbucket => "bitbucket",
            RepositoryPlatform::SelfHosted => "self-hosted",
            RepositoryPlatform::Unknown => "unknown",
        }
    }
}

impl PartialEq<str> for RepositoryPlatform {
    fn eq(&self, other: &str) -> bool {
        self.to_string() == other
    }
}

impl FromStr for RepositoryPlatform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.contains("github") => Ok(RepositoryPlatform::Github),
            s if s.contains("gitlab") => Ok(RepositoryPlatform::Gitlab),
            s if s.contains("bitbucket") => Ok(RepositoryPlatform::Bitbucket),
            _ => Ok(RepositoryPlatform::Unknown),
        }
    }
}

impl ToString for RepositoryPlatform {
    fn to_string(&self) -> String {
        match self {
            RepositoryPlatform::Github => "github".to_string(),
            RepositoryPlatform::Gitlab => "gitlab".to_string(),
            RepositoryPlatform::Bitbucket => "bitbucket".to_string(),
            RepositoryPlatform::SelfHosted => "self-hosted".to_string(),
            RepositoryPlatform::Unknown => "unknown".to_string(),
        }
    }
}
