pub struct Grid {
    pub grid: [[bool;1000];1000]
}

impl Grid {
    /// Turns on all lights from x,y through x,y in the grid.
    pub fn turn_on(&mut self, start: [usize;2], stop: [usize;2]) {
        for x in start[0]..stop[0]+1 {
            for y in start[1]..stop[1]+1 {
                self.grid[x][y] = true;
            }
        }
    }

    /// Turns off all lights from x,y through x,y in the grid.
    pub fn turn_off(&mut self, start: [usize;2], stop: [usize;2]) {
        for x in start[0]..stop[0]+1 {
            for y in start[1]..stop[1]+1 {
                self.grid[x][y] = false;
            }
        }
    }

    /// Toggles lights from x,y through x,y in the grid so that lights that were on are off, and
    /// lights that were off are now on.
    pub fn toggle(&mut self, start: [usize;2], stop: [usize;2]) {
        for x in start[0]..stop[0]+1 {
            for y in start[1]..stop[1]+1 {
                self.grid[x][y] = !self.grid[x][y];
            }
        }
    }

    /// Counts the total number of lights turned on in the grid.
    pub fn turned_on_count(&self) -> usize {
        self.grid.iter().map(|column| column.iter().filter(|&x| *x == true).count())
            .fold(0, |total, next| total + next)
    }
}
