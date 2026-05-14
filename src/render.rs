use crate::grid::{Grid, BIT_N, BIT_S, BIT_E, BIT_W};

pub fn render(grid: &Grid) -> String {
    let cross_width  = &grid.width  - 1;
    let cross_height = &grid.height - 1;
    let cross_size   = cross_width * cross_height;

    let mut maze = String::with_capacity(&grid.width * &grid.height * 2);

    // top border
    maze.push_str("┏━");
    for i in 0..cross_width {
        let ll = &grid.cells[i];
        let lr = &grid.cells[i + 1];

        let descender = (ll.walls & BIT_E) != 0 || (lr.walls & BIT_W) != 0;

        maze.push_str(match descender {
            true  => "┯━",
            false => "━━"
        });
    }
    maze.push_str("┓");
    
    // maze core
    for i in 0..cross_size {
        // decode index
        let x = i % cross_width;
        let y = i / cross_width;

        // get corners
        let ul = &grid.cells[x     +  y      * &grid.width];
        let ur = &grid.cells[x + 1 +  y      * &grid.width];
        let ll = &grid.cells[x     + (y + 1) * &grid.width];
        let lr = &grid.cells[x + 1 + (y + 1) * &grid.width];

        // left border
        if i % cross_width == 0 {
            let line = (ul.walls & BIT_S) != 0 || (ll.walls & BIT_N) != 0;
            maze.push_str(match line {
                true  => "\n┠─",
                false => "\n┃ "
            });
        }

        // rebuild walls
        let mut cross: u8 = 0;
        if (ul.walls & BIT_E) != 0 || (ur.walls & BIT_W) != 0 { cross ^= BIT_N }
        if (ll.walls & BIT_E) != 0 || (lr.walls & BIT_W) != 0 { cross ^= BIT_S }
        if (ur.walls & BIT_S) != 0 || (lr.walls & BIT_N) != 0 { cross ^= BIT_E }
        if (ul.walls & BIT_S) != 0 || (ll.walls & BIT_N) != 0 { cross ^= BIT_W }

        // select glyph
        maze.push_str(match cross {
            0b0000 => "  ",
            0b0001 => "╵ ",
            0b0010 => "╷ ",
            0b0011 => "│ ",
            0b0100 => "╶─",
            0b0101 => "└─",
            0b0110 => "┌─",
            0b0111 => "├─",
            0b1000 => "╴ ",
            0b1001 => "┘ ",
            0b1010 => "┐ ",
            0b1011 => "┤ ",
            0b1100 => "──",
            0b1101 => "┴─",
            0b1110 => "┬─",
            0b1111 => "┼─",
            _ => "??"
        });

        // right border
        if i % cross_width == cross_width - 1 {
            let line = (ur.walls & BIT_S) != 0 || (lr.walls & BIT_N) != 0;
            maze.push(match line {
                true  => '┨',
                false => '┃'
            });
        }
    }


    // bottom border
    maze.push_str("\n┗━");
    for i in 0..cross_width {
        let ul = &grid.cells[i     + &grid.cells.len() - &grid.width];
        let ur = &grid.cells[i + 1 + &grid.cells.len() - &grid.width];

        let ascender = (ul.walls & BIT_E) != 0 || (ur.walls & BIT_W) != 0;

        maze.push_str(match ascender {
            true  => "┷━",
            false => "━━"
        });
    }
    maze.push('┛');

    return maze;
}
