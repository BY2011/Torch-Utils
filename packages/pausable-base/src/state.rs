use create_type_spec_derive::CreateTypeSpec;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// ## Description
/// This structure describes pausable extension state
#[derive(ReadWriteState, Create