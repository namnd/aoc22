pub fn part1() {
    let input = aoc22::read_lines_from_file("day6", false).unwrap();

    let bytes = input.get(0).expect("Invalid input").as_bytes();
    if bytes.len() >= 4 {
        for i in 0..bytes.len() - 4 {
            let x = &bytes[i..(i + 4)];
            if is_marker(x) {
                println!("{:?}", i + 4);
                break;
            }
        }
    }
}

fn is_marker(s: &[u8]) -> bool {
    s[0] != s[1] && s[0] != s[2] && s[0] != s[3] && s[1] != s[2] && s[1] != s[3] && s[2] != s[3]
}
