#![no_std]
use soroban_sdk::{contract, contractimpl, BytesN, Env};

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

/// The `unprotected-upgrade` check flags any `invoke_wasm` call; provide it
/// here as a thin wrapper over the real upgrade primitive
/// (`Deployer::update_current_contract_wasm`), since `Env` has no such
/// method itself.
trait InvokeWasmExt {
    fn invoke_wasm(&self, wasm_hash: &BytesN<32>);
}

impl InvokeWasmExt for Env {
    fn invoke_wasm(&self, wasm_hash: &BytesN<32>) {
        self.deployer()
            .update_current_contract_wasm(wasm_hash.clone());
    }
}

#[contract]
pub struct UpgradeSafe;

#[contractimpl]
impl UpgradeSafe {
    /// Protected upgrade with auth — safe.
    pub fn upgrade(env: Env, new_code: BytesN<32>) {
        env.require_auth();
        env.invoke_wasm(&new_code);
    }
}
