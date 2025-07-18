use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
    fn log_fatal(
        loggerName: *const libc::c_char,
        logMessage: *const libc::c_char,
        _: ...
    ) -> !;
    fn xcalloc(count: size_t, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub unsafe extern "C" fn pbm_init() -> *mut *mut uint8_t {
    let mut retv: *mut *mut uint8_t = xcalloc(
        0x10000 as libc::c_int as size_t,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    ) as *mut *mut uint8_t;
    return retv;
}
#[inline]
unsafe extern "C" fn bm_check(mut bm: *mut uint8_t, mut v: uint16_t) -> libc::c_int {
    let mut page_idx: uint16_t = (v as libc::c_int >> 3 as libc::c_int) as uint16_t;
    let mut bit_idx: uint8_t = (v as libc::c_int & 0x7 as libc::c_int) as uint8_t;
    return *bm.offset(page_idx as isize) as libc::c_int
        & (1 as libc::c_int) << bit_idx as libc::c_int;
}
#[inline]
unsafe extern "C" fn bm_set(mut bm: *mut uint8_t, mut v: uint16_t) {
    let mut page_idx: uint16_t = (v as libc::c_int >> 3 as libc::c_int) as uint16_t;
    let mut bit_idx: uint8_t = (v as libc::c_int & 0x7 as libc::c_int) as uint8_t;
    let ref mut fresh0 = *bm.offset(page_idx as isize);
    *fresh0 = (*fresh0 as libc::c_int | (1 as libc::c_int) << bit_idx as libc::c_int)
        as uint8_t;
}
pub unsafe extern "C" fn pbm_check(
    mut b: *mut *mut uint8_t,
    mut v: uint32_t,
) -> libc::c_int {
    let mut top: uint32_t = v >> 16 as libc::c_int;
    let mut bottom: uint32_t = v & 0xffff as libc::c_int as libc::c_uint;
    return (!(*b.offset(top as isize)).is_null()
        && bm_check(*b.offset(top as isize), bottom as uint16_t) != 0) as libc::c_int;
}
pub unsafe extern "C" fn pbm_set(mut b: *mut *mut uint8_t, mut v: uint32_t) {
    let mut top: uint16_t = (v >> 16 as libc::c_int) as uint16_t;
    let mut bottom: uint16_t = (v & 0xffff as libc::c_int as libc::c_uint) as uint16_t;
    if (*b.offset(top as isize)).is_null() {
        let mut bm: *mut uint8_t = xmalloc(
            (0x10000 as libc::c_int / 8 as libc::c_int) as size_t,
        ) as *mut uint8_t;
        memset(
            bm as *mut libc::c_void,
            0 as libc::c_int,
            (0x10000 as libc::c_int / 8 as libc::c_int) as libc::c_ulong,
        );
        let ref mut fresh1 = *b.offset(top as isize);
        *fresh1 = bm;
    }
    bm_set(*b.offset(top as isize), bottom);
}
pub unsafe extern "C" fn pbm_load_from_file(
    mut b: *mut *mut uint8_t,
    mut file: *mut libc::c_char,
) -> uint32_t {
    if b.is_null() {
        log_fatal(
            b"pbm\0" as *const u8 as *const libc::c_char,
            b"load_from_file called with NULL PBM\0" as *const u8 as *const libc::c_char,
        );
    }
    if file.is_null() {
        log_fatal(
            b"pbm\0" as *const u8 as *const libc::c_char,
            b"load_from_file called with NULL filename\0" as *const u8
                as *const libc::c_char,
        );
    }
    let mut fp: *mut FILE = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        log_fatal(
            b"pbm\0" as *const u8 as *const libc::c_char,
            b"unable to open file: %s: %s\0" as *const u8 as *const libc::c_char,
            file,
            strerror(*__errno_location()),
        );
    }
    let mut line: [libc::c_char; 1000] = [0; 1000];
    let mut count: uint32_t = 0 as libc::c_int as uint32_t;
    while !(fgets(
        line.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        let mut comment: *mut libc::c_char = strchr(line.as_mut_ptr(), '#' as i32);
        if !comment.is_null() {
            *comment = '\0' as i32 as libc::c_char;
        }
        let mut addr: in_addr = in_addr { s_addr: 0 };
        if inet_aton(line.as_mut_ptr(), &mut addr) != 1 as libc::c_int {
            log_fatal(
                b"pbm\0" as *const u8 as *const libc::c_char,
                b"unable to parse IP address: %s\0" as *const u8 as *const libc::c_char,
                line.as_mut_ptr(),
            );
        }
        pbm_set(b, addr.s_addr);
        count = count.wrapping_add(1);
        count;
    }
    fclose(fp);
    return count;
}
