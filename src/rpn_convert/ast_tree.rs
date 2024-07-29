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