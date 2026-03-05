use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::process::exit;
use std::time::Instant;

use invert_image_colors::{invert_colors, read_ppm, write_ppm};

extern "C" {
    fn launch_invert_colors(pixels: *mut u8, size: i32) -> f32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <input_image_path> <output_dir>", args[0]);
        exit(1);
    }
    let image_path = Path::new(&args[1]);
    if !image_path.is_file() {
        println!("{} is not a file", args[1]);
        exit(1);
    }
    let Ok(file) = File::open(image_path) else {
        println!("Can't not open file {}", args[1]);
        exit(1);
    };

    let output_dir = Path::new(&args[2]);
    if !output_dir.exists() {
        if let Err(e) = fs::create_dir_all(output_dir) {
            println!("Can't not create dir {} {}", args[2], e);
            exit(1);
        }
    }

    let (header, mut cpu_pixels) = read_ppm(file).unwrap_or_else(|e| {
        println!("{e}");
        exit(1);
    });

    let mut gpu_pixels = cpu_pixels.clone();
    let ptr = gpu_pixels.as_mut_ptr();
    let size = gpu_pixels.len() as i32;
    let gpu_duration = unsafe { launch_invert_colors(ptr, size) };

    let cpu_start = Instant::now();
    invert_colors(&mut cpu_pixels);
    let cpu_duration = cpu_start.elapsed();

    let output_file_name = format!(
        "invert_colors_{}",
        args[1]
            .split('/')
            .last()
            .unwrap_or("output")
            .split('.')
            .next()
            .unwrap_or("output")
    );
    let cpu_output_path = output_dir.join(format!("{}_cpu.ppm", output_file_name));
    let gpu_output_path = output_dir.join(format!("{}_gpu.ppm", output_file_name));

    if let Err(e) = write_ppm(cpu_output_path.as_path(), &header, &cpu_pixels) {
        println!("Can't write cpu.ppm {e}");
        exit(1);
    }
    if let Err(e) = write_ppm(gpu_output_path.as_path(), &header, &gpu_pixels) {
        println!("Can't write gpu.ppm {e}");
        exit(1);
    }

    println!("Success invert {} colors", args[1]);
    println!("GPU {:.4} ms", gpu_duration);
    println!("CPU {:.4} ms", cpu_duration.as_secs_f32() * 1e3);
}
