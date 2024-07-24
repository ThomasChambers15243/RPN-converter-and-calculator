use std::fmt::{self};
use std::collections::{VecDeque, HashMap};
use lazy_static::lazy_static;

lazy_static! {
    static ref pres_map: HashMap<char, u8> = HashMap::from([
        ('^', 0),
        ('*', 1),
        ('/', 1),
        ('+', 2),
        ('-', 2)
    ]);
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum MathValue {
    Num(f64),
    Alge(String),
    Op(char),
}

struct OpStack {
    elements: Vec<MathValue>,
}

#[allow(dead_code)]
impl OpStack {
    fn new() -> OpStack {
        OpStack {
            elements: Vec::new(),
        }
    }
    fn push(&mut self, token: MathValue) {
        self.elements.push(token);
    }
    fn pop(&mut self) -> Option<MathValue> {
        self.elements.pop()
    }
    fn peak(&mut self) -> Option<&MathValue> {
        if self.elements.is_empty(){
            None
        } else {
            self.elements.last()
        }
    }
}
#[derive(Debug)]
pub struct OutQueue {
    elements: VecDeque<MathValue>,
}

impl fmt::Display for OutQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.queue_as_string())
    }
}

pub struct OutQueueIterator<'a> {
    out_queue: &'a OutQueue,
    index: usize,
}
impl<'a> Iterator for OutQueueIterator<'a> {
    type Item = &'a MathValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.out_queue.elements.len() {
            let result = Some(
                &self.out_queue.elements[self.index]
            );
            self.index += 1;
            result
        } else {
            None
        }
    }

}

impl OutQueue {
    fn new() -> OutQueue {
        OutQueue {
            elements: VecDeque::new(),
        }
    }

    pub fn iter(&self) -> OutQueueIterator {
        OutQueueIterator { out_queue:self, index: 0 }
    }

    pub fn queue_as_string(&self) -> String {
        self.elements.iter().map(|token| {
            match token {
                MathValue::Op(ch) => ch.to_string(),
                MathValue::Alge(al) => al.to_string(),
                MathValue::Num(num) => num.to_string(),
            }
        }).collect::<Vec<String>>().join(" ")
    }
    fn push(&mut self, token: MathValue) {
        self.elements.push_back(token);
    }
}

pub fn convert_numerical_in_to_post_fix(i: &str) -> Result<OutQueue, Box<dyn std::error::Error>> {
    let mut operators = OpStack::new();
    let mut output = OutQueue::new();

    // Clean spaces from string
    let input = i.replace(' ', "");

    let mut digit_tracker = false;
    let mut number_as_string: String = String::new();

    // Loop through chars in input
    for token in input.chars() {
        // If digit
        if handle_non_op_token(&token, &mut digit_tracker, &mut number_as_string) {
            continue;
        }

        // Convert string to f64 and push it to the output
        // Reset digit_tracker and number_as_string
        if digit_tracker {
            output.push(MathValue::Num(number_as_string.parse::<f64>()?));
            digit_tracker = false;
            number_as_string = "".to_string();
        }

        // If Operator or Bracket 
        if let Err(e) = handle_operators(&token, &mut operators, &mut output) {
            return Err(e);
        }
    }
    if digit_tracker { 
        output.push(MathValue::Num(number_as_string.parse::<f64>()?)); 
    }    
    while let Some(ops) = operators.pop() {
        output.push(ops);
    }
    Ok(output)
    
}

pub fn convert_algebra_in_to_post_fix(i: &str) -> Result<OutQueue, Box<dyn std::error::Error>> {
    let mut operators = OpStack::new();
    let mut output = OutQueue::new();

    // Clean spaces from string
    let input = i.replace(' ', "");

    let mut digit_tracker = false;
    let mut number_as_string: String = String::new();

    // Loop through chars in input
    for token in input.chars() {
        // If digit
        if handle_non_op_token(&token, &mut digit_tracker, &mut number_as_string) {
            continue;
        }

        // Convert string to f64 and push it to the output
        // Reset digit_tracker and number_as_string
        if digit_tracker {
            output.push(MathValue::Alge(number_as_string));
            digit_tracker = false;
            number_as_string = "".to_string();
        }

        // If Operator or Bracket 
        if let Err(e) = handle_operators(&token, &mut operators, &mut output) {
            return Err(e);
        }
    }
    if digit_tracker { 
        output.push(MathValue::Alge(number_as_string)); 
    }    
    while let Some(ops) = operators.pop() {
        output.push(ops);
    }
    Ok(output)
}

