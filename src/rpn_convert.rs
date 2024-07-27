use std::fmt::{self};
use std::error::Error;
use std::collections::HashMap;
use lazy_static::lazy_static;

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
            return (false, "Enter atleast 3 elements");
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
            Self::char_contained_in(c, "()") || 
            pres_map.contains_key(&c) ||
            c == '.'
            )}) 
        {
            false
        } else {
            true
        }
    }

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

// Uses shunting_yard algorithm to handle rpn
#[allow(unused)]
pub mod shunting_yard {
    use super::*;

    pub fn convert_in_to_post_fix(input: &str) -> Result<Stack, Box<dyn Error>> {
        let mut operators = Stack::new();
        let mut output = Stack::new();
        
        // Clean spaces from string
        let input = input.replace(' ', "");
        
        let mut digit_tracker = false;
        let mut number_as_string: String = String::new();
        
        // True for alge, false for num
        let conversion_type_has_alge: bool = input.chars().any(|c| c.is_alphabetic());

        // Loop through chars in input
        for token in input.chars() {
            // If digit
            if handle_non_op_token(&token, &mut digit_tracker, &mut number_as_string) {
                continue;
            }

            // Convert string to f64 and push it to the output
            // Reset digit_tracker and number_as_string
            if digit_tracker {
                push_conversion_type(&mut output, number_as_string, conversion_type_has_alge)?;
                digit_tracker = false;
                number_as_string = "".to_string();
            }

            // If Operator or Bracket 
            if let Err(e) = handle_operators(&token, &mut operators, &mut output) {
                return Err(e);
            }
        }
        if digit_tracker { 
            push_conversion_type(&mut output, number_as_string, conversion_type_has_alge)?;
        }    
        while let Some(ops) = operators.pop() {
            output.push(ops);
        }
        Ok(output)
        
    }

    fn handle_operators(token: &char, operators: &mut Stack, output: &mut Stack) -> Result<(), Box<std::io::Error>> {
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
}
// Use post-traversal of a ast_tree to handle rpn
#[allow(dead_code)]
pub mod ast_tree {
    use super::*;

    #[derive(Clone, Debug)]
    struct Node {
        data: MathValue,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }
    
    impl Node {
        fn new(data: MathValue, left: Option<Node>, right: Option<Node>) -> Self {
            Node {
                data,
                left: left.map(Box::new),
                right: right.map(Box::new),
            }
        }
    }

    #[derive(Clone)]
    struct Parser {
        tokens: Vec<MathValue>,
        current_token_index: usize,
    }

    impl Parser {
        fn try_from(input: &str) -> Result<Parser, Box<dyn Error>> {
            // Clean Input
            let input = input.replace(' ', "");
            let tokens_stack = Stack::try_from(input.as_str())?;
            let tokens = Vec::from(tokens_stack.elements);
            Ok(Parser {tokens, current_token_index: 0})
        }
    
        fn advance(&mut self) {
            if self.current_token_index < self.tokens.len() - 1 {
                self.current_token_index += 1;
            }
        }
    
        fn current_token(&self) -> &MathValue {
            &self.tokens[self.current_token_index]
        }

        fn match_token(&mut self, token_type: char) -> bool {
            match self.current_token() {
                MathValue::Op(op) if *op == token_type => {
                    self.advance();
                    true
                },
                _ => false
            }
    
        }

        fn parse_expression(&mut self) -> Node {
            let mut node = self.parse_term();
            loop{
                match self.current_token() {
                    MathValue::Op('+') | MathValue::Op('-') => {
                        let token = self.current_token().clone();
                        self.advance();
                        node = Node::new(token, Some(node), Some(self.parse_term()));
                    },
                    _ => break,
                }
            }
            node
        }

        fn parse_term(&mut self) -> Node {
            let mut node = self.parse_expo();
            loop {
                match self.current_token() {
                    MathValue::Op('*') | MathValue::Op('/') => {
                        let token = self.current_token().clone();
                        self.advance();
                        node = Node::new(token, Some(node), Some(self.parse_expo()));
                    },
                    _ => break,
                }
            }
            node
        }

