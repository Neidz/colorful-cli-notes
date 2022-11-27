use console::Term;
use dialoguer::Select;
use std::io;

use super::structs::Note;

pub fn create_blank_note() -> Note {
    let blank_note = Note {
        title: String::from(""),
        content: String::from(""),
        links: vec![],
        theme: String::from("White"),
    };

    blank_note
}

pub fn edit_title(note: &mut Note) {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => note.title = input.trim().to_string(),
        Err(_) => (),
    };
}

pub fn edit_content(note: &mut Note) {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => note.content = input.trim().to_string(),
        Err(_) => (),
    };
}

pub fn edit_links(note: &mut Note) {
    loop {
        let mut current_list_of_links = note.links.clone();
        let mut link_options = vec![String::from("New link"), String::from("Back")];
        current_list_of_links.append(&mut link_options);

        let new_link_index = current_list_of_links.len() - 2;

        let selection = Select::new()
            .items(&current_list_of_links)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(value) => match value {
                index if index == new_link_index => {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => note.links.push(input.trim().to_string()),
                        Err(_) => (),
                    };
                }
                index if index < new_link_index => {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            note.links[index] = input.trim().to_string();
                        }
                        Err(_) => (),
                    };
                }
                _ => break,
            },
            None => (),
        }
    }
}

pub fn edit_theme(note: &mut Note) {
    // let colors: [Styles; 8] = []

    // let selection = Select::new()
    //         .items(&current_list_of_links)
    //         .interact_on_opt(&Term::stderr())
    //         .unwrap();

    //     match selection {
    //         Some(value) => match value {}
}
