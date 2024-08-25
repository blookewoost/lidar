use std::i32;

use console::Term;

enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Drone {
    x: i32,
    y: i32,
}

impl Drone {
    pub fn new(x: i32, y: i32) -> Drone {
        Drone { x, y }
    }

    fn fly(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }
}


pub fn main_loop(drone: &mut Drone) {
    let stdout = Term::stdout();
    loop {
        match stdout.read_char() {
           Ok('w') => drone.fly(Direction::North),
           Ok('d') => drone.fly(Direction::East),
           Ok('s') => drone.fly(Direction::South),
           Ok('a') => drone.fly(Direction::West),
           _ => break,
        }

        println!("Current drone position: ({},{})", drone.x, drone.y);
    }
}
