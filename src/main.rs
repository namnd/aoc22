use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day1.txt").expect("Cannot open file"));

    let mut max = 0;
    let mut total = 0;
    for line in reader.lines() {
        let s: String = line.unwrap();
        if s == "" {
            total = 0;
        } else {
            let cal: i32 = s.parse().unwrap();
            total += cal;
        }
        if total > max {
            max = total;
        }
    }
    println!("{}", max);
    Ok(())
}
