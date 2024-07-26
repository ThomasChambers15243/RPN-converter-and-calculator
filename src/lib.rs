mod rpn_convert;
use rpn_convert::{MathValue, Stack};
use rpn_convert::shunting_yard::convert_in_to_post_fix;

pub fn solve_numerical(input: &str) -> Result<f64, Box<dyn std::error::Error>>{
    let mut total_stack:Vec<f64> = Vec::new();
    let rpn_form: Stack = convert_in_to_post_fix(input)?;
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
    println!("RPN form is: {}", convert_in_to_post_fix(input)?);
    Ok(total_stack.pop().unwrap())
}

fn calculate(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a+b,
        '-' => a-b,
        '*' => a*b,
        '/' => a/b,
        '^' => a.powf(b),
        _ => panic!("Invalid operations"),
    }
}


pub fn get_rpn(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(convert_in_to_post_fix(input)?.as_string())
}




#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn simple_addition() {
        let input = "3 + 7";
        let expected: f64 = 10.0;
        assert_eq!(expected, solve_numerical(input).unwrap());
    }

    #[test]
    fn simple_subtraction() {
        let input = "10 - 4";
        let expected: f64 = 6.0;
        assert_eq!(expected, solve_numerical(input).unwrap());
    }

    #[test]
    fn simple_multiplication() {
        let input = "5 * 8";
        let expected: f64 = 40.0;
        assert_eq!(expected, solve_numerical(input).unwrap());
    }

    #[test]
    fn simple_division() {
        let input = "20 / 4";
        let expected: f64 = 5.0;
        assert_eq!(expected, solve_numerical(input).unwrap());
    }

    #[test]
    fn mixed_operations() {
        let input = "4 + 2 * 5 - 8 / 4";
        let expected: f64 = 12.0;
        assert_eq!(expected, solve_numerical(input).unwrap());
    }

    #[test]
    fn complex_expression() {
        let input = "3 + 12 * ( 4 - 2 ) / 6";
        let expected: f64 = 7.0;
        assert_eq!(expected, solve_numerical(input).unwrap());
    }

    #[test]    
    fn invalid_operation() {
        let input = "10 = 2";
        let error = solve_numerical(input).unwrap_err();
        assert_eq!("Invalid operator: '='".to_string(), error.to_string());
    }

    #[test]
    fn division_by_zero() {
        let input = "10 / 0";
        assert_eq!(f64::INFINITY, solve_numerical(input).unwrap());
    }
}