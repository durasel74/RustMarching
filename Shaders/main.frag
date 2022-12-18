#version 330 core

struct Camera
{
    vec3 position;
    vec3 direction;
    float field_of_view;
};

uniform vec2 screen_resolution;
uniform Camera camera;

uniform float u_time;
out vec4 frag_color;

float distance_from_sphere(vec3 point, vec3 center, float radius)
{
	return length(point - center) - radius;
}

float map_the_world(in vec3 point)
{
    float displacement = (sin(point.x) * sin(point.y) * sin(point.z)) * u_time;
    float sphere_0 = distance_from_sphere(point, vec3(0.0), 1.0);

    return sphere_0 + displacement;
    //return sphere_0;
}

vec3 calculate_normal(in vec3 p)
{
    const vec3 small_step = vec3(0.001, 0.0, 0.0);

    float gradient_x = map_the_world(p + small_step.xyy) - map_the_world(p - small_step.xyy);
    float gradient_y = map_the_world(p + small_step.yxy) - map_the_world(p - small_step.yxy);
    float gradient_z = map_the_world(p + small_step.yyx) - map_the_world(p - small_step.yyx);

    vec3 normal = vec3(gradient_x, gradient_y, gradient_z);

    return normalize(normal);
}

vec3 ray_march(in vec3 ray_origin, in vec3 ray_direction)
{
    float total_distance_traveled = 0.0;
    const int NUMBER_OF_STEPS = 32;
    const float MINIMUM_HIT_DISTANCE = 0.001;
    const float MAXIMUM_TRACE_DISTANCE = 1000.0;

    for (int i = 0; i < NUMBER_OF_STEPS; ++i)
    {
        vec3 current_position = ray_origin + total_distance_traveled * ray_direction;

        float distance_to_closest = map_the_world(current_position);

        if (distance_to_closest < MINIMUM_HIT_DISTANCE)
        {
            vec3 normal = calculate_normal(current_position);
            
            vec3 light_position = vec3(2.0, -5.0, 3.0);

            vec3 direction_to_light = normalize(current_position - light_position);

            float diffuse_intensity = max(0.0, dot(normal, direction_to_light));

            return vec3(1.0, 0.0, 0.0) * diffuse_intensity;
        }

        if (total_distance_traveled > MAXIMUM_TRACE_DISTANCE)
        {
            break;
        }

        total_distance_traveled += distance_to_closest;
    }

    return vec3(0.1);
}

void main()
{
    vec2 uv = (2.0 * gl_FragCoord.xy - screen_resolution.xy) / screen_resolution.y;

    vec3 ray_origin = camera.position;
    vec3 ray_direction = normalize(vec3(uv, camera.field_of_view));

    vec3 shaded_color = ray_march(ray_origin, ray_direction);

    frag_color = vec4(shaded_color, 1.0);
}
