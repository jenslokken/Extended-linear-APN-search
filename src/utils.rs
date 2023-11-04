
pub fn is_linear(f: &Vec<u32>, dimension: u32) -> bool {
    // Checks if a function is linear
    let size = 1 << dimension;
    for x in 0..size {
        for y in 0..size {
            if f[x^y] != f[x] ^ f[y] {
                return false;
            }
        }
    }
    return true;
}

pub fn is_affine(f: &Vec<u32>, dimension: u32) -> bool {
    // Checks if a function is affine by using the fact that a linear function is affine
    // Add the zero element to every element in the truth table and check if the new function is linear
    let size = 1 << dimension;
    let mut new_f = Vec::new();
    for i in 0..size {
        new_f.push(f[0] ^ f[i])
    }
    return is_linear(&new_f, dimension);
}

pub fn span_truth_table(basis_element: u32, truth_table: &mut Vec<u32>) {
    // Spans the truth table of the basis element given a truth table
    for i in 0..truth_table.len() {
        truth_table.push(truth_table[i] ^ basis_element);
    }
}

pub fn create_tt(basis_elements: &Vec<u32>) -> Vec<u32> {
    // Creates a truth table from a list of basis elements
    let mut truth_table = vec![0];
    for i in 0..basis_elements.len() {
        span_truth_table(basis_elements[i], &mut truth_table);
    }
    return truth_table;
}

pub fn compose(f: &Vec<u32>, g: &Vec<u32>) -> Vec<u32> {
    // Composes function f with function g and returns the result

    let mut h = Vec::new();
    for i in 0..f.len() {
        h.push(f[g[i] as usize]);
    }
    return h;
}

pub fn inverse(f: &Vec<u32>) -> Vec<u32> {
    // Returns the inverse of a function
    let size = f.len();
    let mut inv = vec![0;size];
    for i in 0..size {
        inv[f[i] as usize] = i as u32;
    }
    inv
}

pub fn sum(f: &Vec<u32>, g: &Vec<u32>) -> Vec<u32> {
    // Returns the sum of two functions
    let mut h = Vec::new();
    for i in 0..f.len() {
        h.push(f[i]^g[i]);
    }
    return h;
}

pub fn is_bijective(f: &Vec<u32>) -> bool {
    // Checks if a function is bijective by checking if every element is mapped to a unique element
    let mut seen = vec![false;f.len()];
    for i in 0..f.len() {
        if seen[f[i] as usize] {
            return false;
        }
        seen[f[i] as usize] = true;
    }
    return true;
}

pub fn is_zero_vector(f: &Vec<u32>) -> bool {
    // Checks if a vector is the zero vector by iterating over every element and checking if it is zero
    for i in 0..f.len() {
        if f[i] != 0 {
            return false;
        }
    }
    return true;
}