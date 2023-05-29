//! The converter module is a decorator pattern implementation which allows us to to convert any given (supported)
//! config file to a common OUTPUT object with the relevant information needed to generate a README file.
//!
//! Resources:
//! - https://refactoring.guru/design-patterns/decorator
//! - https://github.com/lpxxn/rust-design-pattern/blob/master/structural/decorator.rs

use std::{
    fmt::Display,
    fs,
    hash::{Hash, Hasher},
    path::Path,
    str::FromStr,
};

use crate::utils::{paths, trim, GenMarkdown};
use anyhow::{anyhow, Error};
use serde::Serialize;
use serde_json::{json, Value};

pub mod cargo_toml;
pub mod composer_json;
pub mod package_json;

// The base Component trait defines operations that can be altered by
// decorators.
pub trait Component {
    /// Convert the config file to the common ConverterOutput object
    fn convert(&self, file_path: String, file_contents: String) -> Result<ConverterOutput, Error>;

    /// Parses a contributor from the config file since they are not always in the same format depending on the
    /// config file type
    fn parse_contributor(&self, contributor: &Value) -> Result<Contributor, Error>;

    /// Parses a dependency from the config file since they are not always in the same format depending on the
    /// config file type
    fn parse_dependency(&self, key: &String, value: &Value) -> Result<Dependency, Error>;

    fn parse_funding(&self, funding: &Value) -> Result<Funding, Error>;
}

// Concrete Components provide default implementations of the operations.
// There might be several variations of these classes.
pub struct ConcreteComponent {}

impl Component for ConcreteComponent {
    fn convert(
        &self,
        _file_path: String,
        _file_contents: String,
    ) -> Result<ConverterOutput, Error> {
        Ok(ConverterOutput {
            source_config_file_path: String::new(),
            name: None,
            description: None,
            version: None,
            contributors: None,
            license: None,
            keywords: None,
            repository_url: None,
            homepage_url: None,
            dependencies: None,
            dev_dependencies: None,
            build_dependencies: None,
            funding: None,
        })
    }

    fn parse_contributor(&self, contributor: &Value) -> Result<Contributor, Error> {
        Ok(Contributor {
            name: contributor["name"].as_str().map(|s| s.to_string()),
            email: contributor["email"].as_str().map(|s| s.to_string()),
            url: contributor["url"].as_str().map(|s| s.to_string()),
        })
    }

    fn parse_dependency(&self, key: &String, value: &Value) -> Result<Dependency, Error> {
        Ok(Dependency {
            name: key.to_string(),
            version: Some(value.to_string()),
        })
    }

    fn parse_funding(&self, funding: &Value) -> Result<Funding, Error> {
        let possible_values: [&str; 6] = [
            (FundingType::BITCOIN.to_string()),
            (FundingType::BuyMeACoffee.to_string()),
            (FundingType::GITHUB.to_string()),
            (FundingType::KOFI.to_string()),
            (FundingType::PATREON.to_string()),
            (FundingType::GITHUB.to_string()),
        ];

        let f_type = funding["type"].to_string();
        let url = funding["url"].to_string();

        for possible_value in possible_values.iter() {
            if possible_value.contains(&f_type) || possible_value.contains(&url) {
                let f_type = match FundingType::from_str(possible_value) {
                    Ok(t) => t,
                    Err(_e) => {
                        return Err(anyhow!("Unsupported funding type"));
                    }
                };

                let funding = Funding {
                    f_type,
                    url: Some(url),
                };

                return Ok(funding);
            }
        }

        Err(anyhow!("Unsupported funding type"))
    }
}

// The base Decorator class follows the same interface as the other
// components. The primary purpose of this class is to define the wrapping
// interface for all concrete decorators. The default implementation of the
// wrapping code might include a field for storing a wrapped component and
// the means to initialize it.
pub trait Decorator: Component {
    fn new(/* component: Rc<dyn Component> */) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub enum SupportedFile {
    ComposerJson,
    PackageJson,
    CargoToml,
}

impl SupportedFile {
    fn from_str(file_type: &str) -> Result<SupportedFile, Error> {
        match file_type {
            "composer.json" => Ok(SupportedFile::ComposerJson),
            "package.json" => Ok(SupportedFile::PackageJson),
            "Cargo.toml" => Ok(SupportedFile::CargoToml),
            _ => Err(anyhow!("Unsupported file type")),
        }
    }
}

impl Display for SupportedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_type = match self {
            SupportedFile::ComposerJson => "composer.json",
            SupportedFile::PackageJson => "package.json",
            SupportedFile::CargoToml => "Cargo.toml",
        };

        write!(f, "{}", file_type)
    }
}

#[derive(Debug, Clone)]
/// Holds the information of a dependency in a config file
pub struct Dependency {
    /// The name of the dependency
    pub name: String,

    /// The version of the dependency, it may be missing!
    version: Option<String>,
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.name,
            self.version.as_ref().unwrap_or(&"None".to_string())
        )
    }
}

