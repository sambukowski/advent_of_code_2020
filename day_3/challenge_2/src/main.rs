use std::fs;

fn main() {
    println!("Advent of Code: Day 3, Challenge 2");

    let contents = fs::read_to_string("input_3.txt")
        .expect("Something went wrong reading the file");

    let mountian: Vec<Vec<char>> = contents.lines()                        // split the input string by each line
                                        .map(|x| x.parse::<String>()    // convert each Line object back into a String
                                            .unwrap()                   // remove any wrappers that might be above the String type to make it just a String
                                            .chars()                    // split the String into the characters that are in it
                                            .collect() )                // translate that iterator of chars into a Vec<char>
                                        .collect();                     // wrap each Vec<char> line in another Vec to make the 2D Vec<Vec<char>> 

    let slopes = vec![
                    (1,1),
                    (3,1),
                    (5,1),
                    (7,1),
                    (1,2)
                ];

    let mut encounters: Vec<usize> = Vec::new();

    for s in slopes {
        let tmp = check_trees(&mountian, s.0, s.1);
        encounters.push( tmp );
        println!("slope: ({}, {}) -> tree encounters: {}",s.0, s.1, tmp);
    }

    let mut result = 1;
    for val in encounters {
        // print!("{} * ",val);
        result *= val;
    }

    println!("result: {}", result);

}

fn check_trees(mountian: &Vec<Vec<char>>, x_step: usize, y_step: usize) -> usize {
    let height = mountian.len();
    let width = mountian[0].len();
    // println!("h: {}, w:{}", height, width);

    // let x_step = 3; // move 3 to the right
    // let y_step = 1; // move 1 down

    let mut x = 0;
    let mut y = 0;

    let mut trees_encountered = 0;

    while y < height {
        if mountian[y][x] == '#' {
            trees_encountered += 1;
            // mountian[y][x] = 'X';
        } 
        // else {
        //     mountian[y][x] = 'O';
        // }
        // lines[y][x] = '!';
        y += y_step;
        x += x_step;
        // x += 1;
        // y += 1;
        if x >= width {
            x -= width;
        }
    }

    // for line in mountian.to_vec() {
    //     for c in line {
    //         print!("{}",c);
    //     }
    //     println!();
    // }

    // println!("Trees encountered: {}", trees_encountered);

    trees_encountered
}