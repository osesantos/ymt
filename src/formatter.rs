use colored::Colorize;

pub fn print_error(message: &str) {
    eprintln!("{}", message.red().bold());
}

// Function to print the yaml file
// The key will be in blue color
// The value will be in green color
pub fn print_yaml(message: &str) {
    for line in message.lines() {
        if let Some((key, value)) = line.split_once(':') {
            println!("{}:{}", key.blue().bold(), value.green());
        } else {
            println!("{}", line);
        }
    }
}

// Function to print a header message in yellow color
pub fn print_header(message: &str) {
    println!("{}", message.yellow().bold());
}

// Validates if the given yaml is valid
// Returns true if valid, false otherwise
pub fn validate_yaml(yaml: &str) -> bool {
    false
}
