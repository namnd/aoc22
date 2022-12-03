pub fn part1() {
    let input = aoc22::read_lines_from_file("day1", false).unwrap();

    let mut elves: Vec<i32> = Vec::new();
    let mut elf_cal = 0;
    for s in input {
        if s == "" {
            if elf_cal > 0 {
                elves.push(elf_cal);
            }
            elf_cal = 0;
        } else {
            let cal: i32 = s.parse().unwrap();
            elf_cal += cal;
        }
    }
    println!("{:?}", elves.iter().max());
}

pub fn part2() {
    let input = aoc22::read_lines_from_file("day1", false).unwrap();

    let mut elves: Vec<i32> = Vec::new();
    let mut elf_cal = 0;
    for s in input {
        if s == "" {
            if elf_cal > 0 {
                elves.push(elf_cal);
            }
            elf_cal = 0;
        } else {
            let cal: i32 = s.parse().unwrap();
            elf_cal += cal;
        }
    }
    elves.sort();
    let mut total = 0;
    for _ in 0..3 {
        if let Some(max) = elves.pop() {
            total += max;
        }
    }
    println!("{}", total);
}
