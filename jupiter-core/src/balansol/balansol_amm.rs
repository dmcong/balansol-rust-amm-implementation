use anchor_lang::prelude::*;
use anyhow::{Ok, Result};
use std::collections::HashMap;

use crate::pool::Pool;
use crate::{
    amms::amm::{Amm, KeyedAccount, Quote, QuoteParams, SwapParams},
    Exchange,
};

pub struct BalansolAmm {
    key: Pubkey,
    label: String,
    program_id: Pubkey,
    pool: Pool,
}

impl BalansolAmm {
    pub fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let pool: Pool = Pool::try_deserialize(&mut keyed_account.account.data.as_ref()).unwrap();
        Ok(Self {
            key: keyed_account.key,
            label: "Balansol".to_string(),
            program_id: keyed_account.account.owner,
            pool,
        })
    }

    fn clone(&self) -> BalansolAmm {
        BalansolAmm {
            key: self.key,
            label: self.label.clone(),
            program_id: self.program_id.clone(),
            pool: self.pool.clone(),
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
        self.pool.mints.clone()
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![self.key]
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Vec<u8>>) -> Result<()> {
        let pool_account = accounts_map.get(&self.key).unwrap();
        let pool: Pool = Pool::try_deserialize(&mut pool_account.as_ref()).unwrap();
        self.pool = pool;
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let QuoteParams {
            in_amount,
            input_mint,
            output_mint,
        } = quote_params;
        let pool = &self.pool;

        let total_fee = pool.fee.checked_add(pool.tax).unwrap();
        let out_amount = pool
            .calc_ask_amount_swap(*in_amount, *input_mint, *output_mint, total_fee)
            .unwrap();

        Ok(Quote {
            in_amount: *in_amount,
            out_amount,
            fee_mint: *output_mint,
            ..Quote::default()
        })
    }

    fn get_swap_leg_and_account_metas(&self, swap_params: &SwapParams) -> Result<bool> {
        // Reference
        // 1. balansol/instruction/balansol_swap.rs
        // 2. balansol/balansol-idl.json
        Ok(false)
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }
}
