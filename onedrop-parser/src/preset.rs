//! Data structures representing a Milkdrop preset.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete Milkdrop preset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilkPreset {
    /// Preset version (e.g., 201 for Milkdrop 2.0)
    pub version: u32,
    
    /// Pixel shader version for warp shader
    pub ps_version_warp: u32,
    
    /// Pixel shader version for composite shader
    pub ps_version_comp: u32,
    
    /// Base parameters (static values)
    pub parameters: PresetParameters,
    
    /// Per-frame equations (executed once per frame)
    pub per_frame_equations: Vec<String>,
    
    /// Per-pixel equations (executed for each pixel)
    pub per_pixel_equations: Vec<String>,
    
    /// Initialization equations (executed once when preset loads)
    pub per_frame_init_equations: Vec<String>,
    
    /// Custom waveforms (up to 4)
    pub waves: Vec<WaveCode>,
    
    /// Custom shapes (up to 4)
    pub shapes: Vec<ShapeCode>,
    
    /// Warp shader code (HLSL/GLSL)
    pub warp_shader: Option<String>,
    
    /// Composite shader code (HLSL/GLSL)
    pub comp_shader: Option<String>,
}

/// Base parameters for a preset (static values).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PresetParameters {
    // Rating and visual adjustments
    pub f_rating: f32,
    pub f_gamma_adj: f32,
    pub f_decay: f32,
    pub f_video_echo_zoom: f32,
    pub f_video_echo_alpha: f32,
    pub n_video_echo_orientation: i32,
    
    // Wave settings
    pub n_wave_mode: i32,
    pub b_additive_waves: bool,
    pub b_wave_dots: bool,
    pub b_wave_thick: bool,
    pub b_mod_wave_alpha_by_volume: bool,
    pub b_maximize_wave_color: bool,
    pub f_wave_alpha: f32,
    pub f_wave_scale: f32,
    pub f_wave_smoothing: f32,
    pub f_wave_param: f32,
    pub f_mod_wave_alpha_start: f32,
    pub f_mod_wave_alpha_end: f32,
    
    // Rendering options
    pub b_tex_wrap: bool,
    pub b_darken_center: bool,
    pub b_red_blue_stereo: bool,
    pub b_brighten: bool,
    pub b_darken: bool,
    pub b_solarize: bool,
    pub b_invert: bool,
    
    // Warp settings
    pub f_warp_anim_speed: f32,
    pub f_warp_scale: f32,
    pub f_zoom_exponent: f32,
    pub f_shader: f32,
    
    // Motion parameters (can be modified by per-frame equations)
    pub zoom: f32,
    pub rot: f32,
    pub cx: f32,
    pub cy: f32,
    pub dx: f32,
    pub dy: f32,
    pub warp: f32,
    pub sx: f32,
    pub sy: f32,
    
    // Wave color
    pub wave_r: f32,
    pub wave_g: f32,
    pub wave_b: f32,
    pub wave_x: f32,
    pub wave_y: f32,
    
    // Outer border
    pub ob_size: f32,
    pub ob_r: f32,
    pub ob_g: f32,
    pub ob_b: f32,
    pub ob_a: f32,
    
    // Inner border
    pub ib_size: f32,
    pub ib_r: f32,
    pub ib_g: f32,
    pub ib_b: f32,
    pub ib_a: f32,
    
    // Motion vectors
    pub n_motion_vectors_x: f32,
    pub n_motion_vectors_y: f32,
    pub mv_dx: f32,
    pub mv_dy: f32,
    pub mv_l: f32,
    pub mv_r: f32,
    pub mv_g: f32,
    pub mv_b: f32,
    pub mv_a: f32,
    
    // Beat detection parameters
    pub b1n: f32,
    pub b2n: f32,
    pub b3n: f32,
    pub b1x: f32,
    pub b2x: f32,
    pub b3x: f32,
    pub b1ed: f32,
    
    // Additional parameters stored as key-value pairs
    pub extra: HashMap<String, String>,
}

/// Custom waveform definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaveCode {
    pub index: usize,
    pub enabled: bool,
    pub samples: i32,
    pub sep: i32,
    pub b_spectrum: bool,
    pub b_use_dots: bool,
    pub b_draw_thick: bool,
    pub b_additive: bool,
    pub scaling: f32,
    pub smoothing: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    
    /// Per-frame equations for this wave
    pub per_frame_equations: Vec<String>,
    
    /// Per-point equations for this wave
    pub per_point_equations: Vec<String>,
    
    /// Initialization equations for this wave
    pub per_frame_init_equations: Vec<String>,
}

/// Custom shape definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeCode {
    pub index: usize,
    pub enabled: bool,
    pub sides: i32,
    pub additive: bool,
    pub thick_outline: bool,
    pub textured: bool,
    pub num_inst: i32,
    pub x: f32,
    pub y: f32,
    pub rad: f32,
    pub ang: f32,
    pub tex_ang: f32,
    pub tex_zoom: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub r2: f32,
    pub g2: f32,
    pub b2: f32,
    pub a2: f32,
    pub border_r: f32,
    pub border_g: f32,
    pub border_b: f32,
    pub border_a: f32,
    
    /// Per-frame equations for this shape
    pub per_frame_equations: Vec<String>,
    
    /// Initialization equations for this shape
    pub per_frame_init_equations: Vec<String>,
}

impl Default for MilkPreset {
    fn default() -> Self {
        Self {
            version: 201,
            ps_version_warp: 2,
            ps_version_comp: 2,
            parameters: PresetParameters::default(),
            per_frame_equations: Vec::new(),
            per_pixel_equations: Vec::new(),
            per_frame_init_equations: Vec::new(),
            waves: Vec::new(),
            shapes: Vec::new(),
            warp_shader: None,
            comp_shader: None,
        }
    }
}
