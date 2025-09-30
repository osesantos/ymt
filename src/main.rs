use clap::Parser;
use std::io::{self, Read};

mod formatter;

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    file: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let input = if let Some(file) = args.file {
        match std::fs::read_to_string(file) {
            Ok(contents) => contents,
            Err(e) => {
                formatter::print_error(&format!("Error reading file: {}", e));
                return;
            }
        }
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    match formatter::validate_yaml(&input) {
        Ok(formatted) => {
            let stringified = serde_yaml::to_string(&formatted).unwrap_or_default();
            formatter::print_success("✅YAML is valid.\n");
            formatter::print_yaml(&stringified);
            return;
        }
        Err(e) => {
            formatter::print_error(&format!("❌YAML is invalid: {}", e));
            return;
        }
    };
}
