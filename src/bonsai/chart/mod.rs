use crate::bonsai::candle;
use std::fmt;

pub struct Chart {
  pub title: String,
  pub candlesticks: Vec<candle::Candle>
}

impl Default for Chart {
  fn default() -> Self {
    Self {
      title: "Default title".to_string(),
      candlesticks: vec![],
    }
  }
}

impl fmt::Display for Chart {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "title: {}", self.title)
  }
}

impl Chart {
  pub fn new(title: String, candlesticks: Vec<candle::Candle>) -> Self {
    return Self {title, candlesticks}
  }
}