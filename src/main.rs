use std::io::{self, Write};

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod interval;
mod camera;
mod material;
mod bvh;
mod scenes;

fn main() -> io::Result<()> {
    display_welcome();
    
    loop {
        display_menu();
        
        print!("Enter your choice: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => {
                println!("Rendering Original Scene...");
                if let Err(e) = scenes::create_original_scene() {
                    eprintln!(" Error rendering scene: {}", e);
                }
            }
            "2" => {
                println!("Rendering Spiral Galaxy...");
                if let Err(e) = scenes::create_spiral_scene() {
                    eprintln!(" Error rendering scene: {}", e);
                }
            }
            "3" => {
                println!("Rendering Crystal Cave...");
                if let Err(e) = scenes::create_crystal_scene() {
                    eprintln!(" Error rendering scene: {}", e);
                }
            }
            "4" => {
                println!("Rendering Planetary Rings...");
                if let Err(e) = scenes::create_planetary_rings_scene() {
                    eprintln!(" Error rendering scene: {}", e);
                }
            }
            "5" => {
                println!("Rendering Bubble Garden...");
                if let Err(e) = scenes::create_bubble_garden_scene() {
                    eprintln!(" Error rendering scene: {}", e);
                }
            }
            "6" => {
                println!("Rendering Enhanced Spiral Galaxy...");
                if let Err(e) = scenes::create_enhanced_spiral_scene() {
                    eprintln!(" Error rendering scene: {}", e);
                }
            }
            "7" => {
                println!("Rendering Enhanced Crystal Cave...");
                if let Err(e) = scenes::create_enhanced_crystal_scene() {
                    eprintln!(" Error rendering scene: {}", e);
                }
            }
            "8" => {
                println!("Rendering Enhanced Planetary Rings...");
                if let Err(e) = scenes::create_enhanced_rings_scene() {
                    eprintln!(" Error rendering scene: {}", e);
                }
            }
            "q" | "Q" | "quit" | "exit" => {
                println!("Exiting!!");
                break;
            }
            "" => {
                println!("Rendering Original Scene (default)...");
                if let Err(e) = scenes::create_original_scene() {
                    eprintln!(" Error rendering scene: {}", e);
                }
            }
            _ => {
                println!(" Choose from the Menu Nigga!");
                continue;
            }
        }
        
        println!("\n{}", "=".repeat(50));
        print!("Press Enter to continue or 'q' to quit: ");
        io::stdout().flush()?;
        
        let mut continue_input = String::new();
        io::stdin().read_line(&mut continue_input)?;
        
        if continue_input.trim().to_lowercase() == "q" {
            println!("Fuck off!! ");
            break;
        }
    }
    
    Ok(())
}

fn display_welcome() {
    println!("RAY TRACER IN RUST");
    println!("Choose a scene to render");
    
}

fn display_menu() {
    println!("\n AVAILABLE SCENES:");
    println!("1. Original");
    println!("2. Spiral Galaxy");
    println!("3. Crystal Cave");
    println!("4. Planetary Rings");
    println!("5. Bubble Garden");
    println!("6. Enhanced Spiral");
    println!("7. Enhanced Crystal");
    println!("8. Enhanced Rings");
    println!("  ");
    println!("  ");
    println!("  => Press Enter (default) for Original Scene");
    println!("  => Type 'q' to quit");
    println!();
}