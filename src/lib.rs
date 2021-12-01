pub struct GoL {
    map : Vec<Vec<bool>>,
    width: i32,
    height: i32
}

impl GoL {
    // Generate a GoL of the given size with all dead cells
    pub fn new(width: i32, height: i32) -> GoL{
        GoL { 
            map: vec![vec![false; height as usize]; width as usize],
            width: width,
            height: height
        }
    }
    // Returns true if (x,y) is in bounds else returns false
    fn bound_check(&self, x: i32, y: i32) -> bool {
        x < self.width && y < self.height && x >= 0 && y >= 0
    }

    // returns the cell at (x,y) or None if out of bounds
    pub fn get_cell_at(&self, x: i32, y: i32) -> Option<bool> {
        if !self.bound_check(x, y) {
            None
        } else {
            Some(self.map[x as usize][y as usize])
        }
    }

    // sets the cell at (x,y) returns Ok if successful else returns Err
    pub fn set_cell_at(&mut self, x: i32, y: i32, val: bool) -> Result<(), ()> {
        if !self.bound_check(x, y) {
            Err(())
        } else {
            self.map[x as usize][y as usize] = val;

            Ok(())
        }
    }

    // returns the neighbors of (x,y) or None if out of bounds
    pub fn get_neighbors_alive_count(&self, x: i32, y: i32) -> Option<i32> {
        if !self.bound_check(x, y) {
            return None;
        }

        let mut count = 0;

        let (mut h, mut v) = (-1, -1);

        while h < 2 {
            while v < 2 {
                if let Some(val) = self.get_cell_at(x+h, y+v) {
                    if val && !(h == 0 && v == 0) { // skip origin
                        count += 1;
                    }
                }

                v += 1;
            }

            h += 1;
            v = -1;
        }

        Some(count)
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

        // Test get_cell_at() returns none for out of bound input
        assert!(game.get_cell_at(6,6).is_none(),
            "Failed: Getting cell out of bounds did not return none");
    }

    #[test]
    fn get_neighbors_test() {
        let mut game = GoL::new(5, 5);
        
        game.set_cell_at(1, 0, true).unwrap();
        game.set_cell_at(0, 1, true).unwrap();
        game.set_cell_at(2, 0, true).unwrap();
        game.set_cell_at(2, 1, true).unwrap();

        // Test out of bounds
        assert!(game.get_neighbors_alive_count(7,7).is_none(),
            "Failed: Getting cell neighbors out of bound did not return none");

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
