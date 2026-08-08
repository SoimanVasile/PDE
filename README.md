# 1D Schrödinger Equation Solver (SOFT Method) ⚛️🦀

A high-performance, time-dependent quantum mechanics engine built from scratch in Rust. This simulator uses the **Split-Operator Fourier Transform (SOFT)** method to model the evolution of a quantum wave packet over time, accurately rendering phenomena like quantum dispersion, tunneling, and wave interference.

## 🚀 Technical Architecture & Optimizations

Simulating quantum mechanics requires extreme computational efficiency. This engine was built with a strict focus on memory management and execution speed:

*   **Zero-Allocation Hot Loops:** Standard array iterations were replaced with zipped mutable iterators (`iter_mut().zip()`). This allows the Rust compiler to strip out bounds checking and auto-vectorize the mathematical operations (SIMD) for massive CPU performance gains.
*   **Memory-Mapped Physics:** FFT frequency wrapping and complex phase rotations are pre-calculated during initialization. The core physics engine only executes blindingly fast constant multiplications during the time loop.
*   **Unitary Preservation:** The underlying mathematical model strictly preserves a total probability density of `1.0`, ensuring physically accurate wave behavior without probability degradation.
*   **Decoupled Rendering:** The Rust engine acts purely as a physics backend, dumping frame data to disk. A decoupled Python pipeline handles the heavy visualization work, keeping the core physics loop lightweight.

## 🛠️ Prerequisites

To run the simulation and render the visualization, you will need:
*   **Rust & Cargo** (for the physics engine)
*   **Python 3.x** (for the renderer)
*   Python Libraries: `pandas`, `matplotlib`
*   **FFmpeg** (to encode the final `.mp4` video)

## 🏃 How to Run the Engine

**1. Run the Physics Simulation**
Execute the Rust engine in release mode. This will rapidly calculate the quantum states and generate a `wave_frames.csv` file containing the probability density map.
```bash
cargo run --release
