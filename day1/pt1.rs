use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut calorie_vec: Vec<i32> = vec![];
        let mut totals_vec: Vec<i32> = vec![];
        for line in lines {
            if let Ok(calorie) = line {
                if calorie.is_empty() {
                    totals_vec.push(calorie_vec.iter().sum());
                    calorie_vec.clear();
                } else {
                    calorie_vec.push(calorie.parse().unwrap());
                }
            }
        }
        println!("Min: {:?}", totals_vec.iter().max());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}