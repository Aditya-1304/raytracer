use std::env;
use std::fs::File;
use std::io::{self, Write};
use color::{Color, write_color};

mod vec3;
mod color;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "image.ppm"
    };

    let image_width  = 256;
    let image_height  = 256;

    let mut file = File::create(filename)?;

    writeln!(file, "P3")?;
    writeln!(file,"{} {}", image_width, image_height)?;
    writeln!(file,"255")?;

   for j in 0..image_height {
    eprint!("\rScanlines remaining: {}", image_height - j);
    io::stderr().flush()?;
        for i in 0..image_width {
  
            let pixel_color = Color::from_values(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0
            );
            write_color(&mut file, &pixel_color)?;
        }
    }
    eprintln!("\rDone.                 ");
    println!("Image saved to {}", filename);
    Ok(())
}
