#[allow(unused)]
use rpn_lib::{solve_numerical, get_rpn_yard, get_rpn_tree};
use std::io;

fn main() {    
    let mut decision: String;
    let mut input: String;
    loop {
        println!("Enter your equation.\ns for solve (only numerical, not variables),\nr for RPN form (postfix),\nb | q to quit...");
        decision = "".to_string();
        input= "".to_string();
        io::stdin().read_line(&mut decision).expect("Failed to read line");

        match decision.trim().to_lowercase().as_str() {
            "b" |"q" => {
                println!("Quitting...");
                break
            },
            "s" => {
                io::stdin().read_line(&mut input).expect("Failed to read line");
                match solve_numerical(&input.trim()) {
                    Ok(answer) => println!("{} = {}\n", input, answer),
                    Err(e) => println!("Error: {}\n", e),
                };
            },
            "r" => {
                io::stdin().read_line(&mut input).expect("Failed to read line");
                match get_rpn_yard(&input.trim()) {
                    Ok(answer) => println!("RPN form: {}\n", answer),
                    Err(e) => println!("Error: {}\n", e),
                };
            },
            _ => (),
        }        
    }
}