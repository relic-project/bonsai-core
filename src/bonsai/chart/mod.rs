use crate::bonsai::candle;
use crate::bonsai::timeframe;

pub struct Chart {
  pub title: String,
  pub candles: Vec<candle::Candle>,
  pub timeframe: timeframe::Timeframe
}

impl Default for Chart {
  fn default() -> Self {
    Self {
      title: "Default title".to_string(),
      candles: vec![],
      timeframe: timeframe::Timeframe::S1
    }
  }
}

impl std::string::ToString for Chart {
  fn to_string(&self) -> String {
    let mut display_candle: String = String::new();
    for candle in &self.candles {
      display_candle.push_str(&candle.to_string());
    }
    format!("title: {} [{}],\ncandles: {}", self.title, self.timeframe.to_string(), display_candle)
  }
}

impl Chart {
  pub fn new(title: String, timeframe: timeframe::Timeframe) -> Self {
    return Self {title, candles: vec!(), timeframe }
  }

  pub fn add(&mut self, candle: candle::Candle) -> &mut Self {
    self.candles.push(candle);
    self
  }

  pub fn add_bulk(&mut self, candles: Vec<candle::Candle> ) -> &mut Self {

    // candles.iter() returns &Candle
    for candle in candles {
      self.add(candle);
    }

    self
  }
}