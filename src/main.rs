use skshader_macro;
pub use skshader_macro::include_shader;
pub use skshaderc_bindings::{compile_shader_file, CompilerSettings, SkShaderCSettings, };


static thing: &'static [u8] = include_shader!("hi.hlsl",
r#"
 #include "stereokit.hlsli"

//--name = sk/default

//--color:color = 1,1,1,1
//--tex_scale   = 1
//--diffuse     = white

float4       color;
float        tex_scale;
Texture2D    diffuse   : register(t0);
SamplerState diffuse_s : register(s0);

struct vsIn {
	float4 pos  : SV_Position;
	float3 norm : NORMAL0;
	float2 uv   : TEXCOORD0;
	float4 col  : COLOR0;
};
struct psIn {
	float4 pos   : SV_Position;
	float2 uv    : TEXCOORD0;
	float4 color : COLOR0;
	uint view_id : SV_RenderTargetArrayIndex;
};

psIn vs(vsIn input, uint id : SV_InstanceID) {
	psIn o;
	o.view_id = id % sk_view_count;
	id        = id / sk_view_count;

	float4 world = mul(input.pos, sk_inst[id].world);
	o.pos        = mul(world,     sk_viewproj[o.view_id]);

	float3 normal = normalize(mul(input.norm, (float3x3)sk_inst[id].world));

	o.uv         = input.uv * tex_scale;
	o.color      = color * input.col * sk_inst[id].color;
	o.color.rgb *= Lighting(normal);
	return o;
}

float4 ps(psIn input) : SV_TARGET {
	float4 col = diffuse.Sample(diffuse_s, input.uv);
	return col * input.color;
}
"#
);

fn main() {
    let bytes = thing.to_vec();
    println!("bytes: {:?}", bytes);
    let utf8 = unsafe { String::from_utf8_unchecked(thing.to_vec())};
    println!("{}", utf8);
}

#[test]
fn test() {
    let utf8 = unsafe { String::from_utf8_unchecked(thing.to_vec()) };
    panic!("{}", utf8);
}
