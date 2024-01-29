#define TWO_PI 6.28318530718
#define NUM_TRACERS 400
#define TRACER_SIZE 2 //Number of floats per tracer
#define SIM_RESOLUTION 1000.0 //Grid size
#define RESOLUTION 800.0 //Canvas size

#define BRIGHTNESS 0.4
#define SPREAD 6.0
#define CROSSHAIR_LINE_WIDTH 0.001
#define CROSSHAIR_SIZE 0.02
#define CROSSHAIR_GAP_SIZE 0.005

//uniform float u_resolution;
uniform float u_time;
uniform float u_tracer_data[NUM_TRACERS * TRACER_SIZE];
uniform vec2 u_mouse_position;
uniform bool u_mouse_active;
uniform bool u_mouse_attractive;

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

vec3 draw_crosshair(vec2 st, vec2 mouse_coords) {
    vec3 crosshair_color = vec3(0.0);
    if(u_mouse_attractive && u_mouse_active) {
        crosshair_color = vec3(1.0, 1.0, 0.0);
    } else if(!u_mouse_attractive && u_mouse_active) {
        crosshair_color = vec3(1.0, 0.0, 0.0);
    } else if(u_mouse_attractive && !u_mouse_active) {
        crosshair_color = vec3(0.0, 1.0, 0.0);
    } else if(!u_mouse_attractive && !u_mouse_active) {
        crosshair_color = vec3(0.0, 0.0, 1.0);
    }

    if(abs(st.x - mouse_coords.x) < CROSSHAIR_GAP_SIZE && abs(st.y - mouse_coords.y) < CROSSHAIR_GAP_SIZE) {
        return vec3(0.0);
    }
    if(st.x >= mouse_coords.x - CROSSHAIR_LINE_WIDTH && st.x <= mouse_coords.x + CROSSHAIR_LINE_WIDTH && abs(st.y - mouse_coords.y) <= CROSSHAIR_SIZE) {
        return crosshair_color;
    }
    if(st.y >= mouse_coords.y - CROSSHAIR_LINE_WIDTH && st.y <= mouse_coords.y + CROSSHAIR_LINE_WIDTH && abs(st.x - mouse_coords.x) <= CROSSHAIR_SIZE) {
        return crosshair_color;
    }
    return vec3(0.0);
}

void main() {
    float spread = SPREAD / 1000.0;
    float cell_size = 1.0 / SIM_RESOLUTION;
    vec2 st = gl_FragCoord.xy / RESOLUTION;
    vec2 mouse_coords = u_mouse_position / RESOLUTION;
    mouse_coords.y = 1.0 - mouse_coords.y;

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

    vec3 crosshair = draw_crosshair(st, mouse_coords);
    if(crosshair != vec3(0.0)) {
        color = crosshair;
    }

    gl_FragColor = vec4(color, 1.0);
}