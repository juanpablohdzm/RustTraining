use std::fmt::{Display, Formatter};

pub const ACTION_TYPES: [TodoCommand; 4] = [TodoCommand::Add, TodoCommand::Remove, TodoCommand::Load, TodoCommand::Save];
pub enum TodoCommand {
    Add,
    Remove,
    Save,
    Load,
}

impl Display for TodoCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            TodoCommand::Add => "Add",
            TodoCommand::Remove => "Remove",
            TodoCommand::Save => "Save",
            TodoCommand::Load => "Load",
        };
        write!(f, "{}", message)
    }
}
