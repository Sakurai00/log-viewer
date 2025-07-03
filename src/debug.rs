use colored::Colorize;
use regex::Regex;

pub fn print_debug_info(
    log_files: &Option<Vec<String>>,
    include_regex: &Option<Regex>,
    exclude_regex: &Option<Regex>,
    default_log_files: &[&str],
) {
    println!();
    println!("{}", "=".repeat(40).cyan());
    println!("{}", "  DEBUG INFO".bold().cyan());
    println!("{}", "=".repeat(40).cyan());

    // Log files
    match log_files {
        Some(files) => println!("{}: {}", "Log files".bold(), files.join(", ")),
        None => println!(
            "{}: {}",
            "Log files".bold(),
            default_log_files.join(", ")
        ),
    }

    // Include regex
    match include_regex {
        Some(regex) => println!("{}: {}", "Include Regex".bold(), regex.to_string()),
        None => println!("{}: None", "Include Regex".bold()),
    }

    // Exclude regex
    match exclude_regex {
        Some(regex) => println!("{}: {}", "Exclude Regex".bold(), regex.to_string()),
        None => println!("{}: None", "Exclude Regex".bold()),
    }

    println!("{}", "=".repeat(40).cyan());
    println!();
}
