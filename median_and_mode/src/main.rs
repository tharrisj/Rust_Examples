use rand::{thread_rng, Rng};
use std::collections::HashMap;

const VEC_SIZE: u32 = 10_u32;

fn get_mode(v: &Vec<i32>) -> i32 {
    let mut num_freq = HashMap::new();
    for num in v {
        let element = num_freq.entry(num).or_insert(0);
        *element+= 1;
    }
    let mut max_key:i32 = 0;
    let mut max_value:i32 = 0;
    for (key, value) in num_freq {
        if value > max_value {
            max_value = value;
            max_key = *key;
        }
    }
    max_key
}

fn main() {
    println!("Generating a random vector");
    let mut rand_nums: Vec<i32> = Vec::new();

    // Fill rand_nums with random numbers between -100 and 100
    for _ in 0..VEC_SIZE {
        rand_nums.push(thread_rng().gen_range(-100..=100));
    }
    rand_nums.push(0_i32);
    rand_nums.push(0_i32);
    println!("Unsorted vector is: {:?}", rand_nums);
    rand_nums.sort_unstable();
    println!("Sorted vector is: {:?}", rand_nums);

    let median: i32 = rand_nums[rand_nums.len() / 2 - 1];
    let mode: i32 = get_mode(&rand_nums);

    println!("Median is: {}", median);
    println!("Mode is: {}", mode);
}