#[derive(Debug, Clone)]
/// This Vec variant is needed to implement the Display trait for the Vec<T> scenarios
///
/// Reference: https://stackoverflow.com/a/30633256/11802618
pub struct Dependencies(Vec<Dependency>);

impl Display for Dependencies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dependencies = String::new();
        for dependency in &self.0 {
            dependencies.push_str(&format!("{} ", dependency));
        }
        write!(f, "{}", dependencies)
    }
}

impl FromIterator<Dependency> for Dependencies {
    fn from_iter<I: IntoIterator<Item = Dependency>>(iter: I) -> Self {
        let mut dependencies = Vec::new();
        for dependency in iter {
            dependencies.push(dependency);
        }
        Dependencies(dependencies)
    }
}

impl Iterator for Dependencies {
    type Item = Dependency;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug, Clone, Serialize, Eq)]
/// A contributor to the project
pub struct Contributor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
}

// two contributors are equal if they have the same email
impl PartialEq for Contributor {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email
    }
}

// Hash only by email
impl Hash for Contributor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

impl Display for Contributor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.name.as_ref().unwrap_or(&"None".to_string()),
            self.email.as_ref().unwrap_or(&"None".to_string()),
            self.url.as_ref().unwrap_or(&"None".to_string())
        )
    }
}

impl GenMarkdown for Contributor {
    fn gen_md(&self) -> Result<String, Error> {
        if self.name.is_none() {
            return Err(anyhow!("Contributor name is missing"));
        }

        // build md string if at least name and one of the other fields are present
        if self.name.is_some() && (self.url.is_some() || self.email.is_some()) {
            let author_tpl = paths::read_util_file_contents(paths::UtilityPath::Author);
            let mut handlebars = handlebars::Handlebars::new();
            handlebars
                .register_template_string("author_tpl", author_tpl)
                .unwrap();

            // extract the url field from the url or email field, at least one of them is present if we are here
            let url = self.url.as_ref().unwrap_or(self.email.as_ref().unwrap());

            let data: Value = json!({
                "name": self.name.as_ref().unwrap(),
                "url": url,
            });

            return Ok(handlebars.render("author_tpl", &data).unwrap());
        }

        Ok(self.name.clone().unwrap())
    }
}

#[derive(Debug, Clone)]
/// This Vec variant is needed to implement the Display trait for the Vec<T> scenarios
///
/// Reference: https://stackoverflow.com/a/30633256/11802618
pub struct Contributors(Vec<Contributor>);

impl Display for Contributors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut contributors = String::new();
        for contributor in &self.0 {
            contributors.push_str(&format!("{} ", contributor));
        }
        write!(f, "{}", contributors)
    }
}

impl FromIterator<Contributor> for Contributors {
    fn from_iter<I: IntoIterator<Item = Contributor>>(iter: I) -> Self {
        let mut contributors = Vec::new();
        for contributor in iter {
            contributors.push(contributor);
        }
        Contributors(contributors)
    }
}

impl Iterator for Contributors {
    type Item = Contributor;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug, Clone)]
/// How a project could be funded
pub struct Funding {
    f_type: FundingType,
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
/// The possible funding types
enum FundingType {
    PAYPAL,
    PATREON,
    BITCOIN,
    BuyMeACoffee,
    KOFI,
    GITHUB,
}

impl FundingType {
    fn to_string(&self) -> &'static str {
        match self {
            FundingType::BITCOIN => "bitcoin",
            FundingType::BuyMeACoffee => "buymeacoffee",
            FundingType::GITHUB => "github",
            FundingType::KOFI => "kofi",
            FundingType::PATREON => "patreon",
            FundingType::PAYPAL => "paypal",
        }
    }
}

enum FundingError {
    FundingNotSupported,
}

impl FromStr for FundingType {
    type Err = FundingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bitcoin" => Ok(FundingType::BITCOIN),
            "buymeacoffee" => Ok(FundingType::BuyMeACoffee),
            "github" => Ok(FundingType::GITHUB),
            "kofi" => Ok(FundingType::KOFI),
            "patreon" => Ok(FundingType::PATREON),
            "paypal" => Ok(FundingType::PAYPAL),
            _ => Err(FundingError::FundingNotSupported),
        }
    }
}

trait EnumIterator {
    type Item;

    fn enum_iterator() -> std::slice::Iter<'static, Self::Item>;
}

impl EnumIterator for FundingType {
    type Item = FundingType;

    fn enum_iterator() -> std::slice::Iter<'static, FundingType> {
        static VARIANTS: [FundingType; 6] = [
            FundingType::BITCOIN,
            FundingType::BuyMeACoffee,
            FundingType::GITHUB,
            FundingType::KOFI,
            FundingType::PATREON,
            FundingType::PAYPAL,
        ];
        VARIANTS.iter()
    }
}

impl Display for Funding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.f_type.to_string(),
            self.url.as_ref().unwrap_or(&"None".to_string())
        )
    }
}

