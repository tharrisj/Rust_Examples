// Sums all odd numbers from 1 to limit given.
fn sum_odd_numbers(limit: u32) -> u32 {
    let mut accumulator: u32 = 0;

    for i in 1..=limit {
        accumulator += match i%2==1 {
            true => i,
            false => continue,
        }
    }
    accumulator
}

fn main() {
    println!("Sum of all odd numbers from 1 to 9: {}", sum_odd_numbers(9));
}