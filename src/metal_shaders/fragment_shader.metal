#include <metal_stdlib>
using namespace metal;

#include "shader_types.h"

fragment float4 fragment_main(VertexOut in [[stage_in]]) {
    return in.color;
}