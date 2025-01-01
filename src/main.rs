use num::complex::Complex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

fn mandelbrot_point(max_iters: usize,x_min: f64,x_max: f64, y_min: f64,y_max: f64, width: usize, height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    rows
}
fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize, ) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}
fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for row in escape_vals {
        for &column in row.iter() {
            let mut color_spec = ColorSpec::new();

            // Choose color based on escape values
            match column {
                0..=2 => color_spec.set_fg(Some(Color::Black)),
                3..=5 => color_spec.set_fg(Some(Color::Blue)),
                6..=10 => color_spec.set_fg(Some(Color::Cyan)),
                11..=30 => color_spec.set_fg(Some(Color::Green)),
                31..=100 => color_spec.set_fg(Some(Color::Yellow)),
                101..=200 => color_spec.set_fg(Some(Color::Magenta)),
                201..=400 => color_spec.set_fg(Some(Color::Red)),
                401..=700 => color_spec.set_fg(Some(Color::White)).set_bold(true),
                _ => color_spec.set_fg(Some(Color::White)).set_bold(true).set_intense(true),
            };

            stdout.set_color(&color_spec).unwrap();

            // Choose character based on escape values
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            write!(&mut stdout, "{}", val).unwrap();
        }
        // Reset color and print a newline after each row
        stdout.reset().unwrap();
        println!();
    }
}

fn main() {
    let mandelbrot =mandelbrot_point(1000, -2.0, 1.0, -1.0, 1.0, 150, 50);
    render_mandelbrot(mandelbrot);
}