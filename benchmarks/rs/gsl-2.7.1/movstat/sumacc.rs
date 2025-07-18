use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_accum {
    pub size: Option::<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option::<unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_int>,
    pub insert: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_int,
    >,
    pub delete_oldest: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub get: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sumacc_state_t {
    pub sum: libc::c_double,
    pub rbuf: *mut ringbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ringbuf {
    pub array: *mut ringbuf_type_t,
    pub head: libc::c_int,
    pub tail: libc::c_int,
    pub size: libc::c_int,
}
pub type ringbuf_type_t = libc::c_double;
unsafe extern "C" fn ringbuf_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<ringbuf>() as libc::c_ulong) as size_t
        as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(
            n.wrapping_mul(::std::mem::size_of::<ringbuf_type_t>() as libc::c_ulong),
        ) as size_t as size_t;
    return size;
}
unsafe extern "C" fn ringbuf_init(n: size_t, mut b: *mut ringbuf) -> libc::c_int {
    (*b)
        .array = (b as *mut libc::c_char)
        .offset(::std::mem::size_of::<ringbuf>() as libc::c_ulong as isize)
        as *mut ringbuf_type_t;
    (*b).head = -(1 as libc::c_int);
    (*b).tail = 0 as libc::c_int;
    (*b).size = n as libc::c_int;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn ringbuf_is_empty(mut b: *const ringbuf) -> libc::c_int {
    return ((*b).head == -(1 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn ringbuf_is_full(mut b: *const ringbuf) -> libc::c_int {
    return ((*b).head == 0 as libc::c_int && (*b).tail == (*b).size - 1 as libc::c_int
        || (*b).head == (*b).tail + 1 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn ringbuf_insert(
    x: ringbuf_type_t,
    mut b: *mut ringbuf,
) -> libc::c_int {
    if (*b).head == -(1 as libc::c_int) {
        (*b).head = 0 as libc::c_int;
        (*b).tail = 0 as libc::c_int;
    } else if (*b).head == 0 as libc::c_int {
        (*b).head = (*b).size - 1 as libc::c_int;
        if (*b).tail == (*b).head && (*b).size > 1 as libc::c_int {
            (*b).tail -= 1;
            (*b).tail;
        }
    } else {
        (*b).head -= 1;
        (*b).head;
        if (*b).tail == (*b).head {
            if (*b).tail == 0 as libc::c_int {
                (*b).tail = (*b).size - 1 as libc::c_int;
            } else {
                (*b).tail -= 1;
                (*b).tail;
            }
        }
    }
    *((*b).array).offset((*b).head as isize) = x;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn ringbuf_pop_back(mut b: *mut ringbuf) -> libc::c_int {
    if ringbuf_is_empty(b) != 0 || (*b).tail < 0 as libc::c_int {
        gsl_error(
            b"buffer is empty\0" as *const u8 as *const libc::c_char,
            b"./ringbuf.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else {
        if (*b).head == (*b).tail {
            (*b).head = -(1 as libc::c_int);
            (*b).tail = -(1 as libc::c_int);
        } else if (*b).tail == 0 as libc::c_int {
            (*b).tail = (*b).size - 1 as libc::c_int;
        } else {
            (*b).tail -= 1;
            (*b).tail;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn ringbuf_peek_back(mut b: *const ringbuf) -> ringbuf_type_t {
    if ringbuf_is_empty(b) != 0 || (*b).tail < 0 as libc::c_int {
        gsl_error(
            b"buffer is empty\0" as *const u8 as *const libc::c_char,
            b"./ringbuf.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int as ringbuf_type_t;
    } else {
        return *((*b).array).offset((*b).tail as isize)
    };
}
unsafe extern "C" fn sumacc_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<sumacc_state_t>() as libc::c_ulong) as size_t
        as size_t;
    size = (size as libc::c_ulong).wrapping_add(ringbuf_size(n)) as size_t as size_t;
    return size;
}
unsafe extern "C" fn sumacc_init(
    n: size_t,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut sumacc_state_t = vstate as *mut sumacc_state_t;
    (*state).sum = 0.0f64;
    (*state)
        .rbuf = (vstate as *mut libc::c_uchar)
        .offset(::std::mem::size_of::<sumacc_state_t>() as libc::c_ulong as isize)
        as *mut ringbuf;
    ringbuf_init(n, (*state).rbuf);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn sumacc_insert(
    x: libc::c_double,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut sumacc_state_t = vstate as *mut sumacc_state_t;
    if ringbuf_is_full((*state).rbuf) != 0 {
        (*state).sum -= ringbuf_peek_back((*state).rbuf);
    }
    (*state).sum += x;
    ringbuf_insert(x, (*state).rbuf);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn sumacc_delete(mut vstate: *mut libc::c_void) -> libc::c_int {
    let mut state: *mut sumacc_state_t = vstate as *mut sumacc_state_t;
    if ringbuf_is_empty((*state).rbuf) == 0 {
        (*state).sum -= ringbuf_peek_back((*state).rbuf);
        ringbuf_pop_back((*state).rbuf);
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn sumacc_get(
    mut params: *mut libc::c_void,
    mut result: *mut libc::c_double,
    mut vstate: *const libc::c_void,
) -> libc::c_int {
    let mut state: *const sumacc_state_t = vstate as *const sumacc_state_t;
    *result = (*state).sum;
    return GSL_SUCCESS as libc::c_int;
}
static mut sum_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(sumacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                sumacc_init
                    as unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_int,
            ),
            insert: Some(
                sumacc_insert
                    as unsafe extern "C" fn(
                        libc::c_double,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            delete_oldest: Some(
                sumacc_delete as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            get: Some(
                sumacc_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_double,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
pub static mut gsl_movstat_accum_sum: *const gsl_movstat_accum = unsafe {
    &sum_accum_type as *const gsl_movstat_accum
};
