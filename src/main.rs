mod bonsai;

use bonsai::candle;

fn main() {
  let candle: candle = candle::new(5.65, 78.4);

  println!("{candle}");
}