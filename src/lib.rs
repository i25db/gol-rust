mod display;
mod pos;

pub use pos::{Dimensions, Position};

pub struct GoL {
    map : Vec<Vec<bool>>,
    dims: pos::Dimensions
}

impl GoL {
    // Generate a GoL of the given size with all dead cells
    pub fn new(dims: Dimensions) -> GoL{
        GoL { 
            map: vec![vec![false; dims.height as usize]; dims.width as usize],
            dims: dims
        }
    }

    fn bound_check(&self, pos: Position) -> Result<(), String> {
        if pos.x < self.dims.width && pos.y < self.dims.height {
            Ok(())
        } else {
            Err(String::from("Position out of bounds"))
        }
    }

    // returns the cell at (x,y) or None if out of bounds
    pub fn get_cell_at(&self, pos: Position) -> Result<bool, String> {
        match self.bound_check(pos) {
            Err(msg) => Err(msg),
            Ok(_) => Ok(self.map[pos.x][pos.y])
        }
    }

    // sets the cell at (x,y) returns Ok if successful else returns Err
    pub fn set_cell_at(&mut self, pos: Position, val: bool) -> Result<(), String> {
        match self.bound_check(pos) {
            Err(msg) => Err(msg),
            Ok(_) => {
                self.map[pos.x][pos.y] = val;
                Ok(())
            }
        }
    }

    // returns the neighbors of (x,y) or None if out of bounds
    pub fn get_neighbors_alive_count(&self, pos: Position) -> Result<i32, String> {
        match self.bound_check(pos) {
            Err(msg) => Err(msg),
            Ok(_) => {
                let mut count = 0;

                let (x, y) = (pos.x as i32, pos.y as i32);
                let (mut h, mut v) = (-1, -1);

                while h < 2 {
                    while v < 2 {
                        if (x+h) < 0 || (y+v) < 0 {
                            v += 1;
                            continue;//return Err(String::from("Position out of bounds"));
                        }

                        if let Ok(val) = self.get_cell_at( Position { x: (x+h) as usize, y: (y+v) as usize}) { 
                            if val && !(h == 0 && v == 0) { count += 1 }
                        }
                        
                        v += 1;
                    }

                    h += 1;
                    v = -1;
                }

                Ok(count)
            }
        }
    }

