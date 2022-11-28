use colored::{ColoredString, Colorize};
use console::Term;
use dialoguer::Select;

use super::manipulate_json::read_json_file;

pub fn change_to_color(text: &str, color: &str) -> ColoredString {
    let colored_text = match color {
        "BrightBlack" => text.bright_black(),
        "BrightRed" => text.bright_red(),
        "BrightGreen" => text.bright_green(),
        "BrightYellow" => text.bright_yellow(),
        "BrightBlue" => text.bright_blue(),
        "BrightMagenta" => text.bright_magenta(),
        "BrightCyan" => text.bright_cyan(),
        "BrightWhite" => text.bright_white(),
        _ => text.color(color),
    };
    colored_text
}

pub fn color_using_theme(text: &str) -> ColoredString {
    let default_theme = read_json_file().options.default_theme_color;

    let colored_text = change_to_color(text, &default_theme);
    colored_text
}

pub fn choose_color() -> String {
    let items = vec![
        change_to_color("Black", "Black"),
        change_to_color("Red", "Red"),
        change_to_color("Green", "Green"),
        change_to_color("Yellow", "Yellow"),
        change_to_color("Blue", "Blue"),
        change_to_color("Magenta", "Magenta"),
        change_to_color("Cyan", "Cyan"),
        change_to_color("White", "White"),
        change_to_color("BrightBlack", "BrightBlack"),
        change_to_color("BrightRed", "BrightRed"),
        change_to_color("BrightGreen", "BrightGreen"),
        change_to_color("BrightYellow", "BrightYellow"),
        change_to_color("BrightBlue", "BrightBlue"),
        change_to_color("BrightMagenta", "BrightMagenta"),
        change_to_color("BrightCyan", "BrightCyan"),
        change_to_color("BrightWhite", "BrightWhite"),
    ];

    let selection = Select::new()
        .items(&items)
        .interact_on_opt(&Term::stderr())
        .unwrap();

    let chosen_color = match selection {
        Some(value) => match value {
            0 => "Black",
            1 => "Red",
            2 => "Green",
            3 => "Yellow",
            4 => "Blue",
            5 => "Magenta",
            6 => "Cyan",
            7 => "White",
            8 => "BrightBlack",
            9 => "BrightRed",
            10 => "BrightGreen",
            11 => "BrightYellow",
            12 => "BrightBlue",
            13 => "BrightMagenta",
            14 => "BrightCyan",
            15 => "BrightWhite",
            _ => "White",
        },
        None => "White",
    };
    String::from(chosen_color)
}
