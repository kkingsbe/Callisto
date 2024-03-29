# Callisto
A fun particle simulation I made to test out some rendering techniques with OpenGL & Rust.
## Controls:
- Ctrl: Toggle left click action betweeen attraction & repulsion
- Left Click: Activate left click action

![ezgif-5-7e7416f53d](https://github.com/kkingsbe/Callisto/assets/22225021/364459bd-822c-48ea-8ecd-68f22067eb77)

## How to edit:
### Simulation Parameters
To modify the parameters of the simulation, you can edit the default function for the `Simulation` struct. Below are descriptions for each of the parameters:
- `dt`: The simulation timestep (in ms)
- `attractive_force`: The value for the attraction between the particles. This is analogus to gravity and follows the inverse-square law.
- `repulsive_force`: The value for the repulsive force between the particles. This is analogus to the forces which prevent stars from collapsing into black holes. If this is set to 0 the same is possible in the sim :). This follows a leonard-jones potential (so that it can start smaller than the attractive force but ramp up quickly after some threshold)
- `drag`: This is the value for the drag force which slows the particles down. Without this, errors in the integration process will cause the energy in the system to increase until it blows up.
- `max_spawn_velocity`: Sets the magnitude of the maximum velocity a particle can be spawned with. Higher value = more initial energy in the system.
- `num_particles`: The number of particles in the simulation. If you change this, you must also change it in the `shaders/visualize.frag` shader
- `microsteps`: Keep this at 1 for now.
- `gravity`: If true it will activate a gravity force which pulls all of the particles down in the -y direction.
- `domain_mode`: This allows you to set how the edgees of the domain (application window) are treated.
  - `DOMAIN_MODE::WRAP`: When particles exit one edge of the screen, they will appear from the other. For example, if a particle moves past the right edge, it will re-enter the window at the left edge.
  - `DOMAIN_MODE::INFINITE`: No edge constraints. Particles past the edge still exist & can be interacted with, but wont be visible until they re-enter the window.
  - `DOMAIN_MODE::WALL`: When particles reach the edge of the window they will bounce back with the same velocity. This mode paired with a relatively high drag value works well.

 ### Shader Parameters
 To modify the parameters within the shader, you can edit `shaders/visualize.frag`. At the top of the file there are a few consts which define some of the renderering behavior.
 - `NUM_TRACERS`: Make sure this is the same as the number of particles in the simulation.
 - `SIM_RESOLUTION`: Defines the number of subdivisions in the grid used for averaging the particle values. Larger value will show more detial. Smaller value will make it appear to be more pixelated.
 - `RESOLUTION`: Keep this at `800.0` for now
 - `BRIGHTNESS`: This seems to effect which portion of the gradient is displayed. Tweaking this value can give vastly different colors.
 - `SPREAD`: Larger values will make the particles blend together more, into more of a fluid. Lower values will allow the particles to be more defined.
 - `CROSSHAIR_LINE_WIDTH`: Controls how thick the lines in the crosshair are.
 - `CROSSHAIR_SIZE`: Controls the total size of the crosshair.
 - `CROSSHAIR_GAP_SIZE`: How large the gap in the middle of the crosshair is.
#### Color Schemes
New color schemes can be added following the format of the functions at the top of the shader. They should take in a float ranging from `0.0` to `1.0` and return a vec3 of the color for that input. I have found there is a bit of exploration to be done here, as you can run the input value through a `smoothstep` (or just a `step`) to get intereesting results. To apply your color scheme, you can just modify the end of the shader to use your function instead. You can also use `mix` to mix multiple color schemes together.
