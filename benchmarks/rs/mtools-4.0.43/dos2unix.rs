use ::libc;
extern "C" {
    pub type doscp_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn get_data_pass_through(
        Stream: *mut Stream_t,
        date: *mut time_t,
        size: *mut mt_off_t,
        type_0: *mut libc::c_int,
        address: *mut uint32_t,
    ) -> libc::c_int;
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
pub type mt_off_t = off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Filter_t {
    pub head: Stream_t,
    pub mode: libc::c_int,
}
unsafe extern "C" fn read_filter(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Filter_t = Stream as *mut Filter_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut ret: ssize_t = 0;
    let mut newchar: libc::c_char = 0;
    ret = ((*(*(*This).head.Next).Class).read).unwrap()((*This).head.Next, buf, len);
    if ret < 0 as libc::c_int as libc::c_long {
        return ret;
    }
    j = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < ret as size_t {
        if !(*buf.offset(i as isize) as libc::c_int == '\r' as i32) {
            if *buf.offset(i as isize) as libc::c_int == 0x1a as libc::c_int {
                break;
            }
            newchar = *buf.offset(i as isize);
            let fresh0 = j;
            j = j.wrapping_add(1);
            *buf.offset(fresh0 as isize) = newchar;
        }
        i = i.wrapping_add(1);
        i;
    }
    return j as ssize_t;
}
static mut FilterClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: Some(
                read_filter
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        size_t,
                    ) -> ssize_t,
            ),
            write: None,
            pread: None,
            pwrite: None,
            flush: None,
            freeFunc: None,
            set_geom: None,
            get_data: Some(
                get_data_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut time_t,
                        *mut mt_off_t,
                        *mut libc::c_int,
                        *mut uint32_t,
                    ) -> libc::c_int,
            ),
            pre_allocate: None,
            get_dosConvert: None,
            discard: None,
        };
        init
    }
};
pub unsafe extern "C" fn open_dos2unix(
    mut Next: *mut Stream_t,
    mut convertCharset: libc::c_int,
) -> *mut Stream_t {
    let mut This: *mut Filter_t = 0 as *mut Filter_t;
    This = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Filter_t>() as libc::c_ulong,
    ) as *mut Filter_t;
    if This.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*This).head, &mut FilterClass, Next);
    return &mut (*This).head;
}
