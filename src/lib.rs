mod rpn_convert;
    use rpn_convert::{
    Validate,
    MathValue, Stack,
    shunting_yard,
    ast_tree,
};

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


pub fn get_rpn_yard(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (is_valid, msg) = Validate::validate_input(input);
    if is_valid {
        Ok(shunting_yard::convert_in_to_post_fix(input)?.as_string())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg)))
    }
}

pub fn get_rpn_tree(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (is_valid, msg) = Validate::validate_input(input);
    if is_valid {
        Ok(ast_tree::convert_in_to_post_fix(input)?.as_string())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg)))
    }
}