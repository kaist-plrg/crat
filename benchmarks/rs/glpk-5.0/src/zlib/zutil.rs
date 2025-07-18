use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type voidpf = *mut libc::c_void;
pub static mut _glp_zlib_z_errmsg: [*const libc::c_char; 10] = [
    b"need dictionary\0" as *const u8 as *const libc::c_char,
    b"stream end\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"file error\0" as *const u8 as *const libc::c_char,
    b"stream error\0" as *const u8 as *const libc::c_char,
    b"data error\0" as *const u8 as *const libc::c_char,
    b"insufficient memory\0" as *const u8 as *const libc::c_char,
    b"buffer error\0" as *const u8 as *const libc::c_char,
    b"incompatible version\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn _glp_zlib_zlibVersion() -> *const libc::c_char {
    return b"1.2.5\0" as *const u8 as *const libc::c_char;
}
pub unsafe extern "C" fn _glp_zlib_zlibCompileFlags() -> uLong {
    let mut flags: uLong = 0;
    flags = 0 as libc::c_int as uLong;
    match ::std::mem::size_of::<uInt>() as libc::c_ulong as libc::c_int {
        2 => {}
        4 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as uLong as uLong;
        }
        8 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as uLong as uLong;
        }
        _ => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong) as uLong as uLong;
        }
    }
    match ::std::mem::size_of::<uLong>() as libc::c_ulong as libc::c_int {
        2 => {}
        4 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong)
                as uLong as uLong;
        }
        8 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((2 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong)
                as uLong as uLong;
        }
        _ => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((3 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong)
                as uLong as uLong;
        }
    }
    match ::std::mem::size_of::<voidpf>() as libc::c_ulong as libc::c_int {
        2 => {}
        4 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong)
                as uLong as uLong;
        }
        8 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((2 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong)
                as uLong as uLong;
        }
        _ => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((3 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong)
                as uLong as uLong;
        }
    }
    match ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as libc::c_int {
        2 => {}
        4 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong)
                as uLong as uLong;
        }
        8 => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((2 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong)
                as uLong as uLong;
        }
        _ => {
            flags = (flags as libc::c_ulong)
                .wrapping_add(((3 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong)
                as uLong as uLong;
        }
    }
    flags = (flags as libc::c_ulong)
        .wrapping_add(((1 as libc::c_long) << 25 as libc::c_int) as libc::c_ulong)
        as uLong as uLong;
    return flags;
}
pub unsafe extern "C" fn _glp_zlib_zError(mut err: libc::c_int) -> *const libc::c_char {
    return _glp_zlib_z_errmsg[(2 as libc::c_int - err) as usize];
}
pub unsafe extern "C" fn _glp_zlib_zcalloc(
    mut opaque: voidpf,
    mut items: libc::c_uint,
    mut size: libc::c_uint,
) -> voidpf {
    if !opaque.is_null() {
        items = items.wrapping_add(size.wrapping_sub(size));
    }
    return if ::std::mem::size_of::<uInt>() as libc::c_ulong
        > 2 as libc::c_int as libc::c_ulong
    {
        malloc(items.wrapping_mul(size) as libc::c_ulong)
    } else {
        calloc(items as libc::c_ulong, size as libc::c_ulong)
    };
}
pub unsafe extern "C" fn _glp_zlib_zcfree(mut opaque: voidpf, mut ptr: voidpf) {
    free(ptr);
    if !opaque.is_null() {
        return;
    }
}
