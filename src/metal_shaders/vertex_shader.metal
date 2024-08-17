#include <metal_stdlib>
using namespace metal;
#include "shader_types.h"

struct VertexUniforms {
    float4x4 viewProjectionMatrix;
};

vertex VertexOut vertex_main(VertexIn vertex_in [[stage_in]],
                             constant VertexUniforms &uniforms [[buffer(1)]]) {
    VertexOut out;
    float4 worldPosition = float4(vertex_in.position, 1.0);
    out.position = uniforms.viewProjectionMatrix * worldPosition;
    out.color = vertex_in.color;
    return out;
}
