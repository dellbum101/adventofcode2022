use std::{collections::HashMap, fs};

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part1(&file));
    println!("{}", process_part2(&file));
}

fn process_part1(input: &str) -> String {
    let mut path = Vec::<String>::new();
    let mut directories = HashMap::<String, usize>::new();

    input.lines().for_each(|line| {
        if line.starts_with("$ cd ") {
            let dir = line.get(5..);
            match dir {
                Some("/") => {
                    path.push("".to_string());
                    return;
                }
                Some("..") => {
                    path.pop();
                    return;
                }
                None => return,
                Some(dir_name) => {
                    path.push(dir_name.to_string());
                    return;
                }
            };
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("dir") {
            let dir = line.get(4..);
            match dir {
                None => return,
                Some(dir_name) => {
                    let dir_path = [path.join("/"), dir_name.to_string()].join("/");
                    directories.entry(dir_path).or_insert(0);
                    return;
                }
            };
        } else {
            let tokens: Vec<&str> = line.split(" ").collect();
            let size = tokens[0].parse::<usize>().unwrap();
            path.clone()
                .into_iter()
                .fold("".to_string(), |mut path, dir| {
                    if path != "/".to_string() {
                        path.push_str("/");
                    }
                    path.push_str(&dir);
                    directories
                        .entry(path.clone())
                        .and_modify(|current_size| *current_size += size);
                    return path;
                });
        }
    });

    let total_size = directories
        .into_iter()
        .map(|(_, dir_size)| dir_size)
        .filter(|dir_size| *dir_size <= 100_000)
        .sum::<usize>();
    return total_size.to_string();
}

fn process_part2(input: &str) -> String {
    let mut path = Vec::<String>::new();
    let mut directories = HashMap::<String, usize>::new();

    input.lines().for_each(|line| {
        if line.starts_with("$ cd ") {
            let dir = line.get(5..);
            match dir {
                Some("/") => {
                    path.push("".to_string());
                    directories.entry("/".to_string()).or_insert(0);
                    return;
                }
                Some("..") => {
                    path.pop();
                    return;
                }
                None => return,
                Some(dir_name) => {
                    path.push(dir_name.to_string());
                    return;
                }
            };
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("dir") {
            let dir = line.get(4..);
            match dir {
                None => return,
                Some(dir_name) => {
                    let mut dir_path: String = "/".to_string();
                    if path.len() != 1 {
                        dir_path = path.join("/").to_string();
                        dir_path.push_str("/");
                    }
                    dir_path.push_str(dir_name);
                    directories.entry(dir_path).or_insert(0);
                    return;
                }
            };
        } else {
            let tokens: Vec<&str> = line.split(" ").collect();
            let size = tokens[0].parse::<usize>().unwrap();
            path.clone()
                .into_iter()
                .fold("/".to_string(), |mut path, dir| {
                    if path != "/".to_string() {
                        path.push_str("/");
                    }
                    path.push_str(&dir);
                    directories
                        .entry(path.clone())
                        .and_modify(|current_size| *current_size += size);
                    return path;
                });
        }
    });

    const TOTAL_SIZE: usize = 70_000_000;
    const REQUIRED_SPACE: usize = 30_000_000;
    let used_space = match directories.get("/") {
        Some(size) => size,
        None => &0,
    };
    let available_space = TOTAL_SIZE - used_space;
    let need_to_free = REQUIRED_SPACE - available_space;
    let mut possible_directories = directories
        .into_iter()
        .map(|(_, dir_size)| dir_size)
        .filter(|dir_size| *dir_size > need_to_free)
        .collect::<Vec<usize>>();
    possible_directories.sort();

    return possible_directories[0].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "24933642");
    }
}
