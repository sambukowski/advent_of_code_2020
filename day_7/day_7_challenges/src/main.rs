use std::fs;
// use itertools::Itertools;

// this one is kinda a total mess

fn main() {
    println!("Advent of Code: Day 7, Challenge 1");

    let contents = fs::read_to_string("input_7.txt")
    // let contents = fs::read_to_string("simple_7_test.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|x| x.parse::<String>().unwrap()).collect();


    print_bag( &build_bag( lines[0].clone() ) );
    print_bag( &build_bag( lines[7].clone() ) );

    let mut bags: Vec<Bag> = Vec::new();
    for line in lines.to_vec() {
        bags.push( build_bag( line ) );
    }

    println!("num lines: {}, num bags: {}", lines.len(), bags.len() );

    let my_bag_style: String = "shiny gold".to_string();
    println!("Number of bags that could hold a {} bag is: {}", my_bag_style.clone(), find_which_bags( bags.to_vec(), my_bag_style.clone() ));

    println!("\nAdvent of Code: Day 7, Challenge 2");
    println!("My {} bag needs to hold {} other bags.", my_bag_style.clone(), count_required_bags( bags.to_vec(), my_bag_style.clone() ) );


}

fn count_required_bags( input_vec: Vec<Bag>, base_bag_style: String) -> u64 {
    let mut current: Bag = Bag::new();

    for bag in input_vec.to_vec() {
        if bag.style == base_bag_style {
            current = bag;
            break;
        }
    }

    let mut sub_bag_count = 0;

    for sub_bag in current.rules.to_vec() {
        if sub_bag.count == 0 {
            return 0;
        }
        else{
            let sub_sub_count = count_required_bags( input_vec.to_vec(), sub_bag.bag_style.clone() );
            if sub_sub_count == 0 {
                sub_bag_count += sub_bag.count;
            }
            else{
                sub_bag_count += sub_bag.count;
                sub_bag_count += sub_bag.count * sub_sub_count;
            }
        }
    }

    sub_bag_count
}

fn find_which_bags(mut input_vec: Vec<Bag>, base_bag_style: String) -> u64 {
    
    let mut bags_added_in_loop = 1;
    let mut added_bags: Vec<Bag> = Vec::new();

    let mut index = 0;
    for i in 0..input_vec.len() {
        if input_vec[i].style == base_bag_style {
            index = i;
            break;
        }
    }
    println!("Original bag removed.");
    input_vec.remove( index );

    let mut need_remove: Vec<usize> = Vec::new();
    for i in 0..input_vec.len() {
        if input_vec[i].rules[0].count == 0 {
            need_remove.push( i );
        }
    }

    need_remove.reverse(); 
    for ind in need_remove.to_vec() {
        input_vec.remove( ind );
    }
    println!("Removed {} bags that can't hold anything.",need_remove.len());

    // println!("\n\n+++\n");
    // for bag in input_vec.to_vec() {
    //     print_bag( &bag );
    // }
    
    while bags_added_in_loop > 0 {
        bags_added_in_loop = 0;

        for bag in input_vec.to_vec() {
            for inner_bag in &bag.rules {
                if inner_bag.bag_style == base_bag_style {
                    added_bags.push( bag.clone() );
                    bags_added_in_loop += 1;
                    // break;
                }
                else {
                    for a_bag in added_bags.to_vec() {
                        if a_bag.style == inner_bag.bag_style {
                            added_bags.push( bag.clone() );
                            bags_added_in_loop += 1;
                            // break;
                        }
                    }
                }
            }
        }

        if bags_added_in_loop != 0 {
            for bag in added_bags.to_vec() {
                if input_vec.contains( &bag ) {
                    let mut index = 0;
                    for i in 0..input_vec.len() {
                        if input_vec[i].style == bag.style {
                            index = i;
                            break;
                        }
                    }
                    input_vec.remove( index );
                }
            }
        }
    }

    added_bags.sort();
    added_bags.dedup();

    // println!("\n+++\n");

    // for bag in added_bags.to_vec() {
    //     print_bag( &bag );
    // }

    return added_bags.len() as u64
}



fn print_bag(input: &Bag) {
    println!("Bag: {}", input.style);
    println!("Rules:");
    for rule in &input.rules {
        println!("  {} {}",rule.count, rule.bag_style);
    }

}

fn build_bag(input: String) -> Bag {
    let parts: Vec<String> = input.split("bags contain").map(|c| c.parse::<String>().unwrap()).collect();
    let style: String = parts[0].trim().to_string();
    let rules: Vec<String> = parts[1].split(",").map(                                                               // split by ','
                                                        |c| c.parse::<String>().unwrap().trim().to_string()         // convert back to String and remove leading and trailing whitespace
                                                        .replace(".","").replace("bags","").replace("bag","")       // remove all other unwanted parts of the string
                                                        .trim().to_string()                                         // remove any L/T whitespace the might be still there and properly convert back to String
                                                    ).collect();                                                    // turn all that into a Vec<String>

    let mut bag = Bag {
        style: style,
        rules: Vec::new(),
    };

    for rule in rules.to_vec() {
        bag.rules.push( build_bag_rule( rule ) );
    }

    return bag
}

fn build_bag_rule(input: String) -> BagRule {
    let number_parse = input.clone().chars().filter(|c| "0123456789".to_string().contains(&c.to_string()) ).collect::<String>().parse::<u64>();
    let mut number: u64 = 0;
    if number_parse.is_ok() {
        number = number_parse.unwrap();
    }

    let letter_parse: String = input.clone().chars().filter(|c| "abcdefghijklmnopqrstuvwxyz ".to_string()
                                                            .contains(&c.to_string()) 
                                                        ).collect::<String>().trim().to_string();

    let bag_rule = BagRule {
        count: number,
        bag_style: letter_parse,
    };

    return bag_rule
}

impl Bag {
    fn new() -> Bag {
        Bag {
            style: String::new(),
            rules: Vec::new(),
        }
    }
}

// impl BagRule {
//     fn new() -> BagRule {
//         BagRule {
//             count: 0,
//             bag_style: String::new(),
//         }
//     } 
// }

#[derive(Clone,PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Bag {
    style: String,
    rules: Vec<BagRule>,
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BagRule {
    count: u64,
    bag_style: String,
}