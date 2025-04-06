mod app;
mod database;

use color_eyre::Result;
use crate::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    App::new().run(terminal)
}