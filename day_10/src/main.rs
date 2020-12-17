use std::fs;
use combinations::Combinations;


fn main() {
    println!("Advent of Code: Day 10, Challenge 1");

    // let contents = fs::read_to_string("input_10.txt")
    // let contents = fs::read_to_string("sample_input_10.txt")
    let contents = fs::read_to_string("sample_input2_10.txt")
        .expect("Something went wrong reading the file");

    let values: Vec<u64> = contents.lines().map(|x| x.parse::<String>().unwrap().parse::<u64>().unwrap()).collect();

    // assert_eq!((22,10),count_jolt_jumps( values ));
    let output = count_jolt_jumps( values.to_vec() );
    println!("The output value is: {}",output.0 * output.1);

    println!("Advent of Code: Day 10, Challenge 2");
    println!("The total number of possible combinations is {}.", count_adapter_combinations( values.to_vec() ));

}

fn count_adapter_combinations( mut input: Vec<u64> ) -> u64 {
    input.sort();
    let min: u64 = 0;
    let max: u64 = input[input.len()-1] + 3;
    let mut count: u64 = 0;

    let mut accepted_combinations: Vec<Vec<u64>> = Vec::new();

    for i in 2..input.len() {
        let it: Vec<Vec<u64>> = Combinations::new(input.to_vec(), i).collect();

        for mut vec in it {
            vec.push(min);
            vec.push(max);
            vec.sort();
            if check_valid_combination( vec.to_vec(), min, max ) {
                if !accepted_combinations.contains( &vec ) {
                    println!("{:?}",vec);
                    accepted_combinations.push( vec );
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_valid_combination(mut input: Vec<u64>, min: u64, max: u64) -> bool {
    input.push(min);
    input.push(max);
    input.sort();
    for i in 1..input.len() {
        if input[i]-input[i-1] > 3 {
            return false
        }
    }
    true
}


fn count_jolt_jumps( mut input: Vec<u64> ) -> (u64, u64) {
    if !input.contains(&0) {
        input.push(0);
    }
    
    input.sort();
    let diff: Vec<u64> = input.windows(2).map(|w| w[1] - w[0]).collect();

    let mut ones_count = 0;
    let mut threes_count = 0;

    for val in diff {
        if val == 1 {
            ones_count += 1;
        }
        else if val == 3 {
            threes_count += 1;
        }
    }
    threes_count += 1; // jump to your device

    ( ones_count, threes_count )
}