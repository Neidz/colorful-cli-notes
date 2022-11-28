use colored::Colorize;
use console::Term;
use dialoguer::Select;

use super::{
    color_utils::{change_to_color, choose_color, color_using_theme},
    manipulate_json::{read_json_file, write_json_to_file},
    note_utils::{create_blank_note, edit_content, edit_links, edit_theme, edit_title},
    structs::Note,
};

pub fn show_notes() {
    let current_content = read_json_file();

    let mut titles: Vec<colored::ColoredString> = current_content
        .notes
        .iter()
        .map(|note| change_to_color(&note.title, &note.theme))
        .collect();
    titles.push(color_using_theme("Back"));

    loop {
        let selection = Select::new()
            .items(&titles)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(index) => match index {
                index if index == titles.len() - 1 => break,
                index => {
                    let note = &current_content.notes[index];

                    println!("{}", change_to_color(&note.title, &note.theme).bold());
                    println!("{}", change_to_color(&note.content, &note.theme));

                    let colored_links: Vec<colored::ColoredString> = note
                        .links
                        .iter()
                        .map(|link| change_to_color(&link, &note.theme).italic())
                        .collect();

                    let selection = Select::new()
                        .items(&colored_links)
                        .interact_on_opt(&Term::stderr())
                        .unwrap();

                    match selection {
                        Some(link_index) => match link_index {
                            link_index if link_index < note.links.len() => {
                                open::that(&note.links[link_index]).unwrap();
                            }
                            _ => (),
                        },
                        None => (),
                    }
                }
            },
            None => (),
        }
    }
}

pub fn add_note() {
    let mut current_content = read_json_file();

    let mut new_note: Note = create_blank_note();

    loop {
        let selection = Select::new()
            .items(&[
                color_using_theme("Title"),
                color_using_theme("Content"),
                color_using_theme("Links"),
                color_using_theme("Theme"),
                color_using_theme("Save note"),
                color_using_theme("Go back"),
            ])
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(value) => match value {
                0 => edit_title(&mut new_note),
                1 => edit_content(&mut new_note),
                2 => edit_links(&mut new_note),
                3 => edit_theme(&mut new_note),
                4 => {
                    current_content.notes.push(new_note);
                    write_json_to_file(current_content);
                    break;
                }
                _ => break,
            },
            None => (),
        }
    }
}

pub fn edit_note() {
    println!("editing notes");
}

pub fn delete_note() {
    println!("deleting notes");
}

pub fn change_theme_color() {
    let mut current_content = read_json_file();
    let new_theme_color = choose_color();

    current_content.options.default_theme_color = new_theme_color;

    write_json_to_file(current_content);
}
