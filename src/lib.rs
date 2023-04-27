pub mod analyzer;
pub mod fs;
pub mod model;
pub mod terraform;

use std::path::Path;
use eyre::{Result};

pub fn load_model<P: AsRef<Path>>(path: P) -> Result<model::Module> {
    let plan = crate::terraform::load_plan(path)?;

    let model = crate::analyzer::analyze(&plan);
    Ok(model)
}

#[cfg(test)]
pub mod examples {
    use eyre::Result;
    use crate::load_model;

    fn check(case: &str) -> Result<()> {
        let path = format!("examples/{}/terraform.tfplan.json", case);
        load_model(path)?;
        Ok(())
    }

    #[test]
    fn example_00_empty() -> Result<()> {
        check("00-empty")
    }

    #[test]
    fn example_01_create() -> Result<()> {
        check("01-create")
    }

    #[test]
    fn example_02_update() -> Result<()> {
        check("02-update")
    }

    #[test]
    fn example_03_replace() -> Result<()> {
        check("03-replace")
    }
}
