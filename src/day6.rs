pub fn part1() {
    let input = aoc22::read_lines_from_file("day6", false).unwrap();

    let bytes = input.get(0).expect("Invalid input").as_bytes();
    const MSG_SIZE: usize = 14;
    if bytes.len() >= MSG_SIZE {
        for i in 0..bytes.len() - MSG_SIZE {
            let x = &bytes[i..(i + MSG_SIZE)];
            if is_marker_recursive(x) {
                println!("{:?}", i + MSG_SIZE);
                break;
            }
        }
    }
}

fn is_marker_recursive(s: &[u8]) -> bool {
    let (first, rest) = s.split_first().unwrap();
    if rest.len() == 0 {
        return true;
    }
    if !is_marker_recursive(rest) {
        return false;
    }
    for e in rest {
        if first == e {
            return false;
        }
    }
    true
}
