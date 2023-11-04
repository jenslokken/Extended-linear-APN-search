use std::fs;
use rayon::prelude::*;

pub fn is_quad(path: &str) {
    let tts = parse_input(path);
    let _tts2: Vec<bool> = tts.par_chunks_exact(64)
    .map(|chunk| is_quadratic(chunk)).collect();
}

fn is_quadratic(f: &[u32]) -> bool {
    let size = f.len();
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                if f[0] ^ f[i] ^ f[j] ^ f[k] ^ f[i^j] ^ f[i ^ k] ^ f[j ^ k] ^ f[i ^ j ^ k] != 0 {
                    return false;
                }
            }
        }
    }
    println!("{:?}", f);
    true
}


fn parse_input(path: &str) -> Vec<u32> {
    let numbers: String = fs::read_to_string(path)
        .expect("File not found");
    let numbers: Vec<u32> = numbers
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid number in the file"))
        .collect();
    numbers
}