use std::{io::stdout, thread::sleep, time::Duration};

use colored::Colorize;
use log_update::LogUpdate;

pub fn header() {
    let app_name = format!("{}", "WRITEME:".cyan());
    let catch_phrase = format!("{}", "Let's write your README!");
    wirtino(app_name, catch_phrase);
}

fn wirtino(app_name: String, catch_phrase: String) {
    let mut log_update = LogUpdate::new(stdout()).unwrap();

    let eyes = vec!["•", "o", "•", "o"];

    let mouths = vec!["O", "•", "O", "•"];

    let walls = vec!["─", "|"];

    let corners = vec!["╭", "╮", "╰", "╯"];

    let loading = vec![".", " ", " ", " "];

    // println!("{}{}{}", corners[0], walls[0], corners[1]);
    // println!(
    //     "{}{}{}\t{}",
    //     eyes[1].cyan().italic(),
    //     " ",
    //     eyes[1].cyan().italic(),
    //     app_name
    // );
    // println!("{}{}{}\t{}", walls[1], " ", walls[1], catch_phrase);
    // println!("{}{}{}", corners[2], mouths[0].cyan().italic(), corners[3]);
    // let f = format!(
    //     "\n{}\t{}",
    //     "v0.1.0".bright_green(),
    //     "I'm reading your stuff...."
    // );
    // println!("{}", f);

    for i in 0..5 {
        let ind = i % 4;
        log_update
            .render(&format!(
                "{}{}{}\n{}{}{}\t{}\n{}{}{}\t{}\n{}{}{}\n{}\t{}{}{}{}{}\n",
                corners[0],
                walls[0],
                corners[1],
                eyes[ind].cyan().italic(),
                " ",
                eyes[ind].cyan().italic(),
                app_name,
                walls[1],
                " ",
                walls[1],
                catch_phrase,
                corners[2],
                mouths[ind].cyan().italic(),
                corners[3],
                "v0.1.0".bright_green(),
                "I'm reading your stuff",
                loading[(ind) % 4],
                loading[(ind + 3) % 4],
                loading[(ind + 2) % 4],
                loading[(ind + 1) % 4],
            ))
            .unwrap();

        sleep(Duration::from_millis(300));
    }
}

// fn conflict<T>(field_name: &str, values: Vec<Option<T>>) -> Option<T> {
//     put a space before and after the field name
//     let field_name = format!(" {} ", field_name);
//     let label = format!(
//         "{}\t{}",
//         field_name.bright_white().on_purple(),
//         "Witch of these you wanna have in your awsome README?".bright_white()
//     );
//     print!("{}", label);
//     return None;
// }
