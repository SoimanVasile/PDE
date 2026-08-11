use std::{f64::consts::PI, sync::Arc};

use num_complex::Complex64;
use rustfft::{Fft, FftPlanner};

/// A high-performance 1D Time-Dependent Schrödinger Equation Solver
pub struct SoftSimulator1D {
    pub state: Vec<Complex64>,
    pub half_potential_op: Vec<Complex64>,
    pub kinetic_op: Vec<Complex64>,
    forward_fft: Arc<dyn Fft<f64>>,
    inverse_fft: Arc<dyn Fft<f64>>,
}

impl SoftSimulator1D {
    pub fn new(n: usize, length: f64, dt: f64, potential_func: impl Fn(f64) -> f64) -> Self {
        let mut planner = FftPlanner::new();
        let forward_fft = planner.plan_fft_forward(n);
        let inverse_fft = planner.plan_fft_inverse(n);

        let dx = length / (n as f64);
        let dp = (2.0 * PI) / length;

        let state = vec![Complex64::new(0.0, 0.0); n];
        let mut half_potential_op = vec![Complex64::new(0.0, 0.0); n];
        let mut kinetic_op = vec![Complex64::new(0.0, 0.0); n];

        for j in 0..n {
            let x = (j as f64) * dx - (length / 2.0);
            let v = potential_func(x);
            let phase_v = -v * (dt / 2.0);
            half_potential_op[j] = Complex64::new(0.0, phase_v).exp();

            let p = if j < n / 2 {
                (j as f64) * dp
            } else {
                ((j as f64) - (n as f64)) * dp
            };
            
            let t = (p * p) / 2.0;
            let phase_t = -t * dt;
            
            kinetic_op[j] = Complex64::new(0.0, phase_t).exp() / (n as f64);
        }

        Self {
            state,
            half_potential_op,
            kinetic_op,
            forward_fft,
            inverse_fft,
        }
    }

    pub fn init_gaussian(&mut self, x0: f64, p0: f64, sigma: f64, length: f64) {
        let n = self.state.len();
        let dx = length / (n as f64);
        let mut norm = 0.0;
        
        for j in 0..n {
            let x = (j as f64) * dx - (length / 2.0);
            let envelope = (-(x - x0).powi(2) / (2.0 * sigma.powi(2))).exp();
            let phase = p0 * x; 
            
            self.state[j] = Complex64::new(envelope * phase.cos(), envelope * phase.sin());
            norm += self.state[j].norm_sqr() * dx;
        }

        let norm_factor = norm.sqrt();
        for j in 0..n {
            self.state[j] /= norm_factor;
        }
    }

    pub fn step_forward(&mut self) {
        self.state.iter_mut().zip(self.half_potential_op.iter()).for_each(|(s, p)| *s *= p);        
        self.forward_fft.process(&mut self.state);
        self.state.iter_mut().zip(self.kinetic_op.iter()).for_each(|(s, k)| {*s *= k;});
        self.inverse_fft.process(&mut self.state);
        self.state.iter_mut().zip(self.half_potential_op.iter()).for_each(|(s, p)| *s *= p);
    }
}

