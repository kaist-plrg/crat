use ::libc;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct casing_suffix_context {
    pub first_char_except_ignorable: uint32_t,
    pub bits: uint32_t,
}
pub type casing_suffix_context_t = casing_suffix_context;
pub static mut unicase_empty_suffix_context: casing_suffix_context_t = {
    let mut init = casing_suffix_context {
        first_char_except_ignorable: 0xfffd as libc::c_int as uint32_t,
        bits: 0 as libc::c_int as uint32_t,
    };
    init
};
