use std::fs;

fn main() {
    println!("Advent of Code: Day 4, Challenge 1");

    let contents = fs::read_to_string("input_4.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|x| x.parse::<String>().unwrap()).collect();

    let mut passports: Vec<String> = Vec::new();
    let mut tmp_passport: String = String::new();
    let mut num_valid = 0;

    for line in lines.to_vec() {
        if line != "" {
            tmp_passport = tmp_passport.clone() + " " + &line;
        }
        else {
            passports.push( tmp_passport.clone() );
            tmp_passport.clear();
        }
    }
    passports.push( tmp_passport.clone() );
    tmp_passport.clear();

    for pspt in passports {
        if check_passport( pspt ) {
            num_valid += 1;
        }
    }

    println!("valid passports: {}",num_valid);
}

fn check_passport( pspt: String ) -> bool {
    let mut byr = false; // birth year
    let mut iyr = false; // issue year
    let mut eyr = false; // expiration year
    let mut hgt = false; // height
    let mut hcl = false; // hair color
    let mut ecl = false; // eye color
    let mut pid = false; // passport id

    if pspt.contains("byr:") { byr = true; }
    if pspt.contains("iyr:") { iyr = true; }
    if pspt.contains("eyr:") { eyr = true; }
    if pspt.contains("hgt:") { hgt = true; }
    if pspt.contains("hcl:") { hcl = true; }
    if pspt.contains("ecl:") { ecl = true; }
    if pspt.contains("pid:") { pid = true; }

    if byr & iyr & eyr & hgt & hcl & ecl & pid {
        return true
    }
    else {
        return false
    }
}