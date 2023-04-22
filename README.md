# Tamatoa

Rust-based tool to generate report for Terraform plan.

# Install

There is currently no distributed release of this project. From source code, use:

```bash
cargo install --path .
```

# Usage

```bash
# Generate Terraform plan
terraform plan -out 'myproject.tfplan' &&
# Output in JSON format
terraform show -json 'myproject.tfplan' > 'myproject.tfplan.json' &&
# Analyse JSON Terraform plan
tamatoa 'myproject.tfplan.json'
```

# Development

## Requirements

* [Rust toolchain](https://rust-lang.org/tools/install)
