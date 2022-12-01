use std::env;
use std::fs;
use std::vec;

fn read_file_from_env() -> String {
    let file_path = env::args().last().expect("No filepath specified");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents
}

fn main() -> Result<(), String> {
    let input_file = read_file_from_env();
    let mut elfs = vec!(0);
    let mut elf_count = 0;

    for line in input_file.lines() {
        if !line.is_empty() {
            elfs[elf_count] += line.parse::<i32>().expect("Not an integer");
        } 
        else {
            elfs.push(0);
            elf_count += 1;
        }
    }

    elfs.sort();
    elfs.reverse();

    // Solution 1
    print!("Most candies: {}\n", elfs[0]);

    // Solution 2
    print!("Top 3 most candies: {}\n", elfs.iter().take(3).sum::<i32>());

    Ok(())
}