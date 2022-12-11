use std::collections::HashSet;

#[derive(Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}
pub fn part1() {
    let input = aoc22::read_lines_from_file("day9", false).unwrap();

    let mut tail_visited: HashSet<String> = HashSet::new();
    let mut head = Coordinate { x: 0, y: 0 };
    let mut tail = Coordinate { x: 0, y: 0 };

    for s in input {
        let parts: Vec<_> = s.split_whitespace().collect();
        let direction = parts[0];
        let distance = parts[1].parse::<i32>().unwrap();

        for _ in 0..distance {
            match direction {
                "R" => {
                    head.x += 1;
                }
                "L" => {
                    head.x -= 1;
                }
                "D" => {
                    head.y -= 1;
                }
                "U" => {
                    head.y += 1;
                }
                _ => {}
            }

            if head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1 {
                if direction == "R" || direction == "L" {
                    tail.y = head.y;
                    if direction == "R" {
                        tail.x += 1;
                    }
                    if direction == "L" {
                        tail.x -= 1;
                    }
                }
                if direction == "U" || direction == "D" {
                    tail.x = head.x;
                    if direction == "U" {
                        tail.y += 1;
                    }
                    if direction == "D" {
                        tail.y -= 1;
                    }
                }
            }

            let key = tail.x.to_string() + &":".to_string() + &tail.y.to_string();
            tail_visited.insert(key);
        }
    }

    println!("{}", tail_visited.len());
}
