
use pbc_contract_common::{
    address::{Address, ShortnameCallback},
    context::CallbackContext,
    events::EventGroupBuilder,
};
use pbc_traits::ReadWriteRPC;

/// ## Description
/// This trait describes methods that must be implemented
/// in order to be able to convert a struct into rpc event
pub trait IntoShortnameRPCEvent {
    fn action_shortname(&self) -> u32;
    fn as_interaction(&self, builder: &mut EventGroupBuilder, dest: &Address);
}

/// ## Description
/// This trait describes methods that must be implemented
/// in order to be able to convert a struct into rpc event with specified cost
pub trait IntoShortnameRPCEventWithCost {
    fn action_shortname(&self) -> u32;
    fn as_interaction(&self, builder: &mut EventGroupBuilder, dest: &Address, cost: u64);
}

/// ## Description
/// Creates a callback event and adds it to event group builder object
/// ## Params
/// * **builder** is an object of type [`EventGroupBuilder`]
///
/// * **callback_byte** is an object of type [`u32`]
///
/// * **msg** is an object of type [`T`]
#[inline]
pub fn build_msg_callback<T>(builder: &mut EventGroupBuilder, callback_byte: u32, msg: &T)
where
    T: ReadWriteRPC + Clone,
{
    builder