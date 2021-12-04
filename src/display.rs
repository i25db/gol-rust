//use std::{error::Error, io, sync::mpsc, thread, time::Duration, iter::Iterator};
//use termion::{
//    event::Key,
//    input::{MouseTerminal, TermRead},
//    raw::IntoRawMode,
//    screen::AlternateScreen,
//};
//use tui::{
//    backend::{Backend, TermionBackend},
//    Terminal,
//};

use super::{Position, Dimensions, GoL};

pub fn get_viewport_data(v_pos: Position, v_dims: Dimensions, gol: &GoL) -> Result<Vec<String>, String> {
    // Perform checks
    let size = Dimensions { width: v_dims.width + v_pos.x, height: v_dims.height + v_pos.y };
    if size.width > gol.dims.width || size.height > gol.dims.height {
        return Err(String::from("Viewport out of bounds"));
    }
    
    let mut result = Vec::new();
    let (mut x, mut y) = (0 as usize, 0 as usize);
    
    while y < v_dims.height {
        // TODO: Implement smaller viewport
        let mut row = String::new();
        while x < v_dims.width {
            if let Ok(val) = gol.get_cell_at(Position { x: v_pos.x+x, y: v_pos.y+y }) {
                if val { row = row + "1" }
                else { row = row + " " }
            }

            x += 1;
        }
        
        result.push(row);
        y += 1;
        x = 0;
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

        // Test get_viewport_data properly returns an error
        assert!(super::get_viewport_data(Position { x: 5, y: 5 }, Dimensions { width: 5, height: 5 }, &gol).is_err(),
            "Failed: Viewport outside of bounds did not return an error");

        // Test get_viewport_data returns the correct viewport
        let viewport = super::get_viewport_data(
            Position { x: 0, y: 0 }, 
            Dimensions { width: 5, height: 5 }, &gol).unwrap();
        assert!(viewport[0] == String::from("     "), "Failed: First viewport row not set properly");
        assert!(viewport[1] == String::from(" 1   "), "Failed: Second viewport row not set properly");
        assert!(viewport[2] == String::from("   1 "), "Failed: Third viewport row not set properly");
        assert!(viewport[3] == String::from("   1 "), "Failed: Fourth viewport row not set properly");
        assert!(viewport[4] == String::from("     "), "Failed: Fifth viewport row not set properly");
    }
}
