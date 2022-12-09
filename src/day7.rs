use std::collections::HashMap;

pub fn part1() {
    let input = aoc22::read_lines_from_file("day7", false).unwrap();

    let mut current_command = "";
    let mut dirs_stack: Vec<String> = Vec::new();
    let mut dirs: HashMap<String, i32> = HashMap::new();

    for (_, s) in input.iter().enumerate() {
        if s.starts_with("$") {
            current_command = s;
            if current_command == "$ cd /" {
                dirs_stack.clear();
                dirs_stack.push(".".to_string());
            } else if current_command.to_string().starts_with("$ cd ..") {
                dirs_stack.pop();
            } else if current_command.starts_with("$ cd") {
                let tail = current_command.split_whitespace().last().unwrap();
                let current_dir = dirs_stack.last().unwrap().to_string() + "/" + tail;
                dirs_stack.push(current_dir);
            }
        }

        if current_command == "$ ls" {
            if !s.starts_with("dir") && !s.starts_with("$") {
                let parts: Vec<&str> = s.split_whitespace().collect();
                let size = parts[0].parse::<i32>().unwrap();
                for dir in &dirs_stack {
                    let ds = dirs.entry(dir.to_string()).or_insert(0);
                    *ds += size;
                }
            }
        }
    }

    let mut sorted: Vec<_> = dirs.iter().collect();
    sorted.sort_by(|&a, &b| a.1.partial_cmp(b.1).unwrap());
    let root_size = sorted.last().unwrap().1;
    let to_free = root_size - (70000000 - 30000000);

    for (_, &size) in sorted {
        if size >= to_free {
            println!("{}", size);
            break;
        }
    }
}
