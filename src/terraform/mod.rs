use std::path::Path;

use eyre::{Result};

pub mod tfplan;

pub fn load_plan<P: AsRef<Path>>(path: P) -> Result<tfplan::TfPlan> {
    let json = crate::fs::read_to_string(&path)?;
    let plan = serde_json::from_str(&json)?;
    Ok(plan)
}
