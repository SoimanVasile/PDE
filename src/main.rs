use num_complex::Complex64;
use rustfft::{Fft, FftPlanner};
use std::f64::consts::PI;
use std::sync::Arc;

/// A high-performance 1D Time-Dependent Schrödinger Equation Solver
pub struct SoftSimulator1D {
    pub state: Vec<Complex64>,
    pub half_potential_op: Vec<Complex64>,
    pub kinetic_op: Vec<Complex64>,
    forward_fft: Arc<dyn Fft<f64>>,
    inverse_fft: Arc<dyn Fft<f64>>,
}

impl SoftSimulator1D {
    /// Initializes the simulation grid and pre-calculates the execution vectors.
    /// Uses Atomic Units: hbar = 1.0, electron_mass = 1.0
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
            // 1. Map Position & Calculate Potential Vector
            // We center the grid at x = 0 (from -L/2 to +L/2)
            let x = (j as f64) * dx - (length / 2.0);
            let v = potential_func(x);
            let phase_v = -v * (dt / 2.0); // exp(-i * V * dt / 2)
            half_potential_op[j] = Complex64::new(0.0, phase_v).exp();

            // 2. Map Momentum & Calculate Kinetic Vector
            // CRITICAL: We map frequencies to match the FFT's non-linear output order
            let p = if j < n / 2 {
                (j as f64) * dp
            } else {
                ((j as f64) - (n as f64)) * dp
            };
            
            let t = (p * p) / 2.0; // Kinetic Energy: p^2 / 2m
            let phase_t = -t * dt; // exp(-i * T * dt)
            
            // We must divide by N to keep the physics accurate.
            // By baking the (1.0 / N) scaling directly into the static kinetic operator here, 
            // we eliminate an entire O(N) division loop during the time execution!
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

}

fn main() {
}
