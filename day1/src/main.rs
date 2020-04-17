use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1() -> io::Result<()> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut val: Vec<f64> = Vec::new();
    for line in reader.lines() {
        let num = line.unwrap().parse::<f64>().unwrap();
        val.push(calc_fuel(num));
    }

    let sum: f64 = val.iter().sum();
    println!("part 1 {}", sum);

    Ok(())
}

fn calc_fuel(val: f64) -> f64 {
    return (val / 3.0).floor() - 2.0
}

fn doit(mut num: f64) -> f64 {
    let mut done = false;

    let mut val: Vec<f64> = Vec::new();
    while !done {
        num = calc_fuel(num);
        if num > 0.0 {
            val.push(num);
        } else {
            done = true;
            println!("Else statement, found: {:?}", num);
            println!("Breaking while loop -----------------------------------");
        }
        println!("Found -> {:?}. Continuing...", val);
    }

    println!("Values found before sum -> {:?}", val);
    let sum: f64 = val.iter().sum();
    println!("Line Total -> {:?}", sum);
    sum
}

fn part2() -> io::Result<()> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let mut val: Vec<f64> = Vec::new();
    for line in reader.lines() {
        let mut num = line.unwrap().parse::<f64>().unwrap();
        num = doit(num);
        val.push(num)
    }

    let sum: f64 = val.iter().sum();
    println!("part 2 {}", sum);

    Ok(())
}

fn main() -> io::Result<()> {
    part1()?;
    part2()?;

    Ok(())
}
