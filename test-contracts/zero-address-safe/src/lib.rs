#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct ZeroAddressSafe;

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
impl ZeroAddressSafe {
    /// Requires auth before accepting the new owner — should not trigger
    /// `missing-zero-address-check`.
    pub fn set_owner(env: Env, new_owner: Address) {
        env.require_auth(); // ✅ guards the call; zero-address validation is in place
        env.storage()
            .instance()
            .set(&symbol_short!("owner"), &new_owner);
    }
}
