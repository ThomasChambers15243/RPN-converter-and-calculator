use std::fmt::{self};
use std::collections::{VecDeque, HashMap};
use lazy_static::lazy_static;

#[derive(Debug)]
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

// Uses shunting_yard algorithm to handle rpn
pub mod shunting_yard {
    use super::*;
        
    struct OpStack {
        elements: Vec<MathValue>,
    }


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
            self.elements.iter().map(|token| 
                match token {
                    MathValue::Op(ch) => ch.to_string(),
                    MathValue::Alge(al) => al.to_string(),
                    MathValue::Num(num) => num.to_string(),
                }
            ).collect::<Vec<String>>().join(" ")
        }
        fn push(&mut self, token: MathValue) {
            self.elements.push_back(token);
        }
    }

    pub fn convert_in_to_post_fix(i: &str) -> Result<OutQueue, Box<dyn std::error::Error>> {
        let mut operators = OpStack::new();
        let mut output = OutQueue::new();
        
        // Clean spaces from string
        let input = i.replace(' ', "");
        
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
                if conversion_type_has_alge {
                    output.push(MathValue::Alge(number_as_string));
                } else {
                    output.push(MathValue::Num(number_as_string.parse::<f64>()?));
                }
                digit_tracker = false;
                number_as_string = "".to_string();
            }

            // If Operator or Bracket 
            if let Err(e) = handle_operators(&token, &mut operators, &mut output) {
                return Err(e);
            }
        }
        if digit_tracker { 
            if conversion_type_has_alge {
                output.push(MathValue::Alge(number_as_string))
            } else {
                output.push(MathValue::Num(number_as_string.parse::<f64>()?));
            }
        }    
        while let Some(ops) = operators.pop() {
            output.push(ops);
        }
        Ok(output)
        
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
            let expected = "5.2 8 +";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().queue_as_string());
        }

        #[test]
        fn simple_large() {
            let input = "50 + 2342 - 234324.8";
            let expected = "50 2342 + 234324.8 -";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().queue_as_string());
        }

        #[test]
        fn complex_small() {
            let input = "40 + 4 - 1 / (9 * 99))";
            let expected = "40 4 + 1 9 99 * / -";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().queue_as_string());
        }

        #[test]
        fn complex_large() {
            let input = "42 - 4234 * (4-234 + (43*43)) - 10";
            let expected = "42 4234 4 234 - 43 43 * + * - 10 -";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().queue_as_string());
        }

        #[test]
        fn quadratic() {
            let input = "(31 + 321)*(32+54)";
            let expected = "31 321 + 32 54 + *";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().queue_as_string());
        }

        #[test]
        fn should_error() {
            let input = "5+6=a";
            let expected = "Invalid operator: '='";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap_err().to_string());
        }

        #[test]
        fn alge_simple_small() {
            let input = "a+7b";
            let expected = "a 7b +";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().to_string());
        }
        #[test]
        fn alge_simple_large() {
            let input = "ab3213 + 131 - p * q";
            let expected = "ab3213 131 + p q * -";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().to_string());
        }
        #[test]
        fn alge_complex_small() {
            let input = "(x*x / (z-32.1c))";
            let expected = "x x * z 32.1c - /";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().to_string());
        }
        #[test]
        fn alge_complex_large() {
            let input = "c(a(b*b+1) - (d123.32/f9.23))";
            let expected = "c a b b * 1 + d123.32 f9.23 / -";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().to_string());
        }
        #[test]
        fn alge_quadratic() {
            let input = "(x + 87.31)*(x-31.23)";
            let expected = "x 87.31 + x 31.23 - *";
            assert_eq!(expected, convert_in_to_post_fix(input).unwrap().to_string());
        }
        // fn should_error() {
        //     let input = "";
        //     let expected = "";
            
        // }
        }
}

// Use post-traversal of a ast_tree to handle rpn
pub mod ast_tree {
    use core::num;

    use super::*;

    struct Node {
        data: MathValue,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }

    impl Node {
        fn new(self, data: MathValue) -> Node {
            Node {
                data: data,
                left: None,
                right: None,
            }
        }
    }

    pub struct Parser {
        tokens: Vec<MathValue>,
    }
    impl fmt::Display for Parser {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.parser_as_string())
        }
    }

    impl Parser {

        fn parser_as_string(&self) -> String {
            self.tokens.iter().map(|el| 
                match el {
                    MathValue::Op(op) => op.to_string(),
                    MathValue::Num(num) => num.to_string(),
                    MathValue::Alge(al) => al.to_string(),
                }
            ).collect::<Vec<String>>().join(" ")
        }

        pub fn try_from_nums(input: &str) -> Result<Parser, Box<dyn std::error::Error>> {
            let mut tokens: Vec<MathValue> = Vec::new();
            let mut number_as_string = String::from("");
            let mut digit_tracker = false;

            for token in input.chars() {
                if handle_non_op_token(&token, &mut digit_tracker, &mut number_as_string) {
                    continue;
                }
                if digit_tracker {
                    tokens.push(MathValue::Num(number_as_string.parse::<f64>()?));
                    digit_tracker = false;
                    number_as_string = "".to_string();
                }
                tokens.push(MathValue::Op(token));
                
            }
            if !number_as_string.is_empty() {
                tokens.push(MathValue::Num(number_as_string.parse::<f64>()?));
            }
            Ok(Parser{tokens})
        }

        // pub fn try_from_alge(input: &str) -> Result<Parser, Box<dyn std::error::Error>> {
        //     let mut tokens: Vec<MathValues> = Vec::new();
        // }
    }







    #[cfg(test)]
    mod Parser_tests {
        use super::*;
        #[test]
        fn parser_num() {
            let input = "2^3+(31*(7-6)/4)";
            let expected = "2 ^ 3 + ( 31 * ( 7 - 6 ) / 4 )";
            assert_eq!(expected, Parser::try_from_nums(input).unwrap().parser_as_string());
        }
        // fn parser_alge() {
        //     let input = "2x^y+(31*(x-6)/4a)";
        //     let expected = "2x ^ y + ( 31 * ( x - 6 ) / 4a )";
        //     assert_eq!(expected, Parser::try_from_alge(input).unwrap().parser_as_string());
        // }
    }
}
