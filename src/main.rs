use std::process::{Command, Stdio};
use clap::Parser;

mod softoned;
mod render;
mod softtwod;
mod softthreed;

use render::render_1d;
use softoned::SoftSimulator1D;

use softtwod::SoftSimulator2D;

use crate::render::render_2d;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Dimension to simulate: 1 or 2
    #[arg(short, long, default_value_t = 1)]
    dim: u8,
    
    /// Number of iterations
    #[arg(short, long, default_value_t = 2000)]
    iterations: usize,
    
    /// Output file name
    #[arg(short, long, default_value = "quantum_wave.mp4")]
    output: String,
}

fn spawn_ffmpeg(width: u32, height: u32, output: &str) -> std::process::Child {
    Command::new("ffmpeg")
        .args(&[
            "-y",
            "-f", "rawvideo",
            "-vcodec", "rawvideo",
            "-s", &format!("{}x{}", width, height),
            "-pix_fmt", "rgb24",
            "-r", "30",
            "-i", "-",
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            output,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn ffmpeg. Ensure ffmpeg is installed.")
}

fn main() {
    let cli = Cli::parse();
    let dt = 0.01;

    if cli.dim == 1 {
        let grid_points = 1 << 12; // Adjusted for speed while rendering
        let grid_length = 40.0;
        
        let potential = |x: f64| -> f64 {
            x*x * 0.5
        };

        let mut sim = SoftSimulator1D::new(grid_points, grid_length, dt, potential);
        sim.init_gaussian(-10.0, 4.0, 1.0, grid_length);
        
        println!("Starting 1D Simulation. Outputting to {}", cli.output);
        
        let width = 800;
        let height = 600;
        let mut ffmpeg = spawn_ffmpeg(width, height, &cli.output);
        let mut stdin = ffmpeg.stdin.take().unwrap();
        let mut frame_buf = vec![0u8; (width * height * 3) as usize];
        
        for i in 0..cli.iterations {
            sim.step_forward();
            if i % 10 == 0 {
                render_1d(&sim, &mut stdin, &mut frame_buf, width, height, grid_length);
            }
        }
        
        drop(stdin);
        ffmpeg.wait().unwrap();
        println!("1D Simulation Complete!");
        
    } else if cli.dim == 2 {
        let nx = 512;
        let ny = 512;
        let length_x = 40.0;
        let length_y = 40.0;
        
        let potential = |x: f64, y: f64| -> f64 {
            // A slit / wall in the middle
            if x > 2.0 && x < 4.0 {
                if y < -2.0 || y > 2.0 { 100.0 } else { 0.0 } // slit
            } else { 
                0.0 
            }
        };

        let mut sim = SoftSimulator2D::new(nx, ny, length_x, length_y, dt, potential);
        // Fire electron towards the slit
        sim.init_gaussian(-10.0, 0.0, 8.0, 0.0, 1.0, length_x, length_y);
        
        println!("Starting 2D Simulation. Outputting to {}", cli.output);
        
        let mut ffmpeg = spawn_ffmpeg(nx as u32, ny as u32, &cli.output);
        let mut stdin = ffmpeg.stdin.take().unwrap();
        let mut frame_buf = vec![0u8; nx * ny * 3];
        
        for i in 0..cli.iterations {
            sim.step_forward();
            if i % 10 == 0 {
                render_2d(&sim, &mut stdin, &mut frame_buf, nx, ny);
            }
        }
        
        drop(stdin);
        ffmpeg.wait().unwrap();
        println!("2D Simulation Complete!");
        

    } else {
        eprintln!("Unsupported dimension: {}", cli.dim);
    }
}
