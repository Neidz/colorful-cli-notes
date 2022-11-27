use console::Term;
use dialoguer::Select;
use serde_json::{from_str, to_string};
use std::fs::{rename, write, File};
use std::io::Read;

use super::structs::{JsonFileStructure, Options};

pub fn create_new_json() -> File {
    File::create("notes.json").expect("Unable to create notes.json file");
    let created_file = File::open("notes.json").expect("Unable to create and read new file");

    let json_template = JsonFileStructure {
        options: Options {
            default_theme_color: String::from("White"),
        },
        notes: vec![],
    };
    let json_template = to_string(&json_template).unwrap();

    write("notes.json", &json_template).expect("Unable to write to file");

    created_file
}

pub fn read_json_file() -> JsonFileStructure {
    let mut file = match File::open("notes.json") {
        Ok(file) => file,
        Err(_) => create_new_json(),
    };
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let notes_file: JsonFileStructure = match from_str(&mut buff) {
        Ok(json) => json,
        Err(_) => {
            println!("{}", "Unexpected file structure, rename current notes.json file to temp.json and create new file for the notes?\n");
            let selection = Select::new()
                .items(&["Yes", "No"])
                .interact_on_opt(&Term::stderr())
                .unwrap();

            match selection {
                Some(value) => match value {
                    0 => {
                        rename("notes.json", "temp.json").expect("Renaming failed");
                        let mut new_file = create_new_json();

                        let mut new_buff = String::new();
                        new_file.read_to_string(&mut buff).unwrap();
                        let new_notes_file: JsonFileStructure = from_str(&mut new_buff).unwrap();
                        new_notes_file
                    }
                    _ => {
                        panic!("bye!");
                    }
                },
                None => {
                    panic!("Something went wrong")
                }
            }
        }
    };
    notes_file
}

pub fn write_json_to_file(new_json: JsonFileStructure) {
    let new_json_string = to_string(&new_json).unwrap();

    write("notes.json", &new_json_string).expect("Unable to write to file");
}
