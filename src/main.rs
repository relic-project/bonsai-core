mod bonsai;

use bonsai::candle;
use bonsai::chart;
use bonsai::timeframe;

fn main() {
  let candle1 = candle::Candle::new(0.5, 0.6, 0.4, 0.7);
  let candle2 = candle::Candle::default();
  let candle3 = candle::Candle::new(0.6, 0.7, 0.3, 0.9);
  let candle4 = candle::Candle::new(0.7, 0.8, 0.5, 1.0);
  
  let mut chart = chart::Chart::new(String::from("BTC/USDT"), timeframe::Timeframe::D1);

  chart.add(candle1);
  chart.add(candle2);

  println!("{}", chart.to_string());
 
  chart.add_bulk(vec![candle3, candle4]);

  println!("\n{}", chart.to_string());
}