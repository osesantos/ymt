# YMT - YAML Formatter & Validator

`ymt` is a simple Rust tool to validate and format YAML files.  
It works with both files and **stdin**, making it easy to use in Linux pipes and workflows.

## ✨ Features
- ✅ Validates any generic YAML (no fixed schema)
- ✅ Formats YAML into a consistent style
- ✅ Supports files or stdin (pipes/redirection)
- ✅ Lightweight, fast, and written in Rust ⚡

## 📦 Installation
Clone and build:
```bash
git clone https://github.com/youruser/ymt.git
cd ymt
cargo build --release
```

The binary will be in `target/release/ymt`.

## 🚀 Usage

### Validate a file
+++bash
ymt config.yaml
+++

### Validate via pipe
+++bash
cat config.yaml | ymt
+++

### Format a file
+++bash
ymt format config.yaml
+++

### Format via pipe
+++bash
cat config.yaml | ymt
+++

> ⚠️ By default, the formatter prints to **stdout**.  

## 🛠️ Stack
- [Rust](https://www.rust-lang.org/)  
- [serde_yaml](https://crates.io/crates/serde_yaml)  
- [clap](https://crates.io/crates/clap)  

---

Made with ❤️ in Rust.
