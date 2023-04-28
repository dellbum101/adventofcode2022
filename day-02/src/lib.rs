pub fn process_part1(input: &str) -> String {
    let result = input
        .split("\n")
        .map(|round| {
            let moves = round.split(" ").collect::<Vec<&str>>();
            let my_move = moves[1];
            let my_move_score =  match my_move{
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };
            let outcome_score = match round {
                "A X"|"B Y"|"C Z" => 3,
                "A Y"|"B Z"|"C X" => 6,
                _ => 0,
            };
            return my_move_score + outcome_score;
        })
        .sum::<u32>();
    return result.to_string();
}

pub fn process_part2(input: &str) -> String {
    let result = input
        .split("\n")
        .map(|round| {
            let moves = round.split(" ").collect::<Vec<&str>>();
            let opponent_move = moves[0];
            let outcome = moves[1];
            return match outcome{
                "X" => 0 + match opponent_move{
                    "A" => 3,
                    "B" => 1,
                    "C" => 2,
                    _ => 0,
                },
                "Y" => 3 + match opponent_move{
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    _ => 0,
                },
                "Z" => 6 + match opponent_move{
                    "A" => 2,
                    "B" => 3,
                    "C" => 1,
                    _ => 0,
                },
                _ => 0,
            };
        })
        .sum::<u32>();
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = 
"A Y
B X
C Z";
        let result = process_part1(input);
        assert_eq!(result, "15");
    }
    
    #[test]
    fn part2_works() {
        let input = 
"A Y
B X
C Z";
        let result = process_part2(input);
        assert_eq!(result, "12");
    }
}
