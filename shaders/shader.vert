#version 330 core
layout (location = 0) in vec3 vPos;

uniform uint gMaxCount;
uniform uint gMaxValue;
uniform uint gValue;
uniform uint gIndex;

void main()
{
    float barWidth = 2.0 / gMaxCount;
    float barHeight = 2.0 * gValue / gMaxValue;
    gl_Position = vec4(
        gIndex * barWidth + vPos.x * barWidth - 1.0,
        vPos.y * barHeight - 1.0,
        vPos.z,
        1.0
    );
}