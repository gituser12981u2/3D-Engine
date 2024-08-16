#include <metal_stdlib>
using namespace metal;
#include "shader_types.h"

vertex VertexOut vertex_main(VertexIn vertex_in [[stage_in]]) {
    VertexOut out;
    out.position = float4(vertex_in.position, 1.0);
    out.color = vertex_in.color;
    return out;
}
