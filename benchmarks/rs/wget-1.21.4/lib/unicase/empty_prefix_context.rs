use ::libc;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct casing_prefix_context {
    pub last_char_except_ignorable: uint32_t,
    pub last_char_normal_or_above: uint32_t,
}
pub type casing_prefix_context_t = casing_prefix_context;
pub static mut unicase_empty_prefix_context: casing_prefix_context_t = {
    let mut init = casing_prefix_context {
        last_char_except_ignorable: 0xfffd as libc::c_int as uint32_t,
        last_char_normal_or_above: 0xfffd as libc::c_int as uint32_t,
    };
    init
};
