use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, Error};
use serde_json::{json, Value};
use strum::EnumIter;

use crate::utils::{paths, GenMarkdown};

#[derive(Debug, Clone)]
/// How a project could be funded
pub struct Funding {
    pub f_type: FundingType,
    pub url: Option<String>,
}

#[derive(Debug, Clone, EnumIter)]
/// The possible funding types
pub enum FundingType {
    PAYPAL,
    PATREON,
    BITCOIN,
    BuyMeACoffee,
    KOFI,
    GITHUB,
}

impl FundingType {
    pub fn to_string(&self) -> &'static str {
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

pub enum FundingError {
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

        let support_tpl: String = paths::read_util_file_contents(paths::UtilityPath::SupportReadme);
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
pub struct Fundings(pub Vec<Funding>);

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
