use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fifo {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
    pub rpnt: size_t,
    pub wrts: size_t,
}
pub unsafe extern "C" fn fifo_init(mut f: *mut fifo) -> libc::c_int {
    (*f).buffer = 0 as *mut libc::c_uchar;
    (*f).size = 0 as libc::c_int as size_t;
    (*f).rpnt = 0 as libc::c_int as size_t;
    (*f).wrts = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fifo_free(mut f: *mut fifo) -> libc::c_int {
    if !((*f).buffer).is_null() {
        free((*f).buffer as *mut libc::c_void);
    }
    fifo_init(f);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fifo_write(
    mut f: *mut fifo,
    mut vbuffer: *mut libc::c_void,
    mut size: size_t,
) -> libc::c_int {
    let mut buffer: *mut libc::c_uchar = vbuffer as *mut libc::c_uchar;
    let mut msize: size_t = 0;
    let mut wpnt: size_t = 0;
    let mut osize: size_t = 0;
    if size > ((*f).size).wrapping_sub((*f).wrts) {
        osize = (*f).size;
        (*f)
            .size = ((*f).wrts)
            .wrapping_add(size)
            .wrapping_add(256 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        (*f)
            .size = (256 as libc::c_int as libc::c_ulong)
            .wrapping_mul(((*f).size).wrapping_div(256 as libc::c_int as libc::c_ulong));
        (*f)
            .buffer = realloc((*f).buffer as *mut libc::c_void, (*f).size)
            as *mut libc::c_uchar;
        if ((*f).buffer).is_null() && (*f).size > 0 as libc::c_int as libc::c_ulong {
            fprintf(
                stderr,
                b"fifo.c: %s.\n\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    b"memory exhausted\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            abort();
        }
        if ((*f).rpnt).wrapping_add((*f).wrts) > osize {
            memmove(
                ((*f).buffer)
                    .offset((*f).rpnt as isize)
                    .offset(((*f).size).wrapping_sub(osize) as isize)
                    as *mut libc::c_void,
                ((*f).buffer).offset((*f).rpnt as isize) as *const libc::c_void,
                osize.wrapping_sub((*f).rpnt),
            );
            (*f)
                .rpnt = ((*f).rpnt as libc::c_ulong)
                .wrapping_add(((*f).size).wrapping_sub(osize)) as size_t as size_t;
        }
    }
    while size > 0 as libc::c_int as libc::c_ulong {
        wpnt = ((*f).rpnt).wrapping_add((*f).wrts);
        if wpnt >= (*f).size {
            wpnt = (wpnt as libc::c_ulong).wrapping_sub((*f).size) as size_t as size_t;
        }
        msize = ((*f).size).wrapping_sub(wpnt);
        if size < msize {
            msize = size;
        }
        if !buffer.is_null() {
            memcpy(
                ((*f).buffer).offset(wpnt as isize) as *mut libc::c_void,
                buffer as *const libc::c_void,
                msize,
            );
            buffer = buffer.offset(msize as isize);
        }
        (*f).wrts = ((*f).wrts as libc::c_ulong).wrapping_add(msize) as size_t as size_t;
        size = (size as libc::c_ulong).wrapping_sub(msize) as size_t as size_t;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn fifo_available(mut f: *mut fifo) -> size_t {
    return (*f).wrts;
}
pub unsafe extern "C" fn fifo_read(
    mut f: *mut fifo,
    mut vbuffer: *mut libc::c_void,
    mut size: size_t,
) -> size_t {
    let mut buffer: *mut libc::c_uchar = vbuffer as *mut libc::c_uchar;
    let mut rsize: size_t = 0;
    let mut rsize0: size_t = 0;
    let mut msize: size_t = 0;
    if size < (*f).wrts {
        rsize = size;
    } else {
        rsize = (*f).wrts;
    }
    rsize0 = rsize;
    while rsize > 0 as libc::c_int as libc::c_ulong {
        msize = ((*f).size).wrapping_sub((*f).rpnt);
        if rsize < msize {
            msize = rsize;
        }
        if !buffer.is_null() {
            memcpy(
                buffer as *mut libc::c_void,
                ((*f).buffer).offset((*f).rpnt as isize) as *const libc::c_void,
                msize,
            );
            buffer = buffer.offset(msize as isize);
        }
        (*f).rpnt = ((*f).rpnt as libc::c_ulong).wrapping_add(msize) as size_t as size_t;
        (*f).wrts = ((*f).wrts as libc::c_ulong).wrapping_sub(msize) as size_t as size_t;
        if (*f).rpnt >= (*f).size {
            (*f).rpnt = 0 as libc::c_int as size_t;
        }
        rsize = (rsize as libc::c_ulong).wrapping_sub(msize) as size_t as size_t;
    }
    return rsize0;
}
pub unsafe extern "C" fn fifo_skip(mut f: *mut fifo, mut size: size_t) -> size_t {
    return fifo_read(f, 0 as *mut libc::c_void, size);
}
pub unsafe extern "C" fn fifo_peek(
    mut f: *mut fifo,
    mut vbuffer: *mut libc::c_void,
    mut size: size_t,
) -> size_t {
    let mut rpnt: size_t = 0;
    let mut wrts: size_t = 0;
    let mut ret: size_t = 0;
    rpnt = (*f).rpnt;
    wrts = (*f).wrts;
    ret = fifo_read(f, vbuffer, size);
    (*f).rpnt = rpnt;
    (*f).wrts = wrts;
    return ret;
}
pub unsafe extern "C" fn fifo_flush(
    mut f: *mut fifo,
    mut vbuffer: *mut libc::c_void,
) -> size_t {
    let mut size: size_t = 0;
    size = fifo_available(f);
    if size > 0 as libc::c_int as libc::c_ulong {
        fifo_read(f, vbuffer, size);
    }
    return size;
}
