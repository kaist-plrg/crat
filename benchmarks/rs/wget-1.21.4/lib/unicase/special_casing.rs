use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const SCC_AFTER_I: C2RustUnnamed = 5;
pub const SCC_BEFORE_DOT: C2RustUnnamed = 4;
pub const SCC_MORE_ABOVE: C2RustUnnamed = 3;
pub const SCC_AFTER_SOFT_DOTTED: C2RustUnnamed = 2;
pub const SCC_FINAL_SIGMA: C2RustUnnamed = 1;
pub const SCC_ALWAYS: C2RustUnnamed = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct special_casing_rule {
    pub code: [libc::c_char; 3],
    #[bitfield(name = "has_next", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "context", ty = "libc::c_int", bits = "1..=7")]
    pub has_next_context: [u8; 1],
    pub language: [libc::c_char; 2],
    pub upper: [libc::c_ushort; 3],
    pub lower: [libc::c_ushort; 3],
    pub title: [libc::c_ushort; 3],
    pub casefold: [libc::c_ushort; 3],
}
#[inline]
unsafe extern "C" fn gl_unicase_special_hash(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> libc::c_uint {
    static mut asso_values: [libc::c_uchar; 257] = [
        2 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        37 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        121 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        120 as libc::c_int as libc::c_uchar,
        119 as libc::c_int as libc::c_uchar,
        118 as libc::c_int as libc::c_uchar,
        117 as libc::c_int as libc::c_uchar,
        116 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        115 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        114 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        111 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        109 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        108 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        107 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        106 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        105 as libc::c_int as libc::c_uchar,
        104 as libc::c_int as libc::c_uchar,
        103 as libc::c_int as libc::c_uchar,
        102 as libc::c_int as libc::c_uchar,
        101 as libc::c_int as libc::c_uchar,
        100 as libc::c_int as libc::c_uchar,
        99 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        98 as libc::c_int as libc::c_uchar,
        97 as libc::c_int as libc::c_uchar,
        96 as libc::c_int as libc::c_uchar,
        95 as libc::c_int as libc::c_uchar,
        94 as libc::c_int as libc::c_uchar,
        93 as libc::c_int as libc::c_uchar,
        92 as libc::c_int as libc::c_uchar,
        91 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        90 as libc::c_int as libc::c_uchar,
        89 as libc::c_int as libc::c_uchar,
        88 as libc::c_int as libc::c_uchar,
        87 as libc::c_int as libc::c_uchar,
        86 as libc::c_int as libc::c_uchar,
        27 as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        85 as libc::c_int as libc::c_uchar,
        84 as libc::c_int as libc::c_uchar,
        83 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        82 as libc::c_int as libc::c_uchar,
        81 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        79 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        78 as libc::c_int as libc::c_uchar,
        77 as libc::c_int as libc::c_uchar,
        76 as libc::c_int as libc::c_uchar,
        75 as libc::c_int as libc::c_uchar,
        74 as libc::c_int as libc::c_uchar,
        73 as libc::c_int as libc::c_uchar,
        72 as libc::c_int as libc::c_uchar,
        71 as libc::c_int as libc::c_uchar,
        70 as libc::c_int as libc::c_uchar,
        69 as libc::c_int as libc::c_uchar,
        68 as libc::c_int as libc::c_uchar,
        67 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        65 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        63 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        62 as libc::c_int as libc::c_uchar,
        61 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        59 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        57 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        56 as libc::c_int as libc::c_uchar,
        55 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        55 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        53 as libc::c_int as libc::c_uchar,
        52 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        48 as libc::c_int as libc::c_uchar,
        47 as libc::c_int as libc::c_uchar,
        46 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        45 as libc::c_int as libc::c_uchar,
        44 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        46 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        42 as libc::c_int as libc::c_uchar,
        41 as libc::c_int as libc::c_uchar,
        39 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        38 as libc::c_int as libc::c_uchar,
        35 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
    ];
    return (asso_values[(*str.offset(2 as libc::c_int as isize) as libc::c_uchar
        as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
        + asso_values[*str.offset(1 as libc::c_int as isize) as libc::c_uchar as usize]
            as libc::c_int
        + asso_values[*str.offset(0 as libc::c_int as isize) as libc::c_uchar as usize]
            as libc::c_int) as libc::c_uint;
}
static mut wordlist: [special_casing_rule; 122] = [special_casing_rule {
    code: [0; 3],
    has_next_context: [0; 1],
    language: [0; 2],
    upper: [0; 3],
    lower: [0; 3],
    title: [0; 3],
    casefold: [0; 3],
}; 122];
pub unsafe extern "C" fn gl_unicase_special_lookup(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *const special_casing_rule {
    static mut lengthtable: [libc::c_uchar; 122] = [
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
    ];
    if len <= 3 as libc::c_int as libc::c_ulong
        && len >= 3 as libc::c_int as libc::c_ulong
    {
        let mut key: libc::c_uint = gl_unicase_special_hash(str, len);
        if key <= 121 as libc::c_int as libc::c_uint {
            if len == lengthtable[key as usize] as libc::c_ulong {
                let mut s: *const libc::c_char = (wordlist[key as usize].code).as_ptr();
                if *str as libc::c_int == *s as libc::c_int
                    && memcmp(
                        str.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) == 0
                {
                    return &*wordlist.as_ptr().offset(key as isize)
                        as *const special_casing_rule;
                }
            }
        }
    }
    return 0 as *const special_casing_rule;
}
unsafe extern "C" fn run_static_initializers() {
    wordlist = [
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x01\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb01 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x66 as libc::c_int as libc::c_ushort,
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x01I\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x2bc as libc::c_int as libc::c_ushort,
                    0x4e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x149 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x2bc as libc::c_int as libc::c_ushort,
                    0x4e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x2bc as libc::c_int as libc::c_ushort,
                    0x6e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\0\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x46 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb00 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x66 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x66 as libc::c_int as libc::c_ushort,
                    0x66 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0I\0"),
                language: ['l' as i32 as libc::c_char, 't' as i32 as libc::c_char],
                upper: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as libc::c_int as libc::c_uint);
            init.set_context(SCC_MORE_ABOVE as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x02\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x4c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb02 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x6c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x66 as libc::c_int as libc::c_ushort,
                    0x6c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x03\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x46 as libc::c_int as libc::c_ushort,
                    0x49 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb03 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x66 as libc::c_int as libc::c_ushort,
                    0x69 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x66 as libc::c_int as libc::c_ushort,
                    0x66 as libc::c_int as libc::c_ushort,
                    0x69 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x010\0"),
                language: ['t' as i32 as libc::c_char, 'r' as i32 as libc::c_char],
                upper: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0I\x01"),
                language: ['t' as i32 as libc::c_char, 'r' as i32 as libc::c_char],
                upper: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x131 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x131 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as libc::c_int as libc::c_uint);
            init.set_context(-(SCC_BEFORE_DOT as libc::c_int));
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0I\x02"),
                language: ['a' as i32 as libc::c_char, 'z' as i32 as libc::c_char],
                upper: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x131 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x131 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as libc::c_int as libc::c_uint);
            init.set_context(-(SCC_BEFORE_DOT as libc::c_int));
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x03\x07\0"),
                language: ['l' as i32 as libc::c_char, 't' as i32 as libc::c_char],
                upper: [
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as libc::c_int as libc::c_uint);
            init.set_context(SCC_AFTER_SOFT_DOTTED as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x010\x01"),
                language: ['a' as i32 as libc::c_char, 'z' as i32 as libc::c_char],
                upper: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x010\x02"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x05\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x53 as libc::c_int as libc::c_ushort,
                    0x54 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb05 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x53 as libc::c_int as libc::c_ushort,
                    0x74 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x73 as libc::c_int as libc::c_ushort,
                    0x74 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x03\x07\x01"),
                language: ['t' as i32 as libc::c_char, 'r' as i32 as libc::c_char],
                upper: [
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as libc::c_int as libc::c_uint);
            init.set_context(SCC_AFTER_I as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x03\x07\x02"),
                language: ['a' as i32 as libc::c_char, 'z' as i32 as libc::c_char],
                upper: [
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_AFTER_I as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0I\x04"),
                language: ['a' as i32 as libc::c_char, 'z' as i32 as libc::c_char],
                upper: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x131 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0\xCC\0"),
                language: ['l' as i32 as libc::c_char, 't' as i32 as libc::c_char],
                upper: [
                    0xcc as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0x307 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0xcc as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0xec as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xCC\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x397 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fc3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fcc as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b7 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA3\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6b as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fab as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f63 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x9E\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2e as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f96 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f26 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x03\xA3\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x3c2 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_FINAL_SIGMA as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1E\x9E\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1e9e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xdf as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1e9e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x73 as libc::c_int as libc::c_ushort,
                    0x73 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x9A\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2a as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f92 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f22 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x99\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f29 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f91 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f99 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f21 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1E\x9A\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x41 as libc::c_int as libc::c_ushort,
                    0x2be as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1e9a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x41 as libc::c_int as libc::c_ushort,
                    0x2be as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x61 as libc::c_int as libc::c_ushort,
                    0x2be as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1E\x99\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x59 as libc::c_int as libc::c_ushort,
                    0x30a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1e99 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x59 as libc::c_int as libc::c_ushort,
                    0x30a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x79 as libc::c_int as libc::c_ushort,
                    0x30a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x98\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f28 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f90 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f98 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f20 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x97\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2f as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f97 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9f as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f27 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1E\x98\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x57 as libc::c_int as libc::c_ushort,
                    0x30a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1e98 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x57 as libc::c_int as libc::c_ushort,
                    0x30a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x77 as libc::c_int as libc::c_ushort,
                    0x30a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1E\x97\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x54 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1e97 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x54 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x74 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x96\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2e as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f96 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f26 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x90\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f28 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f90 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f98 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f20 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1E\x96\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x48 as libc::c_int as libc::c_ushort,
                    0x331 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1e96 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x48 as libc::c_int as libc::c_ushort,
                    0x331 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x68 as libc::c_int as libc::c_ushort,
                    0x331 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x03\x90\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x390 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x87\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0f as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f87 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8f as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f07 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0i\0"),
                language: ['t' as i32 as libc::c_char, 'r' as i32 as libc::c_char],
                upper: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xFC\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a9 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1ff3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1ffc as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c9 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x04\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x46 as libc::c_int as libc::c_ushort,
                    0x4c as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb04 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x46 as libc::c_int as libc::c_ushort,
                    0x66 as libc::c_int as libc::c_ushort,
                    0x6c as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x66 as libc::c_int as libc::c_ushort,
                    0x66 as libc::c_int as libc::c_ushort,
                    0x6c as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xF7\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a9 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1ff7 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a9 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0x345 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c9 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0i\x01"),
                language: ['a' as i32 as libc::c_char, 'z' as i32 as libc::c_char],
                upper: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x130 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0I\x03"),
                language: ['t' as i32 as libc::c_char, 'r' as i32 as libc::c_char],
                upper: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x49 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x131 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(1 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xF6\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a9 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1ff6 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a9 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c9 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xF4\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x38f as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1ff4 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x38f as libc::c_int as libc::c_ushort,
                    0x345 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3ce as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x05\x87\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x535 as libc::c_int as libc::c_ushort,
                    0x552 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x587 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x535 as libc::c_int as libc::c_ushort,
                    0x582 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x565 as libc::c_int as libc::c_ushort,
                    0x582 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xF3\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a9 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1ff3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1ffc as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c9 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xF2\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1ffa as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1ff2 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1ffa as libc::c_int as libc::c_ushort,
                    0x345 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f7c as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x01\xF0\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x4a as libc::c_int as libc::c_ushort,
                    0x30c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x4a as libc::c_int as libc::c_ushort,
                    0x30c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x6a as libc::c_int as libc::c_ushort,
                    0x30c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xE7\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fe7 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xE6\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fe6 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xE4\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a1 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fe4 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a1 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c1 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xE3\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fe3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xE2\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fe2 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0\xDF\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x53 as libc::c_int as libc::c_ushort,
                    0x53 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xdf as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x53 as libc::c_int as libc::c_ushort,
                    0x73 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x73 as libc::c_int as libc::c_ushort,
                    0x73 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xD7\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fd7 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xD6\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fd6 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xD3\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fd3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xD2\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fd2 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x399 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0\xCD\0"),
                language: ['l' as i32 as libc::c_char, 't' as i32 as libc::c_char],
                upper: [
                    0xcd as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0x307 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0xcd as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0xed as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xC7\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x397 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fc7 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x397 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0x345 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b7 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xC6\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x397 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fc6 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x397 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b7 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xC4\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x389 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fc4 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x389 as libc::c_int as libc::c_ushort,
                    0x345 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3ae as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xC3\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x397 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fc3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fcc as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b7 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xC2\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1fca as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fc2 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fca as libc::c_int as libc::c_ushort,
                    0x345 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f74 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xBC\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x391 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fb3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fbc as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b1 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xB7\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x391 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fb7 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x391 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0x345 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b1 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xB6\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x391 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fb6 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x391 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b1 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xB4\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x386 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fb4 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x386 as libc::c_int as libc::c_ushort,
                    0x345 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3ac as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xB3\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x391 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fb3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fbc as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3b1 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xB2\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1fba as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fb2 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fba as libc::c_int as libc::c_ushort,
                    0x345 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f70 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x03\xB0\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x3b0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as libc::c_int as libc::c_ushort,
                    0x308 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xAF\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6f as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa7 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1faf as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f67 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xAE\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6e as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa6 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fae as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f66 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xAD\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6d as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa5 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fad as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f65 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xAC\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6c as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa4 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fac as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f64 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xAB\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6b as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa3 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fab as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f63 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xAA\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6a as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa2 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1faa as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f62 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA9\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f69 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa1 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fa9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f61 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA8\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f68 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fa8 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f60 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA7\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6f as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa7 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1faf as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f67 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA6\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6e as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa6 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fae as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f66 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA5\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6d as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa5 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fad as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f65 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA4\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6c as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa4 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fac as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f64 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA2\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f6a as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa2 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1faa as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f62 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA1\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f69 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa1 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fa9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f61 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\xA0\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f68 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1fa0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1fa8 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f60 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x9F\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2f as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f97 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9f as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f27 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x9D\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2d as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f95 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9d as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f25 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x9C\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2c as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f94 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f24 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x9B\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2b as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f93 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9b as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f23 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x95\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2d as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f95 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9d as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f25 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x94\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2c as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f94 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f24 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x93\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2b as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f93 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9b as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f23 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x92\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f2a as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f92 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f9a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f22 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x91\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f29 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f91 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f99 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f21 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x8F\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0f as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f87 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8f as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f07 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x8E\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0e as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f86 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f06 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x8D\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0d as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f85 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8d as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f05 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x8C\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0c as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f84 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f04 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x8B\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0b as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f83 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8b as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f03 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x8A\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0a as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f82 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f02 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x89\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f09 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f81 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f89 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f01 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x88\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f08 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f80 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f88 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f00 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x86\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0e as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f86 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f06 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x85\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0d as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f85 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8d as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f05 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x84\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0c as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f84 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8c as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f04 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x83\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0b as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f83 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8b as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f03 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x82\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f0a as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f82 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f8a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f02 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x81\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f09 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f81 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f89 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f01 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1F\x80\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x1f08 as libc::c_int as libc::c_ushort,
                    0x399 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f80 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x1f88 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x1f00 as libc::c_int as libc::c_ushort,
                    0x3b9 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1FV\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f56 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0x342 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1FT\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f54 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0x301 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1FR\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f52 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0x300 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x1FP\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x1f50 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x3a5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x3c5 as libc::c_int as libc::c_ushort,
                    0x313 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\0J\0"),
                language: ['l' as i32 as libc::c_char, 't' as i32 as libc::c_char],
                upper: [
                    0x4a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x6a as libc::c_int as libc::c_ushort,
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x4a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x6a as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_MORE_ABOVE as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x01.\0"),
                language: ['l' as i32 as libc::c_char, 't' as i32 as libc::c_char],
                upper: [
                    0x12e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x12f as libc::c_int as libc::c_ushort,
                    0x307 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x12e as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x12f as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_MORE_ABOVE as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\x01(\0"),
                language: ['l' as i32 as libc::c_char, 't' as i32 as libc::c_char],
                upper: [
                    0x128 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0x69 as libc::c_int as libc::c_ushort,
                    0x307 as libc::c_int as libc::c_ushort,
                    0x303 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x128 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x129 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x17\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x544 as libc::c_int as libc::c_ushort,
                    0x53d as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb17 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x544 as libc::c_int as libc::c_ushort,
                    0x56d as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x574 as libc::c_int as libc::c_ushort,
                    0x56d as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x16\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x54e as libc::c_int as libc::c_ushort,
                    0x546 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb16 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x54e as libc::c_int as libc::c_ushort,
                    0x576 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x57e as libc::c_int as libc::c_ushort,
                    0x576 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x15\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x544 as libc::c_int as libc::c_ushort,
                    0x53b as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb15 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x544 as libc::c_int as libc::c_ushort,
                    0x56b as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x574 as libc::c_int as libc::c_ushort,
                    0x56b as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x14\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x544 as libc::c_int as libc::c_ushort,
                    0x535 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb14 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x544 as libc::c_int as libc::c_ushort,
                    0x565 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x574 as libc::c_int as libc::c_ushort,
                    0x565 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x13\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x544 as libc::c_int as libc::c_ushort,
                    0x546 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb13 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x544 as libc::c_int as libc::c_ushort,
                    0x576 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x574 as libc::c_int as libc::c_ushort,
                    0x576 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
        {
            let mut init = special_casing_rule {
                has_next_context: [0; 1],
                code: *::std::mem::transmute::<
                    &[u8; 3],
                    &mut [libc::c_char; 3],
                >(b"\xFB\x06\0"),
                language: ['\0' as i32 as libc::c_char, '\0' as i32 as libc::c_char],
                upper: [
                    0x53 as libc::c_int as libc::c_ushort,
                    0x54 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                lower: [
                    0xfb06 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                title: [
                    0x53 as libc::c_int as libc::c_ushort,
                    0x74 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
                casefold: [
                    0x73 as libc::c_int as libc::c_ushort,
                    0x74 as libc::c_int as libc::c_ushort,
                    0 as libc::c_int as libc::c_ushort,
                ],
            };
            init.set_has_next(0 as libc::c_int as libc::c_uint);
            init.set_context(SCC_ALWAYS as libc::c_int);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
