use crate::{
  constant::PRECISION_U64,
  errors::ErrorCode,
  schema::{
    pool::{MintActionState, Pool, PoolState, PoolValidation},
    pool_trait::Exchange,
  },
};
use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token};
use num_traits::ToPrimitive;

pub const MAX_SHARES: u64 = 3;

#[event]
pub struct SwapEvent {
  pub authority: Pubkey,
  pub pool: Pubkey,
  pub bid_mint: Pubkey,
  pub ask_mint: Pubkey,
  pub bid_amount: u64,
  pub limit: u64,
  pub ask_amount: u64,
  pub total_tax_fee_amount: u64,
}

#[derive(Accounts)]
pub struct Swap<'info> {
  #[account(mut)]
  pub authority: Signer<'info>,
  #[account(mut, has_one = taxman)]
  pub pool: Account<'info, Pool>,
  /// CHECK: Just a pure account
  pub taxman: AccountInfo<'info>,

  // bid mint
  pub bid_mint: Box<Account<'info, token::Mint>>,
  #[account(
    seeds = [b"treasurer", &pool.key().to_bytes()],
    bump
  )]
  /// CHECK: Just a pure account
  pub treasurer: AccountInfo<'info>,
  #[account(
    mut,
    associated_token::mint = bid_mint,
    associated_token::authority = treasurer
  )]
  pub src_treasury: Box<Account<'info, token::TokenAccount>>,
  #[account(mut)]
  pub src_associated_token_account: Box<Account<'info, token::TokenAccount>>,

  // ask mint
  pub ask_mint: Box<Account<'info, token::Mint>>,
  #[account(
    mut,
    associated_token::mint = ask_mint,
    associated_token::authority = treasurer
  )]
  pub dst_treasury: Box<Account<'info, token::TokenAccount>>,
  #[account(
    init_if_needed,
    payer = authority,
    associated_token::mint = ask_mint,
    associated_token::authority = authority
  )]
  pub dst_associated_token_account: Box<Account<'info, token::TokenAccount>>,
  #[account(
    init_if_needed,
    payer = authority,
    associated_token::mint = ask_mint,
    associated_token::authority = taxman
  )]
  pub dst_token_account_taxman: Box<Account<'info, token::TokenAccount>>,

  // programs
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, token::Token>,
  pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
  pub rent: Sysvar<'info, Rent>,
}

impl Swap<'_> {
  pub fn invoke<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Swap<'info>>,
    bid_amount: u64,
    limit: u64,
  ) -> Result<u64> {
    let pool = &mut ctx.accounts.pool;
    let seeds: &[&[&[u8]]] = &[&[
      "treasurer".as_ref(),
      &pool.key().to_bytes(),
      &[ctx.bumps.treasurer],
    ]];

    // TODO: Swap
    
    emit!(SwapEvent {
      authority: ctx.accounts.authority.key(),
      pool: pool.key(),
      bid_mint: ctx.accounts.bid_mint.key(),
      ask_mint: ctx.accounts.ask_mint.key(),
      bid_amount,
      limit,
      ask_amount,
      total_tax_fee_amount: tax_amount,
    });

    Ok(ask_amount)
  }
}
