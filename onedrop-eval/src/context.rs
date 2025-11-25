//! Execution context for Milkdrop expressions.

use evalexpr::{Context, ContextWithMutableVariables, HashMapContext, Value};
use std::collections::HashMap;

/// Execution context containing all Milkdrop variables.
#[derive(Debug, Clone)]
pub struct MilkContext {
    /// Internal evalexpr context
    context: HashMapContext,
    
    /// User-defined variables (q1-q64)
    q_vars: [f64; 64],
    
    /// Custom variables defined in equations
    custom_vars: HashMap<String, f64>,
}

impl MilkContext {
    /// Create a new context with default values.
    pub fn new() -> Self {
        let mut context = HashMapContext::new();
        
        // Initialize default values for all Milkdrop variables
        Self::init_defaults(&mut context);
        
        Self {
            context,
            q_vars: [0.0; 64],
            custom_vars: HashMap::new(),
        }
    }
    
    /// Initialize default values for built-in variables.
    fn init_defaults(ctx: &mut HashMapContext) {
        // Time variables
        ctx.set_value("time".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("frame".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("fps".to_string(), Value::Float(60.0)).ok();
        
        // Audio variables (bass, mid, treble)
        ctx.set_value("bass".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("mid".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("treb".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("bass_att".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("mid_att".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("treb_att".to_string(), Value::Float(0.0)).ok();
        
        // Geometric variables (per-pixel)
        ctx.set_value("x".to_string(), Value::Float(0.5)).ok();
        ctx.set_value("y".to_string(), Value::Float(0.5)).ok();
        ctx.set_value("rad".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ang".to_string(), Value::Float(0.0)).ok();
        
        // Motion parameters
        ctx.set_value("zoom".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("zoomexp".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("rot".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("warp".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("cx".to_string(), Value::Float(0.5)).ok();
        ctx.set_value("cy".to_string(), Value::Float(0.5)).ok();
        ctx.set_value("dx".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("dy".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("sx".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("sy".to_string(), Value::Float(1.0)).ok();
        
        // Wave parameters
        ctx.set_value("wave_r".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("wave_g".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("wave_b".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("wave_a".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("wave_x".to_string(), Value::Float(0.5)).ok();
        ctx.set_value("wave_y".to_string(), Value::Float(0.5)).ok();
        ctx.set_value("wave_mystery".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("wave_mode".to_string(), Value::Float(0.0)).ok();
        
        // Border parameters
        ctx.set_value("ob_size".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ob_r".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ob_g".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ob_b".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ob_a".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ib_size".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ib_r".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ib_g".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ib_b".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("ib_a".to_string(), Value::Float(0.0)).ok();
        
        // Motion vectors
        ctx.set_value("mv_x".to_string(), Value::Float(12.0)).ok();
        ctx.set_value("mv_y".to_string(), Value::Float(9.0)).ok();
        ctx.set_value("mv_dx".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("mv_dy".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("mv_l".to_string(), Value::Float(0.9)).ok();
        ctx.set_value("mv_r".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("mv_g".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("mv_b".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("mv_a".to_string(), Value::Float(0.0)).ok();
        
        // Decay and echo
        ctx.set_value("decay".to_string(), Value::Float(0.98)).ok();
        ctx.set_value("echo_zoom".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("echo_alpha".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("echo_orient".to_string(), Value::Float(0.0)).ok();
        
        // Initialize q variables (q1-q64)
        for i in 1..=64 {
            ctx.set_value(format!("q{}", i), Value::Float(0.0)).ok();
        }
    }
    
    /// Set a variable value.
    pub fn set(&mut self, name: &str, value: f64) {
        // Check if it's a q variable
        if name.starts_with('q') && name.len() > 1 {
            if let Ok(index) = name[1..].parse::<usize>() {
                if index > 0 && index <= 64 {
                    self.q_vars[index - 1] = value;
                    self.context.set_value(name.to_string(), Value::Float(value)).ok();
                    return;
                }
            }
        }
        
        // Set in context
        self.context.set_value(name.to_string(), Value::Float(value)).ok();
        
        // Track custom variables
        if !self.is_builtin(name) {
            self.custom_vars.insert(name.to_string(), value);
        }
    }
    
    /// Get a variable value.
    pub fn get(&self, name: &str) -> Option<f64> {
        // Check if it's a q variable
        if name.starts_with('q') && name.len() > 1 {
            if let Ok(index) = name[1..].parse::<usize>() {
                if index > 0 && index <= 64 {
                    return Some(self.q_vars[index - 1]);
                }
            }
        }
        
        // Get from context (evalexpr 13.0 API)
        match self.context.get_value(name) {
            Some(value) => match value {
                Value::Float(f) => Some(*f),
                Value::Int(i) => Some(*i as f64),
                Value::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
                _ => None,
            },
            None => None,
        }
    }
    
    /// Check if a variable name is a built-in Milkdrop variable.
    fn is_builtin(&self, name: &str) -> bool {
        matches!(
            name,
            "time" | "frame" | "fps" |
            "bass" | "mid" | "treb" | "bass_att" | "mid_att" | "treb_att" |
            "x" | "y" | "rad" | "ang" |
            "zoom" | "zoomexp" | "rot" | "warp" | "cx" | "cy" | "dx" | "dy" | "sx" | "sy" |
            "wave_r" | "wave_g" | "wave_b" | "wave_a" | "wave_x" | "wave_y" | "wave_mystery" | "wave_mode" |
            "ob_size" | "ob_r" | "ob_g" | "ob_b" | "ob_a" |
            "ib_size" | "ib_r" | "ib_g" | "ib_b" | "ib_a" |
            "mv_x" | "mv_y" | "mv_dx" | "mv_dy" | "mv_l" | "mv_r" | "mv_g" | "mv_b" | "mv_a" |
            "decay" | "echo_zoom" | "echo_alpha" | "echo_orient"
        ) || (name.starts_with('q') && name.len() > 1)
    }
    
    /// Get the internal evalexpr context.
    pub fn inner(&self) -> &HashMapContext {
        &self.context
    }
    
    /// Get a mutable reference to the internal evalexpr context.
    pub fn inner_mut(&mut self) -> &mut HashMapContext {
        &mut self.context
    }
    
    /// Get all q variables.
    pub fn q_vars(&self) -> &[f64; 64] {
        &self.q_vars
    }
    
    /// Get all custom variables.
    pub fn custom_vars(&self) -> &HashMap<String, f64> {
        &self.custom_vars
    }
    
    /// Set pixel position for per-pixel evaluation.
    pub fn set_pixel(&mut self, x: f64, y: f64, rad: f64, ang: f64) {
        self.set("x", x);
        self.set("y", y);
        self.set("rad", rad);
        self.set("ang", ang);
    }
    
    /// Set a variable (alias for set).
    pub fn set_var(&mut self, name: &str, value: f64) {
        self.set(name, value);
    }
    
    /// Set time variable.
    pub fn set_time(&mut self, time: f64) {
        self.set("time", time);
    }
    
    /// Set frame variable.
    pub fn set_frame(&mut self, frame: f64) {
        self.set("frame", frame);
    }
    
    /// Set audio variables (bass, mid, treble).
    pub fn set_audio(&mut self, bass: f64, mid: f64, treb: f64) {
        self.set("bass", bass);
        self.set("mid", mid);
        self.set("treb", treb);
    }
    
    /// Get a variable value (alias for get).
    pub fn get_var(&self, name: &str) -> Option<f64> {
        self.get(name)
    }
}

impl Default for MilkContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_context() {
        let ctx = MilkContext::new();
        assert_eq!(ctx.get("time"), Some(0.0));
        assert_eq!(ctx.get("fps"), Some(60.0));
    }

    #[test]
    fn test_set_get_variable() {
        let mut ctx = MilkContext::new();
        
        ctx.set("bass", 1.5);
        assert_eq!(ctx.get("bass"), Some(1.5));
        
        ctx.set("custom_var", 42.0);
        assert_eq!(ctx.get("custom_var"), Some(42.0));
    }

    #[test]
    fn test_q_variables() {
        let mut ctx = MilkContext::new();
        
        // Test all 64 q variables
        for i in 1..=64 {
            let name = format!("q{}", i);
            ctx.set(&name, i as f64);
            assert_eq!(ctx.get(&name), Some(i as f64));
        }
        
        // Verify q_vars array
        assert_eq!(ctx.q_vars()[0], 1.0);
        assert_eq!(ctx.q_vars()[63], 64.0);
    }

    #[test]
    fn test_custom_variables() {
        let mut ctx = MilkContext::new();
        
        ctx.set("my_var", 123.0);
        ctx.set("another_var", 456.0);
        
        assert_eq!(ctx.custom_vars().len(), 2);
        assert_eq!(ctx.custom_vars().get("my_var"), Some(&123.0));
        assert_eq!(ctx.custom_vars().get("another_var"), Some(&456.0));
    }
}
