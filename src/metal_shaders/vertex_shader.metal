#include <metal_stdlib>
using namespace metal;

#include "shader_types.h"

constant bool is_instanced [[function_constant(0)]];
constant bool use_vertex_color [[function_constant(1)]];

struct Uniforms {
    float4x4 viewProjectionMatrix;
};

struct InstanceData {
    float4x4 modelMatrix;
    float4 color;
};

vertex VertexOut vertex_main(
    VertexIn vertexIn [[stage_in]],
    constant Uniforms &uniforms [[buffer(1)]],
    constant InstanceData *instanceData [[buffer(2)]],
    uint vertexID [[vertex_id]],
    uint instanceID [[instance_id]]
) {
    VertexOut out;

    float4x4 modelMatrix;
    if (is_instanced) {
        modelMatrix = instanceData[instanceID].modelMatrix;
    } else {
        // Create an identity matrix when not instanced
        modelMatrix = float4x4(
            float4(1.0, 0.0, 0.0, 0.0),
            float4(0.0, 1.0, 0.0, 0.0),
            float4(0.0, 0.0, 1.0, 0.0),
            float4(0.0, 0.0, 0.0, 1.0)
        );
    }

    float4 worldPosition = modelMatrix * float4(vertexIn.position, 1.0);
    out.position = uniforms.viewProjectionMatrix * worldPosition;
    out.color = use_vertex_color ? vertexIn.color : (is_instanced ? instanceData[instanceID].color : float4(1.0));

    return out;
}


