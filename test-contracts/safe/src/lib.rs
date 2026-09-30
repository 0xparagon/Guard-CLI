#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct SafeContract;

const KEY: Symbol = symbol_short!("owner");

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
impl SafeContract {
    /// Writes storage only after `env.require_auth()` — should pass `missing-require-auth`.
    pub fn set_owner(env: Env, new_owner: Address) {
        env.require_auth();
        env.storage().instance().set(&KEY, &new_owner);
    }
}
