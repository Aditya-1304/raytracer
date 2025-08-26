
fn main() {
    let image_width  = 256;
    let image_height  = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

   for j in 0..image_height {
        for i in 0..image_width {
            let red = i as f32 / (image_width - 1) as f32 ;
            let green = j as f32 / (image_height - 1) as f32;
            let blue = 0.0 ;

            let ired = (255.999 * red) as i32;
            let igreen = (255.999 * green) as i32;
            let iblue = (255.999 * blue) as i32;

            println!("{} {} {}", ired, igreen, iblue);
        }
    }
}
