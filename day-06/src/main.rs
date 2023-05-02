use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
    println!("{}", process_part2(&file));
}

fn process_part1(input: &str) -> String {
    const WINDOW_SIZE: usize = 4;
    let stream = input.chars().collect::<Vec<_>>();
    let mut marker = WINDOW_SIZE - 1;

    stream.windows(WINDOW_SIZE).any(|window| {
        marker += 1;
        return !window.get(1..).unwrap().contains(&window[0])
            && !window.get(2..).unwrap().contains(&window[1])
            && !window.get(3..).unwrap().contains(&window[2]);
    });

    return marker.to_string();
}

fn process_part2(input: &str) -> String {
    const WINDOW_SIZE: usize = 14;
    let stream = input.chars().collect::<Vec<_>>();
    let mut marker = WINDOW_SIZE - 1;

    stream.windows(WINDOW_SIZE).any(|window| {
        marker += 1;
        return (1..WINDOW_SIZE).all(|i| !window.get(i..).unwrap().contains(&window[i - 1]));
    });

    return marker.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "11");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "26");
    }
}
