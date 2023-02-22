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
    /// * **numerator** is an object of type [`u128`]
    ///
    /// * **denominator** is an object of type [`u129`]
    pub fn from_ratio(numerator: u128, denominator: u128) -> Self {
        let a = Decimal::from_u128(numerator).unwrap();
        let b = Decimal::from_u128(denominator).unwrap();

        a.checked_div(b).unwrap().into()
    }

    /// ## Description
    /// Returns [`u128`] converted value with cutted decimals
    pub fn to_u128(&self) -> u128 {
        Decimal::from_i128_with_scale(self.numerator as i128, self.scale)
            .to_u128()
            .unwrap()
    }
}

impl From<Decimal> for DecimalRatio {
    fn from(mut num: Decimal) -> Self {
        let scale = num.scale();
        num.set_scale(0).unwrap();

        Self {
            numerator: num.to_u128().unwrap(),
            scale,
        }
    }
}

impl From<DecimalRatio> for Decimal {
    fn from(dr: DecimalRatio) -> Self {
        Dec