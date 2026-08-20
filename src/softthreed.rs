use std::f64::consts::PI;

use ndarray::{Array2, Array3};
use ndrustfft::FftHandler;
use num_complex::Complex64;


pub struct SoftSimulator3D{
    pub state: Array3<Complex64>,
    pub half_potential_op: Array3<Complex64>,
    pub kinetic_op: Array3<Complex64>,
    pub temp: Array3<Complex64>,
    fft_handler_x: FftHandler<f64>,
    fft_handler_y: FftHandler<f64>,
    fft_handler_z: FftHandler<f64>,
}

impl SoftSimulator3D{
    pub fn new(size: (usize, usize, usize), length: (f64, f64, f64), dt: f64, potential_func: impl Fn(f64, f64, f64) -> f64) -> Self{
        let (nx, ny, nz) = size;
        let (length_x, length_y, length_z) = length;

        let state = Array3::<Complex64>::zeros((nx, ny, nz));
        let mut half_potential_op = Array3::<Complex64>::zeros((nx, ny, nz));
        let mut kinetic_op = Array3::<Complex64>::zeros((nx, ny, nz));
        let temp = Array3::<Complex64>::zeros((nx, ny, nz));

        let dx = length_x / (nx as f64);
        let dy = length_y / (ny as f64);
        let dz = length_z / (nz as f64);
        let dpx = (2.0 * PI) / length_x;
        let dpy = (2.0 * PI) / length_y;
        let dpz = (2.0 * PI) / length_z;

        for i in 0..nx {
            for j in 0..ny {
                for k in 0..nz {
                    let x = (i as f64) * dx - (length_x);
                    let y = (j as f64) * dy - (length_y);
                    let z = (k as f64) * dz - (length_z);
                    let v = potential_func(x, y, z);

                    let phase_v = -v * (dt/2.0);
                    half_potential_op[[i,j,k]] = Complex64::new(0.0, phase_v).exp();

                    let px = if i < nx / 2 {
                        (i as f64) * dpx
                    } else {
                        ((i as f64) - (nx as f64)) * dpx
                    };
                    let py = if j < ny / 2 {
                        (j as f64) * dpy
                    } else {
                        ((j as f64) - (ny as f64)) * dpy
                    };
                    let pz = if k < nz / 2 {
                        (z as f64) * dpz
                    } else {
                        (( k as f64) - (ny as f64)) * dpz
                    };

                    let t = (px * px + py * py + pz * pz) / 2.0;
                    let phase_t = -t * dt;

                    kinetic_op[[i, j, k]] = Complex64::new(0.0, phase_t).exp();
                }
            }
        }

        let fft_handler_x = FftHandler::new(nx);
        let fft_handler_y = FftHandler::new(ny);
        let fft_handler_z = FftHandler::new(nz);
        Self {
            state,
            temp,
            half_potential_op,
            kinetic_op,
            fft_handler_x,
            fft_handler_y,
            fft_handler_z,
        }
    }
}
