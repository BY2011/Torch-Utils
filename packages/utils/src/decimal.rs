use rust_decimal::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

use create_type_spec_derive::CreateTypeSpec;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/// ## Description
/// This structure describes wasm compatible decimal wrapper.
#[derive(
    ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, Copy, Eq, PartialEq, Debug, Default,
)]
pub struct DecimalRat