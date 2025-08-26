use std::env;
use std::fs::File;
use std::io::{self, Write};

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
        for i in 0..image_width {
            let red = i as f32 / (image_width - 1) as f32 ;
            let green = j as f32 / (image_height - 1) as f32;
            let blue = 0.0 ;

            let ired = (255.999 * red) as i32;
            let igreen = (255.999 * green) as i32;
            let iblue = (255.999 * blue) as i32;

            writeln!(file,"{} {} {}", ired, igreen, iblue)?;
        }
    }

    println!("Image saved to {}", filename);
    Ok(())
}
