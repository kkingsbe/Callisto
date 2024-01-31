#define TWO_PI 6.28318530718
#define NUM_TRACERS 200
#define TRACER_SIZE 2 //Number of floats per tracer
#define SIM_RESOLUTION 10000.0 //Grid size

#define BRIGHTNESS 0.1
#define SPREAD 6.0
#define CROSSHAIR_LINE_WIDTH 0.001
#define CROSSHAIR_SIZE 0.02
#define CROSSHAIR_GAP_SIZE 0.005

uniform vec2 u_resolution;
uniform float u_tracer_data[TRACER_SIZE];

void main() {
    float spread = SPREAD / 1000.0;
    float cell_size = 1.0 / SIM_RESOLUTION;
    vec2 st = gl_FragCoord.xy / u_resolution;

    vec3 color = vec3(0.0);

    // Determine what grid square st is in
    vec2 grid_pos = floor(st * SIM_RESOLUTION);

    //Set a point to the midpoint of the grid square
    vec2 uv = ((grid_pos) / SIM_RESOLUTION) + (cell_size / 2.0);

    //float dist = distance(st, uv) / cell_size; //Somehow this is the distance from the grid edge
    //dist = 1.0 - dist; //Invert the distance so that it's 1 at the edge and 0 at the center

    vec2 tracer_pos = vec2(u_tracer_data[0], u_tracer_data[1]);;

    //Calculate distance from point to tracer
    float dist = distance(uv, tracer_pos);
    //float i_dist = 0.025 - dist;
    //float i_dist = (u_time / 100.0) - dist;

    //float spread = 0.05;
    //float brightness = 0.1;

    float density_contrib = spread / (dist);
    if(density_contrib > 1.0) density_contrib = 1.0;
    float density = density_contrib * BRIGHTNESS;

    gl_FragColor = vec4(vec3(1.0), density);
}