        fn parse_expo(&mut self) -> Node {
            let mut node = self.parse_factor();
            loop {
                match self.current_token() {
                    MathValue::Op('^') => {
                        let token = self.current_token().clone();
                        self.advance();
                        node = Node::new(token, Some(node), Some(self.parse_factor()));
                    },
                    _ => break,
                }
            }
            node
        }

        fn parse_factor(&mut self) -> Node {
            let token = self.current_token().clone();
            if self.match_token('(') {
                let node = self.parse_expression();
                self.match_token(')');
                node
            } else if let MathValue::Num(_) | MathValue::Alge(_) = token {
                self.advance();
                Node::new(token, None, None)
            } else {
                panic!("Unknown Factor");
            }
        }

    }

    fn traverse_tree(node: &Node, stack: &mut Stack) {
        if let Some(n) = &node.left {
            traverse_tree(&n, stack);
        } 
        if let Some(n) = &node.right {
            traverse_tree(&n, stack);
        }
        match &node.data {
            MathValue::Alge(_) => stack.push(node.data.clone()),
            MathValue::Num(_) => stack.push(node.data.clone()),
            MathValue::Op(_) => stack.push(node.data.clone()),
        }
    }
    pub fn convert_in_to_post_fix(input: &str) -> Result<Stack, Box<dyn Error>>{
        // Uses an post traversal of an ast tree to produce the 
        // rpn

        let mut parser = Parser::try_from(input).unwrap();
        let mut rpn= Stack::new();
        
        let ast = parser.parse_expression();
        traverse_tree(&ast, &mut rpn);

        Ok(rpn)
    }
}

#[cfg(test)]
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
 
#[cfg(test)]
mod parser_tests {
    use super::*;
    #[test]
    fn parser_num() {
        let input = "2^3+(3.1*(7-6)/4.0)";
        let expected = "2 ^ 3 + ( 3.1 * ( 7 - 6 ) / 4 )";
        assert_eq!(expected, Stack::try_from(input).unwrap().as_string());
    }
    #[test]
    fn parser_alge() {
        let input = "2x^y+(3.1*(x-6)/4.0a)";
        let expected = "2x ^ y + ( 3.1 * ( x - 6 ) / 4.0a )";
        assert_eq!(expected, Stack::try_from(input).unwrap().as_string());
    }        
}

#[cfg(test)]
mod ast_tests {
    use shunting_yard::convert_in_to_post_fix;

    use super::*;

    #[test]
    fn test_num_simple() {
        let input = "42 - 4234 * (4-234 + (43*43)) - 10";
        let expected = "42 4234 4 234 - 43 43 * + * - 10 -";
        assert_eq!(expected, convert_in_to_post_fix(input).unwrap().as_string());
    }
    #[test]
    fn test_num_complex() {
        let input = "(31 + 321)*(32+54)";
        let expected = "31 321 + 32 54 + *";
        assert_eq!(expected, convert_in_to_post_fix(input).unwrap().as_string());
    }
    #[test]
    fn test_brackets() {
        let input = "(4*((5+4)))^2";
        let expected = "4 5 4 + * 2 ^";
        assert_eq!(expected, convert_in_to_post_fix(input).unwrap().as_string());
    }
    #[test]
    fn test_alge_simple() {
        let input = "c*(a*(b*b+1) - (d123.32/f9.23))";
        let expected = "c a b b * 1 + * d123.32 f9.23 / - *";
        assert_eq!(expected, convert_in_to_post_fix(input).unwrap().as_string());
    }
    #[test]
    fn test_alge_complex() {
        let input = "(x + 87.31)*(x-31.23)";
        let expected = "x 87.31 + x 31.23 - *";
        assert_eq!(expected, convert_in_to_post_fix(input).unwrap().as_string());
    }        
}