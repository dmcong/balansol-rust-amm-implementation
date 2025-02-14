use crate::constant::*;
use crate::oracle::*;
use crate::pool_trait::*;
use anchor_lang::prelude::*;
use anchor_spl::token;

///
/// Pool state
///
#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum PoolState {
  Uninitialized,
  Initialized,
  Frozen,
  Deleted,
  Initializing,
}
impl Default for PoolState {
  fn default() -> Self {
    PoolState::Uninitialized
  }
}

///
/// Mint state
///
#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum MintActionState {
  Active,
  BidOnly,
  AskOnly,
  Paused,
}
impl Default for MintActionState {
  fn default() -> Self {
    MintActionState::Active
  }
}

pub struct PoolValidation {
  pub mint_states: Vec<MintActionState>,
  pub pool_states: Vec<PoolState>,
}

#[account]
pub struct Pool {
  pub authority: Pubkey,
  pub fee: u64,
  pub tax: u64,
  pub state: PoolState,
  pub mint_lpt: Pubkey,
  pub taxman: Pubkey,
  pub mints: Vec<Pubkey>,
  pub actions: Vec<MintActionState>,
  pub treasuries: Vec<Pubkey>,
  pub reserves: Vec<u64>,
  pub weights: Vec<u64>,
}
const VECTOR_IN_POOL: usize = 5;

impl Pool {
  pub const LEN: usize = ACCOUNT_DISCRIMINATOR
    + PUBKEY_SIZE
    + U64_SIZE
    + U64_SIZE
    + U8_SIZE
    + PUBKEY_SIZE
    + PUBKEY_SIZE
    + MAXIMUM_MINT_NUMBER * (PUBKEY_SIZE + U8_SIZE + PUBKEY_SIZE + U64_SIZE + U64_SIZE)
    + VECTOR_SIZE * VECTOR_IN_POOL;
  
}
    

///
/// Operation trait
///
impl Operation for Pool {
  fn is_frozen(&self) -> bool {
    self.state == PoolState::Frozen
  }
  fn is_initializing(&self) -> bool {
    return self.state == PoolState::Initializing;
  }
  fn is_active(&self) -> bool {
    return self.state == PoolState::Initialized;
  }
  fn valid_mint_states(&self, mint_idx: usize, mint_states: Vec<MintActionState>) -> bool {
    for idx in 0..mint_states.len() {
      if self.actions[mint_idx] == mint_states[idx] {
        return true;
      }
    }
    return false;
  }
  fn valid_pool_states(&self, pool_states: Vec<PoolState>) -> bool {
    for idx in 0..pool_states.len() {
      if self.state == pool_states[idx] {
        return true;
      }
    }
    return false;
  }
}

///
/// Accessor trait
///
impl Accessor for Pool {
  fn add_reserve(&mut self, idx: usize, amount_in: u64) -> Option<u64> {
    self.reserves[idx] = self.reserves[idx].checked_add(amount_in)?;
    msg!("add_reserve {}->{}->{:?}", idx, amount_in, self.reserves);
    return Some(self.reserves[idx]);
  }
  fn sub_reserve(&mut self, idx: usize, amount_out: u64) -> Option<u64> {
    self.reserves[idx] = self.reserves[idx].checked_sub(amount_out)?;
    msg!("sub_reserve {}->{}->{:?}", idx, amount_out, self.reserves);
    return Some(self.reserves[idx]);
  }
  fn get_normalized_weight(&self, mint: Pubkey) -> Option<f64> {
    let mint_idx = self.get_mint_index(mint)?;
    return Some(normalize_weight(mint_idx, self.weights.clone())?);
  }
  fn get_reserve(&self, mint: Pubkey) -> Option<u64> {
    let mint_idx = self.get_mint_index(mint)?;
    return Some(self.reserves[mint_idx]);
  }
  fn get_mint_index(&self, mint: Pubkey) -> Option<usize> {
    for idx in 0..self.mints.len() {
      if self.mints[idx] == mint {
        return Some(idx);
      }
    }
    return None;
  }
  fn get_treasury_index(&self, treasury: Pubkey) -> Option<usize> {
    for idx in 0..self.mints.len() {
      if self.treasuries[idx] == treasury {
        return Some(idx);
      }
    }
    return None;
  }
}

///
/// Operation trait
///
impl Exchange for Pool {
  fn calc_ask_amount_swap(
    &self,
    bid_amount: u64,
    bid_mint: Pubkey,
    ask_mint: Pubkey,
    total_fee: u64,
  ) -> Option<u64> {
    let bid_mint_idx = self.get_mint_index(bid_mint)?;
    let ask_mint_idx = self.get_mint_index(ask_mint)?;
    let bid_reserve = self.reserves[bid_mint_idx];
    let bid_weight = self.get_normalized_weight(bid_mint)?;
    let ask_reserve = self.reserves[ask_mint_idx];
    let ask_weight = self.get_normalized_weight(ask_mint)?;

    calc_ask_amount_swap(
      bid_amount,
      bid_reserve,
      bid_weight,
      ask_reserve,
      ask_weight,
      total_fee,
    )
  }
}
