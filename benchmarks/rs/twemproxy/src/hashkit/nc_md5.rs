use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type MD5_u32plus = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CTX {
    pub lo: MD5_u32plus,
    pub hi: MD5_u32plus,
    pub a: MD5_u32plus,
    pub b: MD5_u32plus,
    pub c: MD5_u32plus,
    pub d: MD5_u32plus,
    pub buffer: [libc::c_uchar; 64],
    pub block: [MD5_u32plus; 16],
}
unsafe extern "C" fn body(
    mut ctx: *mut MD5_CTX,
    mut data: *const libc::c_void,
    mut size: libc::c_ulong,
) -> *const libc::c_void {
    let mut ptr: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut a: MD5_u32plus = 0;
    let mut b: MD5_u32plus = 0;
    let mut c: MD5_u32plus = 0;
    let mut d: MD5_u32plus = 0;
    let mut saved_a: MD5_u32plus = 0;
    let mut saved_b: MD5_u32plus = 0;
    let mut saved_c: MD5_u32plus = 0;
    let mut saved_d: MD5_u32plus = 0;
    ptr = data as *const libc::c_uchar;
    a = (*ctx).a;
    b = (*ctx).b;
    c = (*ctx).c;
    d = (*ctx).d;
    loop {
        saved_a = a;
        saved_b = b;
        saved_c = c;
        saved_d = d;
        a = (a as libc::c_uint)
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xd76aa478 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 7 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 7 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xe8c7b756 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 12 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 12 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x242070db as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 17 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 17 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xc1bdceee as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 22 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 22 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((4 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xf57c0faf as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 7 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 7 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((5 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x4787c62a as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 12 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 12 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((6 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xa8304613 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 17 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 17 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((7 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xfd469501 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 22 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 22 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((8 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x698098d8 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 7 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 7 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((9 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x8b44f7af as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 12 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 12 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((10 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xffff5bb1 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 17 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 17 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((11 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x895cd7be as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 22 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 22 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((12 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x6b901122 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 7 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 7 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((13 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xfd987193 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 12 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 12 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((14 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xa679438e as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 17 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 17 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((15 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x49b40821 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 22 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 22 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (c ^ d & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xf61e2562 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 5 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 5 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (b ^ c & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((6 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xc040b340 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 9 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (a ^ b & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((11 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x265e5a51 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 14 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 14 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (d ^ a & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xe9b6c7aa as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 20 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 20 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (c ^ d & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((5 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xd62f105d as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 5 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 5 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (b ^ c & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((10 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x2441453 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 9 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (a ^ b & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((15 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xd8a1e681 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 14 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 14 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (d ^ a & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((4 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xe7d3fbc8 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 20 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 20 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (c ^ d & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((9 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x21e1cde6 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 5 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 5 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (b ^ c & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((14 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xc33707d6 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 9 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (a ^ b & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xf4d50d87 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 14 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 14 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (d ^ a & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((8 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x455a14ed as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 20 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 20 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (c ^ d & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((13 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xa9e3e905 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 5 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 5 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (b ^ c & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xfcefa3f8 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 9 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (a ^ b & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((7 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x676f02d9 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 14 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 14 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (d ^ a & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((12 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x8d2a4c8a as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 20 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 20 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(
                        *(&*ptr.offset((5 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xfffa3942 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 4 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 4 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(
                        *(&*ptr.offset((8 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x8771f681 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 11 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(
                        *(&*ptr.offset((11 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x6d9d6122 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 16 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 16 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(
                        *(&*ptr.offset((14 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xfde5380c as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 23 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 23 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(
                        *(&*ptr.offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xa4beea44 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 4 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 4 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(
                        *(&*ptr.offset((4 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x4bdecfa9 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 11 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(
                        *(&*ptr.offset((7 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xf6bb4b60 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 16 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 16 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(
                        *(&*ptr.offset((10 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xbebfbc70 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 23 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 23 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(
                        *(&*ptr.offset((13 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x289b7ec6 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 4 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 4 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(
                        *(&*ptr.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xeaa127fa as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 11 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(
                        *(&*ptr.offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xd4ef3085 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 16 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 16 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(
                        *(&*ptr.offset((6 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x4881d05 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 23 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 23 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(
                        *(&*ptr.offset((9 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xd9d4d039 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 4 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 4 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(
                        *(&*ptr.offset((12 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xe6db99e5 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 11 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(
                        *(&*ptr.offset((15 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x1fa27cf8 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 16 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 16 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(
                        *(&*ptr.offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xc4ac5665 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 23 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 23 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (c ^ (b | !d))
                    .wrapping_add(
                        *(&*ptr.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xf4292244 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 6 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 6 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (b ^ (a | !c))
                    .wrapping_add(
                        *(&*ptr.offset((7 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x432aff97 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 10 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 10 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (a ^ (d | !b))
                    .wrapping_add(
                        *(&*ptr.offset((14 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xab9423a7 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 15 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 15 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (d ^ (c | !a))
                    .wrapping_add(
                        *(&*ptr.offset((5 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xfc93a039 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 21 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 21 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (c ^ (b | !d))
                    .wrapping_add(
                        *(&*ptr.offset((12 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x655b59c3 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 6 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 6 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (b ^ (a | !c))
                    .wrapping_add(
                        *(&*ptr.offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x8f0ccc92 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 10 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 10 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (a ^ (d | !b))
                    .wrapping_add(
                        *(&*ptr.offset((10 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xffeff47d as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 15 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 15 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (d ^ (c | !a))
                    .wrapping_add(
                        *(&*ptr.offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x85845dd1 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 21 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 21 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (c ^ (b | !d))
                    .wrapping_add(
                        *(&*ptr.offset((8 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x6fa87e4f as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 6 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 6 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (b ^ (a | !c))
                    .wrapping_add(
                        *(&*ptr.offset((15 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xfe2ce6e0 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 10 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 10 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (a ^ (d | !b))
                    .wrapping_add(
                        *(&*ptr.offset((6 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xa3014314 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 15 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 15 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (d ^ (c | !a))
                    .wrapping_add(
                        *(&*ptr.offset((13 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x4e0811a1 as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 21 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 21 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint)
            .wrapping_add(
                (c ^ (b | !d))
                    .wrapping_add(
                        *(&*ptr.offset((4 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xf7537e82 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        a = a << 6 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 6 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(b) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint)
            .wrapping_add(
                (b ^ (a | !c))
                    .wrapping_add(
                        *(&*ptr.offset((11 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xbd3af235 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        d = d << 10 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 10 as libc::c_int;
        d = (d as libc::c_uint).wrapping_add(a) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint)
            .wrapping_add(
                (a ^ (d | !b))
                    .wrapping_add(
                        *(&*ptr.offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0x2ad7d2bb as libc::c_int as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        c = c << 15 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 15 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(d) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint)
            .wrapping_add(
                (d ^ (c | !a))
                    .wrapping_add(
                        *(&*ptr.offset((9 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD5_u32plus),
                    )
                    .wrapping_add(0xeb86d391 as libc::c_uint),
            ) as MD5_u32plus as MD5_u32plus;
        b = b << 21 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 21 as libc::c_int;
        b = (b as libc::c_uint).wrapping_add(c) as MD5_u32plus as MD5_u32plus;
        a = (a as libc::c_uint).wrapping_add(saved_a) as MD5_u32plus as MD5_u32plus;
        b = (b as libc::c_uint).wrapping_add(saved_b) as MD5_u32plus as MD5_u32plus;
        c = (c as libc::c_uint).wrapping_add(saved_c) as MD5_u32plus as MD5_u32plus;
        d = (d as libc::c_uint).wrapping_add(saved_d) as MD5_u32plus as MD5_u32plus;
        ptr = ptr.offset(64 as libc::c_int as isize);
        size = size.wrapping_sub(64 as libc::c_int as libc::c_ulong);
        if !(size != 0) {
            break;
        }
    }
    (*ctx).a = a;
    (*ctx).b = b;
    (*ctx).c = c;
    (*ctx).d = d;
    return ptr as *const libc::c_void;
}
pub unsafe extern "C" fn MD5_Init(mut ctx: *mut MD5_CTX) {
    (*ctx).a = 0x67452301 as libc::c_int as MD5_u32plus;
    (*ctx).b = 0xefcdab89 as libc::c_uint;
    (*ctx).c = 0x98badcfe as libc::c_uint;
    (*ctx).d = 0x10325476 as libc::c_int as MD5_u32plus;
    (*ctx).lo = 0 as libc::c_int as MD5_u32plus;
    (*ctx).hi = 0 as libc::c_int as MD5_u32plus;
}
pub unsafe extern "C" fn MD5_Update(
    mut ctx: *mut MD5_CTX,
    mut data: *const libc::c_void,
    mut size: libc::c_ulong,
) {
    let mut saved_lo: MD5_u32plus = 0;
    let mut used: libc::c_ulong = 0;
    let mut free: libc::c_ulong = 0;
    saved_lo = (*ctx).lo;
    (*ctx)
        .lo = ((saved_lo as libc::c_ulong).wrapping_add(size)
        & 0x1fffffff as libc::c_int as libc::c_ulong) as MD5_u32plus;
    if (*ctx).lo < saved_lo {
        (*ctx).hi = ((*ctx).hi).wrapping_add(1);
        (*ctx).hi;
    }
    (*ctx)
        .hi = ((*ctx).hi as libc::c_ulong).wrapping_add(size >> 29 as libc::c_int)
        as MD5_u32plus as MD5_u32plus;
    used = (saved_lo & 0x3f as libc::c_int as libc::c_uint) as libc::c_ulong;
    if used != 0 {
        free = (64 as libc::c_int as libc::c_ulong).wrapping_sub(used);
        if size < free {
            memcpy(
                &mut *((*ctx).buffer).as_mut_ptr().offset(used as isize)
                    as *mut libc::c_uchar as *mut libc::c_void,
                data,
                size,
            );
            return;
        }
        memcpy(
            &mut *((*ctx).buffer).as_mut_ptr().offset(used as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            data,
            free,
        );
        data = (data as *const libc::c_uchar).offset(free as isize)
            as *const libc::c_void;
        size = size.wrapping_sub(free);
        body(
            ctx,
            ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
    }
    if size >= 64 as libc::c_int as libc::c_ulong {
        data = body(ctx, data, size & !(0x3f as libc::c_int as libc::c_ulong));
        size &= 0x3f as libc::c_int as libc::c_ulong;
    }
    memcpy(((*ctx).buffer).as_mut_ptr() as *mut libc::c_void, data, size);
}
pub unsafe extern "C" fn MD5_Final(
    mut result: *mut libc::c_uchar,
    mut ctx: *mut MD5_CTX,
) {
    let mut used: libc::c_ulong = 0;
    let mut free: libc::c_ulong = 0;
    used = ((*ctx).lo & 0x3f as libc::c_int as libc::c_uint) as libc::c_ulong;
    let fresh0 = used;
    used = used.wrapping_add(1);
    (*ctx).buffer[fresh0 as usize] = 0x80 as libc::c_int as libc::c_uchar;
    free = (64 as libc::c_int as libc::c_ulong).wrapping_sub(used);
    if free < 8 as libc::c_int as libc::c_ulong {
        memset(
            &mut *((*ctx).buffer).as_mut_ptr().offset(used as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            0 as libc::c_int,
            free,
        );
        body(
            ctx,
            ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
        used = 0 as libc::c_int as libc::c_ulong;
        free = 64 as libc::c_int as libc::c_ulong;
    }
    memset(
        &mut *((*ctx).buffer).as_mut_ptr().offset(used as isize) as *mut libc::c_uchar
            as *mut libc::c_void,
        0 as libc::c_int,
        free.wrapping_sub(8 as libc::c_int as libc::c_ulong),
    );
    (*ctx).lo <<= 3 as libc::c_int;
    (*ctx).buffer[56 as libc::c_int as usize] = (*ctx).lo as libc::c_uchar;
    (*ctx)
        .buffer[57 as libc::c_int
        as usize] = ((*ctx).lo >> 8 as libc::c_int) as libc::c_uchar;
    (*ctx)
        .buffer[58 as libc::c_int
        as usize] = ((*ctx).lo >> 16 as libc::c_int) as libc::c_uchar;
    (*ctx)
        .buffer[59 as libc::c_int
        as usize] = ((*ctx).lo >> 24 as libc::c_int) as libc::c_uchar;
    (*ctx).buffer[60 as libc::c_int as usize] = (*ctx).hi as libc::c_uchar;
    (*ctx)
        .buffer[61 as libc::c_int
        as usize] = ((*ctx).hi >> 8 as libc::c_int) as libc::c_uchar;
    (*ctx)
        .buffer[62 as libc::c_int
        as usize] = ((*ctx).hi >> 16 as libc::c_int) as libc::c_uchar;
    (*ctx)
        .buffer[63 as libc::c_int
        as usize] = ((*ctx).hi >> 24 as libc::c_int) as libc::c_uchar;
    body(
        ctx,
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        64 as libc::c_int as libc::c_ulong,
    );
    *result.offset(0 as libc::c_int as isize) = (*ctx).a as libc::c_uchar;
    *result
        .offset(
            1 as libc::c_int as isize,
        ) = ((*ctx).a >> 8 as libc::c_int) as libc::c_uchar;
    *result
        .offset(
            2 as libc::c_int as isize,
        ) = ((*ctx).a >> 16 as libc::c_int) as libc::c_uchar;
    *result
        .offset(
            3 as libc::c_int as isize,
        ) = ((*ctx).a >> 24 as libc::c_int) as libc::c_uchar;
    *result.offset(4 as libc::c_int as isize) = (*ctx).b as libc::c_uchar;
    *result
        .offset(
            5 as libc::c_int as isize,
        ) = ((*ctx).b >> 8 as libc::c_int) as libc::c_uchar;
    *result
        .offset(
            6 as libc::c_int as isize,
        ) = ((*ctx).b >> 16 as libc::c_int) as libc::c_uchar;
    *result
        .offset(
            7 as libc::c_int as isize,
        ) = ((*ctx).b >> 24 as libc::c_int) as libc::c_uchar;
    *result.offset(8 as libc::c_int as isize) = (*ctx).c as libc::c_uchar;
    *result
        .offset(
            9 as libc::c_int as isize,
        ) = ((*ctx).c >> 8 as libc::c_int) as libc::c_uchar;
    *result
        .offset(
            10 as libc::c_int as isize,
        ) = ((*ctx).c >> 16 as libc::c_int) as libc::c_uchar;
    *result
        .offset(
            11 as libc::c_int as isize,
        ) = ((*ctx).c >> 24 as libc::c_int) as libc::c_uchar;
    *result.offset(12 as libc::c_int as isize) = (*ctx).d as libc::c_uchar;
    *result
        .offset(
            13 as libc::c_int as isize,
        ) = ((*ctx).d >> 8 as libc::c_int) as libc::c_uchar;
    *result
        .offset(
            14 as libc::c_int as isize,
        ) = ((*ctx).d >> 16 as libc::c_int) as libc::c_uchar;
    *result
        .offset(
            15 as libc::c_int as isize,
        ) = ((*ctx).d >> 24 as libc::c_int) as libc::c_uchar;
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<MD5_CTX>() as libc::c_ulong,
    );
}
pub unsafe extern "C" fn md5_signature(
    mut key: *const libc::c_uchar,
    mut length: libc::c_ulong,
    mut result: *mut libc::c_uchar,
) {
    let mut my_md5: MD5_CTX = MD5_CTX {
        lo: 0,
        hi: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        buffer: [0; 64],
        block: [0; 16],
    };
    MD5_Init(&mut my_md5);
    MD5_Update(&mut my_md5, key as *const libc::c_void, length);
    MD5_Final(result, &mut my_md5);
}
pub unsafe extern "C" fn hash_md5(
    mut key: *const libc::c_char,
    mut key_length: size_t,
) -> uint32_t {
    let mut results: [libc::c_uchar; 16] = [0; 16];
    md5_signature(key as *const libc::c_uchar, key_length, results.as_mut_ptr());
    return ((results[3 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int)
        as uint32_t) << 24 as libc::c_int
        | ((results[2 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int)
            as uint32_t) << 16 as libc::c_int
        | ((results[1 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int)
            as uint32_t) << 8 as libc::c_int
        | (results[0 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int)
            as libc::c_uint;
}
