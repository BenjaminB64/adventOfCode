use std::collections::HashMap;
use std::hash::{Hash, Hasher};
pub type Coord = (u32, u32);
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Direction::Up => 0.hash(state),
            Direction::Down => 1.hash(state),
            Direction::Left => 2.hash(state),
            Direction::Right => 3.hash(state),
        }
    }
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Direction::Up => Direction::Up,
            Direction::Down => Direction::Down,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
        }
    }
}

impl Eq for Direction {}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Direction::Up => {
                match other {
                    Direction::Up => true,
                    _ => false,
                }
            }
            Direction::Down => {
                match other {
                    Direction::Down => true,
                    _ => false,
                }
            }
            Direction::Left => {
                match other {
                    Direction::Left => true,
                    _ => false,
                }
            }
            Direction::Right => {
                match other {
                    Direction::Right => true,
                    _ => false,
                }
            }
        }
    }
}

// Guardian
#[derive(Debug)]
pub struct Guardian {
    pub coord: Coord,
    pub direction: Direction,
}

impl Guardian {
    pub fn move_forward(&self) -> Coord {
        let mut new_coord = self.coord.clone();
        match self.direction {
            Direction::Up => {
                new_coord.1 -= 1;
            }
            Direction::Down => {
                new_coord.1 += 1;
            }
            Direction::Left => {
                new_coord.0 -= 1;
            }
            Direction::Right => {
                new_coord.0 += 1;
            }
        }
        new_coord
    }
    
    // return true if the guardian is on the edge of the map, false if it's a loop
    pub fn run(&self, coord_obs: &Vec<(u32, u32)>, size_x: u32, size_y: u32) -> (bool, Coord, Vec<Guardian>) {
        let mut guardian_history: Vec<Guardian> = Vec::new();

        let mut g = self.clone();
        loop {
            if guardian_history.contains(&g) {
                return (false, g.coord, guardian_history);
            }

            guardian_history.push(g.clone());
            let new_coord = g.move_forward();

            if coord_obs.contains(&new_coord) {
                g.direction = g.direction.turn_right();
                continue;
            }
            g.coord = new_coord;

            if g.coord.0 == 0 || g.coord.0 == size_x-1 || g.coord.1 == 0 || g.coord.1 == size_y-1 {
                return (true, g.coord, guardian_history);
            }
            //print_map(&coord_obs, &g, limit_x, limit_y);
        }
    }
    pub fn look_for_loops(&self, coord_obs: &Vec<(u32, u32)>, size_x: u32, size_y: u32) -> u32 {
        let mut loops: Vec<Coord> = Vec::new();
        let (_, _, guardian_history) = self.clone().run(&coord_obs, size_x, size_y);


        let mut coords: Vec<Coord> = coord_obs.clone();
        for i in 0..guardian_history.len() {
            if loops.contains(&guardian_history[i].coord) {
                continue;
            }

            coords.push(guardian_history[i].coord);

            let (without_loops, _, _) = self.clone().run(&coords, size_x, size_y);
            if !without_loops {
                loops.push(coords.pop().expect("Error"));
                continue;
            }

            coords.pop();
        }
        loops.len() as u32
    }
}

fn print_map(coord_obs: &Vec<(u32, u32)>, guardian: &Guardian, size_x: u32, size_y: u32, history: Vec<Guardian>) {
    println!("---------------------");
    for y in 0..size_y {
        'x:
        for x in 0..size_x {
            for g in &history {
                if g.coord == (x, y) {
                    match g.direction {
                        Direction::Up => {
                            print!("+");
                        }
                        Direction::Down => {
                            print!("+");
                        }
                        Direction::Left => {
                            print!("-");
                        }
                        Direction::Right => {
                            print!("-");
                        }
                    }
                    continue 'x;
                }
            }
            if coord_obs.contains(&(x, y)) {
                print!("#");
            } else if guardian.coord == (x, y) {
                print!("G");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

impl Clone for Guardian {
    fn clone(&self) -> Self {
        Guardian {
            coord: self.coord.clone(),
            direction: self.direction.clone(),
        }
    }
}

impl Hash for Guardian {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
        self.direction.hash(state);
    }
}

impl PartialEq for Guardian {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.direction == other.direction
    }
}

impl Eq for Guardian {}