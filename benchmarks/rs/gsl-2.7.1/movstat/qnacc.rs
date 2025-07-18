use ::libc;
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sort(data: *mut libc::c_double, stride: size_t, n: size_t);
    fn gsl_stats_Qn_from_sorted_data(
        sorted_data: *const libc::c_double,
        stride: size_t,
        n: size_t,
        work: *mut libc::c_double,
        work_int: *mut libc::c_int,
    ) -> libc::c_double;
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
pub type qnacc_type_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct qnacc_state_t {
    pub window: *mut qnacc_type_t,
    pub work: *mut qnacc_type_t,
    pub work_int: *mut libc::c_int,
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
pub type ringbuf_type_t = qnacc_type_t;
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
unsafe extern "C" fn ringbuf_copy(
    mut dest: *mut libc::c_double,
    mut b: *const ringbuf,
) -> size_t {
    if ringbuf_is_empty(b) != 0 || (*b).tail < 0 as libc::c_int {
        return 0 as libc::c_int as size_t
    } else {
        let n: libc::c_int = ringbuf_n(b);
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < n {
            *dest
                .offset(
                    i as isize,
                ) = *((*b).array).offset((((*b).head + i) % (*b).size) as isize);
            i += 1;
            i;
        }
        return n as size_t;
    };
}
unsafe extern "C" fn ringbuf_n(mut b: *const ringbuf) -> libc::c_int {
    let n: libc::c_int = if (*b).head > (*b).tail {
        (*b).size - (*b).head + (*b).tail + 1 as libc::c_int
    } else {
        (*b).tail - (*b).head + 1 as libc::c_int
    };
    return n;
}
unsafe extern "C" fn qnacc_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<qnacc_state_t>() as libc::c_ulong) as size_t
        as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(
            n.wrapping_mul(::std::mem::size_of::<qnacc_type_t>() as libc::c_ulong),
        ) as size_t as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(n)
                .wrapping_mul(::std::mem::size_of::<qnacc_type_t>() as libc::c_ulong),
        ) as size_t as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(
            (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(n)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(ringbuf_size(n)) as size_t as size_t;
    return size;
}
unsafe extern "C" fn qnacc_init(
    n: size_t,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut qnacc_state_t = vstate as *mut qnacc_state_t;
    (*state)
        .window = (vstate as *mut libc::c_uchar)
        .offset(::std::mem::size_of::<qnacc_state_t>() as libc::c_ulong as isize)
        as *mut qnacc_type_t;
    (*state)
        .work = ((*state).window as *mut libc::c_uchar)
        .offset(
            n.wrapping_mul(::std::mem::size_of::<qnacc_type_t>() as libc::c_ulong)
                as isize,
        ) as *mut qnacc_type_t;
    (*state)
        .work_int = ((*state).work as *mut libc::c_uchar)
        .offset(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(n)
                .wrapping_mul(::std::mem::size_of::<qnacc_type_t>() as libc::c_ulong)
                as isize,
        ) as *mut libc::c_int;
    (*state)
        .rbuf = ((*state).work_int as *mut libc::c_uchar)
        .offset(
            (5 as libc::c_int as libc::c_ulong)
                .wrapping_mul(n)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as isize,
        ) as *mut ringbuf;
    ringbuf_init(n, (*state).rbuf);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn qnacc_insert(
    x: qnacc_type_t,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut qnacc_state_t = vstate as *mut qnacc_state_t;
    ringbuf_insert(x, (*state).rbuf);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn qnacc_delete(mut vstate: *mut libc::c_void) -> libc::c_int {
    let mut state: *mut qnacc_state_t = vstate as *mut qnacc_state_t;
    if ringbuf_is_empty((*state).rbuf) == 0 {
        ringbuf_pop_back((*state).rbuf);
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn qnacc_get(
    mut params: *mut libc::c_void,
    mut result: *mut qnacc_type_t,
    mut vstate: *const libc::c_void,
) -> libc::c_int {
    let mut state: *const qnacc_state_t = vstate as *const qnacc_state_t;
    let mut n: size_t = ringbuf_copy((*state).window, (*state).rbuf);
    gsl_sort((*state).window, 1 as libc::c_int as size_t, n);
    *result = gsl_stats_Qn_from_sorted_data(
        (*state).window as *const libc::c_double,
        1 as libc::c_int as size_t,
        n,
        (*state).work,
        (*state).work_int,
    );
    return GSL_SUCCESS as libc::c_int;
}
static mut qn_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(qnacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                qnacc_init
                    as unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_int,
            ),
            insert: Some(
                qnacc_insert
                    as unsafe extern "C" fn(
                        qnacc_type_t,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            delete_oldest: Some(
                qnacc_delete as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            get: Some(
                qnacc_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut qnacc_type_t,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
pub static mut gsl_movstat_accum_Qn: *const gsl_movstat_accum = unsafe {
    &qn_accum_type as *const gsl_movstat_accum
};
