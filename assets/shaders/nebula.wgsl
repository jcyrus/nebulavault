// Nebula Cosmic Cloud Shader
// Domain Warping Noise for cinematic background

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Uniforms {
    time: f32,
    resolution: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Hash function for noise
fn hash(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * 0.13);
    p3 += dot(p3, p3.yzx + 3.333);
    return fract((p3.x + p3.y) * p3.z);
}

// 2D Noise
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    
    let u = f * f * (3.0 - 2.0 * f);
    
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

// Fractional Brownian Motion
fn fbm(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    var p_var = p;
    
    for (var i = 0; i < 6; i++) {
        value += amplitude * noise(p_var * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    
    return value;
}

// Domain Warping
fn domain_warp(p: vec2<f32>, time: f32) -> vec2<f32> {
    let q = vec2<f32>(
        fbm(p + vec2<f32>(0.0, 0.0)),
        fbm(p + vec2<f32>(5.2, 1.3))
    );
    
    let r = vec2<f32>(
        fbm(p + 4.0 * q + vec2<f32>(1.7 - time * 0.15, 9.2)),
        fbm(p + 4.0 * q + vec2<f32>(8.3 - time * 0.1, 2.8))
    );
    
    return p + r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Normalize coordinates
    var uv = in.uv;
    uv = (uv - 0.5) * 2.0;
    uv.x *= uniforms.resolution.x / uniforms.resolution.y;
    
    // Apply domain warping
    let warped = domain_warp(uv * 2.0, uniforms.time);
    
    // Generate noise value
    let n = fbm(warped);
    
    // Color palette
    let navy = vec3<f32>(0.059, 0.090, 0.165);      // #0f172a
    let purple = vec3<f32>(0.486, 0.227, 0.929);    // #7c3aed
    let pink = vec3<f32>(0.859, 0.153, 0.467);      // #db2777
    
    // Mix colors based on noise
    var color = mix(navy, purple, smoothstep(0.3, 0.6, n));
    color = mix(color, pink, smoothstep(0.6, 0.9, n));
    
    // Add subtle animation
    let pulse = sin(uniforms.time * 0.5) * 0.5 + 0.5;
    color += vec3<f32>(0.02) * pulse;
    
    // Vignette effect
    let vignette = 1.0 - length(in.uv - 0.5) * 0.8;
    color *= vignette;
    
    return vec4<f32>(color, 1.0);
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Full-screen quad
    let x = f32((vertex_index & 1u) << 1u) - 1.0;
    let y = f32((vertex_index & 2u)) - 1.0;
    
    out.position = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>((x + 1.0) * 0.5, (1.0 - y) * 0.5);
    
    return out;
}
