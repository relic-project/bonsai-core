mod bonsai;

use bonsai::candle;
use bonsai::chart;

fn main() {
  let candlestick: candle::Candle = candle::Candle::new(0.5, 0.6, 0.4, 0.7);
  let candlestick2: candle::Candle = candle::Candle::default();
  
  let chart: chart::Chart = chart::Chart::new(String::from("BTC/USDT"), vec![]);

  println!("{}", candlestick);
  println!("{}", candlestick2);
  println!("{}", chart);
}