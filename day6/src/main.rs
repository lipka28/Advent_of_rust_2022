use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::Hash;

const OFFSET: usize = 4;
const OFFSET_BIG: usize = 14;

fn read_file_from_env() -> String {
    let file_path = env::args().last().expect("No filepath specified");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn solution(input_signal: &str, offset: usize) -> usize {
    let mut from_start = 0;
    let sig_vec = input_signal.chars().collect::<Vec<_>>();
    for _ in &sig_vec {
        if from_start >= offset {
            let sample = &sig_vec[from_start - offset..from_start];
            if has_unique_elements(sample.iter()) {
                break;
            }
        }
        from_start += 1;
    }
    from_start
}

fn main() -> Result<(), String> {
    let input_file = read_file_from_env();
    let sol1 = solution(&input_file, OFFSET);
    let sol2 = solution(&input_file, OFFSET_BIG);

    print!("Solution1: {}\n", sol1);
    print!("Solution2: {}\n", sol2);

    Ok(())
}
