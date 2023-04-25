use colored::Colorize;
use dialoguer::console::Style;
use dialoguer::Select;
use dialoguer::{console::style, theme::ColorfulTheme};
use itertools::Itertools;
use std::fmt::{Debug, Display};
// use log_update::LogUpdate;
// use std::{io::stdout, thread::sleep, time::Duration};

pub fn hello() {
    wirtino();
}

fn wirtino() {
    let eyes = vec!["•", "o", "•", "o"];
    let mouths = vec!["O", "•", "O", "•"];
    let walls = vec!["─", "|"];
    let corners = vec!["╭", "╮", "╰", "╯"];

    println!("{}{}{}", corners[0], walls[0], corners[1]);
    println!(
        "{}{}{}\t{}",
        eyes[1].cyan().italic(),
        " ",
        eyes[1].cyan().italic(),
        "HI! I AM WRITINO:".cyan()
    );
    println!(
        "{}{}{}\t{}",
        walls[1], " ", walls[1], "Let's write your README!"
    );
    println!("{}{}{}", corners[2], mouths[0].cyan().italic(), corners[3]);
    let f = format!("{} {}", "WRITEME".cyan(), "v0.1.0".bright_green());
    println!("{}", f);

    // let mut log_update = LogUpdate::new(stdout()).unwrap();
    // let loading = vec![".", " ", " ", " "];
    // for i in 0..5 {
    //     let ind = i % 4;
    //     log_update
    //         .render(&format!(
    //             "{}{}{}\n{}{}{}\t{}\n{}{}{}\t{}\n{}{}{}\n{}\t{}{}{}{}{}\n",
    //             corners[0],
    //             walls[0],
    //             corners[1],
    //             eyes[ind].cyan().italic(),
    //             " ",
    //             eyes[ind].cyan().italic(),
    //             app_name,
    //             walls[1],
    //             " ",
    //             walls[1],
    //             catch_phrase,
    //             corners[2],
    //             mouths[ind].cyan().italic(),
    //             corners[3],
    //             "v0.1.0".bright_green(),
    //             "I'm reading your stuff",
    //             loading[(ind) % 4],
    //             loading[(ind + 3) % 4],
    //             loading[(ind + 2) % 4],
    //             loading[(ind + 1) % 4],
    //         ))
    //         .unwrap();

    //     sleep(Duration::from_millis(300));
    // }
}

pub fn conflict<T: Clone + Debug + Display>(field_name: &str, values: Vec<Option<T>>) -> Option<T> {
    // put a space before and after the field name
    let field_name = format!(" {} ", field_name);
    let label = format!(
        "{} {}",
        field_name.bright_white().on_truecolor(127, 0, 255),
        "Which of these do you want in your awesome README?"
    );

    let with_value = values
        .iter()
        .filter(|s| s.is_some())
        .map(|s| s.as_ref().unwrap())
        .collect_vec();

    // every value of the field is empty, return None
    if with_value.is_empty() {
        return None;
    }

    // does the field need merging? it does so when the filtered non-None values are more than one
    let needs_merge = with_value.len() > 1;

    if !needs_merge {
        return Some(with_value[0].clone());
    }

    let theme: ColorfulTheme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        active_item_prefix: style("○".to_string()).for_stderr().green(),
        ..ColorfulTheme::default()
    };

    // ask the user which value to keep
    let selection = Select::with_theme(&theme)
        .with_prompt(label.to_string())
        .items(&with_value)
        .default(0)
        .interact()
        .unwrap_or(0);

    Some(with_value[selection].clone())
}
