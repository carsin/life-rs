use rand::random;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        // Initialize the map randomly
        let mut cells = vec![false; width * height];
        for i in 0..cells.len() {
            cells[i] = random(); // Random generates a bool
        }

        Map {
            width,
            height,
            cells,
        }
    }

    pub fn update(&mut self) {
        let mut next_generation = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let pos = self.pos(x, y);
                // Live cell checks
                if self.cells[pos] == true {
                    let next_state = match self.get_cell_live_neighbor_count(x, y) {
                        2 | 3 => true,
                        _ => false,
                    };

                    next_generation[pos] = next_state;
                } else {
                    // Dead cell checks
                    if self.get_cell_live_neighbor_count(x, y) == 3 {
                        next_generation[pos] = true;
                    }
                }
            }
        }

        self.cells = next_generation;
    }

    fn get_cell_live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut neighbor_count: usize = 0;
        //         █         █
        // x-1,y-1 █ x,  y-1 █ x+1,y-1
        //         █         █
        // █████████████████████████████
        //         █         █
        // x-1,y   █ x,  y   █ x+1,y
        //         █         █
        // █████████████████████████████
        //         █         █
        // x-1,y+1 █ x,  y+1 █ x+1,y+1
        //         █         █

        // Top row checks
        if y > 0 {
            // If not on top edge
            if x > 0 {
                // If not on left edge
                if self.cells[self.pos(x - 1, y - 1)] == true {
                    neighbor_count += 1
                } // Top left
            }

            if self.cells[self.pos(x, y - 1)] == true {
                neighbor_count += 1
            } // Top middle

            if x < self.width - 1 {
                // If not on right edge
                if self.cells[self.pos(x + 1, y - 1)] == true {
                    neighbor_count += 1
                } // Top right
            }
        }

        // Middle row checks
        if x > 0 {
            // If not on left edge
            if self.cells[self.pos(x - 1, y)] == true {
                neighbor_count += 1
            } // Middle left
        }

        if x < self.width - 1 {
            // If not on right edge
            if self.cells[self.pos(x + 1, y)] == true {
                neighbor_count += 1
            } // Middle right
        }

        // Bottom row checks
        if y < self.height - 1 {
            // If not on bottom edge
            if x > 0 {
                // If not on left edge
                if self.cells[self.pos(x - 1, y + 1)] == true {
                    neighbor_count += 1
                } // Bottom left
            }

            if self.cells[self.pos(x, y + 1)] == true {
                neighbor_count += 1;
            } // Bottom middle

            if x < self.width - 1 {
                // If not on right edge
                if self.cells[self.pos(x + 1, y + 1)] == true {
                    neighbor_count += 1;
                } // Bottom right
            }
        }

        neighbor_count
    }

    pub fn pos(&self, x: usize, y: usize) -> usize {
        (x * self.width) + y
    }
}