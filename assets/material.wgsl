#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;

struct FragOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32
}
@fragment
fn fragment(mesh: VertexOutput) -> FragOut {
    let color = textureSample(base_color_texture, base_color_sampler, mesh.uv);

    if color.a < 0.5 {
        discard;
    }

    var out: FragOut;
    out.color = color * material_color;
    if color.r < 0.5 {
        out.depth = color.r / 2.;
        // out.color = vec4(1.,0.,0.,1.);
    } else {
        out.depth = 1.;
        // out.color = vec4(vec3(0.,1.,0.),1.);
    }


    return out;
    
}


