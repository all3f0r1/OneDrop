//! Mathematical functions for MilkDrop expressions.
//!
//! This module provides all the mathematical functions needed for MilkDrop presets,
//! as evalexpr 13.0 does not include trigonometric or advanced math functions by default.

use evalexpr::{Function, HashMapContext, DefaultNumericTypes, ContextWithMutableFunctions, Value};

/// Register all MilkDrop math functions in a HashMapContext.
pub fn register_math_functions(context: &mut HashMapContext<DefaultNumericTypes>) {
    // Trigonometric functions
    context.set_function("sin".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.sin()))
    })).ok();
    
    context.set_function("cos".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.cos()))
    })).ok();
    
    context.set_function("tan".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.tan()))
    })).ok();
    
    context.set_function("asin".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.asin()))
    })).ok();
    
    context.set_function("acos".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.acos()))
    })).ok();
    
    context.set_function("atan".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.atan()))
    })).ok();
    
    context.set_function("atan2".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 2 {
                if let (Ok(y), Ok(x)) = (tuple[0].as_number(), tuple[1].as_number()) {
                    let y: f64 = y;
                    let x: f64 = x;
                    let y: f64 = y;
                    let x: f64 = x;
                    return Ok(Value::Float(y.atan2(x)));
                }
            }
        }
        Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
            expected: 2..=2,
            actual: 1,
        })
    })).ok();
    
    // Exponential and logarithmic functions
    context.set_function("sqrt".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.sqrt()))
    })).ok();
    
    context.set_function("pow".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 2 {
                if let (Ok(base), Ok(exp)) = (tuple[0].as_number(), tuple[1].as_number()) {
                    let base: f64 = base;
                    let exp: f64 = exp;
                    return Ok(Value::Float(base.powf(exp)));
                }
            }
        }
        Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
            expected: 2..=2,
            actual: 1,
        })
    })).ok();
    
    context.set_function("exp".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.exp()))
    })).ok();
    
    context.set_function("log".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.ln()))
    })).ok();
    
    context.set_function("ln".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.ln()))
    })).ok();
    
    context.set_function("log10".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.log10()))
    })).ok();
    
    // Absolute value and sign
    context.set_function("abs".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.abs()))
    })).ok();
    
    context.set_function("sign".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| {
            if n > 0.0 {
                Value::Float(1.0)
            } else if n < 0.0 {
                Value::Float(-1.0)
            } else {
                Value::Float(0.0)
            }
        })
    })).ok();
    
    // Rounding functions
    context.set_function("fract".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.fract()))
    })).ok();
    
    context.set_function("trunc".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.trunc()))
    })).ok();
    
    // Modulo and clamping
    context.set_function("fmod".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 2 {
                if let (Ok(a), Ok(b)) = (tuple[0].as_number(), tuple[1].as_number()) {
                    let a: f64 = a;
                    let b: f64 = b;
                    return Ok(Value::Float(a % b));
                }
            }
        }
        Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
            expected: 2..=2,
            actual: 1,
        })
    })).ok();
    
    context.set_function("clamp".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 3 {
                if let (Ok(value), Ok(min_val), Ok(max_val)) = 
                    (tuple[0].as_number(), tuple[1].as_number(), tuple[2].as_number()) {
                    let value: f64 = value;
                    let min_val: f64 = min_val;
                    let max_val: f64 = max_val;
                    return Ok(Value::Float(value.max(min_val).min(max_val)));
                }
            }
        }
        Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
            expected: 3..=3,
            actual: 1,
        })
    })).ok();
    
    // Hyperbolic functions
    context.set_function("sinh".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.sinh()))
    })).ok();
    
    context.set_function("cosh".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.cosh()))
    })).ok();
    
    context.set_function("tanh".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.tanh()))
    })).ok();
    
    // Additional useful functions
    context.set_function("sqr".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n * n))
    })).ok();
    
    context.set_function("rad".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.to_radians()))
    })).ok();
    
    context.set_function("deg".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.to_degrees()))
    })).ok();
    
    // Random and comparison functions
    context.set_function("rand".into(), Function::new(|arg| {
        use std::time::{SystemTime, UNIX_EPOCH};
        let max = arg.as_number()?;
        let max: f64 = max;
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let random = ((seed % 1000000) as f64 / 1000000.0) * max;
        Ok(Value::Float(random))
    })).ok();
    
    context.set_function("above".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 2 {
                if let (Ok(a), Ok(b)) = (tuple[0].as_number(), tuple[1].as_number()) {
                    let a: f64 = a;
                    let b: f64 = b;
                    return Ok(Value::Float(if a > b { 1.0 } else { 0.0 }));
                }
            }
        }
        Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
            expected: 2..=2,
            actual: 1,
        })
    })).ok();
    
    context.set_function("below".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 2 {
                if let (Ok(a), Ok(b)) = (tuple[0].as_number(), tuple[1].as_number()) {
                    let a: f64 = a;
                    let b: f64 = b;
                    return Ok(Value::Float(if a < b { 1.0 } else { 0.0 }));
                }
            }
        }
        Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
            expected: 2..=2,
            actual: 1,
        })
    })).ok();
    
    context.set_function("equal".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 2 {
                if let (Ok(a), Ok(b)) = (tuple[0].as_number(), tuple[1].as_number()) {
                    let a: f64 = a;
                    let b: f64 = b;
                    return Ok(Value::Float(if (a - b).abs() < 1e-10 { 1.0 } else { 0.0 }));
                }
            }
        }
        Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
            expected: 2..=2,
            actual: 1,
        })
    })).ok();
    
    // Boolean functions
    context.set_function("bnot".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(if n == 0.0 { 1.0 } else { 0.0 }))
    })).ok();
    
    context.set_function("band".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 2 {
                if let (Ok(a), Ok(b)) = (tuple[0].as_number(), tuple[1].as_number()) {
                    let a: f64 = a;
                    let b: f64 = b;
                    return Ok(Value::Float(if a != 0.0 && b != 0.0 { 1.0 } else { 0.0 }));
                }
            }
        }
        Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
            expected: 2..=2,
            actual: 1,
        })
    })).ok();
    
    context.set_function("bor".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 2 {
                if let (Ok(a), Ok(b)) = (tuple[0].as_number(), tuple[1].as_number()) {
                    let a: f64 = a;
                    let b: f64 = b;
                    return Ok(Value::Float(if a != 0.0 || b != 0.0 { 1.0 } else { 0.0 }));
                }
            }
        }
        Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
            expected: 2..=2,
            actual: 1,
        })
    })).ok();
    
    // Type conversion
    context.set_function("int".into(), Function::new(|arg| {
        arg.as_number().map(|n: f64| Value::Float(n.trunc()))
    })).ok();
    
    // MilkDrop-style if function (accepts Float condition)
    context.set_function("milkif".into(), Function::new(|arg| {
        if let Ok(tuple) = arg.as_tuple() {
            if tuple.len() == 3 {
                // Get condition (Float or Boolean)
                let condition = match &tuple[0] {
                    Value::Float(f) => *f != 0.0,
                    Value::Int(i) => *i != 0,
                    Value::Boolean(b) => *b,
                    _ => return Err(evalexpr::EvalexprError::TypeError {
                        expected: vec![evalexpr::ValueType::Float, evalexpr::ValueType::Boolean],
                        actual: tuple[0].clone(),
                    }),
                };
                
                // Return true_val or false_val, converting Int to Float if needed
                let result = if condition { &tuple[1] } else { &tuple[2] };
                match result {
                    Value::Int(i) => Ok(Value::Float(*i as f64)),
                    other => Ok(other.clone()),
                }
            } else {
                Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
                    expected: 3..=3,
                    actual: tuple.len(),
                })
            }
        } else {
            Err(evalexpr::EvalexprError::WrongFunctionArgumentAmount {
                expected: 3..=3,
                actual: 1,
            })
        }
    })).ok();
}

