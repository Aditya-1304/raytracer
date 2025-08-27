use crate::vec3::Vec3;
use crate::interval::Interval;
use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> io::Result<()> {
  let red = pixel_color.x();
  let green = pixel_color.y();
  let blue = pixel_color.z();

  let intensity = Interval::from_range(0.000, 0.999);
  let red_byte = (256.0 * intensity.clamp(red) ) as i32;
  let green_byte = (256.0 * intensity.clamp(green)) as i32;
  let blue_byte = (256.0 * intensity.clamp(blue)) as i32;

  writeln!(out,"{} {} {}", red_byte, green_byte, blue_byte)
}