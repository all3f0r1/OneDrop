//! Execution context for Milkdrop equations.

use evalexpr::{HashMapContext, Context as EvalContext, Value};
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
        
        // Miscellaneous
        ctx.set_value("gamma".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("darken_center".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("wrap".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("invert".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("brighten".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("darken".to_string(), Value::Float(0.0)).ok();
        ctx.set_value("solarize".to_string(), Value::Float(0.0)).ok();
        
        // Monitor aspect ratio
        ctx.set_value("aspectx".to_string(), Value::Float(1.0)).ok();
        ctx.set_value("aspecty".to_string(), Value::Float(1.0)).ok();
        
        // Pixel position (normalized 0-1)
        ctx.set_value("pixelsx".to_string(), Value::Float(1024.0)).ok();
        ctx.set_value("pixelsy".to_string(), Value::Float(768.0)).ok();
        
        // Progress (for preset transitions)
        ctx.set_value("progress".to_string(), Value::Float(0.0)).ok();
        
        // Beat detection
        ctx.set_value("beat".to_string(), Value::Float(0.0)).ok();
    }
    
    /// Get the internal evalexpr context.
    pub fn inner(&self) -> &HashMapContext {
        &self.context
    }
    
    /// Get a mutable reference to the internal context.
    pub fn inner_mut(&mut self) -> &mut HashMapContext {
        &mut self.context
    }
    
    /// Set a variable value.
    pub fn set_var(&mut self, name: &str, value: f64) {
        // Handle q variables specially
        if name.starts_with('q') && name.len() > 1 {
            if let Ok(index) = name[1..].parse::<usize>() {
                if index >= 1 && index <= 64 {
                    self.q_vars[index - 1] = value;
                    self.context.set_value(name.to_string(), Value::Float(value)).ok();
                    return;
                }
            }
        }
        
        // Regular variable
        self.context.set_value(name.to_string(), Value::Float(value)).ok();
        self.custom_vars.insert(name.to_string(), value);
    }
    
    /// Get a variable value.
    pub fn get_var(&self, name: &str) -> Option<f64> {
        // Handle q variables
        if name.starts_with('q') && name.len() > 1 {
            if let Ok(index) = name[1..].parse::<usize>() {
                if index >= 1 && index <= 64 {
                    return Some(self.q_vars[index - 1]);
                }
            }
        }
        
        // Try context first
        if let Ok(value) = self.context.get_value(name) {
            if let Value::Float(f) = value {
                return Some(*f);
            } else if let Value::Int(i) = value {
                return Some(*i as f64);
            }
        }
        
        // Try custom vars
        self.custom_vars.get(name).copied()
    }
    
    /// Set time variable.
    pub fn set_time(&mut self, time: f64) {
        self.set_var("time", time);
    }
    
    /// Set frame variable.
    pub fn set_frame(&mut self, frame: f64) {
        self.set_var("frame", frame);
    }
    
    /// Set audio variables.
    pub fn set_audio(&mut self, bass: f64, mid: f64, treb: f64, bass_att: f64, mid_att: f64, treb_att: f64) {
        self.set_var("bass", bass);
        self.set_var("mid", mid);
        self.set_var("treb", treb);
        self.set_var("bass_att", bass_att);
        self.set_var("mid_att", mid_att);
        self.set_var("treb_att", treb_att);
    }
    
    /// Set pixel position (for per-pixel equations).
    pub fn set_pixel(&mut self, x: f64, y: f64, rad: f64, ang: f64) {
        self.set_var("x", x);
        self.set_var("y", y);
        self.set_var("rad", rad);
        self.set_var("ang", ang);
    }
    
    /// Initialize q variables from array.
    pub fn init_q_vars(&mut self, q_values: &[f64]) {
        for (i, &value) in q_values.iter().enumerate().take(64) {
            let var_name = format!("q{}", i + 1);
            self.set_var(&var_name, value);
        }
    }
    
    /// Get all q variables as array.
    pub fn get_q_vars(&self) -> [f64; 64] {
        self.q_vars
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
    fn test_context_creation() {
        let ctx = MilkContext::new();
        assert_eq!(ctx.get_var("time"), Some(0.0));
        assert_eq!(ctx.get_var("zoom"), Some(1.0));
    }

    #[test]
    fn test_set_get_var() {
        let mut ctx = MilkContext::new();
        ctx.set_var("test", 42.0);
        assert_eq!(ctx.get_var("test"), Some(42.0));
    }

    #[test]
    fn test_q_variables() {
        let mut ctx = MilkContext::new();
        ctx.set_var("q1", 1.0);
        ctx.set_var("q32", 32.0);
        ctx.set_var("q64", 64.0);
        
        assert_eq!(ctx.get_var("q1"), Some(1.0));
        assert_eq!(ctx.get_var("q32"), Some(32.0));
        assert_eq!(ctx.get_var("q64"), Some(64.0));
    }

    #[test]
    fn test_audio_vars() {
        let mut ctx = MilkContext::new();
        ctx.set_audio(0.5, 0.6, 0.7, 0.8, 0.9, 1.0);
        
        assert_eq!(ctx.get_var("bass"), Some(0.5));
        assert_eq!(ctx.get_var("mid"), Some(0.6));
        assert_eq!(ctx.get_var("treb"), Some(0.7));
    }
}
