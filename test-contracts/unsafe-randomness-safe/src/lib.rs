#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct UnsafeRandomnessSafe;

#[contractimpl]
impl UnsafeRandomnessSafe {
    /// Uses oracle for randomness — safe.
    pub fn draw_winner(_env: Env, random_oracle: Address) -> u32 {
        let _oracle = soroban_sdk::token::Client::new(&_env, &random_oracle);
        42
    }

    /// Does not use ledger timestamp or sequence for critical logic — safe.
    pub fn log_block_info(env: Env) -> u64 {
        let _network = env.ledger().network_id();
        0
    }
}
