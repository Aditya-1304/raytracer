use rand::Rng;
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}

pub fn random_float() -> f64 {
  let mut rng = rand::rng();
  rng.random::<f64>()
}

pub fn random_float_range(min: f64, max: f64) -> f64 {
  min + (max - min) * random_float()
}

pub fn random_float_range_direct(min: f64, max: f64) -> f64 {
  let mut rng = rand::rng();
  rng.random_range(min..max)
}