use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut calorie_sum: i32 = 0;
        let mut totals_vec: Vec<i32> = vec![];
        for line in lines {
            if let Ok(calorie) = line {
                if calorie.is_empty() {
                    totals_vec.push(calorie_sum);
                    calorie_sum = 0;
                } else {
                    calorie_sum += calorie.parse::<i32>().unwrap();
                }
            }
        }
        println!("Min: {:?}", totals_vec.iter().max());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
