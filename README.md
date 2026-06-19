# Gradient Descent Simulation

Visualizer for gradient descent optimization algorithms, built with Rust and iced.

Select a function, tweak the parameters, and watch the algorithm find the minimum.

![preview](https://github.com/user-attachments/assets/0e5c7967-6e0c-4cbf-a4c5-0e8b9faeafd7)

## Status
Work in progress

## Stack
- Rust
- iced (GUI + canvas)

## Functions
- Square - f(x) = x²
- Bowl - f(x, y) = x² + y²
- Rosenbrock - f(x, y) = (1 - x)² + 100(y - x²)²
- Himmelblau - f(x, y) = (x² + y - 11)² + (x + y² - 7)²

## Run
```bash
cargo run
```
