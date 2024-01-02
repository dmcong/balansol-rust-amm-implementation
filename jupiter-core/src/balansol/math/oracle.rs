use crate::constant::*;
use crate::f64_trait::F64Trait;
use num_traits::ToPrimitive;

pub fn normalize_weight(weight_idx: usize, weights: Vec<u64>) -> Option<f64> {
    let total_weight: u64 = weights.iter().sum();
    let weight = weights[weight_idx].to_f64()?;
    return Some(weight.checked_div(total_weight.to_f64()?)?);
  }

pub fn calc_ask_amount_swap(
    bid_amount: u64,
    bid_reserve: u64,
    bid_weight: f64,
    ask_reserve: u64,
    ask_weight: f64,
    fee: u64,
  ) -> Option<u64> {
    let _fee = fee.to_f64()?.checked_div(PRECISION_F64)?;
    let _bid_amount = 1_f64.checked_sub(_fee)?.checked_mul(bid_amount.to_f64()?)?;
    let _bid_reserve = bid_reserve.to_f64()?;
    let _ask_reserve = ask_reserve.to_f64()?;
  
    let balance_ratio = _bid_reserve.checked_div(_bid_reserve.checked_add(_bid_amount)?)?;
    let weight_ratio = bid_weight.checked_div(ask_weight)?;
    let ask_amount = 1_f64
      .checked_sub(balance_ratio.checked_pow(weight_ratio)?)?
      .checked_mul(_ask_reserve)?;
  
    ask_amount.to_u64()
  }
  