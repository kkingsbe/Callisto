#define TWO_PI 6.28318530718
#define TRACER_SIZE 2
#define TRACER_RAD 0.005

uniform vec2 u_resolution;
uniform float u_tracer_data[TRACER_SIZE];

void main() {
    vec2 st = gl_FragCoord.xy / u_resolution;

    vec3 color = vec3(0.0);

    vec2 tracer_pos = vec2(u_tracer_data[0], u_tracer_data[1]);;
    float alpha = 1.0 - step(TRACER_RAD, distance(st, tracer_pos));

    gl_FragColor = vec4(vec3(1.0), alpha);
}