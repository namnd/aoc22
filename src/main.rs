use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day1.txt").expect("Cannot open file"));

    let mut top_three = vec![0, 0, 0];
    let mut total = 0;
    for line in reader.lines() {
        let s: String = line.unwrap();
        if s == "" {
            total = 0;
        } else {
            let cal: i32 = s.parse().unwrap();
            total += cal;
        }
        top_three = replace_min(top_three, total);
    }
    println!("Top 3: {:?}", top_three);
    let sum: i32 = top_three.iter().sum();
    println!("Sum: {}", sum);
    Ok(())
}

fn replace_min(mut v: Vec<i32>, n: i32) -> Vec<i32> {
    let min = v.iter().min().unwrap();
    if *min < n {
        let index = v.iter().position(|x| x == min);
        match index {
            Some(i) => v[i] = n,
            None => (),
        }
    }
    v
}
