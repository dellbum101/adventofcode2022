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
      let elf_assignments = line.split(',').collect::<Vec<_>>();
      let elf_a = elf_assignments[0].split('-').collect::<Vec<_>>();
      let elf_b = elf_assignments[1].split('-').collect::<Vec<_>>();
      if (elf_a[0].parse::<u32>().unwrap() >= elf_b[0].parse::<u32>().unwrap()
        && elf_a[1].parse::<u32>().unwrap() <= elf_b[1].parse::<u32>().unwrap())
        || (elf_b[0].parse::<u32>().unwrap() >= elf_a[0].parse::<u32>().unwrap()
        && elf_b[1].parse::<u32>().unwrap() <= elf_a[1].parse::<u32>().unwrap()) {
        return 1;
      }
      return 0;      
    })
    .sum::<u32>();

    return result.to_string();
}

fn process_part2(input: &str) -> String {
  let result = input 
    .lines()
    .map(|line| {
      let elf_assignments = line.split(',').collect::<Vec<_>>();
      let elf_a = elf_assignments[0].split('-').collect::<Vec<_>>();
      let elf_b = elf_assignments[1].split('-').collect::<Vec<_>>();
      if (elf_a[1].parse::<u32>().unwrap() >= elf_b[0].parse::<u32>().unwrap()
        && elf_a[0].parse::<u32>().unwrap() <= elf_b[1].parse::<u32>().unwrap())
        || (elf_b[1].parse::<u32>().unwrap() >= elf_a[0].parse::<u32>().unwrap()
        && elf_b[0].parse::<u32>().unwrap() <= elf_a[1].parse::<u32>().unwrap()) {
        return 1;
      }
      return 0;      
    })
    .sum::<u32>();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}