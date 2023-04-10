use crate::constant::*;
use crate::f64_trait::F64Trait;
use num_traits::ToPrimitive;

pub fn calc_normalize_weight(weight_idx: usize, weights: Vec<u64>) -> Option<f64> {
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
    let _bi_bi_ai = bid_reserve
        .to_f64()?
        .checked_div(bid_reserve.to_f64()?.checked_add(bid_amount.to_f64()?)?)?;
    let _wi_wo = bid_weight.checked_div(ask_weight)?;
    let ask_amount = ask_reserve
        .to_f64()?
        .checked_mul(1_f64.checked_sub(_bi_bi_ai.checked_pow(_wi_wo)?)?)?;
    // fee
    let total_fee = fee.to_f64()?.checked_div(PRECISION)?;
    return Some((ask_amount.checked_mul(1_f64.checked_sub(total_fee)?)?).to_u64()?);
}
