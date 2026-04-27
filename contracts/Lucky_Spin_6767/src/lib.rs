#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, log};

#[contract]
pub struct LuckySpin;

#[contracttype]
#[derive(Clone)]
pub struct SpinState {
    pub last_player: Address,
    pub last_result: (u32, u32, u32, u32),
}

const STATE: Symbol = Symbol::short("STATE");

#[contractimpl]
impl LuckySpin {

    // Main spin function (arcade logic)
    pub fn spin(env: Env, player: Address, a: u32, b: u32, c: u32, d: u32) -> bool {
        let result = (a, b, c, d);

        // store last spin state
        let state = SpinState {
            last_player: player.clone(),
            last_result: result,
        };
        env.storage().instance().set(&STATE, &state);

        // check jackpot condition 6767
        if result == (6, 7, 6, 7) {
            log!(&env, "🎰 JACKPOT 6767 TRIGGERED");
            true
        } else {
            log!(&env, "No win this spin");
            false
        }
    }

    // Reward function (placeholder for XLM/token transfer)
    pub fn reward(env: Env, player: Address) {
        log!(&env, "Reward sent to player wallet");
    }

    // View last spin state
    pub fn get_last_spin(env: Env) -> SpinState {
        env.storage().instance().get(&STATE).unwrap()
    }
}