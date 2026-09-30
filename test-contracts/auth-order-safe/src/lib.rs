#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

#[contract]
pub struct AuthOrderSafe;

/// `Env::require_auth` is not public SDK API (it's `pub(crate)` and takes an
/// `&Address`); forward the zero-arg call the checks expect to the real
/// public `Address::require_auth` on the contract's own address.
trait EnvRequireAuthExt {
    fn require_auth(&self);
}

impl EnvRequireAuthExt for Env {
    fn require_auth(&self) {
        self.current_contract_address().require_auth();
    }
}

#[contractimpl]
impl AuthOrderSafe {
    /// require_auth is called first — should pass `auth-after-storage-write`.
    pub fn set_value(env: Env, value: i128) {
        env.require_auth();
        env.storage().persistent().set(&symbol_short!("val"), &value);
    }
}
