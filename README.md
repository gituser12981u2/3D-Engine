# 3D Engine

This project is not a typical game engine, but an ambitious attempt to create a highly realistic, physics-driven universe simulation from the quantum scale to the cosmic scale. It aims to simulate the entire process of cosmic evolution, from the Big Bang to the formation of complex civilizations.

## Project Vision

- Create a vast, procedurally generated universe with unprecedented physics accuracy, in both the creation of the universe and the live universe.
- Simulate everything from quantum fluctuations in the early universe to the rise and fall of galactic civilizations, all based on fundamental physical principles
- Develop complex economic models for resource distribution and trade
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
