use std::fs;
use itertools::Itertools;

fn main() {
    println!("Advent of Code: Day 9, Challenge 1");

    let contents = fs::read_to_string("input_9.txt")
    // let contents = fs::read_to_string("sample_input_9.txt")
        .expect("Something went wrong reading the file");

    let values: Vec<u64> = contents.lines().map(|x| x.parse::<String>().unwrap().parse::<u64>().unwrap()).collect();

    // let it = (1..4).combinations(2);
    // itertools::assert_equal(it, vec![
    //     vec![1, 2],
    //     vec![1, 3],
    //     vec![2, 3],
    // ]); 

    assert_eq!(false,sum_possible(vec![1,2,3],0) );
    assert_eq!(true,sum_possible(vec![1,2,3],5) );

    let preamble: usize = 25;

    println!("The first value that does follow to XMAS data properties is {}.",values[ search_xmas_data(values.to_vec(), preamble) ]);

    println!("\nAdvent of Code: Day 9, Challenge 2");

    assert_eq!( (true,1,2), range_sum_possible(vec![1,2,3],5) );
    // assert_eq!( , find_xmas_data_weakness(vec![1,2,3,4,5], 3) );

    println!("The XMAS encryption weakness is {}.", find_xmas_data_weakness( values.to_vec(), preamble ) );


    
}

fn find_xmas_data_weakness( input: Vec<u64>, preamble: usize ) -> u64 {
    let index = search_xmas_data(input.to_vec(), preamble);
    let value = input[ index ];
    let weakness = range_sum_possible( input.to_vec(), value );

    let mut range: Vec<u64> = input[ weakness.1..=weakness.2 ].to_vec();
    range.sort();

    return range[0] + range[ range.len()-1 ]
}

fn range_sum_possible( input: Vec<u64>, value: u64 ) -> (bool, usize, usize) {
    for i in 0..input.len() {
        let mut collection: Vec<u64> = Vec::new();
        for j in i..input.len() {
            collection.push( input[j] );
            if collection.iter().sum::<u64>() == value {
                return (true, i, j)
            }
        }
    }
    (false, 0, 0)
}

fn search_xmas_data( input: Vec<u64>, preamble: usize ) -> usize {
    for i in (preamble)..input.len() {
        let valid = sum_possible( input[ (i-preamble)..(i) ].to_vec(), input[i] );
        if !valid {
            return i
        }
    }
    input.len()
}

fn sum_possible( input:Vec<u64>, value: u64 ) -> bool {
    let it: Vec<u64> = input.iter().combinations(2).map(|c| c[0]+c[1]).collect();
    if it.contains( &value ) {
        return true
    }
    else {
        return false
    }
}