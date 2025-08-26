use std::{ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub}};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
  pub e: [f64; 3],
}

impl Vec3 {
  pub fn new() -> Self {
    Vec3 { e: [0.0, 0.0, 0.0] }
  }

  pub fn from_values(e0: f64, e1: f64, e2: f64) -> Self {
    Vec3 { e: [e0, e1, e2] }
  }

  pub fn x(&self) -> f64 {
    self.e[0]
  }

  pub fn y(&self) -> f64 {
    self.e[1]
  }

  pub fn z(&self) -> f64 {
    self.e[2]
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
  }
} 


pub type Point3 = Vec3;

impl Neg for Vec3 {
  type Output = Vec3;

  fn neg(self) -> Self::Output {
    Vec3::from_values(-self.e[0], -self.e[1], -self.e[2])
  }
}

impl Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, index: usize) -> &Self::Output {
    &self.e[index]
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
      &mut self.e[index]
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, v: Vec3) {
      self.e[0] += v.e[0];
      self.e[1] += v.e[1];
      self.e[2] += v.e[2];
  }
}

impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, rhs: f64) {
    self.e[0] *= rhs;
    self.e[1] *= rhs;
    self.e[2] *= rhs;
  }
}

impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, rhs: f64) {
    *self *= 1.0 / rhs;
  }
}

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, v: Vec3) -> Self::Output {
    Vec3::from_values(
      self.e[0] + v.e[0],
      self.e[1] + v.e[1],
      self.e[2] + v.e[2],
    )
  }
}

impl Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, v: Vec3) -> Self::Output {
      Vec3::from_values(
        self.e[0] - v.e[0],
        self.e[1] - v.e[1],
        self.e[2] - v.e[2],
      )
  }
}

impl Mul for Vec3 {
  type Output = Vec3;

  fn mul(self, v: Vec3) -> Self::Output {
      Vec3::from_values(
        self.e[0] * v.e[0],
        self.e[1] * v.e[1],
        self.e[2] * v.e[2],
      )
  }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::from_values(
          self * v.e[0],
          self * v.e[1],
          self * v.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, t: f64) -> Self::Output {
    (1.0 / t) * self
  }
}

impl fmt::Display for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{} {} {}", self.e[0] , self.e[1], self.e[2])
  }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
  u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
  Vec3::from_values(
    u.e[1] * v.e[2] - u.e[2] * v.e[1],
    u.e[2] * v.e[0] - u.e[0] * v.e[2],
    u.e[0] * v.e[1] - u.e[1] * v.e[0],
  )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
  *v / v.length()
}