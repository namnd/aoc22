pub fn part1() {
    let input = aoc22::read_lines_from_file("day3", false).unwrap();

    let mut total: i32 = 0;
    for s in input {
        let st = s.as_bytes();
        let first = &st[..st.len() / 2];
        let second = &st[st.len() / 2..];
        for c in first {
            if second.contains(c) {
                if *c <= 90 {
                    total += *c as i32 - 65 + 27;
                } else {
                    total += *c as i32 - 97 + 1;
                }
                break;
            }
        }
    }
    println!("{:?}", total);
}
