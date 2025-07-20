use colored::Colorize;
use regex::Regex;

pub fn print_debug_info(log_files: &[String], include_regex: &Option<Regex>, exclude_regex: &Option<Regex>) {
    println!();
    println!("{}", "=".repeat(40).cyan());
    println!("{}", "  DEBUG INFO".bold().cyan());
    println!("{}", "=".repeat(40).cyan());

    // Log files
    println!("{}: {}", "Log files".bold(), log_files.join(", "));

    // Include regex
    match include_regex {
        Some(regex) => println!("{}: {}", "Include Regex".bold(), regex),
        None => println!("{}: None", "Include Regex".bold()),
    }

    // Exclude regex
    match exclude_regex {
        Some(regex) => println!("{}: {}", "Exclude Regex".bold(), regex),
        None => println!("{}: None", "Exclude Regex".bold()),
    }

    println!("{}", "=".repeat(40).cyan());
    println!();
}
