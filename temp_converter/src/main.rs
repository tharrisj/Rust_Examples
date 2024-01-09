use std::io;

const F2C_RATIO: f64 = 5_f64/9_f64;
const C2F_RATIO: f64 = 9_f64/5_f64;
const F2C_C2F_DIFF: f64 = 32_f64;

fn convert_f_to_c() {
    let output: &str = "Please enter the value you would like to convert to Celcius: ";
    let farenheit = get_input(output);

    let celcius = f_to_c(farenheit);
    println!("{} in Farenheit is {} in Celcius.", farenheit, celcius);
}

// convert from farenheit to celcius
fn f_to_c(faren: f64) -> f64 {
    (faren - F2C_C2F_DIFF) * F2C_RATIO
}

fn convert_c_to_f() {
    let output: &str = "Please enter the value you would like to convert to Farenheit: ";
    let celcius = get_input(output);

    let farenheit = c_to_f(celcius);
    println!("{} in Celcius is {} in Farenheit", celcius, farenheit);
}

// convert from celcius to farenheit
fn c_to_f(celc: f64) -> f64 {
    (celc * C2F_RATIO) + F2C_C2F_DIFF
}

fn get_input(output: &str) -> f64 {
    println!("{}", output);
    let mut ret: String = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line!");

    let ret: f64 = ret.trim().parse().expect("Couldn't convert input to float!");
    ret
}

fn main() {
    println!("Please enter your type to convert from and to:");
    println!("  1. Celcius to Farenheit");
    println!("  2. Farenheit to Celcius");

    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line!");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Couldn't convert choice to integer! PANIC!");
        }
    };

    match choice {
        1_u32 => convert_c_to_f(),
        2_u32 => convert_f_to_c(),
        _ => {
            println!("Invalid choice. Exiting");
        }
    }

    println!("Finished!");
}
