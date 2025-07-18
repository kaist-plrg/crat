use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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
pub static mut quotes_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: quotes.c,v 1.2 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn zquote_cmd_string(
    mut zorig: *const libc::c_char,
    mut fbackslashonly: boolean,
) -> *mut libc::c_char {
    let mut z: *const libc::c_char = 0 as *const libc::c_char;
    let mut zret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    if zorig.is_null() {
        return 0 as *mut libc::c_char;
    }
    zret = zbufalc(
        (strlen(zorig))
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    zto = zret;
    z = zorig;
    while *z as libc::c_int != '\0' as i32 {
        if *z as libc::c_int == '\\' as i32 {
            let fresh0 = zto;
            zto = zto.offset(1);
            *fresh0 = '\\' as i32 as libc::c_char;
            let fresh1 = zto;
            zto = zto.offset(1);
            *fresh1 = '\\' as i32 as libc::c_char;
        } else if fbackslashonly != 0
            || *(*__ctype_b_loc()).offset(*z as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISgraph as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            let fresh2 = zto;
            zto = zto.offset(1);
            *fresh2 = *z;
        } else {
            sprintf(
                zto,
                b"\\%03o\0" as *const u8 as *const libc::c_char,
                *z as libc::c_uchar as libc::c_uint,
            );
            zto = zto.offset(strlen(zto) as isize);
        }
        z = z.offset(1);
        z;
    }
    *zto = '\0' as i32 as libc::c_char;
    return zret;
}
