use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct TfPlan {
    format_version: String,
    terraform_version: String,
    planned_values: Values,
    configuration: Values,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Values {
    root_module: RootModule,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct RootModule {
}
