use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize,Serialize)]
pub struct Module {
    id: String,
    #[serde(default="Vec::new")]
    resources: Vec<Resource>,
    #[serde(default="Vec::new")]
    modules: Vec<Module>,
}

#[derive(Deserialize,Serialize)]
pub struct Resource {
    id: String,
    #[serde(default)]
    change: Change,
}

#[derive(Deserialize,Serialize)]
#[serde(default)]
pub struct Change {
    before: Value,
    after: Value,
    action: ChangeAction,
}

#[derive(Deserialize,Serialize)]
pub enum ChangeAction {
    Noop,
    Create,
    Delete,
    Update,
    Replace,
}

impl Module {
    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
            resources: Vec::new(),
            modules: Vec::new(),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn resources(&self) -> &Vec<Resource> {
        &self.resources
    }
    pub fn resources_mut(&mut self) -> &mut Vec<Resource> {
        &mut self.resources
    }
}

impl Default for Module {
    fn default() -> Self {
        Self::new("root")
    }
}

impl Resource {
    pub fn new(id: &str) -> Self {
        Self {
            id: String::from(id),
            change: Change::default(),
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn change(&self) -> &Change {
        &self.change
    }
    pub fn change_mut(&mut self) -> &mut Change {
        &mut self.change
    }
}

impl Change {
    pub fn before(&self) -> &Value {
        &self.before
    }
    pub fn before_mut(&mut self) -> &mut Value {
        &mut self.before
    }

    pub fn after(&self) -> &Value {
        &self.after
    }
    pub fn after_mut(&mut self) -> &mut Value {
        &mut self.after
    }

    pub fn action(&self) -> &ChangeAction {
        &self.action
    }
    pub fn action_mut(&mut self) -> &mut ChangeAction {
        &mut self.action
    }
}

impl Default for Change {
    fn default() -> Self {
        Self {
            before: Value::Null,
            after: Value::Null,
            action: ChangeAction::Noop,
        }
    }
}

impl ChangeAction {
    pub fn from_actions(actions: &[&str]) -> ChangeAction {
        match actions {
            &["delete","create"] => Self::Replace,
            &["create","delete"] => Self::Replace,
            &["create"] => Self::Create,
            &["delete"] => Self::Delete,
            &["update"] => Self::Update,
            _ => Self::Noop,
        }
    }
}
