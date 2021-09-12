use create_type_spec_derive::CreateTypeSpec;
use read_write_state_derive::ReadWriteState;

/// ## Description
/// This structure describes contract version base state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug, Default)]
pub struct ContractVersionBase {
    /// contract name
    pub name: String,
    /// contract version
    pub version: String,
}

impl ContractVersionBase {
    /// ## Description
    /// Creates contract version base extension state
    /// ## Params
    /// * **name** is an object of type [`str`]
    ///
    /// * **version** is an object of type [`str`]
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    /// ## Description
    /// Sets new name and version
    /// ## Params
    /// * **name** is an object of type [`str`]
    ///
    /// * **version** is an object of type [`str`]
  