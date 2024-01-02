use crate::schema::pool::{MintActionState, PoolState};
use anchor_lang::prelude::*;

///
/// Pool operation trait
///
pub trait Operation {
  // True if frozen else False
  fn is_frozen(&self) -> bool;
  fn is_initializing(&self) -> bool;
  fn is_active(&self) -> bool;
  fn valid_mint_states(&self, idx: usize, mint_states: Vec<MintActionState>) -> bool;
  fn valid_pool_states(&self, pool_states: Vec<PoolState>) -> bool;
}

///
/// Pool oracle trait
///
pub trait Exchange {
  fn calc_ask_amount_swap(
    &self,
    bid_amount: u64,
    bid_mint: Pubkey,
    ask_mint: Pubkey,
    total_fee: u64,
  ) -> Option<u64>;
}

///
/// Pool Accessor trait
///
pub trait Accessor {
  fn add_reserve(&mut self, idx: usize, new_reserve: u64) -> Option<u64>;
  fn sub_reserve(&mut self, idx: usize, new_reserve: u64) -> Option<u64>;
  fn get_mint_index(&self, mint: Pubkey) -> Option<usize>;
  fn get_treasury_index(&self, mint: Pubkey) -> Option<usize>;
  fn get_normalized_weight(&self, mint: Pubkey) -> Option<f64>;
  fn get_reserve(&self, mint: Pubkey) -> Option<u64>;
}
