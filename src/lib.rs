mod shunting_yard;
use shunting_yard::{
    OutQueue,MathValue,
    convert_numerical_in_to_post_fix,
    convert_algebra_in_to_post_fix,
};

pub fn solve_numerical(input: &str) -> Result<f64, Box<dyn std::error::Error>>{
    let mut total_stack:Vec<f64> = Vec::new();
    let rpn_form: OutQueue = convert_numerical_in_to_post_fix(input)?;
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

#[allow(dead_code)]
pub fn convert_in_to_post_fix(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Not a great way of doing it but its small.
    // True for Alge
    // False for num
    
    let conversion_type: bool = input.chars().any(|c| c.is_alphabetic());
 
    // Algebra
    if conversion_type {
        Ok(convert_algebra_in_to_post_fix(input)?.queue_as_string())
    } else {
    // Numerical
        Ok(convert_numerical_in_to_post_fix(input)?.queue_as_string())
    }
}




#[cfg(test)]
mod tests {
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
