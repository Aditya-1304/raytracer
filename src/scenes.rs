use std::fs::File;
use std::io::{self};
use std::sync::Arc;
use crate::vec3::Point3;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::bvh::BVHNode;
use crate::color::Color;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::rtweekend::{random_float, random_float_range};
use crate::vec3::Vec3;

pub fn create_original_scene() -> io::Result<()> {
    let filename = "original_scene.ppm";
    let mut objects: Vec<Arc<dyn crate::hittable::Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.5, 0.5, 0.5)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point3::from_values(
                a as f64 + 0.9 * random_float(),
                0.2,
                b as f64 + 0.9 * random_float()
            );

            if (center - Point3::from_values(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_float_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    objects.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::from_values(0.4, 0.2, 0.1)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::from_values(0.7, 0.6, 0.5), 0.0));
    objects.push(Arc::new(Sphere::new(Point3::from_values(4.0, 1.0, 0.0), 1.0, material3)));

    let world = BVHNode::new(objects);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 800;
    camera.samples_per_pixel = 100;
    camera.max_depth = 30;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::from_values(13.0, 2.0, 3.0);
    camera.lookat = Point3::from_values(0.0, 1.0, 0.0);
    camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    println!(" Original scene saved to {}", filename);
    Ok(())
}

pub fn create_spiral_scene() -> io::Result<()> {
    let filename = "spiral_galaxy.ppm";
    let mut objects: Vec<Arc<dyn crate::hittable::Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.1, 0.1, 0.2)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let center_x = 0.0;
    let center_z = 0.0;
    for i in 0..200 {
        let t = i as f64 * 0.1;
        let radius = t * 0.5;
        let angle = t * 0.8;
        
        let x = center_x + radius * angle.cos();
        let z = center_z + radius * angle.sin();
        let y = 0.2 + (t * 0.1).sin() * 0.3; 
        
        let center = Point3::from_values(x, y, z);
        let sphere_radius = 0.15 + (t * 0.2).sin().abs() * 0.1;
        
        let choose_mat = random_float();
        if choose_mat < 0.4 {

            let albedo = Color::from_values(
                0.5 + (t * 0.3).sin() * 0.5,
                0.5 + (t * 0.5 + 2.0).sin() * 0.5,
                0.8 + (t * 0.7 + 4.0).sin() * 0.2
            );
            let material = Arc::new(Lambertian::new(albedo));
            objects.push(Arc::new(Sphere::new(center, sphere_radius, material)));
        } else if choose_mat < 0.8 {
            let albedo = Color::random_range(0.6, 1.0);
            let fuzz = random_float_range(0.0, 0.3);
            let material = Arc::new(Metal::new(albedo, fuzz));
            objects.push(Arc::new(Sphere::new(center, sphere_radius, material)));
        } else {
            let material = Arc::new(Dielectric::new(1.5));
            objects.push(Arc::new(Sphere::new(center, sphere_radius, material)));
        }
    }

    let central_material = Arc::new(Metal::new(Color::from_values(1.0, 0.8, 0.2), 0.0));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, 1.0, 0.0), 1.5, central_material)));

    let world = BVHNode::new(objects);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 800;
    camera.samples_per_pixel = 100;
    camera.max_depth = 30;

    camera.vfov = 25.0;
    camera.lookfrom = Point3::from_values(15.0, 8.0, 15.0);
    camera.lookat = Point3::from_values(0.0, 1.0, 0.0);
    camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.3;
    camera.focus_dist = 18.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    println!(" Spiral galaxy scene saved to {}", filename);
    Ok(())
}

