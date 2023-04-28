use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

fn main() {
    // part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines("./src/input1.txt") {
        let mut max_calories: i32 = 0;
        let mut current_elf_calories: i32 = 0;

        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    if current_elf_calories > max_calories {
                        max_calories = current_elf_calories;
                    }
                    current_elf_calories = 0;
                    continue;
                }

                current_elf_calories += ip.parse::<i32>().unwrap();
            }
        }

        println!("Elf's max calories: {}", max_calories);
    }
}

fn part2() {
    if let Ok(lines) = read_lines("./src/input2.txt") {
        let mut current_elf_calories: i32 = 0;

        let mut elfs = Vec::<i32>::new();

        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    elfs.push(current_elf_calories);
                    current_elf_calories = 0;
                    continue;
                }

                current_elf_calories += ip.parse::<i32>().unwrap();
            }
        }
        
        if current_elf_calories > 0 {
            elfs.push(current_elf_calories);
        }

        elfs.sort_unstable();

        // println!("{:?}", elfs);

        let total: i32 = elfs.iter().rev().take(3).sum();


        println!("Elf's max calories: {}", total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

// Inefficient version of read lines

// fn main_bad() {
//     // Store the interator of lines of the file in lines variable.
//     let lines = read_lines_bad("./src/input.txt".to_string());
//     // Iterate over the lines of the file, and in this case print them.
//     for line in lines {
//         println!("{}", line.unwrap());
//     }
// }

// fn read_lines_bad(filename: String) -> io::Lines<BufReader<File>> {
//     // Open the file in read-only mode.
//     let my_file = File::open(filename).unwrap();
//     // Read the file line by line, and return an iterator of lines of the file.
//     return io::BufReader::new(my_file).lines();
// }
