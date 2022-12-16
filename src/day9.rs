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
            move_one(direction, &mut head, &mut tail);

            let key = tail.x.to_string() + &":".to_string() + &tail.y.to_string();
            tail_visited.insert(key);
        }
    }

    println!("{}", tail_visited.len());
}

fn move_one(direction: &str, head: &mut Coordinate, tail: &mut Coordinate) {
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
}

pub fn part2() {
    let input = aoc22::read_lines_from_file("day9", false).unwrap();

    let mut tail_visited: HashSet<String> = HashSet::new();
    let mut rope: Vec<Coordinate> = Vec::new();
    let rope_size = 10;
    for _ in 0..rope_size {
        rope.push(Coordinate { x: 0, y: 0 });
    }

    for s in input {
        let parts: Vec<_> = s.split_whitespace().collect();
        let direction = parts[0];
        let distance = parts[1].parse::<i32>().unwrap();

        for _ in 0..distance {
            let head = move_head(direction, rope.first_mut().unwrap());
            let mut tail = move_next_to_head(direction, head, rope.get_mut(1).unwrap());
            for i in 2..rope_size {
                tail = move_knot(tail, rope.get_mut(i).unwrap());
            }
            let key = tail.x.to_string() + &":".to_string() + &tail.y.to_string();
            tail_visited.insert(key);
        }
        dbg!(&rope);
    }

    println!("{}", tail_visited.len());
}

fn move_head(direction: &str, head: &mut Coordinate) -> Coordinate {
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
    Coordinate {
        x: head.x,
        y: head.y,
    }
}

fn move_next_to_head(direction: &str, head: Coordinate, knot: &mut Coordinate) -> Coordinate {
    if head.x.abs_diff(knot.x) > 1 || head.y.abs_diff(knot.y) > 1 {
        if direction == "R" || direction == "L" {
            knot.y = head.y;
            if direction == "R" {
                knot.x += 1;
            }
            if direction == "L" {
                knot.x -= 1;
            }
        }
        if direction == "U" || direction == "D" {
            knot.x = head.x;
            if direction == "U" {
                knot.y += 1;
            }
            if direction == "D" {
                knot.y -= 1;
            }
        }
    }
    Coordinate {
        x: knot.x,
        y: knot.y,
    }
}

fn move_knot(prev: Coordinate, knot: &mut Coordinate) -> Coordinate {
    if prev.x.abs_diff(knot.x) > 1 || prev.y.abs_diff(knot.y) > 1 {
        if prev.x == knot.x {
            knot.y += (prev.y - knot.y) / 2;
        } else if prev.y == knot.y {
            knot.x += (prev.x - knot.x) / 2;
        } else if (prev.x - knot.x).abs() == 2 {
            knot.y = prev.y;
            knot.x += (prev.x - knot.x) / 2;
        } else if (prev.y - knot.y).abs() == 2 {
            knot.x = prev.x;
            knot.y += (prev.y - knot.y) / 2;
        }
    }
    Coordinate {
        x: knot.x,
        y: knot.y,
    }
}
