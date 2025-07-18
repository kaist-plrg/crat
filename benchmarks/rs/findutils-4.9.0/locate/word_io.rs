use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn clearerr(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
pub type GetwordEndianState = libc::c_uint;
pub const GetwordEndianStateSwab: GetwordEndianState = 2;
pub const GetwordEndianStateNative: GetwordEndianState = 1;
pub const GetwordEndianStateInitial: GetwordEndianState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ival: libc::c_int,
    pub data: [libc::c_uchar; 4],
}
pub const WORDBYTES: C2RustUnnamed_2 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
pub type C2RustUnnamed_2 = libc::c_uint;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
unsafe extern "C" fn decode_value(
    mut data: *const libc::c_uchar,
    mut limit: libc::c_int,
    mut endian_state_flag: *mut GetwordEndianState,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut swapped: libc::c_int = 0;
    let mut u: C2RustUnnamed = C2RustUnnamed { ival: 0 };
    u.ival = 0 as libc::c_int;
    memcpy(
        &mut u.data as *mut [libc::c_uchar; 4] as *mut libc::c_void,
        data as *const libc::c_void,
        WORDBYTES as libc::c_int as libc::c_ulong,
    );
    swapped = __bswap_32(u.ival as __uint32_t) as libc::c_int;
    if *endian_state_flag as libc::c_uint
        == GetwordEndianStateInitial as libc::c_int as libc::c_uint
    {
        if u.ival <= limit {
            if swapped > limit {
                *endian_state_flag = GetwordEndianStateNative;
            }
            return u.ival;
        } else if swapped <= limit {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARNING: locate database %s was built with a different byte order\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, filename),
            );
            *endian_state_flag = GetwordEndianStateSwab;
            return swapped;
        } else {
            return u.ival
        }
    } else if *endian_state_flag as libc::c_uint
        == GetwordEndianStateSwab as libc::c_int as libc::c_uint
    {
        return swapped
    } else {
        return u.ival
    };
}
pub unsafe extern "C" fn getword(
    mut fp: *mut FILE,
    mut filename: *const libc::c_char,
    mut maxvalue: size_t,
    mut endian_state_flag: *mut GetwordEndianState,
) -> libc::c_int {
    let mut data: [libc::c_uchar; 4] = [0; 4];
    let mut bytes_read: size_t = 0;
    clearerr(fp);
    bytes_read = fread(
        data.as_mut_ptr() as *mut libc::c_void,
        WORDBYTES as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        fp,
    );
    if bytes_read != 1 as libc::c_int as libc::c_ulong {
        let mut quoted_name: *const libc::c_char = quotearg_n_style(
            0 as libc::c_int,
            locale_quoting_style,
            filename,
        );
        if feof(fp) != 0 {
            if ::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected EOF in %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quoted_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected EOF in %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quoted_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            if ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error reading a word from %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quoted_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error reading a word from %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quoted_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        abort();
    } else {
        return decode_value(
            data.as_mut_ptr() as *const libc::c_uchar,
            maxvalue as libc::c_int,
            endian_state_flag,
            filename,
        )
    };
}
