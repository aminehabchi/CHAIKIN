# Rust Graphics Demonstrations with `minifb`

This repository showcases three interactive graphical demonstrations built in Rust using the [`minifb`](https://github.com/emoon/rust_minifb) window library.

## Included Demos

### 1. Chaikin's Curve Algorithm Animation
An interactive and animated implementation of Chaikin’s curve smoothing algorithm.

### 2. Radar Animation
A classic radar-style animation featuring a sweeping line and disappearing blips.

### 3. Snake Cursor Follower
A smooth snake-like animation that follows the user's cursor in real-time.

---

## Demonstrations

### Chaikin's Curve Algorithm
![Chaikin's Curve Animation](./chaikin.mp4)

Chaikin's algorithm iteratively smooths a polyline by subdividing line segments, producing a curve-like effect.

**Features:**
- Place control points with **left mouse button**
- Start animation with **Enter**
- Runs through **7 refinement steps**
- Clear screen with **C**
- **Drag points** to move them
- Exit with **Esc**

---

### Radar Animation
![Radar Animation](./radar.mp4)

A stylized radar simulation featuring:

**Features:**
- Rotating sweep line
- Random blips that fade over time
- Concentric range rings
- Exit with **Esc**

---

### Snake Cursor Follower
![Snake Cursor Follower](./snack.mp4)

A fun snake-like follower that reacts to mouse movement.

**Features:**
- Smooth motion with physics-inspired spring behavior
- Segments follow each other with delay
- Exit with **Esc**

---

## Requirements

- Rust **1.50+**
- `cargo` package manager

---

## Installation

Clone this repository:

```bash
git clone https://github.com/aminehabchi/CHAIKIN.git
cd CHAIKIN
```

---

## Building and Running

To build all demos:

```bash
cargo build --release
```

Run a specific demo (example for Chaikin):

```bash
cargo run --release --bin chaikin
```

---

## Implementation Details

### Chaikin's Algorithm
Steps:
1. Users define a polyline by placing control points.
2. Each iteration replaces every segment AB with two points:
   - One at **¼** of the way from A to B
   - One at **¾** of the way from A to B
3. This is repeated for **7 iterations** to smooth the shape.

### Radar Animation
- Uses trigonometric rotation to animate a **sweeping line**
- **Blips** appear randomly and fade
- **Range rings** offer visual scale

### Snake Cursor Follower
- Uses **spring physics** for smooth trailing
- Segments track the one before it
- Real-time interaction with **mouse position**

---

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.
