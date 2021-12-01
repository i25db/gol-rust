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

    pub fn get_cell_at(&self, x: i32, y: i32) -> Option<bool> {
        if x > self.width - 1 || y > self.height - 1 {
            None
        } else {
            Some(self.map[x as usize][y as usize])
        }
    }

    pub fn set_cell_at(&mut self, x: i32, y: i32, val: bool) -> Result<(), ()> {
        if x > self.width - 1 || y > self.height - 1 {
            Err(())
        } else {
            self.map[x as usize][y as usize] = val;

            Ok(())
        }
    }

    pub fn get_neighbors_alive_count(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;

        let (mut h, mut v) = (-1, -1);

        while h < 2 {
            while v < 2 {
                if let Some(val) = self.get_cell_at(x, y) {
                    if val { count += 1 }
                }
                v += 1;
            }
            h += 1;
        }

        count
    }
}
