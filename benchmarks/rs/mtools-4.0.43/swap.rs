use ::libc;
extern "C" {
    pub type doscp_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn printOom();
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn set_geom_pass_through(
        Stream: *mut Stream_t,
        dev: *mut device_t,
        orig_dev: *mut device_t,
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
pub struct Swap_t {
    pub head: Stream_t,
}
unsafe extern "C" fn swap_buffer(mut buf: *mut libc::c_char, mut len: size_t) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < len {
        let mut temp: libc::c_char = *buf.offset(i as isize);
        *buf
            .offset(
                i as isize,
            ) = *buf.offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
        *buf.offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize) = temp;
        i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn swap_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Swap_t = Stream as *mut Swap_t;
    let mut result: ssize_t = ((*(*(*This).head.Next).Class).pread)
        .unwrap()((*This).head.Next, buf, where_0, len);
    if result < 0 as libc::c_int as libc::c_long {
        return result;
    }
    swap_buffer(buf, result as size_t);
    return result;
}
unsafe extern "C" fn swap_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut where_0: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Swap_t = Stream as *mut Swap_t;
    let mut result: ssize_t = 0;
    let mut swapping: *mut libc::c_char = malloc(len) as *mut libc::c_char;
    memcpy(swapping as *mut libc::c_void, buf as *const libc::c_void, len);
    swap_buffer(swapping, len);
    result = ((*(*(*This).head.Next).Class).pwrite)
        .unwrap()((*This).head.Next, swapping, where_0, len);
    free(swapping as *mut libc::c_void);
    return result;
}
static mut SwapClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                swap_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                swap_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: None,
            freeFunc: None,
            set_geom: Some(
                set_geom_pass_through
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut device_t,
                        *mut device_t,
                    ) -> libc::c_int,
            ),
            get_data: None,
            pre_allocate: None,
            get_dosConvert: Some(
                get_dosConvert_pass_through
                    as unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t,
            ),
            discard: None,
        };
        init
    }
};
pub unsafe extern "C" fn OpenSwap(mut Next: *mut Stream_t) -> *mut Stream_t {
    let mut This: *mut Swap_t = 0 as *mut Swap_t;
    This = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Swap_t>() as libc::c_ulong,
    ) as *mut Swap_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Swap_t>() as libc::c_ulong,
    );
    init_head(&mut (*This).head, &mut SwapClass, Next);
    return &mut (*This).head;
}
