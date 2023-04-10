use anchor_lang::{prelude::*, solana_program::*};
use anyhow::{Context, Result};
use spl_token::state::Account as TokenAccount;
use std::{collections::HashMap, convert::TryInto};

use crate::amms::amm::{Amm, KeyedAccount, Quote, QuoteParams, SwapLegAndAccountMetas, SwapParams};

use jupiter::{
    accounts::TokenSwap,
    jupiter_override::{Swap, SwapLeg},
};

use crate::pool::Pool;

pub struct BalansolAmm {
    key: Pubkey,
    label: String,
    program_id: Pubkey,
    mints: Vec<Pubkey>,
    treasuries: Vec<Pubkey>,
    reserves: Vec<u64>,
}

impl BalansolAmm {
    pub fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let account: Pool =
            Pool::try_deserialize(&mut keyed_account.account.data.clone().as_ref()).unwrap();
        Ok(Self {
            key: keyed_account.key,
            label: "Balansol".to_string(),
            mints: account.mints,
            treasuries: account.treasuries,
            reserves: account.reserves,
            program_id: keyed_account.account.owner,
        })
    }

    fn get_authority(&self) -> Pubkey {
        Pubkey::find_program_address(&[&self.key.to_bytes()], &self.program_id).0
    }

    fn clone(&self) -> BalansolAmm {
        BalansolAmm {
            key: self.key,
            label: self.label.clone(),
            treasuries: self.treasuries.clone(),
            mints: self.mints.clone(),
            program_id: self.program_id.clone(),
            reserves: self.reserves.clone(),
        }
    }
}

impl Amm for BalansolAmm {
    fn label(&self) -> String {
        self.label.clone()
    }

    fn key(&self) -> Pubkey {
        self.key
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        self.mints.clone()
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        self.treasuries.clone()
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> Result<()> {
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        Ok(Quote {
            out_amount: 99999,
            ..Quote::default()
        })
    }

    fn get_swap_leg_and_account_metas(
        &self,
        swap_params: &SwapParams,
    ) -> Result<SwapLegAndAccountMetas> {
        let SwapParams {
            destination_mint,
            in_amount,
            source_mint,
            user_destination_token_account,
            user_source_token_account,
            user_transfer_authority,
            open_order_address,
            quote_mint_to_referrer,
        } = swap_params;

        let (swap_source, swap_destination) = if *source_mint == self.program_id {
            (self.program_id, self.program_id)
        } else {
            (self.program_id, self.program_id)
        };

        let account_metas = TokenSwap {
            destination: *user_destination_token_account,
            source: *user_source_token_account,
            user_transfer_authority: *user_transfer_authority,
            authority: self.program_id,
            token_swap_program: self.program_id,
            token_program: spl_token::ID,
            swap: self.key,
            pool_mint: self.program_id,
            pool_fee: self.program_id,
            swap_destination,
            swap_source,
        }
        .to_account_metas(None);

        Ok(SwapLegAndAccountMetas {
            swap_leg: SwapLeg::Swap {
                swap: Swap::TokenSwap,
            },
            account_metas,
        })
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}
