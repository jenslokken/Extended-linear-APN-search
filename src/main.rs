use std::env;
use std::collections::HashMap;
mod parse;
mod utils;
mod is_quad;
mod generate_contradictions;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1);
    
    if file_path.is_none() {
         println!("Please provide a file with f, l1, a and cycles");
         return;
    } 
    let f_l_a = parse::parse_input(file_path.unwrap());
    let f = f_l_a.0[0].clone();
    let l1 = f_l_a.0[1].clone();
    let a = f_l_a.0[2].clone();
    let cycles = f_l_a.1.clone();
    drop(f_l_a);
    
    let (contradictions, guessing_order);
    let third_arg = args.get(1);
    if third_arg.is_some() {
        let max_thread = third_arg
            .unwrap()
            .to_owned()
            .parse::<usize>()
            .unwrap();
        rayon::ThreadPoolBuilder::new().num_threads(max_thread).build_global().unwrap();
    }
    let second_arg = args.get(2);
    match second_arg.map(|s| s.as_str()) {
        Some("--is_quad") => {
            is_quad::is_quad(args.get(3).unwrap());
            return;
        },
        Some("--gen_con") => {
            generate_contradictions::generate_contradictions(&cycles, true);
            return;
        },
        Some("--con") => {
            (contradictions, guessing_order) = parse::read_file(args.get(3).unwrap());
        }
        _ => (contradictions, guessing_order) = generate_contradictions::generate_contradictions(&cycles, false).expect("No contradictions found"),
    }
     
    println!("Guessing order: {:?}", guessing_order);
    println!("cycles: {:?}", cycles);
    let l1_inv = utils::inverse(&l1);
    let size = f.len();
    
    let new_f: Vec<u32> = vec![0; size]; // initialize unset values to 0
    
    search(size, f, &cycles, 0, guessing_order, &l1_inv, &a, &contradictions, true)
    // reversed_search(size, f, &cycles, guessing_order, &l1_inv, &a, &contradictions);
    
}

fn generate_from_cycle(
    // Generates all the values in the cycle from the guessed value
    cycle: &Vec<u32>, 
    l1_inv: &Vec<u32>, 
    a: &Vec<u32>,
    mut guessed_value: u32,
) -> HashMap<u32, u32> {
    let mut element = cycle.get(0).expect("Empty cycle!");
    let mut index_map:HashMap<u32, u32> = HashMap::new();
    index_map.insert(*element, guessed_value);
    for i in 1..cycle.len() {
        guessed_value = l1_inv[(a[*element as usize] ^ guessed_value) as usize];
        element = cycle.get(i).expect("Something wrong with the cycle length!");
        index_map.insert(*element, guessed_value);
    }
    index_map
}

fn validate(f: &Vec<u32>, quadruples: &Vec<Vec<usize>>) -> bool {
    // Returns false if the function is linear
    for quad in quadruples {
        if quad.len() == 0 {
            continue;
        }
        if f[quad[0]] ^ f[quad[1]] ^ f[quad[2]] ^ f[quad[3]] == 0{
            return false;
        }
    }
    true
}

fn get_next_values(
    idx: usize,
    cycle: &Vec<u32>,
    contradictions: &Vec<Vec<Vec<usize>>>,
    mut new_f: Vec<u32>,
    l1_inv: &Vec<u32>,
    a: &Vec<u32>,
    guessed_value: u32,
) -> Option<Vec<u32>> {
    let generated_values = generate_from_cycle(cycle, l1_inv, a, guessed_value);
    generated_values.into_iter().for_each(|(key, value)| new_f[key as usize] = value);
    if !validate(&new_f, contradictions.get(idx).expect("Something wrong with the contradictions!")) {
        return None;
    }
    Some(new_f)
}

fn search(
    size: usize, 
    new_f: Vec<u32>, 
    cycles: &Vec<Vec<u32>>, 
    idx: usize, 
    guessing_order: Vec<usize>,
    l1_inv: &Vec<u32>,
    a: &Vec<u32>,
    contradictions: &Vec<Vec<Vec<usize>>>,
    reverse: bool,
) {

    if idx == guessing_order.len() {
        println!("{:?}", new_f);
        return;
    }
    if let Some(cycle) = cycles.get(guessing_order[idx]) {
        (0..size as u32).into_par_iter().for_each(|i| {
            if let Some(f) = get_next_values(idx, cycle, contradictions, new_f.clone(), l1_inv, a, i) {
                if !reverse || (i != cycle[0]) {
                    search(size, f, cycles, idx + 1, guessing_order.clone(), l1_inv, a, contradictions, false);
                }
            }
        });
    }
    if reverse && idx > 0 {
        search(size, new_f, cycles, idx - 1, guessing_order, l1_inv, a, contradictions, reverse);
    }
}

fn _reversed_search( 
    size: usize, 
    new_f: Vec<u32>, 
    cycles: &Vec<Vec<u32>>, 
    guessing_order: Vec<usize>,
    l1_inv: &Vec<u32>,
    a: &Vec<u32>,
    contradictions: &Vec<Vec<Vec<usize>>>,
){
    
    (0..guessing_order.len())
        .into_par_iter()
        .rev()
        .for_each(|i| search(size, new_f.clone(), cycles, i, guessing_order.clone(), l1_inv, a, contradictions, false));
}
#[cfg(test)]
mod generate_from_cycle_tests {
    use super::*;

    #[test]
    fn get_correct_answer() {
        let cycle: Vec<u32> = vec![1, 50, 6, 44, 38, 4, 59];
        let l1_inv: Vec<u32> = vec![0, 23, 30, 9, 22, 1, 8, 31, 38, 49, 56, 47, 48, 39, 46, 57, 24, 15, 6, 17, 14, 25, 16, 7, 62, 41, 32, 55, 40, 63, 54, 33, 61, 42, 35, 52, 43, 60, 53, 34, 27, 12, 5, 18, 13, 26, 19, 4, 37, 50, 59, 44, 51, 36, 45, 58, 3, 20, 29, 10, 21, 2, 11, 28];
        let a:Vec<u32> = vec![0, 27, 35, 56, 53, 46, 22, 13, 54, 45, 21, 14, 3, 24, 32, 59, 10, 17, 41, 50, 63, 36, 28, 7, 60, 39, 31, 4, 9, 18, 42, 49, 60, 39, 31, 4, 9, 18, 42, 49, 10, 17, 41, 50, 63, 36, 28, 7, 54, 45, 21, 14, 3, 24, 32, 59, 0, 27, 35, 56, 53, 46, 22, 13];
        let guessed_value:u32 = 53;
        let return_value = generate_from_cycle(&cycle, &l1_inv, &a, guessed_value);
        let expected_values: [(u32, u32); 7] = [(1, 53), (50, 19), (6, 8), (44, 54), (38, 49), (4, 55), (59, 30)];
        for (key, value) in expected_values.iter() {
            assert_eq!(return_value.get(key).expect("Key not found!"), value);
        }
    }
}
