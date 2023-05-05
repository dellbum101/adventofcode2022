use std::{collections::HashMap, fs};

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
    println!("{}", process_part2(&file));
}

fn process_part1(input: &str) -> String {
    let mut seen_trees = HashMap::<String, bool>::new();
    let mut rows = Vec::<Vec<i64>>::new();
    let mut cols = Vec::<Vec<i64>>::new();

    input.lines().enumerate().for_each(|(row_num, line)| {
        rows.push(Vec::<i64>::new());
        line.chars().enumerate().for_each(|(col_num, height_char)| {
            if row_num == 0 {
                cols.push(Vec::<i64>::new());
            }
            let height: i64 = match height_char.to_digit(10) {
                Some(value) => value.into(),
                _ => 0,
            };
            rows[row_num].push(height);
            cols[col_num].push(height);
        })
    });

    for row_num in 0..rows.len() {
        let mut tallest_from_left = -1;
        let mut tallest_from_right = -1;
        for col_num in 0..rows[row_num].len() {
            if rows[row_num][col_num] > tallest_from_left {
                let grid_space =
                    [row_num.to_string(), ",".to_string(), col_num.to_string()].join("");
                seen_trees
                    .entry(grid_space)
                    .and_modify(|seen| *seen = true)
                    .or_insert(true);
                tallest_from_left = rows[row_num][col_num];
            }
            let next_right_tree_col = rows[row_num].len() - col_num - 1;
            let next_rigt_tree = rows[row_num][next_right_tree_col];
            if next_rigt_tree > tallest_from_right {
                let grid_space = [
                    row_num.to_string(),
                    ",".to_string(),
                    next_right_tree_col.to_string(),
                ]
                .join("");
                seen_trees
                    .entry(grid_space)
                    .and_modify(|seen| *seen = true)
                    .or_insert(true);
                tallest_from_right = next_rigt_tree;
            }
        }
    }

    for col_num in 0..cols.len() {
        let mut tallest_from_top = -1;
        let mut tallest_from_buttom = -1;
        for row_num in 0..cols[col_num].len() {
            if cols[col_num][row_num] > tallest_from_top {
                let grid_space =
                    [row_num.to_string(), ",".to_string(), col_num.to_string()].join("");
                seen_trees
                    .entry(grid_space)
                    .and_modify(|seen| *seen = true)
                    .or_insert(true);
                tallest_from_top = cols[col_num][row_num];
            }

            let next_bottom_tree_row = cols[col_num].len() - row_num - 1;
            let next_bottom_tree = cols[col_num][next_bottom_tree_row];
            if next_bottom_tree > tallest_from_buttom {
                let grid_space = [
                    next_bottom_tree_row.to_string(),
                    ",".to_string(),
                    col_num.to_string(),
                ]
                .join("");
                seen_trees
                    .entry(grid_space)
                    .and_modify(|seen| *seen = true)
                    .or_insert(true);
                tallest_from_buttom = next_bottom_tree;
            }
        }
    }

    return seen_trees.len().to_string();
}

fn process_part2(input: &str) -> String {
    let mut seen_trees = HashMap::<String, u32>::new();
    let mut rows = Vec::<Vec<i64>>::new();
    let mut cols = Vec::<Vec<i64>>::new();

    input.lines().enumerate().for_each(|(row_num, line)| {
        rows.push(Vec::<i64>::new());
        line.chars().enumerate().for_each(|(col_num, height_char)| {
            if row_num == 0 {
                cols.push(Vec::<i64>::new());
            }
            let height: i64 = match height_char.to_digit(10) {
                Some(value) => value.into(),
                _ => 0,
            };
            rows[row_num].push(height);
            cols[col_num].push(height);
        })
    });

    let mut max_score = 0;

    for tree_row in 1..rows.len() - 1 {
        for tree_col in 1..rows[tree_row].len() - 1 {
            let tree_height = rows[tree_row][tree_col];
            let mut score = 1;

            let mut range = (0..tree_col).collect::<Vec<usize>>();
            range.reverse();
            score *= score_range(tree_height, range, &rows[tree_row]);

            score *= score_range(
                tree_height,
                ((tree_col + 1)..rows[tree_row].len()).collect::<Vec<usize>>(),
                &rows[tree_row],
            );

            let mut range = (0..tree_row).collect::<Vec<usize>>();
            range.reverse();
            score *= score_range(tree_height, range, &cols[tree_col]);

            score *= score_range(
                tree_height,
                ((tree_row + 1)..cols[tree_col].len()).collect::<Vec<usize>>(),
                &cols[tree_col],
            );

            // println!("{},{},{} = {}", tree_row, tree_col, tree_height, score);
            if score > max_score {
                max_score = score;
            }
        }
    }

    return max_score.to_string();
}

fn score_range(tree_height: i64, range: Vec<usize>, row: &Vec<i64>) -> u32 {
    let mut count = 0;
    // print!("  Range: {:#?}", range);
    for col_num in range {
        // print!(" {}", count);
        count += 1;
        if (*row)[col_num] >= tree_height {
            break;
        }
    }
    // println!("  Score: {}", count);
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "8");
    }
}