/// List of all registered math functions.
pub fn list_math_functions() -> Vec<&'static str> {
    vec![
        // Trigonometric
        "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
        // Exponential and logarithmic
        "sqrt", "pow", "exp", "log", "ln", "log10",
        // Absolute and sign
        "abs", "sign",
        // Rounding
        "fract", "trunc",
        // Modulo and clamping
        "fmod", "clamp",
        // Hyperbolic
        "sinh", "cosh", "tanh",
        // Additional
        "sqr", "rad", "deg",
        // Random and comparison
        "rand", "above", "below", "equal",
        // Boolean
        "bnot", "band", "bor",
        // Type conversion
        "int",
        // Control flow
        "milkif",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use evalexpr::ContextWithMutableVariables;
    
    #[test]
    fn test_register_math_functions() {
        let mut context = HashMapContext::<DefaultNumericTypes>::new();
        register_math_functions(&mut context);
        
        // Test that basic functions work
        assert!(evalexpr::eval_number_with_context("sin(0)", &context).is_ok());
        assert!(evalexpr::eval_number_with_context("cos(0)", &context).is_ok());
        assert!(evalexpr::eval_number_with_context("sqrt(4)", &context).is_ok());
    }
    
    #[test]
    fn test_sin_function() {
        let mut context = HashMapContext::<DefaultNumericTypes>::new();
        register_math_functions(&mut context);
        
        let result = evalexpr::eval_number_with_context("sin(0)", &context).unwrap();
        assert_relative_eq!(result, 0.0, epsilon = 1e-10);
        
        let result = evalexpr::eval_number_with_context("sin(1.5707963267948966)", &context).unwrap();
        assert_relative_eq!(result, 1.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_cos_function() {
        let mut context = HashMapContext::<DefaultNumericTypes>::new();
        register_math_functions(&mut context);
        
        let result = evalexpr::eval_number_with_context("cos(0)", &context).unwrap();
        assert_relative_eq!(result, 1.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_sqrt_function() {
        let mut context = HashMapContext::<DefaultNumericTypes>::new();
        register_math_functions(&mut context);
        
        let result = evalexpr::eval_number_with_context("sqrt(16)", &context).unwrap();
        assert_relative_eq!(result, 4.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_abs_function() {
        let mut context = HashMapContext::<DefaultNumericTypes>::new();
        register_math_functions(&mut context);
        
        let result = evalexpr::eval_number_with_context("abs(-5)", &context).unwrap();
        assert_relative_eq!(result, 5.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_pow_function() {
        let mut context = HashMapContext::<DefaultNumericTypes>::new();
        register_math_functions(&mut context);
        
        let result = evalexpr::eval_number_with_context("pow(2, 3)", &context).unwrap();
        assert_relative_eq!(result, 8.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_complex_expression() {
        let mut context = HashMapContext::<DefaultNumericTypes>::new();
        register_math_functions(&mut context);
        context.set_value("time".into(), Value::Float(1.0)).unwrap();
        
        let result = evalexpr::eval_number_with_context(
            "sin(time) * cos(time * 2) + sqrt(abs(time - 0.5))", 
            &context
        ).unwrap();
        
        let time = 1.0_f64;
        let expected = time.sin() * (time * 2.0).cos() + (time - 0.5).abs().sqrt();
        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }
}
