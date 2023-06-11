use colored::Colorize;
use dialoguer::console::Style;
use dialoguer::Select;
use dialoguer::{console::style, theme::ColorfulTheme};
use itertools::Itertools;
use std::fmt::{Debug, Display};

// use log_update::LogUpdate;
// use std::{io::stdout, thread::sleep, time::Duration};

#[derive(Debug, Clone)]
/// Represents a single option in a select menu
pub struct SelectOption<T> {
    pub name: String,
    pub value: Option<T>,
}

impl<T: Display> Display for SelectOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => write!(f, "{} ({})", value, self.name),
            None => write!(f, "None ({})", self.name),
        }
    }
}

// say hi to the user
pub fn hello() {
    wirtino();
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("{} {}\n", "WRITEME".cyan(), VERSION.bright_green());
}

// our little mascot
fn wirtino() {
    let eye = "•";
    let mouth = "O";
    let walls = vec!["─", "|"];
    let corners = vec!["╭", "╮", "╰", "╯"];

    println!("{}{}{}", corners[0], walls[0], corners[1]);
    println!(
        "{} {}\t{}",
        eye.cyan().italic(),
        eye.cyan().italic(),
        "HI! I AM WRITINO:".cyan()
    );
    println!("{} {}\tLet's write your README!", walls[1], walls[1]);
    println!("{}{}{}\n", corners[2], mouth.cyan().italic(), corners[3]);
}

/// Asks the user to choose one of the provided values
pub fn select_option<T: Clone + Debug + Display>(
    field_name: &str,
    values: Vec<SelectOption<T>>,
    custom_label: Option<String>,
) -> Option<T> {
    // put a space before and after the field name
    let field_name = format!(" {} ", field_name);
    let label = format!(
        "{} {}",
        field_name.bright_white().on_truecolor(127, 0, 255),
        custom_label.unwrap_or("Which of these do you want in your awesome README?".to_string())
    );

    let with_value = values.iter().filter(|v| v.value.is_some()).collect_vec();

    // every value of the field is empty, return None
    if with_value.is_empty() {
        return None;
    }

    // does the field need merging? it does so when the filtered non-None values are more than one
    let needs_merge = with_value.len() > 1;

    if !needs_merge {
        return with_value[0].value.clone();
    }

    let theme: ColorfulTheme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        active_item_prefix: style("○".to_string()).for_stderr().green(),
        ..ColorfulTheme::default()
    };

    // ask the user which value to keep
    let selection = Select::with_theme(&theme)
        .with_prompt(label)
        .items(&with_value)
        .default(0)
        .max_length(10)
        .interact()
        .unwrap_or(0);

    println!(" ");

    with_value[selection].value.clone()
}

// show the list of processed files to the user
pub fn processed_files(files: Vec<String>) {
    let mut processed_files = String::new();
    let head_str = "Files processed";
    let no_files_str = "0 files to process";

    let to_show_threshold = 10;

    // max lenght of the files names to show
    let mut max_len = files
        .iter()
        .take(to_show_threshold)
        .map(|f| f.len())
        .max()
        .unwrap_or(0);
    let remanent = files.len() as i16 - to_show_threshold as i16;
    let remanents_str = format!("Others {} files processed", remanent);

    // if the max_len is 0, we need to set the max_len to the length of the no_files_str string

    // if there is more that {to_show_threshold} files, but the max_len is less than 27,
    // we need to set it to remanents_str.len() to avoid the "Others n files processed" string to be cut

    // if there isn't more than {to_show_threshold} files but the max_len is less than no_files_str.len()
    // we need to set it to no_files_str.len() to avoid the "0 files to process" string to be cut
    if max_len == 0 {
        max_len = no_files_str.len();
    } else if max_len < remanents_str.len() && remanent > 0 {
        max_len = remanents_str.len();
    } else if max_len < head_str.len() {
        max_len = head_str.len();
    }

    // head_strer, push ╭─Files processed───────╮
    processed_files.push_str(&format!(
        "╭─{}{}╮\n",
        head_str.cyan(),
        "─".repeat(max_len + 1 - head_str.len())
    ));

    // if there are no files, push | 0 files to process |
    if files.is_empty() {
        processed_files.push_str(&format!(
            "│ {}{} │\n",
            no_files_str,
            " ".repeat(max_len - no_files_str.len())
        ));
    }

    // for each file, push | file_name |
    for file in files.iter().take(to_show_threshold) {
        processed_files.push_str(&format!(
            "│ {}{} │\n",
            file,
            " ".repeat(max_len - file.len())
        ));
    }

    if remanent > 0 {
        processed_files.push_str(&format!(
            "│ {}{} │\n",
            remanents_str,
            " ".repeat(max_len - remanents_str.len())
        ));
    }

    // footer, push ╰──────────────────────╯
    processed_files.push_str(&format!("╰{}╯\n", "─".repeat(max_len + 2)));

    println!("{}", processed_files);
}

// say bye to the user
pub fn bye() {
    println!(
        "{} {}",
        "🎉".bright_green(),
        "Your README is ready!".bright_green()
    );
}

// fuck, something went wrong
pub fn error(msg: &str, arg: &dyn Display) {
    let formatted = format!("{}", arg);
    let result = msg.replace("{}", &formatted);
    println!("{} {}", "🚨".bright_red(), result.bright_red());
}
