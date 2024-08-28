#ifndef ShaderTypes_h
#define ShaderTypes_h
#include <simd/simd.h>
#include <metal_stdlib>
using namespace metal;

struct VertexIn
{
    float3 position [[attribute(0)]];
    float4 color [[attribute(1)]];
};

struct VertexOut
{
    float4 position [[position]];
    float4 color;
};

#endif /* ShaderTypes_h */