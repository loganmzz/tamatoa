use eyre::{eyre, Result};

fn main() -> Result<()> {
    let path = std::env::args().nth(1).ok_or_else(|| eyre!("Missing argument"))?;
    let model = tamatoa::load_model(path)?;
    let output = serde_json::to_string_pretty(&model)?;
    println!("{}", output);

    Ok(())
}
