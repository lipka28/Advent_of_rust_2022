use std::env;
use std::fs;

fn read_file_from_env() -> String {
    let file_path = env::args().last().expect("No filepath specified");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

fn get_range(sorted_vec: &Vec<u32>) -> u32 {
    sorted_vec.last().expect("Empty") - sorted_vec.first().expect("Empty")
}

fn is_fully_contained_in(vec_small: &Vec<u32>, vec_big: &Vec<u32>) -> u32 {
    if vec_small[0] >= vec_big[0] && vec_small[1] <= vec_big[1] {
        return 1;
    }
    0
}

fn is_partialy_contained_in(vec_small: &Vec<u32>, vec_big: &Vec<u32>) -> u32 {
    if (vec_small[0] <= vec_big[1] && vec_small[0] >= vec_big[0])
        || (vec_small[1] >= vec_big[0] && vec_small[1] <= vec_big[1])
    {
        return 1;
    }
    0
}

fn solutions(input_file: &String) -> (u32, u32) {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in input_file.lines() {
        let splited_vecs: Vec<&str> = line.split(',').collect();

        let vec_range1: Vec<u32> = splited_vecs[0]
            .split('-')
            .map(|num| num.parse::<u32>().expect("Not an int"))
            .collect();

        let vec_range2: Vec<u32> = splited_vecs[1]
            .split('-')
            .map(|num| num.parse::<u32>().expect("Not an int"))
            .collect();

        if get_range(&vec_range1) <= get_range(&vec_range2) {
            sum1 += is_fully_contained_in(&vec_range1, &vec_range2);
            sum2 += is_partialy_contained_in(&vec_range1, &vec_range2);
        } else {
            sum1 += is_fully_contained_in(&vec_range2, &vec_range1);
            sum2 += is_partialy_contained_in(&vec_range2, &vec_range1);
        }
    }
    (sum1, sum2)
}

fn main() -> Result<(), String> {
    let input_file = read_file_from_env();
    let (sol1, sol2) = solutions(&input_file);

    print!("Solution1: {}\n", sol1);
    print!("Solution2: {}\n", sol2);

    Ok(())
}
