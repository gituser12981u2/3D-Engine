# 3D Engine

This project s not a typical game engine, but a specialized 3D simulation engine designed for creating a highly realistic, physics-driven universe simulation. It draws inspiration from games like No Man's Sky, but aims to provide a much more realistic and detailed simulation for physics, economics, and civilization development.

## Project Goals

- Create a vast, procedurally generated universe with realistic celestial mechanics, planetary development, and object interactions
- Develop complex economic models for resource distribution and trade
- Simulate the rise and fall fo civilizations based on environmental and societal factors.
- Provide a platform for exploring theoretical models of space colonization, long-term civilization development, the underlying physics of the cosmos

## Key Features

- Efficient rendering using Metal API (current focus on macOS)
- Realistic physics simulation including n-body gravity, relativistic effects, and fluid dynamics.
- High fidelity procedural generation of galaxies, star system, planets, and biomes that have physics to back up every aspect of their being
- Advanced AI for simulating civilization behaviors and decision-making
- Modular architecture for easy extensibility and scientific model integration

## Prerequisites

- Rust (latest stable version)
- Metal-compatible GPU

## Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/gituser12981u2/3d-engine.git
    cd 3D-engine
    ```

2. Build the project:

    ```bash
    cargo build
    ```

## Usage

Here is a basic example of how to use the engine:

```rust
use 3D_engine::{Renderer, Color, RenderVector3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = Renderer::new(800, 600, "My 3D Scene")?;

    renderer.set_render_callback(|r| {
        r.draw_triangle(
            RenderVector3::new(-0.5, -0.5, 0.0),
            RenderVector3::new(0.5, -0.5, 0.0),
            RenderVector3::new(0.0, 0.5, 0.0),
            Color::new(1.0, 0.0, 0.0, 1.0)
        )?;
        Ok(())
    });

    renderer.run()
}
```

## Contributing

Contributions are encouraged from scientists, developers, and enthusiasts. Please read the [Contributing Guide](CONTRIBUTING.md) for more details.

## License

This project is licensed under the MIT License--see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The Rust community for their excellent documentation and crates
- Apple for the Metal API
- Claude AI for excellent troubleshooting and debugging help
