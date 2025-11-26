//! Variable mapping from Milkdrop to WGSL

use crate::error::{CodegenError, Result};

pub struct VariableMapper;

impl VariableMapper {
    pub fn new() -> Self {
        Self
    }
    
    /// Map a Milkdrop variable to WGSL
    pub fn map_variable(&self, var: &str) -> Result<String> {
        match var {
            // Coordinates
            "x" => Ok("vars.x".to_string()),
            "y" => Ok("vars.y".to_string()),
            "rad" => Ok("vars.rad".to_string()),
            "ang" => Ok("vars.ang".to_string()),
            
            // Audio
            "bass" => Ok("vars.bass".to_string()),
            "mid" => Ok("vars.mid".to_string()),
            "treb" => Ok("vars.treb".to_string()),
            "bass_att" => Ok("vars.bass_att".to_string()),
            "mid_att" => Ok("vars.mid_att".to_string()),
            "treb_att" => Ok("vars.treb_att".to_string()),
            
            // Time
            "time" => Ok("vars.time".to_string()),
            "frame" => Ok("vars.frame".to_string()),
            "fps" => Ok("vars.fps".to_string()),
            
            // Q variables
            var if var.starts_with('q') && var.len() > 1 => {
                let num: usize = var[1..].parse()
                    .map_err(|_| CodegenError::InvalidVariable(var.to_string()))?;
                if num >= 1 && num <= 64 {
                    Ok(format!("vars.q[{}]", num - 1))
                } else {
                    Err(CodegenError::InvalidVariable(format!("q{} out of range (1-64)", num)))
                }
            }
            
            // Unknown variable
            _ => Err(CodegenError::InvalidVariable(var.to_string())),
        }
    }
}

impl Default for VariableMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coordinate_variables() {
        let mapper = VariableMapper::new();
        assert_eq!(mapper.map_variable("x").unwrap(), "vars.x");
        assert_eq!(mapper.map_variable("y").unwrap(), "vars.y");
        assert_eq!(mapper.map_variable("rad").unwrap(), "vars.rad");
        assert_eq!(mapper.map_variable("ang").unwrap(), "vars.ang");
    }
    
    #[test]
    fn test_audio_variables() {
        let mapper = VariableMapper::new();
        assert_eq!(mapper.map_variable("bass").unwrap(), "vars.bass");
        assert_eq!(mapper.map_variable("mid").unwrap(), "vars.mid");
        assert_eq!(mapper.map_variable("treb").unwrap(), "vars.treb");
    }
    
    #[test]
    fn test_q_variables() {
        let mapper = VariableMapper::new();
        assert_eq!(mapper.map_variable("q1").unwrap(), "vars.q[0]");
        assert_eq!(mapper.map_variable("q32").unwrap(), "vars.q[31]");
        assert_eq!(mapper.map_variable("q64").unwrap(), "vars.q[63]");
    }
    
    #[test]
    fn test_invalid_variable() {
        let mapper = VariableMapper::new();
        assert!(mapper.map_variable("invalid").is_err());
        assert!(mapper.map_variable("q0").is_err());
        assert!(mapper.map_variable("q65").is_err());
    }
}
