# Rust Graphics Demonstrations with minifb

This repository contains three interactive graphical demonstrations built with Rust and the [minifb](https://github.com/emoon/rust_minifb) library.

## Programs Included

### 1. Chaikin's Curve Algorithm Animation
An interactive implementation of Chaikin's curve algorithm with step-by-step animation.

### 2. Radar Animation
A simple radar animation with sweeping line and blips.

### 3. Snake Cursor Follower
A snake that follows your cursor movements across the screen.

## Demonstrations

### Chaikin's Curve Algorithm
![Chaikin's Curve Animation](./CHAKIN.mov)

Chaikin's algorithm progressively smooths a polyline by replacing each line segment with two shorter segments, creating a curve-like appearance over multiple iterations.

**Features:**
- Place control points with left mouse button
- Press ENTER to start the animation
- Animation cycles through 7 steps of curve refinement
- Clear screen with C key
- Move points by dragging them
- Press ESC to exit

**Usage:**
```bash
cargo run --bin chaikin
```

### Radar Animation
![Radar Animation](./radar.mov)

A classic radar display simulation with a sweeping line and randomly appearing blips.

**Features:**
- Radar sweep animation
- Random echo blips that fade over time
- Press ESC to exit

**Usage:**
```bash
cargo run --bin radar
```

### Snake Cursor Follower
![Snake Cursor Follower](./snack.mov)

A snake-like entity that smoothly follows your mouse cursor around the screen.

**Features:**
- Snake follows cursor movement with natural-looking motion
- Body segments follow the head with delay effect
- Press ESC to exit

**Usage:**
```bash
cargo run --bin snake
```

## Requirements

- Rust 1.50 or higher
- Cargo package manager

## Installation

Clone the repository:
```bash
git https://github.com/aminehabchi/CHAIKIN.git

```

## Building and Running

Build all demos:
```bash
cargo build --release
```

Run a specific demo (examples):
```bash
cargo run --release --bin chaikin
cargo run --release --bin radar
cargo run --release --bin snake
```

## Implementation Details

### Chaikin's Algorithm
The implementation follows these steps:
1. User places control points to form a polyline
2. For each iteration:
   - Each line segment AB is replaced with two new points:
   - One at 1/4 of the way from A to B
   - One at 3/4 of the way from A to B
3. The process repeats for 7 steps, creating a progressively smoothed curve

### Radar Animation
Uses rotation mathematics to create:
1. A sweeping line that rotates 360 degrees
2. Random "blips" that appear and fade over time
3. Distance rings for scale reference

### Snake Cursor Follower
Implements:
1. A physics-based follower system with spring-like properties
2. Multiple segments that follow the segment in front of them
3. Smooth movement with velocity and acceleration

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
