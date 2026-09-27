#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct AdminSafe;

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
impl AdminSafe {
    /// Same entrypoint name with `env.require_auth()` — should pass `unprotected-admin`.
    pub fn set_owner(env: Env, _new_owner: Address) {
        env.require_auth();
    }
}
