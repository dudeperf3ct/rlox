use crate::{
    ast::{
        BinaryOp,
        Expr::{self, Logical},
        LogicalOp, UnaryOp,
    },
    errors::{EvalResult, RuntimeError},
    values::Value,
};

#[derive(Debug, Clone, Copy)]
pub struct Evaluator;

impl Evaluator {
    fn evaluate(expr: &Expr) -> EvalResult<Value> {
        match expr {
            // Types
            Expr::Nil => Ok(Value::Nil),
            Expr::Int(x) => Ok(Value::Int(*x as i64)),
            Expr::Boolean(x) => Ok(Value::Boolean(*x)),
            Expr::Float(x) => Ok(Value::Float(*x as f64)),
            Expr::String(x) => Ok(Value::String(x.clone())),
            // Grouping
            Expr::Grouping(group) => {
                let group_value = Evaluator::evaluate(group)?;
                Ok(group_value)
            }
            // UnaryOps
            Expr::Unary(op, right) => {
                let right_value = Evaluator::evaluate(right)?;
                match (op, right_value) {
                    (UnaryOp::Minus, Value::Int(x)) => Ok(Value::Int(-x)),
                    (UnaryOp::Minus, Value::Float(x)) => Ok(Value::Float(-x)),
                    (UnaryOp::Not, right_value) => Ok(Value::Boolean(!right_value.is_truth())),
                    (UnaryOp::Minus, _) => {
                        return Err(RuntimeError::Error(
                            "Unexpected unary expression".to_string(),
                        ));
                    }
                }
            }
            // BinaryOps
            Expr::Binary(left, op, right) => {
                let left_value = Evaluator::evaluate(left)?;
                let right_value = Evaluator::evaluate(right)?;
                match (op, left_value, right_value) {
                    (BinaryOp::Plus, Value::Int(left_value), Value::Int(right_value)) => {
                        Ok(Value::Int(left_value + right_value))
                    }
                    (BinaryOp::Plus, Value::Float(left_value), Value::Float(right_value)) => {
                        Ok(Value::Float(left_value + right_value))
                    }
                    (BinaryOp::Plus, Value::Int(left_value), Value::Float(right_value)) => {
                        Ok(Value::Float(left_value as f64 + right_value))
                    }
                    (BinaryOp::Plus, Value::Float(left_value), Value::Int(right_value)) => {
                        Ok(Value::Float(left_value + right_value as f64))
                    }
                    (BinaryOp::Plus, Value::String(left_value), Value::String(right_value)) => {
                        Ok(Value::String(left_value + &right_value))
                    }
                    (BinaryOp::Minus, Value::Int(left_value), Value::Int(right_value)) => {
                        Ok(Value::Int(left_value - right_value))
                    }
                    (BinaryOp::Minus, Value::Float(left_value), Value::Float(right_value)) => {
                        Ok(Value::Float(left_value - right_value))
                    }
                    (BinaryOp::Minus, Value::Int(left_value), Value::Float(right_value)) => {
                        Ok(Value::Float(left_value as f64 - right_value))
                    }
                    (BinaryOp::Minus, Value::Float(left_value), Value::Int(right_value)) => {
                        Ok(Value::Float(left_value - right_value as f64))
                    }
                    (BinaryOp::Star, Value::Int(left_value), Value::Int(right_value)) => {
                        Ok(Value::Int(left_value * right_value))
                    }
                    (BinaryOp::Star, Value::Float(left_value), Value::Float(right_value)) => {
                        Ok(Value::Float(left_value * right_value))
                    }
                    (BinaryOp::Star, Value::Int(left_value), Value::Float(right_value)) => {
                        Ok(Value::Float(left_value as f64 * right_value))
                    }
                    (BinaryOp::Star, Value::Float(left_value), Value::Int(right_value)) => {
                        Ok(Value::Float(left_value * right_value as f64))
                    }
                    // Handle case of division by 0
                    (BinaryOp::Slash, Value::Float(_) | Value::Int(_), Value::Int(0)) => {
                        return Err(RuntimeError::Error(("Cannot divide by 0.").to_string()));
                    }
                    (BinaryOp::Slash, Value::Float(_) | Value::Int(_), Value::Float(f))
                        if f == 0.0 =>
                    {
                        return Err(RuntimeError::Error(("Cannot divide by 0.").to_string()));
                    }
                    (BinaryOp::Slash, Value::Int(left_value), Value::Int(right_value)) => {
                        Ok(Value::Int(left_value / right_value))
                    }
                    (BinaryOp::Slash, Value::Float(left_value), Value::Float(right_value)) => {
                        Ok(Value::Float(left_value / right_value))
                    }
                    (BinaryOp::Slash, Value::Int(left_value), Value::Float(right_value)) => {
                        Ok(Value::Float(left_value as f64 / right_value))
                    }
                    (BinaryOp::Slash, Value::Float(left_value), Value::Int(right_value)) => {
                        Ok(Value::Float(left_value / right_value as f64))
                    }
                    // PartialEq on Value takes care of mixed numeric
                    (BinaryOp::Lt, Value::Int(left_value), Value::Int(right_value)) => {
                        Ok(Value::Boolean(left_value < right_value))
                    }
                    (BinaryOp::LtEq, Value::Int(left_value), Value::Int(right_value)) => {
                        Ok(Value::Boolean(left_value <= right_value))
                    }
                    (BinaryOp::Gt, Value::Int(left_value), Value::Int(right_value)) => {
                        Ok(Value::Boolean(left_value > right_value))
                    }
                    (BinaryOp::GtEq, Value::Int(left_value), Value::Int(right_value)) => {
                        Ok(Value::Boolean(left_value >= right_value))
                    }
                    (BinaryOp::Lt, Value::Float(left_value), Value::Float(right_value)) => {
                        Ok(Value::Boolean(left_value < right_value))
                    }
                    (BinaryOp::LtEq, Value::Float(left_value), Value::Float(right_value)) => {
                        Ok(Value::Boolean(left_value <= right_value))
                    }
                    (BinaryOp::Gt, Value::Float(left_value), Value::Float(right_value)) => {
                        Ok(Value::Boolean(left_value > right_value))
                    }
                    (BinaryOp::GtEq, Value::Float(left_value), Value::Float(right_value)) => {
                        Ok(Value::Boolean(left_value >= right_value))
                    }
                    (BinaryOp::EqEq, left_value, right_value) => {
                        Ok(Value::Boolean(left_value == right_value))
                    }
                    (BinaryOp::BangEq, left_value, right_value) => {
                        Ok(Value::Boolean(left_value != right_value))
                    }
                    (
                        BinaryOp::Minus
                        | BinaryOp::Star
                        | BinaryOp::Slash
                        | BinaryOp::Lt
                        | BinaryOp::Gt
                        | BinaryOp::LtEq
                        | BinaryOp::GtEq,
                        _,
                        _,
                    ) => {
                        return Err(RuntimeError::Error(
                            ("Operands must be numbers.").to_string(),
                        ));
                    }
                    (bop, left_value, right_value) => {
                        return Err(RuntimeError::Error(format!(
                            "unexpected binary operation {bop} with operands {left_value}, {right_value}"
                        )));
                    }
                }
            }
            Expr::Logical(left, op, right) => match op {
                LogicalOp::And => {
                    let left_value = Evaluator::evaluate(left)?;
                    if !left_value.is_truth() {
                        Ok(left_value)
                    } else {
                        let right_value = Evaluator::evaluate(right)?;
                        Ok(right_value)
                    }
                }
                LogicalOp::Or => {
                    let left_value = Evaluator::evaluate(left)?;
                    if left_value.is_truth() {
                        Ok(left_value)
                    } else {
                        let right_value = Evaluator::evaluate(right)?;
                        Ok(right_value)
                    }
                }
                _ => unreachable!(),
            },

            _ => todo!("Implement rest"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lexer::tokenize, parser::Parser};

    macro_rules! test_eval {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let tokens = tokenize($source).unwrap();
                let mut parser = Parser::new(tokens);
                let result = Evaluator::evaluate(&parser.parse().unwrap());

                assert_eq!(result.unwrap(), $expected);
            }
        };

        (error $name:ident, $source:expr) => {
            #[test]
            fn $name() {
                let tokens = tokenize($source).unwrap();
                let mut parser = Parser::new(tokens);
                let result = Evaluator::evaluate(&parser.parse().unwrap());

                assert!(
                    result.is_err(),
                    "Expected parsing to fail for: {:?}",
                    $source
                );
            }
        };
    }

