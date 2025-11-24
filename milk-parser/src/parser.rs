//! Parser implementation for .milk files using nom.

use crate::error::{ParseError, Result};
use crate::preset::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, line_ending, multispace0, space0},
    combinator::{map, map_res, opt, recognize},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::collections::HashMap;

/// Parse a complete .milk preset file.
pub fn parse_milk_preset(input: &str) -> Result<MilkPreset> {
    let mut preset = MilkPreset::default();
    let mut lines = input.lines().enumerate();
    
    // Parse header
    for (line_num, line) in lines.by_ref() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if line.starts_with("MILKDROP_PRESET_VERSION=") {
            preset.version = parse_version_line(line)?;
        } else if line.starts_with("PSVERSION_WARP=") {
            preset.ps_version_warp = parse_psversion_line(line)?;
        } else if line.starts_with("PSVERSION_COMP=") {
            preset.ps_version_comp = parse_psversion_line(line)?;
        } else if line.starts_with("[preset") {
            // Found preset section, break to parse body
            break;
        }
    }
    
    // Parse preset body
    for (line_num, line) in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        // Parse per-frame equations
        if line.starts_with("per_frame_") {
            if let Some(equation) = parse_equation_line(line) {
                preset.per_frame_equations.push(equation);
            }
        }
        // Parse per-pixel equations
        else if line.starts_with("per_pixel_") {
            if let Some(equation) = parse_equation_line(line) {
                preset.per_pixel_equations.push(equation);
            }
        }
        // Parse warp shader
        else if line.starts_with("warp_") {
            let shader_line = parse_shader_line(line);
            if let Some(ref mut shader) = preset.warp_shader {
                shader.push_str(&shader_line);
                shader.push('\n');
            } else {
                preset.warp_shader = Some(shader_line + "\n");
            }
        }
        // Parse comp shader
        else if line.starts_with("comp_") {
            let shader_line = parse_shader_line(line);
            if let Some(ref mut shader) = preset.comp_shader {
                shader.push_str(&shader_line);
                shader.push('\n');
            } else {
                preset.comp_shader = Some(shader_line + "\n");
            }
        }
        // Parse wavecode
        else if line.starts_with("wavecode_") {
            parse_wavecode_line(line, &mut preset.waves)?;
        }
        // Parse shapecode
        else if line.starts_with("shapecode_") {
            parse_shapecode_line(line, &mut preset.shapes)?;
        }
        // Parse regular parameters
        else if let Some((key, value)) = line.split_once('=') {
            parse_parameter(key.trim(), value.trim(), &mut preset.parameters)?;
        }
    }
    
    Ok(preset)
}

/// Parse version line (e.g., "MILKDROP_PRESET_VERSION=201")
fn parse_version_line(line: &str) -> Result<u32> {
    line.split('=')
        .nth(1)
        .and_then(|v| v.trim().parse().ok())
        .ok_or_else(|| ParseError::InvalidVersion(line.to_string()))
}

/// Parse PS version line
fn parse_psversion_line(line: &str) -> Result<u32> {
    line.split('=')
        .nth(1)
        .and_then(|v| v.trim().parse().ok())
        .ok_or_else(|| ParseError::ParseFailed(format!("Invalid PSVERSION: {}", line)))
}

/// Parse equation line (e.g., "per_frame_1=wave_r = 0.5;")
fn parse_equation_line(line: &str) -> Option<String> {
    line.split_once('=')
        .map(|(_, equation)| equation.trim().to_string())
}

/// Parse shader line (e.g., "warp_1=`shader_body")
fn parse_shader_line(line: &str) -> String {
    line.split_once('=')
        .map(|(_, code)| {
            // Remove backtick prefix if present
            code.trim().trim_start_matches('`').to_string()
        })
        .unwrap_or_default()
}

