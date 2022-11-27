use console::Term;
use dialoguer::Select;

use super::{
    manipulate_json::{read_json_file, write_json_to_file},
    note_utils::{create_blank_note, edit_content, edit_links, edit_theme, edit_title},
    structs::Note,
};

pub fn show_notes() {
    println!("showing notes");
}

pub fn add_note() {
    let mut current_content = read_json_file();

    let mut new_note: Note = create_blank_note();

    loop {
        let selection = Select::new()
            .items(&["Title", "Content", "Links", "Theme", "Save note"])
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
                _ => (),
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

pub fn options() {
    println!("options");
}
