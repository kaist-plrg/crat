use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
    fn zbufalc(csize: size_t) -> *mut libc::c_char;
    fn zbufcpy(z: *const libc::c_char) -> *mut libc::c_char;
    fn ubuffree(z: *mut libc::c_char);
}
pub type size_t = libc::c_ulong;
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
pub type tlog = libc::c_uint;
pub const LOG_DEBUG_END: tlog = 6;
pub const LOG_DEBUG_CONTINUE: tlog = 5;
pub const LOG_DEBUG_START: tlog = 4;
pub const LOG_DEBUG: tlog = 3;
pub const LOG_FATAL: tlog = 2;
pub const LOG_ERROR: tlog = 1;
pub const LOG_NORMAL: tlog = 0;
pub static mut iDebug: libc::c_int = 0;
static mut azDebug_names: [*const libc::c_char; 12] = [
    b"a\0" as *const u8 as *const libc::c_char,
    b"ch\0" as *const u8 as *const libc::c_char,
    b"h\0" as *const u8 as *const libc::c_char,
    b"u\0" as *const u8 as *const libc::c_char,
    b"pr\0" as *const u8 as *const libc::c_char,
    b"po\0" as *const u8 as *const libc::c_char,
    b"co\0" as *const u8 as *const libc::c_char,
    b"s\0" as *const u8 as *const libc::c_char,
    b"e\0" as *const u8 as *const libc::c_char,
    b"i\0" as *const u8 as *const libc::c_char,
    b"o\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
pub unsafe extern "C" fn idebug_parse(mut z: *const libc::c_char) -> libc::c_int {
    let mut zend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut iret: libc::c_int = 0;
    let mut zcopy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ztok: *mut libc::c_char = 0 as *mut libc::c_char;
    if strncasecmp(
        z,
        b"n\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    i = strtol(z as *mut libc::c_char, &mut zend, 0 as libc::c_int) as libc::c_int;
    if *zend as libc::c_int == '\0' as i32 {
        if i > 15 as libc::c_int {
            i = 15 as libc::c_int;
        } else if i < 0 as libc::c_int {
            i = 0 as libc::c_int;
        }
        return ((1 as libc::c_int) << i) - 1 as libc::c_int;
    }
    zcopy = zbufcpy(z);
    iret = 0 as libc::c_int;
    ztok = strtok(zcopy, b", \t\0" as *const u8 as *const libc::c_char);
    while !ztok.is_null() {
        if strcasecmp(ztok, b"all\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            iret = 0o3777 as libc::c_int;
            break;
        } else {
            i = 0 as libc::c_int;
            while !(azDebug_names[i as usize]).is_null() {
                if strncasecmp(
                    ztok,
                    azDebug_names[i as usize],
                    strlen(azDebug_names[i as usize]),
                ) == 0 as libc::c_int
                {
                    iret |= (1 as libc::c_int) << i;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            if (azDebug_names[i as usize]).is_null() {
                ulog(
                    LOG_ERROR,
                    b"Unrecognized debugging option \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    ztok,
                );
            }
            ztok = strtok(
                0 as *mut libc::c_void as *mut libc::c_char,
                b", \t\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    ubuffree(zcopy);
    return iret;
}
pub unsafe extern "C" fn cdebug_char(
    mut z: *mut libc::c_char,
    mut ichar: libc::c_int,
) -> size_t {
    let mut b: libc::c_char = 0;
    if *(*__ctype_b_loc()).offset(ichar as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
        && ichar != '"' as i32 && ichar != '\\' as i32
    {
        let fresh0 = z;
        z = z.offset(1);
        *fresh0 = ichar as libc::c_char;
        *z = '\0' as i32 as libc::c_char;
        return 1 as libc::c_int as size_t;
    }
    let fresh1 = z;
    z = z.offset(1);
    *fresh1 = '\\' as i32 as libc::c_char;
    match ichar {
        10 => {
            b = 'n' as i32 as libc::c_char;
        }
        13 => {
            b = 'r' as i32 as libc::c_char;
        }
        34 => {
            b = '"' as i32 as libc::c_char;
        }
        92 => {
            b = '\\' as i32 as libc::c_char;
        }
        _ => {
            sprintf(
                z,
                b"%03o\0" as *const u8 as *const libc::c_char,
                ichar as libc::c_uchar as libc::c_uint,
            );
            return (strlen(z)).wrapping_add(1 as libc::c_int as libc::c_ulong);
        }
    }
    let fresh2 = z;
    z = z.offset(1);
    *fresh2 = b;
    *z = '\0' as i32 as libc::c_char;
    return 2 as libc::c_int as size_t;
}
pub unsafe extern "C" fn udebug_buffer(
    mut zhdr: *const libc::c_char,
    mut zbuf: *const libc::c_char,
    mut clen: size_t,
) {
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zalc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    zalc = zbufalc(
        clen
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    z = zalc;
    i = 0 as libc::c_int as size_t;
    while i < clen && i < 80 as libc::c_int as libc::c_ulong {
        z = z.offset(cdebug_char(z, *zbuf.offset(i as isize) as libc::c_int) as isize);
        i = i.wrapping_add(1);
        i;
    }
    if i < clen {
        let fresh3 = z;
        z = z.offset(1);
        *fresh3 = '.' as i32 as libc::c_char;
        let fresh4 = z;
        z = z.offset(1);
        *fresh4 = '.' as i32 as libc::c_char;
        let fresh5 = z;
        z = z.offset(1);
        *fresh5 = '.' as i32 as libc::c_char;
    }
    *z = '\0' as i32 as libc::c_char;
    ulog(
        LOG_DEBUG,
        b"%s %lu \"%s\"\0" as *const u8 as *const libc::c_char,
        zhdr,
        clen,
        zalc,
    );
    ubuffree(zalc);
}
