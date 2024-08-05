//! Converts and solves infix to postfix (reverse-polish notation [RPM]). Values containing variables cannot be solved, only converted.
//! To input negatives, type either ' ¬ ' or ' ! '. 
//! 
//! Uses 2 different algorithms to convert, shunting yard or an AST tree post-order traversal. The default is shunting yard due to increased performance.
//! 'solve_numerical' takes in an numerical infix expression as a string reference and solves it using shunting yard.
//! 'in_to_rpn' converts an infix expression to rpn using shunting.
//! 'get_rpn_yard' & 'get_rpn_tree' use their respective algorithm to convert.
//! 
//! Acceptable operators are +, -, /, ^, *
//! 
//! Example
//! '''Rust
//! match solve_numerical("3+¬43 * (!3+2)^2") {
//!     Ok(answer) => println!("{} = {}\n", input, answer),
//!     Err(e) => println!("Error: {}\n", e),
//! };
//! // prints -> -40
//! match in_to_rpn("(x-¬321)  *  (x+(!32-x))") {
//!     Ok(answer) => println!("RPN form: {}\n", answer),
//!     Err(e) => println!("Error: {}\n", e),
//!};
//! // prints -> x -321 - x -32 x - + *

mod rpn_convert;
    use rpn_convert::{
    Validate,
    MathValue, Stack,
    shunting_yard,
    ast_tree,
};

use std::error::Error;

/// Solves the given numerical expression
pub fn solve_numerical(input: &str) -> Result<f64, Box<dyn std::error::Error>>{
    let mut total_stack:Vec<f64> = Vec::new();
    let rpn_form: Stack = shunting_yard::convert_in_to_post_fix(input)?;
    let form_iter = rpn_form.iter();

    for value in form_iter {
        match value {
            MathValue::Num(num) => total_stack.push(*num),
            MathValue::Op(op) => {
                let b: f64 = total_stack.pop().unwrap();
                let a: f64 = total_stack.pop().unwrap();
                total_stack.push(calculate(a, b, *op));
            },
            MathValue::Alge(_al) => continue,
        }
    }
    println!("RPN form is: {}", shunting_yard::convert_in_to_post_fix(input)?);
    Ok(total_stack.pop().unwrap())
}

/// Converts an infix expression to a post fix expression (RPN)
pub fn in_to_rpn(input: &str) -> Result<String, Box<dyn Error>> {
    get_rpn_yard(input)
}

/// Calculates the solution from the given operators
/// Works left to right - a op b
fn calculate(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        '^' => a.powf(b),
        '%' => a % b,
        _ => panic!("Invalid operations"),
    }
}

/// Converts an infix expression to a post fix expression (RPN) using shunting yard
pub fn get_rpn_yard(input: &str) -> Result<String, Box<dyn Error>> {
    let (is_valid, msg) = Validate::validate_input(input);
    if is_valid {
        Ok(shunting_yard::convert_in_to_post_fix(input)?.as_string())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg)))
    }
}

/// Converts an infix expression to a post fix expression (RPN) using post-order of an AST Tree
pub fn get_rpn_tree(input: &str) -> Result<String, Box<dyn Error>> {
    let (is_valid, msg) = Validate::validate_input(input);
    if is_valid {
        Ok(ast_tree::convert_in_to_post_fix(input)?.as_string())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg)))
    }
}