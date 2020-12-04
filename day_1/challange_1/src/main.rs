use std::fs;

fn main() {
    println!("Advent of Code: Day 1, Challenge 1");

    let contents = fs::read_to_string("input_1.txt")
        .expect("Something went wrong reading the file");

    let numbers: Vec<u32> = contents.lines().map(|x| x.parse::<u32>().unwrap()).collect();


    let mut result = 0;
    let mut pos = 0;
    for num in numbers.to_vec() {
        let mut other_pos = 0;
        for other_num in numbers.to_vec() {
            if pos != other_pos && num + other_num == 2020 {
                result = num * other_num;
            }
            other_pos += 1;
        }
        pos += 1;
    }

    println!("result: {}",result);

}
