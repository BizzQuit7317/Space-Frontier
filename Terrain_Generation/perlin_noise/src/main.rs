use image::{ImageBuffer, RgbImage, Rgb};
use noise::{Perlin, NoiseFn};

fn main() {
    let width = 1024;
    let height = 512;
    let scale = 0.01;
    let seed = 1;

    let perlin = Perlin::new(seed);

    let mut img: RgbImage = ImageBuffer::from_pixel(width, height, Rgb([0, 0, 0]));

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;

            let value = perlin.get([nx, ny]);
            let normalized = (value + 1.0) / 2.0;

            let pixel = if normalized < 0.3 {
                Rgb([0, 0, 128])       // Deep water (dark blue)
            } else if normalized < 0.4 {
                Rgb([0, 105, 148])     // Shore (light blue)
            } else if normalized < 0.6 {
                Rgb([34, 139, 34])     // Grass (green)
            } else if normalized < 0.8 {
                Rgb([139, 69, 19])     // Mountain (brown)
            } else {
                Rgb([255, 255, 255])   // Snow
            };
            
            img.put_pixel(x, y, pixel);
        }
    }

    img.save("terrain.png").expect("Failed to save image! ")
}
