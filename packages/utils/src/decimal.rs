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
pub struct DecimalRatio {
    // numerator
    numerator: u128,
    // decimal number scale
    scale: u32,
}

impl DecimalRatio {
    /// ## Description
    /// Creates new instance of [`DecimalRatio`] with initial values
    /// ## Params
    /// * **numerator** is an object of type [`u128`]
    ///
    /// * **scale** is an object of type [`u32`]
    pub fn new(numerator: u128, scale: u32) -> Self {
        Self { numerator, scale }
    }

    /// ## Description
    /// Returns [`DecimalRatio`] with value equals to 0
    pub fn zero() -> Self {
        Decimal::ZERO.into()
    }

    /// ## Description
    /// Returns [`DecimalRatio`] with value equals to 1
    pub fn one() -> Self {
        Decimal::ONE.into()
    }

    /// ## Description
    /// Performes native decimal division and returns wrapped
    /// [`DecimalRatio`] result
    /// ## Params
    /// * **numerator** is an ob