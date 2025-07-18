use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn log(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn gh_log_die() -> !;
    fn gh_log_write(message: *const libc::c_char, _: ...);
    fn gh_log_instance() -> *const libc::c_char;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multibloom {
    pub bits: libc::c_int,
    pub bytes: libc::c_int,
    pub hashes: libc::c_int,
    pub filters: [*mut libc::c_uchar; 0],
}
#[inline]
unsafe extern "C" fn xcalloc(mut n: size_t, mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = calloc(n, size);
    if (ptr == 0 as *mut libc::c_void) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return ptr;
}
#[inline]
unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = malloc(size);
    if (ptr == 0 as *mut libc::c_void) as libc::c_int as libc::c_long != 0 {
        fprintf(stderr, b"[FATAL]: oom\n\0" as *const u8 as *const libc::c_char);
        gh_log_die();
    }
    return ptr;
}
pub unsafe extern "C" fn multibloom_check(
    mut bloom: *mut multibloom,
    mut f: libc::c_int,
    mut a: uint32_t,
    mut b: uint32_t,
) -> libc::c_int {
    let mut filter: *mut libc::c_uchar = *((*bloom).filters)
        .as_mut_ptr()
        .offset(f as isize);
    let mut hits: libc::c_int = 0 as libc::c_int;
    let mut x: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut byte: uint32_t = 0;
    let mut mask: uint32_t = 0;
    let mut c: libc::c_uchar = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < (*bloom).hashes as libc::c_uint {
        x = a
            .wrapping_add(i.wrapping_mul(b))
            .wrapping_rem((*bloom).bits as libc::c_uint);
        byte = x >> 3 as libc::c_int;
        c = *filter.offset(byte as isize);
        mask = ((1 as libc::c_int) << x.wrapping_rem(8 as libc::c_int as libc::c_uint))
            as uint32_t;
        if c as libc::c_uint & mask != 0 {
            hits += 1;
            hits;
        } else {
            *filter.offset(byte as isize) = (c as libc::c_uint | mask) as libc::c_uchar;
        }
        i = i.wrapping_add(1);
        i;
    }
    return (hits == (*bloom).hashes) as libc::c_int;
}
pub unsafe extern "C" fn multibloom_reset(
    mut bloom: *mut multibloom,
    mut f: libc::c_int,
) {
    memset(
        *((*bloom).filters).as_mut_ptr().offset(f as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (*bloom).bytes as libc::c_ulong,
    );
}
pub unsafe extern "C" fn multibloom_new(
    mut filters: libc::c_int,
    mut entries: libc::c_int,
    mut error: libc::c_double,
) -> *mut multibloom {
    let bpe: libc::c_double = -(log(error) / 0.480453013918201f64);
    let mut i: libc::c_int = 0;
    let mut bloom: *mut multibloom = xmalloc(
        (::std::mem::size_of::<multibloom>() as libc::c_ulong)
            .wrapping_add(
                (filters as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ),
            ),
    ) as *mut multibloom;
    (*bloom).bits = (entries as libc::c_double * bpe) as libc::c_int;
    (*bloom).bytes = (*bloom).bits / 8 as libc::c_int;
    if (*bloom).bits % 8 as libc::c_int != 0 {
        (*bloom).bytes += 1;
        (*bloom).bytes;
    }
    (*bloom).hashes = ceil(0.693147180559945f64 * bpe) as libc::c_int;
    i = 0 as libc::c_int;
    while i < filters {
        let ref mut fresh0 = *((*bloom).filters).as_mut_ptr().offset(i as isize);
        *fresh0 = xcalloc(1 as libc::c_int as size_t, (*bloom).bytes as size_t)
            as *mut libc::c_uchar;
        i += 1;
        i;
    }
    gh_log_write(
        b"instance=%s event=bloom_init entries=%d error=%f bits=%d bpe=%f bytes=%d hash_funcs=%d\n\0"
            as *const u8 as *const libc::c_char,
        gh_log_instance(),
        entries,
        error,
        (*bloom).bits,
        bpe,
        (*bloom).bytes,
        (*bloom).hashes,
    );
    return bloom;
}
