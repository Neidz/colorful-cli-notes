mod notes;
use console::Term;
use dialoguer::Select;
use notes::actions::{add_note, change_theme_color, delete_note, edit_note, show_notes};
use notes::color_utils::color_using_theme;

fn main() {
    loop {
        let items = vec![
            color_using_theme("Show notes"),
            color_using_theme("New note"),
            color_using_theme("Edit note"),
            color_using_theme("Delete note"),
            color_using_theme("Change theme color"),
            color_using_theme("Exit"),
        ];

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
                4 => change_theme_color(),
                _ => {
                    println!("{}", color_using_theme("bye!"));
                    break;
                }
            },
            None => println!("something went wrong"),
        }
    }
}
