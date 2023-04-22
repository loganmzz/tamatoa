use eyre::Result;

fn main() -> Result<()> {
    let plan = tamatoa::terraform::load_plan("examples/00-empty/init.tfplan.json")?;
    println!("{:?}", plan);
    Ok(())
}
