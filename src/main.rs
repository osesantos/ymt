use clap::Parser;
use std::fs;

mod formatter;

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    file: String,
}

fn main() {
    let args = Cli::parse();

    match fs::read_to_string(&args.file) {
        Ok(contents) => {
            match formatter::validate_yaml(&contents) {
                Ok(_) => formatter::print_success("✅YAML is valid."),
                Err(e) => {
                    formatter::print_error(&format!("❌YAML is invalid: {}", e));
                    return;
                }
            }
            formatter::print_header(&format!("Reading file: {}\n", args.file));
            formatter::print_yaml(&contents);
        }
        Err(e) => formatter::print_error(&format!("Error reading file {}: {}", args.file, e)),
    }
}
