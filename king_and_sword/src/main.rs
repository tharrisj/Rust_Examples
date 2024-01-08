mod kns_vec;
mod kns_binary;

use std::io;
use std::time::Instant;

fn get_input(output: &str) -> u32 {
    println!("{}", output);
    let mut ret: String = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line!");

    let ret: u32 = ret.trim().parse().expect("Couldn't convert input to float!");
    ret
}

fn print_scenario() {
    println!("Everyone should know how this goes. A king wants his wisest knight to take over the kingdom and will \
    only give the kingdom if they pass a test. In order to see who is the wisest (and to weed out all other \
    possible rivals) the king will arrange his knights in a line. The first knight will kill the second and \
    give the sword to the third who will kill the fourth. This will continue on (with the final knight in \
    the line acting as if the first knight is next to him). Figure out where to stand in line in order to \
    be the last knight standing and inherit the kingdom! \
    ");
}

fn main() {
    print_scenario();
    let number_of_knights: u32 = get_input("Please input the number of knights you'd like to calculate:");

    let start = Instant::now();
    let vec_surv: u32 = kns_vec::get_last_man_standing(number_of_knights);
    let duration = start.elapsed();
    println!("Time elapsed for Vector calculation is: {:?}", duration);

    let start = Instant::now();
    let bin_surv: u32 = kns_binary::get_last_man_standing(number_of_knights);
    let duration = start.elapsed();
    println!("Time elapsed for Binary manipulation is: {:?}", duration);

    println!("Vector calculation result survivor is: {}", vec_surv);
    println!("Binary manipulation result survivor is: {}", bin_surv);
    println!("Done!");
}
