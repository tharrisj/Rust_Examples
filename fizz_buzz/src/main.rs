fn main() {
    let to: u32 = 150;
    fizzbuzz_to(to);
}

fn is_divisible_by(num: u32, denom: u32) -> bool {
    if num == 0 {
        return false;
    }

    num % denom == 0
}

fn fizzbuzz(num: u32) {
    let mut strin = String::new();
    if is_divisible_by(num, 3) { strin.push_str("fizz"); }
    if is_divisible_by(num, 5) { strin.push_str("buzz"); }
    if is_divisible_by(num, 7) { strin.push_str("bazz"); }

    if strin.is_empty() {
        strin.push_str(&num.to_string());
    }
    println!("{}", strin);
}

fn fizzbuzz_to(to: u32) {
    for n in 1..=to {
        fizzbuzz(n);
    }
}