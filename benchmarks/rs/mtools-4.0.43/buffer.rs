use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn get_dosConvert_pass_through(Stream: *mut Stream_t) -> *mut doscp_t;
    fn force_pwrite(
        Stream: *mut Stream_t,
        buf: *mut libc::c_char,
        start: mt_off_t,
        len: size_t,
    ) -> ssize_t;
    fn get_data_pass_through(
        Stream: *mut Stream_t,
        date: *mut time_t,
        size: *mut mt_off_t,
        type_0: *mut libc::c_int,
        address: *mut uint32_t,
    ) -> libc::c_int;
    fn init_head(Stream: *mut Stream_t, Class: *mut Class_t, Next: *mut Stream_t);
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
pub struct Buffer_t {
    pub head: Stream_t,
    pub size: size_t,
    pub dirty: libc::c_int,
    pub sectorSize: size_t,
    pub cylinderSize: size_t,
    pub ever_dirty: libc::c_int,
    pub dirty_pos: size_t,
    pub dirty_end: size_t,
    pub current: mt_off_t,
    pub cur_size: size_t,
    pub buf: *mut libc::c_char,
}
pub const ERROR: position_t = 3;
pub const INSIDE: position_t = 2;
pub const APPEND: position_t = 1;
pub const OUTSIDE: position_t = 0;
pub type position_t = libc::c_uint;
unsafe extern "C" fn abs_pos(mut Buffer: *mut Buffer_t, mut rel: size_t) -> mt_off_t {
    return (*Buffer).current + rel as mt_off_t;
}
unsafe extern "C" fn cur_end(mut Buffer: *mut Buffer_t) -> mt_off_t {
    return abs_pos(Buffer, (*Buffer).cur_size);
}
unsafe extern "C" fn pos_to_next_full_cyl(
    mut Buffer: *mut Buffer_t,
    mut pos: mt_off_t,
) -> size_t {
    return ((*Buffer).cylinderSize)
        .wrapping_sub((pos % (*Buffer).cylinderSize as mt_off_t) as size_t);
}
unsafe extern "C" fn _buf_flush(mut Buffer: *mut Buffer_t) -> libc::c_int {
    let mut ret: ssize_t = 0;
    if !((*Buffer).head.Next).is_null() {} else {
        __assert_fail(
            b"Buffer->head.Next != NULL\0" as *const u8 as *const libc::c_char,
            b"buffer.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"int _buf_flush(Buffer_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7387: {
        if !((*Buffer).head.Next).is_null() {} else {
            __assert_fail(
                b"Buffer->head.Next != NULL\0" as *const u8 as *const libc::c_char,
                b"buffer.c\0" as *const u8 as *const libc::c_char,
                69 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"int _buf_flush(Buffer_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*Buffer).dirty == 0 {
        return 0 as libc::c_int;
    }
    ret = force_pwrite(
        (*Buffer).head.Next,
        ((*Buffer).buf).offset((*Buffer).dirty_pos as isize),
        (*Buffer).current + (*Buffer).dirty_pos as mt_off_t,
        ((*Buffer).dirty_end).wrapping_sub((*Buffer).dirty_pos),
    );
    if ret < 0 as libc::c_int as libc::c_long {
        perror(b"buffer_flush: write\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    if ret as size_t != ((*Buffer).dirty_end).wrapping_sub((*Buffer).dirty_pos) {
        fprintf(
            stderr,
            b"buffer_flush: short write\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*Buffer).dirty = 0 as libc::c_int;
    (*Buffer).dirty_end = 0 as libc::c_int as size_t;
    (*Buffer).dirty_pos = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn invalidate_buffer(
    mut Buffer: *mut Buffer_t,
    mut start: mt_off_t,
) -> libc::c_int {
    if _buf_flush(Buffer) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*Buffer).current = start - start % (*Buffer).sectorSize as mt_off_t;
    (*Buffer).cur_size = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn isInBuffer(
    mut This: *mut Buffer_t,
    mut start: mt_off_t,
    mut len: *mut size_t,
) -> position_t {
    if start >= (*This).current && start < cur_end(This) {
        if *len > ((*This).cur_size).wrapping_sub((start - (*This).current) as size_t) {
            *len = ((*This).cur_size).wrapping_sub((start - (*This).current) as size_t);
        }
        return INSIDE;
    } else if start == cur_end(This) && (*This).cur_size < (*This).size
        && *len >= (*This).sectorSize
    {
        if *len > ((*This).size).wrapping_sub((*This).cur_size) {
            *len = ((*This).size).wrapping_sub((*This).cur_size);
        }
        *len = (*len).wrapping_sub((*len).wrapping_rem((*This).sectorSize));
        return APPEND;
    } else {
        if invalidate_buffer(This, start) < 0 as libc::c_int {
            return ERROR;
        }
        if *len
            > ((*This).cylinderSize).wrapping_sub((start - (*This).current) as size_t)
        {
            *len = ((*This).cylinderSize)
                .wrapping_sub((start - (*This).current) as size_t);
        }
        if *len > pos_to_next_full_cyl(This, (*This).current) {
            *len = pos_to_next_full_cyl(This, (*This).current);
        }
        return OUTSIDE;
    };
}
unsafe extern "C" fn buf_pread(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut length: size_t = 0;
    let mut offset: size_t = 0;
    let mut disk_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: ssize_t = 0;
    let mut This: *mut Buffer_t = Stream as *mut Buffer_t;
    if len == 0 {
        return 0 as libc::c_int as ssize_t;
    }
    match isInBuffer(This, start, &mut len) as libc::c_uint {
        0 | 1 => {
            length = pos_to_next_full_cyl(This, cur_end(This));
            if length > ((*This).size).wrapping_sub((*This).cur_size) {
                length = ((*This).size).wrapping_sub((*This).cur_size);
            }
            ret = ((*(*(*This).head.Next).Class).pread)
                .unwrap()(
                (*This).head.Next,
                ((*This).buf).offset((*This).cur_size as isize),
                (*This).current + (*This).cur_size as mt_off_t,
                length,
            );
            if ret < 0 as libc::c_int as libc::c_long {
                return ret;
            }
            (*This)
                .cur_size = ((*This).cur_size as libc::c_ulong)
                .wrapping_add(ret as size_t) as size_t as size_t;
            if ((*This).current + (*This).cur_size as mt_off_t) < start {
                fprintf(
                    stderr,
                    b"Short buffer fill\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        3 => return -(1 as libc::c_int) as ssize_t,
        2 | _ => {}
    }
    offset = (start - (*This).current) as size_t;
    disk_ptr = ((*This).buf).offset(offset as isize);
    if len > ((*This).cur_size).wrapping_sub(offset) {
        len = ((*This).cur_size).wrapping_sub(offset);
    }
    memcpy(buf as *mut libc::c_void, disk_ptr as *const libc::c_void, len);
    return len as ssize_t;
}
unsafe extern "C" fn buf_pwrite(
    mut Stream: *mut Stream_t,
    mut buf: *mut libc::c_char,
    mut start: mt_off_t,
    mut len: size_t,
) -> ssize_t {
    let mut disk_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut This: *mut Buffer_t = Stream as *mut Buffer_t;
    let mut offset: size_t = 0 as libc::c_int as size_t;
    if len == 0 {
        return 0 as libc::c_int as ssize_t;
    }
    (*This).ever_dirty = 1 as libc::c_int;
    let mut current_block_38: u64;
    match isInBuffer(This, start, &mut len) as libc::c_uint {
        0 => {
            if start % (*This).cylinderSize as mt_off_t != 0 || len < (*This).sectorSize
            {
                let mut readSize: size_t = 0;
                let mut ret: ssize_t = 0;
                let mut bytes_read: size_t = 0;
                readSize = ((*This).cylinderSize)
                    .wrapping_sub(
                        ((*This).current % (*This).cylinderSize as mt_off_t) as size_t,
                    );
                ret = ((*(*(*This).head.Next).Class).pread)
                    .unwrap()((*This).head.Next, (*This).buf, (*This).current, readSize);
                if ret < 0 as libc::c_int as libc::c_long {
                    return ret;
                }
                bytes_read = ret as size_t;
                if bytes_read.wrapping_rem((*This).sectorSize) != 0 {
                    fprintf(
                        stderr,
                        b"Weird: read size (%ld) not a multiple of sector size (%d)\n\0"
                            as *const u8 as *const libc::c_char,
                        bytes_read,
                        (*This).sectorSize as libc::c_int,
                    );
                    bytes_read = (bytes_read as libc::c_ulong)
                        .wrapping_sub(bytes_read.wrapping_rem((*This).sectorSize))
                        as size_t as size_t;
                    if bytes_read == 0 as libc::c_int as libc::c_ulong {
                        fprintf(
                            stderr,
                            b"Nothing left\n\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                }
                (*This).cur_size = bytes_read;
                if (*This).cur_size == 0 {
                    memset((*This).buf as *mut libc::c_void, 0 as libc::c_int, readSize);
                    (*This).cur_size = readSize;
                }
                offset = (start - (*This).current) as size_t;
                current_block_38 = 7333393191927787629;
            } else {
                current_block_38 = 14277861673517172765;
            }
        }
        1 => {
            current_block_38 = 14277861673517172765;
        }
        2 => {
            offset = (start - (*This).current) as size_t;
            if len > ((*This).cur_size).wrapping_sub(offset) {
                len = ((*This).cur_size).wrapping_sub(offset);
            }
            current_block_38 = 7333393191927787629;
        }
        3 => return -(1 as libc::c_int) as ssize_t,
        _ => {
            current_block_38 = 7333393191927787629;
        }
    }
    match current_block_38 {
        14277861673517172765 => {
            len = len.wrapping_sub(len.wrapping_rem((*This).sectorSize));
            offset = (start - (*This).current) as size_t;
            if len > ((*This).size).wrapping_sub(offset) {
                len = ((*This).size).wrapping_sub(offset);
            }
            (*This)
                .cur_size = ((*This).cur_size as libc::c_ulong).wrapping_add(len)
                as size_t as size_t;
            if ((*(*(*This).head.Next).Class).pre_allocate).is_some() {
                ((*(*(*This).head.Next).Class).pre_allocate)
                    .unwrap()((*This).head.Next, cur_end(This));
            }
        }
        _ => {}
    }
    disk_ptr = ((*This).buf).offset(offset as isize);
    if offset.wrapping_add(len) > (*This).cur_size {
        len = (len as libc::c_ulong)
            .wrapping_sub(offset.wrapping_add(len).wrapping_rem((*This).sectorSize))
            as size_t as size_t;
        (*This).cur_size = len.wrapping_add(offset);
    }
    memcpy(disk_ptr as *mut libc::c_void, buf as *const libc::c_void, len);
    if (*This).dirty == 0 || offset < (*This).dirty_pos {
        (*This).dirty_pos = offset.wrapping_sub(offset.wrapping_rem((*This).sectorSize));
    }
    if (*This).dirty == 0 || offset.wrapping_add(len) > (*This).dirty_end {
        (*This)
            .dirty_end = offset
            .wrapping_add(len)
            .wrapping_add((*This).sectorSize)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(
                offset
                    .wrapping_add(len)
                    .wrapping_add((*This).sectorSize)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_rem((*This).sectorSize),
            );
    }
    if (*This).dirty_end > (*This).cur_size {
        fprintf(
            stderr,
            b"Internal error, dirty end too big dirty_end=%x cur_size=%x len=%x offset=%d sectorSize=%x\n\0"
                as *const u8 as *const libc::c_char,
            (*This).dirty_end as libc::c_uint,
            (*This).cur_size as libc::c_uint,
            len as libc::c_uint,
            offset as libc::c_int,
            (*This).sectorSize as libc::c_int,
        );
        fprintf(
            stderr,
            b"offset + len + grain - 1 = %x\n\0" as *const u8 as *const libc::c_char,
            offset
                .wrapping_add(len)
                .wrapping_add((*This).sectorSize)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        );
        fprintf(
            stderr,
            b"ROUNDOWN(offset + len + grain - 1) = %x\n\0" as *const u8
                as *const libc::c_char,
            offset
                .wrapping_add(len)
                .wrapping_add((*This).sectorSize)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(
                    offset
                        .wrapping_add(len)
                        .wrapping_add((*This).sectorSize)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_rem((*This).sectorSize),
                ) as libc::c_int,
        );
        fprintf(
            stderr,
            b"This->dirty = %d\n\0" as *const u8 as *const libc::c_char,
            (*This).dirty,
        );
        exit(1 as libc::c_int);
    }
    (*This).dirty = 1 as libc::c_int;
    return len as ssize_t;
}
unsafe extern "C" fn buf_flush(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut This: *mut Buffer_t = Stream as *mut Buffer_t;
    if (*This).ever_dirty == 0 {
        return 0 as libc::c_int;
    }
    ret = _buf_flush(This);
    if ret == 0 as libc::c_int {
        (*This).ever_dirty = 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn buf_free(mut Stream: *mut Stream_t) -> libc::c_int {
    let mut This: *mut Buffer_t = Stream as *mut Buffer_t;
    if !((*This).buf).is_null() {
        free((*This).buf as *mut libc::c_void);
    }
    (*This).buf = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
static mut BufferClass: Class_t = unsafe {
    {
        let mut init = Class_t {
            read: None,
            write: None,
            pread: Some(
                buf_pread
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            pwrite: Some(
                buf_pwrite
                    as unsafe extern "C" fn(
                        *mut Stream_t,
                        *mut libc::c_char,
                        mt_off_t,
                        size_t,
                    ) -> ssize_t,
            ),
            flush: Some(buf_flush as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int),
            freeFunc: Some(
                buf_free as unsafe extern "C" fn(*mut Stream_t) -> libc::c_int,
            ),
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
            get_dosConvert: Some(
                get_dosConvert_pass_through
                    as unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t,
            ),
            discard: None,
        };
        init
    }
};
pub unsafe extern "C" fn buf_init(
    mut Next: *mut Stream_t,
    mut size: size_t,
    mut cylinderSize: size_t,
    mut sectorSize: size_t,
) -> *mut Stream_t {
    let mut Buffer: *mut Buffer_t = 0 as *mut Buffer_t;
    if size != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"size != 0\0" as *const u8 as *const libc::c_char,
            b"buffer.c\0" as *const u8 as *const libc::c_char,
            364 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_8959: {
        if size != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"size != 0\0" as *const u8 as *const libc::c_char,
                b"buffer.c\0" as *const u8 as *const libc::c_char,
                364 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if cylinderSize != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"cylinderSize != 0\0" as *const u8 as *const libc::c_char,
            b"buffer.c\0" as *const u8 as *const libc::c_char,
            365 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_8921: {
        if cylinderSize != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"cylinderSize != 0\0" as *const u8 as *const libc::c_char,
                b"buffer.c\0" as *const u8 as *const libc::c_char,
                365 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if sectorSize != 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"sectorSize != 0\0" as *const u8 as *const libc::c_char,
            b"buffer.c\0" as *const u8 as *const libc::c_char,
            366 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_8882: {
        if sectorSize != 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"sectorSize != 0\0" as *const u8 as *const libc::c_char,
                b"buffer.c\0" as *const u8 as *const libc::c_char,
                366 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if !Next.is_null() {} else {
        __assert_fail(
            b"Next != NULL\0" as *const u8 as *const libc::c_char,
            b"buffer.c\0" as *const u8 as *const libc::c_char,
            367 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_8838: {
        if !Next.is_null() {} else {
            __assert_fail(
                b"Next != NULL\0" as *const u8 as *const libc::c_char,
                b"buffer.c\0" as *const u8 as *const libc::c_char,
                367 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"Stream_t *buf_init(Stream_t *, size_t, size_t, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if size.wrapping_rem(cylinderSize) != 0 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"size not multiple of cylinder size\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if cylinderSize.wrapping_rem(sectorSize) != 0 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"cylinder size not multiple of sector size\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    Buffer = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<Buffer_t>() as libc::c_ulong,
    ) as *mut Buffer_t;
    if Buffer.is_null() {
        return 0 as *mut Stream_t;
    }
    init_head(&mut (*Buffer).head, &mut BufferClass, Next);
    (*Buffer).buf = malloc(size) as *mut libc::c_char;
    if ((*Buffer).buf).is_null() {
        free(Buffer as *mut libc::c_char as *mut libc::c_void);
        return 0 as *mut Stream_t;
    }
    (*Buffer).size = size;
    (*Buffer).dirty = 0 as libc::c_int;
    (*Buffer).cylinderSize = cylinderSize;
    (*Buffer).sectorSize = sectorSize;
    (*Buffer).ever_dirty = 0 as libc::c_int;
    (*Buffer).dirty_pos = 0 as libc::c_int as size_t;
    (*Buffer).dirty_end = 0 as libc::c_int as size_t;
    (*Buffer).current = 0 as libc::c_long;
    (*Buffer).cur_size = 0 as libc::c_int as size_t;
    return &mut (*Buffer).head;
}
