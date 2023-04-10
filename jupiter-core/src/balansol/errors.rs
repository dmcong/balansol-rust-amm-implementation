use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Operation overflowed")]
  Overflow,
  #[msg("Not have permission!")]
  InvalidPermission,
  // params
  #[msg("Invalid length of parameters!")]
  ParamsLength,
  #[msg("Zero value is invalid!")]
  ParamsZero,
  #[msg("Invalid weights!")]
  ParamsWeights,
  // accounts
  #[msg("Invalid mint address!")]
  AccountMint,
  #[msg("Invalid treasury address!")]
  AccountTreasury,
  // pool state
  #[msg("Invalid pool state!")]
  PoolState,
  #[msg("Pool is Stopped!")]
  PoolStopped,
  #[msg("Pool is not active!")]
  PoolNotActive,
  #[msg("Pool is not frozen!")]
  PoolNotFrozen,
  // mint state
  #[msg("Invalid mint state!")]
  MintState,
  // calc
  #[msg("Cant calculate starting lpt")]
  CalcStartingLpt,
  #[msg("Cant withdraw sigle")]
  CalcWithdrawSingle,
  #[msg("Cant calculate swap")]
  CalcSwap,
  #[msg("Cant calculate full side lpt")]
  CalcFullSizeLpt,
  #[msg("Cant calculate withdraw lpt")]
  CalcWithdrawLpt,
  #[msg("Cant calculate full side lpt")]
  CalcSideSizeLpt,
  //
  #[msg("Slippage error")]
  Slippage,
}
