use std::collections::HashMap;

pub fn part1() {
    let input = aoc22::read_lines_from_file("day5", false).unwrap();

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    let first_line = input.get(0).expect("No line of data");
    let stack_counts = (first_line.as_bytes().len() + 1) / 4;
    for i in 0..stack_counts {
        stacks.insert(i + 1, Vec::new());
    }

    let mut start_instructing = false;
    let mut building_stacks = true;
    for s in input {
        if building_stacks {
            for (i, b) in s.as_bytes().iter().enumerate() {
                if *b >= 65 && *b <= 90 {
                    let key = (i + 3) / 4;
                    let stack = stacks.entry(key).or_default();
                    stack.push(*b as char);
                }
            }
        }

        if s.is_empty() {
            start_instructing = true;
            building_stacks = false;

            for i in 0..stack_counts {
                let stack = stacks.entry(i + 1).or_default();
                stack.reverse();
            }

            println!("{:?}", stacks);
            continue;
        }

        if start_instructing {
            let seg: Vec<&str> = s.split_whitespace().collect();
            if seg.len() == 6 {
                let count: u8 = seg[1].parse().unwrap();
                let source_key: usize = seg[3].parse().unwrap();
                let dest_key: usize = seg[5].parse().unwrap();
                for _ in 0..count {
                    if let Some(c) = stacks.entry(source_key).or_default().pop() {
                        stacks.entry(dest_key).or_default().push(c);
                    }
                }
            }
        }
    }

    println!("{:?}", stacks);
    for i in 0..stack_counts {
        let stack = stacks.entry(i + 1).or_default();
        print!("{:?}", stack.last());
    }
}
