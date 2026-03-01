// Vertex shader
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );
    
    return vec4<f32>(pos[vertex_index], 0.0, 1.0);
}

// Fragment shader
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // Placeholder - in real implementation, this would sample from video texture
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}