use ::libc;
extern "C" {
    pub type doscp_t;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: libc::c_int,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub pread: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub pwrite: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
    pub flush: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub freeFunc: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
    pub set_geom: Option::<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> libc::c_int,
    >,
    pub get_data: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut libc::c_int,
            *mut uint32_t,
        ) -> libc::c_int,
    >,
    pub pre_allocate: Option::<
        unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> libc::c_int,
    >,
    pub get_dosConvert: Option::<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option::<unsafe extern "C" fn(*mut Stream_t) -> libc::c_int>,
}
pub type mt_off_t = off_t;
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const libc::c_char,
    pub drive: libc::c_char,
    pub fat_bits: libc::c_int,
    pub mode: libc::c_int,
    pub tracks: libc::c_uint,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: libc::c_uint,
    pub offset: off_t,
    pub partition: libc::c_uint,
    pub misc_flags: libc::c_uint,
    pub ssize: uint8_t,
    pub use_2m: libc::c_uint,
    pub precmd: *mut libc::c_char,
    pub file_nr: libc::c_int,
    pub blocksize: libc::c_uint,
    pub codepage: libc::c_uint,
    pub data_map: *const libc::c_char,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut libc::c_char,
    pub cfg_filename: *const libc::c_char,
}
unsafe extern "C" fn force_pio(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
    mut io: Option::<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut libc::c_char,
            mt_off_t,
            size_t,
        ) -> ssize_t,
    >,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    let mut done: libc::c_int = 0 as libc::c_int;
    while len != 0 {
        ret = io.unwrap()(Stream, buf, start, len);
        if ret <= 0 as libc::c_int as libc::c_long {
            if done != 0 { return done as ssize_t } else { return ret }
        }
        if ret as size_t <= len {} else {
            __assert_fail(
                b"(size_t)ret <= len\0" as *const u8 as *const libc::c_char,
                b"force_io.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 107],
                    &[libc::c_char; 107],
                >(
                    b"ssize_t force_pio(Stream_t *, char *, mt_off_t, size_t, ssize_t (*)(Stream_t *, char *, mt_off_t, size_t))\0",
                ))
                    .as_ptr(),
            );
        }
        'c_7049: {
            if ret as size_t <= len {} else {
                __assert_fail(
                    b"(size_t)ret <= len\0" as *const u8 as *const libc::c_char,
                    b"force_io.c\0" as *const u8 as *const libc::c_char,
                    45 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 107],
                        &[libc::c_char; 107],
                    >(
                        b"ssize_t force_pio(Stream_t *, char *, mt_off_t, size_t, ssize_t (*)(Stream_t *, char *, mt_off_t, size_t))\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        start = (start as libc::c_ulong).wrapping_add(ret as size_t) as mt_off_t
            as mt_off_t;
        done = (done as libc::c_long + ret) as libc::c_int;
        len = (len as libc::c_ulong).wrapping_sub(ret as size_t) as size_t as size_t;
        buf = buf.offset(ret as isize);
    }
    return done as ssize_t;
}
unsafe extern "C" fn write_wrapper(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return ((*(*Stream).Class).write).unwrap()(Stream, buf, len);
}
pub unsafe extern "C" fn force_write(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> ssize_t {
    return force_pio(
        Stream,
        buf,
        0 as libc::c_int as mt_off_t,
        len,
        Some(
            write_wrapper
                as unsafe extern "C" fn(
                    *mut Stream_t,
                    *mut libc::c_char,
                    mt_off_t,
                    size_t,
                ) -> ssize_t,
        ),
    );
}
pub unsafe extern "C" fn force_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return force_pio(Stream, buf, start, len, (*(*Stream).Class).pwrite);
}
pub unsafe extern "C" fn force_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return force_pio(Stream, buf, start, len, (*(*Stream).Class).pread);
}