pub fn create_crystal_scene() -> io::Result<()> {
    let filename = "crystal_cave.ppm";
    let mut objects: Vec<Arc<dyn crate::hittable::Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.05, 0.05, 0.1)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for cluster in 0..8 {
        let cluster_angle = cluster as f64 * std::f64::consts::PI * 2.0 / 8.0;
        let cluster_x = cluster_angle.cos() * 6.0;
        let cluster_z = cluster_angle.sin() * 6.0;
        
        for _crystal in 0..random_float_range(5.0, 12.0) as i32 {
            let offset_x = random_float_range(-1.5, 1.5);
            let offset_z = random_float_range(-1.5, 1.5);
            let height = random_float_range(0.5, 4.0);
            let radius = random_float_range(0.2, 0.8);
            
            let center = Point3::from_values(
                cluster_x + offset_x,
                height,
                cluster_z + offset_z
            );
            
            let crystal_type = random_float();
            if crystal_type < 0.5 {

                let material = Arc::new(Dielectric::new(1.8));
                objects.push(Arc::new(Sphere::new(center, radius, material)));
            } else if crystal_type < 0.8 {

                let albedo = Color::from_values(
                    random_float_range(0.1, 0.3),
                    random_float_range(0.3, 0.8),
                    random_float_range(0.7, 1.0)
                );
                let material = Arc::new(Lambertian::new(albedo));
                objects.push(Arc::new(Sphere::new(center, radius, material)));
            } else {

                let albedo = Color::from_values(0.8, 0.9, 1.0);
                let material = Arc::new(Metal::new(albedo, 0.1));
                objects.push(Arc::new(Sphere::new(center, radius, material)));
            }
        }
    }


    for i in 0..20 {
        let angle = i as f64 * std::f64::consts::PI * 2.0 / 20.0;
        let radius_orbit = 8.0 + random_float_range(-2.0, 2.0);
        let center = Point3::from_values(
            angle.cos() * radius_orbit,
            4.0 + random_float_range(-1.0, 2.0),
            angle.sin() * radius_orbit
        );
        
        let orb_material = Arc::new(Dielectric::new(1.3));
        objects.push(Arc::new(Sphere::new(center, 0.3, orb_material)));
    }

    let world = BVHNode::new(objects);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 800;
    camera.samples_per_pixel = 150;
    camera.max_depth = 35;

    camera.vfov = 30.0;
    camera.lookfrom = Point3::from_values(0.0, 3.0, 12.0);
    camera.lookat = Point3::from_values(0.0, 2.0, 0.0);
    camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.5;
    camera.focus_dist = 12.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    println!(" Crystal cave scene saved to {}", filename);
    Ok(())
}

pub fn create_planetary_rings_scene() -> io::Result<()> {
    let filename = "planetary_rings.ppm";
    let mut objects: Vec<Arc<dyn crate::hittable::Hittable>> = Vec::new();


    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.01, 0.01, 0.05)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));


    let planet_material = Arc::new(Lambertian::new(Color::from_values(0.8, 0.4, 0.2)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, 2.0, 0.0), 2.5, planet_material)));


    let rings = vec![
        (4.5, 0.3),  
        (6.0, 0.4),  
        (8.5, 0.5),
        (11.0, 0.3),
    ];

    for (ring_radius, thickness) in rings {
        let particles_in_ring = ((ring_radius * 8.0) as i32).max(20);
        
        for i in 0..particles_in_ring {
            let angle = i as f64 * std::f64::consts::PI * 2.0 / particles_in_ring as f64;
            let radius_variation = random_float_range(-thickness, thickness);
            let actual_radius = ring_radius + radius_variation;
            
            let x = actual_radius * angle.cos();
            let z = actual_radius * angle.sin();
            let y = 2.0 + random_float_range(-0.2, 0.2);
            
            let center = Point3::from_values(x, y, z);
            let particle_size = random_float_range(0.05, 0.2);
            
            let material_choice = random_float();
            let material: Arc<dyn crate::material::Material> = if material_choice < 0.4 {

                Arc::new(Dielectric::new(1.31))
            } else if material_choice < 0.7 {

                let albedo = Color::from_values(
                    random_float_range(0.3, 0.6),
                    random_float_range(0.2, 0.5),
                    random_float_range(0.1, 0.3)
                );
                Arc::new(Lambertian::new(albedo))
            } else {

                let albedo = Color::random_range(0.5, 0.9);
                Arc::new(Metal::new(albedo, random_float_range(0.0, 0.4)))
            };
            
            objects.push(Arc::new(Sphere::new(center, particle_size, material)));
        }
    }


    let moon_positions = vec![
        Point3::from_values(-15.0, 3.0, 5.0),
        Point3::from_values(12.0, 4.0, -8.0),
        Point3::from_values(8.0, 1.0, 14.0),
    ];
    
    for (i, pos) in moon_positions.iter().enumerate() {
        let moon_material: Arc<dyn crate::material::Material> = if i == 0 {
            Arc::new(Lambertian::new(Color::from_values(0.7, 0.7, 0.8)))
        } else if i == 1 {
            Arc::new(Metal::new(Color::from_values(0.9, 0.8, 0.6), 0.2))
        } else {
            Arc::new(Dielectric::new(1.5))
        };
        
        objects.push(Arc::new(Sphere::new(*pos, 0.8, moon_material)));
    }

    let world = BVHNode::new(objects);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 800;
    camera.samples_per_pixel = 120;
    camera.max_depth = 30;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::from_values(20.0, 8.0, 15.0);
    camera.lookat = Point3::from_values(0.0, 2.0, 0.0);
    camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.2;
    camera.focus_dist = 25.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    println!("✅ Planetary rings scene saved to {}", filename);
    Ok(())
}

