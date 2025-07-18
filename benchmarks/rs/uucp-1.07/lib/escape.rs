use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn ulog(ttype: tlog, zfmt: *const libc::c_char, _: ...);
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
pub unsafe extern "C" fn cescape(mut z: *mut libc::c_char) -> size_t {
    let mut zto: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zfrom: *mut libc::c_char = 0 as *mut libc::c_char;
    zto = z;
    zfrom = z;
    while *zfrom as libc::c_int != '\0' as i32 {
        if *zfrom as libc::c_int != '\\' as i32 {
            let fresh0 = zfrom;
            zfrom = zfrom.offset(1);
            let fresh1 = zto;
            zto = zto.offset(1);
            *fresh1 = *fresh0;
        } else {
            zfrom = zfrom.offset(1);
            zfrom;
            let mut current_block_31: u64;
            match *zfrom as libc::c_int {
                45 => {
                    let fresh2 = zto;
                    zto = zto.offset(1);
                    *fresh2 = '-' as i32 as libc::c_char;
                    current_block_31 = 13131896068329595644;
                }
                98 => {
                    let fresh3 = zto;
                    zto = zto.offset(1);
                    *fresh3 = '\u{8}' as i32 as libc::c_char;
                    current_block_31 = 13131896068329595644;
                }
                110 => {
                    let fresh4 = zto;
                    zto = zto.offset(1);
                    *fresh4 = '\n' as i32 as libc::c_char;
                    current_block_31 = 13131896068329595644;
                }
                78 => {
                    let fresh5 = zto;
                    zto = zto.offset(1);
                    *fresh5 = '\0' as i32 as libc::c_char;
                    current_block_31 = 13131896068329595644;
                }
                114 => {
                    let fresh6 = zto;
                    zto = zto.offset(1);
                    *fresh6 = '\r' as i32 as libc::c_char;
                    current_block_31 = 13131896068329595644;
                }
                115 => {
                    let fresh7 = zto;
                    zto = zto.offset(1);
                    *fresh7 = ' ' as i32 as libc::c_char;
                    current_block_31 = 13131896068329595644;
                }
                116 => {
                    let fresh8 = zto;
                    zto = zto.offset(1);
                    *fresh8 = '\t' as i32 as libc::c_char;
                    current_block_31 = 13131896068329595644;
                }
                0 => {
                    zfrom = zfrom.offset(-1);
                    zfrom;
                    current_block_31 = 10388410863253996547;
                }
                92 => {
                    current_block_31 = 10388410863253996547;
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    let mut i: libc::c_int = 0;
                    i = *zfrom as libc::c_int - '0' as i32;
                    if *zfrom.offset(1 as libc::c_int as isize) as libc::c_int
                        >= '0' as i32
                        && *zfrom.offset(1 as libc::c_int as isize) as libc::c_int
                            <= '7' as i32
                    {
                        zfrom = zfrom.offset(1);
                        i = 8 as libc::c_int * i + *zfrom as libc::c_int - '0' as i32;
                    }
                    if *zfrom.offset(1 as libc::c_int as isize) as libc::c_int
                        >= '0' as i32
                        && *zfrom.offset(1 as libc::c_int as isize) as libc::c_int
                            <= '7' as i32
                    {
                        zfrom = zfrom.offset(1);
                        i = 8 as libc::c_int * i + *zfrom as libc::c_int - '0' as i32;
                    }
                    let fresh10 = zto;
                    zto = zto.offset(1);
                    *fresh10 = i as libc::c_char;
                    current_block_31 = 13131896068329595644;
                }
                120 => {
                    let mut i_0: libc::c_int = 0;
                    i_0 = 0 as libc::c_int;
                    while *(*__ctype_b_loc())
                        .offset(
                            *zfrom.offset(1 as libc::c_int as isize) as libc::c_uchar
                                as libc::c_int as isize,
                        ) as libc::c_int
                        & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        if *(*__ctype_b_loc())
                            .offset(
                                *zfrom.offset(1 as libc::c_int as isize) as libc::c_uchar
                                    as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            zfrom = zfrom.offset(1);
                            i_0 = 16 as libc::c_int * i_0 + *zfrom as libc::c_int
                                - '0' as i32;
                        } else if *(*__ctype_b_loc())
                            .offset(
                                *zfrom.offset(1 as libc::c_int as isize) as libc::c_uchar
                                    as libc::c_int as isize,
                            ) as libc::c_int
                            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            zfrom = zfrom.offset(1);
                            i_0 = 16 as libc::c_int * i_0 + *zfrom as libc::c_int
                                - 'A' as i32 + 10 as libc::c_int;
                        } else {
                            zfrom = zfrom.offset(1);
                            i_0 = 16 as libc::c_int * i_0 + *zfrom as libc::c_int
                                - 'a' as i32 + 10 as libc::c_int;
                        }
                    }
                    let fresh11 = zto;
                    zto = zto.offset(1);
                    *fresh11 = i_0 as libc::c_char;
                    current_block_31 = 13131896068329595644;
                }
                _ => {
                    ulog(
                        LOG_ERROR,
                        b"Unrecognized escape sequence \\%c\0" as *const u8
                            as *const libc::c_char,
                        *zfrom as libc::c_int,
                    );
                    let fresh12 = zto;
                    zto = zto.offset(1);
                    *fresh12 = *zfrom;
                    current_block_31 = 13131896068329595644;
                }
            }
            match current_block_31 {
                10388410863253996547 => {
                    let fresh9 = zto;
                    zto = zto.offset(1);
                    *fresh9 = '\\' as i32 as libc::c_char;
                }
                _ => {}
            }
            zfrom = zfrom.offset(1);
            zfrom;
        }
    }
    *zto = '\0' as i32 as libc::c_char;
    return zto.offset_from(z) as libc::c_long as size_t;
}
