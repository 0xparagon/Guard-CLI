#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[cfg(any())]
use std::collections::HashMap;

#[contract]
pub struct StdImportsVulnerable;

#[contractimpl]
impl StdImportsVulnerable {
    /// Vulnerable: Soroban contracts should not import from `std`.
    pub fn count(env: Env) -> u32 {
        let _ = env;
        0
    }
}
