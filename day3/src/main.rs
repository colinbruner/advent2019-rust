// All credit for this code goes to https://dev.to/rpalo/comment/ig0e 

use std::fs;
use std::ops::Add;
use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinate {
    x: isize,
    y: isize,

}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self {x, y}
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

type Wire = Vec<Coordinate>;
type Moves = Vec<Coordinate>;

fn format_instructions(wire_instructions: &str) -> Moves {

    // Represents each x, y coordinate move-by-move
    let mut moves: Moves = Vec::new();

    // For each wire, parse instructions and 
    for task in wire_instructions.split(",") {
        // turn: U,D,L,R distance: int
        let (turn, distance_text) = task.split_at(1);
        let distance: usize = distance_text.parse().unwrap();

        let coord = match turn {
            "U" => Coordinate::new(0, 1),
            "R" => Coordinate::new(1, 0),
            "D" => Coordinate::new(0, -1),
            "L" => Coordinate::new(-1, 0),
            _   => panic!("Direction {:?} not valid.", turn)
        };

        for _ in 0..distance {
            //println!("{:?}", c);
            moves.push(coord);
        }

    }
    moves
}

/// Build a Wire out of relative Moves
fn make_wire(moves: Moves) -> Wire {
    let mut current = Coordinate { x: 0, y: 0 };
    let mut results: Wire = Vec::new();

    for step in moves {
        current = step + current;
        results.push(current);

    }

    results
}

fn manhattan_distance(coord: &Coordinate) -> usize {
    (coord.x.abs() + coord.y.abs()) as usize
}

fn find_closest_cross(wire_1: &Wire, wire_2: &Wire) -> Coordinate {
    let wire_1: HashSet<&Coordinate> = HashSet::from_iter(wire_1.iter());
    let wire_2: HashSet<&Coordinate> = HashSet::from_iter(wire_2.iter());
    **wire_1.intersection(&wire_2).min_by_key(|c| manhattan_distance(c)).unwrap()
}

fn find_in_wire(wire: &Wire, target: &Coordinate) -> usize {
    wire.iter().position(|e| e == target).unwrap() + 1
}

fn shortest_cross_distance(a: &Wire, b: &Wire) -> usize {
    let a_set: HashSet<&Coordinate> = HashSet::from_iter(a.iter());
    let b_set: HashSet<&Coordinate> = HashSet::from_iter(b.iter());
    a_set.intersection(&b_set).map(|c| find_in_wire(a, c) + find_in_wire(b, c)).min().unwrap()
}

fn main() {
    let text = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = text.split("\n").collect();

    let mut wires = lines.into_iter().map(format_instructions).map(make_wire);

    let (wire1, wire2) = (wires.next().unwrap(), wires.next().unwrap());

    let closest_cross = find_closest_cross(&wire1, &wire2);
    println!("Manhatten Distance of closest cross {:?}", manhattan_distance(&closest_cross));
    println!("Fewest combined steps: {}", shortest_cross_distance(&wire1, &wire2));

}
