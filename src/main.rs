use image::{ImageBuffer, RgbImage};
use num::complex::Complex;
use std::cmp::min;
use std::env;
// use rand::Rng;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;

fn mandelbrot_color_at_point(cx: f64, cy: f64, max_iters: usize) -> image::Rgb<u8> {
  let mut z = Complex { re: 0.0, im: 0.0 };
  let c = Complex::new(cx, cy);

  for i in 0..=max_iters {
      if z.norm() > 2.0 {
          let mut v: f64 = (i as f64 + 1.5 - (z.norm() as f64).abs().log10().log2()).log10() / 3.4;
          if v < 1.0 {
              return image::Rgb([
                  min(255, (255.0 * v.powf(4.0)).round() as u8),
                  min(255, (255.0 * v.powf(2.5)).round() as u8),
                  min(255, (255.0 * v).round() as u8),
                ]);
          } else {
              v = (2.0 - v).max(0.0);
              return image::Rgb([
                  min(255, (255.0 * v).round() as u8),
                  min(255, (255.0 * v.powf(1.5)).round() as u8),
                  min(255, (255.0 * v.powf(3.0)).round() as u8),
              ]);
          }
      }
      z = z * z + c;
  }

  return image::Rgb([0, 0, 0])
}

fn mandelbrot_at_point(cx: f64, cy: f64,max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
      if z.norm() > 2.0 {
        return i;
      }
      z = z * z + c;
    }
    return max_iters
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut image: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let mandl_x_min: f64 = -2.1;
    let mandl_x_max: f64 = 0.8;
    let mandl_y_min: f64 = ((mandl_x_min - mandl_x_max) * 0.5 * HEIGHT as f64) / HEIGHT as f64;
    let mandl_y_max: f64 = (0.0 - mandl_y_min as f64) + 0.01;
    for cur_y in 0..HEIGHT{
      for cur_x in 0..WIDTH{
        let x_percent = cur_x as f64 / WIDTH as f64;
        let y_percent = cur_y as f64 / HEIGHT as f64;
        let cx = mandl_x_min + (mandl_x_max - mandl_x_min) * x_percent;
        let cy = mandl_y_min + (mandl_y_max - mandl_y_min) * y_percent;
        let rgb = mandelbrot_color_at_point(cx as f64 , cy as f64, 500);
        println!("{:?}", rgb);
        *image.get_pixel_mut(cur_x as u32, cur_y as u32) = rgb;
      }
    }

    image.save("fractal.png").unwrap()
}