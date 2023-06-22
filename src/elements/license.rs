use std::{collections::HashMap, fmt::Display, fs, str::FromStr};

use anyhow::Error;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use strum::EnumIter;

use crate::{
    converter::ConverterOutput,
    utils::{paths, GenMarkdown},
};

#[derive(Debug, Clone, PartialEq, EnumIter, Copy, Eq, Hash)]
/// The available licenses for a project which a user can choose from
pub enum SupportedLicense {
    Unknown,
    Apache20,
    MIT,
    GNUGeneralPublicLicense,
}

impl ToString for SupportedLicense {
    fn to_string(&self) -> String {
        match self {
            SupportedLicense::Unknown => "Unknown",
            SupportedLicense::Apache20 => "Apache-2.0",
            SupportedLicense::MIT => "MIT",
            SupportedLicense::GNUGeneralPublicLicense => "GNU General Public License",
        }
        .to_string()
    }
}

impl FromStr for SupportedLicense {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        let mut keywords: HashMap<Vec<String>, SupportedLicense> = HashMap::new();

        // TODO: raplace with file or something more clean
        keywords.insert(
            vec![
                SupportedLicense::Apache20.to_string(),
                "apache2".to_string(),
                "apache-2.0".to_string(),
                "apache-2".to_string(),
                "apache2.0".to_string(),
            ],
            SupportedLicense::Apache20,
        );
        keywords.insert(
            vec![
                SupportedLicense::MIT.to_string(),
                "mit".to_string(),
                "mit license".to_string(),
            ],
            SupportedLicense::MIT,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicense.to_string(),
                "gnu general public license".to_string(),
                "gnu gpl".to_string(),
                "gpl".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicense,
        );

        // check if contains the license
        let found = keywords.iter().find(|option| {
            // make all the options a regex and check if it matches
            let options: Vec<String> = option.0.iter().map(|c| format!(r"(?i)\b{}\b", c)).collect();
            let regex_set: regex::RegexSet = regex::RegexSet::new(options).unwrap();

            regex_set.is_match(s.as_str())
        });

        if let Some((_, license)) = found {
            Ok(*license)
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// The license object and related information
pub struct License {
    /// The license name
    pub name: SupportedLicense,
    /// The location in the project structure
    pub path: Option<String>,
    /// The location on the web (github repository for example)
    pub url: Option<String>,
}

impl License {
    /// Create a new license object from a name (string)
    ///
    /// If the name of the license is not recognized, the license will be set to `SupportedLicense::Unknown`
    pub fn from_name(name: String) -> Self {
        let license = match SupportedLicense::from_str(&name) {
            Ok(license) => license,
            Err(_) => {
                return Self {
                    name: SupportedLicense::Unknown,
                    path: None,
                    url: None,
                }
            }
        };

        Self {
            name: license,
            path: None,
            url: None,
        }
    }

    /// Create a new license object from a file
    ///
    /// If the license file is from a known platform (eg: github), the url will be set to the file location on the platform
    pub fn from_file(path: String) -> Self {
        let content = match fs::read_to_string(&path) {
            Ok(license) => license,
            Err(_) => {
                return Self {
                    name: SupportedLicense::Unknown,
                    path: None,
                    url: None,
                }
            }
        };

        match SupportedLicense::from_str(&content) {
            Ok(license) => Self {
                name: license,
                path: Some(path),
                url: None,
            },
            Err(_) => Self {
                name: SupportedLicense::Unknown,
                path: Some(path),
                url: None,
            },
        }
    }

    /// Returns a vector of ConverterOutput containing the license information
    ///
    pub fn scan(paths: &Vec<String>) -> Result<Vec<ConverterOutput>, Error> {
        let contents: String = paths::read_util_file_contents(paths::UtilityPath::Lincenses);
        let all_licenses: HashMap<String, Vec<String>> = serde_yaml::from_str(&contents).unwrap();

        let mut converter_outputs: Vec<ConverterOutput> = vec![];
        let mut license_present: Vec<String> = vec![];

        let all_license_file: Vec<String> = all_licenses
            .values()
            .flatten()
            .map(|c| format!(r"(?i){}$", c))
            .collect();

        let regex_set: regex::RegexSet = regex::RegexSet::new(all_license_file).unwrap();

        for path in paths {
            let path_str = path.as_str();

            let matches: Vec<_> = regex_set.matches(path_str).into_iter().collect();
            if !matches.is_empty() {
                license_present.push(path_str.to_string());
            }
        }

        license_present.iter().for_each(|p| {
            let mut converter = ConverterOutput::empty();

            converter.license = Option::from(License::from_file(p.to_string()));

            converter_outputs.push(converter);
        });

        Ok(converter_outputs)
    }

    pub fn create(project_location: &str, license: &License) -> Result<Option<String>, Error> {
        if license.path.is_some() || license.name == SupportedLicense::Unknown {
            return Ok(None);
        }

        let output_path = format!("{}/LICENSE.md", project_location);
        let mut license_file = match File::create(output_path.clone()) {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::new(e));
            }
        };

        license_file.write_all(b"\n")?;

        return Ok(Option::from(output_path));
    }
}

impl GenMarkdown for License {
    fn gen_md(&self) -> Result<String, anyhow::Error> {
        let license_tpl = paths::read_util_file_contents(paths::UtilityPath::License);
        let mut handlebars = handlebars::Handlebars::new();
        handlebars.register_template_string("license_tpl", license_tpl)?;

        let data = if let Some(url) = &self.url {
            json!({
                "name": self.name.to_string(),
                "target": url.clone()
            })
        } else {
            json!({
                "name": self.name.to_string()
            })
        };

        Ok(handlebars.render("license_tpl", &data).unwrap())
    }
}

impl Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _license = String::new();

        if self.path.is_some() {
            return write!(
                f,
                "{} ({})",
                self.name.to_string(),
                self.path.clone().unwrap()
            );
        };

        write!(f, "{}", self.name.to_string())
    }
}
