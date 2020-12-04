use std::fs;

fn main() {
    println!("Advent of Code: Day 2, Challenge 1");

    let contents = fs::read_to_string("input_2.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|x| x.parse::<String>().unwrap()).collect();

    let mut valid_passwords = 0;

    for line in lines.to_vec() {
        if check_password(String::from( line )) {
            valid_passwords += 1;
        }
    }

    println!("Out of {} passwords, {} are valid.", lines.len(), valid_passwords);


}

fn check_password(line: String) -> bool {
    // lines come in the form -> 'min-max character: password'
    let elements: Vec<String> = line.split(':').map(|x| String::from(x.trim())).collect::<Vec<String>>();
    let password = &elements[1];

    let values: Vec<String> = elements[0].split(' ').map(String::from).collect::<Vec<String>>();
    let character = values[1].parse::<char>().unwrap();

    let max_min: Vec<String> = values[0].split('-').map(String::from).collect::<Vec<String>>();
    let min = max_min[0].parse::<u32>().unwrap();
    let max = max_min[1].parse::<u32>().unwrap();
    
    // println!("min: {}", min);
    // println!("max: {}", max);
    // println!("char: {}", character);
    // println!("password: {}", password);

    let mut char_count = 0;
    for c in password.chars() {
        if c == character {
            char_count += 1;
        }
    }
    
    if char_count >= min && char_count <= max {
        return true
    }
    else {
        return false
    }
}