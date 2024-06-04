#version 330 core
out vec4 foColor;

uniform uint gMaxValue;
uniform uint gValue;
uniform uint gIndexHighlight;
uniform uint gIndex;

vec3 hueToRgb(float hue)
{
    vec3 c = vec3(hue, 1.0, 0.8);
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main()
{
    if (gIndex == gIndexHighlight)
    {
        foColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
    else
    {
        float value = gValue;
        float max = gMaxValue;
        foColor = vec4(
            hueToRgb(0.9 * value / max),
            1.0
        );
    }
}
