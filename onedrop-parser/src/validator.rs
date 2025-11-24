//! Strict validation for preset parameters.

use crate::error::{ParseError, Result};
use crate::preset::{MilkPreset, PresetParameters};

/// Validation rules for preset parameters.
pub struct Validator {
    /// Allow out-of-range values (lenient mode)
    lenient: bool,
}

impl Validator {
    /// Create a new strict validator.
    pub fn new() -> Self {
        Self { lenient: false }
    }
    
    /// Create a lenient validator that allows out-of-range values.
    pub fn lenient() -> Self {
        Self { lenient: true }
    }
    
    /// Validate a complete preset.
    pub fn validate(&self, preset: &MilkPreset) -> Result<()> {
        self.validate_version(preset.version)?;
        self.validate_parameters(&preset.parameters)?;
        self.validate_equations(&preset.per_frame_equations, "per_frame")?;
        self.validate_equations(&preset.per_pixel_equations, "per_pixel")?;
        Ok(())
    }
    
    /// Validate preset version.
    fn validate_version(&self, version: u32) -> Result<()> {
        if version == 0 {
            return Err(ParseError::InvalidParameter {
                name: "version".to_string(),
                value: version.to_string(),
                reason: "Version must be greater than 0".to_string(),
            });
        }
        
        // Known versions: 200, 201, 202
        if !self.lenient && version > 202 {
            return Err(ParseError::InvalidParameter {
                name: "version".to_string(),
                value: version.to_string(),
                reason: format!("Unknown version (expected 200-202, got {})", version),
            });
        }
        
        Ok(())
    }
    
    /// Validate preset parameters.
    fn validate_parameters(&self, params: &PresetParameters) -> Result<()> {
        // Validate ranges
        self.validate_range("decay", params.decay, 0.0, 1.0)?;
        self.validate_range("gamma", params.gamma, 0.0, 10.0)?;
        self.validate_range("echo_zoom", params.echo_zoom, 0.0, 10.0)?;
        self.validate_range("echo_alpha", params.echo_alpha, 0.0, 1.0)?;
        
        // Validate wave parameters
        self.validate_range("wave_r", params.wave_r, 0.0, 1.0)?;
        self.validate_range("wave_g", params.wave_g, 0.0, 1.0)?;
        self.validate_range("wave_b", params.wave_b, 0.0, 1.0)?;
        self.validate_range("wave_a", params.wave_a, 0.0, 1.0)?;
        
        // Validate border parameters
        self.validate_range("ob_a", params.ob_a, 0.0, 1.0)?;
        self.validate_range("ib_a", params.ib_a, 0.0, 1.0)?;
        
        // Validate motion vectors
        self.validate_range("mv_a", params.mv_a, 0.0, 1.0)?;
        
        // Validate zoom (should be positive)
        if params.zoom <= 0.0 && !self.lenient {
            return Err(ParseError::InvalidParameter {
                name: "zoom".to_string(),
                value: params.zoom.to_string(),
                reason: "Zoom must be positive".to_string(),
            });
        }
        
        // Validate wave mode
        if params.wave_mode > 7 && !self.lenient {
            return Err(ParseError::InvalidParameter {
                name: "wave_mode".to_string(),
                value: params.wave_mode.to_string(),
                reason: "Wave mode must be 0-7".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Validate a parameter is within range.
    fn validate_range(&self, name: &str, value: f32, min: f32, max: f32) -> Result<()> {
        if self.lenient {
            return Ok(());
        }
        
        if !value.is_finite() {
            return Err(ParseError::InvalidParameter {
                name: name.to_string(),
                value: value.to_string(),
                reason: "Value must be finite".to_string(),
            });
        }
        
        if value < min || value > max {
            return Err(ParseError::InvalidParameter {
                name: name.to_string(),
                value: value.to_string(),
                reason: format!("Value must be between {} and {}", min, max),
            });
        }
        
        Ok(())
    }
    
    /// Validate equations.
    fn validate_equations(&self, equations: &[String], eq_type: &str) -> Result<()> {
        for (i, eq) in equations.iter().enumerate() {
            if eq.trim().is_empty() {
                return Err(ParseError::InvalidEquation {
                    line: i + 1,
                    equation: eq.clone(),
                    reason: "Equation cannot be empty".to_string(),
                });
            }
            
            // Check for basic syntax (must contain '=')
            if !eq.contains('=') && !self.lenient {
                return Err(ParseError::InvalidEquation {
                    line: i + 1,
                    equation: eq.clone(),
                    reason: "Equation must contain '='".to_string(),
                });
            }
            
            // Check for suspicious patterns
            if eq.contains("//") || eq.contains("/*") {
                // Might be a comment, warn but don't fail
                if !self.lenient {
                    log::warn!("{} equation {} contains comment syntax", eq_type, i + 1);
                }
            }
        }
        
        Ok(())
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_version() {
        let validator = Validator::new();
        
        assert!(validator.validate_version(201).is_ok());
        assert!(validator.validate_version(0).is_err());
    }

    #[test]
    fn test_validate_range() {
        let validator = Validator::new();
        
        assert!(validator.validate_range("test", 0.5, 0.0, 1.0).is_ok());
        assert!(validator.validate_range("test", -0.1, 0.0, 1.0).is_err());
        assert!(validator.validate_range("test", 1.1, 0.0, 1.0).is_err());
        assert!(validator.validate_range("test", f32::NAN, 0.0, 1.0).is_err());
    }

    #[test]
    fn test_validate_parameters() {
        let validator = Validator::new();
        let mut params = PresetParameters::default();
        
        // Valid parameters
        params.decay = 0.98;
        params.wave_r = 0.5;
        assert!(validator.validate_parameters(&params).is_ok());
        
        // Invalid decay
        params.decay = 1.5;
        assert!(validator.validate_parameters(&params).is_err());
    }

    #[test]
    fn test_lenient_mode() {
        let validator = Validator::lenient();
        let mut params = PresetParameters::default();
        
        // Out of range values should be accepted in lenient mode
        params.decay = 1.5;
        params.wave_r = 2.0;
        assert!(validator.validate_parameters(&params).is_ok());
    }

    #[test]
    fn test_validate_equations() {
        let validator = Validator::new();
        
        let valid_eqs = vec!["x = x + 1".to_string(), "y = sin(time)".to_string()];
        assert!(validator.validate_equations(&valid_eqs, "per_frame").is_ok());
        
        let empty_eq = vec!["".to_string()];
        assert!(validator.validate_equations(&empty_eq, "per_frame").is_err());
        
        let no_equals = vec!["x + 1".to_string()];
        assert!(validator.validate_equations(&no_equals, "per_frame").is_err());
    }
}
