/*
Data gets produced by the vertex shader.
It is basically required at a GPU level like this to have a shader.

The GPU should run our shader every vertex and the struct is basically values passed from it to the rendering pipeline.
*/
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>, // The position of the vertex in clip space which is basically just a Coordinate system from 1 to -1 in both axis.
};

@vertex
fn vs_main(
    @builtin(vertex_index) index: u32,
) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        // The positions of the vertices in clip space.
        vec2<f32>(-0.4, 0.4),
        vec2<f32>(-0.4, -0.4),
        vec2<f32>(0.4, 0.4),
        vec2<f32>(0.4, 0.4),
        vec2<f32>(-0.4, -0.4),
        vec2<f32>(0.4, -0.4),
    );

    var output: VertexOutput;

    output.clip_position = vec4<f32>(positions[index], 0.0, 1.0);

    return output;
}

@fragment
fn fs_main(
    _input: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4<f32>(0.15, 0.35, 0.65, 1.0);
}