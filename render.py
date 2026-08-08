import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation

# 1. Load the massive CSV file into memory
print("Loading data...")
df = pd.read_csv("wave.csv", names=["frame", "x", "prob"])
frames = df["frame"].unique()

# 2. Setup the Canvas
fig, ax = plt.subplots(figsize=(10, 6))
fig.canvas.manager.set_window_title('Quantum Split-Operator Engine')

# Set the grid limits (matches your Rust length of 40.0, from -20 to 20)
ax.set_xlim(-20, 20)
ax.set_ylim(0, 0.5)
ax.set_xlabel("Position (x)")
ax.set_ylabel("Probability Density |ψ|^2")
ax.set_title("Time-Dependent Schrödinger Equation")
ax.grid(True, linestyle='--', alpha=0.6)

# Initialize an empty line
line, = ax.plot([], [], lw=2, color='cyan')

# Customize the background for that dark terminal aesthetic
ax.set_facecolor('#1e1e2e')
fig.patch.set_facecolor('#1e1e2e')
ax.xaxis.label.set_color('white')
ax.yaxis.label.set_color('white')
ax.title.set_color('white')
ax.tick_params(colors='white')

# 3. The Animation Function


def init():
    line.set_data([], [])
    return line,


def update(frame):
    # Filter the dataframe for only the current frame's data
    data = df[df["frame"] == frame]
    line.set_data(data["x"], data["prob"])
    return line,


print("Rendering animation...")
# Blit=True makes the rendering significantly faster by only redrawing the moving parts
ani = FuncAnimation(fig, update, frames=frames,
                    init_func=init, blit=True, interval=20)

plt.show()
