use std::fs;

fn main() {
    println!("Advent of Code: Day 6, Challenge 1");

    let contents = fs::read_to_string("input_6.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|x| x.parse::<String>().unwrap()).collect();

    let mut declarations: Vec<String> = Vec::new();
    let mut tmp_dec: String = String::new();

    for line in lines.to_vec() {
        if line != "" {
            tmp_dec = tmp_dec.clone() + " " + &line;
        }
        else {
            declarations.push( tmp_dec.clone() );
            tmp_dec.clear();
        }
    }
    declarations.push( tmp_dec.clone() );
    tmp_dec.clear();

    println!("declarations format: {}",declarations[0]);

    assert_eq!(3, find_responses_anyone("abc".to_string()));
    assert_eq!(3, find_responses_anyone("abc".to_string()));
    assert_eq!(3, find_responses_anyone("abac".to_string()));
    assert_eq!(1, find_responses_anyone("aaaa".to_string()));
    assert_eq!(1, find_responses_anyone("b".to_string()));
    
    let last = declarations[declarations.len()-1].clone();
    println!("len: {}, last: {}, num response: {}", declarations.len(),&last, find_responses_anyone(last.clone()) );

    let mut total_sum: u64 = 0;
    for dec in declarations.to_vec() {
        total_sum += find_responses_anyone( dec );
    }
    println!("Total sum: {}",total_sum);

    println!("Advent of Code: Day 6, Challenge 2");
    // find_responses_everyone(declarations[0].clone());
    // find_responses_everyone("a b c".to_string());

    assert_eq!(3, find_responses_everyone("abc".to_string()));
    assert_eq!(0, find_responses_everyone("a b c".to_string()));
    assert_eq!(1, find_responses_everyone("ab ac".to_string()));
    assert_eq!(1, find_responses_everyone("a a a a".to_string()));
    assert_eq!(1, find_responses_everyone("b".to_string()));

    total_sum = 0;
    for dec in declarations.to_vec() {
        total_sum += find_responses_everyone( dec );
    }
    println!("Total sum: {}",total_sum);


}

fn find_responses_everyone(input: String) -> u64 {
    let mut answers: Vec<String> = input.split(' ').map(|x| x.parse::<String>().unwrap()).collect();
    answers.retain(|c| c != "");

    let mut responses: Vec<char> = Vec::new();
    // let mut bad_responses: Vec<char> = Vec::new();
    let num_answers: usize = answers.len();

    // println!("answers: {:?}, len: {}",answers, num_answers);


    for answer in answers.to_vec() {        // check every letter of every answer
        for c in answer.chars() {
            if !responses.contains(&c) {    // 
                let mut count = 0;
                for ans in answers.to_vec() {
                    if ans.contains( &c.to_string() ) {
                        count += 1;
                    }
                }
                if count == num_answers {
                    responses.push( c );
                }
            }
        }
    }

    // println!("responses: {:?}", responses);

    return responses.len() as u64
}

fn find_responses_anyone(mut input: String) -> u64 {
    input.retain(|c| !c.is_whitespace());
    let mut reduced: Vec<char> = input.chars().collect::<Vec<char>>();
    reduced.sort_by(|a, b| b.cmp(a));
    reduced.dedup();
    
    return reduced.len() as u64
}