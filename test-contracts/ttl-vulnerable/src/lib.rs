#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct TtlVulnerable;

const KEY: Symbol = symbol_short!("data");
const KEY2: Symbol = symbol_short!("meta");

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
impl TtlVulnerable {
    /// Writes to persistent storage but never calls extend_ttl — entry can expire.
    pub fn store(env: Env, v: u32) {
        env.require_auth();
        env.storage().persistent().set(&KEY, &v);
    }

    /// Writes two persistent keys (KEY and KEY2) but only extends the TTL for KEY2.
    /// This is the regression case for issue #362: the old function-wide `has_extend`
    /// flag would have suppressed the finding for KEY, which is incorrect.
    pub fn update(env: Env, a: u32, b: u32) {
        env.require_auth();
        env.storage().persistent().set(&KEY, &a);
        env.storage().persistent().set(&KEY2, &b);
        // Only KEY2 gets its TTL extended — KEY is left vulnerable.
        env.storage().persistent().extend_ttl(&KEY2, 100, 1000);
    }
}
