pub mod candle {
  struct Candle {
    open: f64,
    close: f64
  }

  impl Candle {
    pub fn new(open: f64, close: f64) -> Self {
      Self { open, close }
    }
  }
}