pub fn create_bubble_garden_scene() -> io::Result<()> {
    let filename = "bubble_garden.ppm";
    let mut objects: Vec<Arc<dyn crate::hittable::Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.1, 0.3, 0.4)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for _i in 0..50 {
        let x = random_float_range(-10.0, 10.0);
        let z = random_float_range(-10.0, 10.0);
        let height = random_float_range(0.3, 2.0);
        let radius = random_float_range(0.2, 0.6);
        
        let center = Point3::from_values(x, height, z);

        let albedo = Color::from_values(
            random_float_range(0.5, 1.0),
            random_float_range(0.2, 0.8),
            random_float_range(0.3, 0.9)
        );
        
        let material = Arc::new(Lambertian::new(albedo));
        objects.push(Arc::new(Sphere::new(center, radius, material)));
    }


    for column in 0..12 {
        let column_x = (column as f64 - 6.0) * 1.5;
        let column_z = random_float_range(-5.0, 5.0);
        

        for bubble_level in 0..15 {
            let y = bubble_level as f64 * 0.8 + random_float_range(0.0, 0.5);
            let x_offset = (y * 0.3).sin() * 0.5; // Slight wave motion
            let z_offset = (y * 0.2 + column as f64).cos() * 0.3;
            
            let center = Point3::from_values(
                column_x + x_offset,
                y + 1.0,
                column_z + z_offset
            );
            
            let bubble_size = random_float_range(0.1, 0.4);
            
            let material = Arc::new(Dielectric::new(1.33)); 
            objects.push(Arc::new(Sphere::new(center, bubble_size, material)));
        }
    }

    for i in 0..8 {
        let angle = i as f64 * std::f64::consts::PI * 2.0 / 8.0;
        let radius = random_float_range(4.0, 8.0);
        let center = Point3::from_values(
            angle.cos() * radius,
            random_float_range(3.0, 6.0),
            angle.sin() * radius
        );
        
        let jelly_material = Arc::new(Dielectric::new(1.1)); 
        objects.push(Arc::new(Sphere::new(center, 0.8, jelly_material)));
    }

    let world = BVHNode::new(objects);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 800;
    camera.samples_per_pixel = 150;
    camera.max_depth = 40;

    camera.vfov = 35.0;
    camera.lookfrom = Point3::from_values(5.0, 4.0, 8.0);
    camera.lookat = Point3::from_values(0.0, 3.0, 0.0);
    camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.8;
    camera.focus_dist = 10.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    println!(" Bubble garden scene saved to {}", filename);
    Ok(())
}

