pub fn part1() {
    let input = aoc22::read_lines_from_file("day4", false).unwrap();

    let mut count = 0;
    for s in input {
        let pair: Vec<&str> = s.split(",").collect();
        let (min1, max1) = extract(pair[0]);
        let (min2, max2) = extract(pair[1]);
        if (min1 <= min2 && max2 <= max1) || (min2 <= min1 && max1 <= max2) {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn part2() {
    let input = aoc22::read_lines_from_file("day4", false).unwrap();

    let mut count = 0;
    for s in input {
        let pair: Vec<&str> = s.split(",").collect();
        let (min1, max1) = extract(pair[0]);
        let (min2, max2) = extract(pair[1]);
        if (min1 <= min2 && min2 <= max1) || (min2 <= min1 && min1 <= max2) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn extract(s: &str) -> (u8, u8) {
    let b: Vec<&str> = s.split("-").collect();
    return (b[0].parse::<u8>().unwrap(), b[1].parse::<u8>().unwrap());
}
