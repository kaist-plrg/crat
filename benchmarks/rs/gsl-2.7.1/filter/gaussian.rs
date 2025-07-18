use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exp(_: libc::c_double) -> libc::c_double;
    fn gsl_pow_uint(x: libc::c_double, n: libc::c_uint) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_vector_view_array(v: *mut libc::c_double, n: size_t) -> _gsl_vector_view;
    fn gsl_vector_scale(a: *mut gsl_vector, x: libc::c_double) -> libc::c_int;
    fn gsl_movstat_alloc_with_size(
        accum_state_size: size_t,
        H: size_t,
        J: size_t,
    ) -> *mut gsl_movstat_workspace;
    fn gsl_movstat_free(w: *mut gsl_movstat_workspace);
    fn gsl_movstat_apply_accum(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        accum: *const gsl_movstat_accum,
        accum_params: *mut libc::c_void,
        y: *mut gsl_vector,
        z: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> libc::c_int;
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
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
pub type gsl_movstat_end_t = libc::c_uint;
pub const GSL_MOVSTAT_END_TRUNCATE: gsl_movstat_end_t = 2;
pub const GSL_MOVSTAT_END_PADVALUE: gsl_movstat_end_t = 1;
pub const GSL_MOVSTAT_END_PADZERO: gsl_movstat_end_t = 0;
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
pub struct gsl_movstat_workspace {
    pub H: size_t,
    pub J: size_t,
    pub K: size_t,
    pub work: *mut libc::c_double,
    pub state: *mut libc::c_void,
    pub state_size: size_t,
}
pub type gsl_filter_end_t = libc::c_uint;
pub const GSL_FILTER_END_TRUNCATE: gsl_filter_end_t = 2;
pub const GSL_FILTER_END_PADVALUE: gsl_filter_end_t = 1;
pub const GSL_FILTER_END_PADZERO: gsl_filter_end_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_filter_gaussian_workspace {
    pub K: size_t,
    pub kernel: *mut libc::c_double,
    pub movstat_workspace_p: *mut gsl_movstat_workspace,
}
pub type ringbuf_type_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ringbuf {
    pub array: *mut ringbuf_type_t,
    pub head: libc::c_int,
    pub tail: libc::c_int,
    pub size: libc::c_int,
}
pub type gaussian_type_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gaussian_state_t {
    pub n: size_t,
    pub window: *mut libc::c_double,
    pub rbuf: *mut ringbuf,
}
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_ptr(
    mut v: *mut gsl_vector,
    i: size_t,
) -> *mut libc::c_double {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_poly_eval(
    mut c: *const libc::c_double,
    len: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut ans: libc::c_double = *c.offset((len - 1 as libc::c_int) as isize);
    i = len - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        ans = *c.offset((i - 1 as libc::c_int) as isize) + x * ans;
        i -= 1;
        i;
    }
    return ans;
}
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
            132 as libc::c_int,
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
        let n: libc::c_int = if (*b).head > (*b).tail {
            (*b).size - (*b).head + (*b).tail + 1 as libc::c_int
        } else {
            (*b).tail - (*b).head + 1 as libc::c_int
        };
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < n {
            *dest
                .offset(
                    (n - i - 1 as libc::c_int) as isize,
                ) = *((*b).array).offset((((*b).head + i) % (*b).size) as isize);
            i += 1;
            i;
        }
        return n as size_t;
    };
}
pub unsafe extern "C" fn gsl_filter_gaussian_alloc(
    K: size_t,
) -> *mut gsl_filter_gaussian_workspace {
    let H: size_t = K.wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut w: *mut gsl_filter_gaussian_workspace = 0
        as *mut gsl_filter_gaussian_workspace;
    let mut state_size: size_t = 0;
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<gsl_filter_gaussian_workspace>() as libc::c_ulong,
    ) as *mut gsl_filter_gaussian_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"gaussian.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_gaussian_workspace;
    }
    (*w)
        .K = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(H)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*w)
        .kernel = malloc(
        ((*w).K).wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*w).kernel).is_null() {
        gsl_filter_gaussian_free(w);
        gsl_error(
            b"failed to allocate space for kernel\0" as *const u8 as *const libc::c_char,
            b"gaussian.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_gaussian_workspace;
    }
    state_size = gaussian_size((*w).K);
    (*w).movstat_workspace_p = gsl_movstat_alloc_with_size(state_size, H, H);
    if ((*w).movstat_workspace_p).is_null() {
        gsl_filter_gaussian_free(w);
        gsl_error(
            b"failed to allocate space for movstat workspace\0" as *const u8
                as *const libc::c_char,
            b"gaussian.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_gaussian_workspace;
    }
    return w;
}
pub unsafe extern "C" fn gsl_filter_gaussian_free(
    mut w: *mut gsl_filter_gaussian_workspace,
) {
    if !((*w).kernel).is_null() {
        free((*w).kernel as *mut libc::c_void);
    }
    if !((*w).movstat_workspace_p).is_null() {
        gsl_movstat_free((*w).movstat_workspace_p);
    }
    free(w as *mut libc::c_void);
}
pub unsafe extern "C" fn gsl_filter_gaussian(
    endtype: gsl_filter_end_t,
    alpha: libc::c_double,
    order: size_t,
    mut x: *const gsl_vector,
    mut y: *mut gsl_vector,
    mut w: *mut gsl_filter_gaussian_workspace,
) -> libc::c_int {
    if (*x).size != (*y).size {
        gsl_error(
            b"input and output vectors must have same length\0" as *const u8
                as *const libc::c_char,
            b"gaussian.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            GSL_EBADLEN as libc::c_int,
        );
        return GSL_EBADLEN as libc::c_int;
    } else if alpha <= 0.0f64 {
        gsl_error(
            b"alpha must be positive\0" as *const u8 as *const libc::c_char,
            b"gaussian.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut status: libc::c_int = 0;
        let mut kernel: gsl_vector_view = gsl_vector_view_array((*w).kernel, (*w).K);
        gsl_filter_gaussian_kernel(alpha, order, 1 as libc::c_int, &mut kernel.vector);
        status = gsl_movstat_apply_accum(
            endtype as gsl_movstat_end_t,
            x,
            &gaussian_accum_type,
            (*w).kernel as *mut libc::c_void,
            y,
            0 as *mut gsl_vector,
            (*w).movstat_workspace_p,
        );
        return status;
    };
}
pub unsafe extern "C" fn gsl_filter_gaussian_kernel(
    alpha: libc::c_double,
    order: size_t,
    normalize: libc::c_int,
    mut kernel: *mut gsl_vector,
) -> libc::c_int {
    let N: size_t = (*kernel).size;
    if alpha <= 0.0f64 {
        gsl_error(
            b"alpha must be positive\0" as *const u8 as *const libc::c_char,
            b"gaussian.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if order > 10 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"derivative order is too large\0" as *const u8 as *const libc::c_char,
            b"gaussian.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let half: libc::c_double = 0.5f64 * (N as libc::c_double - 1.0f64);
        let mut sum: libc::c_double = 0.0f64;
        let mut i: size_t = 0;
        if N == 1 as libc::c_int as libc::c_ulong {
            if order == 0 as libc::c_int as libc::c_ulong {
                gsl_vector_set(kernel, 0 as libc::c_int as size_t, 1.0f64);
            } else {
                gsl_vector_set(kernel, 0 as libc::c_int as size_t, 0.0f64);
            }
            return GSL_SUCCESS as libc::c_int;
        }
        i = 0 as libc::c_int as size_t;
        while i < N {
            let mut xi: libc::c_double = (i as libc::c_double - half) / half;
            let mut yi: libc::c_double = alpha * xi;
            let mut gi: libc::c_double = exp(-0.5f64 * yi * yi);
            gsl_vector_set(kernel, i, gi);
            sum += gi;
            i = i.wrapping_add(1);
            i;
        }
        if normalize != 0 {
            gsl_vector_scale(kernel, 1.0f64 / sum);
        }
        if order > 0 as libc::c_int as libc::c_ulong {
            let beta: libc::c_double = -0.5f64 * alpha * alpha;
            let mut q: [libc::c_double; 11] = [0.; 11];
            let mut k: size_t = 0;
            q[0 as libc::c_int
                as usize] = 1.0f64 / gsl_pow_uint(half, order as libc::c_uint);
            i = 1 as libc::c_int as size_t;
            while i <= 10 as libc::c_int as libc::c_ulong {
                q[i as usize] = 0.0f64;
                i = i.wrapping_add(1);
                i;
            }
            k = 1 as libc::c_int as size_t;
            while k <= order {
                let mut qm1: libc::c_double = q[0 as libc::c_int as usize];
                q[0 as libc::c_int as usize] = q[1 as libc::c_int as usize];
                i = 1 as libc::c_int as size_t;
                while i <= k {
                    let mut tmp: libc::c_double = q[i as usize];
                    q[i
                        as usize] = (i as libc::c_double + 1.0f64)
                        * q[i.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize]
                        + 2.0f64 * beta * qm1;
                    qm1 = tmp;
                    i = i.wrapping_add(1);
                    i;
                }
                k = k.wrapping_add(1);
                k;
            }
            i = 0 as libc::c_int as size_t;
            while i < N {
                let mut xi_0: libc::c_double = (i as libc::c_double - half) / half;
                let mut qn: libc::c_double = gsl_poly_eval(
                    q.as_mut_ptr() as *const libc::c_double,
                    order.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                    xi_0,
                );
                let mut wn: *mut libc::c_double = gsl_vector_ptr(kernel, i);
                *wn *= qn;
                i = i.wrapping_add(1);
                i;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn gaussian_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<gaussian_state_t>() as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(
            n.wrapping_mul(::std::mem::size_of::<gaussian_type_t>() as libc::c_ulong),
        ) as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(ringbuf_size(n)) as size_t as size_t;
    return size;
}
unsafe extern "C" fn gaussian_init(
    n: size_t,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut gaussian_state_t = vstate as *mut gaussian_state_t;
    (*state).n = n;
    (*state)
        .window = (vstate as *mut libc::c_uchar)
        .offset(::std::mem::size_of::<gaussian_state_t>() as libc::c_ulong as isize)
        as *mut gaussian_type_t;
    (*state)
        .rbuf = ((*state).window as *mut libc::c_uchar)
        .offset(
            n.wrapping_mul(::std::mem::size_of::<gaussian_type_t>() as libc::c_ulong)
                as isize,
        ) as *mut ringbuf;
    ringbuf_init(n, (*state).rbuf);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gaussian_insert(
    x: gaussian_type_t,
    mut vstate: *mut libc::c_void,
) -> libc::c_int {
    let mut state: *mut gaussian_state_t = vstate as *mut gaussian_state_t;
    ringbuf_insert(x, (*state).rbuf);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gaussian_delete(mut vstate: *mut libc::c_void) -> libc::c_int {
    let mut state: *mut gaussian_state_t = vstate as *mut gaussian_state_t;
    if ringbuf_is_empty((*state).rbuf) == 0 {
        ringbuf_pop_back((*state).rbuf);
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn gaussian_get(
    mut params: *mut libc::c_void,
    mut result: *mut gaussian_type_t,
    mut vstate: *const libc::c_void,
) -> libc::c_int {
    let mut state: *const gaussian_state_t = vstate as *const gaussian_state_t;
    let mut kernel: *const libc::c_double = params as *const libc::c_double;
    let mut n: size_t = ringbuf_copy((*state).window, (*state).rbuf);
    let mut sum: libc::c_double = 0.0f64;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        sum
            += *((*state).window).offset(i as isize)
                * *kernel
                    .offset(
                        n.wrapping_sub(i).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    );
        i = i.wrapping_add(1);
        i;
    }
    *result = sum;
    return GSL_SUCCESS as libc::c_int;
}
static mut gaussian_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(gaussian_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                gaussian_init
                    as unsafe extern "C" fn(size_t, *mut libc::c_void) -> libc::c_int,
            ),
            insert: Some(
                gaussian_insert
                    as unsafe extern "C" fn(
                        gaussian_type_t,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            delete_oldest: Some(
                gaussian_delete as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
            get: Some(
                gaussian_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut gaussian_type_t,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
