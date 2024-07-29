use std::fmt::{self};
use std::error::Error;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub mod ast_tree;
pub mod shunting_yard;

#[derive(Debug, Clone)]
pub enum MathValue {
    Num(f64),
    Alge(String),
    Op(char),
}

lazy_static! {
    static ref pres_map: HashMap<char, u8> = HashMap::from([
        ('^', 0),
        ('*', 1),
        ('/', 1),
        ('+', 2),
        ('-', 2)
    ]);
}
trait Push {
    fn push(&mut self, token: MathValue);
}

pub struct Validate;
impl Validate {
    pub fn validate_input(input: &str) ->(bool, &str) {
        // Remove spaces
        let input = input.replace(" ", "");
        if !Self::validate_len(&input) {
            return (false, "Enter at least 3 elements");
        }
    
        // Check every value is either in pres_map, alpha, digit or bracket
        if !Self::validate_chars(&input){
            return (false, "Invalid Char");
        }
    
        // Check no sandwiched operators (a OP b)
        if !Self::validate_sandwich_operators(&input) {
            return (false, "Invalid order of operators");
        }
    
        // Check correct number of brackets
        if !Self::validate_parentheses(&input) {
            return (false, "Invalid order of parentheses");
        }
    
        (true, "Is_valid")
    }

    fn char_contained_in(ch: char, haystack: &str) -> bool {
        haystack.chars().any(|c| c == ch)
    }

    fn validate_len(input: &str) -> bool{
        let input = input.replace(" ", "");
        if input.len() < 3 {
            false
        } else {
            true
        }
    }

    fn validate_chars(input: &str) -> bool {
        if input.chars().any(|c| {
            !(
            c.is_alphabetic() || 
            c.is_ascii_digit() || 
            Self::char_contained_in(c, "().!") ||
            pres_map.contains_key(&c)
            )}) 
        {
            false
        } else {
            true
        }
    }

    // Change for Sliding Window
    fn validate_sandwich_operators(input: &str) -> bool {
        let mut iter = input.chars().peekable();
        while let Some(first) = &iter.next() {
            if let Some(second) = iter.peek() {
                if pres_map.contains_key(first) && pres_map.contains_key(second) { 
                    return false;
                }
            }
        }
        true
    }

    fn validate_parentheses(input: &str) -> bool {
        let mut parentheses:Vec<char> = Vec::new();
        for bracket in input.chars().filter(|b| Self::char_contained_in(*b, "()")) {
            match bracket {
                '(' => parentheses.push(bracket),
                ')' => {
                    if parentheses.pop().is_none() {
                        return false;
                    }
                },
                _ => return false,
            }
        }
        parentheses.is_empty()
    }
}

#[derive(Debug)]
pub struct Stack {
    elements: Vec<MathValue>,
}
impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Push for Stack {
    fn push(&mut self, token: MathValue) {
        self.elements.push(token);
    }  
}
#[allow(unused)]
impl Stack {
    fn new() -> Stack {
        Stack {
            elements: Vec::new(),
        }
    }   
    pub fn try_from(input: &str) -> Result<Stack, Box<dyn Error>> {
        let (is_valid, msg) = Validate::validate_input(input);
        if !is_valid {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg)));
        }
        let mut stack = Stack::new();
        let mut number_as_string = String::from("");
        let mut digit_tracker = false;

        // True for alge, else false
        let conversion_type_is_alge = input.chars().any(|c| c.is_alphabetic());

        for token in input.chars() {
            if handle_non_op_token(&token, &mut digit_tracker, &mut number_as_string) {
                continue;
            }
            if digit_tracker {
                push_conversion_type(&mut stack, number_as_string, conversion_type_is_alge)?;
                digit_tracker = false;
                number_as_string = "".to_string();
            }
            stack.push(MathValue::Op(token));
            
        }
        if !number_as_string.is_empty() {
            if conversion_type_is_alge {}
            push_conversion_type(&mut stack, number_as_string, conversion_type_is_alge)?;
        }
        Ok(stack)        
    }

    pub fn iter(&self) -> StackIter {
        StackIter { stack: self, index: 0}
    }

    pub fn len(self) -> usize {
        self.elements.len()
    }
    
    pub fn as_string(&self) -> String {
        self.iter().map(|el| 
            match el {
                MathValue::Num(num) => num.to_string(),
                MathValue::Alge(al) => al.to_string(),
                MathValue::Op(op) => op.to_string(),
            }
        ).collect::<Vec<String>>().join(" ")
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


pub struct StackIter<'a> {
    stack: &'a Stack,
    index: usize,
}
impl<'a> Iterator for StackIter<'a> {
    type Item = &'a MathValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.stack.elements.len() {
            let result = Some(
                &self.stack.elements[self.index]
            );
            self.index += 1;
            result
        } else {
            None
        }
    }

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
        '!' => {
            number_as_string.push('-');
            *digit_tracker = true;
            true
        }
        _ => false
    }
}

fn push_conversion_type<T: Push>(target: &mut T, value: String, conversion_type: bool) -> Result<(), Box<dyn Error>> {
    if conversion_type {
        target.push(MathValue::Alge(value));
    } else {
        target.push(MathValue::Num(value.parse::<f64>()?));
    }
    Ok(())
}

mod validate_input_tests {
    use super::*;
    // Unit tests
    #[test]
    fn less_than_3() {
        let input_true = "2a+3.1";
        let input_false = "";
        assert_eq!(true, Validate::validate_len(input_true));
        assert_eq!(false, Validate::validate_len(input_false));
    }
    #[test]
    fn invalid_char() {
        let input_true = "3.1+8a";
        let input_false = "3@5+7*(8+4)";
        assert_eq!(true, Validate::validate_chars(input_true));
        assert_eq!(false, Validate::validate_chars(input_false));
    }
    #[test]
    fn invalid_sandwich_operators() {
        let input_true = "2.1+5a-3*(5-2)";
        let input_false = "2++5-3*(5--2)";
        assert_eq!(true,  Validate::validate_sandwich_operators(input_true));
        assert_eq!(false, Validate::validate_sandwich_operators(input_false));
    }
    #[test]
    fn invalid_params() { 
        let input_true = "(2.1+3)^2 -(3a+(4^32.3-1)+x^2)";
        let input_false = "(2+3)^2 -(3+(4^32.3-1)+x^2))";
        let input_false_2 = "(()";
        let input_false_3 = "))((";
        assert_eq!(true, Validate::validate_parentheses(input_true));        
        assert_eq!(false, Validate::validate_parentheses(input_false));
        assert_eq!(false, Validate::validate_parentheses(input_false_2));
        assert_eq!(false, Validate::validate_parentheses(input_false_3));
    }
    // Integration Test
    #[test]
    fn validate_input_integration() {
        assert_eq!((true, "Is_valid"), Validate::validate_input("2+5-1/7*2^(2-1)+a21"));
    }
}