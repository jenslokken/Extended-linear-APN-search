use rayon::prelude::*;

fn generate_quads(cycle: &Vec<u32>, tuples: &Vec<Vec<(u32, u32, u32, u32)>>) -> Vec<(u32, u32, u32, u32)> {
    let mut output: Vec<(u32, u32, u32, u32)> = Vec::new();
    let mut cycle = cycle.clone();
    cycle.push(0);
    cycle.sort_unstable();
    if cycle.len() < 4 {
        return output;
    }
    let size = cycle.len();
    output.par_extend((0..size - 3).into_par_iter().flat_map(move |i| {
        let cycle_clone = cycle.clone();
        (i + 1..size - 2).into_par_iter().flat_map(move |j| {
            let cycle_clone = cycle_clone.clone();
            (j + 1..size - 1).into_par_iter().flat_map(move |k| {
                let cycle_clone = cycle_clone.clone();
                let triple = cycle_clone[i] ^ cycle_clone[j] ^ cycle_clone[k];
                ((k)..size).into_par_iter().filter_map(move |l| {
                        if triple != cycle_clone[l] {
                            return None;
                        } 
                        let quad:(u32, u32, u32, u32) = (cycle_clone[i], cycle_clone[j], cycle_clone[k], cycle_clone[l]);
                        if tuples.par_iter().any(|comb| comb.contains(&quad)) {
                            return None;
                        }
                    Some(quad)
                })
            })
        })
    }));
    output
}
fn search(cycles: &Vec<Vec<u32>>, last_combinations: &Vec<u32>, i: usize, idx_list: &mut Vec<usize>, output_list: &mut Vec<Vec<(u32, u32, u32, u32)>>, stop: usize) -> (Vec<Vec<(u32, u32, u32, u32)>>, Vec<usize>) {
    if i == stop {
        return (output_list.clone(), idx_list.clone());
    }
    let mut tuples: Vec<Vec<(u32, u32, u32, u32)>> = Vec::new();
    let mut new_cycles: Vec<Vec<u32>> = Vec::new();
    if i == 0 {
        new_cycles = cycles.clone();
    } else {
        for j in 0..cycles.len() {
            if idx_list.contains(&j) {
                new_cycles.push(Vec::new());
                continue;
            }
            new_cycles.push(last_combinations.iter().chain(cycles[j].iter()).copied().collect::<Vec<u32>>());
        }
    }
    for cycle in &new_cycles {
        if output_list.is_empty() {
            tuples.push(generate_quads(cycle, &Vec::new()));
        } else {
            tuples.push(generate_quads(cycle, &output_list));
        }
    }
    let mut max_length = 0;
    let mut max_idx = 0;
    for (k, t) in tuples.iter().enumerate() {
        if t.len() > max_length {
            max_length = t.len();
            max_idx = k as u32;
        }
    }
    idx_list.push(max_idx as usize);
    output_list.push(tuples[max_idx as usize].clone());
    
    let last_combination = new_cycles[max_idx as usize].clone();
    
    search(cycles, &last_combination, i + 1, idx_list, output_list, stop)
}

// Generates and prints out all possible contradictions for the cycles given. 
pub fn generate_contradictions(cycles: &Vec<Vec<u32>>, print: bool) -> Option<(Vec<Vec<Vec<usize>>>, Vec<usize>)>{
    let contradictions = search(cycles, &Vec::new(), 0, &mut Vec::new(), &mut Vec::new(), cycles.len());
    if !print {
        let mut temp_contradictions: Vec<Vec<Vec<usize>>> =  Vec::new();
        for cons in contradictions.0 {
            let mut temp_cons: Vec<Vec<usize>> = Vec::new();
            for con in cons {
                temp_cons.push(vec![con.0 as usize, con.1 as usize, con.2 as usize, con.3 as usize]);
            }
            temp_contradictions.push(temp_cons);
        }
        return Some((temp_contradictions, contradictions.1));
    }
    for contradiction in contradictions.0 {
        println!("{:?}", contradiction);
    }
    println!("{:?}", contradictions.1);
    None
}
