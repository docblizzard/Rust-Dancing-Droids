use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::PartialEq;
use std::fmt::Display;

/// Tableau
struct Dimension {
    x: i32,
    y: i32
}

/// Etat du Robot et position
struct Robot {
    direction : Direction,
    pos_X : i32,
    pos_Y : i32,
}

/// Instruction Donné

enum Instruction {
    L,
    R,
    F
}

/// Possible Directions
#[derive(PartialEq, Debug)]
enum Direction {
    N,
    S,
    W,
    E
}


/// Recupere une direction donné en char et renvoie en Enum Direction
fn read_direction( d : char) -> Direction {
    match d {
        'N' => Direction:: N,
        'S' => Direction:: S,
        'W' => Direction:: W,
        'E' => Direction:: E,
        _ => Direction:: N
    }
}

/// Instruction Donné selon la direction dans la quelle le robot regarde

fn read_instruction(rbt : Robot, charac : char) -> Direction {
    match charac {
        'L' => match rbt.direction {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        },
        'R' => match rbt.direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        },
        _ => rbt.direction
    }
}

impl Robot {
    fn Collision(&self,  other: &Robot) -> bool {
        self.pos_X == other.pos_X || self.pos_Y == other.pos_Y
    }

    fn check_dir(&self) -> bool{
        self.direction == Direction:: S
    }

}


fn main() {
    let mut countchars = 0;
    let mut countlines = 0;
    let mut robot1 = Robot {
        direction: Direction::W,
        pos_Y: 0,
        pos_X: 0
    };
    let mut robot2 = Robot {
        direction: Direction::N,
        pos_Y: 5,
        pos_X: 5
    };
    if robot1.check_dir() == true {
        robot1.pos_Y = robot1.pos_Y + 1;
    }
        /// Lis le fichier text et execute les differentes fonctions selon la ligne

    let mut text = File::open("src/text/two_robots.txt").expect("Open Failed");
    let mut text = BufReader::new(text);
    for line in text.lines().filter_map(|result| result.ok()) {
        for c in line.chars() {
            if countlines == 0 {
                if c == 'N' || c == 'S' || c == 'W' || c == 'E' {
                    robot1.direction = read_direction(c);
                }
          /*  if countlines == 1{
                if c == 'L' || c == 'F' || c == 'R' {
                    fn read_instruction(robot1, c : char);
                }
            } */
            }
        }
        countlines = countlines +1;
    }


    println!("Robots colliding = {}, robot 1 in pos_Y = {:?}", robot1.Collision(&robot2), robot1.direction) ;
}
