//use my_crate::solve_numerical;


// Integration tests for rpn_convert
#[cfg(test)]
mod integration {
    // Solve Numerical
    mod test_solve_numerical {
        use rpn::solve_numerical;
        #[test]    
        fn invalid_operation() {
            let error = solve_numerical("10=2").unwrap_err();
            assert_eq!("Invalid operator: '='".to_string(), error.to_string());
        }

        #[test]
        fn division_by_zero() {
            assert_eq!(f64::INFINITY, solve_numerical("10/0").unwrap());
        }

        #[test]
        fn negatives() {
            assert_eq!(10.0, solve_numerical("(!5+!5) * !1").unwrap());
        }

        #[test]
        fn simple_addition() {
            assert_eq!(10.0, solve_numerical("3 + 7").unwrap());
        }

        #[test]
        fn simple_subtraction() {
            assert_eq!(6.0, solve_numerical("10 - 4").unwrap());
        }

        #[test]
        fn simple_multiplication() {
            assert_eq!(40.0, solve_numerical("5*8").unwrap());
        }

        #[test]
        fn simple_division() {
            assert_eq!(5.0, solve_numerical("20 / 4").unwrap());
        }

        #[test]
        fn mixed_operations() {
            assert_eq!(12.0, solve_numerical("4 + 2 * 5 - 8 / 4").unwrap());
        }

        #[test]
        fn complex_expression() {
            assert_eq!(-0.41000000000000014, solve_numerical("3.34 + 12 * ( 4 - 2 ) / !6.4").unwrap());
        }
    }


    mod test_get_rpn {
        //use super::test_get_rpn::*;
        use rpn::{
            get_rpn_yard,
            get_rpn_tree
        };
        type rpn_return = Result<String, Box<dyn std::error::Error>>;

        pub fn negatives(func: fn(&str) -> rpn_return) {
            assert_eq!("-5 -5 + -1 *", 
            func("(!5+!5) * !1").unwrap());
        }

        pub fn rpn_1(func: fn(&str) -> rpn_return) {
            assert_eq!("31 321 + 32 54 + *", 
            func("(31 + 321)*(32+54)").unwrap());
        }

        pub fn rpn_2(func: fn(&str) -> rpn_return) {
            assert_eq!("c a b b * 1 + * d123.32 f9.23 / - *", 
            func("c*(a*(b*b+1) - (d123.32/f9.23))").unwrap());
        }

        pub fn rpn_3(func: fn(&str) -> rpn_return) {
            assert_eq!("-4.3a b 2 ^ -10 - x 1 2 / / * +",
            func("!4.3a + (b^2-!10)*(x/(1/2))").unwrap());
        }

        #[test]
        fn test_negatives() {
            negatives(get_rpn_yard);
            negatives(get_rpn_tree);
        }
        #[test]
        fn test_rpn_1() {
            rpn_1(get_rpn_yard);
            rpn_1(get_rpn_tree);
        }
        #[test]
        fn test_rpn_2() {
            rpn_2(get_rpn_yard);
            rpn_2(get_rpn_tree);
        }
        #[test]
        fn test_rpn_3() {
            rpn_3(get_rpn_yard);
            rpn_3(get_rpn_tree);
        }

    }
}