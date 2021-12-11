use std::error::Error;
use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;
use gol_rust::app::App;
use gol_rust::start_ui;

// main.rs
fn main() -> Result<(), Box<dyn Error>> {
    let app = Rc::new(RefCell::new(App::new())); // TODO app is useless for now
    start_ui(app)?;
    Ok(())
}