/// Parse a parameter and store it in PresetParameters
fn parse_parameter(key: &str, value: &str, params: &mut PresetParameters) -> Result<()> {
    // Helper to parse float
    let parse_f32 = |v: &str| -> Result<f32> {
        v.parse().map_err(|_| ParseError::InvalidParameter {
            name: key.to_string(),
            value: v.to_string(),
            reason: "Expected float".to_string(),
        })
    };
    
    // Helper to parse int
    let parse_i32 = |v: &str| -> Result<i32> {
        v.parse().map_err(|_| ParseError::InvalidParameter {
            name: key.to_string(),
            value: v.to_string(),
            reason: "Expected integer".to_string(),
        })
    };
    
    // Helper to parse bool
    let parse_bool = |v: &str| -> Result<bool> {
        match v {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(ParseError::InvalidParameter {
                name: key.to_string(),
                value: v.to_string(),
                reason: "Expected 0 or 1".to_string(),
            }),
        }
    };
    
    match key {
        // Float parameters
        "fRating" => params.f_rating = parse_f32(value)?,
        "fGammaAdj" => params.f_gamma_adj = parse_f32(value)?,
        "fDecay" => params.f_decay = parse_f32(value)?,
        "fVideoEchoZoom" => params.f_video_echo_zoom = parse_f32(value)?,
        "fVideoEchoAlpha" => params.f_video_echo_alpha = parse_f32(value)?,
        "fWaveAlpha" => params.f_wave_alpha = parse_f32(value)?,
        "fWaveScale" => params.f_wave_scale = parse_f32(value)?,
        "fWaveSmoothing" => params.f_wave_smoothing = parse_f32(value)?,
        "fWaveParam" => params.f_wave_param = parse_f32(value)?,
        "fModWaveAlphaStart" => params.f_mod_wave_alpha_start = parse_f32(value)?,
        "fModWaveAlphaEnd" => params.f_mod_wave_alpha_end = parse_f32(value)?,
        "fWarpAnimSpeed" => params.f_warp_anim_speed = parse_f32(value)?,
        "fWarpScale" => params.f_warp_scale = parse_f32(value)?,
        "fZoomExponent" => params.f_zoom_exponent = parse_f32(value)?,
        "fShader" => params.f_shader = parse_f32(value)?,
        
        // Motion parameters
        "zoom" => params.zoom = parse_f32(value)?,
        "rot" => params.rot = parse_f32(value)?,
        "cx" => params.cx = parse_f32(value)?,
        "cy" => params.cy = parse_f32(value)?,
        "dx" => params.dx = parse_f32(value)?,
        "dy" => params.dy = parse_f32(value)?,
        "warp" => params.warp = parse_f32(value)?,
        "sx" => params.sx = parse_f32(value)?,
        "sy" => params.sy = parse_f32(value)?,
        
        // Wave colors
        "wave_r" => params.wave_r = parse_f32(value)?,
        "wave_g" => params.wave_g = parse_f32(value)?,
        "wave_b" => params.wave_b = parse_f32(value)?,
        "wave_x" => params.wave_x = parse_f32(value)?,
        "wave_y" => params.wave_y = parse_f32(value)?,
        
        // Borders
        "ob_size" => params.ob_size = parse_f32(value)?,
        "ob_r" => params.ob_r = parse_f32(value)?,
        "ob_g" => params.ob_g = parse_f32(value)?,
        "ob_b" => params.ob_b = parse_f32(value)?,
        "ob_a" => params.ob_a = parse_f32(value)?,
        "ib_size" => params.ib_size = parse_f32(value)?,
        "ib_r" => params.ib_r = parse_f32(value)?,
        "ib_g" => params.ib_g = parse_f32(value)?,
        "ib_b" => params.ib_b = parse_f32(value)?,
        "ib_a" => params.ib_a = parse_f32(value)?,
        
        // Motion vectors
        "nMotionVectorsX" => params.n_motion_vectors_x = parse_f32(value)?,
        "nMotionVectorsY" => params.n_motion_vectors_y = parse_f32(value)?,
        "mv_dx" => params.mv_dx = parse_f32(value)?,
        "mv_dy" => params.mv_dy = parse_f32(value)?,
        "mv_l" => params.mv_l = parse_f32(value)?,
        "mv_r" => params.mv_r = parse_f32(value)?,
        "mv_g" => params.mv_g = parse_f32(value)?,
        "mv_b" => params.mv_b = parse_f32(value)?,
        "mv_a" => params.mv_a = parse_f32(value)?,
        
        // Beat detection
        "b1n" => params.b1n = parse_f32(value)?,
        "b2n" => params.b2n = parse_f32(value)?,
        "b3n" => params.b3n = parse_f32(value)?,
        "b1x" => params.b1x = parse_f32(value)?,
        "b2x" => params.b2x = parse_f32(value)?,
        "b3x" => params.b3x = parse_f32(value)?,
        "b1ed" => params.b1ed = parse_f32(value)?,
        
        // Integer parameters
        "nVideoEchoOrientation" => params.n_video_echo_orientation = parse_i32(value)?,
        "nWaveMode" => params.n_wave_mode = parse_i32(value)?,
        
        // Boolean parameters
        "bAdditiveWaves" => params.b_additive_waves = parse_bool(value)?,
        "bWaveDots" => params.b_wave_dots = parse_bool(value)?,
        "bWaveThick" => params.b_wave_thick = parse_bool(value)?,
        "bModWaveAlphaByVolume" => params.b_mod_wave_alpha_by_volume = parse_bool(value)?,
        "bMaximizeWaveColor" => params.b_maximize_wave_color = parse_bool(value)?,
        "bTexWrap" => params.b_tex_wrap = parse_bool(value)?,
        "bDarkenCenter" => params.b_darken_center = parse_bool(value)?,
        "bRedBlueStereo" => params.b_red_blue_stereo = parse_bool(value)?,
        "bBrighten" => params.b_brighten = parse_bool(value)?,
        "bDarken" => params.b_darken = parse_bool(value)?,
        "bSolarize" => params.b_solarize = parse_bool(value)?,
        "bInvert" => params.b_invert = parse_bool(value)?,
        
        // Unknown parameters go to extra map
        _ => {
            params.extra.insert(key.to_string(), value.to_string());
        }
    }
    
    Ok(())
}

