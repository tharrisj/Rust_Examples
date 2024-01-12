const VOWELS: [char; 10] = ['a','e','i','o','u', 'A', 'E', 'I', 'O', 'U'];

fn pig_latinize(s: &str) -> String {
    let mut ret: String = String::new();
    for word in s.split(" ") {
        let chars: Vec<char> = word.chars().collect();
//        if chars.starts_with(&VOWELS) {
        if VOWELS.contains(&chars[0]) {
            ret.push_str(word);
            ret.push_str("-hay");
        } else {
            let add_consonant = chars[0];
            let add_chars: Vec<char> = chars[1..].to_vec();
            ret+= &add_chars.into_iter().collect::<String>();
            ret.push('-');
            ret.push(add_consonant);
            ret.push_str("ay");
        }
        ret.push(' ');
    }

    ret
}

fn main() {
    let test_string: String = "The quick brown Fox jumped over the lazy Dog".to_owned();
    let ret_string: String = pig_latinize(&test_string);

    println!("test string:   {}", test_string);
    println!("pig string:    {}", ret_string);
}
