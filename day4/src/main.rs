use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

/*
enum Direction {
    U,
    D,
    L,
    R
}

impl Distance {
    fn Move(&self, dis: i32) {
        match self {
            U => 
            D => 
            L => 
            R => 
        }
    }
}
*/

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    dir: String,
}

impl Position {
    fn move_pos(&mut self, dis: &i32) { 
        match self.dir.as_str() {
            /*
            "U" => println!("Dis {:?} Self {:?}", dis, self),
            "R" => println!("Dis {:?} Self {:?}", dis, self),
            "D" => println!("Dis {:?} Self {:?}", dis, self),
            "L" => println!("Dis {:?} Self {:?}", dis, self),
            _   => println!("Dis {:?} Self {:?}", dis, self),
            */
            "U" => self.y + dis,
            "R" => self.x + dis,
            "D" => self.y - dis,
            "L" => self.x - dis,
            _   => self.x + 0
        };
    }
}

fn main() {
    let mut p = Position {x: 0, y: 0, dir: "None".to_string() };
    println!("{:?}", p);
    p.dir = "D".to_string();
    p.move_pos(&123);
    println!("{:?}", p);

}