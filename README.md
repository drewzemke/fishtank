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
- **Particle Count** - number of fluid particles
- **Gravity** - downward force
- **Target Density** - rest density of the fluid
- **Viscosity** - fluid thickness/resistance
- **Stiffness** - pressure response strength
- **Smoothing Radius** - particle interaction distance
- **Dampening** - boundary energy loss
- **Mouse Force** - strength of mouse interactions
- **Mouse Radius** - range of mouse forces

## Future Goals

- [ ] add actual fish