fn handle_non_op_token(token: &char, digit_tracker: &mut bool, number_as_string: &mut String) -> bool {
    match *token {
        '0'..='9' | '.' => {
            number_as_string.push(*token);
            *digit_tracker = true;
            true 
        },
        'a'..='z' => {
            number_as_string.push(*token);
            *digit_tracker = true;
            true
        },
        _ => false
    }
}

fn handle_operators(token: &char, operators: &mut OpStack, output: &mut OutQueue) -> Result<(), Box<std::io::Error>> {
    match pres_map.get(&token) {            
        // Operators
        Some(pres) => {
            while let Some(top_of_stack) = operators.peak() {
                if let MathValue::Op(op) = top_of_stack {
                    // If bracket, set to prec value which always fails
                    if pres_map.get(op).unwrap_or(&8) <= pres {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
            }
            operators.push(MathValue::Op(*token));                                 
        },
        None => {
            // Brackets
            if *token == '(' {
                operators.push(MathValue::Op(*token));
            } else if *token == ')' {
                // If left bracket, discard
                // else push to output 
                while let Some(MathValue::Op(op)) = operators.pop() {
                    if op == '(' {
                        break;
                    } else {
                        output.push(MathValue::Op(op));
                    }
                }
            }
            else {
                let err_msg = format!("Invalid operator: '{}'", token);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)));
            }
        }            
    }
    Ok(())
}


#[cfg(test)]
mod convert_numerical_in_to_post_fix_tests {
    use super::*;

    #[test]
    fn simple_small() {
        let input = "5.2 + 8.0";
        let expected = "5.28+";
        assert_eq!(expected, convert_numerical_in_to_post_fix(input).unwrap().queue_as_string());
    }

    #[test]
    fn simple_large() {
        let input = "50 + 2342 - 234324.8";
        let expected = "502342+234324.8-";
        assert_eq!(expected, convert_numerical_in_to_post_fix(input).unwrap().queue_as_string());
    }

    #[test]
    fn complex_small() {
        let input = "40 + 4 - 1 / (9 * 99))";
        let expected = "404+1999*/-";
        assert_eq!(expected, convert_numerical_in_to_post_fix(input).unwrap().queue_as_string());
    }

    #[test]
    fn complex_large() {
        let input = "42 - 4234 * (4-234 + (43*43)) - 10";
        let expected = "4242344234-4343*+*-10-";
        assert_eq!(expected, convert_numerical_in_to_post_fix(input).unwrap().queue_as_string());
    }

    #[test]
    fn quadratic() {
        let input = "(31 + 321)*(32+54)";
        let expected = "31321+3254+*";
        assert_eq!(expected, convert_numerical_in_to_post_fix(input).unwrap().queue_as_string());
    }

    #[test]
    fn should_error() {
        let input = "5+6a";
        let expected = "invalid float literal";
        assert_eq!(expected, convert_numerical_in_to_post_fix(input).unwrap_err().to_string());
    }
}

#[cfg(test)]
mod convert_algrebra_in_to_post_fix_tests {
    use crate::shunting_yard::convert_algebra_in_to_post_fix;

    #[test]
    fn simple_small() {
        let input = "a+7b";
        let expected = "a7b+";
        assert_eq!(expected, convert_algebra_in_to_post_fix(input).unwrap().to_string());
    }
    #[test]
    fn simple_large() {
        let input = "ab3213 + 131 - p * q";
        let expected = "ab3213131+pq*-";
        assert_eq!(expected, convert_algebra_in_to_post_fix(input).unwrap().to_string());
    }
    #[test]
    fn complex_small() {
        let input = "(x*x / (z-32.1c))";
        let expected = "xx*z32.1c-/";
        assert_eq!(expected, convert_algebra_in_to_post_fix(input).unwrap().to_string());
    }
    #[test]
    fn complex_large() {
        let input = "c(a(b*b+1) - (d123.32/f9.23))";
        let expected = "cabb*1+d123.32f9.23/-";
        assert_eq!(expected, convert_algebra_in_to_post_fix(input).unwrap().to_string());
    }
    #[test]
    fn quadratic() {
        let input = "(x + 87.31)*(x-31.23)";
        let expected = "x87.31+x31.23-*";
        assert_eq!(expected, convert_algebra_in_to_post_fix(input).unwrap().to_string());
    }
    // fn should_error() {
    //     let input = "";
    //     let expected = "";
        
    // }
}