#  RustyTracer

A high-performance ray tracer built in Rust featuring 8 stunning preset scenes, BVH acceleration, and parallel processing for fast renders.

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Version](https://img.shields.io/badge/version-0.1.0-green.svg?style=for-the-badge)

</div>

##  Features

-  **Blazing Fast**: BVH acceleration + Rayon parallelization
-  **8 Preset Scenes**: From classic to ultra-high quality renders
-  **Advanced Materials**: Realistic glass, metal, and diffuse surfaces
-  **Interactive Menu**: User-friendly scene selection interface
-  **High Resolution**: Up to 1400px with 300+ samples per pixel
-  **Parallel Processing**: Multi-threaded rendering for maximum performance
-  **Optimized**: 10x+ faster rendering compared to single-threaded implementations


##  Quick Start

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Installation & Usage

```bash
cargo install rustytracer

#then run
raytracer
```

Or you can run from source

```bash
# Clone the repository
git clone https://github.com/Aditya-1304/raytracer.git
cd raytracer

# Run with optimizations (strongly recommended!)
cargo run --release

# Follow the interactive menu to select and render scenes
```

##  Technical Implementation

### Core Features
- **BVH Acceleration**: Spatial partitioning reduces intersection tests from O(n) to O(log n)
- **Parallel Processing**: Rayon-powered multi-threading across CPU cores
- **Advanced Materials**:
  - Lambertian (diffuse) surfaces with perfect scattering
  - Metal surfaces with configurable roughness
  - Dielectric materials with realistic refraction (glass, water, crystals)
- **Camera Effects**: Depth of field, anti-aliasing, adjustable field of view
- **Optimized Sampling**: Importance sampling for realistic lighting


### Architecture
```
src/
├── main.rs           # Interactive CLI interface
├── scenes.rs         # Scene definitions and generators  
├── camera.rs         # Camera with depth of field
├── material.rs       # Material implementations
├── bvh.rs           # BVH acceleration structure
├── vec3.rs          # 3D vector math
├── ray.rs           # Ray definition and operations
├── sphere.rs        # Sphere primitive
├── color.rs         # Color handling and gamma correction
└── rtweekend.rs     # Utilities and random number generation
```


##  Usage Tips

### Output
- Images are saved as `.ppm` files in the project directory
- Convert to common formats: `convert image.ppm image.jpg` (ImageMagick)
- Or use online converters for quick sharing

### Customization
- Modify `src/scenes.rs` to create your own scenes
- Adjust camera parameters for different perspectives
- Experiment with material properties for unique effects

##  Contributing

Contributions are welcome! Here are some ideas:

-  **New Scenes**: Create unique ray-traced scenes
-  **Performance**: GPU acceleration, SIMD optimizations  
-  **Features**: New materials, lighting models, primitives
-  **Bug Fixes**: Resolve issues and improve stability
-  **Documentation**: Improve guides and code comments

### Development Setup
```bash
git clone https://github.com/Aditya-1304/raytracer.git
cd raytracer

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy
```

 Acknowledgments

- **Peter Shirley**: ["Ray Tracing in One Weekend"](https://raytracing.github.io/) series
- **Rust Community**: For excellent documentation and crates
- **Graphics Researchers**: Decades of ray tracing innovation

## Inspiration

This project demonstrates the power of Rust for high-performance graphics programming, combining memory safety with zero-cost abstractions to achieve both correctness and speed.

##  License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

##  Contact

**Aditya Mandal**
- Email: mandaladitya1614@gmail.com  
- GitHub: [@Aditya-1304](https://github.com/Aditya-1304)

---

<div align="center">


</div>
