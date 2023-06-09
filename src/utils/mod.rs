use anyhow::{anyhow, Error};
use handlebars::Handlebars;
use rand::seq::SliceRandom;
use rust_search::{FilterExt, SearchBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Paths to output files saved to disk produced by the application
pub mod outputs {
    pub const README: &str = "README.md";
    pub const CONTRIBUTING: &str = "CONTRIBUTING.md";
}

/// Paths to significant files
pub mod paths {
    pub enum UtilityPath {
        Configs,
        Techs,
        Lincenses,

        // license templates
        Apache20,
        MIT,
        GNUGPL,
        CreativeCommonsAttributionShareAlike40,

        //README.md templates
        // small pieces of markdown which require some data to be filled in
        ShieldReadme,
        AuthorReadme,
        ContribRocksReadme,
        SupportReadme,
        LicenseReadme,

        // large macro templates of the README file
        HeaderReadme,
        TocReadme,
        BodyReadme,
        FooterReadme,

        //CONTRIBUTING.md templates
        BodyContributing,
    }

    /// Returns the path of the given file for the given utility type
    pub fn read_util_file_contents(path: UtilityPath) -> String {
        let target = match path {
            UtilityPath::Configs => include_str!("../../conf/configs.yml"),
            UtilityPath::Techs => include_str!("../../conf/techs.yml"),
            UtilityPath::Lincenses => include_str!("../../conf/licenses.yml"),
            UtilityPath::Apache20 => include_str!("../../conf/lic/APACHE_20.md"),
            UtilityPath::MIT => include_str!("../../conf/lic/MIT.md"),
            UtilityPath::GNUGPL => include_str!("../../conf/lic/GNU_GPL.md"),
            UtilityPath::CreativeCommonsAttributionShareAlike40 => {
                include_str!("../../conf/lic/CREATIVE_COMMONS_ATTRIBUTION_SHARE_ALIKE_40.md")
            }
            UtilityPath::ShieldReadme => include_str!("../../conf/tpl/readme/SHIELD.md"),
            UtilityPath::AuthorReadme => include_str!("../../conf/tpl/readme/AUTHOR.md"),
            UtilityPath::ContribRocksReadme => {
                include_str!("../../conf/tpl/readme/CONTRIB_ROCKS.md")
            }
            UtilityPath::SupportReadme => include_str!("../../conf/tpl/readme/SUPPORT.md"),
            UtilityPath::HeaderReadme => include_str!("../../conf/tpl/readme/HEADER.md"),
            UtilityPath::TocReadme => include_str!("../../conf/tpl/readme/TABLE_OF_CONTENT.md"),
            UtilityPath::BodyReadme => include_str!("../../conf/tpl/readme/BODY.md"),
            UtilityPath::FooterReadme => include_str!("../../conf/tpl/readme/FOOTER.md"),
            UtilityPath::LicenseReadme => include_str!("../../conf/tpl/readme/LICENSE.md"),
            UtilityPath::BodyContributing => include_str!("../../conf/tpl/contributing/BODY.md"),
        };

        target.to_string()
    }
}

/// Used from entities that can be displayed as markdown
pub trait GenMarkdown {
    /// Generates the markdown code for the given object
    fn gen_md(&self) -> Result<String, Error>;
}

/// Structure used to represent shields.io badges
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shield {
    label: String,
    message: String,
    color: String,
    logo: String,
    label_color: String,
    logo_color: String,
    style: String,
    logo_width: i32,
    alt_text: String,
    target: String,
}

/// Structure used to represent a technology possibly used in the project
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tech {
    pub config_files: Vec<String>,
    pub dependency_names: Vec<String>,
    pub shield: Shield,
}

impl GenMarkdown for Shield {
    fn gen_md(&self) -> Result<String, Error> {
        let shield_tpl = paths::read_util_file_contents(paths::UtilityPath::ShieldReadme);
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("shield_tpl", shield_tpl)
            .unwrap();

        let data: Value = json!(self);

        Ok(handlebars.render("shield_tpl", &data).unwrap())
    }
}

/// Used to trim string removing quotes and spaces from the extremities
/// Returns a string slice with leading and trailing annoyingChars removed.

/// 'annoyingChars' are whitespace and double quote.
pub fn trim(s: String) -> Result<String, Error> {
    Ok(s.trim().trim_matches('"').to_string())
}

pub enum Alignment {
    Row,
    // Column,
}

/// Returns the markdown of shields related with the technologies in the project
pub fn shields(techs: Vec<String>, aligment: Alignment) -> Result<String, Error> {
    let contents: String = paths::read_util_file_contents(paths::UtilityPath::Techs);
    let all_techs: HashMap<String, Tech> = match serde_yaml::from_str(&contents) {
        Ok(t) => t,
        Err(_) => return Err(anyhow!("Error while parsing techs.yml")),
    };

    let mut shields: String = String::new();

    for (name, tech) in all_techs {
        if techs.contains(&name) {
            match tech.shield.gen_md() {
                Ok(md) => {
                    shields.push_str(&md);
                    match aligment {
                        Alignment::Row => shields.push(' '),
                        // Alignment::Column => shields.push_str("</br>"),
                    }
                }
                // if there is an error to generate markdown, just skip this shield
                Err(_) => continue,
            }
        }
    }

    Ok(shields)
}

#[derive(Clone, Debug)]
/// Structure used to represent the project aka global stuff useful when scanning a project
pub struct Project {
    /// it contains the paths of all the files in the project
    pub paths: Vec<String>,
}

/// Filter used to blacklist some directories from the search
fn blacklist_filter(entry: &rust_search::DirEntry) -> bool {
    let blacklist = vec![
        "node_modules",
        "target",
        "dist",
        "build",
        "vendor",
        "bin",
        ".git",
        ".bashrc",
        ".bash_profile",
        ".zshrc",
        ".zprofile",
        ".ssh",
        ".cargo",
        ".cache",
        ".config",
        ".vscode",
        ".idea",
        ".DS_Store",
        ".local",
        ".npm",
    ];
    !blacklist.contains(&entry.file_name().to_str().unwrap())
}

/// Implementation of the Project structure
/// - load: loads the project from the given location filling the paths vector
impl Project {
    pub fn load(project_location: &str) -> Result<Project, Error> {
        let paths: Vec<String> = SearchBuilder::default()
            .location(project_location)
            .custom_filter(blacklist_filter)
            .hidden()
            .build()
            .collect();

        Ok(Project { paths })
    }
}

// random fantasy descriptions
const DESCRIPTIONS: [&str; 42] = [
    "a mystical realm where imagination reigns supreme",
    "a celestial dance of colors and melodies",
    "an enchanted forest of boundless creativity",
    "a shimmering star guiding us towards innovation",
    "a symphony of brilliant minds harmonizing together",
    "a phoenix rising from the ashes of conventional thinking",
    "a magical tapestry of art, science, and wonder",
    "a breathtaking voyage through uncharted territories",
    "an ethereal sanctuary where imagination knows no bounds",
    "a kaleidoscope of possibilities waiting to be explored",
    "a luminous beacon of inspiration in a world of shadows",
    "a majestic dragon breathing life into daring ideas",
    "a secret garden where dreams take root and flourish",
    "a mystical portal to realms yet undiscovered",
    "an alchemical fusion of passion, vision, and brilliance",
    "a sanctuary where innovation and creativity intertwine",
    "an ever-evolving tapestry of boundless imagination",
    "a cosmic birthplace of revolutionary concepts",
    "a captivating symphony of technology and artistry",
    "a celestial masterpiece painted on the canvas of possibility",
    "a project that makes unicorns jealous",
    "the Gandalf of innovation, uttering \"You shall not stagnate!\"",
    "like a magical potion that turns ideas into gold",
    "the Chuck Norris of creativity, delivering roundhouse kicks to the mundane",
    "a project so epic, it could defeat the One Ring",
    "the secret ingredient in the recipe for mind-blowing awesomeness",
    "the Batman of projects, silently saving the world from mediocrity",
    "the Queen Bey of innovation, slaying it on every stage",
    "the project equivalent of a double rainbow, but with unicorns riding skateboards",
    "the Elon Musk of ideas, launching us into a future we never imagined",
    "a project that makes even cats say, \"I can haz innovation?\"",
    "the project equivalent of a disco ball, making everything sparkle with ingenuity",
    "the project that can turn \"undefined\" into \"defined\" with a single line of code",
    "like a regex wizard, matching innovation patterns with supernatural accuracy",
    "the project that can refactor spaghetti code into a gourmet lasagna of elegance",
    "like a CSS sorcerer, magically styling innovation with pixel-perfect precision",
    "the project that can conquer infinite loops with the power of infinite recursion",
    "the project that can optimize algorithms faster than the Flash on an espresso high",
    "the project that can debug issues with the ease of Neo dodging bullets in The Matrix",
    "the project that can deploy to production so seamlessly, it makes NASA's rocket launches ",
    "the project that can handle more data than a black hole can handle gravity",
    "the project that can refactor legacy code so effectively, it makes Dumbledore's wand ",
];

// use to generate a random fantasy description for the project
pub fn fantasy_description() -> String {
    let mut rng = rand::thread_rng();
    let fantasy_description = *DESCRIPTIONS.choose(&mut rng).unwrap();
    fantasy_description.to_string()
}
