pub fn part1() {
    let input = aoc22::read_lines_from_file("day3", false).unwrap();

    let mut total: i32 = 0;
    for s in input {
        let st = s.as_bytes();
        let first = &st[..st.len() / 2];
        let second = &st[st.len() / 2..];
        for c in first {
            if second.contains(c) {
                total += cal_priority(*c);
                break;
            }
        }
    }
    println!("{:?}", total);
}

pub fn part2() {
    let input = aoc22::read_lines_from_file("day3", false).unwrap();
    let mut count = 0;
    let mut group: Vec<String> = Vec::new();
    let mut total: i32 = 0;
    for s in input {
        if count % 3 == 0 {
            group.clear();
        }
        group.push(s);
        count += 1;
        if group.len() == 3 {
            for c in group[0].as_bytes() {
                if group[1].as_bytes().contains(c) && group[2].as_bytes().contains(c) {
                    total += cal_priority(*c);
                    break;
                }
            }
        }
    }
    println!("{:?}", total);
}

fn cal_priority(c: u8) -> i32 {
    if c <= 90 {
        return c as i32 - 65 + 27;
    }
    return c as i32 - 97 + 1;
}
