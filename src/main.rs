mod notes;
use console::Term;
use dialoguer::Select;
use notes::actions::{add_note, delete_note, edit_note, options, show_notes};
use notes::manipulate_json::read_json_file;

fn main() {
    read_json_file();

    let items = vec![
        "Show notes",
        "New note",
        "Edit note",
        "Delete note",
        "Options",
        "Exit",
    ];

    loop {
        let selection = Select::new()
            .items(&items)
            .interact_on_opt(&Term::stderr())
            .unwrap();

        match selection {
            Some(value) => match value {
                0 => show_notes(),
                1 => add_note(),
                2 => edit_note(),
                3 => delete_note(),
                4 => options(),
                _ => {
                    println!("bye!");
                    break;
                }
            },
            None => println!("something went wrong"),
        }
    }
}
