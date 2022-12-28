use create_type_spec_derive::CreateTypeSpec;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// ## Description
/// This structure describes pausable extension state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PausableBaseState {
    /// paused or not
    paused: bool,
}

impl PausableBaseState {
    /// ## Description
    //