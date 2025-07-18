use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type pointer = *mut libc::c_void;
pub const _ISspace: C2RustUnnamed = 8192;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub static mut _uuconf_split_rcsid: [libc::c_char; 49] = unsafe {
    *::std::mem::transmute::<
        &[u8; 49],
        &[libc::c_char; 49],
    >(b"$Id: split.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn _uuconf_istrsplit(
    mut zline: *mut libc::c_char,
    mut bsep: libc::c_int,
    mut ppzsplit: *mut *mut *mut libc::c_char,
    mut pcsplit: *mut size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    loop {
        if bsep == '\0' as i32 {
            while *(*__ctype_b_loc())
                .offset(*zline as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                zline = zline.offset(1);
                zline;
            }
            if *zline as libc::c_int == '\0' as i32 {
                break;
            }
        }
        if i >= *pcsplit {
            let mut pznew: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut cnew: size_t = 0;
            if *pcsplit == 0 as libc::c_int as libc::c_ulong {
                cnew = 8 as libc::c_int as size_t;
                pznew = malloc(
                    cnew
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
            } else {
                cnew = (*pcsplit).wrapping_mul(2 as libc::c_int as libc::c_ulong);
                pznew = realloc(
                    *ppzsplit as pointer,
                    cnew
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
            }
            if pznew.is_null() {
                return -(1 as libc::c_int);
            }
            *ppzsplit = pznew;
            *pcsplit = cnew;
        }
        let ref mut fresh0 = *(*ppzsplit).offset(i as isize);
        *fresh0 = zline;
        i = i.wrapping_add(1);
        i;
        if bsep == '\0' as i32 {
            while *zline as libc::c_int != '\0' as i32
                && *(*__ctype_b_loc())
                    .offset(*zline as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                zline = zline.offset(1);
                zline;
            }
        } else {
            while *zline as libc::c_int != '\0' as i32 && *zline as libc::c_int != bsep {
                zline = zline.offset(1);
                zline;
            }
        }
        if *zline as libc::c_int == '\0' as i32 {
            break;
        }
        let fresh1 = zline;
        zline = zline.offset(1);
        *fresh1 = '\0' as i32 as libc::c_char;
    }
    return i as libc::c_int;
}
