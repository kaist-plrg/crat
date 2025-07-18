use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type doscp_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
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
    fn wcschr(_: *const libc::c_int, _: libc::c_int) -> *mut libc::c_int;
    fn wcspbrk(__wcs: *const wchar_t, __accept: *const wchar_t) -> *mut wchar_t;
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    fn putwc(__wc: wchar_t, __stream: *mut __FILE) -> wint_t;
    fn getDrive(Stream: *mut Stream_t) -> libc::c_char;
    fn getDirentry(Stream: *mut Stream_t) -> *mut direntry_t;
    fn wchar_to_native(
        wchar: *const wchar_t,
        native: *mut libc::c_char,
        len: size_t,
        out_len: size_t,
    ) -> size_t;
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
pub type wchar_t = libc::c_int;
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
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
pub type wint_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub attr: libc::c_uchar,
    pub Case: libc::c_uchar,
    pub ctime_ms: libc::c_uchar,
    pub ctime: [libc::c_uchar; 2],
    pub cdate: [libc::c_uchar; 2],
    pub adate: [libc::c_uchar; 2],
    pub startHi: [libc::c_uchar; 2],
    pub time: [libc::c_uchar; 2],
    pub date: [libc::c_uchar; 2],
    pub start: [libc::c_uchar; 2],
    pub size: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: libc::c_int,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: libc::c_uint,
    pub endSlot: libc::c_uint,
}
pub unsafe extern "C" fn initializeDirentry(
    mut entry: *mut direntry_t,
    mut Dir: *mut Stream_t,
) {
    memset(
        entry as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<direntry_t>() as libc::c_ulong,
    );
    (*entry).entry = -(1 as libc::c_int);
    (*entry).Dir = Dir;
    (*entry).beginSlot = 0 as libc::c_int as libc::c_uint;
    (*entry).endSlot = 0 as libc::c_int as libc::c_uint;
}
pub unsafe extern "C" fn isNotFound(mut entry: *mut direntry_t) -> libc::c_int {
    return ((*entry).entry == -(2 as libc::c_int)) as libc::c_int;
}
pub unsafe extern "C" fn isRootEntry(mut entry: *mut direntry_t) -> libc::c_int {
    return ((*entry).entry == -(3 as libc::c_int)) as libc::c_int;
}
pub unsafe extern "C" fn setEntryForIteration(
    mut entry: *mut direntry_t,
    mut in_0: libc::c_uint,
) {
    let mut out: libc::c_int = in_0 as libc::c_int;
    if out >= 0 as libc::c_int {} else {
        __assert_fail(
            b"out >= 0\0" as *const u8 as *const libc::c_char,
            b"direntry.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 54],
                &[libc::c_char; 54],
            >(b"void setEntryForIteration(direntry_t *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7381: {
        if out >= 0 as libc::c_int {} else {
            __assert_fail(
                b"out >= 0\0" as *const u8 as *const libc::c_char,
                b"direntry.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 54],
                    &[libc::c_char; 54],
                >(b"void setEntryForIteration(direntry_t *, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*entry).entry = out - 1 as libc::c_int;
}
pub unsafe extern "C" fn setEntryToPos(
    mut entry: *mut direntry_t,
    mut in_0: libc::c_uint,
) {
    let mut out: libc::c_int = in_0 as libc::c_int;
    if out >= 0 as libc::c_int {} else {
        __assert_fail(
            b"out >= 0\0" as *const u8 as *const libc::c_char,
            b"direntry.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"void setEntryToPos(direntry_t *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7439: {
        if out >= 0 as libc::c_int {} else {
            __assert_fail(
                b"out >= 0\0" as *const u8 as *const libc::c_char,
                b"direntry.c\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"void setEntryToPos(direntry_t *, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*entry).entry = out;
}
pub unsafe extern "C" fn getEntryAsPos(mut entry: *mut direntry_t) -> libc::c_uint {
    let mut pos: libc::c_int = (*entry).entry;
    if pos >= 0 as libc::c_int {} else {
        __assert_fail(
            b"pos >= 0\0" as *const u8 as *const libc::c_char,
            b"direntry.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"unsigned int getEntryAsPos(direntry_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7491: {
        if pos >= 0 as libc::c_int {} else {
            __assert_fail(
                b"pos >= 0\0" as *const u8 as *const libc::c_char,
                b"direntry.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"unsigned int getEntryAsPos(direntry_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return pos as libc::c_uint;
}
pub unsafe extern "C" fn getNextEntryAsPos(mut entry: *mut direntry_t) -> libc::c_uint {
    let mut pos: libc::c_int = (*entry).entry + 1 as libc::c_int;
    if pos >= 0 as libc::c_int {} else {
        __assert_fail(
            b"pos >= 0\0" as *const u8 as *const libc::c_char,
            b"direntry.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"unsigned int getNextEntryAsPos(direntry_t *)\0"))
                .as_ptr(),
        );
    }
    'c_7544: {
        if pos >= 0 as libc::c_int {} else {
            __assert_fail(
                b"pos >= 0\0" as *const u8 as *const libc::c_char,
                b"direntry.c\0" as *const u8 as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"unsigned int getNextEntryAsPos(direntry_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return pos as libc::c_uint;
}
pub unsafe extern "C" fn getParent(mut entry: *mut direntry_t) -> *mut direntry_t {
    return getDirentry((*entry).Dir);
}
unsafe extern "C" fn getPathLen(mut entry: *mut direntry_t) -> size_t {
    let mut length: size_t = 0 as libc::c_int as size_t;
    loop {
        if isRootEntry(entry) != 0 {
            return length.wrapping_add(3 as libc::c_int as libc::c_ulong);
        }
        length = (length as libc::c_ulong)
            .wrapping_add(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(wcslen(((*entry).name).as_mut_ptr())),
            ) as size_t as size_t;
        entry = getDirentry((*entry).Dir);
    };
}
unsafe extern "C" fn sprintPwd(
    mut entry: *mut direntry_t,
    mut ptr: *mut libc::c_char,
    mut len_available: *mut size_t,
) -> *mut libc::c_char {
    if isRootEntry(entry) != 0 {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = getDrive((*entry).Dir);
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = ':' as i32 as libc::c_char;
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 = '/' as i32 as libc::c_char;
        *len_available = (*len_available as libc::c_ulong)
            .wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t as size_t;
    } else {
        let mut bytes_converted: size_t = 0;
        ptr = sprintPwd(getDirentry((*entry).Dir), ptr, len_available);
        if *ptr.offset(-(1 as libc::c_int) as isize) as libc::c_int != '/' as i32 {
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            *fresh3 = '/' as i32 as libc::c_char;
            *len_available = (*len_available).wrapping_sub(1);
            *len_available;
        }
        bytes_converted = wchar_to_native(
            ((*entry).name).as_mut_ptr(),
            ptr,
            255 as libc::c_int as size_t,
            *len_available,
        );
        ptr = ptr.offset(bytes_converted as isize);
        *len_available = (*len_available as libc::c_ulong).wrapping_sub(bytes_converted)
            as size_t as size_t;
    }
    return ptr;
}
unsafe extern "C" fn _fprintPwd(
    mut f: *mut FILE,
    mut entry: *mut direntry_t,
    mut recurs: libc::c_int,
    mut escape: libc::c_int,
) {
    if isRootEntry(entry) != 0 {
        putc(getDrive((*entry).Dir) as libc::c_int, f);
        putc(':' as i32, f);
        if recurs == 0 {
            putc('/' as i32, f);
        }
    } else {
        _fprintPwd(f, getDirentry((*entry).Dir), 1 as libc::c_int, escape);
        if escape != 0
            && !(wcspbrk(
                ((*entry).name).as_mut_ptr(),
                (*::std::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_int; 4],
                >(b"\"\0\0\0$\0\0\0\\\0\0\0\0\0\0\0"))
                    .as_ptr(),
            ))
                .is_null()
        {
            let mut ptr: *mut wchar_t = 0 as *mut wchar_t;
            putc('/' as i32, f);
            ptr = ((*entry).name).as_mut_ptr();
            while *ptr != 0 {
                if !(wcschr(
                    (*::std::mem::transmute::<
                        &[u8; 16],
                        &[libc::c_int; 4],
                    >(b"\"\0\0\0$\0\0\0\\\0\0\0\0\0\0\0"))
                        .as_ptr(),
                    *ptr,
                ))
                    .is_null()
                {
                    putc('\\' as i32, f);
                }
                putwc(*ptr, f);
                ptr = ptr.offset(1);
                ptr;
            }
        } else {
            let mut tmp: [libc::c_char; 1021] = [0; 1021];
            wchar_to_native(
                ((*entry).name).as_mut_ptr(),
                tmp.as_mut_ptr(),
                255 as libc::c_int as size_t,
                ::std::mem::size_of::<[libc::c_char; 1021]>() as libc::c_ulong,
            );
            fprintf(f, b"/%s\0" as *const u8 as *const libc::c_char, tmp.as_mut_ptr());
        }
    };
}
pub unsafe extern "C" fn fprintPwd(
    mut f: *mut FILE,
    mut entry: *mut direntry_t,
    mut escape: libc::c_int,
) {
    if escape != 0 {
        putc('"' as i32, f);
    }
    _fprintPwd(f, entry, 0 as libc::c_int, escape);
    if escape != 0 {
        putc('"' as i32, f);
    }
}
unsafe extern "C" fn _fprintShortPwd(
    mut f: *mut FILE,
    mut entry: *mut direntry_t,
    mut recurs: libc::c_int,
) {
    if isRootEntry(entry) != 0 {
        putc(getDrive((*entry).Dir) as libc::c_int, f);
        putc(':' as i32, f);
        if recurs == 0 {
            putc('/' as i32, f);
        }
    } else {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        _fprintShortPwd(f, getDirentry((*entry).Dir), 1 as libc::c_int);
        putc('/' as i32, f);
        i = 7 as libc::c_int;
        while i >= 0 as libc::c_int
            && (*entry).dir.name[i as usize] as libc::c_int == ' ' as i32
        {
            i -= 1;
            i;
        }
        j = 0 as libc::c_int;
        while j <= i {
            putc((*entry).dir.name[j as usize] as libc::c_int, f);
            j += 1;
            j;
        }
        i = 2 as libc::c_int;
        while i >= 0 as libc::c_int
            && (*entry).dir.ext[i as usize] as libc::c_int == ' ' as i32
        {
            i -= 1;
            i;
        }
        if i > 0 as libc::c_int {
            putc('.' as i32, f);
        }
        j = 0 as libc::c_int;
        while j <= i {
            putc((*entry).dir.ext[j as usize] as libc::c_int, f);
            j += 1;
            j;
        }
    };
}
pub unsafe extern "C" fn fprintShortPwd(mut f: *mut FILE, mut entry: *mut direntry_t) {
    _fprintShortPwd(f, entry, 0 as libc::c_int);
}
pub unsafe extern "C" fn getPwd(mut entry: *mut direntry_t) -> *mut libc::c_char {
    let mut size: size_t = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_size: size_t = 0;
    size = getPathLen(entry);
    buf_size = size
        .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    ret = malloc(buf_size) as *mut libc::c_char;
    if ret.is_null() {
        return 0 as *mut libc::c_char;
    }
    end = sprintPwd(entry, ret, &mut buf_size);
    *end = '\0' as i32 as libc::c_char;
    return ret;
}
pub unsafe extern "C" fn isSubdirOf(
    mut inside: *mut Stream_t,
    mut outside: *mut Stream_t,
) -> libc::c_int {
    loop {
        if inside == outside {
            return 1 as libc::c_int;
        }
        if isRootEntry(getDirentry(inside)) != 0 {
            return 0 as libc::c_int;
        }
        inside = (*getDirentry(inside)).Dir;
    };
}
