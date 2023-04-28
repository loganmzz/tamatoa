use eyre::Result;
use std::fmt::Write;

pub fn write_to_string(model: &crate::model::Module) -> Result<String> {
    let mut output = String::new();
    write!(output, "{} {{\n", model.id())?;
    write!(output, "  shape: package\n")?;
    write!(output, "\n")?;
    for resource in model.resources() {
        write!(output, "  {}\n", resource.id())?;
    }
    write!(output, "}}\n")?;
    Ok(output)
}

#[cfg(test)]
mod test {
    use eyre::Result;
    use serde_json::json;

    use crate::model::Module;
    use crate::output::d2::write_to_string;

    fn assert_string(json: serde_json::Value, expected: &str) -> Result<()> {
        let root: Module = serde_json::from_value(json)?;
        assert_eq!(expected, write_to_string(&root)?);
        Ok(())
    }

    #[test]
    fn string_empty() -> Result<()> {
        assert_string(
            json!({
                "id": "root",
            }),
            r#"root {
  shape: package

}
"#,
        )
    }

    #[test]
    fn string_single_resource() -> Result<()> {
        assert_string(
            json!({
                "id": "root",
                "resources": [
                    {
                        "id": "foo",
                    },
                ],
            }),
            r#"root {
  shape: package

  foo
}
"#,
        )
    }

    #[test]
    fn string_two_resources() -> Result<()> {
        assert_string(
            json!({
                "id": "root",
                "resources": [
                    {
                        "id": "foo",
                    },
                    {
                        "id": "bar",
                    },
                ],
            }),
            r#"root {
  shape: package

  foo
  bar
}
"#,
        )
    }
}
