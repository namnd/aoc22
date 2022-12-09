use std::collections::HashSet;

pub fn part1() {
    let input = aoc22::read_lines_from_file("day8", false).unwrap();

    let mut left_wall: Vec<u8> = Vec::new();
    let mut right_wall: Vec<u8> = Vec::new();
    let mut trees: Vec<Vec<u8>> = Vec::new();
    for s in input {
        let row: Vec<u8> = s.as_bytes().iter().map(|x| x - 48).collect();
        left_wall.push(row[0]);
        right_wall.push(*row.last().unwrap());
        trees.push(row);
    }

    let mut top_wall = trees.get(0).unwrap().to_vec();
    let mut bottom_wall = trees.last().unwrap().to_vec();

    let rows_count = trees.len();
    let cols_count = trees.get(0).unwrap().len();
    let mut count = (rows_count + cols_count) * 2 - 4;
    let mut visible_trees: HashSet<String> = HashSet::new();

    for row_index in 1..rows_count - 1 {
        for col_index in 1..cols_count - 1 {
            let key = format!("{col_index}-{row_index}");
            let tree = trees.get(row_index).unwrap().get(col_index).unwrap();
            let top = top_wall.get_mut(col_index).unwrap();
            let left = left_wall.get_mut(row_index).unwrap();
            if tree > top || tree > left {
                if !visible_trees.contains(&key) {
                    visible_trees.insert(key.to_string());
                    count += 1;
                }
                if tree > top {
                    *top = *tree;
                }
                if tree > left {
                    *left = *tree;
                }
            }

            let col_index_reverse = cols_count - col_index - 1;
            let row_index_reverse = rows_count - row_index - 1;
            let key_reverse = format!("{col_index_reverse}-{row_index_reverse}");
            let tree_reverse = trees
                .get(row_index_reverse)
                .unwrap()
                .get(col_index_reverse)
                .unwrap();
            let bottom = bottom_wall.get_mut(col_index_reverse).unwrap();
            let right = right_wall.get_mut(row_index_reverse).unwrap();
            if tree_reverse > right || tree_reverse > bottom {
                if !visible_trees.contains(&key_reverse) {
                    visible_trees.insert(key_reverse.to_string());
                    count += 1;
                }
                if tree_reverse > right {
                    *right = *tree_reverse;
                }
                if tree_reverse > bottom {
                    *bottom = *tree_reverse;
                }
            }
        }
    }

    println!("{}", count);
}