    // Types
    test_eval!(eval_integer_literal, "42", Value::Int(42));
    test_eval!(eval_float_literal, "42.0", Value::Float(42.0));
    test_eval!(eval_nil_literal, "nil", Value::Nil);
    test_eval!(eval_true_literal, "true", Value::Boolean(true));
    test_eval!(eval_false_literal, "false", Value::Boolean(false));

    // Binary ops
    test_eval!(eval_plus_int, "2+3", Value::Int(5));
    test_eval!(eval_plus_float, "2+3.0", Value::Float(5.0));
    test_eval!(
        eval_plus_string,
        r#""a"+"b""#,
        Value::String("ab".to_string())
    );
    test_eval!(eval_minus_int, "2-3", Value::Int(-1));
    test_eval!(eval_minus_float, "2-3.0", Value::Float(-1.0));
    test_eval!(eval_mul_int, "2*3", Value::Int(6));
    test_eval!(eval_mul_float, "2*3.0", Value::Float(6.0));
    test_eval!(eval_div_int, "2/3", Value::Int(0));
    // Todo assert two float within tolerance
    test_eval!(eval_div_float, "3/3.0", Value::Float(1.0));
    test_eval!(error eval_div_by_zero, "2/0.0");
    test_eval!(error eval_div_by_zero_both, "0/0.0");
    test_eval!(eval_equality, "1 != 2", Value::Boolean(true));
    test_eval!(eval_not_equality, "1 == 2", Value::Boolean(false));

    // Grouping
    test_eval!(eval_grouping, "(1 + 2) * 3", Value::Int(9));

    // UnaryOps
    test_eval!(eval_unary_minus, "-42", Value::Int(-42));
    test_eval!(eval_unary_nested, "--42", Value::Int(42));
    test_eval!(eval_unary_not_false, "!false", Value::Boolean(true));
    test_eval!(eval_unary_not_true, "!true", Value::Boolean(false));

    // Logical
    test_eval!(eval_logical_and, "true and false", Value::Boolean(false));
    test_eval!(eval_logical_or, "true or false", Value::Boolean(true));
    test_eval!(
        eval_or_short_circuit,
        "true or (1 / 0 > 0)",
        Value::Boolean(true)
    );
    test_eval!(
        eval_and_short_circuit,
        "false and (1 / 0 > 0)",
        Value::Boolean(false)
    );

    // Comparison
    test_eval!(eval_comparison, "1 + 2 > 2", Value::Boolean(true));
}
