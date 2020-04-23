use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn calc(operation: usize, cursor: usize, program: &mut Vec<usize>) {
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

fn int_code_program(mut program: &mut Vec<usize>, mut cursor: usize) -> usize {
    while program[cursor] != 99 {
        if program[cursor] == 1 {
            calc(1, cursor, &mut program)
        } else if program[cursor] == 2 {
            calc(2, cursor, &mut program)
        }
        cursor += 4;
    }
    let res = program[0];
    res
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines().map(|l| l.unwrap());

    // ASSUMPTION: 1 line contains 1 "program" run.
    for line in lines {
        let cursor: usize = 0;
        // Split line by ',' and parse to a vec<u32> ints
        let orig_program = line
            .split(",")
            .filter_map(|p| p.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        let target: usize = 19690720;
        let mut noun: usize = 0;

        while noun != 99 {
            for i in 0..99 {
                let verb = i;
                let mut program = Vec::new();
                program.extend(&orig_program);

                program[1] = noun;
                program[2] = verb;

                let res = int_code_program(&mut program, cursor);
                if res == target {
                    let ans_noun = noun;
                    let ans_verb = verb;
                    println!("Target {:?}", res);
                    println!("Answer {:?}", 100 * ans_noun + ans_verb);

                } else {
                };
            }
            noun += 1;
        }
    }

    Ok(())
}