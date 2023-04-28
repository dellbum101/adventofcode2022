pub fn process_part1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    return result.to_string();
}

pub fn process_part2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    result.sort_unstable_by(|a, b| b.cmp(a));

    return result
        .iter()
        .take(3)
        .sum::<u32>()
        .to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = 
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = process_part2(input);
        assert_eq!(result, "45000");
    }
}
