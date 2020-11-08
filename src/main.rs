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
/*
enum Instruction {
    L,
    R,
    F
}
*/
/// Possible Directions
#[derive(PartialEq, Debug)] /// Permet de comparer un enum Direction
enum Direction {
    N,
    S,
    W,
    E
}


/// Recupere une direction donné en char et renvoie en Enum Direction
fn read_direction( d : &char) -> Direction {
    match d {
        'N' => Direction:: N,
        'S' => Direction:: S,
        'W' => Direction:: W,
        'E' => Direction:: E,
        _ => Direction:: N
    }
}
/// Check Collision avant d'avancer
/*

/// Test Limite du tableau

fn check_limit(posX: i32,posY:i32, rbt: Robot){
    let posX = &rbt.pos_X;
    let posY = &rbt.pos_Y;
    match rbt.direction {
        Direction::N => match rbt.pos_Y {
            0 => posY,
            _ => posY = &posY +1,
        },
        Direction::E => match rbt.pos_X {
            5 => posX,
            _ => posX = posX + 1,
        },
        Direction::S => match rbt.pos_Y {
            5 => posY,
            _ => posY = posY+ 1,
        },
        _ => match rbt.pos_X {
            0 => posX,
            _ => posX = posX + 1,
        }
    }
}
*/
/// Instruction Donné selon la direction dans la quelle le robot regarde

fn read_instruction(rbt : &Robot, chr : &char) -> Direction {
    match chr {
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
        _    => match rbt.direction {
            Direction::E => Direction::E,
            Direction::S => Direction::S,
            Direction::W => Direction::W,
            Direction::N => Direction::N,
        }
    }
}

impl Robot {
    fn collision(&self,  &posX: &i32, &posY: &i32) -> bool {
            self.pos_X == posX && self.pos_Y == posY
        }
}

    /// Lis le fichier text et execute les differentes fonctions selon la ligne du fichier texte
fn main() {
    let mut countchars = 0;
    let mut countlines = 0;

    let mut robot1 = Robot {
        direction: Direction::S,
        pos_Y: 3,
        pos_X: 3
    };
    let mut robot2 = Robot {
        direction: Direction::N,
        pos_Y: 2,
        pos_X: 2
    };


    let  text = File::open("src/text/two_robots.txt").expect("Open Failed");
    let  text = BufReader::new(text);
    for line in text.lines().filter_map(|result| result.ok()) {             // Récupéré sur OS
        for c in line.chars() {
            if countlines == 0 {
                if c == 'N' || c == 'S' || c == 'W' || c == 'E' {
                    robot1.direction = read_direction(&c);
                }
            }
            if countlines == 1{
                if c == 'L' || c == 'F' || c == 'R' {
                    robot1.direction = read_instruction(&robot1, &c );
                    if c == 'F' {
                        if robot1.direction == Direction::S {
                            if robot2.collision(&(&robot1.pos_X), &(&robot1.pos_Y +1)  ) == true {
                                println!("Collision entre les robots en x = {}, y = {}", (&robot1.pos_X +1), &robot1.pos_X)
                            }
                            else {
                                robot1.pos_Y = &robot1.pos_Y + 1;
                                println!("Robot 1 est maintenant en x = {}, y = {}", &robot1.pos_X, &robot1.pos_Y);
                            }
                        }
                    }
                }
            }

            if countlines == 3 {
                if c == 'N' || c == 'S' || c == 'W' || c == 'E' {
                    robot2.direction = read_direction(&c);
                }
            }

            if countlines == 4 {
                if c == 'L' || c == 'F' || c == 'R' {
                    robot2.direction = read_instruction(&robot2, &c);
                }
            }
        }
        countlines = countlines +1;
    }

        println!("robot 1 en x = {}, y = {}  en direction {:?} ",robot1.pos_X, robot1.pos_Y,  robot1.direction) ;
        println!("La direction de robot 2 est {:?}",robot2.direction);

}
