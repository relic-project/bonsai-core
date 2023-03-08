use std::fmt;

pub struct Candle {
  pub open: f64,
  pub close: f64,
  pub min: f64,
  pub max: f64
}

impl Default for Candle {
  fn default() -> Self {
    Self {
      close: 0.0,
      open: 0.0,
      min: 0.0,
      max: 0.0
    }
  }
}

impl fmt::Display for Candle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "open: {}, close: {}, min: {}, max: {}", self.open, self.close, self.min, self.max)
  }
}

impl Candle {
  pub fn new(open: f64, close: f64, min: f64, max: f64) -> Self {
    let mut lowest: f64 = open;
    let mut highest: f64 = close;
    if open > close {
      highest = open;
      lowest = close;
    }

    // CHECK INTEGRITY
    if min > lowest{
      panic!("min value cannot be higher than lowest value");
    }
    if max < highest {
      panic!("max value cannot be lower than highest value");
    }

    return Self {close, open, min, max}
  }
}