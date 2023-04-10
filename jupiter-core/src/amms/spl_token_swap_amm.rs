use std::collections::HashMap;

use lazy_static::lazy_static;
use solana_sdk::{pubkey, pubkey::Pubkey};

use crate::{
    amm::{Amm, QuoteParams},
    BalansolAmm,
};

mod spl_token_swap_programs {
    use super::*;
    pub const BALANSOL: Pubkey = pubkey!("6SRa2Kc3G4wTG319G4Se6yrRWeS1A1Hj79BC3o7X9v6T");
}

lazy_static! {
    pub static ref SPL_TOKEN_SWAP_PROGRAMS: HashMap<Pubkey, String> = {
        let mut m = HashMap::new();
        m.insert(spl_token_swap_programs::BALANSOL, "Balansol".into());
        m
    };
}

#[test]
fn test_new_spl_token_swap() -> Result<(), String> {
    use crate::amms::test_harness::AmmTestHarness;

    // Devnet
    pub const USDC_MINT: Pubkey = pubkey!("2z6Ci38Cx6PyL3tFrT95vbEeB3izqpoLdxxBkJk2euyj");
    pub const SNTR_MINT: Pubkey = pubkey!("5YwUkPdXLoujGkZuo9B4LsLKj3hdkDcfP4derpspifSJ");
    const USDC_SNTR_POOL: Pubkey = pubkey!("HVBPjtbK4Hrk7DHj1GjaFwprj5oLmQKoGMBti1rZQKuo");

    let test_harness = AmmTestHarness::new();

    let keyed_account = test_harness.get_keyed_account(USDC_SNTR_POOL).unwrap();
    let amm = BalansolAmm::from_keyed_account(&keyed_account).unwrap();

    let quote = amm
        .quote(&QuoteParams {
            input_mint: USDC_MINT,
            in_amount: 1000000000, // 1USDC
            output_mint: SNTR_MINT,
        })
        .unwrap();

    println!("Quote result: {:?}", quote);

    Err("STOP".to_string())
}
