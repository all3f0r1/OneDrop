//! Safe preset loading with error recovery.

use crate::engine::MilkEngine;
use crate::error::{EngineError, Result};
use std::path::Path;

/// Safe preset loader with automatic fallback.
pub struct SafePresetLoader;

impl SafePresetLoader {
    /// Try to load a preset, falling back to default on error.
    ///
    /// This method:
    /// 1. Tries to load the requested preset
    /// 2. On error, logs the issue and loads default preset
    /// 3. Never fails (always returns Ok)
    pub fn load_with_fallback<P: AsRef<Path>>(engine: &mut MilkEngine, path: P) -> Result<()> {
        let path_ref = path.as_ref();

        match engine.load_preset(path_ref) {
            Ok(()) => {
                log::info!("Successfully loaded preset: {}", path_ref.display());
                Ok(())
            }
            Err(e) => {
                log::error!(
                    "Failed to load preset {}: {}. Loading default preset.",
                    path_ref.display(),
                    e
                );

                // Try to load default preset
                match engine.load_default_preset() {
                    Ok(()) => {
                        log::info!("Successfully loaded default preset as fallback");
                        Ok(())
                    }
                    Err(fallback_err) => {
                        // This should never happen since default preset is hardcoded
                        log::error!("CRITICAL: Failed to load default preset: {}", fallback_err);
                        Err(EngineError::PresetLoadFailed(format!(
                            "Both preset and fallback failed: {} / {}",
                            e, fallback_err
                        )))
                    }
                }
            }
        }
    }

    /// Try to load a preset, retrying on transient errors.
    ///
    /// # Arguments
    /// * `engine` - The MilkEngine instance
    /// * `path` - Path to the preset file
    /// * `max_retries` - Maximum number of retry attempts
    pub fn load_with_retry<P: AsRef<Path>>(
        engine: &mut MilkEngine,
        path: P,
        max_retries: usize,
    ) -> Result<()> {
        let path_ref = path.as_ref();
        let mut last_error = None;

        for attempt in 0..=max_retries {
            match engine.load_preset(path_ref) {
                Ok(()) => {
                    if attempt > 0 {
                        log::info!(
                            "Successfully loaded preset {} after {} retries",
                            path_ref.display(),
                            attempt
                        );
                    }
                    return Ok(());
                }
                Err(e) => {
                    log::warn!(
                        "Attempt {}/{} failed to load preset {}: {}",
                        attempt + 1,
                        max_retries + 1,
                        path_ref.display(),
                        e
                    );
                    last_error = Some(e);

                    // Wait a bit before retrying (exponential backoff)
                    if attempt < max_retries {
                        let wait_ms = 100 * (1 << attempt); // 100ms, 200ms, 400ms, ...
                        std::thread::sleep(std::time::Duration::from_millis(wait_ms));
                    }
                }
            }
        }

        // All retries failed, fall back to default
        log::error!(
            "All {} attempts failed for preset {}. Loading default preset.",
            max_retries + 1,
            path_ref.display()
        );

        engine.load_default_preset().map_err(|fallback_err| {
            EngineError::PresetLoadFailed(format!(
                "Preset loading failed after {} retries, and fallback also failed: {} / {}",
                max_retries,
                last_error
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "Unknown".to_string()),
                fallback_err
            ))
        })
    }

    /// Validate a preset file without loading it.
    ///
    /// Returns Ok(()) if the preset is valid, Err otherwise.
    pub fn validate_preset<P: AsRef<Path>>(path: P) -> Result<()> {
        let path_ref = path.as_ref();

        // Check file exists
        if !path_ref.exists() {
            return Err(EngineError::PresetLoadFailed(format!(
                "File does not exist: {}",
                path_ref.display()
            )));
        }

        // Check file is readable
        let content = std::fs::read_to_string(path_ref)?;

        // Try to parse
        onedrop_parser::parse_preset(&content)?;

        log::debug!("Preset {} is valid", path_ref.display());
        Ok(())
    }

    /// Scan a directory for valid presets.
    ///
    /// Returns a list of paths to valid preset files.
    pub fn scan_directory<P: AsRef<Path>>(dir: P) -> Vec<std::path::PathBuf> {
        let dir_ref = dir.as_ref();
        let mut valid_presets = Vec::new();

        if let Ok(entries) = std::fs::read_dir(dir_ref) {
            for entry in entries.flatten() {
                let path = entry.path();

                // Check if it's a .milk file
                if path.extension().and_then(|s| s.to_str()) == Some("milk") {
                    // Validate the preset
                    if Self::validate_preset(&path).is_ok() {
                        valid_presets.push(path);
                    } else {
                        log::warn!("Skipping invalid preset: {}", path.display());
                    }
                }
            }
        }

        log::info!(
            "Found {} valid presets in {}",
            valid_presets.len(),
            dir_ref.display()
        );
        valid_presets
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::EngineConfig;

    #[test]
    fn test_load_default_preset() {
        let config = EngineConfig::default();
        let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();

        // Should be able to load default preset
        let result = engine.load_default_preset();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_preset() {
        // Invalid path should fail
        let result = SafePresetLoader::validate_preset("nonexistent.milk");
        assert!(result.is_err());
    }
}
