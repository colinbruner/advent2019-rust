use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn calc(file: File) -> Vec<f64> {
    let reader = BufReader::new(file);

    let mut val: Vec<f64> = Vec::new();
    for line in reader.lines() {
        let num = line.unwrap().parse::<f64>().unwrap();
        val.push((num / 3.0).floor() - 2.0);
    }
    val
}

fn main() -> io::Result<()> {
    let input = File::open("input.txt")?;

    let vec = calc(input);
    let sum: f64 = vec.iter().sum();
    println!("{}", sum);

    Ok(())
}
