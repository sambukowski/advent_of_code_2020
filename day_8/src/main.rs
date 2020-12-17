use std::fs;

fn main() {
    println!("Advent of Code: Day 8, Challenge 1");

    let contents = fs::read_to_string("input_8.txt")
    // let contents = fs::read_to_string("sample_input_8.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|x| x.parse::<String>().unwrap()).collect();
    let commands = convert_to_commands( lines.to_vec() );
    let p1_result = execute( commands );
    println!("Before boot-up failure, the accumulator had a value of {}.",p1_result.1);
    println!("Did the final command run? -> {}",p1_result.0);


    println!("Advent of Code: Day 8, Challenge 2");
    let p2_results = fix_execution( lines.to_vec() );
    println!("After inspection, line {} was corrected and the accumulator had a final value of {}.", p2_results.0, p2_results.1);
}

// first return is the line number that was corrected, second is accumulator on completion
fn fix_execution( input: Vec<String> ) -> (usize, i64) {
    let commands = convert_to_commands( input );

    let length = commands.len();

    for i in 0..length {
        let mut copy_cmd = commands.clone();

        if copy_cmd[i].op.as_str() == "jmp" {
            copy_cmd[i].op = "nop".to_string();
            // index = i;
            let current_result = execute( copy_cmd );

            if current_result.0 == true {
                return (i, current_result.1)
            }
        }
        else if copy_cmd[i].op.as_str() == "nop" {
            copy_cmd[i].op = "jmp".to_string();
            // index = i;
            let current_result = execute( copy_cmd );

            if current_result.0 == true {
                return (i, current_result.1)
            }
        }
    }
    (0,0)
}

fn execute(input: Vec<Command>) -> (bool, i64) {
    // bool let's you know if the last statement was run
    // i64 is the accumulator value
    let mut commands = input.to_vec();
    let total_commands = input.len();
    let mut accumulator: i64 = 0;
    let mut index: i64 = 0;
    let mut loop_count: usize = 0;

    loop {
        if loop_count >= total_commands || index as usize >= total_commands || commands[index as usize].visited == true {
            break;
        }

        match commands[index as usize].op.as_str() {
            "acc" => {
                    accumulator += commands[index as usize].arg;
                    commands[index as usize].visited = true;
                    index += 1;
                },
            "jmp" => {
                    commands[index as usize].visited = true;
                    index += commands[index as usize].arg;
                },
            "nop" => {
                    commands[index as usize].visited = true;
                    index += 1;
                },
            _ => {},
        }

        loop_count += 1;
    }

    let mut bool_out = false;
    if index as usize == total_commands && commands[total_commands-1].visited == true {
        bool_out = true;
    }

    (bool_out, accumulator)
}

fn convert_to_commands( lines: Vec<String> ) -> Vec<Command> {
    let mut output: Vec<Command> = Vec::new();

    for line in lines.to_vec() {
        output.push( parse_line_to_command( line.clone() ) );
    }

    output
}

fn parse_line_to_command( input: String ) -> Command {
    let bits: Vec<String> = input.split(" ").map(|c| c.parse::<String>().unwrap()).collect();
    let op: String = bits[0].clone();
    let mut arg: i64 = 1;
    if bits[1].contains("-") {
        arg *= -1;
    }
    let tmp_num: i64 = bits[1].clone().chars().filter(|c| "0123456789".to_string().contains(&c.to_string()) ).collect::<String>().parse::<i64>().unwrap();
    arg = arg * tmp_num;

    Command {
        op: op,
        arg: arg,
        visited: false,
    }
}

#[derive(Clone,PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Command {
    op: String,
    arg: i64,
    visited: bool,
}
