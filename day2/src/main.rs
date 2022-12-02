use std::env;
use std::fs;

fn read_file_from_env() -> String {
    let file_path = env::args().last().expect("No filepath specified");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents
}

// solution 1
fn calculate_score_1(round: &str) -> i32 {
    match round {
        "A X" => return 4,
        "A Y" => return 8,
        "A Z" => return 3,
        "B X" => return 1,
        "B Y" => return 5,
        "B Z" => return 9,
        "C X" => return 7,
        "C Y" => return 2,
        "C Z" => return 6,
        _ => return 0,
    }
}

// solution 2
fn calculate_score_2(round: &str) -> i32 {
    match round {
        "A X" => return 3,
        "A Y" => return 4,
        "A Z" => return 8,
        "B X" => return 1,
        "B Y" => return 5,
        "B Z" => return 9,
        "C X" => return 2,
        "C Y" => return 6,
        "C Z" => return 7,
        _ => return 0,
    }
}

fn main() -> Result<(), String> {
    let input_file = read_file_from_env();
    let mut score1 = 0;
    let mut score2 = 0;

    // Solution 1
    for line in input_file.lines() {
        score1 += calculate_score_1(line);
        score2 += calculate_score_2(line);
    }
    
    print!("My score, by method 1, is: {} points\n", score1);
    print!("My score, by method 2, is: {} points\n", score2);

    Ok(())
}