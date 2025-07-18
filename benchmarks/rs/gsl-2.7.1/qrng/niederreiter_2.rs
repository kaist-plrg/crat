use ::libc;
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
pub struct gsl_qrng_type {
    pub name: *const libc::c_char,
    pub max_dimension: libc::c_uint,
    pub state_size: Option::<unsafe extern "C" fn(libc::c_uint) -> size_t>,
    pub init_state: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_uint) -> libc::c_int,
    >,
    pub get: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_uint,
            *mut libc::c_double,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nied2_state_t {
    pub sequence_count: libc::c_uint,
    pub cj: [[libc::c_int; 12]; 31],
    pub nextq: [libc::c_int; 12],
}
static mut nied2_type: gsl_qrng_type = unsafe {
    {
        let mut init = gsl_qrng_type {
            name: b"niederreiter-base-2\0" as *const u8 as *const libc::c_char,
            max_dimension: 12 as libc::c_int as libc::c_uint,
            state_size: Some(
                nied2_state_size as unsafe extern "C" fn(libc::c_uint) -> size_t,
            ),
            init_state: Some(
                nied2_init
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                    ) -> libc::c_int,
            ),
            get: Some(
                nied2_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                        *mut libc::c_double,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
pub static mut gsl_qrng_niederreiter_2: *const gsl_qrng_type = unsafe {
    &nied2_type as *const gsl_qrng_type
};
static mut primitive_poly: [[libc::c_int; 6]; 13] = [
    [
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ],
];
static mut poly_degree: [libc::c_int; 13] = [
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
];
unsafe extern "C" fn nied2_state_size(mut dimension: libc::c_uint) -> size_t {
    return ::std::mem::size_of::<nied2_state_t>() as libc::c_ulong;
}
unsafe extern "C" fn poly_multiply(
    mut pa: *const libc::c_int,
    mut pa_degree: libc::c_int,
    mut pb: *const libc::c_int,
    mut pb_degree: libc::c_int,
    mut pc: *mut libc::c_int,
    mut pc_degree: *mut libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pt: [libc::c_int; 51] = [0; 51];
    let mut pt_degree: libc::c_int = pa_degree + pb_degree;
    k = 0 as libc::c_int;
    while k <= pt_degree {
        let mut term: libc::c_int = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j <= k {
            let conv_term: libc::c_int = *pa.offset((k - j) as isize)
                * *pb.offset(j as isize) % 2 as libc::c_int;
            term = (term + conv_term) % 2 as libc::c_int;
            j += 1;
            j;
        }
        pt[k as usize] = term;
        k += 1;
        k;
    }
    k = 0 as libc::c_int;
    while k <= pt_degree {
        *pc.offset(k as isize) = pt[k as usize];
        k += 1;
        k;
    }
    k = pt_degree + 1 as libc::c_int;
    while k <= 50 as libc::c_int {
        *pc.offset(k as isize) = 0 as libc::c_int;
        k += 1;
        k;
    }
    *pc_degree = pt_degree;
}
unsafe extern "C" fn calculate_v(
    mut px: *const libc::c_int,
    mut px_degree: libc::c_int,
    mut pb: *mut libc::c_int,
    mut pb_degree: *mut libc::c_int,
    mut v: *mut libc::c_int,
    mut maxv: libc::c_int,
) {
    let nonzero_element: libc::c_int = 1 as libc::c_int;
    let arbitrary_element: libc::c_int = 1 as libc::c_int;
    let mut ph: [libc::c_int; 51] = [0; 51];
    let mut bigm: libc::c_int = *pb_degree;
    let mut m: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut kj: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k <= 50 as libc::c_int {
        ph[k as usize] = *pb.offset(k as isize);
        k += 1;
        k;
    }
    poly_multiply(px, px_degree, pb as *const libc::c_int, *pb_degree, pb, pb_degree);
    m = *pb_degree;
    kj = bigm;
    r = 0 as libc::c_int;
    while r < kj {
        *v.offset(r as isize) = 0 as libc::c_int;
        r += 1;
        r;
    }
    *v.offset(kj as isize) = 1 as libc::c_int;
    if kj >= bigm {
        r = kj + 1 as libc::c_int;
        while r < m {
            *v.offset(r as isize) = arbitrary_element;
            r += 1;
            r;
        }
    } else {
        let mut term: libc::c_int = (0 as libc::c_int + ph[kj as usize])
            % 2 as libc::c_int;
        r = kj + 1 as libc::c_int;
        while r < bigm {
            *v.offset(r as isize) = arbitrary_element;
            term = (term + ph[r as usize] * *v.offset(r as isize) % 2 as libc::c_int)
                % 2 as libc::c_int;
            r += 1;
            r;
        }
        *v.offset(bigm as isize) = (nonzero_element + term) % 2 as libc::c_int;
        r = bigm + 1 as libc::c_int;
        while r < m {
            *v.offset(r as isize) = arbitrary_element;
            r += 1;
            r;
        }
    }
    r = 0 as libc::c_int;
    while r <= maxv - m {
        let mut term_0: libc::c_int = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k < m {
            term_0 = (term_0
                + *pb.offset(k as isize) * *v.offset((r + k) as isize)
                    % 2 as libc::c_int) % 2 as libc::c_int;
            k += 1;
            k;
        }
        *v.offset((r + m) as isize) = term_0;
        r += 1;
        r;
    }
}
unsafe extern "C" fn calculate_cj(
    mut ns: *mut nied2_state_t,
    mut dimension: libc::c_uint,
) {
    let mut ci: [[libc::c_int; 31]; 31] = [[0; 31]; 31];
    let mut v: [libc::c_int; 82] = [0; 82];
    let mut r: libc::c_int = 0;
    let mut i_dim: libc::c_uint = 0;
    i_dim = 0 as libc::c_int as libc::c_uint;
    while i_dim < dimension {
        let poly_index: libc::c_int = i_dim
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        let mut u: libc::c_int = 0 as libc::c_int;
        let mut pb: [libc::c_int; 51] = [0; 51];
        let mut px: [libc::c_int; 51] = [0; 51];
        let mut px_degree: libc::c_int = poly_degree[poly_index as usize];
        let mut pb_degree: libc::c_int = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k <= px_degree {
            px[k as usize] = primitive_poly[poly_index as usize][k as usize];
            pb[k as usize] = 0 as libc::c_int;
            k += 1;
            k;
        }
        while k < 50 as libc::c_int + 1 as libc::c_int {
            px[k as usize] = 0 as libc::c_int;
            pb[k as usize] = 0 as libc::c_int;
            k += 1;
            k;
        }
        pb[0 as libc::c_int as usize] = 1 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 30 as libc::c_int + 1 as libc::c_int {
            if u == 0 as libc::c_int {
                calculate_v(
                    px.as_mut_ptr() as *const libc::c_int,
                    px_degree,
                    pb.as_mut_ptr(),
                    &mut pb_degree,
                    v.as_mut_ptr(),
                    30 as libc::c_int + 1 as libc::c_int + 50 as libc::c_int,
                );
            }
            r = 0 as libc::c_int;
            while r < 30 as libc::c_int + 1 as libc::c_int {
                ci[r as usize][j as usize] = v[(r + u) as usize];
                r += 1;
                r;
            }
            u += 1;
            u;
            if u == px_degree {
                u = 0 as libc::c_int;
            }
            j += 1;
            j;
        }
        r = 0 as libc::c_int;
        while r < 30 as libc::c_int + 1 as libc::c_int {
            let mut term: libc::c_int = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < 30 as libc::c_int + 1 as libc::c_int {
                term = 2 as libc::c_int * term + ci[r as usize][j as usize];
                j += 1;
                j;
            }
            (*ns).cj[r as usize][i_dim as usize] = term;
            r += 1;
            r;
        }
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
}
unsafe extern "C" fn nied2_init(
    mut state: *mut libc::c_void,
    mut dimension: libc::c_uint,
) -> libc::c_int {
    let mut n_state: *mut nied2_state_t = state as *mut nied2_state_t;
    let mut i_dim: libc::c_uint = 0;
    if dimension < 1 as libc::c_int as libc::c_uint
        || dimension > 12 as libc::c_int as libc::c_uint
    {
        return GSL_EINVAL as libc::c_int;
    }
    calculate_cj(n_state, dimension);
    i_dim = 0 as libc::c_int as libc::c_uint;
    while i_dim < dimension {
        (*n_state).nextq[i_dim as usize] = 0 as libc::c_int;
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    (*n_state).sequence_count = 0 as libc::c_int as libc::c_uint;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn nied2_get(
    mut state: *mut libc::c_void,
    mut dimension: libc::c_uint,
    mut v: *mut libc::c_double,
) -> libc::c_int {
    static mut recip: libc::c_double = 1.0f64
        / ((1 as libc::c_uint) << 30 as libc::c_int + 1 as libc::c_int)
            as libc::c_double;
    let mut n_state: *mut nied2_state_t = state as *mut nied2_state_t;
    let mut r: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut i_dim: libc::c_uint = 0;
    i_dim = 0 as libc::c_int as libc::c_uint;
    while i_dim < dimension {
        *v
            .offset(
                i_dim as isize,
            ) = (*n_state).nextq[i_dim as usize] as libc::c_double * recip;
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    r = 0 as libc::c_int;
    c = (*n_state).sequence_count as libc::c_int;
    while c % 2 as libc::c_int == 1 as libc::c_int {
        r += 1;
        r;
        c /= 2 as libc::c_int;
    }
    if r >= 30 as libc::c_int + 1 as libc::c_int {
        return GSL_EFAILED as libc::c_int;
    }
    i_dim = 0 as libc::c_int as libc::c_uint;
    while i_dim < dimension {
        (*n_state).nextq[i_dim as usize] ^= (*n_state).cj[r as usize][i_dim as usize];
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    (*n_state).sequence_count = ((*n_state).sequence_count).wrapping_add(1);
    (*n_state).sequence_count;
    return GSL_SUCCESS as libc::c_int;
}
