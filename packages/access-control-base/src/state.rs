
use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::{address::Address, context::ContractContext};
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

pub const DEFAULT_ADMIN_ROLE: u8 = 0x00;

/// ## Description
/// This structure describes access control extension state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug, Default)]
pub struct AccessControlBaseState {
    /// configured roles
    pub roles: BTreeMap<u8, Role>,
}

/// ## Description
/// This structure describes role with some granted access control
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Role {
    /// configured admin role
    pub admin_role: u8,
    /// whitelisted role members
    pub members: BTreeMap<Address, bool>,
}

impl AccessControlBaseState {
    /// ## Description
    /// Grants specified tole to specified account
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **account** is an object of type [`Address`]
    ///
    /// * **ctx** is an object of type [`ContractContext`]
    pub fn grant_role(&mut self, role: u8, account: &Address, ctx: &ContractContext) {
        self.assert_only_role(self.get_role_admin(role).unwrap(), ctx);
        self.set_role(role, account);
    }

    /// ## Description
    /// Setups new role
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **account** is an object of type [`Address`]
    pub fn setup_role(&mut self, role: u8, account: &Address) {
        self.set_role(role, account);
    }

    /// ## Description
    /// Removes role access for specified account
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **account** is an object of type [`Address`]
    ///
    /// * **ctx** is an object of type [`ContractContext`]
    pub fn revoke_role(&mut self, role: u8, account: &Address, ctx: &ContractContext) {
        self.assert_only_role(self.get_role_admin(role).unwrap(), ctx);

        if self.has_role(role, account) {
            self.roles.entry(role).and_modify(|role| {
                role.members.remove(account);
            });
        }
    }

    /// ## Description
    /// Removes sender access to role
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **ctx** is an object of type [`ContractContext`]
    pub fn renounce_role(&mut self, role: u8, ctx: &ContractContext) {
        if self.has_role(role, &ctx.sender) {
            self.roles.entry(role).and_modify(|role| {
                role.members.remove(&ctx.sender);
            });
        }
    }

    /// ## Description
    /// Sets new tole admin for role
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **admin_role** is an object of type [`u8`]
    pub fn set_role_admin(&mut self, role: u8, admin_role: u8) {
        self.roles
            .entry(role)
            .and_modify(|role| role.admin_role = admin_role)
            .or_insert(Role {
                admin_role,
                members: BTreeMap::new(),