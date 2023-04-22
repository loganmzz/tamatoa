use std::fs;
use std::path::Path;
use eyre::Result;

pub mod tfplan;

pub fn load_plan<P: AsRef<Path>>(path: P) -> Result<tfplan::TfPlan> {
    let json = fs::read_to_string(path)?;
    // let reader = BufReader::new(file);
    let plan = serde_json::from_str(&json)?;
    Ok(plan)
}