pub fn create_enhanced_spiral_scene() -> io::Result<()> {
    let filename = "enhanced_spiral.ppm";
    let mut objects: Vec<Arc<dyn crate::hittable::Hittable>> = Vec::new();


    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.02, 0.02, 0.08)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));


    for arm in 0..3 {
        let arm_offset = arm as f64 * std::f64::consts::PI * 2.0 / 3.0;
        
        for i in 0..80 {
            let t = i as f64 * 0.15;
            let radius = t * 0.7 + 2.0; 
            let angle = t * 0.6 + arm_offset;
            
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            let y = 0.3 + (t * 0.2).sin() * 0.4 + random_float_range(-0.1, 0.1);
            
            let center = Point3::from_values(x, y, z);
            let sphere_radius = 0.1 + (t * 0.1).sin().abs() * 0.15;
            
            let distance_factor = radius / 20.0;
            let choose_mat = random_float();
            
            if choose_mat < 0.3 {
                
                let albedo = if distance_factor < 0.5 {
                    Color::from_values(1.0, 0.6 + distance_factor, 0.2)  // 
                } else {
                    Color::from_values(0.3, 0.5 + distance_factor, 1.0)  // 
                };
                let material = Arc::new(Lambertian::new(albedo));
                objects.push(Arc::new(Sphere::new(center, sphere_radius, material)));
            } else if choose_mat < 0.7 {
                let albedo = Color::from_values(
                    0.8 + distance_factor * 0.2,
                    0.9,
                    0.7 + distance_factor * 0.3
                );
                let material = Arc::new(Metal::new(albedo, 0.1));
                objects.push(Arc::new(Sphere::new(center, sphere_radius, material)));
            } else {
                let material = Arc::new(Dielectric::new(1.5));
                objects.push(Arc::new(Sphere::new(center, sphere_radius, material)));
            }
        }
    }

    let central_material = Arc::new(Metal::new(Color::from_values(1.0, 0.9, 0.3), 0.0));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, 1.2, 0.0), 2.0, central_material)));

    for i in 0..5 {
        let angle = i as f64 * std::f64::consts::PI * 2.0 / 5.0;
        let radius = random_float_range(15.0, 25.0);
        let center = Point3::from_values(
            angle.cos() * radius,
            random_float_range(2.0, 8.0),
            angle.sin() * radius
        );
        
        let dust_material = Arc::new(Dielectric::new(1.05)); 
        objects.push(Arc::new(Sphere::new(center, random_float_range(3.0, 8.0), dust_material)));
    }

    let world = BVHNode::new(objects);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;  
    camera.samples_per_pixel = 200;  
    camera.max_depth = 35;

    camera.vfov = 20.0;  
    camera.lookfrom = Point3::from_values(25.0, 15.0, 25.0);
    camera.lookat = Point3::from_values(0.0, 2.0, 0.0);
    camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.1;  
    camera.focus_dist = 30.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    println!(" Enhanced spiral galaxy saved to {}", filename);
    Ok(())
}

pub fn create_enhanced_crystal_scene() -> io::Result<()> {
    let filename = "enhanced_crystal.ppm";
    let mut objects: Vec<Arc<dyn crate::hittable::Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.01, 0.02, 0.05)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for ring in 0..3 {
        let ring_radius = 4.0 + ring as f64 * 3.0;
        let crystals_in_ring = 6 + ring * 2;
        
        for crystal in 0..crystals_in_ring {
            let angle = crystal as f64 * std::f64::consts::PI * 2.0 / crystals_in_ring as f64;
            let x = ring_radius * angle.cos() + random_float_range(-0.5, 0.5);
            let z = ring_radius * angle.sin() + random_float_range(-0.5, 0.5);
            
            let base_height = 1.0 + ring as f64 * 0.5;
            for level in 0..random_float_range(3.0, 8.0) as i32 {
                let y = base_height + level as f64 * 0.6 + random_float_range(-0.2, 0.2);
                let radius = 0.4 - level as f64 * 0.05; 
                if radius > 0.1 {
                    let center = Point3::from_values(x, y, z);
                    
                    let crystal_type = random_float();
                    if crystal_type < 0.4 {
                        let ri = random_float_range(1.3, 2.0);
                        let material = Arc::new(Dielectric::new(ri));
                        objects.push(Arc::new(Sphere::new(center, radius, material)));
                    } else if crystal_type < 0.7 {
                        let hue = angle + ring as f64 * 0.5; 
                        let albedo = Color::from_values(
                            0.2 + (hue).sin().abs() * 0.6,
                            0.3 + (hue + 2.0).sin().abs() * 0.7,
                            0.8 + (hue + 4.0).sin().abs() * 0.2
                        );
                        let material = Arc::new(Lambertian::new(albedo));
                        objects.push(Arc::new(Sphere::new(center, radius, material)));
                    } else {

                        let albedo = Color::from_values(0.9, 0.95, 1.0);
                        let material = Arc::new(Metal::new(albedo, 0.05));
                        objects.push(Arc::new(Sphere::new(center, radius, material)));
                    }
                }
            }
        }
    }

    for _i in 0..30 {
        let angle = random_float() * std::f64::consts::PI * 2.0;
        let radius = random_float_range(8.0, 15.0);
        let center = Point3::from_values(
            angle.cos() * radius,
            random_float_range(6.0, 12.0),
            angle.sin() * radius
        );
        
        let orb_size = random_float_range(0.2, 0.5);
        let orb_material = Arc::new(Dielectric::new(random_float_range(1.1, 1.4)));
        objects.push(Arc::new(Sphere::new(center, orb_size, orb_material)));
    }

    let world = BVHNode::new(objects);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 250; 
    camera.max_depth = 50;  

    camera.vfov = 25.0;
    camera.lookfrom = Point3::from_values(8.0, 6.0, 12.0);
    camera.lookat = Point3::from_values(0.0, 3.0, 0.0);
    camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.8;  
    camera.focus_dist = 15.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    println!(" Enhanced crystal cave saved to {}", filename);
    Ok(())
}

