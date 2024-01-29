#define TWO_PI 6.28318530718
#define NUM_TRACERS 400
#define TRACER_SIZE 2 //Number of floats per tracer
#define SIM_RESOLUTION 1000.0 //Grid size
#define RESOLUTION 800.0 //Canvas size

#define BRIGHTNESS 0.4
#define SPREAD 6.0

//uniform float u_resolution;
uniform float u_time;
uniform float u_tracer_data[NUM_TRACERS * TRACER_SIZE];

vec3 lightblue(float value) {
    return mix(vec3(0.0, 0.222, 0.731), vec3(0.212, 0.625, 0.684), value);
}

vec3 sand(float value) {
    float modded = smoothstep(0.8, 1.0, value);
    return mix(vec3(0.773, 0.475, 0.428), vec3(0.858, 0.902, 0.865), modded);
}

vec3 purplered(float value) {
    return mix(vec3(0.925, 0.4314, 0.678), vec3(0.204, 0.58, 0.9), value);
}

void main() {
    float spread = SPREAD / 1000.0;
    float cell_size = 1.0 / SIM_RESOLUTION;
    vec2 st = gl_FragCoord.xy / RESOLUTION;
    float density = 0.0;
    vec3 color = vec3(0.0);

    // Determine what grid square st is in
    vec2 grid_pos = floor(st * SIM_RESOLUTION);

    //Set a point to the midpoint of the grid square
    vec2 uv = ((grid_pos) / SIM_RESOLUTION) + (cell_size / 2.0);

    //float dist = distance(st, uv) / cell_size; //Somehow this is the distance from the grid edge
    //dist = 1.0 - dist; //Invert the distance so that it's 1 at the edge and 0 at the center

    //Iterate over each tracer
    for (int i = 0; i < NUM_TRACERS * TRACER_SIZE; i += 2) {
        //Get tracer data
        vec2 tracer_pos = vec2(u_tracer_data[i], u_tracer_data[i + 1]);;
        //vec2 tracer_pos = vec2(u_tracer_data[0], u_tracer_data[1]);;

        //Calculate distance from point to tracer
        float dist = distance(uv, tracer_pos);
        //float i_dist = 0.025 - dist;
        float i_dist = (u_time / 100.0) - dist;

        //float spread = 0.05;
        //float brightness = 0.1;

        float density_contrib = spread / (dist);
        if(density_contrib > 1.0) density_contrib = 1.0;
        density += density_contrib * BRIGHTNESS;
    }

    //Render the denisty as a heatmap
    //color = lightblue(density);
    //color = purplered(density);
    color = lightblue(density);

    gl_FragColor = vec4(color, 1.0);
}