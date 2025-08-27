use crate::rtweekend::INFINITY;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
  pub min: f64,
  pub max: f64,
}

impl Interval {
  pub fn new() -> Self {
    Interval {
      min: INFINITY,
      max: -INFINITY,
    }
  }

  pub fn from_range(min: f64, max: f64) -> Self {
    Interval{ min, max}
  }

  pub fn size(&self) -> f64 {
    self.max - self.min
  }

  pub fn contains(&self, x: f64) -> bool {
    self.min <= x && x <= self.max
  }

  pub fn surrounds(&self, x: f64) -> bool {
    self.min < x && x < self.max
  }

  pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
  };

  pub const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
  };


}

impl Default for Interval {
    fn default() -> Self {
        Self::new()
    }
}