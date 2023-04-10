# Balansol

## _rust-amm-implementation_

This is a guide to help implementation Balansol with Jupiter

- **Balansol Amm**: jupiter-core/balansol/**balansol_amm.rs**
- **Balansol Swap IDL**: jupiter-core/balansol/**balansol-idl.json**

### Instruction Name: 'Swap'

## Accounts

| Account Name                 | Description                                               |
| ---------------------------- | --------------------------------------------------------- |
| authority                    | Signer                                                    |
| pool                         | Pool pubKey                                               |
| taxman                       | Pool.tax_man                                              |
| bid_mint                     | Mint: Input mint                                          |
| treasurer                    | PDA ([ "treasurer", pool.key() ], BalansolProgramID )     |
| src_treasury                 | TokenAccount: mint=bid_mint, authority=treasurer          |
| src_associated_token_account | TokenAccount: mint=bid_mint, authority=authority (Signer) |
| ask_mint                     | Mint: output_mint                                         |
| dst_treasury                 | TokenAccount: mint=ask_mint, authority=treasurer          |
| dst_associated_token_account | TokenAccount: mint=ask_mint, authority=authority (Signer) |
| dst_token_account_taxman     | TokenAccount: mint=ask_mint, authority=taxman => Fee      |
| system_program               | Program<'info, System>,                                   |
| token_program                | Program<'info, token::Token>                              |
| associated_token_program     | Program<'info, associated_token::AssociatedToken>         |
| rent                         | Sysvar<'info, Rent>,                                      |

## Params

| Account Name | Description           |
| ------------ | --------------------- |
| bid_amount   | u64: Input amount     |
| limit        | u64: Out amount limit |
