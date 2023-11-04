use std::fs;

pub fn parse_input(path: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    // Parses the input file and returns a tuple with a vector of f, l1, and a, and a vector of cycles
    let content = fs::read_to_string(path).expect("File not found");

    let mut result = content
        .lines()
        .into_iter();

    let parse_line = |line:&str| line.split_whitespace().map(|num| num.parse::<u32>().expect("Invalid number in the file")).collect::<Vec<u32>>();
    let mut f_l_a = Vec::new();
    for _ in 0..3 {
        let line = result.next().expect("Invalid file format");
        f_l_a.push(parse_line(line));
    }
    let cycles = result.next()
    .expect("Cycles in invalid format")
    .split(";")
    .map(|cycle| parse_line(cycle))
    .collect();
    (f_l_a, cycles)
}

pub fn read_file(path: &str) -> (Vec<Vec<Vec<usize>>>, Vec<usize>){
    let all_quads: String = fs::read_to_string(path).expect("Usage: ./explore-new-functions zero_quads");
    let mut contradictions: Vec<Vec<Vec<usize>>> = Vec::new();
    let mut all_quads = all_quads.lines().peekable();
    let mut indices: Vec<usize> = Vec::new();
    while let Some(line) = all_quads.next() {
        if all_quads.peek().is_none() {
            indices = line.split_whitespace().map(|num| num.parse::<usize>().expect("Could not parse nums in guessing order.")).collect();
            break;
        }
        let mut quad: Vec<Vec<usize>> = Vec::new();
        for quads in line.split(";") {
            let nums: Vec<usize> = quads.split_whitespace().map(|num| num.parse::<usize>().expect("Could not parse int.")).collect();
            quad.push(nums);
        }
        contradictions.push(quad);
    }
    (contradictions, indices)
}
