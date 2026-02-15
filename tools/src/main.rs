//! Test preset compatibility with OneDrop parser and evaluator.
//!
//! This tool loads all .milk presets and tests if they can be:
//! 1. Parsed successfully
//! 2. Evaluated without errors
//!
//! Usage: cargo run --bin test_preset_compatibility

use onedrop_eval::MilkEvaluator;
use onedrop_parser::parse_preset;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
struct CompatibilityReport {
    total_presets: usize,
    parse_success: usize,
    parse_failures: Vec<(String, String)>,
    eval_success: usize,
    eval_failures: Vec<(String, String)>,
}

impl CompatibilityReport {
    fn parse_success_rate(&self) -> f64 {
        if self.total_presets == 0 {
            0.0
        } else {
            (self.parse_success as f64 / self.total_presets as f64) * 100.0
        }
    }

    fn eval_success_rate(&self) -> f64 {
        if self.parse_success == 0 {
            0.0
        } else {
            (self.eval_success as f64 / self.parse_success as f64) * 100.0
        }
    }

    fn overall_success_rate(&self) -> f64 {
        if self.total_presets == 0 {
            0.0
        } else {
            (self.eval_success as f64 / self.total_presets as f64) * 100.0
        }
    }
}

fn test_preset(path: &Path, report: &mut CompatibilityReport) {
    let filename = path.file_name().unwrap().to_string_lossy().to_string();

    // Try to read the file
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            report
                .parse_failures
                .push((filename, format!("Read error: {}", e)));
            return;
        }
    };

    // Try to parse
    let preset = match parse_preset(&content) {
        Ok(p) => {
            report.parse_success += 1;
            p
        }
        Err(e) => {
            report
                .parse_failures
                .push((filename, format!("Parse error: {:?}", e)));
            return;
        }
    };

    // Try to evaluate per-frame equations
    let mut evaluator = MilkEvaluator::new();

    // Test per-frame equations
    let mut eval_errors = Vec::new();

    for (i, eq) in preset.per_frame_equations.iter().enumerate() {
        if let Err(e) = evaluator.eval(eq) {
            eval_errors.push(format!("Frame eq {}: {:?}", i, e));
        }
    }

    // Test per-pixel equations
    for (i, eq) in preset.per_pixel_equations.iter().enumerate() {
        if let Err(e) = evaluator.eval(eq) {
            eval_errors.push(format!("Pixel eq {}: {:?}", i, e));
        }
    }

    if eval_errors.is_empty() {
        report.eval_success += 1;
    } else {
        report
            .eval_failures
            .push((filename, eval_errors.join("; ")));
    }
}

fn main() {
    println!("OneDrop Preset Compatibility Test");
    println!("==================================\n");

    let preset_dir = "test-presets";

    if !Path::new(preset_dir).exists() {
        eprintln!("Error: {} directory not found", preset_dir);
        std::process::exit(1);
    }

    let mut report = CompatibilityReport::default();

    // Find all .milk files
    let milk_files: Vec<_> = fs::read_dir(preset_dir)
        .expect("Failed to read preset directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "milk"))
        .collect();

    report.total_presets = milk_files.len();

    println!("Found {} presets to test\n", report.total_presets);
    println!("Testing presets...");

    for (i, path) in milk_files.iter().enumerate() {
        if (i + 1) % 10 == 0 {
            print!("\rProgress: {}/{}", i + 1, report.total_presets);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }
        test_preset(path, &mut report);
    }

    println!("\n\n=== COMPATIBILITY REPORT ===\n");
    println!("Total presets tested: {}", report.total_presets);
    println!();
    println!("Parse Results:");
    println!(
        "  Success: {} ({:.1}%)",
        report.parse_success,
        report.parse_success_rate()
    );
    println!("  Failures: {}", report.parse_failures.len());
    println!();
    println!("Evaluation Results:");
    println!(
        "  Success: {} ({:.1}% of parsed)",
        report.eval_success,
        report.eval_success_rate()
    );
    println!("  Failures: {}", report.eval_failures.len());
    println!();
    println!(
        "Overall Compatibility: {:.1}%",
        report.overall_success_rate()
    );

    if !report.parse_failures.is_empty() {
        println!("\n=== PARSE FAILURES ===");
        for (i, (name, error)) in report.parse_failures.iter().enumerate().take(10) {
            println!("{}. {}", i + 1, name);
            println!("   {}", error);
        }
        if report.parse_failures.len() > 10 {
            println!("... and {} more", report.parse_failures.len() - 10);
        }
    }

    if !report.eval_failures.is_empty() {
        println!("\n=== EVALUATION FAILURES ===");
        for (i, (name, error)) in report.eval_failures.iter().enumerate().take(10) {
            println!("{}. {}", i + 1, name);
            println!("   {}", error);
        }
        if report.eval_failures.len() > 10 {
            println!("... and {} more", report.eval_failures.len() - 10);
        }
    }

    // Save detailed report to file
    let report_path = "preset_compatibility_report.txt";
    let mut report_content = String::new();
    report_content.push_str("OneDrop Preset Compatibility Report\n");
    report_content.push_str("====================================\n\n");
    report_content.push_str(&format!("Total presets: {}\n", report.total_presets));
    report_content.push_str(&format!(
        "Parse success: {} ({:.1}%)\n",
        report.parse_success,
        report.parse_success_rate()
    ));
    report_content.push_str(&format!(
        "Eval success: {} ({:.1}%)\n",
        report.eval_success,
        report.eval_success_rate()
    ));
    report_content.push_str(&format!(
        "Overall: {:.1}%\n\n",
        report.overall_success_rate()
    ));

    if !report.parse_failures.is_empty() {
        report_content.push_str("Parse Failures:\n");
        for (name, error) in &report.parse_failures {
            report_content.push_str(&format!("- {}: {}\n", name, error));
        }
        report_content.push('\n');
    }

    if !report.eval_failures.is_empty() {
        report_content.push_str("Evaluation Failures:\n");
        for (name, error) in &report.eval_failures {
            report_content.push_str(&format!("- {}: {}\n", name, error));
        }
    }

    fs::write(report_path, report_content).expect("Failed to write report");
    println!("\nDetailed report saved to: {}", report_path);
}
