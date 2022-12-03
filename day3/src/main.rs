use std::env;
use std::fs;

const LOWERCASE_OFFSET: u32 = 96;
const UPPERCASE_OFFSET: u32 = 38;

fn read_file_from_env() -> String {
    let file_path = env::args().last().expect("No filepath specified");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents
}

fn letter_to_value(letter: &char) -> u32 {
    let ascii_letter = letter.clone();
    if ascii_letter.is_lowercase() {
        ascii_letter as u32 - LOWERCASE_OFFSET
    } else {
        ascii_letter as u32 - UPPERCASE_OFFSET
    }
}

fn solution1(input_file: &String) -> u32 {
    let mut sum = 0;

    for line in input_file.lines() {
        let letters_vec = line.chars().collect::<Vec<_>>();
        let word_middle = letters_vec.len()/2;
        let comps: Vec<&[char]> = letters_vec.chunks(word_middle).collect();
        
        for c1 in comps[0] {
            if comps[1].contains(&c1) {
                sum += letter_to_value(c1);
                break
            }
        }
    }

    sum
}

fn solution2(input_file: &String) -> u32 {
    let mut sum = 0;
    let elf_groups_vec = input_file.lines().collect::<Vec<_>>();
    let elf_groups: Vec<_> = elf_groups_vec.chunks(3).collect();

    for elf_group in elf_groups {
        for c1 in elf_group[0].chars() {
            if elf_group[1].contains(c1) && elf_group[2].contains(c1) {
                sum += letter_to_value(&c1);
                break
            }
        }
    }

    sum
}

fn main() -> Result<(), String> {
    let input_file = read_file_from_env();
    let result1 = solution1(&input_file);
    let result2 = solution2(&input_file);

    print!("Result 1: {} \n", result1);
    print!("Result 2: {} \n", result2);

    Ok(())
}