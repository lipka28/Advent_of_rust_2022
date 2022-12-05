use std::collections::VecDeque;
use std::env;
use std::fs;

const SECTOR_SIZE: usize = 4;

fn read_file_from_env() -> String {
    let file_path = env::args().last().expect("No filepath specified");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

fn process_data(data: Vec<&str>) -> Vec<VecDeque<String>> {
    let mut first_line = true;
    let mut processed_data: Vec<VecDeque<String>> = Vec::new();

    for line in data.iter().rev() {
        let mut _i = 0;
        let temp_line = line.chars().collect::<Vec<_>>();
        let creates = temp_line.chunks(SECTOR_SIZE).collect::<Vec<_>>();

        for create in creates {
            if first_line {
                processed_data.push(VecDeque::new());
            } else if create[1] != ' ' {
                processed_data[_i].push_back(String::from_iter(&create[0..SECTOR_SIZE - 1]));
                _i += 1;
            } else {
                _i += 1;
            }
        }
        first_line = false;
        _i = 0;
    }

    processed_data
}

fn splitout(input_file: &String) -> (Vec<&str>, Vec<&str>) {
    let mut data = Vec::new();
    let mut instructions = Vec::new();
    let mut reading_data = true;

    for line in input_file.lines() {
        if reading_data && !line.is_empty() {
            data.push(line);
        } else if !line.is_empty() {
            instructions.push(line);
        } else {
            reading_data = false
        }
    }

    (data, instructions)
}

fn apply_instructions_1(data: &mut Vec<VecDeque<String>>, instructions: &Vec<&str>) {
    for instruction in instructions {
        let instruction_split = instruction.split(' ').collect::<Vec<&str>>();
        let amount = instruction_split[1].parse::<usize>().expect("Not an int");
        let from = instruction_split[3].parse::<usize>().expect("Not an int") - 1;
        let to = instruction_split[5].parse::<usize>().expect("Not an int") - 1;

        for _ in 0..amount {
            let data_from = data[from].pop_back().expect("Empty");
            data[to].push_back(data_from);
        }
    }
}

fn apply_instructions_2(data: &mut Vec<VecDeque<String>>, instructions: &Vec<&str>) {
    print!("DAta: {:?}\n", data);
    for instruction in instructions {
        let instruction_split = instruction.split(' ').collect::<Vec<&str>>();
        let amount = instruction_split[1].parse::<usize>().expect("Not an int");
        let from = instruction_split[3].parse::<usize>().expect("Not an int") - 1;
        let to = instruction_split[5].parse::<usize>().expect("Not an int") - 1;

        let mut temp_vec = VecDeque::new();

        for _ in 0..amount {
            let data_from = data[from].pop_back().expect("Empty");
            temp_vec.push_front(data_from);
        }

        data[to].extend(temp_vec);
    }
}

fn read_tops(data: &Vec<VecDeque<String>>) {
    for col in data {
        print!(" {} ", col.back().expect("Empty"));
    }
}

fn main() -> Result<(), String> {
    let input_file = read_file_from_env();
    let (data, instructions) = splitout(&input_file);

    let mut processed_data_1 = process_data(data);
    let mut processed_data_2 = processed_data_1.clone();

    apply_instructions_1(&mut processed_data_1, &instructions);
    apply_instructions_2(&mut processed_data_2, &instructions);

    print!("Result 1:\n");
    read_tops(&processed_data_1);
    print!("\nResult 2:\n");
    read_tops(&processed_data_2);

    Ok(())
}
