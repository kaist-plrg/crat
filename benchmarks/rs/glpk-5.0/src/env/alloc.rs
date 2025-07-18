use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENV {
    pub self_0: *mut ENV,
    pub term_buf: *mut libc::c_char,
    pub term_out: libc::c_int,
    pub term_hook: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> libc::c_int,
    >,
    pub term_info: *mut libc::c_void,
    pub tee_file: *mut FILE,
    pub err_st: libc::c_int,
    pub err_file: *const libc::c_char,
    pub err_line: libc::c_int,
    pub err_hook: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_info: *mut libc::c_void,
    pub err_buf: *mut libc::c_char,
    pub mem_limit: size_t,
    pub mem_ptr: *mut MBD,
    pub mem_count: libc::c_int,
    pub mem_cpeak: libc::c_int,
    pub mem_total: size_t,
    pub mem_tpeak: size_t,
    pub gmp_pool: *mut libc::c_void,
    pub gmp_size: libc::c_int,
    pub gmp_work: *mut libc::c_ushort,
    pub h_odbc: *mut libc::c_void,
    pub h_mysql: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
unsafe extern "C" fn dma(
    mut func: *const libc::c_char,
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut mbd: *mut MBD = 0 as *mut MBD;
    if ptr.is_null() {
        mbd = 0 as *mut MBD;
    } else {
        mbd = (ptr as *mut libc::c_char)
            .offset(
                -((::std::mem::size_of::<MBD>() as libc::c_ulong)
                    .wrapping_add(
                        (16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                    )
                    .wrapping_div(16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(16 as libc::c_int as libc::c_ulong) as isize),
            ) as *mut MBD;
        if (*mbd).self_0 != mbd {
            (glp_error_(
                b"env/alloc.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int,
            ))
                .unwrap()(
                b"%s: ptr = %p; invalid pointer\n\0" as *const u8 as *const libc::c_char,
                func,
                ptr,
            );
        }
        (*mbd).self_0 = 0 as *mut MBD;
        if ((*mbd).prev).is_null() {
            (*env).mem_ptr = (*mbd).next;
        } else {
            (*(*mbd).prev).next = (*mbd).next;
        }
        if !((*mbd).next).is_null() {
            (*(*mbd).next).prev = (*mbd).prev;
        }
        if !((*env).mem_count >= 1 as libc::c_int && (*env).mem_total >= (*mbd).size) {
            (glp_error_(
                b"env/alloc.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int,
            ))
                .unwrap()(
                b"%s: memory allocation error\n\0" as *const u8 as *const libc::c_char,
                func,
            );
        }
        (*env).mem_count -= 1;
        (*env).mem_count;
        (*env)
            .mem_total = ((*env).mem_total as libc::c_ulong).wrapping_sub((*mbd).size)
            as size_t as size_t;
        if size == 0 as libc::c_int as libc::c_ulong {
            free(mbd as *mut libc::c_void);
            return 0 as *mut libc::c_void;
        }
    }
    if size
        > (!(0 as libc::c_int as size_t))
            .wrapping_sub(
                (::std::mem::size_of::<MBD>() as libc::c_ulong)
                    .wrapping_add(
                        (16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                    )
                    .wrapping_div(16 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(16 as libc::c_int as libc::c_ulong),
            )
    {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
        ))
            .unwrap()(
            b"%s: block too large\n\0" as *const u8 as *const libc::c_char,
            func,
        );
    }
    size = (size as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<MBD>() as libc::c_ulong)
                .wrapping_add((16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                .wrapping_div(16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(16 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    if size > ((*env).mem_limit).wrapping_sub((*env).mem_total) {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
        ))
            .unwrap()(
            b"%s: memory allocation limit exceeded\n\0" as *const u8
                as *const libc::c_char,
            func,
        );
    }
    if (*env).mem_count == 2147483647 as libc::c_int {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
        ))
            .unwrap()(
            b"%s: too many memory blocks allocated\n\0" as *const u8
                as *const libc::c_char,
            func,
        );
    }
    mbd = (if mbd.is_null() {
        malloc(size)
    } else {
        realloc(mbd as *mut libc::c_void, size)
    }) as *mut MBD;
    if mbd.is_null() {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int,
        ))
            .unwrap()(
            b"%s: no memory available\n\0" as *const u8 as *const libc::c_char,
            func,
        );
    }
    (*mbd).size = size;
    (*mbd).self_0 = mbd;
    (*mbd).prev = 0 as *mut MBD;
    (*mbd).next = (*env).mem_ptr;
    if !((*mbd).next).is_null() {
        (*(*mbd).next).prev = mbd;
    }
    (*env).mem_ptr = mbd;
    (*env).mem_count += 1;
    (*env).mem_count;
    if (*env).mem_cpeak < (*env).mem_count {
        (*env).mem_cpeak = (*env).mem_count;
    }
    (*env)
        .mem_total = ((*env).mem_total as libc::c_ulong).wrapping_add(size) as size_t
        as size_t;
    if (*env).mem_tpeak < (*env).mem_total {
        (*env).mem_tpeak = (*env).mem_total;
    }
    return (mbd as *mut libc::c_char)
        .offset(
            (::std::mem::size_of::<MBD>() as libc::c_ulong)
                .wrapping_add((16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                .wrapping_div(16 as libc::c_int as libc::c_ulong)
                .wrapping_mul(16 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut libc::c_void;
}
pub unsafe extern "C" fn glp_alloc(
    mut n: libc::c_int,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    if n < 1 as libc::c_int {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int,
        ))
            .unwrap()(
            b"glp_alloc: n = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            n,
        );
    }
    if size < 1 as libc::c_int {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
        ))
            .unwrap()(
            b"glp_alloc: size = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            size,
        );
    }
    if n as size_t > (!(0 as libc::c_int as size_t)).wrapping_div(size as size_t) {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
        ))
            .unwrap()(
            b"glp_alloc: n = %d, size = %d; block too large\n\0" as *const u8
                as *const libc::c_char,
            n,
            size,
        );
    }
    return dma(
        b"glp_alloc\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
        (n as size_t).wrapping_mul(size as size_t),
    );
}
pub unsafe extern "C" fn glp_realloc(
    mut ptr: *mut libc::c_void,
    mut n: libc::c_int,
    mut size: libc::c_int,
) -> *mut libc::c_void {
    if ptr.is_null() {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
        ))
            .unwrap()(
            b"glp_realloc: ptr = %p; invalid pointer\n\0" as *const u8
                as *const libc::c_char,
            ptr,
        );
    }
    if n < 1 as libc::c_int {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
        ))
            .unwrap()(
            b"glp_realloc: n = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            n,
        );
    }
    if size < 1 as libc::c_int {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
        ))
            .unwrap()(
            b"glp_realloc: size = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            size,
        );
    }
    if n as size_t > (!(0 as libc::c_int as size_t)).wrapping_div(size as size_t) {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
        ))
            .unwrap()(
            b"glp_realloc: n = %d, size = %d; block too large\n\0" as *const u8
                as *const libc::c_char,
            n,
            size,
        );
    }
    return dma(
        b"glp_realloc\0" as *const u8 as *const libc::c_char,
        ptr,
        (n as size_t).wrapping_mul(size as size_t),
    );
}
pub unsafe extern "C" fn glp_free(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
        ))
            .unwrap()(
            b"glp_free: ptr = %p; invalid pointer\n\0" as *const u8
                as *const libc::c_char,
            ptr,
        );
    }
    dma(
        b"glp_free\0" as *const u8 as *const libc::c_char,
        ptr,
        0 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn glp_mem_limit(mut limit: libc::c_int) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if limit < 1 as libc::c_int {
        (glp_error_(
            b"env/alloc.c\0" as *const u8 as *const libc::c_char,
            197 as libc::c_int,
        ))
            .unwrap()(
            b"glp_mem_limit: limit = %d; invalid parameter\n\0" as *const u8
                as *const libc::c_char,
            limit,
        );
    }
    if limit as size_t <= !(0 as libc::c_int as size_t) >> 20 as libc::c_int {
        (*env).mem_limit = (limit as size_t) << 20 as libc::c_int;
    } else {
        (*env).mem_limit = !(0 as libc::c_int as size_t);
    };
}
pub unsafe extern "C" fn glp_mem_usage(
    mut count: *mut libc::c_int,
    mut cpeak: *mut libc::c_int,
    mut total: *mut size_t,
    mut tpeak: *mut size_t,
) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !count.is_null() {
        *count = (*env).mem_count;
    }
    if !cpeak.is_null() {
        *cpeak = (*env).mem_cpeak;
    }
    if !total.is_null() {
        *total = (*env).mem_total;
    }
    if !tpeak.is_null() {
        *tpeak = (*env).mem_tpeak;
    }
}
