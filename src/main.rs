// altwolff 2025
use image::{Rgb, RgbImage};
use num::Complex;

fn newton(width: i32, height: i32, max_iterations: i32) -> RgbImage {
    let min_x = -2.0;
    let max_x = 2.0;
    let min_y = -2.0;
    let max_y = 2.0;

    let mut img = RgbImage::new(width as u32, height as u32);

    let epsilon = 1e-6;

    // Edit colours:
    let root1_color = Rgb([77, 182, 172]);
    let root2_color = Rgb([255, 138, 101]);
    let root3_color = Rgb([149, 117, 205]);
    let default_color = Rgb([0, 0, 0]);

    for y in 0..height {
        for x in 0..width {
            let real = min_x + (x as f64) * (max_x - min_x) / (width as f64);
            let imag = min_y + (y as f64) * (max_y - min_y) / (height as f64);

            let mut z = Complex::new(real, imag);
            let mut iteration = 0;

            // Newton-Raphson:
            while iteration < max_iterations {
                let f_z = z.powi(3) - Complex::new(1.0, 0.0);
                let f_prime_z = 3.0 * z.powi(2);
                let z_next = z - f_z / f_prime_z;
                if (z_next - z).norm() < epsilon {
                    break;
                }
                z = z_next;
                iteration += 1;
            }

            let color = if (z.re - 1.0).abs() < epsilon && z.im.abs() < epsilon {
                root1_color
            } else if (z.re + 0.5).abs() < epsilon && z.im > 0.0 {
                root2_color
            } else if (z.re + 0.5).abs() < epsilon && z.im < 0.0 {
                root3_color
            } else {
                default_color
            };

            img.put_pixel(x as u32, y as u32, color);
        }
    }

    img
}

fn main() {
    // Edit parameters:
    let width = 800;
    let height = 800;
    let max_iterations = 1000;

    let img = newton(width, height, max_iterations);
    img.save("newton.png").unwrap();
    println!("Image saved as 'newton.png'");
}