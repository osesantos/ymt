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

pub fn print_success(message: &str) {
    println!("{}", message.green().bold());
}

// Validates if the given yaml is valid
// Returns true if valid, false otherwise
pub fn validate_yaml(yaml: &str) -> Result<serde_yaml::Value, serde_yaml::Error> {
    let formatted: serde_yaml::Value = serde_yaml::from_str(yaml)?;
    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_yaml_valid() {
        let yaml = r#"
        key1: value1
        key2:
          - item1
          - item2
        "#;
        assert!(validate_yaml(yaml).is_ok());
    }

    #[test]
    fn test_validate_yaml_invalid() {
        let yaml = r#"
        key1: value1
        key2
          - item1
          - item2
        "#;
        assert!(validate_yaml(yaml).is_err());
    }
}
