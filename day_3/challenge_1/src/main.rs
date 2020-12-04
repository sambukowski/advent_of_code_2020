use std::fs;

fn main() {
    println!("Advent of Code: Day 3, Challenge 1");

    let contents = fs::read_to_string("input_3.txt")
        .expect("Something went wrong reading the file");

    let mut lines: Vec<Vec<char>> = contents.lines()                        // split the input string by each line
                                        .map(|x| x.parse::<String>()    // convert each Line object back into a String
                                            .unwrap()                   // remove any wrappers that might be above the String type to make it just a String
                                            .chars()                    // split the String into the characters that are in it
                                            .collect() )                // translate that iterator of chars into a Vec<char>
                                        .collect();                     // wrap each Vec<char> line in another Vec to make the 2D Vec<Vec<char>> 

    let height = lines.len();
    let width = lines[0].len();
    println!("h: {}, w:{}", height, width);

    let x_step = 3; // move 3 to the right
    let y_step = 1; // move 1 down

    let mut x = 0;
    let mut y = 0;

    let mut trees_encountered = 0;

    while y < height {
        if lines[y][x] == '#' {
            trees_encountered += 1;
            lines[y][x] = 'X';
        } 
        else {
            lines[y][x] = 'O';
        }
        // lines[y][x] = '!';
        y += y_step;
        x += x_step;
        // x += 1;
        // y += 1;
        if x >= width {
            x -= width;
        }
    }

    for line in lines.to_vec() {
        for c in line {
            print!("{}",c);
        }
        println!();
    }

    println!("Trees encountered: {}", trees_encountered);
}
