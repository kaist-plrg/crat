use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn limitSizeToOffT(len: *mut size_t, maxLen: mt_off_t);
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
    fn set_geom_pass_through(
        Stream: *mut Stream_t,
        dev: *mut device_t,
        orig_dev: *mut device_t,
    ) -> libc::c_int;
    fn adjust_tot_sectors(
        dev: *mut device,
        offset: mt_off_t,
        errmsg: *mut libc::c_char,
    ) -> libc::c_int;
    fn str_to_off_with_end(
        str: *const libc::c_char,
        endp: *mut *mut libc::c_char,
    ) -> mt_off_t;
    fn printOom();
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub struct Remap_t {
    pub head: Stream_t,
    pub map: *mut map,
    pub mapSize: libc::c_int,
    pub net_offset: mt_off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct map {
    pub orig: mt_off_t,
    pub remapped: mt_off_t,
    pub type_0: map_type_t,
}
pub type map_type_t = libc::c_uint;
pub const POS: map_type_t = 3;
pub const SKIP: map_type_t = 2;
pub const ZERO: map_type_t = 1;
pub const DATA: map_type_t = 0;
unsafe extern "C" fn remap(
    mut This: *mut Remap_t,
    mut start: *mut mt_off_t,
    mut len: *mut size_t,
) -> map_type_t {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*This).mapSize - 1 as libc::c_int {
        if *start < (*((*This).map).offset((i + 1 as libc::c_int) as isize)).remapped {
            limitSizeToOffT(
                len,
                (*((*This).map).offset((i + 1 as libc::c_int) as isize)).remapped
                    - *start,
            );
            break;
        } else {
            i += 1;
            i;
        }
    }
    *start = *start - (*((*This).map).offset(i as isize)).remapped
        + (*((*This).map).offset(i as isize)).orig;
    return (*((*This).map).offset(i as isize)).type_0;
}
unsafe extern "C" fn remap_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Remap_t = Stream as *mut Remap_t;
    if remap(This, &mut start, &mut len) as libc::c_uint
        == DATA as libc::c_int as libc::c_uint
    {
        return ((*(*(*This).head.Next).Class).pread)
            .unwrap()((*This).head.Next, buf, start, len)
    } else {
        memset(buf as *mut libc::c_void, 0 as libc::c_int, len);
        return len as ssize_t;
    };
}
unsafe extern "C" fn remap_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut This: *mut Remap_t = Stream as *mut Remap_t;
    if remap(This, &mut start, &mut len) as libc::c_uint
        == DATA as libc::c_int as libc::c_uint
    {
        return ((*(*(*This).head.Next).Class).pwrite)
            .unwrap()((*This).head.Next, buf, start, len)
    } else {
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong) < len {
            if *buf.offset(i as isize) != 0 {
                fprintf(
                    stderr,
                    b"Bad data written to unmapped sectors\n\0" as *const u8
                        as *const libc::c_char,
                );
                *__errno_location() = 14 as libc::c_int;
                return -(1 as libc::c_int) as ssize_t;
            }
            i = i.wrapping_add(1);
            i;
        }
        return len as ssize_t;
    };
}
unsafe extern "C" fn remap_free(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut Remap_t = Stream as *mut Remap_t;
    if !((*This).map).is_null() {
        free((*This).map as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
static mut RemapClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                remap_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                remap_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: None,
            freeFunc: Some(
                remap_free as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
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
unsafe extern "C" fn process_map(
    mut This: *mut Remap_t,
    mut ptr: *const libc::c_char,
    mut countOnly: libc::c_int,
    mut errmsg: *mut libc::c_char,
) -> libc::c_int {
    let mut orig: mt_off_t = 0 as libc::c_int as mt_off_t;
    let mut remapped: mt_off_t = 0 as libc::c_int as mt_off_t;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut atEnd: libc::c_int = 0 as libc::c_int;
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    while atEnd == 0 {
        let mut len: mt_off_t = 0;
        let mut type_0: map_type_t = DATA;
        if *ptr as libc::c_int == '\0' as i32 {
            type_0 = DATA;
            atEnd = 1 as libc::c_int;
        } else if strncmp(
            ptr,
            b"skip\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            type_0 = SKIP;
            ptr = ptr.offset(4 as libc::c_int as isize);
        } else if strncmp(
            ptr,
            b"zero\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            type_0 = ZERO;
            ptr = ptr.offset(4 as libc::c_int as isize);
        } else if strncmp(
            ptr,
            b"pos\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            type_0 = POS;
            ptr = ptr.offset(3 as libc::c_int as isize);
        } else {
            type_0 = DATA;
        }
        len = str_to_off_with_end(ptr, &mut eptr);
        ptr = eptr;
        match *ptr as libc::c_int {
            0 => {}
            44 => {
                ptr = ptr.offset(1);
                ptr;
            }
            _ => {
                sprintf(
                    errmsg,
                    b"Bad number %s\n\0" as *const u8 as *const libc::c_char,
                    ptr,
                );
                return -(1 as libc::c_int);
            }
        }
        if type_0 as libc::c_uint == POS as libc::c_int as libc::c_uint {
            orig = len;
        } else {
            if type_0 as libc::c_uint != SKIP as libc::c_int as libc::c_uint {
                if countOnly == 0 {
                    let mut m: *mut map = ((*This).map).offset(count as isize);
                    (*m).orig = orig;
                    (*m).remapped = remapped;
                    (*m).type_0 = type_0;
                }
                remapped += len;
                count += 1;
                count;
            }
            if type_0 as libc::c_uint != ZERO as libc::c_int as libc::c_uint {
                orig += len;
            }
        }
    }
    (*This).net_offset = orig - remapped;
    return count;
}
pub unsafe extern "C" fn Remap(
    mut Next: *mut Stream_t,
    mut dev: *mut device,
    mut errmsg: *mut libc::c_char,
) -> *mut Stream_t {
    let mut This: *mut Remap_t = 0 as *mut Remap_t;
    let mut nrItems: libc::c_int = 0 as libc::c_int;
    let mut map: *const libc::c_char = (*dev).data_map;
    This = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Remap_t>() as libc::c_ulong,
    ) as *mut Remap_t;
    if This.is_null() {
        printOom();
        return 0 as *mut Stream_t;
    }
    memset(
        This as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Remap_t>() as libc::c_ulong,
    );
    init_head(&mut (*This).head, &mut RemapClass, Next);
    nrItems = process_map(This, map, 1 as libc::c_int, errmsg);
    if nrItems < 0 as libc::c_int {
        free(This as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    (*This)
        .map = calloc(nrItems as size_t, ::std::mem::size_of::<map>() as libc::c_ulong)
        as *mut map;
    if ((*This).map).is_null() {
        printOom();
    } else {
        process_map(This, map, 0 as libc::c_int, errmsg);
        if adjust_tot_sectors(dev, (*This).net_offset, errmsg) < 0 as libc::c_int {
            free((*This).map as *mut libc::c_void);
        } else {
            (*This).mapSize = nrItems;
            return &mut (*This).head;
        }
    }
    free(This as *mut libc::c_void);
    printOom();
    return 0 as *mut Stream_t;
}
