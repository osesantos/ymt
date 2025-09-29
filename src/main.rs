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
            formatter::print_header(&format!("Reading file: {}\n", args.file));
            formatter::print_yaml(&contents);
        }
        Err(e) => formatter::print_error(&format!("Error reading file {}: {}", args.file, e)),
    }
}
