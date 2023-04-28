use eyre::{eyre, Result};

fn main() -> Result<()> {
    let path = std::env::args().nth(1).ok_or_else(|| eyre!("Missing argument"))?;

    println!("Load Terraform plan from {}", path);
    let model = tamatoa::load_model(&path)?;

    println!("Generate D2 output");
    let output = tamatoa::output::d2::write_to_string(&model)?;

    let dest = format!("{}.d2", path);
    println!("Write content to {}", dest);
    std::fs::write(dest, output)?;

    Ok(())
}
