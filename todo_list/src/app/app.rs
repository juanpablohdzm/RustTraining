use crate::database::{DataBase, Item};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};
use uuid::Uuid;
use crate::app::InputMode;

/// App holds the state of the application.
pub struct App {
    /// Current value of the input box.
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// Current input mode.
    input_mode: InputMode,
    /// Selected todo index.
    selected: usize,
    /// Database.
    db: DataBase,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            character_index: 0,
            selected: 0,
            db: DataBase::new("todos.db").unwrap(), // Initialize the database
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        if self.character_index != 0 {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    /// When the user confirms an edit, the input is recorded as a new todo.
    fn submit_todo(&mut self) {
        self.db.add_item(Item::new(0, self.input.clone())).expect("Failed to add item");
        self.input.clear();
        self.reset_cursor();
        self.selected = self.db.get_items_length().saturating_sub(1);
    }

    /// Modify the selected todo.
    fn modify_selected(&mut self) {
        
        let item  = self.db.get_item_by_index(self.selected);
        if let Some(item) = item {
            let item = item.clone();
            
            let _ = self.db.remove_item(item.get_id());
            self.input = item.get_name().to_string();
            self.character_index = self.input.len();
            self.input_mode = InputMode::Editing;
            
            let items_length = self.db.get_items_length();
            if self.selected >= items_length && items_length > 0 {
                self.selected = items_length - 1;
            }
        }
    }

    /// Remove the selected todo.
    fn remove_selected(&mut self) {
        let item = self.db.get_item_by_index(self.selected);
        if let Some(item) = item {
            let item = item.clone();
            let _ = self.db.remove_item(item.get_id());
            self.selected = self.db.get_items_length().saturating_sub(1);
        }
    }

    /// Main event loop for the application.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        // 'a' starts adding a new todo.
                        KeyCode::Char('a') => {
                            self.input.clear();
                            self.reset_cursor();
                            self.input_mode = InputMode::Editing;
                        }
                        // 'm' modifies the selected todo.
                        KeyCode::Char('m') => {
                            self.modify_selected();
                        }
                        // 'r' removes the selected todo.
                        KeyCode::Char('r') => {
                            self.remove_selected();
                        }
                        // Up arrow moves the selection up.
                        KeyCode::Up => {
                            let items_length = self.db.get_items_length();
                            if items_length > 0 {
                                self.selected = self.selected.saturating_sub(1);
                            }
                        }
                        // Down arrow moves the selection down.
                        KeyCode::Down => {
                            let items_length = self.db.get_items_length();
                            if items_length > 0  {
                                self.selected = (self.selected + 1).min(items_length - 1);
                            }
                        }
                        // 'q' quits the application.
                        KeyCode::Char('q') => {
                            break;
                        }
                        _ => {}
                    },
                    // In editing mode, allow typing and editing the input.
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_todo(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        // Esc cancels editing and returns to normal mode.
                        KeyCode::Esc => {
                            self.input.clear();
                            self.reset_cursor();
                            self.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
        ratatui::restore();
        Ok(())
    }

    /// Draw the UI including the menu, input area, and todo list.
    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(1),
        ]);
        let [menu_area, input_area, todos_area] = vertical.areas(frame.area());

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "a".bold(),
                    " to add, ".into(),
                    "m".bold(),
                    " to modify, ".into(),
                    "r".bold(),
                    " to remove, ".into(),
                    "q".bold(),
                    " to quit. Use up/down arrows to select an item.".into(),
                ],
                Style::default(),
            ),
            InputMode::Editing => (
                vec![
                    "Editing: type your todo and press ".into(),
                    "Enter".bold(),
                    " to confirm or ".into(),
                    "Esc".bold(),
                    " to cancel.".into(),
                ],
                Style::default().fg(Color::Yellow),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let menu = Paragraph::new(text).block(Block::bordered().title("Menu"));
        frame.render_widget(menu, menu_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        let todos: Vec<ListItem> = self.db.get_items().unwrap().iter().enumerate().map(|(i, todo)| {
            // Highlight the selected todo.
            let style = if i == self.selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let content = Line::from(Span::styled(format!("{}: {}", i + 1, todo), style));
            ListItem::new(content)
        }).collect();
        let todos_list = List::new(todos).block(Block::bordered().title("Todo List"));
        frame.render_widget(todos_list, todos_area);
    }
}
