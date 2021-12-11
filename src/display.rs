use std::{error::Error, io, time::Duration, io::Stdout};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

use super::{Position, Dimensions, GoL};

pub fn setup_terminal() -> Result<Terminal<TermionBackend<RawTerminal<Stdout>>>, Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal) 
}

pub fn run_gol<B: Backend>(
    term: &mut Terminal<B>, 
    gol: &mut GoL,
    _tick_rate: Duration
    ) -> Result<(), Box<dyn Error>> {
    
    term.clear()?;
    loop {
        term.draw(|f| ui(f, &gol))?;
        for key in io::stdin().keys() {
            match key? {
                Key::Char('q') => {
                    term.clear()?;
                    return Ok(());
                },
                _ => {}
            }
            
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, gol: &GoL) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(size);

    // GoL block
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::from("Conway's Game of Life"))
        .border_type(BorderType::Rounded);

    let paragraph = Paragraph::new(get_viewport_data(
            Position { x: 0, y: 0 },
            Dimensions { width: chunks[0].width as usize, height: chunks[1].height as usize },
            &gol)
            .expect("Failed to draw GOL").join("\n"))
        .block(block);
    f.render_widget(paragraph, chunks[0]);

    // Commands block
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::from("Commands"))
        .border_type(BorderType::Rounded);
    f.render_widget(block, chunks[1]);
}

pub fn get_viewport_data(v_pos: Position, v_dims: Dimensions, gol: &GoL) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    
    // If v_dims.y > gol.dims.y prepend lines equal to (v_dims.y - gol.dims.y) / 2
    // If v_dims.x > gold.dims.x each line gets a left padding equal to (v_dims.x - gol.dims.y) / 2
    if v_dims.height > gol.dims.height {
        let diff = v_dims.height - gol.dims.height;
        for _ in 0..(diff/2) {
            result.push(String::from("\n"));
        }
    }
    
    let mut padding = String::new();
    if v_dims.width > gol.dims.width {
        let diff = v_dims.width - gol.dims.width;
        for _ in 0..(diff/2) {
            padding.push(' ');
        }
    }

    for y in 0..v_dims.height {
        // TODO: Implement smaller viewport
        let mut row = padding.clone();
        for x in 0..v_dims.width {
            if v_pos.x + x >= gol.dims.width {
                row = row + padding.clone().as_ref();
                break;
            }

            if v_pos.y + y >= gol.dims.height {
                if v_dims.height > gol.dims.height {
                    let diff = v_dims.height - gol.dims.height;
                    for _ in 0..(diff/2) {
                        result.push(String::from("\n"));
                    }
                }

                return Ok(result);
            }

            if let Ok(val) = gol.get_cell_at(Position { x: v_pos.x+x, y: v_pos.y+y }) {
                if val { row.push('1') }
                else { row.push(' ') }
            }
        }
        
        result.push(row);
    }

    

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::super::{GoL, Position, Dimensions};

    #[test]
    fn viewport_test() {
        let mut gol = GoL::new(Dimensions { width: 5, height: 5 });
        gol.set_cell_at(Position { x: 1, y: 1 }, true).unwrap();
        gol.set_cell_at(Position { x: 3, y: 2 }, true).unwrap();
        gol.set_cell_at(Position { x: 3, y: 3 }, true).unwrap();

        // Test get_viewport_data returns the correct viewport
        let viewport =  super::get_viewport_data(
            Position { x: 0, y: 0 }, 
            Dimensions { width: 5, height: 5 }, &gol).unwrap();
        assert!(viewport[0] == String::from("     "), "Failed: First viewport row not set properly");
        assert!(viewport[1] == String::from(" 1   "), "Failed: Second viewport row not set properly");
        assert!(viewport[2] == String::from("   1 "), "Failed: Third viewport row not set properly");
        assert!(viewport[3] == String::from("   1 "), "Failed: Fourth viewport row not set properly");
        assert!(viewport[4] == String::from("     "), "Failed: Fifth viewport row not set properly");

        // 1 1
        // 1 1
        // 1 1
        let mut gol = GoL::new(Dimensions { width: 3, height: 3 });
        gol.set_cell_at(Position { x: 0, y: 0 }, true).unwrap();
        gol.set_cell_at(Position { x: 2, y: 0 }, true).unwrap();
        gol.set_cell_at(Position { x: 0, y: 1 }, true).unwrap();
        gol.set_cell_at(Position { x: 2, y: 1 }, true).unwrap();
        gol.set_cell_at(Position { x: 0, y: 2 }, true).unwrap();
        gol.set_cell_at(Position { x: 2, y: 2 }, true).unwrap();
        
        // Test gol width smaller than viewport
        let viewport = super::get_viewport_data(
        Position { x: 0, y: 0 }, 
        Dimensions { width: 5, height: 3 }, &gol).unwrap();

        assert!(viewport[0] == String::from(" 1 1 "), "Failed: First viewport row not padded properly");
        assert!(viewport[1] == String::from(" 1 1 "), "Failed: Second viewport row not padded properly");
        assert!(viewport[2] == String::from(" 1 1 "), "Failed: Third viewport row not padded properly");

        // Test gol height smaller than viewport
        let viewport = super::get_viewport_data(
        Position { x: 0, y: 0 }, 
        Dimensions { width: 3, height: 5 }, &gol).unwrap();

        assert!(viewport[0] == String::from("\n"), "Failed: First viewport row not set to newline");
        assert!(viewport[4] == String::from("\n"), "Failed: Fifth viewport row not set to newline");
    }
}
