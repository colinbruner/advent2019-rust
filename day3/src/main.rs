use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;


enum Wire {
    Move(String, i32),
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    Position { x: i32, y: i32 }
}

struct Position {
    x: i32,
    y: i32,
}

impl Wire {
    fn Up(&self, dis: i32) {
        &self::Position::y + dis
    }
}

fn format_path(direction: &str, pathes: &mut HashMap<String, u32>) {
    // U,D,L,R
    let path = direction.chars().next().unwrap().to_string();

    // Take all the string chars, except the zeroth element, and convert to u32.
    let distance: u32 = direction.chars().as_str()[1..].parse::<u32>().unwrap();

    pathes.insert(path, distance);
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let input = BufReader::new(file).lines().map(|l| l.unwrap());

    // Contains as many elements are "wires" to check pathes for
    let mut wires: Vec<Vec<HashMap<String, u32>>> = Vec::new();

    for path in input {
        // Contains HashMaps of all K: Path -> V: Distance
        let mut all_pathes: Vec<HashMap<String, u32>> = Vec::new();

        // Split all paths in input by ','
        for direction in path.split(",") {

            // K: Path will be U, D, L, R.
            // V: Distance will be an integer containing manhatten distance hops
            let mut pathes: HashMap<String, u32> = HashMap::new();

            // Split input and gather data
            format_path(direction, &mut pathes);

            // Push to vec holding data
            all_pathes.push(pathes);
        }
        // Represents the wire being runs pathes
        wires.push(all_pathes);
    }

    for wire_path in wires {
        for path in wire_path {
            for (dir, dis) in path {
                println!("{:?}", dir);
                println!("{:?}", dis);
            }
            println!("---------------------------------");
        }
    }

    Ok(())
}
