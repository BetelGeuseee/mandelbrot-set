# Mandelbrot Set Visualization

This project generates and visualizes the Mandelbrot Set in your terminal using ASCII characters and colors. It leverages the Rust programming language and the `termcolor` crate for colorful output.

## Features
- Visualizes the Mandelbrot set in the terminal.
- Color-coded escape values to make the visualization more vivid and intuitive.
- Easy-to-configure resolution and boundaries.

## Example Output
![Screenshot from 2025-01-01 20-59-25](https://github.com/user-attachments/assets/0cf6af86-7e7b-4f4f-878f-01c5d87d1172)

## How It Works
1. **Complex Plane Mapping**:
   - The complex plane is divided into a grid based on the provided boundaries (`x_min`, `x_max`, `y_min`, `y_max`) and resolution (`width`, `height`).
2. **Iteration**:
   - For each grid point \((cx, cy)\), the function checks whether the corresponding complex number is bounded (part of the Mandelbrot set) by iterating the formula:
     \[ z_{n+1} = z_n^2 + c \]
     until either the magnitude \(|z_n|\) exceeds 2 or the maximum number of iterations is reached.
3. **Color Mapping**:
   - Points that escape the set are assigned a color and symbol based on the number of iterations taken to escape.

## Installation
1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/your-username/mandelbrot-set-visualization.git
   cd mandelbrot-set-visualization
   ```
3. Install dependencies:
   ```bash
   cargo build
   ```

## Usage
Run the program with:
```bash
cargo run
```

## Configuration
You can modify the following parameters in the `main` function:
- **Boundaries**:
  ```rust
  let mandelbrot = mandelbrot_point(1000, -2.0, 1.0, -1.0, 1.0, 150, 50);
  ```
  - `x_min` and `x_max`: Horizontal range of the complex plane.
  - `y_min` and `y_max`: Vertical range of the complex plane.
- **Resolution**:
  - `width` and `height`: Number of grid points in the horizontal and vertical directions.
- **Iterations**:
  - `max_iters`: Maximum number of iterations to determine boundedness.

## Dependencies
This project uses the following crate:
- [`termcolor`](https://docs.rs/termcolor/1.2.0/termcolor/): For terminal color output.

To add the dependency, include this in your `Cargo.toml`:
```toml
[dependencies]
termcolor = "1.2"
```
