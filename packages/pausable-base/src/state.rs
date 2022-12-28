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
    /// Creates pausable extension state
    pub fn new() -> Self {
        Self { paused: false }
    }

    /// ## Description
    /// Pauses contract
    pub fn pause(&mut self) {
        self.assert_not_paused();
        self.paused = true
    }

    /// ## Description
    /// Unpauses contract
    pub fn unpause(&mut self) {
        self.assert_paused();
        self.paused = false
    }

    /// ## Description
    /// Returns current pausable state
    pub fn paused(&self) -> bool {
        self.paused
    }

    /// ## Description
    /// Verifies that contract is paused
    pub fn assert_paused(&self) {
        assert!(self.paused(), "{}", ContractErro