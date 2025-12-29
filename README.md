# fishtank

A real-time SPH (Smoothed Particle Hydrodynamics) fluid simulator running in the terminal.

## Installation

```bash
cargo install --path .
```

Or run directly:

```bash
cargo run --release
```

## Controls

### General
- `q` - quit
- `i` - toggle info panel
- `s` - toggle settings panel

### Settings
- `↑` / `↓` - navigate parameters
- `←` / `→` - decrease/increase selected parameter
- `r` - reset selected parameter to default

### Mouse
- left click & drag - apply attractive force
- right click & drag - apply repulsive force

## Parameters

The settings panel lets you adjust simulation parameters in real-time:
- **Particle Count** - Number of fluid particles
- **Gravity** - Downward force
- **Target Density** - Rest density of the fluid
- **Viscosity** - Fluid thickness/resistance
- **Stiffness** - Pressure response strength
- **Smoothing Radius** - Particle interaction distance
- **Dampening** - Boundary energy loss
- **Mouse Force** - Strength of mouse interactions
- **Mouse Radius** - Range of mouse forces