impl GenMarkdown for Funding {
    fn gen_md(&self) -> Result<String, Error> {
        if self.url.is_none() {
            return Err(anyhow!("Funding url is missing"));
        }

        let support_tpl: String = paths::read_util_file_contents(paths::UtilityPath::Support);
        let mut handlebars = handlebars::Handlebars::new();
        handlebars
            .register_template_string("support_tpl", support_tpl)
            .unwrap();

        // use only url for now type is useless
        let url = self.url.as_ref().unwrap();

        let template_url = match self.f_type {
            FundingType::BITCOIN => "https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white",
            FundingType::BuyMeACoffee => "https://img.shields.io/badge/BuyMeACoffee-F16061?style=for-the-badge&logo=buymeacoffee&logoColor=FFDD00",
            FundingType::GITHUB => "https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white",
            FundingType::KOFI => "https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white",
            FundingType::PATREON => "https://img.shields.io/badge/Patreon-F16061?style=for-the-badge&logo=patreon&logoColor=white",
            FundingType::PAYPAL => "https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white",
        };

        let data: Value = json!({
            "url": url ,
            "template_url": template_url,
        });

        Ok(handlebars.render("support_tpl", &data).unwrap())
    }
}

#[derive(Debug, Clone)]
/// This Vec variant is needed to implement the Display trait for the Vec<T> scenarios
///
/// Reference: https://stackoverflow.com/a/30633256/11802618
pub struct Fundings(Vec<Funding>);

impl Display for Fundings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut funding = String::new();
        for f in &self.0 {
            funding.push_str(&format!("{} ", f));
        }
        write!(f, "{}", funding)
    }
}

impl FromIterator<Funding> for Fundings {
    fn from_iter<I: IntoIterator<Item = Funding>>(iter: I) -> Self {
        let mut funding = Vec::new();
        for f in iter {
            funding.push(f);
        }
        Fundings(funding)
    }
}

impl Iterator for Fundings {
    type Item = Funding;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug, Clone)]
/// The output object that will be returned from each converter implementation regardless of the config file provided
pub struct ConverterOutput {
    pub source_config_file_path: String,

    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub contributors: Option<Contributors>,
    pub license: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub repository_url: Option<String>,
    pub homepage_url: Option<String>,

    /// dependencies of the project
    pub dependencies: Option<Dependencies>,

    /// dev dependencies of the project
    pub dev_dependencies: Option<Dependencies>,

    /// build dependencies of the project, not every config file supports this
    pub build_dependencies: Option<Dependencies>,

    /// funding of the project, not every config file supports this (eg. Cargo.toml)
    pub funding: Option<Fundings>,
}

impl ConverterOutput {
    /// Creates a new empty output object
    pub fn empty() -> Self {
        ConverterOutput {
            source_config_file_path: String::new(),
            name: None,
            description: None,
            version: None,
            contributors: None,
            license: None,
            keywords: None,
            repository_url: None,
            homepage_url: None,
            dependencies: None,
            dev_dependencies: None,
            build_dependencies: None,
            funding: None,
        }
    }

    // trim all the fields removing quotes and spaces from start and end
    pub fn trim(&mut self) {
        self.name = self.name.take().map(|s| trim(s).unwrap());
        self.description = self.description.take().map(|s| trim(s).unwrap());
        self.version = self.version.take().map(|s| trim(s).unwrap());
        self.license = self.license.take().map(|s| trim(s).unwrap());
        self.repository_url = self.repository_url.take().map(|s| trim(s).unwrap());
        self.homepage_url = self.homepage_url.take().map(|s| trim(s).unwrap());
    }
}

/// Converts a given config file to a common Output object
pub struct Converter;

impl Converter {
    pub fn new() -> Self {
        Converter {}
    }
    // pub fn convert<T: Component>(
    //     &self,
    //     path: &str,
    //     component: &T,
    // ) -> Result<ConverterOutput, Error> {
    //     let contents = std::fs::read_to_string(&path)
    //         .expect("Should have been able to read the template file");

    //     component.convert(contents)
    // }
    /// Gets the filename from a path string
    fn get_filename(path: &str) -> Option<&str> {
        let path = Path::new(path);

        path.file_name().and_then(|s| s.to_str())
    }

    /// Converts a given config file to a common Output object
    pub fn convert(&self, path: &str) -> Result<ConverterOutput, Error> {
        let contents =
            fs::read_to_string(path).expect("Should have been able to read the template file");

        let config_file = match Converter::get_filename(path).map(SupportedFile::from_str) {
            Some(Ok(f)) => f,
            Some(Err(e)) => return Err(anyhow!(e)),
            None => return Err(anyhow!("File not found")),
        };

        match config_file {
            SupportedFile::PackageJson => {
                package_json::PackageJson::new().convert(path.to_string(), contents)
            }
            SupportedFile::ComposerJson => {
                composer_json::ComposerJson::new().convert(path.to_string(), contents)
            }
            SupportedFile::CargoToml => {
                cargo_toml::CargoToml::new().convert(path.to_string(), contents)
            }
        }
    }
}
