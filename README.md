# Garbage Truck Game

Vibecoded it for my kids. Pretty sure the code is garbage as well.

A simple 2D top-down arcade game built with [Rust](https://www.rust-lang.org/) and [Bevy](https://bevyengine.org/).

## Description

Drive a garbage truck around an infinite parking lot, collecting full garbage bins while avoiding obstacles. The game features an infinite scrolling background and score tracking.

## Features

- **Infinite World**: The background wraps around the camera, creating an endless play area.
- **Asset Embedding**: All game assets are embedded directly into the binary, making it easy to distribute as a single file.
- **Particle Effects**: Satisfying sparkle effects when collecting garbage.
- **Score System**: Track your progress as you clean up the lot.

## Controls

- **W / Up Arrow**: Accelerate forward
- **S / Down Arrow**: Reverse / Brake
- **A / Left Arrow**: Turn Left
- **D / Right Arrow**: Turn Right
- **ESC**: Quit the game

## Installation and Running

Ensure you have Rust installed. Clone the repository and run:

```bash
cargo run --release
```

## Technologies Used

- **Bevy 0.17**: Game engine
- **bevy_hanabi**: Particle effects
- **rand**: Random number generation
