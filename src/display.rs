use std::{error::Error, io, sync::mpsc, thread, time::Duration};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};

struct Position { pub x: usize, pub y: usize }
struct Dimensions { pub width: usize, pub height: usize }

pub fn get_viewport_data(v_pos: Position, v_dims: Dimensions, gol: &super::GoL) -> Result<Vec<String>, String> {
    // Perform checks
    
    Err(String::from(""))
}
