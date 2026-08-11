# 1D & 2D Schrödinger Equation Solver (SOFT Method) ⚛️🦀

A high-performance, time-dependent quantum mechanics engine built from scratch in Rust. This simulator uses the **Split-Operator Fourier Transform (SOFT)** method to model the evolution of a quantum wave packet over time, accurately rendering phenomena like quantum dispersion, tunneling, and wave interference.

## 🚀 Technical Architecture & Optimizations

Simulating quantum mechanics requires extreme computational efficiency. This engine was built with a strict focus on memory management and execution speed:

* **Zero-Allocation Hot Loops:** Standard array iterations were replaced with zipped mutable iterators (`iter_mut().zip()`). This allows the Rust compiler to strip out bounds checking and auto-vectorize the mathematical operations (SIMD) for massive CPU performance gains.
* **Memory-Mapped Physics:** FFT frequency wrapping and complex phase rotations are pre-calculated during initialization. The core physics engine only executes blindingly fast constant multiplications during the time loop.
* **Unitary Preservation:** The underlying mathematical model strictly preserves a total probability density of `1.0`, ensuring physically accurate wave behavior without probability degradation.
* **Integrated Rendering:** The Rust engine directly pipes raw video frames to FFmpeg, removing the need for intermediary CSV files or external Python scripts.

## 🛠️ Prerequisites

To run the simulation and render the visualization, you will need:
* **Rust & Cargo** (for the physics engine)
* **FFmpeg** (to encode the final `.mp4` video)

## 🏃 How to Run the Engine

**Run the Simulation**
Execute the Rust engine in release mode. The engine accepts CLI arguments to specify the dimension, number of iterations, and output file.

```bash
cargo run --release -- --dim 1 --iterations 2000 --output quantum_wave.mp4
```

* `-d, --dim <DIM>`: Dimension to simulate (1 or 2). Default is 1.
* `-i, --iterations <ITERATIONS>`: Number of iterations. Default is 2000.
* `-o, --output <OUTPUT>`: Output file name. Default is `quantum_wave.mp4`.

*Note: This will directly output a `quantum_wave.mp4` (or your chosen output file) in your directory.*

## 🛣️ Roadmap

<<<<<<< HEAD
* **Dimensional Scaling:** Architecting dedicated `SoftSimulator2D` and `SoftSimulator3D` structs to handle multi-dimensional contiguous memory mapping.
* **Asynchronous I/O:** Implementing a Producer-Consumer threading model using `mpsc` channels to completely decouple disk writes from the main physics execution thread.
=======
* **Dimensional Scaling:** Architecting dedicated `SoftSimulator3D` struct to handle 3D memory mapping.
* **Asynchronous I/O:** Implementing a Producer-Consumer threading model using `mpsc` channels to completely decouple ffmpeg pipes from the main physics execution thread.

## 👨‍💻 About the Developer

Built by Șoiman Vasile-Cristian, a Computer Science student at Babeș-Bolyai University. 

I am currently seeking software engineering internships and junior roles, focusing on high-performance computing, systems engineering, and Rust backend architectures.
>>>>>>> 189181c (README)
