#![no_std]
//! # Dummy Soroban Contract
//!
//! A minimal counter contract used exclusively to dogfood and validate the
//! `soroban-actions` composite action.  It intentionally exercises:
//!
//! - `soroban_sdk::contract` + `soroban_sdk::contractimpl` macros
//! - Persistent ledger storage via `Env::storage()`
//! - Arithmetic that the compiler can overflow-check
//! - Unit tests that run under `cargo test` (native, no emulator needed)

use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER_KEY: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    /// Increment the on-chain counter by `amount` and return the new value.
    pub fn increment(env: Env, amount: u32) -> u32 {
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(0u32);

        count = count.checked_add(amount).expect("counter overflow");

        env.storage()
            .instance()
            .set(&COUNTER_KEY, &count);

        log!(&env, "counter incremented to {}", count);
        count
    }

    /// Return the current counter value without modifying state.
    pub fn get(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&COUNTER_KEY)
            .unwrap_or(0u32)
    }

    /// Reset the counter back to zero.
    pub fn reset(env: Env) {
        env.storage()
            .instance()
            .set(&COUNTER_KEY, &0u32);
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Unit tests (native – no Wasm emulator required)
// ──────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Ledger;
    use soroban_sdk::Env;

    fn make_env() -> Env {
        Env::default()
    }

    #[test]
    fn test_get_initial_value_is_zero() {
        let env = make_env();
        let contract_id = env.register(CounterContract, ());
        let client = CounterContractClient::new(&env, &contract_id);

        assert_eq!(client.get(), 0, "initial counter must be 0");
    }

    #[test]
    fn test_increment_by_one() {
        let env = make_env();
        let contract_id = env.register(CounterContract, ());
        let client = CounterContractClient::new(&env, &contract_id);

        let result = client.increment(&1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_increment_multiple_times() {
        let env = make_env();
        let contract_id = env.register(CounterContract, ());
        let client = CounterContractClient::new(&env, &contract_id);

        assert_eq!(client.increment(&5), 5);
        assert_eq!(client.increment(&3), 8);
        assert_eq!(client.increment(&2), 10);
        assert_eq!(client.get(), 10);
    }

    #[test]
    fn test_reset_returns_zero() {
        let env = make_env();
        let contract_id = env.register(CounterContract, ());
        let client = CounterContractClient::new(&env, &contract_id);

        client.increment(&42);
        assert_eq!(client.get(), 42);

        client.reset();
        assert_eq!(client.get(), 0);
    }

    #[test]
    fn test_get_after_reset_is_zero() {
        let env = make_env();
        let contract_id = env.register(CounterContract, ());
        let client = CounterContractClient::new(&env, &contract_id);

        client.increment(&100);
        client.reset();
        assert_eq!(client.get(), 0);
    }

    #[test]
    fn test_ledger_sequence_advances() {
        let env = make_env();
        env.ledger().set_sequence_number(1000);
        let contract_id = env.register(CounterContract, ());
        let client = CounterContractClient::new(&env, &contract_id);

        client.increment(&1);
        assert_eq!(env.ledger().sequence(), 1000);
    }
}
