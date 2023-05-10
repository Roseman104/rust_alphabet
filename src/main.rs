use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

fn main() {
    let start: Instant = Instant::now();
    loop_alphabet();
    let duration: Duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

//set a loop to increment the alphabet from a to z of a given length.
fn loop_alphabet() {
    let strings = increment_alphabet(3);

    // Open the file for writing
    let mut file = File::create("output.txt").expect("Failed to create file");

    for s in strings {
        println!("{}", s);
        writeln!(file, "{}", s).expect("Failed to write to file");
    }
}

//function to produce a vector of strings of length n containing all possible combinations of the alphabet
fn increment_alphabet(length: usize) -> Vec<String> {
    let mut results = vec!["".to_string()];
    for _ in 0..length {
        let mut new_results = Vec::new();
        for s in results {
            for c in b'a'..=b'z' {
                let mut new_string = s.clone();
                new_string.push(c as char);
                new_results.push(new_string);
            }
        }
        results = new_results;
    }
    results
}
