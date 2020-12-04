use std::fs;

fn main() {
    println!("Advent of Code: Day 4, Challenge 2");

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

    let pairs: Vec<Vec<String>> = pspt.split(' ').map(|x| String::from(x).split(':').map(|x| String::from(x)).collect()).collect();

    let mut byr = false; // birth year
    let mut iyr = false; // issue year
    let mut eyr = false; // expiration year
    let mut hgt = false; // height
    let mut hcl = false; // hair color
    let mut ecl = false; // eye color
    let mut pid = false; // passport id

    for pair in pairs {
        if pair[0].contains("byr") { 
            let birth_year = pair[1].parse::<usize>().unwrap();
            if birth_year >= 1920 && birth_year <= 2002 {
                byr = true; 
            }
        }
        if pair[0].contains("iyr") { 
            let issue_year = pair[1].parse::<usize>().unwrap();
            if issue_year >= 2010 && issue_year <= 2020 {
                iyr = true; 
            }
        }
        if pair[0].contains("eyr") { 
            let expir_year = pair[1].parse::<usize>().unwrap();
            if expir_year >= 2020 && expir_year <= 2030 {
                eyr = true; 
            }
        }
        if pair[0].contains("hgt") { 
            let number = pair[1].chars().filter(|c| "0123456789".to_string().contains(&c.to_string()) ).collect::<String>().parse::<usize>().unwrap();
            if pair[1].contains("cm") && number >= 150 && number <=193 {
                hgt = true; 
            }
            else if pair[1].contains("in") && number >= 59 && number <= 76 {
                hgt = true; 
            }
        }
        if pair[0].contains("hcl") { 
            let hex = pair[1].chars().filter(|c| "0123456789abcdef".to_string().contains(&c.to_string()) ).collect::<String>();
            
            if hex.len() == 6 && pair[1].contains("#") {
                hcl = true; 
            } 
            // println!("{}, {} -> {} {}",pair[0], pair[1], hex, hcl );

        }
        if pair[0].contains("ecl") { 
            match pair[1].as_str() {
                "amb" => ecl = true,
                "blu" => ecl = true,
                "brn" => ecl = true,
                "gry" => ecl = true,
                "grn" => ecl = true,
                "hzl" => ecl = true,
                "oth" => ecl = true,
                _ => ecl = false,
            }
            // println!("{}, {} -> {}",pair[0], pair[1], ecl);
        }
        if pair[0].contains("pid") { 
            let pspt_id = pair[1].chars().filter(|c| "0123456789".to_string().contains(&c.to_string()) ).collect::<String>();
            if pspt_id.len() == 9 {
                pid = true; 
            }
            // println!("{}, {} -> {}",pair[0], pair[1], pid);
        }
    }

    if byr & iyr & eyr & hgt & hcl & ecl & pid {
        return true;
    }
    else {
        return false 
    }
}