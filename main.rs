use rand::{thread_rng, Rng};
use rand::distributions::{Uniform};
use std::time::SystemTime;
// use crossterm::{terminal, ClearType};
use std::io;
// use crossterm::terminal::ClearType::CurrentLine;

const LINE_MAX: u8 = 64;

fn main() {
    let rng = thread_rng();

    let mut size = String::new();

    io::stdin().read_line(&mut size)
        .expect("Failed to read line");

    let size: usize = match size.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Bad input parsing"),
    };


    // get time stamp
    let start_time = SystemTime::now();

    let mut weights_vector: Vec<u8> = rng.sample_iter(Uniform::from(1..=LINE_MAX)).take(size).collect(); 
    // let mut weights_vector: Vec<u8> = vec![50, 57, 63, 23, 28, 35, 44, 8, 53, 60, 2, 64, 31, 58, 11];
    println!("Weights_vector is:\n{:?}", weights_vector);

    let optimal_used_lines = bin_pack_wrapper(&mut weights_vector);

    // get time difference
    let end_time = SystemTime::now();
    let difference = end_time.duration_since(start_time)
                         .expect("SystemTime::duration_since failed");
    println!("Found solution runtime: {:?}", difference);
    println!("Found solution lines: {}", optimal_used_lines.len());
    println!("Found solution:\n{:?}", optimal_used_lines);
    
}

fn bin_pack_wrapper(weights_vector: &mut Vec<u8>) -> Vec<Vec<u8>> {
    
    // sort the weights vecotr, and reverse its order so item addition begins from larger to smaller
    weights_vector.sort();
    weights_vector.reverse();

    // initilize an empty array for to be filled by global minimum
    let mut bin_solution_minimum: Vec<Vec<u8>>  = Vec::new();
    bin_solution_minimum.push(Vec::new());

    // initilize an empty array for to be filled by iterative minimum
    let mut bin_solution_initial = bin_solution_minimum.clone();

    // create a new weights vector, without the item that was added to the current bin
    let mut weights_vector_new = weights_vector.clone();
    weights_vector_new.remove(0);

    // calculate the remaining space in the bin
    let space_left = LINE_MAX - weights_vector[0];

    // add the biggest item first
    bin_solution_initial[0].push(weights_vector[0]);

    // add the rest of the items
    let bin_solution_iter = add_items(weights_vector_new, 0, space_left, bin_solution_initial);

    // change the input bin solution input to global minimum that was found
    return bin_solution_iter;
}

fn add_items(weights_vector: Vec<u8>, current_bin: usize, current_left: u8, bin_solution: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
     
    // recursive stop condition
    if weights_vector.len() == 0 {
        return bin_solution;
    }

    // initilize the first mimumn solution to the worst case
    let mut bin_solution_minimum = bin_solution.clone();
    let mut bin_solution_minimum_cnt = weights_vector.len() + bin_solution.len();

    let mut bin_solution_minimum_limit = 0.0;
    for item_weight in weights_vector.iter(){
        bin_solution_minimum_limit += *item_weight as f32 / LINE_MAX as f32;
    }
    bin_solution_minimum_limit = bin_solution_minimum_limit + bin_solution.len() as f32;

    for (idx, item_weight) in weights_vector.iter().enumerate() {
        let mut current_bin_iter = current_bin;
        let mut bin_solution_iter = bin_solution.clone(); // TODO something more memory effecient

        // create a new weights vector, without the item that was added to the current bin
        let mut weights_vector_new = weights_vector.clone();
        weights_vector_new.remove(idx);
        
        // if the item has room in the current bin, add it and add the rest of the items
        let space_left: u8;
        if *item_weight <= current_left {
            space_left = current_left - *item_weight;
        }
        else {
            space_left = LINE_MAX - *item_weight;
            current_bin_iter = current_bin_iter + 1;
            bin_solution_iter.push(Vec::new());
        };

        // add the item to the current bin 
        bin_solution_iter[current_bin_iter].push(*item_weight);

        // add the rest of the items
        let iter_solution = add_items(weights_vector_new, current_bin_iter, space_left, bin_solution_iter);

        if iter_solution.len() <= bin_solution_minimum_cnt {
            bin_solution_minimum = iter_solution.clone();
            bin_solution_minimum_cnt = iter_solution.len();
        }

        if bin_solution_minimum_cnt as f32 <= bin_solution_minimum_limit {
            return bin_solution_minimum;
        }
    }

    // change the input bin solution input to global minimum that was found
    return bin_solution_minimum;
}
