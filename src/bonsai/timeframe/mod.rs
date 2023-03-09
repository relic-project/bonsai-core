use std::fmt;

#[allow(dead_code)]

pub enum TimeframeList {
  S1 = 1,
  S30 = 30,
  M1 = 60,
  M2 = 120,
  M3 = 180,
  M4 = 240,
  M5 = 300,
  M10 = 600,
  M15 = 900,
  M30 = 1_800,
  H1 = 3_600,
  H2 = 7_200,
  H4 = 14_400,
  H6 = 21_600,
  H12 = 43_200,
  D1 = 86_400,
  D3 = 259_200,
  W1 = 604_800,
  W2 = 1_209_600,
  MO1 = 2_419_200,
  Y1 = 29_030_400,
}

pub type Timeframe = TimeframeList;

impl fmt::Display for Timeframe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      TimeframeList::S1 => write!(f, "S1"),
      TimeframeList::S30 => write!(f, "S30"),
      TimeframeList::M1 => write!(f, "M1"),
      TimeframeList::M2 => write!(f, "M2"),
      TimeframeList::M3 => write!(f, "M3"),
      TimeframeList::M4 => write!(f, "M4"),
      TimeframeList::M5 => write!(f, "M5"),
      TimeframeList::M10 => write!(f, "M10"),
      TimeframeList::M15 => write!(f, "M15"),
      TimeframeList::M30 => write!(f, "M30"),
      TimeframeList::H1 => write!(f, "H1"),
      TimeframeList::H2 => write!(f, "H2"),
      TimeframeList::H4 => write!(f, "H4"),
      TimeframeList::H6 => write!(f, "H6"),
      TimeframeList::H12 => write!(f, "H12"),
      TimeframeList::D1 => write!(f, "D1"),
      TimeframeList::D3 => write!(f, "D3"),
      TimeframeList::W1 => write!(f, "W1"),
      TimeframeList::W2 => write!(f, "W2"),
      TimeframeList::MO1 => write!(f, "MO1"),
      TimeframeList::Y1 => write!(f, "Y1"),
    }
  }
}