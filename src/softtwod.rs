use ndrustfft::{FftHandler, ndfft, ndifft};
use num_complex::Complex64;
use std::f64::consts::PI;
use ndarray::Array2;


/// A high-performance 2D Time-Dependent Schrödinger Equation Solver
pub struct SoftSimulator2D {
    pub state: Array2<Complex64>,
    pub half_potential_op: Array2<Complex64>,
    pub kinetic_op: Array2<Complex64>,
    pub temp: Array2<Complex64>,
    fft_handler_x: FftHandler<f64>,
    fft_handler_y: FftHandler<f64>,
}

impl SoftSimulator2D {
    pub fn new(nx: usize, ny: usize, length_x: f64, length_y: f64, dt: f64, potential_func: impl Fn(f64, f64) -> f64) -> Self {
        let state = Array2::<Complex64>::zeros((nx, ny));
        let mut half_potential_op = Array2::<Complex64>::zeros((nx, ny));
        let mut kinetic_op = Array2::<Complex64>::zeros((nx, ny));
        let temp = Array2::<Complex64>::zeros((nx, ny));

        let dx = length_x / (nx as f64);
        let dy = length_y / (ny as f64);
        let dpx = (2.0 * PI) / length_x;
        let dpy = (2.0 * PI) / length_y;

        for i in 0..nx {
            for j in 0..ny {
                let x = (i as f64) * dx - (length_x / 2.0);
                let y = (j as f64) * dy - (length_y / 2.0);
                let v = potential_func(x, y);
                let phase_v = -v * (dt / 2.0);
                half_potential_op[[i, j]] = Complex64::new(0.0, phase_v).exp();

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

                let t = (px * px + py * py) / 2.0;
                let phase_t = -t * dt;
               
                kinetic_op[[i, j]] = Complex64::new(0.0, phase_t).exp();
            }
        }

        let fft_handler_x = FftHandler::new(nx);
        let fft_handler_y = FftHandler::new(ny);

        Self {
            state,
            temp,
            half_potential_op,
            kinetic_op,
            fft_handler_x,
            fft_handler_y,
        }
    }

    pub fn init_gaussian(&mut self, x0: f64, y0: f64, px0: f64, py0: f64, sigma: f64, length_x: f64, length_y: f64) {
        let (nx, ny) = self.state.dim();
        let dx = length_x / (nx as f64);
        let dy = length_y / (ny as f64);
        let mut norm = 0.0;
        
        for i in 0..nx {
            for j in 0..ny {
                let x = (i as f64) * dx - (length_x / 2.0);
                let y = (j as f64) * dy - (length_y / 2.0);
                
                let envelope = (-(x - x0).powi(2) / (2.0 * sigma.powi(2)) - (y - y0).powi(2) / (2.0 * sigma.powi(2))).exp();
                let phase = px0 * x + py0 * y; 
                
                self.state[[i, j]] = Complex64::new(envelope * phase.cos(), envelope * phase.sin());
                norm += self.state[[i, j]].norm_sqr() * dx * dy;
            }
        }

        let norm_factor = norm.sqrt();
        for val in self.state.iter_mut() {
            *val /= norm_factor;
        }
    }

    pub fn step_forward(&mut self) {
        ndarray::azip!((s in &mut self.state, &p in &self.half_potential_op) *s *= p);

        ndfft(&self.state, &mut self.temp, &mut self.fft_handler_x, 0);
        ndfft(&self.temp, &mut self.state, &mut self.fft_handler_y, 1);

        for (s, k) in self.state.iter_mut().zip(self.kinetic_op.iter()) {
            *s *= k;
        }

        ndifft(&self.state, &mut self.temp, &mut self.fft_handler_x, 0);
        ndifft(&self.temp, &mut self.state, &mut self.fft_handler_y, 1);

        ndarray::azip!((s in &mut self.state, &p in &self.half_potential_op) *s *= p);
    }
}


