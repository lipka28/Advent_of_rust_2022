use std::env;
use std::fs;

const FILE_SYSTEM_SIZE: u32 = 70000000;
const NEEDED_SIZE: u32 = 30000000;

#[derive(Debug, Clone)]
struct Folder {
    name: String,
    size: u32,
}

fn read_file_from_env() -> String {
    let file_path = env::args().last().expect("No filepath specified");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

fn generate_file_structure(input: String) -> Vec<Folder> {
    let mut folder_list: Vec<Folder> = Vec::new();
    let mut curr_dir: Vec<Folder> = Vec::new();

    for line in input.lines() {
        let command = line.split(' ').collect::<Vec<_>>();

        if command[0] != "$" && command[0] != "dir" {
            let size = command[0].parse::<u32>().expect("Not an int");
            curr_dir.last_mut().expect("No current folder").size += size;
        }
        if command[1] == "cd" {
            if command[2] == ".." {
                let curr = curr_dir.pop().expect("No current directory");
                let size = curr.size;
                curr_dir.last_mut().expect("No current folder").size += size;

                folder_list.push(curr);
            } else if command[2] == "/" {
                let folder = Folder {
                    name: command[2].to_string(),
                    size: 0,
                };

                curr_dir.push(folder);
            } else {
                let prev = curr_dir.last().expect("No current directory");
                let mut name = prev.name.clone();
                name.push_str(&format!("{}/", command[2]));

                let folder = Folder { name, size: 0 };

                curr_dir.push(folder);
            }
        }
    }
    for i in (0..curr_dir.len()).rev() {
        if i > 0 {
            let curr = curr_dir.pop().expect("No current directory");
            let size = curr.size;
            curr_dir.last_mut().expect("No current folder").size += size;

            folder_list.push(curr);
        } else {
            let curr = curr_dir.pop().expect("No current directory");
            folder_list.push(curr);
        }
    }

    folder_list
}

fn solution1(folder_list: &Vec<Folder>) -> u32 {
    let mut sum = 0;
    for folder in folder_list {
        if folder.size <= 100000 {
            sum += folder.size;
        }
    }
    sum
}

fn solution2(folder_list: &Vec<Folder>) -> u32 {
    let unsued_space = FILE_SYSTEM_SIZE - folder_list.last().unwrap().size;
    let needed_size = NEEDED_SIZE - unsued_space;
    let mut candidates: Vec<u32> = Vec::new();
    for folder in folder_list {
        if folder.size >= needed_size {
            candidates.push(folder.size);
        }
    }
    *candidates.iter().min().expect("No value")
}

fn main() -> Result<(), String> {
    let input_file = read_file_from_env();
    let folder_list = generate_file_structure(input_file);

    print!("Result 1: {}\n", solution1(&folder_list));
    print!("Result 2: {}\n", solution2(&folder_list));

    Ok(())
}
