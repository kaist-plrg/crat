use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des_ctx {
    pub key: [uint32_t; 32],
}
static mut des_keymap: [uint32_t; 512] = [
    0x2080008 as libc::c_int as uint32_t,
    0x2082000 as libc::c_int as uint32_t,
    0x2008 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x2002000 as libc::c_int as uint32_t,
    0x80008 as libc::c_int as uint32_t,
    0x2080000 as libc::c_int as uint32_t,
    0x2082008 as libc::c_int as uint32_t,
    0x8 as libc::c_int as uint32_t,
    0x2000000 as libc::c_int as uint32_t,
    0x82000 as libc::c_int as uint32_t,
    0x2008 as libc::c_int as uint32_t,
    0x82008 as libc::c_int as uint32_t,
    0x2002008 as libc::c_int as uint32_t,
    0x2000008 as libc::c_int as uint32_t,
    0x2080000 as libc::c_int as uint32_t,
    0x2000 as libc::c_int as uint32_t,
    0x82008 as libc::c_int as uint32_t,
    0x80008 as libc::c_int as uint32_t,
    0x2002000 as libc::c_int as uint32_t,
    0x2082008 as libc::c_int as uint32_t,
    0x2000008 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x82000 as libc::c_int as uint32_t,
    0x2000000 as libc::c_int as uint32_t,
    0x80000 as libc::c_int as uint32_t,
    0x2002008 as libc::c_int as uint32_t,
    0x2080008 as libc::c_int as uint32_t,
    0x80000 as libc::c_int as uint32_t,
    0x2000 as libc::c_int as uint32_t,
    0x2082000 as libc::c_int as uint32_t,
    0x8 as libc::c_int as uint32_t,
    0x80000 as libc::c_int as uint32_t,
    0x2000 as libc::c_int as uint32_t,
    0x2000008 as libc::c_int as uint32_t,
    0x2082008 as libc::c_int as uint32_t,
    0x2008 as libc::c_int as uint32_t,
    0x2000000 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x82000 as libc::c_int as uint32_t,
    0x2080008 as libc::c_int as uint32_t,
    0x2002008 as libc::c_int as uint32_t,
    0x2002000 as libc::c_int as uint32_t,
    0x80008 as libc::c_int as uint32_t,
    0x2082000 as libc::c_int as uint32_t,
    0x8 as libc::c_int as uint32_t,
    0x80008 as libc::c_int as uint32_t,
    0x2002000 as libc::c_int as uint32_t,
    0x2082008 as libc::c_int as uint32_t,
    0x80000 as libc::c_int as uint32_t,
    0x2080000 as libc::c_int as uint32_t,
    0x2000008 as libc::c_int as uint32_t,
    0x82000 as libc::c_int as uint32_t,
    0x2008 as libc::c_int as uint32_t,
    0x2002008 as libc::c_int as uint32_t,
    0x2080000 as libc::c_int as uint32_t,
    0x8 as libc::c_int as uint32_t,
    0x2082000 as libc::c_int as uint32_t,
    0x82008 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x2000000 as libc::c_int as uint32_t,
    0x2080008 as libc::c_int as uint32_t,
    0x2000 as libc::c_int as uint32_t,
    0x82008 as libc::c_int as uint32_t,
    0x8000004 as libc::c_int as uint32_t,
    0x20004 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x8020200 as libc::c_int as uint32_t,
    0x20004 as libc::c_int as uint32_t,
    0x200 as libc::c_int as uint32_t,
    0x8000204 as libc::c_int as uint32_t,
    0x20000 as libc::c_int as uint32_t,
    0x204 as libc::c_int as uint32_t,
    0x8020204 as libc::c_int as uint32_t,
    0x20200 as libc::c_int as uint32_t,
    0x8000000 as libc::c_int as uint32_t,
    0x8000200 as libc::c_int as uint32_t,
    0x8000004 as libc::c_int as uint32_t,
    0x8020000 as libc::c_int as uint32_t,
    0x20204 as libc::c_int as uint32_t,
    0x20000 as libc::c_int as uint32_t,
    0x8000204 as libc::c_int as uint32_t,
    0x8020004 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x200 as libc::c_int as uint32_t,
    0x4 as libc::c_int as uint32_t,
    0x8020200 as libc::c_int as uint32_t,
    0x8020004 as libc::c_int as uint32_t,
    0x8020204 as libc::c_int as uint32_t,
    0x8020000 as libc::c_int as uint32_t,
    0x8000000 as libc::c_int as uint32_t,
    0x204 as libc::c_int as uint32_t,
    0x4 as libc::c_int as uint32_t,
    0x20200 as libc::c_int as uint32_t,
    0x20204 as libc::c_int as uint32_t,
    0x8000200 as libc::c_int as uint32_t,
    0x204 as libc::c_int as uint32_t,
    0x8000000 as libc::c_int as uint32_t,
    0x8000200 as libc::c_int as uint32_t,
    0x20204 as libc::c_int as uint32_t,
    0x8020200 as libc::c_int as uint32_t,
    0x20004 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x8000200 as libc::c_int as uint32_t,
    0x8000000 as libc::c_int as uint32_t,
    0x200 as libc::c_int as uint32_t,
    0x8020004 as libc::c_int as uint32_t,
    0x20000 as libc::c_int as uint32_t,
    0x20004 as libc::c_int as uint32_t,
    0x8020204 as libc::c_int as uint32_t,
    0x20200 as libc::c_int as uint32_t,
    0x4 as libc::c_int as uint32_t,
    0x8020204 as libc::c_int as uint32_t,
    0x20200 as libc::c_int as uint32_t,
    0x20000 as libc::c_int as uint32_t,
    0x8000204 as libc::c_int as uint32_t,
    0x8000004 as libc::c_int as uint32_t,
    0x8020000 as libc::c_int as uint32_t,
    0x20204 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x200 as libc::c_int as uint32_t,
    0x8000004 as libc::c_int as uint32_t,
    0x8000204 as libc::c_int as uint32_t,
    0x8020200 as libc::c_int as uint32_t,
    0x8020000 as libc::c_int as uint32_t,
    0x204 as libc::c_int as uint32_t,
    0x4 as libc::c_int as uint32_t,
    0x8020004 as libc::c_int as uint32_t,
    0x80040100 as libc::c_uint,
    0x1000100 as libc::c_int as uint32_t,
    0x80000000 as libc::c_uint,
    0x81040100 as libc::c_uint,
    0 as libc::c_int as uint32_t,
    0x1040000 as libc::c_int as uint32_t,
    0x81000100 as libc::c_uint,
    0x80040000 as libc::c_uint,
    0x1040100 as libc::c_int as uint32_t,
    0x81000000 as libc::c_uint,
    0x1000000 as libc::c_int as uint32_t,
    0x80000100 as libc::c_uint,
    0x81000000 as libc::c_uint,
    0x80040100 as libc::c_uint,
    0x40000 as libc::c_int as uint32_t,
    0x1000000 as libc::c_int as uint32_t,
    0x81040000 as libc::c_uint,
    0x40100 as libc::c_int as uint32_t,
    0x100 as libc::c_int as uint32_t,
    0x80000000 as libc::c_uint,
    0x40100 as libc::c_int as uint32_t,
    0x81000100 as libc::c_uint,
    0x1040000 as libc::c_int as uint32_t,
    0x100 as libc::c_int as uint32_t,
    0x80000100 as libc::c_uint,
    0 as libc::c_int as uint32_t,
    0x80040000 as libc::c_uint,
    0x1040100 as libc::c_int as uint32_t,
    0x1000100 as libc::c_int as uint32_t,
    0x81040000 as libc::c_uint,
    0x81040100 as libc::c_uint,
    0x40000 as libc::c_int as uint32_t,
    0x81040000 as libc::c_uint,
    0x80000100 as libc::c_uint,
    0x40000 as libc::c_int as uint32_t,
    0x81000000 as libc::c_uint,
    0x40100 as libc::c_int as uint32_t,
    0x1000100 as libc::c_int as uint32_t,
    0x80000000 as libc::c_uint,
    0x1040000 as libc::c_int as uint32_t,
    0x81000100 as libc::c_uint,
    0 as libc::c_int as uint32_t,
    0x100 as libc::c_int as uint32_t,
    0x80040000 as libc::c_uint,
    0 as libc::c_int as uint32_t,
    0x81040000 as libc::c_uint,
    0x1040100 as libc::c_int as uint32_t,
    0x100 as libc::c_int as uint32_t,
    0x1000000 as libc::c_int as uint32_t,
    0x81040100 as libc::c_uint,
    0x80040100 as libc::c_uint,
    0x40000 as libc::c_int as uint32_t,
    0x81040100 as libc::c_uint,
    0x80000000 as libc::c_uint,
    0x1000100 as libc::c_int as uint32_t,
    0x80040100 as libc::c_uint,
    0x80040000 as libc::c_uint,
    0x40100 as libc::c_int as uint32_t,
    0x1040000 as libc::c_int as uint32_t,
    0x81000100 as libc::c_uint,
    0x80000100 as libc::c_uint,
    0x1000000 as libc::c_int as uint32_t,
    0x81000000 as libc::c_uint,
    0x1040100 as libc::c_int as uint32_t,
    0x4010801 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x10800 as libc::c_int as uint32_t,
    0x4010000 as libc::c_int as uint32_t,
    0x4000001 as libc::c_int as uint32_t,
    0x801 as libc::c_int as uint32_t,
    0x4000800 as libc::c_int as uint32_t,
    0x10800 as libc::c_int as uint32_t,
    0x800 as libc::c_int as uint32_t,
    0x4010001 as libc::c_int as uint32_t,
    0x1 as libc::c_int as uint32_t,
    0x4000800 as libc::c_int as uint32_t,
    0x10001 as libc::c_int as uint32_t,
    0x4010800 as libc::c_int as uint32_t,
    0x4010000 as libc::c_int as uint32_t,
    0x1 as libc::c_int as uint32_t,
    0x10000 as libc::c_int as uint32_t,
    0x4000801 as libc::c_int as uint32_t,
    0x4010001 as libc::c_int as uint32_t,
    0x800 as libc::c_int as uint32_t,
    0x10801 as libc::c_int as uint32_t,
    0x4000000 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x10001 as libc::c_int as uint32_t,
    0x4000801 as libc::c_int as uint32_t,
    0x10801 as libc::c_int as uint32_t,
    0x4010800 as libc::c_int as uint32_t,
    0x4000001 as libc::c_int as uint32_t,
    0x4000000 as libc::c_int as uint32_t,
    0x10000 as libc::c_int as uint32_t,
    0x801 as libc::c_int as uint32_t,
    0x4010801 as libc::c_int as uint32_t,
    0x10001 as libc::c_int as uint32_t,
    0x4010800 as libc::c_int as uint32_t,
    0x4000800 as libc::c_int as uint32_t,
    0x10801 as libc::c_int as uint32_t,
    0x4010801 as libc::c_int as uint32_t,
    0x10001 as libc::c_int as uint32_t,
    0x4000001 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x4000000 as libc::c_int as uint32_t,
    0x801 as libc::c_int as uint32_t,
    0x10000 as libc::c_int as uint32_t,
    0x4010001 as libc::c_int as uint32_t,
    0x800 as libc::c_int as uint32_t,
    0x4000000 as libc::c_int as uint32_t,
    0x10801 as libc::c_int as uint32_t,
    0x4000801 as libc::c_int as uint32_t,
    0x4010800 as libc::c_int as uint32_t,
    0x800 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x4000001 as libc::c_int as uint32_t,
    0x1 as libc::c_int as uint32_t,
    0x4010801 as libc::c_int as uint32_t,
    0x10800 as libc::c_int as uint32_t,
    0x4010000 as libc::c_int as uint32_t,
    0x4010001 as libc::c_int as uint32_t,
    0x10000 as libc::c_int as uint32_t,
    0x801 as libc::c_int as uint32_t,
    0x4000800 as libc::c_int as uint32_t,
    0x4000801 as libc::c_int as uint32_t,
    0x1 as libc::c_int as uint32_t,
    0x4010000 as libc::c_int as uint32_t,
    0x10800 as libc::c_int as uint32_t,
    0x400 as libc::c_int as uint32_t,
    0x20 as libc::c_int as uint32_t,
    0x100020 as libc::c_int as uint32_t,
    0x40100000 as libc::c_int as uint32_t,
    0x40100420 as libc::c_int as uint32_t,
    0x40000400 as libc::c_int as uint32_t,
    0x420 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x100000 as libc::c_int as uint32_t,
    0x40100020 as libc::c_int as uint32_t,
    0x40000020 as libc::c_int as uint32_t,
    0x100400 as libc::c_int as uint32_t,
    0x40000000 as libc::c_int as uint32_t,
    0x100420 as libc::c_int as uint32_t,
    0x100400 as libc::c_int as uint32_t,
    0x40000020 as libc::c_int as uint32_t,
    0x40100020 as libc::c_int as uint32_t,
    0x400 as libc::c_int as uint32_t,
    0x40000400 as libc::c_int as uint32_t,
    0x40100420 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x100020 as libc::c_int as uint32_t,
    0x40100000 as libc::c_int as uint32_t,
    0x420 as libc::c_int as uint32_t,
    0x40100400 as libc::c_int as uint32_t,
    0x40000420 as libc::c_int as uint32_t,
    0x100420 as libc::c_int as uint32_t,
    0x40000000 as libc::c_int as uint32_t,
    0x40000420 as libc::c_int as uint32_t,
    0x40100400 as libc::c_int as uint32_t,
    0x20 as libc::c_int as uint32_t,
    0x100000 as libc::c_int as uint32_t,
    0x40000420 as libc::c_int as uint32_t,
    0x100400 as libc::c_int as uint32_t,
    0x40100400 as libc::c_int as uint32_t,
    0x40000020 as libc::c_int as uint32_t,
    0x400 as libc::c_int as uint32_t,
    0x20 as libc::c_int as uint32_t,
    0x100000 as libc::c_int as uint32_t,
    0x40100400 as libc::c_int as uint32_t,
    0x40100020 as libc::c_int as uint32_t,
    0x40000420 as libc::c_int as uint32_t,
    0x420 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x20 as libc::c_int as uint32_t,
    0x40100000 as libc::c_int as uint32_t,
    0x40000000 as libc::c_int as uint32_t,
    0x100020 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x40100020 as libc::c_int as uint32_t,
    0x100020 as libc::c_int as uint32_t,
    0x420 as libc::c_int as uint32_t,
    0x40000020 as libc::c_int as uint32_t,
    0x400 as libc::c_int as uint32_t,
    0x40100420 as libc::c_int as uint32_t,
    0x100000 as libc::c_int as uint32_t,
    0x100420 as libc::c_int as uint32_t,
    0x40000000 as libc::c_int as uint32_t,
    0x40000400 as libc::c_int as uint32_t,
    0x40100420 as libc::c_int as uint32_t,
    0x40100000 as libc::c_int as uint32_t,
    0x100420 as libc::c_int as uint32_t,
    0x100400 as libc::c_int as uint32_t,
    0x40000400 as libc::c_int as uint32_t,
    0x800000 as libc::c_int as uint32_t,
    0x1000 as libc::c_int as uint32_t,
    0x40 as libc::c_int as uint32_t,
    0x801042 as libc::c_int as uint32_t,
    0x801002 as libc::c_int as uint32_t,
    0x800040 as libc::c_int as uint32_t,
    0x1042 as libc::c_int as uint32_t,
    0x801000 as libc::c_int as uint32_t,
    0x1000 as libc::c_int as uint32_t,
    0x2 as libc::c_int as uint32_t,
    0x800002 as libc::c_int as uint32_t,
    0x1040 as libc::c_int as uint32_t,
    0x800042 as libc::c_int as uint32_t,
    0x801002 as libc::c_int as uint32_t,
    0x801040 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x1040 as libc::c_int as uint32_t,
    0x800000 as libc::c_int as uint32_t,
    0x1002 as libc::c_int as uint32_t,
    0x42 as libc::c_int as uint32_t,
    0x800040 as libc::c_int as uint32_t,
    0x1042 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x800002 as libc::c_int as uint32_t,
    0x2 as libc::c_int as uint32_t,
    0x800042 as libc::c_int as uint32_t,
    0x801042 as libc::c_int as uint32_t,
    0x1002 as libc::c_int as uint32_t,
    0x801000 as libc::c_int as uint32_t,
    0x40 as libc::c_int as uint32_t,
    0x42 as libc::c_int as uint32_t,
    0x801040 as libc::c_int as uint32_t,
    0x801040 as libc::c_int as uint32_t,
    0x800042 as libc::c_int as uint32_t,
    0x1002 as libc::c_int as uint32_t,
    0x801000 as libc::c_int as uint32_t,
    0x1000 as libc::c_int as uint32_t,
    0x2 as libc::c_int as uint32_t,
    0x800002 as libc::c_int as uint32_t,
    0x800040 as libc::c_int as uint32_t,
    0x800000 as libc::c_int as uint32_t,
    0x1040 as libc::c_int as uint32_t,
    0x801042 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x1042 as libc::c_int as uint32_t,
    0x800000 as libc::c_int as uint32_t,
    0x40 as libc::c_int as uint32_t,
    0x1002 as libc::c_int as uint32_t,
    0x800042 as libc::c_int as uint32_t,
    0x40 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x801042 as libc::c_int as uint32_t,
    0x801002 as libc::c_int as uint32_t,
    0x801040 as libc::c_int as uint32_t,
    0x42 as libc::c_int as uint32_t,
    0x1000 as libc::c_int as uint32_t,
    0x1040 as libc::c_int as uint32_t,
    0x801002 as libc::c_int as uint32_t,
    0x800040 as libc::c_int as uint32_t,
    0x42 as libc::c_int as uint32_t,
    0x2 as libc::c_int as uint32_t,
    0x1042 as libc::c_int as uint32_t,
    0x801000 as libc::c_int as uint32_t,
    0x800002 as libc::c_int as uint32_t,
    0x10400000 as libc::c_int as uint32_t,
    0x404010 as libc::c_int as uint32_t,
    0x10 as libc::c_int as uint32_t,
    0x10400010 as libc::c_int as uint32_t,
    0x10004000 as libc::c_int as uint32_t,
    0x400000 as libc::c_int as uint32_t,
    0x10400010 as libc::c_int as uint32_t,
    0x4010 as libc::c_int as uint32_t,
    0x400010 as libc::c_int as uint32_t,
    0x4000 as libc::c_int as uint32_t,
    0x404000 as libc::c_int as uint32_t,
    0x10000000 as libc::c_int as uint32_t,
    0x10404010 as libc::c_int as uint32_t,
    0x10000010 as libc::c_int as uint32_t,
    0x10000000 as libc::c_int as uint32_t,
    0x10404000 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x10004000 as libc::c_int as uint32_t,
    0x404010 as libc::c_int as uint32_t,
    0x10 as libc::c_int as uint32_t,
    0x10000010 as libc::c_int as uint32_t,
    0x10404010 as libc::c_int as uint32_t,
    0x4000 as libc::c_int as uint32_t,
    0x10400000 as libc::c_int as uint32_t,
    0x10404000 as libc::c_int as uint32_t,
    0x400010 as libc::c_int as uint32_t,
    0x10004010 as libc::c_int as uint32_t,
    0x404000 as libc::c_int as uint32_t,
    0x4010 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x400000 as libc::c_int as uint32_t,
    0x10004010 as libc::c_int as uint32_t,
    0x404010 as libc::c_int as uint32_t,
    0x10 as libc::c_int as uint32_t,
    0x10000000 as libc::c_int as uint32_t,
    0x4000 as libc::c_int as uint32_t,
    0x10000010 as libc::c_int as uint32_t,
    0x10004000 as libc::c_int as uint32_t,
    0x404000 as libc::c_int as uint32_t,
    0x10400010 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x404010 as libc::c_int as uint32_t,
    0x4010 as libc::c_int as uint32_t,
    0x10404000 as libc::c_int as uint32_t,
    0x10004000 as libc::c_int as uint32_t,
    0x400000 as libc::c_int as uint32_t,
    0x10404010 as libc::c_int as uint32_t,
    0x10000000 as libc::c_int as uint32_t,
    0x10004010 as libc::c_int as uint32_t,
    0x10400000 as libc::c_int as uint32_t,
    0x400000 as libc::c_int as uint32_t,
    0x10404010 as libc::c_int as uint32_t,
    0x4000 as libc::c_int as uint32_t,
    0x400010 as libc::c_int as uint32_t,
    0x10400010 as libc::c_int as uint32_t,
    0x4010 as libc::c_int as uint32_t,
    0x400010 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x10404000 as libc::c_int as uint32_t,
    0x10000010 as libc::c_int as uint32_t,
    0x10400000 as libc::c_int as uint32_t,
    0x10004010 as libc::c_int as uint32_t,
    0x10 as libc::c_int as uint32_t,
    0x404000 as libc::c_int as uint32_t,
    0x208080 as libc::c_int as uint32_t,
    0x8000 as libc::c_int as uint32_t,
    0x20200000 as libc::c_int as uint32_t,
    0x20208080 as libc::c_int as uint32_t,
    0x200000 as libc::c_int as uint32_t,
    0x20008080 as libc::c_int as uint32_t,
    0x20008000 as libc::c_int as uint32_t,
    0x20200000 as libc::c_int as uint32_t,
    0x20008080 as libc::c_int as uint32_t,
    0x208080 as libc::c_int as uint32_t,
    0x208000 as libc::c_int as uint32_t,
    0x20000080 as libc::c_int as uint32_t,
    0x20200080 as libc::c_int as uint32_t,
    0x200000 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x20008000 as libc::c_int as uint32_t,
    0x8000 as libc::c_int as uint32_t,
    0x20000000 as libc::c_int as uint32_t,
    0x200080 as libc::c_int as uint32_t,
    0x8080 as libc::c_int as uint32_t,
    0x20208080 as libc::c_int as uint32_t,
    0x208000 as libc::c_int as uint32_t,
    0x20000080 as libc::c_int as uint32_t,
    0x200080 as libc::c_int as uint32_t,
    0x20000000 as libc::c_int as uint32_t,
    0x80 as libc::c_int as uint32_t,
    0x8080 as libc::c_int as uint32_t,
    0x20208000 as libc::c_int as uint32_t,
    0x80 as libc::c_int as uint32_t,
    0x20200080 as libc::c_int as uint32_t,
    0x20208000 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x20208080 as libc::c_int as uint32_t,
    0x200080 as libc::c_int as uint32_t,
    0x20008000 as libc::c_int as uint32_t,
    0x208080 as libc::c_int as uint32_t,
    0x8000 as libc::c_int as uint32_t,
    0x20000080 as libc::c_int as uint32_t,
    0x200080 as libc::c_int as uint32_t,
    0x20208000 as libc::c_int as uint32_t,
    0x80 as libc::c_int as uint32_t,
    0x8080 as libc::c_int as uint32_t,
    0x20200000 as libc::c_int as uint32_t,
    0x20008080 as libc::c_int as uint32_t,
    0x20000000 as libc::c_int as uint32_t,
    0x20200000 as libc::c_int as uint32_t,
    0x208000 as libc::c_int as uint32_t,
    0x20208080 as libc::c_int as uint32_t,
    0x8080 as libc::c_int as uint32_t,
    0x208000 as libc::c_int as uint32_t,
    0x20200080 as libc::c_int as uint32_t,
    0x200000 as libc::c_int as uint32_t,
    0x20000080 as libc::c_int as uint32_t,
    0x20008000 as libc::c_int as uint32_t,
    0 as libc::c_int as uint32_t,
    0x8000 as libc::c_int as uint32_t,
    0x200000 as libc::c_int as uint32_t,
    0x20200080 as libc::c_int as uint32_t,
    0x208080 as libc::c_int as uint32_t,
    0x20000000 as libc::c_int as uint32_t,
    0x20208000 as libc::c_int as uint32_t,
    0x80 as libc::c_int as uint32_t,
    0x20008080 as libc::c_int as uint32_t,
];
static mut rotors: [uint8_t; 768] = [
    34 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    48 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    47 as libc::c_int as uint8_t,
    38 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    37 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    27 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    55 as libc::c_int as uint8_t,
    39 as libc::c_int as uint8_t,
    40 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    25 as libc::c_int as uint8_t,
    34 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    46 as libc::c_int as uint8_t,
    26 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    29 as libc::c_int as uint8_t,
    23 as libc::c_int as uint8_t,
    17 as libc::c_int as uint8_t,
    42 as libc::c_int as uint8_t,
    50 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    30 as libc::c_int as uint8_t,
    35 as libc::c_int as uint8_t,
    43 as libc::c_int as uint8_t,
    24 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    31 as libc::c_int as uint8_t,
    41 as libc::c_int as uint8_t,
    52 as libc::c_int as uint8_t,
    19 as libc::c_int as uint8_t,
    32 as libc::c_int as uint8_t,
    54 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    18 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    20 as libc::c_int as uint8_t,
    53 as libc::c_int as uint8_t,
    33 as libc::c_int as uint8_t,
    49 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    21 as libc::c_int as uint8_t,
    22 as libc::c_int as uint8_t,
    16 as libc::c_int as uint8_t,
    44 as libc::c_int as uint8_t,
    28 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    51 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    36 as libc::c_int as uint8_t,
];
unsafe extern "C" fn DesSmallFipsEncrypt(
    mut d: *mut uint8_t,
    mut r: *const uint32_t,
    mut s: *const uint8_t,
) {
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut z: uint32_t = 0;
    x = *s.offset(7 as libc::c_int as isize) as uint32_t;
    x <<= 8 as libc::c_int;
    x |= *s.offset(6 as libc::c_int as isize) as libc::c_uint;
    x <<= 8 as libc::c_int;
    x |= *s.offset(5 as libc::c_int as isize) as libc::c_uint;
    x <<= 8 as libc::c_int;
    x |= *s.offset(4 as libc::c_int as isize) as libc::c_uint;
    y = *s.offset(3 as libc::c_int as isize) as uint32_t;
    y <<= 8 as libc::c_int;
    y |= *s.offset(2 as libc::c_int as isize) as libc::c_uint;
    y <<= 8 as libc::c_int;
    y |= *s.offset(1 as libc::c_int as isize) as libc::c_uint;
    y <<= 8 as libc::c_int;
    y |= *s.offset(0 as libc::c_int as isize) as libc::c_uint;
    z = ((x >> 0o4 as libc::c_int ^ y) as libc::c_long & 0xf0f0f0f as libc::c_long)
        as uint32_t;
    x ^= z << 0o4 as libc::c_int;
    y ^= z;
    z = ((y >> 0o20 as libc::c_int ^ x) as libc::c_long & 0xffff as libc::c_long)
        as uint32_t;
    y ^= z << 0o20 as libc::c_int;
    x ^= z;
    z = ((x >> 0o2 as libc::c_int ^ y) as libc::c_long & 0x33333333 as libc::c_long)
        as uint32_t;
    x ^= z << 0o2 as libc::c_int;
    y ^= z;
    z = ((y >> 0o10 as libc::c_int ^ x) as libc::c_long & 0xff00ff as libc::c_long)
        as uint32_t;
    y ^= z << 0o10 as libc::c_int;
    x ^= z;
    x = x >> 1 as libc::c_int | x << 31 as libc::c_int;
    z = ((x ^ y) as libc::c_long & 0x55555555 as libc::c_long) as uint32_t;
    y ^= z;
    x ^= z;
    y = y >> 1 as libc::c_int | y << 31 as libc::c_int;
    z = *r.offset(0 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(1 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(2 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(3 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(4 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(5 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(6 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(7 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(8 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(9 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(10 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(11 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(12 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(13 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(14 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(15 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(16 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(17 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(18 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(19 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(20 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(21 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(22 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(23 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(24 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(25 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(26 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(27 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(28 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(29 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(30 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(31 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    x = x << 1 as libc::c_int | x >> 31 as libc::c_int;
    z = ((x ^ y) as libc::c_long & 0x55555555 as libc::c_long) as uint32_t;
    y ^= z;
    x ^= z;
    y = y << 1 as libc::c_int | y >> 31 as libc::c_int;
    z = ((x >> 0o10 as libc::c_int ^ y) as libc::c_long & 0xff00ff as libc::c_long)
        as uint32_t;
    x ^= z << 0o10 as libc::c_int;
    y ^= z;
    z = ((y >> 0o2 as libc::c_int ^ x) as libc::c_long & 0x33333333 as libc::c_long)
        as uint32_t;
    y ^= z << 0o2 as libc::c_int;
    x ^= z;
    z = ((x >> 0o20 as libc::c_int ^ y) as libc::c_long & 0xffff as libc::c_long)
        as uint32_t;
    x ^= z << 0o20 as libc::c_int;
    y ^= z;
    z = ((y >> 0o4 as libc::c_int ^ x) as libc::c_long & 0xf0f0f0f as libc::c_long)
        as uint32_t;
    y ^= z << 0o4 as libc::c_int;
    x ^= z;
    *d.offset(0 as libc::c_int as isize) = x as uint8_t;
    x >>= 8 as libc::c_int;
    *d.offset(1 as libc::c_int as isize) = x as uint8_t;
    x >>= 8 as libc::c_int;
    *d.offset(2 as libc::c_int as isize) = x as uint8_t;
    x >>= 8 as libc::c_int;
    *d.offset(3 as libc::c_int as isize) = x as uint8_t;
    *d.offset(4 as libc::c_int as isize) = y as uint8_t;
    y >>= 8 as libc::c_int;
    *d.offset(5 as libc::c_int as isize) = y as uint8_t;
    y >>= 8 as libc::c_int;
    *d.offset(6 as libc::c_int as isize) = y as uint8_t;
    y >>= 8 as libc::c_int;
    *d.offset(7 as libc::c_int as isize) = y as uint8_t;
}
unsafe extern "C" fn DesSmallFipsDecrypt(
    mut d: *mut uint8_t,
    mut r: *const uint32_t,
    mut s: *const uint8_t,
) {
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    let mut z: uint32_t = 0;
    x = *s.offset(7 as libc::c_int as isize) as uint32_t;
    x <<= 8 as libc::c_int;
    x |= *s.offset(6 as libc::c_int as isize) as libc::c_uint;
    x <<= 8 as libc::c_int;
    x |= *s.offset(5 as libc::c_int as isize) as libc::c_uint;
    x <<= 8 as libc::c_int;
    x |= *s.offset(4 as libc::c_int as isize) as libc::c_uint;
    y = *s.offset(3 as libc::c_int as isize) as uint32_t;
    y <<= 8 as libc::c_int;
    y |= *s.offset(2 as libc::c_int as isize) as libc::c_uint;
    y <<= 8 as libc::c_int;
    y |= *s.offset(1 as libc::c_int as isize) as libc::c_uint;
    y <<= 8 as libc::c_int;
    y |= *s.offset(0 as libc::c_int as isize) as libc::c_uint;
    z = ((x >> 0o4 as libc::c_int ^ y) as libc::c_long & 0xf0f0f0f as libc::c_long)
        as uint32_t;
    x ^= z << 0o4 as libc::c_int;
    y ^= z;
    z = ((y >> 0o20 as libc::c_int ^ x) as libc::c_long & 0xffff as libc::c_long)
        as uint32_t;
    y ^= z << 0o20 as libc::c_int;
    x ^= z;
    z = ((x >> 0o2 as libc::c_int ^ y) as libc::c_long & 0x33333333 as libc::c_long)
        as uint32_t;
    x ^= z << 0o2 as libc::c_int;
    y ^= z;
    z = ((y >> 0o10 as libc::c_int ^ x) as libc::c_long & 0xff00ff as libc::c_long)
        as uint32_t;
    y ^= z << 0o10 as libc::c_int;
    x ^= z;
    x = x >> 1 as libc::c_int | x << 31 as libc::c_int;
    z = ((x ^ y) as libc::c_long & 0x55555555 as libc::c_long) as uint32_t;
    y ^= z;
    x ^= z;
    y = y >> 1 as libc::c_int | y << 31 as libc::c_int;
    z = *r.offset(31 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(30 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(29 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(28 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(27 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(26 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(25 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(24 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(23 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(22 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(21 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(20 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(19 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(18 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(17 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(16 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(15 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(14 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(13 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(12 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(11 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(10 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(9 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(8 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(7 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(6 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(5 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(4 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(3 as libc::c_int as isize);
    z ^= y;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(2 as libc::c_int as isize);
    z ^= y;
    x
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    x
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(1 as libc::c_int as isize);
    z ^= x;
    z = z << 4 as libc::c_int | z >> 28 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(448 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(384 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(320 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(256 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z = *r.offset(0 as libc::c_int as isize);
    z ^= x;
    y
        ^= *((des_keymap.as_ptr().offset(192 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(128 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr().offset(64 as libc::c_int as isize) as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    z >>= 8 as libc::c_int;
    y
        ^= *((des_keymap.as_ptr() as *mut uint8_t)
            .offset((0xfc as libc::c_int as libc::c_uint & z) as isize)
            as *mut uint32_t);
    x = x << 1 as libc::c_int | x >> 31 as libc::c_int;
    z = ((x ^ y) as libc::c_long & 0x55555555 as libc::c_long) as uint32_t;
    y ^= z;
    x ^= z;
    y = y << 1 as libc::c_int | y >> 31 as libc::c_int;
    z = ((x >> 0o10 as libc::c_int ^ y) as libc::c_long & 0xff00ff as libc::c_long)
        as uint32_t;
    x ^= z << 0o10 as libc::c_int;
    y ^= z;
    z = ((y >> 0o2 as libc::c_int ^ x) as libc::c_long & 0x33333333 as libc::c_long)
        as uint32_t;
    y ^= z << 0o2 as libc::c_int;
    x ^= z;
    z = ((x >> 0o20 as libc::c_int ^ y) as libc::c_long & 0xffff as libc::c_long)
        as uint32_t;
    x ^= z << 0o20 as libc::c_int;
    y ^= z;
    z = ((y >> 0o4 as libc::c_int ^ x) as libc::c_long & 0xf0f0f0f as libc::c_long)
        as uint32_t;
    y ^= z << 0o4 as libc::c_int;
    x ^= z;
    *d.offset(0 as libc::c_int as isize) = x as uint8_t;
    x >>= 8 as libc::c_int;
    *d.offset(1 as libc::c_int as isize) = x as uint8_t;
    x >>= 8 as libc::c_int;
    *d.offset(2 as libc::c_int as isize) = x as uint8_t;
    x >>= 8 as libc::c_int;
    *d.offset(3 as libc::c_int as isize) = x as uint8_t;
    *d.offset(4 as libc::c_int as isize) = y as uint8_t;
    y >>= 8 as libc::c_int;
    *d.offset(5 as libc::c_int as isize) = y as uint8_t;
    y >>= 8 as libc::c_int;
    *d.offset(6 as libc::c_int as isize) = y as uint8_t;
    y >>= 8 as libc::c_int;
    *d.offset(7 as libc::c_int as isize) = y as uint8_t;
}
static mut parity_16: [libc::c_uint; 16] = [
    0 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    1 as libc::c_int as libc::c_uint,
    0 as libc::c_int as libc::c_uint,
];
pub unsafe extern "C" fn nettle_des_check_parity(
    mut length: size_t,
    mut key: *const uint8_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < length {
        if parity_16[(*key.offset(i as isize) as libc::c_int & 0xf as libc::c_int)
            as usize]
            ^ parity_16[(*key.offset(i as isize) as libc::c_int >> 4 as libc::c_int
                & 0xf as libc::c_int) as usize] == 0
        {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_des_fix_parity(
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < length {
        *dst
            .offset(
                i as isize,
            ) = (*src.offset(i as isize) as libc::c_uint
            ^ (parity_16[(*src.offset(i as isize) as libc::c_int & 0xf as libc::c_int)
                as usize]
                ^ parity_16[(*src.offset(i as isize) as libc::c_int >> 4 as libc::c_int
                    & 0xf as libc::c_int) as usize]) ^ 1 as libc::c_int as libc::c_uint)
            as uint8_t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn des_weak_p(mut key: *const uint8_t) -> libc::c_int {
    static mut asso_values: [libc::c_uchar; 129] = [
        16 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ];
    static mut weak_key_hash: [[int8_t; 4]; 26] = [
        [
            0x7f as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
        ],
        [
            0x7f as libc::c_int as int8_t,
            0x70 as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
            0x78 as libc::c_int as int8_t,
        ],
        [
            0x7f as libc::c_int as int8_t,
            0xf as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
            0x7 as libc::c_int as int8_t,
        ],
        [
            0x70 as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
            0x78 as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
        ],
        [
            0x70 as libc::c_int as int8_t,
            0x70 as libc::c_int as int8_t,
            0x78 as libc::c_int as int8_t,
            0x78 as libc::c_int as int8_t,
        ],
        [
            0x70 as libc::c_int as int8_t,
            0xf as libc::c_int as int8_t,
            0x78 as libc::c_int as int8_t,
            0x7 as libc::c_int as int8_t,
        ],
        [
            0xf as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
            0x7 as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
        ],
        [
            0xf as libc::c_int as int8_t,
            0x70 as libc::c_int as int8_t,
            0x7 as libc::c_int as int8_t,
            0x78 as libc::c_int as int8_t,
        ],
        [
            0xf as libc::c_int as int8_t,
            0xf as libc::c_int as int8_t,
            0x7 as libc::c_int as int8_t,
            0x7 as libc::c_int as int8_t,
        ],
        [
            0x7f as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            0x70 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0x78 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            0xf as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0x7 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0x7f as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0x70 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0x78 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0xf as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0x7 as libc::c_int as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
    ];
    let mut k0: int8_t = (*key.offset(0 as libc::c_int as isize) as libc::c_int
        >> 1 as libc::c_int) as int8_t;
    let mut k1: int8_t = (*key.offset(1 as libc::c_int as isize) as libc::c_int
        >> 1 as libc::c_int) as int8_t;
    let mut hash: libc::c_uint = (asso_values[(k1 as libc::c_int + 1 as libc::c_int)
        as usize] as libc::c_int + asso_values[k0 as usize] as libc::c_int)
        as libc::c_uint;
    let mut candidate: *const int8_t = 0 as *const int8_t;
    if hash > 25 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    candidate = (weak_key_hash[hash as usize]).as_ptr();
    if k0 as libc::c_int != *candidate.offset(0 as libc::c_int as isize) as libc::c_int
        || k1 as libc::c_int
            != *candidate.offset(1 as libc::c_int as isize) as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if *key.offset(2 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int
        != k0 as libc::c_int
        || *key.offset(3 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int
            != k1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    k0 = (*key.offset(4 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int8_t;
    k1 = (*key.offset(5 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int)
        as int8_t;
    if k0 as libc::c_int != *candidate.offset(2 as libc::c_int as isize) as libc::c_int
        || k1 as libc::c_int
            != *candidate.offset(3 as libc::c_int as isize) as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if *key.offset(6 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int
        != k0 as libc::c_int
        || *key.offset(7 as libc::c_int as isize) as libc::c_int >> 1 as libc::c_int
            != k1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn nettle_des_set_key(
    mut ctx: *mut des_ctx,
    mut key: *const uint8_t,
) -> libc::c_int {
    let mut n: uint32_t = 0;
    let mut w: uint32_t = 0;
    let mut b0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bits0: [libc::c_char; 56] = [0; 56];
    let mut bits1: [libc::c_char; 56] = [0; 56];
    let mut method: *mut uint32_t = 0 as *mut uint32_t;
    let mut k: *const uint8_t = 0 as *const uint8_t;
    n = 56 as libc::c_int as uint32_t;
    b0 = bits0.as_mut_ptr();
    b1 = bits1.as_mut_ptr();
    k = key;
    loop {
        let fresh0 = k;
        k = k.offset(1);
        w = ((256 as libc::c_int | *fresh0 as libc::c_int) << 2 as libc::c_int)
            as uint32_t;
        loop {
            n = n.wrapping_sub(1);
            n;
            *b1
                .offset(
                    n as isize,
                ) = (8 as libc::c_int as libc::c_uint & w) as libc::c_char;
            w >>= 1 as libc::c_int;
            *b0
                .offset(
                    n as isize,
                ) = (4 as libc::c_int as libc::c_uint & w) as libc::c_char;
            if !(w >= 16 as libc::c_int as libc::c_uint) {
                break;
            }
        }
        if !(n != 0) {
            break;
        }
    }
    n = 16 as libc::c_int as uint32_t;
    k = rotors.as_ptr();
    method = ((*ctx).key).as_mut_ptr();
    loop {
        w = ((*b1.offset(*k.offset(0 as libc::c_int as isize) as isize) as libc::c_int
            | *b0.offset(*k.offset(1 as libc::c_int as isize) as isize) as libc::c_int)
            << 4 as libc::c_int) as uint32_t;
        w
            |= ((*b1.offset(*k.offset(2 as libc::c_int as isize) as isize) as libc::c_int
                | *b0.offset(*k.offset(3 as libc::c_int as isize) as isize)
                    as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        w
            |= (*b1.offset(*k.offset(4 as libc::c_int as isize) as isize) as libc::c_int
                | *b0.offset(*k.offset(5 as libc::c_int as isize) as isize)
                    as libc::c_int) as libc::c_uint;
        w <<= 8 as libc::c_int;
        w
            |= ((*b1.offset(*k.offset(6 as libc::c_int as isize) as isize) as libc::c_int
                | *b0.offset(*k.offset(7 as libc::c_int as isize) as isize)
                    as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        w
            |= ((*b1.offset(*k.offset(8 as libc::c_int as isize) as isize) as libc::c_int
                | *b0.offset(*k.offset(9 as libc::c_int as isize) as isize)
                    as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        w
            |= (*b1.offset(*k.offset(10 as libc::c_int as isize) as isize) as libc::c_int
                | *b0.offset(*k.offset(11 as libc::c_int as isize) as isize)
                    as libc::c_int) as libc::c_uint;
        w <<= 8 as libc::c_int;
        w
            |= ((*b1.offset(*k.offset(12 as libc::c_int as isize) as isize)
                as libc::c_int
                | *b0.offset(*k.offset(13 as libc::c_int as isize) as isize)
                    as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        w
            |= ((*b1.offset(*k.offset(14 as libc::c_int as isize) as isize)
                as libc::c_int
                | *b0.offset(*k.offset(15 as libc::c_int as isize) as isize)
                    as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        w
            |= (*b1.offset(*k.offset(16 as libc::c_int as isize) as isize) as libc::c_int
                | *b0.offset(*k.offset(17 as libc::c_int as isize) as isize)
                    as libc::c_int) as libc::c_uint;
        w <<= 8 as libc::c_int;
        w
            |= ((*b1.offset(*k.offset(18 as libc::c_int as isize) as isize)
                as libc::c_int
                | *b0.offset(*k.offset(19 as libc::c_int as isize) as isize)
                    as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        w
            |= ((*b1.offset(*k.offset(20 as libc::c_int as isize) as isize)
                as libc::c_int
                | *b0.offset(*k.offset(21 as libc::c_int as isize) as isize)
                    as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        w
            |= (*b1.offset(*k.offset(22 as libc::c_int as isize) as isize) as libc::c_int
                | *b0.offset(*k.offset(23 as libc::c_int as isize) as isize)
                    as libc::c_int) as libc::c_uint;
        *method.offset(0 as libc::c_int as isize) = w;
        w = ((*b1
            .offset(*k.offset((0 as libc::c_int + 24 as libc::c_int) as isize) as isize)
            as libc::c_int
            | *b0
                .offset(
                    *k.offset((1 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int) << 4 as libc::c_int) as uint32_t;
        w
            |= ((*b1
                .offset(
                    *k.offset((2 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((3 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        w
            |= (*b1
                .offset(
                    *k.offset((4 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((5 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) as libc::c_uint;
        w <<= 8 as libc::c_int;
        w
            |= ((*b1
                .offset(
                    *k.offset((6 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((7 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        w
            |= ((*b1
                .offset(
                    *k.offset((8 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((9 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        w
            |= (*b1
                .offset(
                    *k.offset((10 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((11 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) as libc::c_uint;
        w <<= 8 as libc::c_int;
        w
            |= ((*b1
                .offset(
                    *k.offset((12 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((13 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        w
            |= ((*b1
                .offset(
                    *k.offset((14 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((15 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        w
            |= (*b1
                .offset(
                    *k.offset((16 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((17 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) as libc::c_uint;
        w <<= 8 as libc::c_int;
        w
            |= ((*b1
                .offset(
                    *k.offset((18 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((19 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        w
            |= ((*b1
                .offset(
                    *k.offset((20 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((21 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) << 2 as libc::c_int) as libc::c_uint;
        w
            |= (*b1
                .offset(
                    *k.offset((22 as libc::c_int + 24 as libc::c_int) as isize) as isize,
                ) as libc::c_int
                | *b0
                    .offset(
                        *k.offset((23 as libc::c_int + 24 as libc::c_int) as isize)
                            as isize,
                    ) as libc::c_int) as libc::c_uint;
        w = w >> 4 as libc::c_int | w << 28 as libc::c_int;
        *method.offset(1 as libc::c_int as isize) = w;
        k = k.offset(48 as libc::c_int as isize);
        method = method.offset(2 as libc::c_int as isize);
        n = n.wrapping_sub(1);
        if !(n != 0) {
            break;
        }
    }
    return (des_weak_p(key) == 0) as libc::c_int;
}
pub unsafe extern "C" fn nettle_des_encrypt(
    mut ctx: *const des_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!(length % DES_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
            b"des.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"void nettle_des_encrypt(const struct des_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8498: {
        if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!(length % DES_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
                b"des.c\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"void nettle_des_encrypt(const struct des_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        DesSmallFipsEncrypt(dst, ((*ctx).key).as_ptr(), src);
        length = (length as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        src = src.offset(8 as libc::c_int as isize);
        dst = dst.offset(8 as libc::c_int as isize);
    }
}
pub unsafe extern "C" fn nettle_des_decrypt(
    mut ctx: *const des_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
        __assert_fail(
            b"!(length % DES_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
            b"des.c\0" as *const u8 as *const libc::c_char,
            298 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 84],
                &[libc::c_char; 84],
            >(
                b"void nettle_des_decrypt(const struct des_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_12605: {
        if length.wrapping_rem(8 as libc::c_int as libc::c_ulong) == 0 {} else {
            __assert_fail(
                b"!(length % DES_BLOCK_SIZE)\0" as *const u8 as *const libc::c_char,
                b"des.c\0" as *const u8 as *const libc::c_char,
                298 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 84],
                    &[libc::c_char; 84],
                >(
                    b"void nettle_des_decrypt(const struct des_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        DesSmallFipsDecrypt(dst, ((*ctx).key).as_ptr(), src);
        length = (length as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong) as size_t as size_t;
        src = src.offset(8 as libc::c_int as isize);
        dst = dst.offset(8 as libc::c_int as isize);
    }
}
