#![no_std]
use soroban_sdk::{contract, contractimpl, Bytes, Env};

#[contract]
pub struct ContractDeploymentSafe;

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
impl ContractDeploymentSafe {
    pub fn upload(env: Env, wasm: Bytes) {
        env.require_auth();
        env.deployer().upload_contract_wasm(wasm);
    }
}
