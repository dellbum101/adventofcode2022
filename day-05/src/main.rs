use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    // println!("{}", process_part1(&file));
    println!("{}", process_part2(&file));
}

fn process_part1(input: &str) -> String {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let start_positions = sections[0];
    let move_lines = sections[1];
    let mut stacks = Vec::<Vec<char>>::new();

    const STACK_CHAR_LEN: usize = 4;
    const STACK_CHAR_POSITION: usize = 1;
    start_positions.lines().for_each(|line| {
        let line_chars = line.chars().collect::<Vec<_>>();

        if line_chars[1] == '1' {
            return;
        }

        let mut stack_no: usize = 0;
        while stack_no * STACK_CHAR_LEN < line.len() {
            if stacks.len() <= stack_no {
                stacks.push(Vec::<char>::new());
            }
            if line_chars[stack_no * STACK_CHAR_LEN + STACK_CHAR_POSITION] != ' ' {
                stacks[stack_no].insert(
                    0,
                    line_chars[stack_no * STACK_CHAR_LEN + STACK_CHAR_POSITION],
                );
            }
            stack_no += 1;
        }
    });

    // dbg!(&stacks);

    move_lines.lines().for_each(|line| {
        let tokens = line.split(' ').collect::<Vec<_>>();
        let number_items_to_move = tokens[1].parse::<usize>().unwrap();
        let start_stack = tokens[3].parse::<usize>().unwrap();
        let end_stoack = tokens[5].parse::<usize>().unwrap();

        for _ in 0..number_items_to_move {
            let item = stacks[start_stack - 1].pop().unwrap();
            stacks[end_stoack - 1].push(item);
        }
    });

    let result = stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .fold("".to_string(), |acc, item| acc + &item.to_string());

    return result;
}

fn process_part2(input: &str) -> String {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let start_positions = sections[0];
    let move_lines = sections[1];
    let mut stacks = Vec::<Vec<char>>::new();

    const STACK_CHAR_LEN: usize = 4;
    const STACK_CHAR_POSITION: usize = 1;
    start_positions.lines().for_each(|line| {
        let line_chars = line.chars().collect::<Vec<_>>();

        if line_chars[1] == '1' {
            return;
        }

        let mut stack_no: usize = 0;
        while stack_no * STACK_CHAR_LEN < line.len() {
            if stacks.len() <= stack_no {
                stacks.push(Vec::<char>::new());
            }
            if line_chars[stack_no * STACK_CHAR_LEN + STACK_CHAR_POSITION] != ' ' {
                stacks[stack_no].insert(
                    0,
                    line_chars[stack_no * STACK_CHAR_LEN + STACK_CHAR_POSITION],
                );
            }
            stack_no += 1;
        }
    });

    // dbg!(&stacks);

    move_lines.lines().for_each(|line| {
        let tokens = line.split(' ').collect::<Vec<_>>();
        let number_items_to_move = tokens[1].parse::<usize>().unwrap();
        let start_stack = tokens[3].parse::<usize>().unwrap();
        let end_stack = tokens[5].parse::<usize>().unwrap();

        let mut temp_stack = Vec::<char>::new();
        for _ in 0..number_items_to_move {
            let item = stacks[start_stack - 1].pop().unwrap();
            temp_stack.push(item);
        }

        temp_stack.reverse();
        stacks[end_stack - 1].append(&mut temp_stack);
    });

    let result = stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .fold("".to_string(), |acc, item| acc + &item.to_string());

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "MCD");
    }
}
