pub fn part1() {
    let input = aoc22::read_lines_from_file("day2", false).unwrap();

    let mut total_score = 0;
    for s in input {
        let st = s.as_bytes();
        let opponent = opponent_base(st[0]);
        let you = you_base(st[2]);
        total_score += cal_score(you) + cal_outcome(opponent, you);
    }
    println!("{}", total_score);
}

pub fn part2() {
    let input = aoc22::read_lines_from_file("day2", false).unwrap();

    let mut total_score = 0;
    for s in input {
        let st = s.as_bytes();
        let opponent = opponent_base(st[0]);
        let outcome = outcome(st[2]);
        let your_play = your_play(opponent, outcome);
        total_score += cal_score(your_play) + outcome;
    }
    println!("{}", total_score);
}

fn opponent_base(a: u8) -> u8 {
    return a - 65;
}
fn you_base(x: u8) -> u8 {
    return x - 88;
}

fn cal_outcome(opponent: u8, you: u8) -> i32 {
    if opponent == you {
        return 3;
    }
    if (opponent == 0 && you == 1) || (opponent == 1 && you == 2) || (opponent == 2 && you == 0) {
        return 6;
    }
    return 0;
}

fn cal_score(s: u8) -> i32 {
    return s as i32 + 1;
}

fn outcome(x: u8) -> i32 {
    return you_base(x) as i32 * 3;
}
fn your_play(opponent: u8, outcome: i32) -> u8 {
    if outcome == 3 {
        return opponent;
    }

    if outcome == 6 {
        return (opponent + 1) % 3;
    }
    return (opponent + 3 - 1) % 3;
}
