use console::Term;
use dialoguer::Select;
use crate::TodoList::ACTION_TYPES;

pub struct App {}

impl App {
    pub fn run() {
        let term = Term::stdout();
        term.write_line("Welcome to your TODO List").unwrap();

        let selection = Select::new()
            .with_prompt("What do you choose?")
            .items(&ACTION_TYPES)
            .interact()
            .unwrap();
        
        println!("Option {}", &ACTION_TYPES[selection]);

    }
}
