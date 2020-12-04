use std::fs;

fn main() {
    println!("Advent of Code: Day 2, Challenge 2");

    let contents = fs::read_to_string("input_2.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|x| x.parse::<String>().unwrap()).collect();


    assert_eq!(check_password(String::from("1-3 a: abcd")), true);
    assert_eq!(check_password(String::from("1-3 a: abad")), false);
    assert_eq!(check_password(String::from("1-3 a: cbcd")), false);

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
    let password: Vec<char> = elements[1].chars().collect();

    let values: Vec<String> = elements[0].split(' ').map(String::from).collect::<Vec<String>>();
    let character = values[1].parse::<char>().unwrap();

    let pos1_pos2: Vec<String> = values[0].split('-').map(String::from).collect::<Vec<String>>();
    let pos1 = pos1_pos2[0].parse::<usize>().unwrap();
    let pos2 = pos1_pos2[1].parse::<usize>().unwrap();
    
    // println!("pos1: {}", pos1);
    // println!("pos2: {}", pos2);
    // println!("char: {}", character);
    // println!("password: {:?}", password);

    let mut at_pos1 = false;
    let mut at_pos2 = false;
    for c in 0..password.len() {
        if password[c] == character {
            if c == pos1 - 1 {
                at_pos1 = true;
            }
            if c == pos2 - 1 {
                at_pos2 = true;
            }
        }
    }

    // println!("at_pos1: {}", at_pos1);
    // println!("at_pos2: {}", at_pos2);
    
    if (at_pos1 || at_pos2) && (at_pos1 != at_pos2) {
        // println!("-> true");
        return true
    }
    else {
        // println!("-> false");
        return false
    }
}