/// Parse wavecode line
fn parse_wavecode_line(line: &str, waves: &mut Vec<WaveCode>) -> Result<()> {
    // Extract wave index and parameter name
    // Format: wavecode_N_param=value
    let parts: Vec<&str> = line.split('_').collect();
    if parts.len() < 3 {
        return Ok(()); // Skip malformed lines
    }
    
    let index: usize = parts[1].parse().unwrap_or(0);
    let param_and_value = line.split_once('=');
    
    if let Some((param_full, value)) = param_and_value {
        let param = param_full.split('_').skip(2).collect::<Vec<_>>().join("_");
        
        // Ensure wave exists
        while waves.len() <= index {
            waves.push(WaveCode {
                index: waves.len(),
                enabled: false,
                samples: 512,
                sep: 0,
                b_spectrum: false,
                b_use_dots: false,
                b_draw_thick: false,
                b_additive: false,
                scaling: 1.0,
                smoothing: 0.5,
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
                per_frame_equations: Vec::new(),
                per_point_equations: Vec::new(),
                per_frame_init_equations: Vec::new(),
            });
        }
        
        // Parse parameter
        let wave = &mut waves[index];
        match param.as_str() {
            "enabled" => wave.enabled = value == "1",
            "samples" => wave.samples = value.parse().unwrap_or(512),
            "sep" => wave.sep = value.parse().unwrap_or(0),
            "bSpectrum" => wave.b_spectrum = value == "1",
            "bUseDots" => wave.b_use_dots = value == "1",
            "bDrawThick" => wave.b_draw_thick = value == "1",
            "bAdditive" => wave.b_additive = value == "1",
            "scaling" => wave.scaling = value.parse().unwrap_or(1.0),
            "smoothing" => wave.smoothing = value.parse().unwrap_or(0.5),
            "r" => wave.r = value.parse().unwrap_or(1.0),
            "g" => wave.g = value.parse().unwrap_or(1.0),
            "b" => wave.b = value.parse().unwrap_or(1.0),
            "a" => wave.a = value.parse().unwrap_or(1.0),
            _ => {} // Ignore unknown parameters
        }
    }
    
    Ok(())
}

