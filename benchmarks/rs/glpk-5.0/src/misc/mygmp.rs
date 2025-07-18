use ::libc;
extern "C" {
    pub type DMP;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn _glp_bigmul(
        n: libc::c_int,
        m: libc::c_int,
        x: *mut libc::c_ushort,
        y: *mut libc::c_ushort,
    );
    fn _glp_bigdiv(
        n: libc::c_int,
        m: libc::c_int,
        x: *mut libc::c_ushort,
        y: *mut libc::c_ushort,
    );
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: libc::c_int);
    static mut stdout: *mut FILE;
    fn _glp_dmp_in_use(pool: *mut DMP) -> size_t;
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
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz {
    pub val: libc::c_int,
    pub ptr: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz_seg {
    pub d: [libc::c_ushort; 6],
    pub next: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpq {
    pub p: mpz,
    pub q: mpz,
}
pub type mpz_t = *mut mpz;
pub type mpq_t = *mut mpq;
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
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}
pub type FILE = _IO_FILE;
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
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
pub unsafe extern "C" fn _glp_gmp_get_atom(mut size: libc::c_int) -> *mut libc::c_void {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if ((*env).gmp_pool).is_null() {
        (*env).gmp_pool = _glp_dmp_create_pool() as *mut libc::c_void;
    }
    return _glp_dmp_get_atom((*env).gmp_pool as *mut DMP, size);
}
pub unsafe extern "C" fn _glp_gmp_free_atom(
    mut ptr: *mut libc::c_void,
    mut size: libc::c_int,
) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    (!((*env).gmp_pool).is_null()
        || {
            glp_assert_(
                b"gmp_pool != NULL\0" as *const u8 as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_dmp_free_atom((*env).gmp_pool as *mut DMP, ptr, size);
}
pub unsafe extern "C" fn _glp_gmp_pool_count() -> libc::c_int {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if ((*env).gmp_pool).is_null() {
        return 0 as libc::c_int
    } else {
        return _glp_dmp_in_use((*env).gmp_pool as *mut DMP) as libc::c_int
    };
}
pub unsafe extern "C" fn _glp_gmp_get_work(
    mut size: libc::c_int,
) -> *mut libc::c_ushort {
    let mut env: *mut ENV = _glp_get_env_ptr();
    (size > 0 as libc::c_int
        || {
            glp_assert_(
                b"size > 0\0" as *const u8 as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*env).gmp_size < size {
        if (*env).gmp_size == 0 as libc::c_int {
            (((*env).gmp_work).is_null()
                || {
                    glp_assert_(
                        b"gmp_work == NULL\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        65 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*env).gmp_size = 100 as libc::c_int;
        } else {
            (!((*env).gmp_work).is_null()
                || {
                    glp_assert_(
                        b"gmp_work != NULL\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        69 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            glp_free((*env).gmp_work as *mut libc::c_void);
        }
        while (*env).gmp_size < size {
            (*env).gmp_size += (*env).gmp_size;
        }
        (*env)
            .gmp_work = glp_alloc(
            (*env).gmp_size,
            ::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_ushort;
    }
    return (*env).gmp_work;
}
pub unsafe extern "C" fn _glp_gmp_free_mem() {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !((*env).gmp_pool).is_null() {
        _glp_dmp_delete_pool((*env).gmp_pool as *mut DMP);
    }
    if !((*env).gmp_work).is_null() {
        glp_free((*env).gmp_work as *mut libc::c_void);
    }
    (*env).gmp_pool = 0 as *mut libc::c_void;
    (*env).gmp_size = 0 as libc::c_int;
    (*env).gmp_work = 0 as *mut libc::c_ushort;
}
pub unsafe extern "C" fn _glp_mpz_init() -> mpz_t {
    let mut x: mpz_t = 0 as *mut mpz;
    x = _glp_gmp_get_atom(::std::mem::size_of::<mpz>() as libc::c_ulong as libc::c_int)
        as mpz_t;
    (*x).val = 0 as libc::c_int;
    (*x).ptr = 0 as *mut mpz_seg;
    return x;
}
pub unsafe extern "C" fn _glp_mpz_clear(mut x: mpz_t) {
    _glp_mpz_set_si(x, 0 as libc::c_int);
    (((*x).ptr).is_null()
        || {
            glp_assert_(
                b"x->ptr == NULL\0" as *const u8 as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_gmp_free_atom(
        x as *mut libc::c_void,
        ::std::mem::size_of::<mpz>() as libc::c_ulong as libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_mpz_set(mut z: mpz_t, mut x: mpz_t) {
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ee: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    if z != x {
        _glp_mpz_set_si(z, 0 as libc::c_int);
        (*z).val = (*x).val;
        (((*z).ptr).is_null()
            || {
                glp_assert_(
                    b"z->ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    117 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        e = (*x).ptr;
        es = 0 as *mut mpz_seg;
        while !e.is_null() {
            ee = _glp_gmp_get_atom(
                ::std::mem::size_of::<mpz_seg>() as libc::c_ulong as libc::c_int,
            ) as *mut mpz_seg;
            memcpy(
                ((*ee).d).as_mut_ptr() as *mut libc::c_void,
                ((*e).d).as_mut_ptr() as *const libc::c_void,
                12 as libc::c_int as libc::c_ulong,
            );
            (*ee).next = 0 as *mut mpz_seg;
            if ((*z).ptr).is_null() {
                (*z).ptr = ee;
            } else {
                (*es).next = ee;
            }
            es = ee;
            e = (*e).next;
        }
    }
}
pub unsafe extern "C" fn _glp_mpz_set_si(mut x: mpz_t, mut val: libc::c_int) {
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    while !((*x).ptr).is_null() {
        e = (*x).ptr;
        (*x).ptr = (*e).next;
        _glp_gmp_free_atom(
            e as *mut libc::c_void,
            ::std::mem::size_of::<mpz_seg>() as libc::c_ulong as libc::c_int,
        );
    }
    if val as libc::c_uint == 0x80000000 as libc::c_uint {
        (*x).val = -(1 as libc::c_int);
        e = _glp_gmp_get_atom(
            ::std::mem::size_of::<mpz_seg>() as libc::c_ulong as libc::c_int,
        ) as *mut mpz_seg;
        (*x).ptr = e;
        memset(
            ((*e).d).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            12 as libc::c_int as libc::c_ulong,
        );
        (*e).d[1 as libc::c_int as usize] = 0x8000 as libc::c_int as libc::c_ushort;
        (*e).next = 0 as *mut mpz_seg;
    } else {
        (*x).val = val;
    };
}
pub unsafe extern "C" fn _glp_mpz_get_d(mut x: mpz_t) -> libc::c_double {
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut j: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    let mut deg: libc::c_double = 0.;
    if ((*x).ptr).is_null() {
        val = (*x).val as libc::c_double;
    } else {
        ((*x).val != 0 as libc::c_int
            || {
                glp_assert_(
                    b"x->val != 0\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    165 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        val = 0.0f64;
        deg = 1.0f64;
        e = (*x).ptr;
        while !e.is_null() {
            j = 0 as libc::c_int;
            while j <= 5 as libc::c_int {
                val += deg * (*e).d[j as usize] as libc::c_int as libc::c_double;
                deg *= 65536.0f64;
                j += 1;
                j;
            }
            e = (*e).next;
        }
        if (*x).val < 0 as libc::c_int {
            val = -val;
        }
    }
    return val;
}
pub unsafe extern "C" fn _glp_mpz_get_d_2exp(
    mut exp: *mut libc::c_int,
    mut x: mpz_t,
) -> libc::c_double {
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    if ((*x).ptr).is_null() {
        val = (*x).val as libc::c_double;
        n = 0 as libc::c_int;
    } else {
        ((*x).val != 0 as libc::c_int
            || {
                glp_assert_(
                    b"x->val != 0\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    193 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        val = 0.0f64;
        n = 0 as libc::c_int;
        e = (*x).ptr;
        while !e.is_null() {
            j = 0 as libc::c_int;
            while j <= 5 as libc::c_int {
                val += (*e).d[j as usize] as libc::c_int as libc::c_double;
                val /= 65536.0f64;
                n += 16 as libc::c_int;
                j += 1;
                j;
            }
            e = (*e).next;
        }
        if (*x).val < 0 as libc::c_int {
            val = -val;
        }
    }
    val = frexp(val, &mut n1);
    *exp = n + n1;
    return val;
}
pub unsafe extern "C" fn _glp_mpz_swap(mut x: mpz_t, mut y: mpz_t) {
    let mut val: libc::c_int = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    val = (*x).val;
    ptr = (*x).ptr as *mut libc::c_void;
    (*x).val = (*y).val;
    (*x).ptr = (*y).ptr;
    (*y).val = val;
    (*y).ptr = ptr as *mut mpz_seg;
}
unsafe extern "C" fn normalize(mut x: mpz_t) {
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    if ((*x).ptr).is_null() {
        ((*x).val as libc::c_uint != 0x80000000 as libc::c_uint
            || {
                glp_assert_(
                    b"x->val != 0x80000000\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    225 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    } else {
        ((*x).val == 1 as libc::c_int || (*x).val == -(1 as libc::c_int)
            || {
                glp_assert_(
                    b"x->val == +1 || x->val == -1\0" as *const u8
                        as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    228 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        es = 0 as *mut mpz_seg;
        e = (*x).ptr;
        while !e.is_null() {
            if (*e).d[0 as libc::c_int as usize] as libc::c_int != 0
                || (*e).d[1 as libc::c_int as usize] as libc::c_int != 0
                || (*e).d[2 as libc::c_int as usize] as libc::c_int != 0
                || (*e).d[3 as libc::c_int as usize] as libc::c_int != 0
                || (*e).d[4 as libc::c_int as usize] as libc::c_int != 0
                || (*e).d[5 as libc::c_int as usize] as libc::c_int != 0
            {
                es = e;
            }
            e = (*e).next;
        }
        if es.is_null() {
            _glp_mpz_set_si(x, 0 as libc::c_int);
        } else {
            while !((*es).next).is_null() {
                e = (*es).next;
                (*es).next = (*e).next;
                _glp_gmp_free_atom(
                    e as *mut libc::c_void,
                    ::std::mem::size_of::<mpz_seg>() as libc::c_ulong as libc::c_int,
                );
            }
            e = (*x).ptr;
            if ((*e).next).is_null()
                && (*e).d[1 as libc::c_int as usize] as libc::c_int
                    <= 0x7fff as libc::c_int && (*e).d[2 as libc::c_int as usize] == 0
                && (*e).d[3 as libc::c_int as usize] == 0
                && (*e).d[4 as libc::c_int as usize] == 0
                && (*e).d[5 as libc::c_int as usize] == 0
            {
                let mut val: libc::c_int = 0;
                val = (*e).d[0 as libc::c_int as usize] as libc::c_int
                    + (((*e).d[1 as libc::c_int as usize] as libc::c_int)
                        << 16 as libc::c_int);
                if (*x).val < 0 as libc::c_int {
                    val = -val;
                }
                _glp_mpz_set_si(x, val);
            }
        }
    };
}
pub unsafe extern "C" fn _glp_mpz_add(mut z: mpz_t, mut x: mpz_t, mut y: mpz_t) {
    let mut current_block: u64;
    static mut zero: mpz_seg = {
        let mut init = mpz_seg {
            d: [
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
            ],
            next: 0 as *const mpz_seg as *mut mpz_seg,
        };
        init
    };
    let mut dumx: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut dumy: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut ex: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ey: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ez: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ee: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut k: libc::c_int = 0;
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    if (*x).val == 0 as libc::c_int {
        (((*x).ptr).is_null()
            || {
                glp_assert_(
                    b"x->ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    268 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_mpz_set(z, y);
    } else if (*y).val == 0 as libc::c_int {
        (((*y).ptr).is_null()
            || {
                glp_assert_(
                    b"y->ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    274 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_mpz_set(z, x);
    } else {
        if ((*x).ptr).is_null() && ((*y).ptr).is_null() {
            let mut xval: libc::c_int = (*x).val;
            let mut yval: libc::c_int = (*y).val;
            let mut zval: libc::c_int = (*x).val + (*y).val;
            (xval as libc::c_uint != 0x80000000 as libc::c_uint
                && yval as libc::c_uint != 0x80000000 as libc::c_uint
                || {
                    glp_assert_(
                        b"xval != 0x80000000 && yval != 0x80000000\0" as *const u8
                            as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        281 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if !(xval > 0 as libc::c_int && yval > 0 as libc::c_int
                && zval <= 0 as libc::c_int
                || xval < 0 as libc::c_int && yval < 0 as libc::c_int
                    && zval >= 0 as libc::c_int)
            {
                _glp_mpz_set_si(z, zval);
                current_block = 17898794103029372929;
            } else {
                current_block = 13536709405535804910;
            }
        } else {
            current_block = 13536709405535804910;
        }
        match current_block {
            17898794103029372929 => {}
            _ => {
                if ((*x).ptr).is_null() {
                    ((*x).val as libc::c_uint != 0x80000000 as libc::c_uint
                        || {
                            glp_assert_(
                                b"x->val != 0x80000000\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                                290 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if (*x).val >= 0 as libc::c_int {
                        sx = 1 as libc::c_int;
                        t = (*x).val as libc::c_uint;
                    } else {
                        sx = -(1 as libc::c_int);
                        t = -(*x).val as libc::c_uint;
                    }
                    ex = &mut dumx;
                    (*ex).d[0 as libc::c_int as usize] = t as libc::c_ushort;
                    (*ex)
                        .d[1 as libc::c_int
                        as usize] = (t >> 16 as libc::c_int) as libc::c_ushort;
                    (*ex)
                        .d[5 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_ushort;
                    (*ex)
                        .d[4 as libc::c_int
                        as usize] = (*ex).d[5 as libc::c_int as usize];
                    (*ex)
                        .d[3 as libc::c_int
                        as usize] = (*ex).d[4 as libc::c_int as usize];
                    (*ex)
                        .d[2 as libc::c_int
                        as usize] = (*ex).d[3 as libc::c_int as usize];
                    (*ex).next = 0 as *mut mpz_seg;
                } else {
                    sx = (*x).val;
                    (sx == 1 as libc::c_int || sx == -(1 as libc::c_int)
                        || {
                            glp_assert_(
                                b"sx == +1 || sx == -1\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                                307 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ex = (*x).ptr;
                }
                if ((*y).ptr).is_null() {
                    ((*y).val as libc::c_uint != 0x80000000 as libc::c_uint
                        || {
                            glp_assert_(
                                b"y->val != 0x80000000\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                                312 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if (*y).val >= 0 as libc::c_int {
                        sy = 1 as libc::c_int;
                        t = (*y).val as libc::c_uint;
                    } else {
                        sy = -(1 as libc::c_int);
                        t = -(*y).val as libc::c_uint;
                    }
                    ey = &mut dumy;
                    (*ey).d[0 as libc::c_int as usize] = t as libc::c_ushort;
                    (*ey)
                        .d[1 as libc::c_int
                        as usize] = (t >> 16 as libc::c_int) as libc::c_ushort;
                    (*ey)
                        .d[5 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_ushort;
                    (*ey)
                        .d[4 as libc::c_int
                        as usize] = (*ey).d[5 as libc::c_int as usize];
                    (*ey)
                        .d[3 as libc::c_int
                        as usize] = (*ey).d[4 as libc::c_int as usize];
                    (*ey)
                        .d[2 as libc::c_int
                        as usize] = (*ey).d[3 as libc::c_int as usize];
                    (*ey).next = 0 as *mut mpz_seg;
                } else {
                    sy = (*y).val;
                    (sy == 1 as libc::c_int || sy == -(1 as libc::c_int)
                        || {
                            glp_assert_(
                                b"sy == +1 || sy == -1\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                                329 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ey = (*y).ptr;
                }
                sz = sx;
                es = 0 as *mut mpz_seg;
                ez = es;
                if sx > 0 as libc::c_int && sy > 0 as libc::c_int
                    || sx < 0 as libc::c_int && sy < 0 as libc::c_int
                {
                    t = 0 as libc::c_int as libc::c_uint;
                    while !ex.is_null() || !ey.is_null() {
                        if ex.is_null() {
                            ex = &mut zero;
                        }
                        if ey.is_null() {
                            ey = &mut zero;
                        }
                        ee = _glp_gmp_get_atom(
                            ::std::mem::size_of::<mpz_seg>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut mpz_seg;
                        k = 0 as libc::c_int;
                        while k <= 5 as libc::c_int {
                            t = t.wrapping_add((*ex).d[k as usize] as libc::c_uint);
                            t = t.wrapping_add((*ey).d[k as usize] as libc::c_uint);
                            (*ee).d[k as usize] = t as libc::c_ushort;
                            t >>= 16 as libc::c_int;
                            k += 1;
                            k;
                        }
                        (*ee).next = 0 as *mut mpz_seg;
                        if ez.is_null() {
                            ez = ee;
                        } else {
                            (*es).next = ee;
                        }
                        es = ee;
                        ex = (*ex).next;
                        ey = (*ey).next;
                    }
                    if t != 0 {
                        ee = _glp_gmp_get_atom(
                            ::std::mem::size_of::<mpz_seg>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut mpz_seg;
                        (*ee)
                            .d[0 as libc::c_int
                            as usize] = 1 as libc::c_int as libc::c_ushort;
                        (*ee)
                            .d[5 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_ushort;
                        (*ee)
                            .d[4 as libc::c_int
                            as usize] = (*ee).d[5 as libc::c_int as usize];
                        (*ee)
                            .d[3 as libc::c_int
                            as usize] = (*ee).d[4 as libc::c_int as usize];
                        (*ee)
                            .d[2 as libc::c_int
                            as usize] = (*ee).d[3 as libc::c_int as usize];
                        (*ee)
                            .d[1 as libc::c_int
                            as usize] = (*ee).d[2 as libc::c_int as usize];
                        (*ee).next = 0 as *mut mpz_seg;
                        (!es.is_null()
                            || {
                                glp_assert_(
                                    b"es != NULL\0" as *const u8 as *const libc::c_char,
                                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                                    363 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        (*es).next = ee;
                    }
                } else {
                    t = 1 as libc::c_int as libc::c_uint;
                    while !ex.is_null() || !ey.is_null() {
                        if ex.is_null() {
                            ex = &mut zero;
                        }
                        if ey.is_null() {
                            ey = &mut zero;
                        }
                        ee = _glp_gmp_get_atom(
                            ::std::mem::size_of::<mpz_seg>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut mpz_seg;
                        k = 0 as libc::c_int;
                        while k <= 5 as libc::c_int {
                            t = t.wrapping_add((*ex).d[k as usize] as libc::c_uint);
                            t = t
                                .wrapping_add(
                                    (0xffff as libc::c_int as libc::c_uint)
                                        .wrapping_sub((*ey).d[k as usize] as libc::c_uint),
                                );
                            (*ee).d[k as usize] = t as libc::c_ushort;
                            t >>= 16 as libc::c_int;
                            k += 1;
                            k;
                        }
                        (*ee).next = 0 as *mut mpz_seg;
                        if ez.is_null() {
                            ez = ee;
                        } else {
                            (*es).next = ee;
                        }
                        es = ee;
                        ex = (*ex).next;
                        ey = (*ey).next;
                    }
                    if t == 0 {
                        sz = -sz;
                        t = 1 as libc::c_int as libc::c_uint;
                        ee = ez;
                        while !ee.is_null() {
                            k = 0 as libc::c_int;
                            while k <= 5 as libc::c_int {
                                t = t
                                    .wrapping_add(
                                        (0xffff as libc::c_int as libc::c_uint)
                                            .wrapping_sub((*ee).d[k as usize] as libc::c_uint),
                                    );
                                (*ee).d[k as usize] = t as libc::c_ushort;
                                t >>= 16 as libc::c_int;
                                k += 1;
                                k;
                            }
                            ee = (*ee).next;
                        }
                    }
                }
                _glp_mpz_set_si(z, 0 as libc::c_int);
                (*z).val = sz;
                (*z).ptr = ez;
                normalize(z);
            }
        }
    };
}
pub unsafe extern "C" fn _glp_mpz_sub(mut z: mpz_t, mut x: mpz_t, mut y: mpz_t) {
    if x == y {
        _glp_mpz_set_si(z, 0 as libc::c_int);
    } else {
        (*y).val = -(*y).val;
        _glp_mpz_add(z, x, y);
        if y != z {
            (*y).val = -(*y).val;
        }
    };
}
pub unsafe extern "C" fn _glp_mpz_mul(mut z: mpz_t, mut x: mpz_t, mut y: mpz_t) {
    let mut current_block: u64;
    let mut dumx: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut dumy: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut ex: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ey: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut work: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut wx: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut wy: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    if (*x).val == 0 as libc::c_int {
        (((*x).ptr).is_null()
            || {
                glp_assert_(
                    b"x->ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    431 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_mpz_set_si(z, 0 as libc::c_int);
    } else if (*y).val == 0 as libc::c_int {
        (((*y).ptr).is_null()
            || {
                glp_assert_(
                    b"y->ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    437 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_mpz_set_si(z, 0 as libc::c_int);
    } else {
        if ((*x).ptr).is_null() && ((*y).ptr).is_null() {
            let mut xval: libc::c_int = (*x).val;
            let mut yval: libc::c_int = (*y).val;
            let mut sz: libc::c_int = 1 as libc::c_int;
            (xval as libc::c_uint != 0x80000000 as libc::c_uint
                && yval as libc::c_uint != 0x80000000 as libc::c_uint
                || {
                    glp_assert_(
                        b"xval != 0x80000000 && yval != 0x80000000\0" as *const u8
                            as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        444 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if xval < 0 as libc::c_int {
                xval = -xval;
                sz = -sz;
            }
            if yval < 0 as libc::c_int {
                yval = -yval;
                sz = -sz;
            }
            if xval <= 0x7fffffff as libc::c_int / yval {
                _glp_mpz_set_si(z, sz * (xval * yval));
                current_block = 4270002457095487676;
            } else {
                current_block = 12599329904712511516;
            }
        } else {
            current_block = 12599329904712511516;
        }
        match current_block {
            4270002457095487676 => {}
            _ => {
                if ((*x).ptr).is_null() {
                    ((*x).val as libc::c_uint != 0x80000000 as libc::c_uint
                        || {
                            glp_assert_(
                                b"x->val != 0x80000000\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                                456 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if (*x).val >= 0 as libc::c_int {
                        sx = 1 as libc::c_int;
                        t = (*x).val as libc::c_uint;
                    } else {
                        sx = -(1 as libc::c_int);
                        t = -(*x).val as libc::c_uint;
                    }
                    ex = &mut dumx;
                    (*ex).d[0 as libc::c_int as usize] = t as libc::c_ushort;
                    (*ex)
                        .d[1 as libc::c_int
                        as usize] = (t >> 16 as libc::c_int) as libc::c_ushort;
                    (*ex)
                        .d[5 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_ushort;
                    (*ex)
                        .d[4 as libc::c_int
                        as usize] = (*ex).d[5 as libc::c_int as usize];
                    (*ex)
                        .d[3 as libc::c_int
                        as usize] = (*ex).d[4 as libc::c_int as usize];
                    (*ex)
                        .d[2 as libc::c_int
                        as usize] = (*ex).d[3 as libc::c_int as usize];
                    (*ex).next = 0 as *mut mpz_seg;
                } else {
                    sx = (*x).val;
                    (sx == 1 as libc::c_int || sx == -(1 as libc::c_int)
                        || {
                            glp_assert_(
                                b"sx == +1 || sx == -1\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                                473 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ex = (*x).ptr;
                }
                if ((*y).ptr).is_null() {
                    ((*y).val as libc::c_uint != 0x80000000 as libc::c_uint
                        || {
                            glp_assert_(
                                b"y->val != 0x80000000\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                                478 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if (*y).val >= 0 as libc::c_int {
                        sy = 1 as libc::c_int;
                        t = (*y).val as libc::c_uint;
                    } else {
                        sy = -(1 as libc::c_int);
                        t = -(*y).val as libc::c_uint;
                    }
                    ey = &mut dumy;
                    (*ey).d[0 as libc::c_int as usize] = t as libc::c_ushort;
                    (*ey)
                        .d[1 as libc::c_int
                        as usize] = (t >> 16 as libc::c_int) as libc::c_ushort;
                    (*ey)
                        .d[5 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_ushort;
                    (*ey)
                        .d[4 as libc::c_int
                        as usize] = (*ey).d[5 as libc::c_int as usize];
                    (*ey)
                        .d[3 as libc::c_int
                        as usize] = (*ey).d[4 as libc::c_int as usize];
                    (*ey)
                        .d[2 as libc::c_int
                        as usize] = (*ey).d[3 as libc::c_int as usize];
                    (*ey).next = 0 as *mut mpz_seg;
                } else {
                    sy = (*y).val;
                    (sy == 1 as libc::c_int || sy == -(1 as libc::c_int)
                        || {
                            glp_assert_(
                                b"sy == +1 || sy == -1\0" as *const u8
                                    as *const libc::c_char,
                                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                                495 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ey = (*y).ptr;
                }
                n = 0 as libc::c_int;
                nx = n;
                e = ex;
                while !e.is_null() {
                    k = 0 as libc::c_int;
                    while k <= 5 as libc::c_int {
                        n += 1;
                        n;
                        if (*e).d[k as usize] != 0 {
                            nx = n;
                        }
                        k += 1;
                        k;
                    }
                    e = (*e).next;
                }
                (nx > 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"nx > 0\0" as *const u8 as *const libc::c_char,
                            b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                            507 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                n = 0 as libc::c_int;
                ny = n;
                e = ey;
                while !e.is_null() {
                    k = 0 as libc::c_int;
                    while k <= 5 as libc::c_int {
                        n += 1;
                        n;
                        if (*e).d[k as usize] != 0 {
                            ny = n;
                        }
                        k += 1;
                        k;
                    }
                    e = (*e).next;
                }
                (ny > 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"ny > 0\0" as *const u8 as *const libc::c_char,
                            b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                            517 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                work = _glp_gmp_get_work(nx + ny + ny);
                wx = &mut *work.offset(0 as libc::c_int as isize) as *mut libc::c_ushort;
                n = 0 as libc::c_int;
                while n < nx {
                    *wx.offset((ny + n) as isize) = 0 as libc::c_int as libc::c_ushort;
                    n += 1;
                    n;
                }
                n = 0 as libc::c_int;
                e = ex;
                while !e.is_null() {
                    k = 0 as libc::c_int;
                    while k <= 5 as libc::c_int {
                        if (*e).d[k as usize] != 0 {
                            *wx.offset((ny + n) as isize) = (*e).d[k as usize];
                        }
                        k += 1;
                        k;
                        n += 1;
                        n;
                    }
                    e = (*e).next;
                }
                wy = &mut *work.offset((nx + ny) as isize) as *mut libc::c_ushort;
                n = 0 as libc::c_int;
                while n < ny {
                    *wy.offset(n as isize) = 0 as libc::c_int as libc::c_ushort;
                    n += 1;
                    n;
                }
                n = 0 as libc::c_int;
                e = ey;
                while !e.is_null() {
                    k = 0 as libc::c_int;
                    while k <= 5 as libc::c_int {
                        if (*e).d[k as usize] != 0 {
                            *wy.offset(n as isize) = (*e).d[k as usize];
                        }
                        k += 1;
                        k;
                        n += 1;
                        n;
                    }
                    e = (*e).next;
                }
                _glp_bigmul(nx, ny, wx, wy);
                _glp_mpz_set_si(z, 0 as libc::c_int);
                (*z).val = sx * sy;
                es = 0 as *mut mpz_seg;
                k = 6 as libc::c_int;
                n = 0 as libc::c_int;
                while n < nx + ny {
                    if k > 5 as libc::c_int {
                        e = _glp_gmp_get_atom(
                            ::std::mem::size_of::<mpz_seg>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut mpz_seg;
                        (*e)
                            .d[2 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_ushort;
                        (*e)
                            .d[1 as libc::c_int
                            as usize] = (*e).d[2 as libc::c_int as usize];
                        (*e)
                            .d[0 as libc::c_int
                            as usize] = (*e).d[1 as libc::c_int as usize];
                        (*e)
                            .d[5 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_ushort;
                        (*e)
                            .d[4 as libc::c_int
                            as usize] = (*e).d[5 as libc::c_int as usize];
                        (*e)
                            .d[3 as libc::c_int
                            as usize] = (*e).d[4 as libc::c_int as usize];
                        (*e).next = 0 as *mut mpz_seg;
                        if ((*z).ptr).is_null() {
                            (*z).ptr = e;
                        } else {
                            (*es).next = e;
                        }
                        es = e;
                        k = 0 as libc::c_int;
                    }
                    let fresh0 = k;
                    k = k + 1;
                    (*es).d[fresh0 as usize] = *wx.offset(n as isize);
                    n += 1;
                    n;
                }
                normalize(z);
            }
        }
    };
}
pub unsafe extern "C" fn _glp_mpz_neg(mut z: mpz_t, mut x: mpz_t) {
    _glp_mpz_set(z, x);
    (*z).val = -(*z).val;
}
pub unsafe extern "C" fn _glp_mpz_abs(mut z: mpz_t, mut x: mpz_t) {
    _glp_mpz_set(z, x);
    if (*z).val < 0 as libc::c_int {
        (*z).val = -(*z).val;
    }
}
pub unsafe extern "C" fn _glp_mpz_div(
    mut q: mpz_t,
    mut r: mpz_t,
    mut x: mpz_t,
    mut y: mpz_t,
) {
    let mut dumx: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut dumy: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut ex: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ey: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut es: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut e: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut ny: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut work: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut wx: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut wy: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    if (*y).val == 0 as libc::c_int {
        (((*y).ptr).is_null()
            || {
                glp_assert_(
                    b"y->ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    596 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (glp_error_(
            b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
            597 as libc::c_int,
        ))
            .unwrap()(
            b"mpz_div: divide by zero not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*x).val == 0 as libc::c_int {
        (((*x).ptr).is_null()
            || {
                glp_assert_(
                    b"x->ptr == NULL\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    601 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !q.is_null() {
            _glp_mpz_set_si(q, 0 as libc::c_int);
        }
        if !r.is_null() {
            _glp_mpz_set_si(r, 0 as libc::c_int);
        }
    } else if ((*x).ptr).is_null() && ((*y).ptr).is_null() {
        let mut xval: libc::c_int = (*x).val;
        let mut yval: libc::c_int = (*y).val;
        (xval as libc::c_uint != 0x80000000 as libc::c_uint
            && yval as libc::c_uint != 0x80000000 as libc::c_uint
            || {
                glp_assert_(
                    b"xval != 0x80000000 && yval != 0x80000000\0" as *const u8
                        as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    611 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !q.is_null() {
            _glp_mpz_set_si(q, xval / yval);
        }
        if !r.is_null() {
            _glp_mpz_set_si(r, xval % yval);
        }
    } else {
        if ((*x).ptr).is_null() {
            ((*x).val as libc::c_uint != 0x80000000 as libc::c_uint
                || {
                    glp_assert_(
                        b"x->val != 0x80000000\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        621 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if (*x).val >= 0 as libc::c_int {
                sx = 1 as libc::c_int;
                t = (*x).val as libc::c_uint;
            } else {
                sx = -(1 as libc::c_int);
                t = -(*x).val as libc::c_uint;
            }
            ex = &mut dumx;
            (*ex).d[0 as libc::c_int as usize] = t as libc::c_ushort;
            (*ex)
                .d[1 as libc::c_int
                as usize] = (t >> 16 as libc::c_int) as libc::c_ushort;
            (*ex).d[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
            (*ex).d[4 as libc::c_int as usize] = (*ex).d[5 as libc::c_int as usize];
            (*ex).d[3 as libc::c_int as usize] = (*ex).d[4 as libc::c_int as usize];
            (*ex).d[2 as libc::c_int as usize] = (*ex).d[3 as libc::c_int as usize];
            (*ex).next = 0 as *mut mpz_seg;
        } else {
            sx = (*x).val;
            (sx == 1 as libc::c_int || sx == -(1 as libc::c_int)
                || {
                    glp_assert_(
                        b"sx == +1 || sx == -1\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        638 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ex = (*x).ptr;
        }
        if ((*y).ptr).is_null() {
            ((*y).val as libc::c_uint != 0x80000000 as libc::c_uint
                || {
                    glp_assert_(
                        b"y->val != 0x80000000\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        643 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if (*y).val >= 0 as libc::c_int {
                sy = 1 as libc::c_int;
                t = (*y).val as libc::c_uint;
            } else {
                sy = -(1 as libc::c_int);
                t = -(*y).val as libc::c_uint;
            }
            ey = &mut dumy;
            (*ey).d[0 as libc::c_int as usize] = t as libc::c_ushort;
            (*ey)
                .d[1 as libc::c_int
                as usize] = (t >> 16 as libc::c_int) as libc::c_ushort;
            (*ey).d[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
            (*ey).d[4 as libc::c_int as usize] = (*ey).d[5 as libc::c_int as usize];
            (*ey).d[3 as libc::c_int as usize] = (*ey).d[4 as libc::c_int as usize];
            (*ey).d[2 as libc::c_int as usize] = (*ey).d[3 as libc::c_int as usize];
            (*ey).next = 0 as *mut mpz_seg;
        } else {
            sy = (*y).val;
            (sy == 1 as libc::c_int || sy == -(1 as libc::c_int)
                || {
                    glp_assert_(
                        b"sy == +1 || sy == -1\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        660 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ey = (*y).ptr;
        }
        n = 0 as libc::c_int;
        nx = n;
        e = ex;
        while !e.is_null() {
            k = 0 as libc::c_int;
            while k <= 5 as libc::c_int {
                n += 1;
                n;
                if (*e).d[k as usize] != 0 {
                    nx = n;
                }
                k += 1;
                k;
            }
            e = (*e).next;
        }
        (nx > 0 as libc::c_int
            || {
                glp_assert_(
                    b"nx > 0\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    672 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        n = 0 as libc::c_int;
        ny = n;
        e = ey;
        while !e.is_null() {
            k = 0 as libc::c_int;
            while k <= 5 as libc::c_int {
                n += 1;
                n;
                if (*e).d[k as usize] != 0 {
                    ny = n;
                }
                k += 1;
                k;
            }
            e = (*e).next;
        }
        (ny > 0 as libc::c_int
            || {
                glp_assert_(
                    b"ny > 0\0" as *const u8 as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    682 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if nx < ny {
            if !r.is_null() {
                _glp_mpz_set(r, x);
            }
            if !q.is_null() {
                _glp_mpz_set_si(q, 0 as libc::c_int);
            }
        } else {
            work = _glp_gmp_get_work(nx + ny + 1 as libc::c_int);
            wx = &mut *work.offset(0 as libc::c_int as isize) as *mut libc::c_ushort;
            n = 0 as libc::c_int;
            while n < nx {
                *wx.offset(n as isize) = 0 as libc::c_int as libc::c_ushort;
                n += 1;
                n;
            }
            n = 0 as libc::c_int;
            e = ex;
            while !e.is_null() {
                k = 0 as libc::c_int;
                while k <= 5 as libc::c_int {
                    if (*e).d[k as usize] != 0 {
                        *wx.offset(n as isize) = (*e).d[k as usize];
                    }
                    k += 1;
                    k;
                    n += 1;
                    n;
                }
                e = (*e).next;
            }
            wy = &mut *work.offset((nx + 1 as libc::c_int) as isize)
                as *mut libc::c_ushort;
            n = 0 as libc::c_int;
            while n < ny {
                *wy.offset(n as isize) = 0 as libc::c_int as libc::c_ushort;
                n += 1;
                n;
            }
            n = 0 as libc::c_int;
            e = ey;
            while !e.is_null() {
                k = 0 as libc::c_int;
                while k <= 5 as libc::c_int {
                    if (*e).d[k as usize] != 0 {
                        *wy.offset(n as isize) = (*e).d[k as usize];
                    }
                    k += 1;
                    k;
                    n += 1;
                    n;
                }
                e = (*e).next;
            }
            (*wy.offset((ny - 1 as libc::c_int) as isize) as libc::c_int
                != 0 as libc::c_int
                || {
                    glp_assert_(
                        b"wy[ny-1] != 0\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        710 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_bigdiv(nx - ny, ny, wx, wy);
            if !q.is_null() {
                _glp_mpz_set_si(q, 0 as libc::c_int);
                (*q).val = sx * sy;
                es = 0 as *mut mpz_seg;
                k = 6 as libc::c_int;
                n = ny;
                while n <= nx {
                    if k > 5 as libc::c_int {
                        e = _glp_gmp_get_atom(
                            ::std::mem::size_of::<mpz_seg>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut mpz_seg;
                        (*e)
                            .d[2 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_ushort;
                        (*e)
                            .d[1 as libc::c_int
                            as usize] = (*e).d[2 as libc::c_int as usize];
                        (*e)
                            .d[0 as libc::c_int
                            as usize] = (*e).d[1 as libc::c_int as usize];
                        (*e)
                            .d[5 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_ushort;
                        (*e)
                            .d[4 as libc::c_int
                            as usize] = (*e).d[5 as libc::c_int as usize];
                        (*e)
                            .d[3 as libc::c_int
                            as usize] = (*e).d[4 as libc::c_int as usize];
                        (*e).next = 0 as *mut mpz_seg;
                        if ((*q).ptr).is_null() {
                            (*q).ptr = e;
                        } else {
                            (*es).next = e;
                        }
                        es = e;
                        k = 0 as libc::c_int;
                    }
                    let fresh1 = k;
                    k = k + 1;
                    (*es).d[fresh1 as usize] = *wx.offset(n as isize);
                    n += 1;
                    n;
                }
                normalize(q);
            }
            if !r.is_null() {
                _glp_mpz_set_si(r, 0 as libc::c_int);
                (*r).val = sx;
                es = 0 as *mut mpz_seg;
                k = 6 as libc::c_int;
                n = 0 as libc::c_int;
                while n < ny {
                    if k > 5 as libc::c_int {
                        e = _glp_gmp_get_atom(
                            ::std::mem::size_of::<mpz_seg>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut mpz_seg;
                        (*e)
                            .d[2 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_ushort;
                        (*e)
                            .d[1 as libc::c_int
                            as usize] = (*e).d[2 as libc::c_int as usize];
                        (*e)
                            .d[0 as libc::c_int
                            as usize] = (*e).d[1 as libc::c_int as usize];
                        (*e)
                            .d[5 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_ushort;
                        (*e)
                            .d[4 as libc::c_int
                            as usize] = (*e).d[5 as libc::c_int as usize];
                        (*e)
                            .d[3 as libc::c_int
                            as usize] = (*e).d[4 as libc::c_int as usize];
                        (*e).next = 0 as *mut mpz_seg;
                        if ((*r).ptr).is_null() {
                            (*r).ptr = e;
                        } else {
                            (*es).next = e;
                        }
                        es = e;
                        k = 0 as libc::c_int;
                    }
                    let fresh2 = k;
                    k = k + 1;
                    (*es).d[fresh2 as usize] = *wx.offset(n as isize);
                    n += 1;
                    n;
                }
                normalize(r);
            }
        }
    };
}
pub unsafe extern "C" fn _glp_mpz_gcd(mut z: mpz_t, mut x: mpz_t, mut y: mpz_t) {
    let mut u: mpz_t = 0 as *mut mpz;
    let mut v: mpz_t = 0 as *mut mpz;
    let mut r: mpz_t = 0 as *mut mpz;
    u = _glp_mpz_init();
    v = _glp_mpz_init();
    r = _glp_mpz_init();
    _glp_mpz_abs(u, x);
    _glp_mpz_abs(v, y);
    while _glp_mpz_sgn(v) != 0 {
        _glp_mpz_div(0 as mpz_t, r, u, v);
        _glp_mpz_set(u, v);
        _glp_mpz_set(v, r);
    }
    _glp_mpz_set(z, u);
    _glp_mpz_clear(u);
    _glp_mpz_clear(v);
    _glp_mpz_clear(r);
}
pub unsafe extern "C" fn _glp_mpz_cmp(mut x: mpz_t, mut y: mpz_t) -> libc::c_int {
    static mut zero: mpz_seg = {
        let mut init = mpz_seg {
            d: [
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
                0 as libc::c_int as libc::c_ushort,
            ],
            next: 0 as *const mpz_seg as *mut mpz_seg,
        };
        init
    };
    let mut dumx: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut dumy: mpz_seg = mpz_seg {
        d: [0; 6],
        next: 0 as *mut mpz_seg,
    };
    let mut ex: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut ey: *mut mpz_seg = 0 as *mut mpz_seg;
    let mut cc: libc::c_int = 0;
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    if x == y {
        cc = 0 as libc::c_int;
    } else if ((*x).ptr).is_null() && ((*y).ptr).is_null() {
        let mut xval: libc::c_int = (*x).val;
        let mut yval: libc::c_int = (*y).val;
        (xval as libc::c_uint != 0x80000000 as libc::c_uint
            && yval as libc::c_uint != 0x80000000 as libc::c_uint
            || {
                glp_assert_(
                    b"xval != 0x80000000 && yval != 0x80000000\0" as *const u8
                        as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    797 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        cc = if xval > yval {
            1 as libc::c_int
        } else if xval < yval {
            -(1 as libc::c_int)
        } else {
            0 as libc::c_int
        };
    } else if (*x).val > 0 as libc::c_int && (*y).val <= 0 as libc::c_int
        || (*x).val == 0 as libc::c_int && (*y).val < 0 as libc::c_int
    {
        cc = 1 as libc::c_int;
    } else if (*x).val < 0 as libc::c_int && (*y).val >= 0 as libc::c_int
        || (*x).val == 0 as libc::c_int && (*y).val > 0 as libc::c_int
    {
        cc = -(1 as libc::c_int);
    } else {
        if ((*x).ptr).is_null() {
            ((*x).val as libc::c_uint != 0x80000000 as libc::c_uint
                || {
                    glp_assert_(
                        b"x->val != 0x80000000\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        812 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if (*x).val >= 0 as libc::c_int {
                sx = 1 as libc::c_int;
                t = (*x).val as libc::c_uint;
            } else {
                sx = -(1 as libc::c_int);
                t = -(*x).val as libc::c_uint;
            }
            ex = &mut dumx;
            (*ex).d[0 as libc::c_int as usize] = t as libc::c_ushort;
            (*ex)
                .d[1 as libc::c_int
                as usize] = (t >> 16 as libc::c_int) as libc::c_ushort;
            (*ex).d[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
            (*ex).d[4 as libc::c_int as usize] = (*ex).d[5 as libc::c_int as usize];
            (*ex).d[3 as libc::c_int as usize] = (*ex).d[4 as libc::c_int as usize];
            (*ex).d[2 as libc::c_int as usize] = (*ex).d[3 as libc::c_int as usize];
            (*ex).next = 0 as *mut mpz_seg;
        } else {
            sx = (*x).val;
            (sx == 1 as libc::c_int || sx == -(1 as libc::c_int)
                || {
                    glp_assert_(
                        b"sx == +1 || sx == -1\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        829 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ex = (*x).ptr;
        }
        if ((*y).ptr).is_null() {
            ((*y).val as libc::c_uint != 0x80000000 as libc::c_uint
                || {
                    glp_assert_(
                        b"y->val != 0x80000000\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        834 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if (*y).val >= 0 as libc::c_int {
                sy = 1 as libc::c_int;
                t = (*y).val as libc::c_uint;
            } else {
                sy = -(1 as libc::c_int);
                t = -(*y).val as libc::c_uint;
            }
            ey = &mut dumy;
            (*ey).d[0 as libc::c_int as usize] = t as libc::c_ushort;
            (*ey)
                .d[1 as libc::c_int
                as usize] = (t >> 16 as libc::c_int) as libc::c_ushort;
            (*ey).d[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
            (*ey).d[4 as libc::c_int as usize] = (*ey).d[5 as libc::c_int as usize];
            (*ey).d[3 as libc::c_int as usize] = (*ey).d[4 as libc::c_int as usize];
            (*ey).d[2 as libc::c_int as usize] = (*ey).d[3 as libc::c_int as usize];
            (*ey).next = 0 as *mut mpz_seg;
        } else {
            sy = (*y).val;
            (sy == 1 as libc::c_int || sy == -(1 as libc::c_int)
                || {
                    glp_assert_(
                        b"sy == +1 || sy == -1\0" as *const u8 as *const libc::c_char,
                        b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                        851 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ey = (*y).ptr;
        }
        (sx > 0 as libc::c_int && sy > 0 as libc::c_int
            || sx < 0 as libc::c_int && sy < 0 as libc::c_int
            || {
                glp_assert_(
                    b"sx > 0 && sy > 0 || sx < 0 && sy < 0\0" as *const u8
                        as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    855 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        cc = 0 as libc::c_int;
        while !ex.is_null() || !ey.is_null() {
            if ex.is_null() {
                ex = &mut zero;
            }
            if ey.is_null() {
                ey = &mut zero;
            }
            k = 0 as libc::c_int;
            while k <= 5 as libc::c_int {
                if (*ex).d[k as usize] as libc::c_int
                    > (*ey).d[k as usize] as libc::c_int
                {
                    cc = 1 as libc::c_int;
                }
                if ((*ex).d[k as usize] as libc::c_int)
                    < (*ey).d[k as usize] as libc::c_int
                {
                    cc = -(1 as libc::c_int);
                }
                k += 1;
                k;
            }
            ex = (*ex).next;
            ey = (*ey).next;
        }
        if sx < 0 as libc::c_int {
            cc = -cc;
        }
    }
    return cc;
}
pub unsafe extern "C" fn _glp_mpz_sgn(mut x: mpz_t) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = if (*x).val > 0 as libc::c_int {
        1 as libc::c_int
    } else if (*x).val < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    return s;
}
pub unsafe extern "C" fn _glp_mpz_out_str(
    mut _fp: *mut libc::c_void,
    mut base: libc::c_int,
    mut x: mpz_t,
) -> libc::c_int {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut b: mpz_t = 0 as *mut mpz;
    let mut y: mpz_t = 0 as *mut mpz;
    let mut r: mpz_t = 0 as *mut mpz;
    let mut n: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nwr: libc::c_int = 0 as libc::c_int;
    let mut d: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    static mut set: *mut libc::c_char = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    if !(2 as libc::c_int <= base && base <= 36 as libc::c_int) {
        (glp_error_(
            b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
            891 as libc::c_int,
        ))
            .unwrap()(
            b"mpz_out_str: base = %d; invalid base\n\0" as *const u8
                as *const libc::c_char,
            base,
        );
    }
    b = _glp_mpz_init();
    _glp_mpz_set_si(b, base);
    y = _glp_mpz_init();
    r = _glp_mpz_init();
    _glp_mpz_abs(y, x);
    n = 0 as libc::c_int;
    while _glp_mpz_sgn(y) != 0 as libc::c_int {
        _glp_mpz_div(y, 0 as mpz_t, y, b);
        n += 1;
        n;
    }
    if n == 0 as libc::c_int {
        n = 1 as libc::c_int;
    }
    d = glp_alloc(1 as libc::c_int, n) as *mut libc::c_uchar;
    _glp_mpz_abs(y, x);
    j = 0 as libc::c_int;
    while j < n {
        _glp_mpz_div(y, r, y, b);
        (0 as libc::c_int <= (*r).val && (*r).val < base && ((*r).ptr).is_null()
            || {
                glp_assert_(
                    b"0 <= r->val && r->val < base && r->ptr == NULL\0" as *const u8
                        as *const libc::c_char,
                    b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                    906 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        *d.offset(j as isize) = (*r).val as libc::c_uchar;
        j += 1;
        j;
    }
    if fp.is_null() {
        fp = stdout;
    }
    if _glp_mpz_sgn(x) < 0 as libc::c_int {
        fputc('-' as i32, fp);
        nwr += 1;
        nwr;
    }
    j = n - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        fputc(*set.offset(*d.offset(j as isize) as isize) as libc::c_int, fp);
        nwr += 1;
        nwr;
        j -= 1;
        j;
    }
    if ferror(fp) != 0 {
        nwr = 0 as libc::c_int;
    }
    _glp_mpz_clear(b);
    _glp_mpz_clear(y);
    _glp_mpz_clear(r);
    glp_free(d as *mut libc::c_void);
    return nwr;
}
pub unsafe extern "C" fn _glp_mpq_init() -> mpq_t {
    let mut x: mpq_t = 0 as *mut mpq;
    x = _glp_gmp_get_atom(::std::mem::size_of::<mpq>() as libc::c_ulong as libc::c_int)
        as mpq_t;
    (*x).p.val = 0 as libc::c_int;
    (*x).p.ptr = 0 as *mut mpz_seg;
    (*x).q.val = 1 as libc::c_int;
    (*x).q.ptr = 0 as *mut mpz_seg;
    return x;
}
pub unsafe extern "C" fn _glp_mpq_clear(mut x: mpq_t) {
    _glp_mpz_set_si(&mut (*x).p, 0 as libc::c_int);
    (((*x).p.ptr).is_null()
        || {
            glp_assert_(
                b"x->p.ptr == NULL\0" as *const u8 as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                941 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_mpz_set_si(&mut (*x).q, 0 as libc::c_int);
    (((*x).q.ptr).is_null()
        || {
            glp_assert_(
                b"x->q.ptr == NULL\0" as *const u8 as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                943 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_gmp_free_atom(
        x as *mut libc::c_void,
        ::std::mem::size_of::<mpq>() as libc::c_ulong as libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_mpq_canonicalize(mut x: mpq_t) {
    let mut f: mpz_t = 0 as *mut mpz;
    ((*x).q.val != 0 as libc::c_int
        || {
            glp_assert_(
                b"x->q.val != 0\0" as *const u8 as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                953 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*x).q.val < 0 as libc::c_int {
        _glp_mpz_neg(&mut (*x).p, &mut (*x).p);
        _glp_mpz_neg(&mut (*x).q, &mut (*x).q);
    }
    f = _glp_mpz_init();
    _glp_mpz_gcd(f, &mut (*x).p, &mut (*x).q);
    if !((*f).val == 1 as libc::c_int && ((*f).ptr).is_null()) {
        _glp_mpz_div(&mut (*x).p, 0 as mpz_t, &mut (*x).p, f);
        _glp_mpz_div(&mut (*x).q, 0 as mpz_t, &mut (*x).q, f);
    }
    _glp_mpz_clear(f);
}
pub unsafe extern "C" fn _glp_mpq_set(mut z: mpq_t, mut x: mpq_t) {
    if z != x {
        _glp_mpz_set(&mut (*z).p, &mut (*x).p);
        _glp_mpz_set(&mut (*z).q, &mut (*x).q);
    }
}
pub unsafe extern "C" fn _glp_mpq_set_si(
    mut x: mpq_t,
    mut p: libc::c_int,
    mut q: libc::c_uint,
) {
    if q == 0 as libc::c_int as libc::c_uint {
        (glp_error_(
            b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
            980 as libc::c_int,
        ))
            .unwrap()(
            b"mpq_set_si: zero denominator not allowed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    _glp_mpz_set_si(&mut (*x).p, p);
    (q <= 0x7fffffff as libc::c_int as libc::c_uint
        || {
            glp_assert_(
                b"q <= 0x7FFFFFFF\0" as *const u8 as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                982 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_mpz_set_si(&mut (*x).q, q as libc::c_int);
}
pub unsafe extern "C" fn _glp_mpq_get_d(mut x: mpq_t) -> libc::c_double {
    let mut np: libc::c_int = 0;
    let mut nq: libc::c_int = 0;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    p = _glp_mpz_get_d_2exp(&mut np, &mut (*x).p);
    q = _glp_mpz_get_d_2exp(&mut nq, &mut (*x).q);
    return ldexp(p / q, np - nq);
}
pub unsafe extern "C" fn _glp_mpq_set_d(mut x: mpq_t, mut val: libc::c_double) {
    let mut current_block: u64;
    let mut s: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    let mut temp: mpz_t = 0 as *mut mpz;
    (-1.7976931348623157e+308f64 <= val && val <= 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"-DBL_MAX <= val && val <= +DBL_MAX\0" as *const u8
                    as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                1001 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_mpq_set_si(x, 0 as libc::c_int, 1 as libc::c_int as libc::c_uint);
    if val > 0.0f64 {
        s = 1 as libc::c_int;
        current_block = 15427931788582360902;
    } else if val < 0.0f64 {
        s = -(1 as libc::c_int);
        current_block = 15427931788582360902;
    } else {
        current_block = 14232691863315389226;
    }
    match current_block {
        15427931788582360902 => {
            f = frexp(fabs(val), &mut n);
            temp = _glp_mpz_init();
            while f != 0.0f64 {
                f *= 16.0f64;
                n -= 4 as libc::c_int;
                d = f as libc::c_int;
                (0 as libc::c_int <= d && d <= 15 as libc::c_int
                    || {
                        glp_assert_(
                            b"0 <= d && d <= 15\0" as *const u8 as *const libc::c_char,
                            b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                            1015 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                f -= d as libc::c_double;
                _glp_mpz_set_si(temp, 16 as libc::c_int);
                _glp_mpz_mul(&mut (*x).p, &mut (*x).p, temp);
                _glp_mpz_set_si(temp, d);
                _glp_mpz_add(&mut (*x).p, &mut (*x).p, temp);
            }
            _glp_mpz_clear(temp);
            if n > 0 as libc::c_int {
                j = 1 as libc::c_int;
                while j <= n {
                    _glp_mpz_add(&mut (*x).p, &mut (*x).p, &mut (*x).p);
                    j += 1;
                    j;
                }
            } else if n < 0 as libc::c_int {
                j = 1 as libc::c_int;
                while j <= -n {
                    _glp_mpz_add(&mut (*x).q, &mut (*x).q, &mut (*x).q);
                    j += 1;
                    j;
                }
                _glp_mpq_canonicalize(x);
            }
            if s < 0 as libc::c_int {
                _glp_mpq_neg(x, x);
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn _glp_mpq_add(mut z: mpq_t, mut x: mpq_t, mut y: mpq_t) {
    let mut p: mpz_t = 0 as *mut mpz;
    let mut q: mpz_t = 0 as *mut mpz;
    p = _glp_mpz_init();
    q = _glp_mpz_init();
    _glp_mpz_mul(p, &mut (*x).p, &mut (*y).q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).p);
    _glp_mpz_add(p, p, q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).q);
    _glp_mpz_set(&mut (*z).p, p);
    _glp_mpz_set(&mut (*z).q, q);
    _glp_mpz_clear(p);
    _glp_mpz_clear(q);
    _glp_mpq_canonicalize(z);
}
pub unsafe extern "C" fn _glp_mpq_sub(mut z: mpq_t, mut x: mpq_t, mut y: mpq_t) {
    let mut p: mpz_t = 0 as *mut mpz;
    let mut q: mpz_t = 0 as *mut mpz;
    p = _glp_mpz_init();
    q = _glp_mpz_init();
    _glp_mpz_mul(p, &mut (*x).p, &mut (*y).q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).p);
    _glp_mpz_sub(p, p, q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).q);
    _glp_mpz_set(&mut (*z).p, p);
    _glp_mpz_set(&mut (*z).q, q);
    _glp_mpz_clear(p);
    _glp_mpz_clear(q);
    _glp_mpq_canonicalize(z);
}
pub unsafe extern "C" fn _glp_mpq_mul(mut z: mpq_t, mut x: mpq_t, mut y: mpq_t) {
    _glp_mpz_mul(&mut (*z).p, &mut (*x).p, &mut (*y).p);
    _glp_mpz_mul(&mut (*z).q, &mut (*x).q, &mut (*y).q);
    _glp_mpq_canonicalize(z);
}
pub unsafe extern "C" fn _glp_mpq_div(mut z: mpq_t, mut x: mpq_t, mut y: mpq_t) {
    let mut p: mpz_t = 0 as *mut mpz;
    let mut q: mpz_t = 0 as *mut mpz;
    if _glp_mpq_sgn(y) == 0 as libc::c_int {
        (glp_error_(
            b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
            1085 as libc::c_int,
        ))
            .unwrap()(
            b"mpq_div: zero divisor not allowed\n\0" as *const u8 as *const libc::c_char,
        );
    }
    p = _glp_mpz_init();
    q = _glp_mpz_init();
    _glp_mpz_mul(p, &mut (*x).p, &mut (*y).q);
    _glp_mpz_mul(q, &mut (*x).q, &mut (*y).p);
    _glp_mpz_set(&mut (*z).p, p);
    _glp_mpz_set(&mut (*z).q, q);
    _glp_mpz_clear(p);
    _glp_mpz_clear(q);
    _glp_mpq_canonicalize(z);
}
pub unsafe extern "C" fn _glp_mpq_neg(mut z: mpq_t, mut x: mpq_t) {
    _glp_mpq_set(z, x);
    _glp_mpz_neg(&mut (*z).p, &mut (*z).p);
}
pub unsafe extern "C" fn _glp_mpq_abs(mut z: mpq_t, mut x: mpq_t) {
    _glp_mpq_set(z, x);
    _glp_mpz_abs(&mut (*z).p, &mut (*z).p);
    (_glp_mpz_sgn(&mut (*x).q) > 0 as libc::c_int
        || {
            glp_assert_(
                b"mpz_sgn(&x->q) > 0\0" as *const u8 as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                1109 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpq_cmp(mut x: mpq_t, mut y: mpq_t) -> libc::c_int {
    let mut temp: mpq_t = 0 as *mut mpq;
    let mut s: libc::c_int = 0;
    temp = _glp_mpq_init();
    _glp_mpq_sub(temp, x, y);
    s = _glp_mpq_sgn(temp);
    _glp_mpq_clear(temp);
    return s;
}
pub unsafe extern "C" fn _glp_mpq_sgn(mut x: mpq_t) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = _glp_mpz_sgn(&mut (*x).p);
    (_glp_mpz_sgn(&mut (*x).q) > 0 as libc::c_int
        || {
            glp_assert_(
                b"mpz_sgn(&x->q) > 0\0" as *const u8 as *const libc::c_char,
                b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
                1129 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return s;
}
pub unsafe extern "C" fn _glp_mpq_out_str(
    mut _fp: *mut libc::c_void,
    mut base: libc::c_int,
    mut x: mpq_t,
) -> libc::c_int {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut nwr: libc::c_int = 0;
    if !(2 as libc::c_int <= base && base <= 36 as libc::c_int) {
        (glp_error_(
            b"misc/mygmp.c\0" as *const u8 as *const libc::c_char,
            1143 as libc::c_int,
        ))
            .unwrap()(
            b"mpq_out_str: base = %d; invalid base\n\0" as *const u8
                as *const libc::c_char,
            base,
        );
    }
    if fp.is_null() {
        fp = stdout;
    }
    nwr = _glp_mpz_out_str(fp as *mut libc::c_void, base, &mut (*x).p);
    if !((*x).q.val == 1 as libc::c_int && ((*x).q.ptr).is_null()) {
        fputc('/' as i32, fp);
        nwr += 1;
        nwr;
        nwr += _glp_mpz_out_str(fp as *mut libc::c_void, base, &mut (*x).q);
    }
    if ferror(fp) != 0 {
        nwr = 0 as libc::c_int;
    }
    return nwr;
}
