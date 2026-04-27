#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_win_6767() {
    let env = Env::default();
    let player = Address::generate(&env);

    let result = LuckySpin::spin(env.clone(), player, 6, 7, 6, 7);
    assert!(result);
}

#[test]
fn test_loss() {
    let env = Env::default();
    let player = Address::generate(&env);

    let result = LuckySpin::spin(env.clone(), player, 1, 2, 3, 4);
    assert!(!result);
}

#[test]
fn test_state_saved() {
    let env = Env::default();
    let player = Address::generate(&env);

    LuckySpin::spin(env.clone(), player.clone(), 6, 7, 6, 7);

    let state = LuckySpin::get_last_spin(env);
    assert_eq!(state.last_result, (6, 7, 6, 7));
}

#[test]
fn test_reward_call() {
    let env = Env::default();
    let player = Address::generate(&env);

    LuckySpin::reward(env.clone(), player);
}

#[test]
fn test_spin_updates_player() {
    let env = Env::default();
    let player = Address::generate(&env);

    LuckySpin::spin(env.clone(), player.clone(), 6, 7, 6, 7);

    let state = LuckySpin::get_last_spin(env);
    assert_eq!(state.last_player, player);
}