pub fn create_enhanced_rings_scene() -> io::Result<()> {
    let filename = "enhanced_rings.ppm";
    let mut objects: Vec<Arc<dyn crate::hittable::Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Color::from_values(0.005, 0.005, 0.02)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let planet_material = Arc::new(Lambertian::new(Color::from_values(0.7, 0.3, 0.1)));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, 2.0, 0.0), 2.8, planet_material)));
    
    let atmosphere_material = Arc::new(Dielectric::new(1.02));
    objects.push(Arc::new(Sphere::new(Point3::from_values(0.0, 2.0, 0.0), 3.2, atmosphere_material)));


    let rings = vec![
        (4.8, 0.4, 80),   
        (6.5, 0.6, 120),  
        (9.0, 0.8, 150),
        (11.5, 0.3, 80),
        (13.0, 0.5, 100),
    ];

    for (ring_radius, thickness, particle_count) in rings {
        for i in 0..particle_count {
            let angle = i as f64 * std::f64::consts::PI * 2.0 / particle_count as f64;
            let eccentricity = random_float_range(0.95, 1.05);
            let actual_radius = ring_radius * eccentricity + random_float_range(-thickness, thickness);
            
            let x = actual_radius * angle.cos();
            let z = actual_radius * angle.sin();
            let y = 2.0 + random_float_range(-0.1, 0.1); 
            
            let center = Point3::from_values(x, y, z);
            let particle_size = random_float_range(0.03, 0.15);
            
            
            let material: Arc<dyn crate::material::Material> = if particle_size > 0.1 {
            
                let albedo = Color::from_values(
                    random_float_range(0.4, 0.7),
                    random_float_range(0.3, 0.6),
                    random_float_range(0.2, 0.4)
                );
                Arc::new(Lambertian::new(albedo))
            } else if random_float() < 0.6 {
            
                Arc::new(Dielectric::new(1.31))
            } else {
            
                let albedo = Color::random_range(0.6, 0.9);
                Arc::new(Metal::new(albedo, random_float_range(0.1, 0.5)))
            };
            
            objects.push(Arc::new(Sphere::new(center, particle_size, material)));
        }
    }

    
    let moon_data = vec![
        (Point3::from_values(-18.0, 4.0, 6.0), 1.2, Color::from_values(0.8, 0.8, 0.9), "ice"),
        (Point3::from_values(15.0, 3.0, -10.0), 0.8, Color::from_values(0.9, 0.7, 0.5), "metal"),
        (Point3::from_values(10.0, 1.0, 16.0), 1.0, Color::from_values(0.6, 0.4, 0.8), "crystal"),
        (Point3::from_values(-8.0, 6.0, -12.0), 0.6, Color::from_values(1.0, 0.8, 0.3), "metal"),
    ];
    
    for (pos, size, color, mat_type) in moon_data {
        let material: Arc<dyn crate::material::Material> = match mat_type {
            "ice" => Arc::new(Dielectric::new(1.31)),
            "crystal" => Arc::new(Dielectric::new(1.5)),
            "metal" => Arc::new(Metal::new(color, 0.1)),
            _ => Arc::new(Lambertian::new(color)),
        };
        objects.push(Arc::new(Sphere::new(pos, size, material)));
    }

    let world = BVHNode::new(objects);

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1400;  
    camera.samples_per_pixel = 300;
    camera.max_depth = 35;

    camera.vfov = 15.0;  
    camera.lookfrom = Point3::from_values(30.0, 12.0, 20.0);
    camera.lookat = Point3::from_values(0.0, 2.0, 0.0);
    camera.vup = Vec3::from_values(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.05; 
    camera.focus_dist = 35.0;

    let mut file = File::create(filename)?;
    camera.render(&world, &mut file)?;
    println!(" Enhanced planetary rings saved to {}", filename);
    Ok(())
}