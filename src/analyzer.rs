use std::collections::HashMap;
use crate::terraform::tfplan::TfPlan;
use crate::model::{ChangeAction, Module, Resource};

pub fn analyze(plan: &TfPlan) -> Module {
    let mut root = Module::new("root");
    let changes: HashMap<_,_> = plan.resource_changes().iter().map(|r| (r.address(), r)).collect();
    for resource in plan.configuration().root_module().resources() {
        let mut output = Resource::new(resource.address());
        if let Some(change) = changes.get(resource.address()) {
            *output.change_mut().before_mut() = change.change().before().clone();
            *output.change_mut().after_mut() = change.change().after().clone();
            let actions: Vec<_> = change.change().actions().iter().map(|a| a.as_str()).collect();
            *output.change_mut().action_mut() = ChangeAction::from_actions(&actions);
        }
        root.resources_mut().push(output);
    }
    root
}
