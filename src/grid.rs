pub const BIT_N: u8 = 0b0001;
pub const BIT_S: u8 = 0b0010;
pub const BIT_E: u8 = 0b0100;
pub const BIT_W: u8 = 0b1000;

pub enum Direction { N, S, E, W }

#[derive(Clone, Copy)]
pub struct Cell { pub id: usize, pub visited: bool, pub walls: u8 }

pub struct Neighbor { pub direction: Direction, pub cell: Cell }

pub struct Grid { pub width: usize, pub height: usize, pub cells: Vec<Cell> }
impl Grid {
    pub fn build(width: usize, height: usize) -> Self {
        let size = width * height;

        let mut cells: Vec<Cell> = Vec::new();
        for i in 0..size { cells.push(Cell { id: i, visited: false, walls: 0b1111 }); } // W, E, S, N

        Self { width, height, cells }
    }

    pub fn visit(&mut self, pos: &usize) { self.cells[*pos].visited = true; }

    pub fn find_neighbors(&self, pos: &usize) -> [Option<Neighbor>; 4] {
        let mut neighbors: [Option<Neighbor>; 4] = [None, None, None, None];

        if *pos > self.width - 1                { neighbors[0] = Some(Neighbor { direction: Direction::N, cell: self.cells[*pos - self.width] }) }
        if *pos < self.cells.len() - self.width { neighbors[1] = Some(Neighbor { direction: Direction::S, cell: self.cells[*pos + self.width] }) }
        if *pos %  self.width < self.width - 1  { neighbors[2] = Some(Neighbor { direction: Direction::E, cell: self.cells[*pos + 1]          }) }
        if *pos %  self.width > 0               { neighbors[3] = Some(Neighbor { direction: Direction::W, cell: self.cells[*pos - 1]          }) }

        return neighbors
    }

    pub fn break_walls(&mut self, pos: &usize, target: &Neighbor) {
        match target.direction {
            Direction::N => {
                self.cells[*pos].walls           ^= BIT_N;
                self.cells[target.cell.id].walls ^= BIT_S;
            },
            Direction::S => {
                self.cells[*pos].walls           ^= BIT_S;
                self.cells[target.cell.id].walls ^= BIT_N;
            },
            Direction::E => {
                self.cells[*pos].walls           ^= BIT_E;
                self.cells[target.cell.id].walls ^= BIT_W;
            },
            Direction::W => {
                self.cells[*pos].walls           ^= BIT_W;
                self.cells[target.cell.id].walls ^= BIT_E;
            }
        }
    }
}