    pub fn simulate_next_step(gol: &GoL) -> Result<GoL, String> {
        let mut result = GoL::new(gol.dims);
        
        for (x, h) in gol.map.iter().enumerate() {
            for (y, _) in h.iter().enumerate() {
                let neighbors = match gol.get_neighbors_alive_count(Position {x, y}) {
                    Ok(n) => n,
                    Err(msg) => return Err(msg)
                };

                let working_cell = match gol.get_cell_at(Position {x, y}) {
                    Ok(val) => val,
                    Err(msg) => return Err(msg)
                };

                if neighbors < 2 {
                    if let Err(msg) = result.set_cell_at(Position {x, y}, false) {
                        return Err(msg);
                    }
                } else if (neighbors == 2 || neighbors == 3) && working_cell {
                    if let Err(msg) = result.set_cell_at(Position {x, y}, true) {
                        return Err(msg);
                    }
                } else if neighbors > 3 {
                    if let Err(msg) = result.set_cell_at(Position {x, y}, false) {
                        return Err(msg);
                    }
                } else if neighbors == 3 {
                    if let Err(msg) = result.set_cell_at(Position {x, y}, true) {
                        return Err(msg);
                    }
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::GoL;
    use super::Dimensions;
    use super::Position;

    #[test]
    fn make_game_test() {
        let game = GoL::new(Dimensions { width: 5, height: 5 });

        assert_eq!(game.dims.width, 5, "Failed: Width not set properly");
        assert_eq!(game.dims.height, 5, "Failed: Height not set properly");

        // Test map initialized to false
        for i in game.map {
            for val in i {
                assert!(!val, "Failed: map not initialized to false");
            }
        }
    }

    #[test]
    fn set_game_test() {
        let mut game = GoL::new(Dimensions { width: 5, height: 5 });

        game.get_cell_at(Position { x: 0, y: 0 }).unwrap();

        // Test set_cell_at() returns ok for valid input
        assert!(game.set_cell_at(Position { x: 0, y: 0 }, true).is_ok(),
            "Failed at set_cell_at().is_ok()");

        // Test set_cell_at() sets the cell
        assert!(game.get_cell_at(Position { x: 0, y: 0 }).unwrap(),
            "Failed: Cell never got set");

        // Test set_call_at() returns err for out of bound input
        assert!(game.set_cell_at(Position { x: 5, y: 5 }, true).is_err(),
            "Failed: Setting cell out of bounds did not return an error");
    }

    #[test]
    fn get_game_test() {
        let mut game = GoL::new(Dimensions {width: 5, height: 5});
        game.set_cell_at(Position {x: 1, y: 1}, true).unwrap();

        // Test get_cell_at() returns a set value
        assert!(game.get_cell_at(Position { x: 1, y: 1 }).unwrap(),
            "Failed at get_cell_at().unwrap()");

        // Test get_cell_at() returns err for out of bound input
        assert!(game.get_cell_at(Position { x: 6, y: 6 }).is_err(),
            "Failed: Getting cell out of bounds did not return err");
    }

    #[test]
    fn get_neighbors_test() {
        let mut game = GoL::new(Dimensions {width: 5, height: 5});
        
        game.set_cell_at(Position {x: 1, y: 0}, true).unwrap();
        game.set_cell_at(Position {x: 0, y: 1}, true).unwrap();
        game.set_cell_at(Position {x: 2, y: 0}, true).unwrap();
        game.set_cell_at(Position {x: 2, y: 1}, true).unwrap();

        // Test out of bounds
        assert!(game.get_neighbors_alive_count(Position {x: 7, y: 7}).is_err(),
            "Failed: Getting cell neighbors out of bound did not return err");

        // Test no neighbors corner
        assert_eq!(0, game.get_neighbors_alive_count(Position { x: 4, y: 4 }).unwrap(),
            "Failed: Found neighbors without any neighbors (corner)");
        // Test no neighbors edge
        assert_eq!(0, game.get_neighbors_alive_count(Position { x: 0, y: 3 }).unwrap(),
            "Failed: Found neighbors without any neighbors (edge)");
        // Test no neighbors center
        assert_eq!(0, game.get_neighbors_alive_count(Position { x: 3, y: 3 }).unwrap(),
            "Failed: Found neighbors without any neighbors (center)");
        
        // Test neighbors corner
        assert_eq!(2, game.get_neighbors_alive_count(Position { x: 0, y: 0 }).unwrap(),
            "Failed: Found incorrect number of neighbors (corner)");
        // Test neighbors edge
        assert_eq!(1, game.get_neighbors_alive_count(Position { x: 0, y: 1 }).unwrap(),
            "Failed: Found incorrect number of neighbors (edge)");
        // Test neighbors center
        assert_eq!(1, game.get_neighbors_alive_count(Position { x: 2, y: 2 }).unwrap(),
            "Failed: Found incorrect number of neighbors (center)");
    }

    #[test]
    fn simulate_next_step_test() {
        let mut gol = GoL::new(Dimensions { width: 3, height: 3 });
        
        // 0 neighbors test
        // 0 0 0    0 0 0
        // 0 1 0 => 0 0 0
        // 0 0 0    0 0 0
        gol.set_cell_at(Position { x: 1, y: 1 }, true).unwrap();
        gol = GoL::simulate_next_step(&gol).unwrap();

        assert!(!gol.get_cell_at(Position { x: 1, y: 1 }).unwrap(), "Failed: 0 neighbors test; didn't kill cell");
        assert_eq!(0, gol.get_neighbors_alive_count(Position { x: 1, y: 1 }).unwrap(),
            "Failed: 0 neighbors test; generated neighbors");

        gol = GoL::new(Dimensions { width: 3, height: 3 });

        // 1 neighbor test
        // 1 0 0    0 0 0
        // 0 1 0 => 0 0 0
        // 0 0 0    0 0 0
        gol.set_cell_at(Position { x: 0, y: 0 }, true).unwrap();
        gol.set_cell_at(Position { x: 1, y: 1 }, true).unwrap();
        
        gol = GoL::simulate_next_step(&gol).unwrap();

        assert!(!gol.get_cell_at(Position { x: 1, y: 1 }).unwrap(), "Failed: 1 neighbor test; didn't kill cell");
        assert!(!gol.get_cell_at(Position { x: 0, y: 0 }).unwrap(), "Failed: 1 neighbor test; didn't kill cell");
       
        assert_eq!(0, gol.get_neighbors_alive_count(Position { x: 1, y: 1 }).unwrap(),
            "Failed: 1 neighbor test; generated neighbors");

        gol = GoL::new(Dimensions { width: 3, height: 3 });

        // 2 neighbor test
        // 1 0 1    0 1 0
        // 0 1 0 => 0 1 0
        // 0 0 0    0 0 0
        gol.set_cell_at(Position { x: 0, y: 0 }, true).unwrap();
        gol.set_cell_at(Position { x: 1, y: 1 }, true).unwrap();
        gol.set_cell_at(Position { x: 2, y: 0 }, true).unwrap();
        
        gol = GoL::simulate_next_step(&gol).unwrap();

        assert!(!gol.get_cell_at(Position { x: 0, y: 0 }).unwrap(), "Failed: 2 neighbors test; didn't kill cell");
        assert!(!gol.get_cell_at(Position { x: 2, y: 0 }).unwrap(), "Failed: 2 neighbors test; didn't kill cell");
        assert!(gol.get_cell_at(Position { x: 1, y: 0 }).unwrap(), "Failed: 2 neighbors test; didn't generate cell");
        assert!(gol.get_cell_at(Position { x: 1, y: 1 }).unwrap(), "Failed: 2 neighbors test; didn't keep cell alive");

        assert_eq!(1, gol.get_neighbors_alive_count(Position { x: 1, y: 1 }).unwrap(),
            "Failed: 2 neighbor test; incorrect neighbors count");

        gol = GoL::new(Dimensions { width: 3, height: 3 });

        // 3 neighbor test
        // 1 0 1    0 0 0
        // 0 0 0 => 0 1 0
        // 0 1 0    0 0 0
        gol.set_cell_at(Position { x: 0, y: 0 }, true).unwrap();
        gol.set_cell_at(Position { x: 2, y: 0 }, true).unwrap();
        gol.set_cell_at(Position { x: 1, y: 2 }, true).unwrap();
        
        gol = GoL::simulate_next_step(&gol).unwrap();

        assert!(gol.get_cell_at(Position { x: 1, y: 1 }).unwrap(), "Failed: 3 neighbors test; didn't generate cell");

        assert_eq!(0, gol.get_neighbors_alive_count(Position { x: 1, y: 1 }).unwrap(),
            "Failed: 3 neighbor test; generated neighbors");

        gol = GoL::new(Dimensions { width: 3, height: 3 });

        // 4 neighbor test
        // 1 0 1    0 1 0
        // 0 1 0 => 1 0 1
        // 1 0 1    0 1 0
        gol.set_cell_at(Position { x: 0, y: 0 }, true).unwrap();
        gol.set_cell_at(Position { x: 2, y: 0 }, true).unwrap();
        gol.set_cell_at(Position { x: 1, y: 1 }, true).unwrap();
        gol.set_cell_at(Position { x: 0, y: 2 }, true).unwrap();
        gol.set_cell_at(Position { x: 2, y: 2 }, true).unwrap();
        
        gol = GoL::simulate_next_step(&gol).unwrap();

        assert!(gol.get_cell_at(Position { x: 1, y: 0 }).unwrap(), "Failed: 4 neighbors test; cell not alive");
        assert!(gol.get_cell_at(Position { x: 0, y: 1 }).unwrap(), "Failed: 4 neighbors test; cell not alive");
        assert!(gol.get_cell_at(Position { x: 2, y: 1 }).unwrap(), "Failed: 4 neighbors test; cell not alive");
        assert!(gol.get_cell_at(Position { x: 1, y: 2 }).unwrap(), "Failed: 4 neighbors test; cell not alive");
        assert!(!gol.get_cell_at(Position { x: 1, y: 1 }).unwrap(), "Failed: 4 neighbors test; center alive");
        
        assert_eq!(4, gol.get_neighbors_alive_count(Position { x: 1, y: 1 }).unwrap(),
            "Failed: 4 neighbor test; incorrect neighbors count");
    }
}
