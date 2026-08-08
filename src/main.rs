use num_complex::Complex64;
use rustfft::{Fft, FftPlanner};
use std::f64::consts::PI;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use std::fs::{self, OpenOptions};


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
            // We center the grid at x = 0 (from -L/2 to +L/2) and calculates the potential energy
            // at every point so we dont have to calculate it every time we loop
            let x = (j as f64) * dx - (length / 2.0);
            let v = potential_func(x);
            let phase_v = -v * (dt / 2.0); // exp(-i * V * dt / 2) represesnts the first and third
                                           // termen of the split method
            half_potential_op[j] = Complex64::new(0.0, phase_v).exp();

            // 2. Map Momentum & Calculate Kinetic Vector
            // CRITICAL: We map the first index to represents a sitting electron, then the first
            // half to represents the electrons going to the right in a decreasing order and the
            // second half representing the electron going to the left in a decreasing order
            let p = if j < n / 2 {
                (j as f64) * dp
            } else {
                ((j as f64) - (n as f64)) * dp
            };
            
            let t = (p * p) / 2.0; // Kinetic Energy: p^2 / 2m
            let phase_t = -t * dt; // exp(-i * T * dt)
            
            // We must divide by N to keep the physics accurate.
            // By baking the (1.0 / N) scaling directly into the static kinetic operator here so we
            // dont have to do division during the loop
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

    /// Injects a moving electron (Gaussian wave packet) onto the grid
    pub fn init_gaussian(&mut self, x0: f64, p0: f64, sigma: f64, length: f64) {
        let n = self.state.len();
        let dx = length / (n as f64);
        let mut norm = 0.0;
        
        for j in 0..n {
            let x = (j as f64) * dx - (length / 2.0);
            
            // Envelope: exp(-(x-x0)^2 / (2 * sigma^2))
            let envelope = (-(x - x0).powi(2) / (2.0 * sigma.powi(2))).exp();
            
            // Phase: exp(i * p0 * x)
            let phase = p0 * x; 
            
            self.state[j] = Complex64::new(envelope * phase.cos(), envelope * phase.sin());
            norm += self.state[j].norm_sqr() * dx;
        }

        // Normalize the wave function so total probability = 1.0
        let norm_factor = norm.sqrt();
        for j in 0..n {
            self.state[j] /= norm_factor;
        }
    }

    /// The core physics engine. Advances the universe by one dt.
    pub fn step_forward(&mut self) {

        // Step 1: Position Space (Half-Potential)
        self.state
            .iter_mut()
            .zip(self.half_potential_op.iter())
            .for_each(|(s, p)| *s *= p);        

        // Step 2: The Jump to Momentum Space
        self.forward_fft.process(&mut self.state);

        // Step 3: Momentum Space (Full-Kinetic + Implicit Normalization)
        self.state
            .iter_mut()
            .zip(self.kinetic_op.iter())
            .for_each(|(s, k)| {*s *= k;});


        // Step 4: The Return to Position Space
        self.inverse_fft.process(&mut self.state);

        // Step 5: Position Space (Final Half-Potential)
        self.state
            .iter_mut()
            .zip(self.half_potential_op.iter())
            .for_each(|(s, p)| *s *= p);
    }
}

fn main() {
    let grid_points = 2048; // Must be a power of 2 for maximum FFT speed
    let grid_length = 40.0;
    let dt = 0.01;
    let iterations = 1000;

    // Define a harmonic oscillator potential: 0.5 * k * x^2
    // (You can change this to a wall step to watch quantum tunneling)
    let potential = |x: f64| -> f64 {
        0.5 * 0.1 * x.powi(2) 
    };

    // Initialize the engine
    let mut sim = SoftSimulator1D::new(grid_points, grid_length, dt, potential);

    // Fire an electron starting at x = -5.0, moving right with momentum p = 2.0
    sim.init_gaussian(-5.0, 2.0, 1.0, grid_length);

    println!("Engine initialized. Starting time evolution...");

    let fd = fs::OpenOptions::new()
        .write(true)
        .open("wave.csv").expect("Couldnt open the file!");
    let mut writer = BufWriter::new(fd);
    // ---- THE EXECUTION LOOP ----
    for i in 0..iterations {
        sim.step_forward();

        // Save a frame every 10 iterations to keep the file size manageable
        if i % 10 == 0 {
            let dx = grid_length / (grid_points as f64);
                
            for j in 0..grid_points {
                let x = (j as f64) * dx - (grid_length / 2.0);
                
                // We only care about the probability density (amplitude squared) for the graph
                let prob = sim.state[j].norm_sqr(); 
                    
                // Write format: frame_number, x_coordinate, probability
                writeln!(writer, "{},{},{}", i, x, prob).unwrap();
            }
        }
    } 
}
