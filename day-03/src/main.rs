use std::fs;

fn main() {
  let file = fs::read_to_string("./input.txt").unwrap();
  // println!("{}", process_part1(&file));
  println!("{}", process_part2(&file));
}

fn process_part1(input: &str) -> String {
  let result = input 
    .lines()
    .map(|line| {
      let capacity = line.len() / 2;
      let compartment1 = &line[0..capacity];
      let compartment2 = &line[capacity..line.len()];
      let repeated_char = compartment1.chars().find(|char| {
        return compartment2.contains(*char);
      }).unwrap();
      if repeated_char.is_ascii_lowercase() {
        return repeated_char as u32 - 96;
      } else if repeated_char.is_ascii_uppercase() {
        return repeated_char as u32 - 38;
      } else {
        return 0;
      }
    })
    .sum::<u32>();

    return result.to_string();
}

fn process_part2(input: &str) -> String {
  let lines = input 
    .lines()
    .collect::<Vec<_>>();

  let result = lines.chunks(3)
    .map(|group| {
      let repeated_char = group[0].chars().find(|char| {
        return group[1].contains(*char) && group[2].contains(*char);
      }).unwrap();
      if repeated_char.is_ascii_lowercase() {
        return repeated_char as u32 - 96;
      } else if repeated_char.is_ascii_uppercase() {
        return repeated_char as u32 - 38;
      } else {
        return 0;
      }
    })
    .sum::<u32>();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}