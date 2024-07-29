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

#[cfg(test)]
mod shunting_yard_tests {
    use super::*;
    use shunting_yard::convert_in_to_post_fix;
    use rpn_convert_unit_tests::*;

    #[test]
    fn test_num_simple() {
        num_simple(convert_in_to_post_fix);
    }
    #[test]
    fn test_num_complex() {
        num_complex(convert_in_to_post_fix);
    }
    #[test]
    fn test_brackets() {
        brackets(convert_in_to_post_fix);
    }
    #[test]
    fn test_alge_simple() {
        alge_simple(convert_in_to_post_fix);
    }
    #[test]
    fn test_alge_complex() {
        alge_complex(convert_in_to_post_fix);
    }        
}
