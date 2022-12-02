use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("inputs/day2.txt").expect("Cannot open file"));

    let mut score_map = HashMap::new();
    score_map.insert("X", 1);
    score_map.insert("Y", 2);
    score_map.insert("Z", 3);
    let mut outcome_map = HashMap::new();
    outcome_map.insert("A X", 3);
    outcome_map.insert("A Y", 6);
    outcome_map.insert("A Z", 0);
    outcome_map.insert("B X", 0);
    outcome_map.insert("B Y", 3);
    outcome_map.insert("B Z", 6);
    outcome_map.insert("C X", 6);
    outcome_map.insert("C Y", 0);
    outcome_map.insert("C Z", 3);

    let mut total_scores = 0;
    for line in reader.lines() {
        let s: String = line.unwrap();
        let play = s.split(" ");

        let our = play.last();
        if let Some(score) = our {
            total_scores += score_map[score];
        }
        total_scores += outcome_map[s.as_str()];
    }
    println!("{}", total_scores);

    Ok(())
}
