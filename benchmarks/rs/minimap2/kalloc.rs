use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
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
pub struct km_stat_t {
    pub capacity: size_t,
    pub available: size_t,
    pub n_blocks: size_t,
    pub n_cores: size_t,
    pub largest: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct header_t {
    pub size: size_t,
    pub ptr: *mut header_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmem_t {
    pub par: *mut libc::c_void,
    pub min_core_size: size_t,
    pub base: header_t,
    pub loop_head: *mut header_t,
    pub core_head: *mut header_t,
}
unsafe extern "C" fn panic(mut s: *const libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, s);
    abort();
}
pub unsafe extern "C" fn km_init2(
    mut km_par: *mut libc::c_void,
    mut min_core_size: size_t,
) -> *mut libc::c_void {
    let mut km: *mut kmem_t = 0 as *mut kmem_t;
    km = kcalloc(
        km_par,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kmem_t>() as libc::c_ulong,
    ) as *mut kmem_t;
    (*km).par = km_par;
    (*km)
        .min_core_size = if min_core_size > 0 as libc::c_int as libc::c_ulong {
        min_core_size
    } else {
        0x80000 as libc::c_int as libc::c_ulong
    };
    return km as *mut libc::c_void;
}
pub unsafe extern "C" fn km_init() -> *mut libc::c_void {
    return km_init2(0 as *mut libc::c_void, 0 as libc::c_int as size_t);
}
pub unsafe extern "C" fn km_destroy(mut _km: *mut libc::c_void) {
    let mut km: *mut kmem_t = _km as *mut kmem_t;
    let mut km_par: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut header_t = 0 as *mut header_t;
    let mut q: *mut header_t = 0 as *mut header_t;
    if km.is_null() {
        return;
    }
    km_par = (*km).par;
    p = (*km).core_head;
    while !p.is_null() {
        q = (*p).ptr;
        kfree(km_par, p as *mut libc::c_void);
        p = q;
    }
    kfree(km_par, km as *mut libc::c_void);
}
unsafe extern "C" fn morecore(mut km: *mut kmem_t, mut nu: size_t) -> *mut header_t {
    let mut q: *mut header_t = 0 as *mut header_t;
    let mut bytes: size_t = 0;
    let mut p: *mut size_t = 0 as *mut size_t;
    nu = nu
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            ((*km).min_core_size).wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_div((*km).min_core_size)
        .wrapping_mul((*km).min_core_size);
    bytes = nu.wrapping_mul(::std::mem::size_of::<header_t>() as libc::c_ulong);
    q = kmalloc((*km).par, bytes) as *mut header_t;
    if q.is_null() {
        panic(b"[morecore] insufficient memory\0" as *const u8 as *const libc::c_char);
    }
    (*q).ptr = (*km).core_head;
    (*q).size = nu;
    (*km).core_head = q;
    p = q.offset(1 as libc::c_int as isize) as *mut size_t;
    *p = nu.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    kfree(
        km as *mut libc::c_void,
        p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
    );
    return (*km).loop_head;
}
#[no_mangle] pub unsafe extern "C" fn kfree(mut _km: *mut libc::c_void, mut ap: *mut libc::c_void) {
    let mut p: *mut header_t = 0 as *mut header_t;
    let mut q: *mut header_t = 0 as *mut header_t;
    let mut km: *mut kmem_t = _km as *mut kmem_t;
    if ap.is_null() {
        return;
    }
    if km.is_null() {
        free(ap);
        return;
    }
    p = (ap as *mut size_t).offset(-(1 as libc::c_int as isize)) as *mut header_t;
    (*p).size = *(ap as *mut size_t).offset(-(1 as libc::c_int as isize));
    q = (*km).loop_head;
    while !(p > q && p < (*q).ptr) {
        if q >= (*q).ptr && (p > q || p < (*q).ptr) {
            break;
        }
        q = (*q).ptr;
    }
    if p.offset((*p).size as isize) == (*q).ptr {
        (*p)
            .size = ((*p).size as libc::c_ulong).wrapping_add((*(*q).ptr).size) as size_t
            as size_t;
        (*p).ptr = (*(*q).ptr).ptr;
    } else if p.offset((*p).size as isize) > (*q).ptr && (*q).ptr >= p {
        panic(
            b"[kfree] The end of the allocated block enters a free block.\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        (*p).ptr = (*q).ptr;
    }
    if q.offset((*q).size as isize) == p {
        (*q)
            .size = ((*q).size as libc::c_ulong).wrapping_add((*p).size) as size_t
            as size_t;
        (*q).ptr = (*p).ptr;
        (*km).loop_head = q;
    } else if q.offset((*q).size as isize) > p && p >= q {
        panic(
            b"[kfree] The end of a free block enters the allocated block.\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        (*km).loop_head = p;
        (*q).ptr = p;
    };
}
#[no_mangle] pub unsafe extern "C" fn kmalloc(
    mut _km: *mut libc::c_void,
    mut n_bytes: size_t,
) -> *mut libc::c_void {
    let mut km: *mut kmem_t = _km as *mut kmem_t;
    let mut n_units: size_t = 0;
    let mut p: *mut header_t = 0 as *mut header_t;
    let mut q: *mut header_t = 0 as *mut header_t;
    if n_bytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    if km.is_null() {
        return malloc(n_bytes);
    }
    n_units = n_bytes
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<header_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<header_t>() as libc::c_ulong);
    q = (*km).loop_head;
    if q.is_null() {
        (*km).base.ptr = &mut (*km).base;
        (*km).loop_head = (*km).base.ptr;
        q = (*km).loop_head;
    }
    p = (*q).ptr;
    loop {
        if (*p).size >= n_units {
            if (*p).size == n_units {
                (*q).ptr = (*p).ptr;
            } else {
                (*p)
                    .size = ((*p).size as libc::c_ulong).wrapping_sub(n_units) as size_t
                    as size_t;
                p = p.offset((*p).size as isize);
                *(p as *mut size_t) = n_units;
            }
            (*km).loop_head = q;
            return (p as *mut size_t).offset(1 as libc::c_int as isize)
                as *mut libc::c_void;
        }
        if p == (*km).loop_head {
            p = morecore(km, n_units);
            if p.is_null() {
                return 0 as *mut libc::c_void;
            }
        }
        q = p;
        p = (*p).ptr;
    };
}
#[no_mangle] pub unsafe extern "C" fn kcalloc(
    mut _km: *mut libc::c_void,
    mut count: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut km: *mut kmem_t = _km as *mut kmem_t;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 as libc::c_int as libc::c_ulong
        || count == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut libc::c_void;
    }
    if km.is_null() {
        return calloc(count, size);
    }
    p = kmalloc(km as *mut libc::c_void, count.wrapping_mul(size));
    memset(p, 0 as libc::c_int, count.wrapping_mul(size));
    return p;
}
#[no_mangle] pub unsafe extern "C" fn krealloc(
    mut _km: *mut libc::c_void,
    mut ap: *mut libc::c_void,
    mut n_bytes: size_t,
) -> *mut libc::c_void {
    let mut km: *mut kmem_t = _km as *mut kmem_t;
    let mut cap: size_t = 0;
    let mut p: *mut size_t = 0 as *mut size_t;
    let mut q: *mut size_t = 0 as *mut size_t;
    if n_bytes == 0 as libc::c_int as libc::c_ulong {
        kfree(km as *mut libc::c_void, ap);
        return 0 as *mut libc::c_void;
    }
    if km.is_null() {
        return realloc(ap, n_bytes);
    }
    if ap.is_null() {
        return kmalloc(km as *mut libc::c_void, n_bytes);
    }
    p = (ap as *mut size_t).offset(-(1 as libc::c_int as isize));
    cap = (*p)
        .wrapping_mul(::std::mem::size_of::<header_t>() as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<size_t>() as libc::c_ulong);
    if cap >= n_bytes {
        return ap;
    }
    q = kmalloc(km as *mut libc::c_void, n_bytes) as *mut size_t;
    memcpy(q as *mut libc::c_void, ap, cap);
    kfree(km as *mut libc::c_void, ap);
    return q as *mut libc::c_void;
}
pub unsafe extern "C" fn km_stat(mut _km: *const libc::c_void, mut s: *mut km_stat_t) {
    let mut km: *mut kmem_t = _km as *mut kmem_t;
    let mut p: *mut header_t = 0 as *mut header_t;
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<km_stat_t>() as libc::c_ulong,
    );
    if km.is_null() || ((*km).loop_head).is_null() {
        return;
    }
    p = (*km).loop_head;
    loop {
        (*s)
            .available = ((*s).available as libc::c_ulong)
            .wrapping_add(
                ((*p).size)
                    .wrapping_mul(::std::mem::size_of::<header_t>() as libc::c_ulong),
            ) as size_t as size_t;
        if (*p).size != 0 as libc::c_int as libc::c_ulong {
            (*s).n_blocks = ((*s).n_blocks).wrapping_add(1);
            (*s).n_blocks;
        }
        if (*p).ptr > p && p.offset((*p).size as isize) > (*p).ptr {
            panic(
                b"[km_stat] The end of a free block enters another free block.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if (*p).ptr == (*km).loop_head {
            break;
        }
        p = (*p).ptr;
    }
    p = (*km).core_head;
    while !p.is_null() {
        let mut size: size_t = ((*p).size)
            .wrapping_mul(::std::mem::size_of::<header_t>() as libc::c_ulong);
        (*s).n_cores = ((*s).n_cores).wrapping_add(1);
        (*s).n_cores;
        (*s)
            .capacity = ((*s).capacity as libc::c_ulong).wrapping_add(size) as size_t
            as size_t;
        (*s).largest = if (*s).largest > size { (*s).largest } else { size };
        p = (*p).ptr;
    }
}
