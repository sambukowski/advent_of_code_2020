use std::fs;

fn main() {
    println!("Advent of Code: Day 5, Challenge 1");

    let contents = fs::read_to_string("input_5.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|x| x.parse::<String>().unwrap().split_whitespace().collect()).collect();

    // for line in lines {
    //     println!("{}",line);
    // }
    // let test = (70, 7, 567);
    // print_pass(test);

    println!("Testing");
    print_pass( gen_boarding_pass_info("BFFFBBFRRR".to_string()) ); // BFFFBBFRRR: row 70, column 7, seat ID 567.
    print_pass( gen_boarding_pass_info("FFFBBBFRRR".to_string()) ); // FFFBBBFRRR: row 14, column 7, seat ID 119.
    print_pass( gen_boarding_pass_info("BBFFBBFRLL".to_string()) ); // BBFFBBFRLL: row 102, column 4, seat ID 820.

    let mut seats: Vec<(u64,u64,u64)> = Vec::new();

    for line in lines {
        seats.push( gen_boarding_pass_info( line ) );
    }

    let mut max_id : u64= 0;
    for seat in seats.to_vec() {
        if seat.2 > max_id {
            max_id = seat.2;
        }
    }

    println!("Max id on flight: {}",max_id);

    println!("\nAdvent of Code: Day 5, Challenge 2");
    seats.sort_by_key(|k| k.2);

    for i in 0..(seats.len()-1) {
        if seats[i+1].2 - seats[i].2 > 1 {
            println!("My seat id: {}", seats[i].2+1);
            break;
        }
    }
}

fn gen_boarding_pass_info( input: String) -> (u64, u64, u64) {

    let row_input = input[..7].to_string();
    let col_input = input[7..].to_string();

    // println!("row: {}",row_input);
    // println!("col: {}",col_input);

    let mut row: Vec<u64> = (0..=127).collect();
    let mut col: Vec<u64> = (0..=7).collect();
    // println!("{:?}",range);

    for c in row_input.chars() {
        if c == 'F' {
            row = row[..(row.len()/2)].to_vec();
        }
        else if c == 'B' {
            row = row[(row.len()/2)..].to_vec();
        }
    }

    for c in col_input.chars() {
        if c == 'L' {
            col = col[..(col.len()/2)].to_vec();
        }
        else if c == 'R' {
            col = col[(col.len()/2)..].to_vec();
        }
    }

    let id: u64 = row[0] * 8 + col[0];

    // println!("row: {}",row[0]);
    // println!("col: {}",col[0]);
    // println!("ID: {}",id);


    (row[0],col[0],id)
}

fn print_pass((a, b, c): (u64, u64, u64)) {
    println!("Row: {}, Column: {}, Seat ID: {}", a, b, c);
}