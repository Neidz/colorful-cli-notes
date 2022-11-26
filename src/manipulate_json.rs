use colored::Colorize;
use console::Term;
use dialoguer::Select;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::fs::{rename, write, File};
use std::io::Read;

#[derive(Deserialize, Serialize)]
pub struct Options {
    pub default_theme_color: String,
}

#[derive(Deserialize, Serialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub links: Vec<String>,
    pub theme_color: String,
}

#[derive(Deserialize, Serialize)]
pub struct JsonFileStructure {
    pub options: Options,
    pub notes: Vec<Note>,
}

fn create_new_json() -> File {
    File::create("notes.json").expect("Couldn't create notes.json file");
    let created_file = File::open("notes.json").expect("Couldn't create and read new file");

    let json_template: JsonFileStructure = JsonFileStructure {
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
            println!("{}", "Unexpected file structure, rename current notes.json file to temp.json and create new file for the notes?\n".red());
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
