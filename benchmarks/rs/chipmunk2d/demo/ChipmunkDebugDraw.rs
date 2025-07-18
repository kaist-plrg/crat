use ::libc;
extern "C" {
    fn sg_make_buffer(desc: *const sg_buffer_desc) -> sg_buffer;
    fn sg_make_shader(desc: *const sg_shader_desc) -> sg_shader;
    fn sg_make_pipeline(desc: *const sg_pipeline_desc) -> sg_pipeline;
    fn sg_update_buffer(
        buf: sg_buffer,
        data_ptr: *const libc::c_void,
        data_size: libc::c_int,
    );
    fn sg_apply_pipeline(pip: sg_pipeline);
    fn sg_apply_bindings(bindings_0: *const sg_bindings);
    fn sg_apply_uniforms(
        stage: sg_shader_stage,
        ub_index: libc::c_int,
        data: *const libc::c_void,
        num_bytes: libc::c_int,
    );
    fn sg_draw(
        base_element: libc::c_int,
        num_elements: libc::c_int,
        num_instances: libc::c_int,
    );
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn abort() -> !;
    fn cpMessage(
        condition: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        isError: libc::c_int,
        isHardError: libc::c_int,
        message: *const libc::c_char,
        _: ...
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_buffer {
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_image {
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader {
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pipeline {
    pub id: uint32_t,
}
pub type sg_usage = libc::c_uint;
pub const _SG_USAGE_FORCE_U32: sg_usage = 2147483647;
pub const _SG_USAGE_NUM: sg_usage = 4;
pub const SG_USAGE_STREAM: sg_usage = 3;
pub const SG_USAGE_DYNAMIC: sg_usage = 2;
pub const SG_USAGE_IMMUTABLE: sg_usage = 1;
pub const _SG_USAGE_DEFAULT: sg_usage = 0;
pub type sg_buffer_type = libc::c_uint;
pub const _SG_BUFFERTYPE_FORCE_U32: sg_buffer_type = 2147483647;
pub const _SG_BUFFERTYPE_NUM: sg_buffer_type = 3;
pub const SG_BUFFERTYPE_INDEXBUFFER: sg_buffer_type = 2;
pub const SG_BUFFERTYPE_VERTEXBUFFER: sg_buffer_type = 1;
pub const _SG_BUFFERTYPE_DEFAULT: sg_buffer_type = 0;
pub type sg_index_type = libc::c_uint;
pub const _SG_INDEXTYPE_FORCE_U32: sg_index_type = 2147483647;
pub const _SG_INDEXTYPE_NUM: sg_index_type = 4;
pub const SG_INDEXTYPE_UINT32: sg_index_type = 3;
pub const SG_INDEXTYPE_UINT16: sg_index_type = 2;
pub const SG_INDEXTYPE_NONE: sg_index_type = 1;
pub const _SG_INDEXTYPE_DEFAULT: sg_index_type = 0;
pub type sg_image_type = libc::c_uint;
pub const _SG_IMAGETYPE_FORCE_U32: sg_image_type = 2147483647;
pub const _SG_IMAGETYPE_NUM: sg_image_type = 5;
pub const SG_IMAGETYPE_ARRAY: sg_image_type = 4;
pub const SG_IMAGETYPE_3D: sg_image_type = 3;
pub const SG_IMAGETYPE_CUBE: sg_image_type = 2;
pub const SG_IMAGETYPE_2D: sg_image_type = 1;
pub const _SG_IMAGETYPE_DEFAULT: sg_image_type = 0;
pub type sg_shader_stage = libc::c_uint;
pub const _SG_SHADERSTAGE_FORCE_U32: sg_shader_stage = 2147483647;
pub const SG_SHADERSTAGE_FS: sg_shader_stage = 1;
pub const SG_SHADERSTAGE_VS: sg_shader_stage = 0;
pub type sg_pixel_format = libc::c_uint;
pub const _SG_PIXELFORMAT_FORCE_U32: sg_pixel_format = 2147483647;
pub const _SG_PIXELFORMAT_NUM: sg_pixel_format = 24;
pub const SG_PIXELFORMAT_ETC2_SRGB8: sg_pixel_format = 23;
pub const SG_PIXELFORMAT_ETC2_RGB8: sg_pixel_format = 22;
pub const SG_PIXELFORMAT_PVRTC4_RGBA: sg_pixel_format = 21;
pub const SG_PIXELFORMAT_PVRTC2_RGBA: sg_pixel_format = 20;
pub const SG_PIXELFORMAT_PVRTC4_RGB: sg_pixel_format = 19;
pub const SG_PIXELFORMAT_PVRTC2_RGB: sg_pixel_format = 18;
pub const SG_PIXELFORMAT_DEPTHSTENCIL: sg_pixel_format = 17;
pub const SG_PIXELFORMAT_DEPTH: sg_pixel_format = 16;
pub const SG_PIXELFORMAT_DXT5: sg_pixel_format = 15;
pub const SG_PIXELFORMAT_DXT3: sg_pixel_format = 14;
pub const SG_PIXELFORMAT_DXT1: sg_pixel_format = 13;
pub const SG_PIXELFORMAT_L8: sg_pixel_format = 12;
pub const SG_PIXELFORMAT_R16F: sg_pixel_format = 11;
pub const SG_PIXELFORMAT_R32F: sg_pixel_format = 10;
pub const SG_PIXELFORMAT_RGBA16F: sg_pixel_format = 9;
pub const SG_PIXELFORMAT_RGBA32F: sg_pixel_format = 8;
pub const SG_PIXELFORMAT_R10G10B10A2: sg_pixel_format = 7;
pub const SG_PIXELFORMAT_R5G5B5A1: sg_pixel_format = 6;
pub const SG_PIXELFORMAT_R5G6B5: sg_pixel_format = 5;
pub const SG_PIXELFORMAT_RGBA4: sg_pixel_format = 4;
pub const SG_PIXELFORMAT_RGB8: sg_pixel_format = 3;
pub const SG_PIXELFORMAT_RGBA8: sg_pixel_format = 2;
pub const SG_PIXELFORMAT_NONE: sg_pixel_format = 1;
pub const _SG_PIXELFORMAT_DEFAULT: sg_pixel_format = 0;
pub type sg_primitive_type = libc::c_uint;
pub const _SG_PRIMITIVETYPE_FORCE_U32: sg_primitive_type = 2147483647;
pub const _SG_PRIMITIVETYPE_NUM: sg_primitive_type = 6;
pub const SG_PRIMITIVETYPE_TRIANGLE_STRIP: sg_primitive_type = 5;
pub const SG_PRIMITIVETYPE_TRIANGLES: sg_primitive_type = 4;
pub const SG_PRIMITIVETYPE_LINE_STRIP: sg_primitive_type = 3;
pub const SG_PRIMITIVETYPE_LINES: sg_primitive_type = 2;
pub const SG_PRIMITIVETYPE_POINTS: sg_primitive_type = 1;
pub const _SG_PRIMITIVETYPE_DEFAULT: sg_primitive_type = 0;
pub type sg_vertex_format = libc::c_uint;
pub const _SG_VERTEXFORMAT_FORCE_U32: sg_vertex_format = 2147483647;
pub const _SG_VERTEXFORMAT_NUM: sg_vertex_format = 14;
pub const SG_VERTEXFORMAT_UINT10_N2: sg_vertex_format = 13;
pub const SG_VERTEXFORMAT_SHORT4N: sg_vertex_format = 12;
pub const SG_VERTEXFORMAT_SHORT4: sg_vertex_format = 11;
pub const SG_VERTEXFORMAT_SHORT2N: sg_vertex_format = 10;
pub const SG_VERTEXFORMAT_SHORT2: sg_vertex_format = 9;
pub const SG_VERTEXFORMAT_UBYTE4N: sg_vertex_format = 8;
pub const SG_VERTEXFORMAT_UBYTE4: sg_vertex_format = 7;
pub const SG_VERTEXFORMAT_BYTE4N: sg_vertex_format = 6;
pub const SG_VERTEXFORMAT_BYTE4: sg_vertex_format = 5;
pub const SG_VERTEXFORMAT_FLOAT4: sg_vertex_format = 4;
pub const SG_VERTEXFORMAT_FLOAT3: sg_vertex_format = 3;
pub const SG_VERTEXFORMAT_FLOAT2: sg_vertex_format = 2;
pub const SG_VERTEXFORMAT_FLOAT: sg_vertex_format = 1;
pub const SG_VERTEXFORMAT_INVALID: sg_vertex_format = 0;
pub type sg_vertex_step = libc::c_uint;
pub const _SG_VERTEXSTEP_FORCE_U32: sg_vertex_step = 2147483647;
pub const _SG_VERTEXSTEP_NUM: sg_vertex_step = 3;
pub const SG_VERTEXSTEP_PER_INSTANCE: sg_vertex_step = 2;
pub const SG_VERTEXSTEP_PER_VERTEX: sg_vertex_step = 1;
pub const _SG_VERTEXSTEP_DEFAULT: sg_vertex_step = 0;
pub type sg_uniform_type = libc::c_uint;
pub const _SG_UNIFORMTYPE_FORCE_U32: sg_uniform_type = 2147483647;
pub const _SG_UNIFORMTYPE_NUM: sg_uniform_type = 6;
pub const SG_UNIFORMTYPE_MAT4: sg_uniform_type = 5;
pub const SG_UNIFORMTYPE_FLOAT4: sg_uniform_type = 4;
pub const SG_UNIFORMTYPE_FLOAT3: sg_uniform_type = 3;
pub const SG_UNIFORMTYPE_FLOAT2: sg_uniform_type = 2;
pub const SG_UNIFORMTYPE_FLOAT: sg_uniform_type = 1;
pub const SG_UNIFORMTYPE_INVALID: sg_uniform_type = 0;
pub type sg_cull_mode = libc::c_uint;
pub const _SG_CULLMODE_FORCE_U32: sg_cull_mode = 2147483647;
pub const _SG_CULLMODE_NUM: sg_cull_mode = 4;
pub const SG_CULLMODE_BACK: sg_cull_mode = 3;
pub const SG_CULLMODE_FRONT: sg_cull_mode = 2;
pub const SG_CULLMODE_NONE: sg_cull_mode = 1;
pub const _SG_CULLMODE_DEFAULT: sg_cull_mode = 0;
pub type sg_face_winding = libc::c_uint;
pub const _SG_FACEWINDING_FORCE_U32: sg_face_winding = 2147483647;
pub const _SG_FACEWINDING_NUM: sg_face_winding = 3;
pub const SG_FACEWINDING_CW: sg_face_winding = 2;
pub const SG_FACEWINDING_CCW: sg_face_winding = 1;
pub const _SG_FACEWINDING_DEFAULT: sg_face_winding = 0;
pub type sg_compare_func = libc::c_uint;
pub const _SG_COMPAREFUNC_FORCE_U32: sg_compare_func = 2147483647;
pub const _SG_COMPAREFUNC_NUM: sg_compare_func = 9;
pub const SG_COMPAREFUNC_ALWAYS: sg_compare_func = 8;
pub const SG_COMPAREFUNC_GREATER_EQUAL: sg_compare_func = 7;
pub const SG_COMPAREFUNC_NOT_EQUAL: sg_compare_func = 6;
pub const SG_COMPAREFUNC_GREATER: sg_compare_func = 5;
pub const SG_COMPAREFUNC_LESS_EQUAL: sg_compare_func = 4;
pub const SG_COMPAREFUNC_EQUAL: sg_compare_func = 3;
pub const SG_COMPAREFUNC_LESS: sg_compare_func = 2;
pub const SG_COMPAREFUNC_NEVER: sg_compare_func = 1;
pub const _SG_COMPAREFUNC_DEFAULT: sg_compare_func = 0;
pub type sg_stencil_op = libc::c_uint;
pub const _SG_STENCILOP_FORCE_U32: sg_stencil_op = 2147483647;
pub const _SG_STENCILOP_NUM: sg_stencil_op = 9;
pub const SG_STENCILOP_DECR_WRAP: sg_stencil_op = 8;
pub const SG_STENCILOP_INCR_WRAP: sg_stencil_op = 7;
pub const SG_STENCILOP_INVERT: sg_stencil_op = 6;
pub const SG_STENCILOP_DECR_CLAMP: sg_stencil_op = 5;
pub const SG_STENCILOP_INCR_CLAMP: sg_stencil_op = 4;
pub const SG_STENCILOP_REPLACE: sg_stencil_op = 3;
pub const SG_STENCILOP_ZERO: sg_stencil_op = 2;
pub const SG_STENCILOP_KEEP: sg_stencil_op = 1;
pub const _SG_STENCILOP_DEFAULT: sg_stencil_op = 0;
pub type sg_blend_factor = libc::c_uint;
pub const _SG_BLENDFACTOR_FORCE_U32: sg_blend_factor = 2147483647;
pub const _SG_BLENDFACTOR_NUM: sg_blend_factor = 16;
pub const SG_BLENDFACTOR_ONE_MINUS_BLEND_ALPHA: sg_blend_factor = 15;
pub const SG_BLENDFACTOR_BLEND_ALPHA: sg_blend_factor = 14;
pub const SG_BLENDFACTOR_ONE_MINUS_BLEND_COLOR: sg_blend_factor = 13;
pub const SG_BLENDFACTOR_BLEND_COLOR: sg_blend_factor = 12;
pub const SG_BLENDFACTOR_SRC_ALPHA_SATURATED: sg_blend_factor = 11;
pub const SG_BLENDFACTOR_ONE_MINUS_DST_ALPHA: sg_blend_factor = 10;
pub const SG_BLENDFACTOR_DST_ALPHA: sg_blend_factor = 9;
pub const SG_BLENDFACTOR_ONE_MINUS_DST_COLOR: sg_blend_factor = 8;
pub const SG_BLENDFACTOR_DST_COLOR: sg_blend_factor = 7;
pub const SG_BLENDFACTOR_ONE_MINUS_SRC_ALPHA: sg_blend_factor = 6;
pub const SG_BLENDFACTOR_SRC_ALPHA: sg_blend_factor = 5;
pub const SG_BLENDFACTOR_ONE_MINUS_SRC_COLOR: sg_blend_factor = 4;
pub const SG_BLENDFACTOR_SRC_COLOR: sg_blend_factor = 3;
pub const SG_BLENDFACTOR_ONE: sg_blend_factor = 2;
pub const SG_BLENDFACTOR_ZERO: sg_blend_factor = 1;
pub const _SG_BLENDFACTOR_DEFAULT: sg_blend_factor = 0;
pub type sg_blend_op = libc::c_uint;
pub const _SG_BLENDOP_FORCE_U32: sg_blend_op = 2147483647;
pub const _SG_BLENDOP_NUM: sg_blend_op = 4;
pub const SG_BLENDOP_REVERSE_SUBTRACT: sg_blend_op = 3;
pub const SG_BLENDOP_SUBTRACT: sg_blend_op = 2;
pub const SG_BLENDOP_ADD: sg_blend_op = 1;
pub const _SG_BLENDOP_DEFAULT: sg_blend_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_bindings {
    pub _start_canary: uint32_t,
    pub vertex_buffers: [sg_buffer; 4],
    pub vertex_buffer_offsets: [libc::c_int; 4],
    pub index_buffer: sg_buffer,
    pub index_buffer_offset: libc::c_int,
    pub vs_images: [sg_image; 12],
    pub fs_images: [sg_image; 12],
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_buffer_desc {
    pub _start_canary: uint32_t,
    pub size: libc::c_int,
    pub type_0: sg_buffer_type,
    pub usage: sg_usage,
    pub content: *const libc::c_void,
    pub label: *const libc::c_char,
    pub gl_buffers: [uint32_t; 2],
    pub mtl_buffers: [*const libc::c_void; 2],
    pub d3d11_buffer: *const libc::c_void,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_attr_desc {
    pub name: *const libc::c_char,
    pub sem_name: *const libc::c_char,
    pub sem_index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_uniform_desc {
    pub name: *const libc::c_char,
    pub type_0: sg_uniform_type,
    pub array_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_uniform_block_desc {
    pub size: libc::c_int,
    pub uniforms: [sg_shader_uniform_desc; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_image_desc {
    pub name: *const libc::c_char,
    pub type_0: sg_image_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_stage_desc {
    pub source: *const libc::c_char,
    pub byte_code: *const uint8_t,
    pub byte_code_size: libc::c_int,
    pub entry: *const libc::c_char,
    pub uniform_blocks: [sg_shader_uniform_block_desc; 4],
    pub images: [sg_shader_image_desc; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_shader_desc {
    pub _start_canary: uint32_t,
    pub attrs: [sg_shader_attr_desc; 16],
    pub vs: sg_shader_stage_desc,
    pub fs: sg_shader_stage_desc,
    pub label: *const libc::c_char,
    pub _end_canary: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_buffer_layout_desc {
    pub stride: libc::c_int,
    pub step_func: sg_vertex_step,
    pub step_rate: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_vertex_attr_desc {
    pub buffer_index: libc::c_int,
    pub offset: libc::c_int,
    pub format: sg_vertex_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_layout_desc {
    pub buffers: [sg_buffer_layout_desc; 4],
    pub attrs: [sg_vertex_attr_desc; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_stencil_state {
    pub fail_op: sg_stencil_op,
    pub depth_fail_op: sg_stencil_op,
    pub pass_op: sg_stencil_op,
    pub compare_func: sg_compare_func,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_depth_stencil_state {
    pub stencil_front: sg_stencil_state,
    pub stencil_back: sg_stencil_state,
    pub depth_compare_func: sg_compare_func,
    pub depth_write_enabled: bool,
    pub stencil_enabled: bool,
    pub stencil_read_mask: uint8_t,
    pub stencil_write_mask: uint8_t,
    pub stencil_ref: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_blend_state {
    pub enabled: bool,
    pub src_factor_rgb: sg_blend_factor,
    pub dst_factor_rgb: sg_blend_factor,
    pub op_rgb: sg_blend_op,
    pub src_factor_alpha: sg_blend_factor,
    pub dst_factor_alpha: sg_blend_factor,
    pub op_alpha: sg_blend_op,
    pub color_write_mask: uint8_t,
    pub color_attachment_count: libc::c_int,
    pub color_format: sg_pixel_format,
    pub depth_format: sg_pixel_format,
    pub blend_color: [libc::c_float; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_rasterizer_state {
    pub alpha_to_coverage_enabled: bool,
    pub cull_mode: sg_cull_mode,
    pub face_winding: sg_face_winding,
    pub sample_count: libc::c_int,
    pub depth_bias: libc::c_float,
    pub depth_bias_slope_scale: libc::c_float,
    pub depth_bias_clamp: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sg_pipeline_desc {
    pub _start_canary: uint32_t,
    pub layout: sg_layout_desc,
    pub shader: sg_shader,
    pub primitive_type: sg_primitive_type,
    pub index_type: sg_index_type,
    pub depth_stencil: sg_depth_stencil_state,
    pub blend: sg_blend_state,
    pub rasterizer: sg_rasterizer_state,
    pub label: *const libc::c_char,
    pub _end_canary: uint32_t,
}
pub type cpFloat = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpTransform {
    pub a: cpFloat,
    pub b: cpFloat,
    pub c: cpFloat,
    pub d: cpFloat,
    pub tx: cpFloat,
    pub ty: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBB {
    pub l: cpFloat,
    pub b: cpFloat,
    pub r: cpFloat,
    pub t: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceDebugColor {
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
    pub a: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Uniforms {
    pub U_vp_matrix: [libc::c_float; 16],
}
pub type Index = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub pos: float2,
    pub uv: float2,
    pub r: libc::c_float,
    pub fill: RGBA8,
    pub outline: RGBA8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RGBA8 {
    pub r: uint8_t,
    pub g: uint8_t,
    pub b: uint8_t,
    pub a: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct float2 {
    pub x: libc::c_float,
    pub y: libc::c_float,
}
#[inline]
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn cpv(x: cpFloat, y: cpFloat) -> cpVect {
    let mut v: cpVect = {
        let mut init = cpVect { x: x, y: y };
        init
    };
    return v;
}
#[inline]
unsafe extern "C" fn cpvadd(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x + v2.x, v1.y + v2.y);
}
#[inline]
unsafe extern "C" fn cpvsub(v1: cpVect, v2: cpVect) -> cpVect {
    return cpv(v1.x - v2.x, v1.y - v2.y);
}
#[inline]
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    return cpv(v.x * s, v.y * s);
}
#[inline]
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvrperp(v: cpVect) -> cpVect {
    return cpv(v.y, -v.x);
}
#[inline]
unsafe extern "C" fn cpvforangle(a: cpFloat) -> cpVect {
    return cpv(cos(a), sin(a));
}
#[inline]
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    return sqrt(cpvdot(v, v));
}
#[inline]
unsafe extern "C" fn cpvnormalize(v: cpVect) -> cpVect {
    return cpvmult(
        v,
        1.0f32 as libc::c_double / (cpvlength(v) + 2.2250738585072014e-308f64),
    );
}
#[inline]
unsafe extern "C" fn LAColor(
    mut l: libc::c_float,
    mut a: libc::c_float,
) -> cpSpaceDebugColor {
    let mut color: cpSpaceDebugColor = {
        let mut init = cpSpaceDebugColor {
            r: l,
            g: l,
            b: l,
            a: a,
        };
        init
    };
    return color;
}
pub static mut ChipmunkDebugDrawVPMatrix: cpTransform = cpTransform {
    a: 0.,
    b: 0.,
    c: 0.,
    d: 0.,
    tx: 0.,
    ty: 0.,
};
pub static mut ChipmunkDebugDrawPointLineScale: libc::c_float = 1.0f32;
static mut bindings: sg_bindings = sg_bindings {
    _start_canary: 0,
    vertex_buffers: [sg_buffer { id: 0 }; 4],
    vertex_buffer_offsets: [0; 4],
    index_buffer: sg_buffer { id: 0 },
    index_buffer_offset: 0,
    vs_images: [sg_image { id: 0 }; 12],
    fs_images: [sg_image { id: 0 }; 12],
    _end_canary: 0,
};
static mut pipeline: sg_pipeline = sg_pipeline { id: 0 };
unsafe extern "C" fn cp_to_rgba(mut c: cpSpaceDebugColor) -> RGBA8 {
    return {
        let mut init = RGBA8 {
            r: (0xff as libc::c_int as libc::c_float * c.r) as uint8_t,
            g: (0xff as libc::c_int as libc::c_float * c.g) as uint8_t,
            b: (0xff as libc::c_int as libc::c_float * c.b) as uint8_t,
            a: (0xff as libc::c_int as libc::c_float * c.a) as uint8_t,
        };
        init
    };
}
static mut VertexBuffer: sg_buffer = sg_buffer { id: 0 };
static mut IndexBuffer: sg_buffer = sg_buffer { id: 0 };
static mut VertexCount: size_t = 0;
static mut IndexCount: size_t = 0;
static mut Vertexes: [Vertex; 65536] = [Vertex {
    pos: float2 { x: 0., y: 0. },
    uv: float2 { x: 0., y: 0. },
    r: 0.,
    fill: RGBA8 { r: 0, g: 0, b: 0, a: 0 },
    outline: RGBA8 { r: 0, g: 0, b: 0, a: 0 },
}; 65536];
static mut Indexes: [Index; 262144] = [0; 262144];
pub unsafe extern "C" fn ChipmunkDebugDrawInit() {
    VertexBuffer = sg_make_buffer(
        &mut {
            let mut init = sg_buffer_desc {
                _start_canary: 0,
                size: ((64 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Vertex>() as libc::c_ulong)
                    as libc::c_int,
                type_0: SG_BUFFERTYPE_VERTEXBUFFER,
                usage: SG_USAGE_STREAM,
                content: 0 as *const libc::c_void,
                label: b"ChipmunkDebugDraw Vertex Buffer\0" as *const u8
                    as *const libc::c_char,
                gl_buffers: [0; 2],
                mtl_buffers: [0 as *const libc::c_void; 2],
                d3d11_buffer: 0 as *const libc::c_void,
                _end_canary: 0,
            };
            init
        },
    );
    IndexBuffer = sg_make_buffer(
        &mut {
            let mut init = sg_buffer_desc {
                _start_canary: 0,
                size: ((4 as libc::c_int * (64 as libc::c_int * 1024 as libc::c_int))
                    as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Index>() as libc::c_ulong)
                    as libc::c_int,
                type_0: SG_BUFFERTYPE_INDEXBUFFER,
                usage: SG_USAGE_STREAM,
                content: 0 as *const libc::c_void,
                label: b"ChipmunkDebugDraw Index Buffer\0" as *const u8
                    as *const libc::c_char,
                gl_buffers: [0; 2],
                mtl_buffers: [0 as *const libc::c_void; 2],
                d3d11_buffer: 0 as *const libc::c_void,
                _end_canary: 0,
            };
            init
        },
    );
    bindings = {
        let mut init = sg_bindings {
            _start_canary: 0,
            vertex_buffers: [
                VertexBuffer,
                sg_buffer { id: 0 },
                sg_buffer { id: 0 },
                sg_buffer { id: 0 },
            ],
            vertex_buffer_offsets: [0; 4],
            index_buffer: IndexBuffer,
            index_buffer_offset: 0,
            vs_images: [sg_image { id: 0 }; 12],
            fs_images: [sg_image { id: 0 }; 12],
            _end_canary: 0,
        };
        init
    };
    let mut shd: sg_shader = sg_make_shader(
        &mut {
            let mut init = sg_shader_desc {
                _start_canary: 0,
                attrs: [sg_shader_attr_desc {
                    name: 0 as *const libc::c_char,
                    sem_name: 0 as *const libc::c_char,
                    sem_index: 0,
                }; 16],
                vs: {
                    let mut init = sg_shader_stage_desc {
                        source: b"#version 330\nlayout(location = 0) in vec2 IN_pos; layout(location = 1) in vec2 IN_uv; layout(location = 2) in float IN_radius; layout(location = 3) in vec4 IN_fill; layout(location = 4) in vec4 IN_outline; uniform mat4 U_vp_matrix; out struct { vec2 uv; vec4 fill; vec4 outline; } FRAG; void main(){ gl_Position = U_vp_matrix*vec4(IN_pos + IN_radius*IN_uv, 0, 1); FRAG.uv = IN_uv; FRAG.fill = IN_fill; FRAG.fill.rgb *= IN_fill.a; FRAG.outline = IN_outline; FRAG.outline.a *= IN_outline.a; }\0"
                            as *const u8 as *const libc::c_char,
                        byte_code: 0 as *const uint8_t,
                        byte_code_size: 0,
                        entry: 0 as *const libc::c_char,
                        uniform_blocks: [
                            {
                                let mut init = sg_shader_uniform_block_desc {
                                    size: ::std::mem::size_of::<Uniforms>() as libc::c_ulong
                                        as libc::c_int,
                                    uniforms: [
                                        {
                                            let mut init = sg_shader_uniform_desc {
                                                name: b"U_vp_matrix\0" as *const u8 as *const libc::c_char,
                                                type_0: SG_UNIFORMTYPE_MAT4,
                                                array_count: 0,
                                            };
                                            init
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                        sg_shader_uniform_desc {
                                            name: 0 as *const libc::c_char,
                                            type_0: SG_UNIFORMTYPE_INVALID,
                                            array_count: 0,
                                        },
                                    ],
                                };
                                init
                            },
                            sg_shader_uniform_block_desc {
                                size: 0,
                                uniforms: [sg_shader_uniform_desc {
                                    name: 0 as *const libc::c_char,
                                    type_0: SG_UNIFORMTYPE_INVALID,
                                    array_count: 0,
                                }; 16],
                            },
                            sg_shader_uniform_block_desc {
                                size: 0,
                                uniforms: [sg_shader_uniform_desc {
                                    name: 0 as *const libc::c_char,
                                    type_0: SG_UNIFORMTYPE_INVALID,
                                    array_count: 0,
                                }; 16],
                            },
                            sg_shader_uniform_block_desc {
                                size: 0,
                                uniforms: [sg_shader_uniform_desc {
                                    name: 0 as *const libc::c_char,
                                    type_0: SG_UNIFORMTYPE_INVALID,
                                    array_count: 0,
                                }; 16],
                            },
                        ],
                        images: [sg_shader_image_desc {
                            name: 0 as *const libc::c_char,
                            type_0: _SG_IMAGETYPE_DEFAULT,
                        }; 12],
                    };
                    init
                },
                fs: {
                    let mut init = sg_shader_stage_desc {
                        source: b"#version 330\nin struct { vec2 uv; vec4 fill; vec4 outline; } FRAG; out vec4 OUT_color; void main(){ float len = length(FRAG.uv); float fw = length(fwidth(FRAG.uv)); float mask = smoothstep(-1, fw - 1, -len); float outline = 1 - fw; float outline_mask = smoothstep(outline - fw, outline, len); vec4 color = FRAG.fill + (FRAG.outline - FRAG.fill*FRAG.outline.a)*outline_mask; OUT_color = color*mask; }\0"
                            as *const u8 as *const libc::c_char,
                        byte_code: 0 as *const uint8_t,
                        byte_code_size: 0,
                        entry: 0 as *const libc::c_char,
                        uniform_blocks: [sg_shader_uniform_block_desc {
                            size: 0,
                            uniforms: [sg_shader_uniform_desc {
                                name: 0 as *const libc::c_char,
                                type_0: SG_UNIFORMTYPE_INVALID,
                                array_count: 0,
                            }; 16],
                        }; 4],
                        images: [sg_shader_image_desc {
                            name: 0 as *const libc::c_char,
                            type_0: _SG_IMAGETYPE_DEFAULT,
                        }; 12],
                    };
                    init
                },
                label: 0 as *const libc::c_char,
                _end_canary: 0,
            };
            init
        },
    );
    pipeline = sg_make_pipeline(
        &mut {
            let mut init = sg_pipeline_desc {
                _start_canary: 0,
                layout: {
                    let mut init = sg_layout_desc {
                        buffers: [sg_buffer_layout_desc {
                            stride: 0,
                            step_func: _SG_VERTEXSTEP_DEFAULT,
                            step_rate: 0,
                        }; 4],
                        attrs: [
                            {
                                let mut init = sg_vertex_attr_desc {
                                    buffer_index: 0,
                                    offset: 0 as libc::c_ulong as libc::c_int,
                                    format: SG_VERTEXFORMAT_FLOAT2,
                                };
                                init
                            },
                            {
                                let mut init = sg_vertex_attr_desc {
                                    buffer_index: 0,
                                    offset: 8 as libc::c_ulong as libc::c_int,
                                    format: SG_VERTEXFORMAT_FLOAT2,
                                };
                                init
                            },
                            {
                                let mut init = sg_vertex_attr_desc {
                                    buffer_index: 0,
                                    offset: 16 as libc::c_ulong as libc::c_int,
                                    format: SG_VERTEXFORMAT_FLOAT,
                                };
                                init
                            },
                            {
                                let mut init = sg_vertex_attr_desc {
                                    buffer_index: 0,
                                    offset: 20 as libc::c_ulong as libc::c_int,
                                    format: SG_VERTEXFORMAT_UBYTE4N,
                                };
                                init
                            },
                            {
                                let mut init = sg_vertex_attr_desc {
                                    buffer_index: 0,
                                    offset: 24 as libc::c_ulong as libc::c_int,
                                    format: SG_VERTEXFORMAT_UBYTE4N,
                                };
                                init
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                            sg_vertex_attr_desc {
                                buffer_index: 0,
                                offset: 0,
                                format: SG_VERTEXFORMAT_INVALID,
                            },
                        ],
                    };
                    init
                },
                shader: shd,
                primitive_type: _SG_PRIMITIVETYPE_DEFAULT,
                index_type: SG_INDEXTYPE_UINT16,
                depth_stencil: sg_depth_stencil_state {
                    stencil_front: sg_stencil_state {
                        fail_op: _SG_STENCILOP_DEFAULT,
                        depth_fail_op: _SG_STENCILOP_DEFAULT,
                        pass_op: _SG_STENCILOP_DEFAULT,
                        compare_func: _SG_COMPAREFUNC_DEFAULT,
                    },
                    stencil_back: sg_stencil_state {
                        fail_op: _SG_STENCILOP_DEFAULT,
                        depth_fail_op: _SG_STENCILOP_DEFAULT,
                        pass_op: _SG_STENCILOP_DEFAULT,
                        compare_func: _SG_COMPAREFUNC_DEFAULT,
                    },
                    depth_compare_func: _SG_COMPAREFUNC_DEFAULT,
                    depth_write_enabled: false,
                    stencil_enabled: false,
                    stencil_read_mask: 0,
                    stencil_write_mask: 0,
                    stencil_ref: 0,
                },
                blend: {
                    let mut init = sg_blend_state {
                        enabled: 1 as libc::c_int != 0,
                        src_factor_rgb: SG_BLENDFACTOR_ONE,
                        dst_factor_rgb: SG_BLENDFACTOR_ONE_MINUS_SRC_ALPHA,
                        op_rgb: _SG_BLENDOP_DEFAULT,
                        src_factor_alpha: _SG_BLENDFACTOR_DEFAULT,
                        dst_factor_alpha: _SG_BLENDFACTOR_DEFAULT,
                        op_alpha: _SG_BLENDOP_DEFAULT,
                        color_write_mask: 0,
                        color_attachment_count: 0,
                        color_format: _SG_PIXELFORMAT_DEFAULT,
                        depth_format: _SG_PIXELFORMAT_DEFAULT,
                        blend_color: [0.; 4],
                    };
                    init
                },
                rasterizer: sg_rasterizer_state {
                    alpha_to_coverage_enabled: false,
                    cull_mode: _SG_CULLMODE_DEFAULT,
                    face_winding: _SG_FACEWINDING_DEFAULT,
                    sample_count: 0,
                    depth_bias: 0.,
                    depth_bias_slope_scale: 0.,
                    depth_bias_clamp: 0.,
                },
                label: 0 as *const libc::c_char,
                _end_canary: 0,
            };
            init
        },
    );
}
unsafe extern "C" fn push_vertexes(
    mut vcount: size_t,
    mut index_src: *const Index,
    mut icount: size_t,
) -> *mut Vertex {
    if !(VertexCount.wrapping_add(vcount)
        <= (64 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
        && IndexCount.wrapping_add(icount)
            <= (4 as libc::c_int * (64 as libc::c_int * 1024 as libc::c_int))
                as libc::c_ulong)
    {
        cpMessage(
            b"VertexCount + vcount <= VERTEX_MAX && IndexCount + icount <= INDEX_MAX\0"
                as *const u8 as *const libc::c_char,
            b"../../demo/ChipmunkDebugDraw.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Geometry buffer full.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    let mut vertex_dst: *mut Vertex = Vertexes.as_mut_ptr().offset(VertexCount as isize);
    let mut base: size_t = VertexCount;
    VertexCount = (VertexCount as libc::c_ulong).wrapping_add(vcount) as size_t
        as size_t;
    let mut index_dst: *mut Index = Indexes.as_mut_ptr().offset(IndexCount as isize);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < icount {
        *index_dst
            .offset(
                i as isize,
            ) = (*index_src.offset(i as isize) as libc::c_int
            + base as Index as libc::c_int) as Index;
        i = i.wrapping_add(1);
        i;
    }
    IndexCount = (IndexCount as libc::c_ulong).wrapping_add(icount) as size_t as size_t;
    return vertex_dst;
}
pub unsafe extern "C" fn ChipmunkDebugDrawDot(
    mut size: cpFloat,
    mut pos: cpVect,
    mut fillColor: cpSpaceDebugColor,
) {
    let mut r: libc::c_float = (size * 0.5f32 as libc::c_double
        * ChipmunkDebugDrawPointLineScale as libc::c_double) as libc::c_float;
    let mut fill: RGBA8 = cp_to_rgba(fillColor);
    let mut vertexes: *mut Vertex = push_vertexes(
        4 as libc::c_int as size_t,
        [
            0 as libc::c_int as Index,
            1 as libc::c_int as Index,
            2 as libc::c_int as Index,
            0 as libc::c_int as Index,
            2 as libc::c_int as Index,
            3 as libc::c_int as Index,
        ]
            .as_mut_ptr(),
        6 as libc::c_int as size_t,
    );
    *vertexes
        .offset(
            0 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: pos.x as libc::c_float,
                    y: pos.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: -(1 as libc::c_int) as libc::c_float,
                    y: -(1 as libc::c_int) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: fill,
        };
        init
    };
    *vertexes
        .offset(
            1 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: pos.x as libc::c_float,
                    y: pos.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: -(1 as libc::c_int) as libc::c_float,
                    y: 1 as libc::c_int as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: fill,
        };
        init
    };
    *vertexes
        .offset(
            2 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: pos.x as libc::c_float,
                    y: pos.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: 1 as libc::c_int as libc::c_float,
                    y: 1 as libc::c_int as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: fill,
        };
        init
    };
    *vertexes
        .offset(
            3 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: pos.x as libc::c_float,
                    y: pos.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: 1 as libc::c_int as libc::c_float,
                    y: -(1 as libc::c_int) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: fill,
        };
        init
    };
}
pub unsafe extern "C" fn ChipmunkDebugDrawCircle(
    mut pos: cpVect,
    mut angle: cpFloat,
    mut radius: cpFloat,
    mut outlineColor: cpSpaceDebugColor,
    mut fillColor: cpSpaceDebugColor,
) {
    let mut r: libc::c_float = radius as libc::c_float + ChipmunkDebugDrawPointLineScale;
    let mut fill: RGBA8 = cp_to_rgba(fillColor);
    let mut outline: RGBA8 = cp_to_rgba(outlineColor);
    let mut vertexes: *mut Vertex = push_vertexes(
        4 as libc::c_int as size_t,
        [
            0 as libc::c_int as Index,
            1 as libc::c_int as Index,
            2 as libc::c_int as Index,
            0 as libc::c_int as Index,
            2 as libc::c_int as Index,
            3 as libc::c_int as Index,
        ]
            .as_mut_ptr(),
        6 as libc::c_int as size_t,
    );
    *vertexes
        .offset(
            0 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: pos.x as libc::c_float,
                    y: pos.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: -(1 as libc::c_int) as libc::c_float,
                    y: -(1 as libc::c_int) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            1 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: pos.x as libc::c_float,
                    y: pos.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: -(1 as libc::c_int) as libc::c_float,
                    y: 1 as libc::c_int as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            2 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: pos.x as libc::c_float,
                    y: pos.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: 1 as libc::c_int as libc::c_float,
                    y: 1 as libc::c_int as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            3 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: pos.x as libc::c_float,
                    y: pos.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: 1 as libc::c_int as libc::c_float,
                    y: -(1 as libc::c_int) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    ChipmunkDebugDrawSegment(
        pos,
        cpvadd(pos, cpvmult(cpvforangle(angle), 0.75f32 as libc::c_double * radius)),
        outlineColor,
    );
}
pub unsafe extern "C" fn ChipmunkDebugDrawSegment(
    mut a: cpVect,
    mut b: cpVect,
    mut color: cpSpaceDebugColor,
) {
    ChipmunkDebugDrawFatSegment(a, b, 0.0f32 as cpFloat, color, color);
}
pub unsafe extern "C" fn ChipmunkDebugDrawFatSegment(
    mut a: cpVect,
    mut b: cpVect,
    mut radius: cpFloat,
    mut outlineColor: cpSpaceDebugColor,
    mut fillColor: cpSpaceDebugColor,
) {
    static mut indexes: [Index; 18] = [
        0 as libc::c_int as Index,
        1 as libc::c_int as Index,
        2 as libc::c_int as Index,
        1 as libc::c_int as Index,
        2 as libc::c_int as Index,
        3 as libc::c_int as Index,
        2 as libc::c_int as Index,
        3 as libc::c_int as Index,
        4 as libc::c_int as Index,
        3 as libc::c_int as Index,
        4 as libc::c_int as Index,
        5 as libc::c_int as Index,
        4 as libc::c_int as Index,
        5 as libc::c_int as Index,
        6 as libc::c_int as Index,
        5 as libc::c_int as Index,
        6 as libc::c_int as Index,
        7 as libc::c_int as Index,
    ];
    let mut vertexes: *mut Vertex = push_vertexes(
        8 as libc::c_int as size_t,
        indexes.as_ptr(),
        18 as libc::c_int as size_t,
    );
    let mut t: cpVect = cpvnormalize(cpvsub(b, a));
    let mut r: libc::c_float = radius as libc::c_float + ChipmunkDebugDrawPointLineScale;
    let mut fill: RGBA8 = cp_to_rgba(fillColor);
    let mut outline: RGBA8 = cp_to_rgba(outlineColor);
    *vertexes
        .offset(
            0 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: a.x as libc::c_float,
                    y: a.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: (-t.x + t.y) as libc::c_float,
                    y: (-t.x - t.y) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            1 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: a.x as libc::c_float,
                    y: a.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: (-t.x - t.y) as libc::c_float,
                    y: (t.x - t.y) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            2 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: a.x as libc::c_float,
                    y: a.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: (-0.0f64 + t.y) as libc::c_float,
                    y: (-t.x + 0.0f64) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            3 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: a.x as libc::c_float,
                    y: a.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: (-0.0f64 - t.y) as libc::c_float,
                    y: (t.x + 0.0f64) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            4 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: b.x as libc::c_float,
                    y: b.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: (0.0f64 + t.y) as libc::c_float,
                    y: (-t.x - 0.0f64) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            5 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: b.x as libc::c_float,
                    y: b.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: (0.0f64 - t.y) as libc::c_float,
                    y: (t.x - 0.0f64) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            6 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: b.x as libc::c_float,
                    y: b.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: (t.x + t.y) as libc::c_float,
                    y: (-t.x + t.y) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
    *vertexes
        .offset(
            7 as libc::c_int as isize,
        ) = {
        let mut init = Vertex {
            pos: {
                let mut init = float2 {
                    x: b.x as libc::c_float,
                    y: b.y as libc::c_float,
                };
                init
            },
            uv: {
                let mut init = float2 {
                    x: (t.x - t.y) as libc::c_float,
                    y: (t.x + t.y) as libc::c_float,
                };
                init
            },
            r: r,
            fill: fill,
            outline: outline,
        };
        init
    };
}
pub unsafe extern "C" fn ChipmunkDebugDrawPolygon(
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut radius: cpFloat,
    mut outlineColor: cpSpaceDebugColor,
    mut fillColor: cpSpaceDebugColor,
) {
    let mut fill: RGBA8 = cp_to_rgba(fillColor);
    let mut outline: RGBA8 = cp_to_rgba(outlineColor);
    let mut indexes: [Index; 954] = [0; 954];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count - 2 as libc::c_int {
        indexes[(3 as libc::c_int * i + 0 as libc::c_int)
            as usize] = 0 as libc::c_int as Index;
        indexes[(3 as libc::c_int * i + 1 as libc::c_int)
            as usize] = (4 as libc::c_int * (i + 1 as libc::c_int)) as Index;
        indexes[(3 as libc::c_int * i + 2 as libc::c_int)
            as usize] = (4 as libc::c_int * (i + 2 as libc::c_int)) as Index;
        i += 1;
        i;
    }
    let mut cursor: *mut Index = indexes
        .as_mut_ptr()
        .offset((3 as libc::c_int * (count - 2 as libc::c_int)) as isize);
    let mut i0: libc::c_int = 0 as libc::c_int;
    while i0 < count {
        let mut i1: libc::c_int = (i0 + 1 as libc::c_int) % count;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 0 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i0 + 0 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 1 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i0 + 1 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 2 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i0 + 2 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 3 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i0 + 0 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 4 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i0 + 2 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 5 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i0 + 3 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 6 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i0 + 0 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 7 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i0 + 3 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 8 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i1 + 0 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 9 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i0 + 3 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 10 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i1 + 0 as libc::c_int) as Index;
        *cursor
            .offset(
                (12 as libc::c_int * i0 + 11 as libc::c_int) as isize,
            ) = (4 as libc::c_int * i1 + 1 as libc::c_int) as Index;
        i0 += 1;
        i0;
    }
    let mut inset: libc::c_float = -cpfmax(
        0 as libc::c_int as cpFloat,
        (2 as libc::c_int as libc::c_float * ChipmunkDebugDrawPointLineScale)
            as libc::c_double - radius,
    ) as libc::c_float;
    let mut outset: libc::c_float = radius as libc::c_float
        + ChipmunkDebugDrawPointLineScale;
    let mut r: libc::c_float = outset - inset;
    let mut vertexes: *mut Vertex = push_vertexes(
        (4 as libc::c_int * count) as size_t,
        indexes.as_mut_ptr(),
        (3 as libc::c_int * (5 as libc::c_int * count - 2 as libc::c_int)) as size_t,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < count {
        let mut v0: cpVect = *verts.offset(i_0 as isize);
        let mut v_prev: cpVect = *verts
            .offset(((i_0 + (count - 1 as libc::c_int)) % count) as isize);
        let mut v_next: cpVect = *verts
            .offset(((i_0 + (count + 1 as libc::c_int)) % count) as isize);
        let mut n1: cpVect = cpvnormalize(cpvrperp(cpvsub(v0, v_prev)));
        let mut n2: cpVect = cpvnormalize(cpvrperp(cpvsub(v_next, v0)));
        let mut of: cpVect = cpvmult(
            cpvadd(n1, n2),
            1.0f64 / (cpvdot(n1, n2) + 1.0f32 as libc::c_double),
        );
        let mut v: cpVect = cpvadd(v0, cpvmult(of, inset as cpFloat));
        *vertexes
            .offset(
                (4 as libc::c_int * i_0 + 0 as libc::c_int) as isize,
            ) = {
            let mut init = Vertex {
                pos: {
                    let mut init = float2 {
                        x: v.x as libc::c_float,
                        y: v.y as libc::c_float,
                    };
                    init
                },
                uv: {
                    let mut init = float2 { x: 0.0f32, y: 0.0f32 };
                    init
                },
                r: 0.0f32,
                fill: fill,
                outline: outline,
            };
            init
        };
        *vertexes
            .offset(
                (4 as libc::c_int * i_0 + 1 as libc::c_int) as isize,
            ) = {
            let mut init = Vertex {
                pos: {
                    let mut init = float2 {
                        x: v.x as libc::c_float,
                        y: v.y as libc::c_float,
                    };
                    init
                },
                uv: {
                    let mut init = float2 {
                        x: n1.x as libc::c_float,
                        y: n1.y as libc::c_float,
                    };
                    init
                },
                r: r,
                fill: fill,
                outline: outline,
            };
            init
        };
        *vertexes
            .offset(
                (4 as libc::c_int * i_0 + 2 as libc::c_int) as isize,
            ) = {
            let mut init = Vertex {
                pos: {
                    let mut init = float2 {
                        x: v.x as libc::c_float,
                        y: v.y as libc::c_float,
                    };
                    init
                },
                uv: {
                    let mut init = float2 {
                        x: of.x as libc::c_float,
                        y: of.y as libc::c_float,
                    };
                    init
                },
                r: r,
                fill: fill,
                outline: outline,
            };
            init
        };
        *vertexes
            .offset(
                (4 as libc::c_int * i_0 + 3 as libc::c_int) as isize,
            ) = {
            let mut init = Vertex {
                pos: {
                    let mut init = float2 {
                        x: v.x as libc::c_float,
                        y: v.y as libc::c_float,
                    };
                    init
                },
                uv: {
                    let mut init = float2 {
                        x: n2.x as libc::c_float,
                        y: n2.y as libc::c_float,
                    };
                    init
                },
                r: r,
                fill: fill,
                outline: outline,
            };
            init
        };
        i_0 += 1;
        i_0;
    }
}
pub unsafe extern "C" fn ChipmunkDebugDrawBB(
    mut bb: cpBB,
    mut color: cpSpaceDebugColor,
) {
    let mut verts: [cpVect; 4] = [
        cpv(bb.r, bb.b),
        cpv(bb.r, bb.t),
        cpv(bb.l, bb.t),
        cpv(bb.l, bb.b),
    ];
    ChipmunkDebugDrawPolygon(
        4 as libc::c_int,
        verts.as_mut_ptr(),
        0.0f32 as cpFloat,
        color,
        LAColor(0 as libc::c_int as libc::c_float, 0 as libc::c_int as libc::c_float),
    );
}
pub unsafe extern "C" fn ChipmunkDebugDrawFlushRenderer() {
    let mut t: cpTransform = ChipmunkDebugDrawVPMatrix;
    let mut uniforms: Uniforms = {
        let mut init = Uniforms {
            U_vp_matrix: [
                t.a as libc::c_float,
                t.b as libc::c_float,
                0.0f32,
                0.0f32,
                t.c as libc::c_float,
                t.d as libc::c_float,
                0.0f32,
                0.0f32,
                0.0f32,
                0.0f32,
                1.0f32,
                0.0f32,
                t.tx as libc::c_float,
                t.ty as libc::c_float,
                0.0f32,
                1.0f32,
            ],
        };
        init
    };
    sg_update_buffer(
        VertexBuffer,
        Vertexes.as_mut_ptr() as *const libc::c_void,
        VertexCount.wrapping_mul(::std::mem::size_of::<Vertex>() as libc::c_ulong)
            as libc::c_int,
    );
    sg_update_buffer(
        IndexBuffer,
        Indexes.as_mut_ptr() as *const libc::c_void,
        IndexCount.wrapping_mul(::std::mem::size_of::<Index>() as libc::c_ulong)
            as libc::c_int,
    );
    sg_apply_pipeline(pipeline);
    sg_apply_bindings(&mut bindings);
    sg_apply_uniforms(
        SG_SHADERSTAGE_VS,
        0 as libc::c_int,
        &mut uniforms as *mut Uniforms as *const libc::c_void,
        ::std::mem::size_of::<Uniforms>() as libc::c_ulong as libc::c_int,
    );
    sg_draw(0 as libc::c_int, IndexCount as libc::c_int, 1 as libc::c_int);
}
pub unsafe extern "C" fn ChipmunkDebugDrawClearRenderer() {
    IndexCount = 0 as libc::c_int as size_t;
    VertexCount = IndexCount;
}
static mut PushedVertexCount: size_t = 0;
static mut PushedIndexCount: size_t = 0;
pub unsafe extern "C" fn ChipmunkDebugDrawPushRenderer() {
    PushedVertexCount = VertexCount;
    PushedIndexCount = IndexCount;
}
pub unsafe extern "C" fn ChipmunkDebugDrawPopRenderer() {
    VertexCount = PushedVertexCount;
    IndexCount = PushedIndexCount;
}
