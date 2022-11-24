mod actions;
use actions::{add_note, delete_note, edit_note, show_notes};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let items = vec!["Show notes", "New note", "Edit note", "Delete note", "Exit"];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(value) => match value {
                0 => show_notes(),
                1 => add_note(),
                2 => edit_note(),
                3 => delete_note(),
                _ => {
                    println!("bye!");
                    break;
                }
            },
            None => println!("something went wrong"),
        }
    }
}
