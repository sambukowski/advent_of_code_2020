use std::fs;

fn main() {
    println!("Advent of Code: Day 1, Challenge 1");

    let contents = fs::read_to_string("input_1.txt")
        .expect("Something went wrong reading the file");

    let mut numbers: Vec<u32> = contents.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    numbers.sort();

    let sum = 2020;
  
    for i in 0..(numbers.len() - 2) { 
          
        let mut l = i + 1;   
        let mut r = numbers.len() - 1; 

        while l < r { 
          
            if numbers[i] + numbers[l] + numbers[r] == sum {
                let result = numbers[i] * numbers[l] * numbers[r];
                println!("result: {} * {} * {} = {}", numbers[i], numbers[l], numbers[r], result);
                return
            }
              
            else if numbers[i] + numbers[l] + numbers[r] < sum {
                l += 1;
            }
            else { // A[i] + A[l] + A[r] > sum 
                r -= 1;
            }
        }
    }
}
