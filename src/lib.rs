pub struct GoL {
    map : Vec<Vec<bool>>,
    width: usize,
    height: usize
}

impl GoL {
    // Generate a GoL of the given size with all dead cells
    pub fn new(width: usize, height: usize) -> GoL{
        GoL { 
            map: vec![vec![false; height as usize]; width as usize],
            width: width,
            height: height
        }
    }

    fn bound_check(&self, x: usize, y: usize) -> Result<(), String> {
        if x < self.width && y < self.height && x >= 0 && y >= 0 {
            Ok(())
        } else {
            Err(String::from("Position out of bounds"))
        }
    }

    // returns the cell at (x,y) or None if out of bounds
    pub fn get_cell_at(&self, x: usize, y: usize) -> Result<bool, String> {
        match self.bound_check(x, y) {
            Err(msg) => Err(msg),
            Ok(_) => Ok(self.map[x][y])
        }
    }

    // sets the cell at (x,y) returns Ok if successful else returns Err
    pub fn set_cell_at(&mut self, x: usize, y: usize, val: bool) -> Result<(), String> {
        match self.bound_check(x, y) {
            Err(msg) => Err(msg),
            Ok(_) => {
                self.map[x][y] = val;
                Ok(())
            }
        }
    }

    // returns the neighbors of (x,y) or None if out of bounds
    pub fn get_neighbors_alive_count(&self, x: usize, y: usize) -> Result<i32, String> {
        match self.bound_check(x, y) {
            Err(msg) => Err(msg),
            Ok(_) => {
                let mut count = 0;

                let (x, y) = (x as i32, y as i32);
                let (mut h, mut v) = (-1, -1);

                while h < 2 {
                    while v < 2 {
                        if (x+h) < 0 || (y+v) < 0 {
                            return Err(String::from("Position out of bounds"));
                        }

                        match self.get_cell_at((x+h) as usize, (y+v) as usize) {
                            Err(msg) => return Err(msg),
                            Ok(val) => {
                                if val && !(h == 0 && v == 0) { count += 1 }
                            }
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

    pub fn simulate_next_step(gol: GoL) -> Result<GoL, String> {
        let mut result = GoL::new(gol.width, gol.height);
        
        for (x, h) in gol.map.iter().enumerate() {
            for (y, _) in h.iter().enumerate() {
                let neighbors = match gol.get_neighbors_alive_count(x,y) {
                    Ok(n) => n,
                    Err(msg) => return Err(msg)
                };

                let working_cell = match gol.get_cell_at(x,y) {
                    Ok(val) => val,
                    Err(msg) => return Err(msg)
                }

                if neighbors < 2 {
                    if let Err(msg) = result.set_cell_at(x,y, false) {
                        return Err(msg);
                    }
                } else if (neighbors == 2 || neighbors == 3) && working_cell {
                    if let Err(msg) = result.set_cell_at(x,y, true) {
                        return Err(msg);
                    }
                } else if neighbors > 3 {
                    if let Err(msg) = result.set_cell_at(x,y, false) {
                        return Err(msg);
                    }
                } else if neighbors == 3 {
                    if let Err(msg) = result.set_cell_at(x,y, true) {
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

    #[test]
    fn make_game_test() {
        let game = GoL::new(5, 5);

        assert_eq!(game.width, 5, "Failed: Width not set properly");
        assert_eq!(game.height, 5, "Failed: Height not set properly");

        // Test map initialized to false
        for i in game.map {
            for val in i {
                assert!(!val, "Failed: map not initialized to false");
            }
        }
    }

    #[test]
    fn set_game_test() {
        let mut game = GoL::new(5, 5);

        game.get_cell_at(0, 0).unwrap();

        // Test set_cell_at() returns ok for valid input
        assert!(game.set_cell_at(0, 0, true).is_ok(),
            "Failed at set_cell_at().is_ok()");

        // Test set_cell_at() sets the cell
        assert!(game.get_cell_at(0, 0).unwrap(),
            "Failed: Cell never got set");

        // Test set_call_at() returns err for out of bound input
        assert!(game.set_cell_at(5, 5, true).is_err(),
            "Failed: Setting cell out of bounds did not return an error");
    }

    #[test]
    fn get_game_test() {
        let mut game = GoL::new(5, 5);
        game.set_cell_at(1, 1, true).unwrap();

        // Test get_cell_at() returns a set value
        assert!(game.get_cell_at(1,1).unwrap(),
            "Failed at get_cell_at().unwrap()");

        // Test get_cell_at() returns err for out of bound input
        assert!(game.get_cell_at(6,6).is_err(),
            "Failed: Getting cell out of bounds did not return err");
    }

    #[test]
    fn get_neighbors_test() {
        let mut game = GoL::new(5, 5);
        
        game.set_cell_at(1, 0, true).unwrap();
        game.set_cell_at(0, 1, true).unwrap();
        game.set_cell_at(2, 0, true).unwrap();
        game.set_cell_at(2, 1, true).unwrap();

        // Test out of bounds
        assert!(game.get_neighbors_alive_count(7,7).is_err(),
            "Failed: Getting cell neighbors out of bound did not return err");

        // Test no neighbors corner
        assert_eq!(0, game.get_neighbors_alive_count(4,4).unwrap(),
            "Failed: Found neighbors without any neighbors (corner)");
        // Test no neighbors edge
        assert_eq!(0, game.get_neighbors_alive_count(0,3).unwrap(),
            "Failed: Found neighbors without any neighbors (edge)");
        // Test no neighbors center
        assert_eq!(0, game.get_neighbors_alive_count(3,3).unwrap(),
            "Failed: Found neighbors without any neighbors (center)");
        
        // Test neighbors corner
        assert_eq!(2, game.get_neighbors_alive_count(0,0).unwrap(),
            "Failed: Found incorrect number of neighbors (corner)");
        // Test neighbors edge
        assert_eq!(1, game.get_neighbors_alive_count(0,1).unwrap(),
            "Failed: Found incorrect number of neighbors (edge)");
        // Test neighbors center
        assert_eq!(1, game.get_neighbors_alive_count(2,2).unwrap(),
            "Failed: Found incorrect number of neighbors (center)");
    }
}
