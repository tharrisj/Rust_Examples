use std::io;

const SEED_VAL_1: u32 = 0_u32;
const SEED_VAL_2: u32 = 1_u32;

fn get_input(output: &str) -> u32 {
    println!("{}", output);
    let mut ret: String = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line!");

    let ret: u32 = ret.trim().parse().expect("Couldn't convert input to integer!");
    ret
}

fn fib(nth: u32) -> u32 {
    let res: u32 = match nth {
        1_u32 => SEED_VAL_1,
        2_u32 => SEED_VAL_2,
        _ => {
            let mut prev_prev: u32 = SEED_VAL_1;
            let mut prev: u32 = SEED_VAL_2;
            let mut current: u32;

            let mut current_num: u32 = 3;
            let result = loop {
                current = prev_prev + prev;
                if current_num == nth {
                    break current;
                }
                prev_prev = prev;
                prev = current;
                current_num+= 1_u32;
            };
            result
        }
    };
    res
}

fn main() {
    let output: &str = "What number do you want to calculate the fibonacci sequence to?";
    let nth: u32 = get_input(output);

    let calc_val = fib(nth);
    println!("The {} value in the fibonacci sequence is: {}", nth, calc_val);

}
