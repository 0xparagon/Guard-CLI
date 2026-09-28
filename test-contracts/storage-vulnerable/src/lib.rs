#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct StorageVulnerable;

const KEY: Symbol = symbol_short!("cache");

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
impl StorageVulnerable {
    /// Writes to temporary storage (TTL-bound) — should trigger `unsafe-storage-patterns` (Medium).
    pub fn stash(env: Env, v: u32) {
        env.require_auth();
        env.storage().temporary().set(&KEY, &v);
    }
}