/// Parse shapecode line
fn parse_shapecode_line(line: &str, shapes: &mut Vec<ShapeCode>) -> Result<()> {
    // Extract shape index and parameter name
    // Format: shapecode_N_param=value
    let parts: Vec<&str> = line.split('_').collect();
    if parts.len() < 3 {
        return Ok(()); // Skip malformed lines
    }
    
    let index: usize = parts[1].parse().unwrap_or(0);
    let param_and_value = line.split_once('=');
    
    if let Some((param_full, value)) = param_and_value {
        let param = param_full.split('_').skip(2).collect::<Vec<_>>().join("_");
        
        // Ensure shape exists
        while shapes.len() <= index {
            shapes.push(ShapeCode {
                index: shapes.len(),
                enabled: false,
                sides: 4,
                additive: false,
                thick_outline: false,
                textured: false,
                num_inst: 1,
                x: 0.5,
                y: 0.5,
                rad: 0.1,
                ang: 0.0,
                tex_ang: 0.0,
                tex_zoom: 1.0,
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
                r2: 0.0,
                g2: 0.0,
                b2: 0.0,
                a2: 0.0,
                border_r: 1.0,
                border_g: 1.0,
                border_b: 1.0,
                border_a: 0.0,
                per_frame_equations: Vec::new(),
                per_frame_init_equations: Vec::new(),
            });
        }
        
        // Parse parameter
        let shape = &mut shapes[index];
        match param.as_str() {
            "enabled" => shape.enabled = value == "1",
            "sides" => shape.sides = value.parse().unwrap_or(4),
            "additive" => shape.additive = value == "1",
            "thickOutline" => shape.thick_outline = value == "1",
            "textured" => shape.textured = value == "1",
            "num_inst" | "num inst" => shape.num_inst = value.parse().unwrap_or(1),
            "x" => shape.x = value.parse().unwrap_or(0.5),
            "y" => shape.y = value.parse().unwrap_or(0.5),
            "rad" => shape.rad = value.parse().unwrap_or(0.1),
            "ang" => shape.ang = value.parse().unwrap_or(0.0),
            "tex_ang" | "tex ang" => shape.tex_ang = value.parse().unwrap_or(0.0),
            "tex_zoom" | "tex zoom" => shape.tex_zoom = value.parse().unwrap_or(1.0),
            "r" => shape.r = value.parse().unwrap_or(1.0),
            "g" => shape.g = value.parse().unwrap_or(1.0),
            "b" => shape.b = value.parse().unwrap_or(1.0),
            "a" => shape.a = value.parse().unwrap_or(1.0),
            "r2" => shape.r2 = value.parse().unwrap_or(0.0),
            "g2" => shape.g2 = value.parse().unwrap_or(0.0),
            "b2" => shape.b2 = value.parse().unwrap_or(0.0),
            "a2" => shape.a2 = value.parse().unwrap_or(0.0),
            "border_r" | "border r" => shape.border_r = value.parse().unwrap_or(1.0),
            "border_g" | "border g" => shape.border_g = value.parse().unwrap_or(1.0),
            "border_b" | "border b" => shape.border_b = value.parse().unwrap_or(1.0),
            "border_a" | "border a" => shape.border_a = value.parse().unwrap_or(0.0),
            _ => {} // Ignore unknown parameters
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let line = "MILKDROP_PRESET_VERSION=201";
        assert_eq!(parse_version_line(line).unwrap(), 201);
    }

    #[test]
    fn test_parse_equation() {
        let line = "per_frame_1=wave_r = 0.5;";
        assert_eq!(parse_equation_line(line), Some("wave_r = 0.5;".to_string()));
    }

    #[test]
    fn test_parse_shader() {
        let line = "warp_1=`shader_body";
        assert_eq!(parse_shader_line(line), "shader_body");
    }
}
