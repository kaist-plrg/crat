use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type boolean = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub unsafe extern "C" fn fspool_file(mut zfile: *const libc::c_char) -> boolean {
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    if *zfile as libc::c_int != 'C' as i32 && *zfile as libc::c_int != 'D' as i32
        && *zfile as libc::c_int != 'X' as i32
    {
        return 0 as libc::c_int;
    }
    if *zfile.offset(1 as libc::c_int as isize) as libc::c_int != '.' as i32 {
        return 0 as libc::c_int;
    }
    z = zfile.offset(2 as libc::c_int as isize);
    while *z as libc::c_int != '\0' as i32 {
        if *z as libc::c_int == '/' as i32
            || *(*__ctype_b_loc()).offset(*z as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            || *(*__ctype_b_loc()).offset(*z as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            return 0 as libc::c_int;
        }
        z = z.offset(1);
        z;
    }
    return 1 as libc::c_int;
}
