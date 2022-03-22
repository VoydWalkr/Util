use num_rational::Rational64;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

/// Standard `num_rational::Rational64` type alias used commonly across the VoydWalkr Suite.
pub type Rate = Rational64;

/// Serializable bean version of `num_rational::Rational64`.
/// Note that it does not simplify the fraction - this happens only in the `Rational64::new` constructor.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema)]
pub struct SerRate {
  numer: i64,
  denom: i64,
}
impl SerRate {
  pub fn new(numer: i64, denom: i64) -> SerRate {
    SerRate { numer: numer, denom: denom }
  }
}

impl From<SerRate> for Rate {
  fn from(rate: SerRate) -> Rate {
    Rate::new(rate.numer, rate.denom)
  }
}
impl From<Rate> for SerRate {
  fn from(rate: Rate) -> SerRate {
    SerRate::new(rate.numer().clone(), rate.denom().clone())
  }
}
impl From<SerRate> for String {
  fn from(rate: SerRate) -> String {
    format!("{}/{}", rate.numer, rate.denom)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_rate_from() {
    assert_eq!(SerRate::from(Rate::new(42, 2)), SerRate::new(21, 1));
    assert_eq!(Rate::from(SerRate::new(42, 2)), Rate::new(21, 1));
  }
}
