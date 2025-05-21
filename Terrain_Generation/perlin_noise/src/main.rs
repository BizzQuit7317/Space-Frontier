use image::{ImageBuffer, RgbImage, Rgb};
use noise::{Perlin, NoiseFn};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.r#gen();;

    let width = 1024;
    let height = 512;
    let scale = 0.01;
    let seed = num;

    let perlin = Perlin::new(seed);

    let mut img: RgbImage = ImageBuffer::from_pixel(width, height, Rgb([0, 0, 0]));

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;

            let value = perlin.get([nx, ny]);
            let normalized = (value + 1.0) / 2.0;

            let pixel = if normalized < 0.3 {
                Rgb([5, 102, 141])       // Deep water (dark blue)
            } else if normalized < 0.4 {
                Rgb([177, 248, 242])     // Shore (light blue)
            } else if normalized < 0.5 {
                Rgb([237, 180, 88])     // Shore (Sand)
            }else if normalized < 0.6 {
                Rgb([21, 113, 69])     // Grass (green)
            } else if normalized < 0.8 {
                Rgb([8, 7, 8])     // Mountain (brown)
            } else {
                Rgb([255, 255, 255])   // Snow
            };
            
            img.put_pixel(x, y, pixel);
        }
    }

    img.save("terrain.png").expect("Failed to save image! ")
}
