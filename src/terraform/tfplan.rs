/// See https://developer.hashicorp.com/terraform/internals/json-format#plan-representation

use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

/// A plan consists of a prior state, the configuration that is being applied to that state, and the
/// set of changes Terraform plans to make to achieve that.
///
/// For ease of consumption by callers, the plan representation includes a partial representation of
/// the values in the final state (using a [value representation](https://developer.hashicorp.com/terraform/internals/json-format#values-representation)),
/// allowing callers to easily analyze the planned outcome using similar code as for analyzing the prior state.
#[derive(Debug,Deserialize,Serialize)]
pub struct TfPlan {
    /// The output includes a `format_version` key, which as of Terraform 1.1.0 has value `"1.0"`. The
    /// semantics of this version are:
    ///
    /// * We will increment the minor version, e.g. `"1.1"`, for backward-compatible changes or
    /// additions. Ignore any object properties with unrecognized names to remain forward-compatible
    /// with future minor versions.
    ///
    /// * We will increment the major version, e.g. `"2.0"`, for changes that are not backward-compatible.
    /// Reject any input which reports an unsupported major version.
    format_version: String,
    terraform_version: String,
    /// "configuration" is a representation of the configuration being applied to the prior state,
    /// using the configuration representation described above.
    configuration: Root<ConfigurationResource>,
    /// "planned_values" is a description of what is known so far of the outcome in the standard
    /// value representation, with any as-yet-unknown values omitted.
    planned_values: Root<ValueResource>,
    /// "resource_changes" is a description of the individual change actions that Terraform plans to
    /// use to move from the prior state to a new state matching the configuration.
    #[serde(default="Vec::new")]
    resource_changes: Vec<ResourceChange>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Root<R> {
    root_module: Module<R>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Module<R> {
    /// "resources" describes the "resource" and "data" blocks in the module
    #[serde(default = "Vec::new")]
    resources: Vec<R>,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct ValueResource {
    /// "address" is the opaque absolute address for the resource itself.
    address: String,
    /// "mode" can be "managed", for resources, or "data", for data resources
    mode: String,
    r#type: String,
    name: String,
    provider_name: String,
    /// "values" is the JSON representation of the attribute values of the
    /// resource, whose structure depends on the resource type schema. Any
    /// unknown values are omitted or set to null, making them
    /// indistinguishable from absent values; callers which need to distinguish
    /// unknown from unset must use the plan-specific or configuration-specific
    /// structures described in later sections.
    values: Json,
    sensitive_values: Json,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct ConfigurationResource {
    /// "address" is the opaque absolute address for the resource itself.
    address: String,
    /// "mode" can be "managed", for resources, or "data", for data resources
    mode: String,
    r#type: String,
    name: String,
    provider_config_key: String,
    /// "expressions" describes the resource-type-specific content of the
    /// configuration block.
    expressions: Json,
}

/// Each element describes the action to take for one instance object. All resources in the
/// configuration are included in this list.
#[derive(Debug,Deserialize,Serialize)]
pub struct ResourceChange {
    /// "address" is the full absolute address of the resource instance this change applies to, in
    /// the same format as addresses in a value representation.
    address: String,
    /// "mode" can be "managed", for resources, or "data", for data resources
    mode: String,
    r#type: String,
    name: String,
    provider_name: String,
    change: Change,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Change {
    /// "actions" are the actions that will be taken on the object selected by the
    /// properties below.
    /// Valid actions values are:
    ///
    /// * `["no-op"]`
    /// * `["create"]`
    /// * `["read"]`
    /// * `["update"]`
    /// * `["delete", "create"]`
    /// * `["create", "delete"]`
    /// * `["delete"]`
    ///
    /// The two "replace" actions are represented in this way to allow callers to
    /// e.g. just scan the list for "delete" to recognize all three situations
    /// where the object will be deleted, allowing for any new deletion
    /// combinations that might be added in future.
    actions: Vec<String>,
    /// "before" and "after" are representations of the object value both before
    /// and after the action. For ["create"] and ["delete"] actions, either
    /// "before" or "after" is unset (respectively). For ["no-op"], the before and
    /// after values are identical. The "after" value will be incomplete if there
    /// are values within it that won't be known until after apply.
    before: Json,
    /// "before" and "after" are representations of the object value both before
    /// and after the action. For ["create"] and ["delete"] actions, either
    /// "before" or "after" is unset (respectively). For ["no-op"], the before and
    /// after values are identical. The "after" value will be incomplete if there
    /// are values within it that won't be known until after apply.
    after: Json,
    /// "after_unknown" is an object value with similar structure to "after", but
    /// with all unknown leaf values replaced with "true", and all known leaf
    /// values omitted. This can be combined with "after" to reconstruct a full
    /// value after the action, including values which will only be known after
    /// apply.
    after_unknown: Json,
    /// "before_sensitive" and "after_sensitive" are object values with similar
    /// structure to "before" and "after", but with all sensitive leaf values
    /// replaced with true, and all non-sensitive leaf values omitted. These
    /// objects should be combined with "before" and "after" to prevent accidental
    /// display of sensitive values in user interfaces.
    before_sensitive: Json,
    /// "before_sensitive" and "after_sensitive" are object values with similar
    /// structure to "before" and "after", but with all sensitive leaf values
    /// replaced with true, and all non-sensitive leaf values omitted. These
    /// objects should be combined with "before" and "after" to prevent accidental
    /// display of sensitive values in user interfaces.
    after_sensitive: Json,
}

impl TfPlan {
    pub fn planned_values(&self) -> &Root<ValueResource> {
        &self.planned_values
    }
    pub fn configuration(&self) -> &Root<ConfigurationResource> {
        &self.configuration
    }
    pub fn resource_changes(&self) -> &Vec<ResourceChange> {
        &self.resource_changes
    }
}

impl<R> Root<R> {
    pub fn root_module(&self) -> &Module<R> {
        &self.root_module
    }
}

impl<R> Module<R> {
    pub fn resources(&self) -> &Vec<R> {
        &self.resources
    }
}

impl ValueResource {
    pub fn address(&self) -> &String {
        &self.address
    }
    pub fn r#type(&self) -> &String {
        &self.r#type
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn provider_name(&self) -> &String {
        &self.provider_name
    }
    pub fn values(&self) -> &Json {
        &self.values
    }
    pub fn sensitive_values(&self) -> &Json {
        &self.sensitive_values
    }
}

impl ConfigurationResource {
    pub fn address(&self) -> &String {
        &self.address
    }
    pub fn r#type(&self) -> &String {
        &self.r#type
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn provider_config_key(&self) -> &String {
        &self.provider_config_key
    }
    pub fn expressions(&self) -> &Json {
        &self.expressions
    }
}

impl ResourceChange {
    pub fn address(&self) -> &String {
        &self.address
    }
    pub fn r#type(&self) -> &String {
        &self.r#type
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn provider_name(&self) -> &String {
        &self.provider_name
    }
    pub fn change(&self) -> &Change {
        &self.change
    }
}

impl Change {
    pub fn actions(&self) -> &Vec<String> {
        &self.actions
    }
    pub fn before(&self) -> &Json {
        &self.before
    }
    pub fn after(&self) -> &Json {
        &self.after
    }
    pub fn after_unknown(&self) -> &Json {
        &self.after_unknown
    }
    pub fn before_sensitive(&self) -> &Json {
        &self.before_sensitive
    }
    pub fn after_sensitive(&self) -> &Json {
        &self.after_sensitive
    }
}
