use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn calcu(operation: usize, cursor: usize, program: &mut Vec<usize>) {
    // Determine first, second and third values
    let op1 = program[program[cursor + 1]];
    let op2 = program[program[cursor + 2]];
    let pos = program[cursor + 3];

    let res = match operation {
        1 => { op1 + op2 }
        2 => { op1 * op2 }
        _ => 0
    };
    program[pos] = res
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // ASSUMPTION: 1 line contains 1 "program" run.
    for line in reader.lines() {
        let mut cursor: usize = 0;
        // Split line by ',' and parse to a vec<u32> ints
        let mut program = line.unwrap()
            .split(",")
            .filter_map(|p| p.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        while program[cursor] != 99 {
            if program[cursor] == 1 {
                calcu(1, cursor, &mut program)
            } else if program[cursor] == 2 {
                calcu(2, cursor, &mut program)
            }
            cursor += 4;
        }

        println!("{:?}", program[0]);

    }

    Ok(())
}