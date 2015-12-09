pub struct Grid {
    pub grid: [[usize;1000];1000]
}

impl Grid {
    /// +1 brightness to all lights from x,y through x,y in the grid.
    pub fn turn_on(&mut self, start: [usize;2], stop: [usize;2]) {
        for x in start[0]..stop[0]+1 {
            for y in start[1]..stop[1]+1 {
                self.grid[x][y] += 1;
            }
        }
    }

    /// -1 brightness to all lights from x,y through x,y in the grid.
    pub fn turn_off(&mut self, start: [usize;2], stop: [usize;2]) {
        for x in start[0]..stop[0]+1 {
            for y in start[1]..stop[1]+1 {
                if self.grid[x][y] != 0 { self.grid[x][y] -= 1; }
            }
        }
    }

    /// +2 brightness to all lights from x,y through x,y in the grid.
    pub fn toggle(&mut self, start: [usize;2], stop: [usize;2]) {
        for x in start[0]..stop[0]+1 {
            for y in start[1]..stop[1]+1 {
                self.grid[x][y] += 2;
            }
        }
    }

    /// Counts the max brightness of all lights in the grid.
    pub fn turned_on_count(&self) -> usize {
        self.grid.iter().map(
            |column| column.iter().fold(0, |total, next| total + next)
        ).fold(0, |total, next| total + next)
    }
}
