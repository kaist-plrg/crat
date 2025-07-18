use ::libc;
extern "C" {
    pub type doscp_t;
    fn free(__ptr: *mut libc::c_void);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
pub type smt_off_t = mt_off_t;
pub static mut batchmode: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn limitSizeToOffT(mut len: *mut size_t, mut maxLen: mt_off_t) {
    if *len > maxLen as size_t {
        *len = maxLen as size_t;
    }
}
pub unsafe extern "C" fn init_head(
    mut Stream: *mut Stream_t,
    mut Class: *mut Class_t,
    mut Next: *mut Stream_t,
) {
    (*Stream).Class = Class;
    (*Stream).refs = 1 as libc::c_int;
    (*Stream).Next = Next;
}
pub unsafe extern "C" fn flush_stream(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if batchmode == 0 {
        if ((*(*Stream).Class).flush).is_some() {
            ret |= ((*(*Stream).Class).flush).unwrap()(Stream);
        }
        if !((*Stream).Next).is_null() {
            ret |= flush_stream((*Stream).Next);
        }
    }
    return ret;
}
pub unsafe extern "C" fn copy_stream(mut Stream: *mut Stream_t) -> *mut Stream_t {
    if !Stream.is_null() {
        (*Stream).refs += 1;
        (*Stream).refs;
    }
    return Stream;
}
pub unsafe extern "C" fn free_stream(mut Stream: *mut *mut Stream_t) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*Stream).is_null() {
        return -(1 as libc::c_int);
    }
    (**Stream).refs -= 1;
    if (**Stream).refs == 0 {
        if ((*(**Stream).Class).flush).is_some() {
            ret |= ((*(**Stream).Class).flush).unwrap()(*Stream);
        }
        if ((*(**Stream).Class).freeFunc).is_some() {
            ret |= ((*(**Stream).Class).freeFunc).unwrap()(*Stream);
        }
        if !((**Stream).Next).is_null() {
            ret |= free_stream(&mut (**Stream).Next);
        }
        free(*Stream as *mut libc::c_char as *mut libc::c_void);
    }
    *Stream = 0 as *mut Stream_t;
    return ret;
}
pub unsafe extern "C" fn set_geom_pass_through(
    mut Stream: *mut Stream_t,
    mut dev: *mut device_t,
    mut orig_dev: *mut device_t,
) -> libc::c_int {
    return ((*(*(*Stream).Next).Class).set_geom).unwrap()((*Stream).Next, dev, orig_dev);
}
pub unsafe extern "C" fn set_geom_noop(
    mut Stream: *mut Stream_t,
    mut dev: *mut device_t,
    mut orig_dev: *mut device_t,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_data_pass_through(
    mut Stream: *mut Stream_t,
    mut date: *mut time_t,
    mut size: *mut mt_off_t,
    mut type_0: *mut libc::c_int,
    mut address: *mut uint32_t,
) -> libc::c_int {
    return ((*(*(*Stream).Next).Class).get_data)
        .unwrap()((*Stream).Next, date, size, type_0, address);
}
pub unsafe extern "C" fn pread_pass_through(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return ((*(*(*Stream).Next).Class).pread).unwrap()((*Stream).Next, buf, start, len);
}
pub unsafe extern "C" fn pwrite_pass_through(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    return ((*(*(*Stream).Next).Class).pwrite).unwrap()((*Stream).Next, buf, start, len);
}
pub unsafe extern "C" fn get_dosConvert_pass_through(
    mut Stream: *mut Stream_t,
) -> *mut doscp_t {
    return ((*(*(*Stream).Next).Class).get_dosConvert).unwrap()((*Stream).Next);
}
pub unsafe extern "C" fn adjust_tot_sectors(
    mut dev: *mut device,
    mut offset: mt_off_t,
    mut errmsg: *mut libc::c_char,
) -> libc::c_int {
    let mut offs_sectors: mt_off_t = 0;
    if (*dev).tot_sectors == 0 {
        return 0 as libc::c_int;
    }
    offs_sectors = offset
        / (if (*dev).sector_size as libc::c_int != 0 {
            (*dev).sector_size as libc::c_int
        } else {
            512 as libc::c_int
        }) as libc::c_long;
    if offs_sectors > 0 as libc::c_int as libc::c_long
        && ((*dev).tot_sectors as libc::c_long) < offs_sectors
    {
        if !errmsg.is_null() {
            sprintf(
                errmsg,
                b"init: Offset bigger than base image\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    (*dev)
        .tot_sectors = ((*dev).tot_sectors as libc::c_uint)
        .wrapping_sub(offs_sectors as uint32_t) as uint32_t as uint32_t;
    return 0 as libc::c_int;
}
