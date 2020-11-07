use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::PartialEq;

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
#[derive(PartialEq)]
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
/*
fn read_instruction(rbt : Robot, dir : char) -> Direction {
    match dir {
        'L' => match rbt.direction {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::N => Direction::E,               A Reparer
            Direction::E => Direction::N,
            _ => Direction:: N,
        },
        'R' => match rbt.direction {
            Direction:: N => Direction:: E,
            Direction:: E => Direction:: S,
            Direction:: S => Direction:: W,
            Direction:: W => Direction:: N,
            _ => Direction:: N,
        },
        'F' => Direction:: N,
        _ => {}
    }
}
*/

impl Robot {
    fn Collision(&self,  other: &Robot) -> bool {
        self.pos_X == other.pos_X || self.pos_Y == other.pos_Y
    }

    fn check_dir(&self) -> bool{
        self.direction == Direction:: S
    }

}

fn main() {
    let mut robot1 = Robot {
        direction : Direction:: S,
        pos_Y : 0,
        pos_X : 0
    };
    let mut robot2 = Robot {
        direction : Direction:: N,
        pos_Y : 5,
        pos_X : 5
    };
    if robot1.check_dir() = true{
        robot1.pos_Y = robot1.pos_Y + 1;
    }

    let f = File::open("src/data/content.txt");


    println!("Robots colliding = {}, robot 1 in pos_Y = {}", robot1.Collision(&robot2), robot1.pos_Y) ;
}
