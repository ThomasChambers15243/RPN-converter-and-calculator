use rpn::{solve_numerical, convert_in_to_post_fix};
use std::io;

fn main() {    
    let mut decision = String::new();
    let mut input = String::new();
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
                match convert_in_to_post_fix(&input.trim()) {
                    Ok(answer) => println!("RPN form: {}\n", answer),
                    Err(e) => println!("Error: {}\n", e),
                };
            },
            _ => (),
        }        
    }
}