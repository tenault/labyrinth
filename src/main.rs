use std::env;
use std::process;

use crate::grid::{Grid, Neighbor};
use crate::rand::{XORShiftRNG, shuffle, rand};
use crate::render::render;

mod grid;
mod rand;
mod render;

fn main() {
    // get size
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { eprintln!("Usage: {} <width> <height>", args[0]); process::exit(1); }

    let width: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => { eprintln!("Error: {} is not a valid width.", args[1]); process::exit(1); }
    };

    let height: usize = match args[2].parse() {
        Ok(num) => num,
        Err(_) => { eprintln!("Error: {} is not a valid height.", args[2]); process::exit(1); }
    };

    if width == 0 || height == 0 { eprintln!("Error: Maze dimensions must be greater than zero."); process::exit(1); }

    // build the grid and initialize
    let mut grid = Grid::build(width, height);
    let mut xsr = XORShiftRNG::new();

    // setup yarn
    let mut yarn = vec![0];
    let mut pos = rand(&grid.cells.len(), &mut xsr);
    grid.visit(&pos);

    // check neighbors, break walls, and retrace
    while yarn.len() > 0 {
        // find potential neighbors
        let mut neighbors = grid.find_neighbors(&pos);
        shuffle(&mut neighbors, &mut xsr);

        // stalk neighbors
        let mut target: Option<Neighbor> = None;
        for neighbor in neighbors {
            if let Some(neighbor) = neighbor && neighbor.cell.visited == false {
                target = Some(neighbor);
                break
            }
        }

        // invade neighbor or retreat
        if let Some(target) = target {
            grid.break_walls(&pos, &target);
            pos = target.cell.id;
            grid.visit(&pos);
            yarn.push(pos);
        } else if let Some(trace) = yarn.pop() { // no more yarn means maze complete
            pos = trace;
        }
    }

    // export maze as string
    println!("{}", render(&grid));
}

