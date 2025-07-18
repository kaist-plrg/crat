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
pub struct sobol_state_t {
    pub sequence_count: libc::c_uint,
    pub last_denominator_inv: libc::c_double,
    pub last_numerator_vec: [libc::c_int; 40],
    pub v_direction: [[libc::c_int; 40]; 30],
}
static mut sobol_type: gsl_qrng_type = unsafe {
    {
        let mut init = gsl_qrng_type {
            name: b"sobol\0" as *const u8 as *const libc::c_char,
            max_dimension: 40 as libc::c_int as libc::c_uint,
            state_size: Some(
                sobol_state_size as unsafe extern "C" fn(libc::c_uint) -> size_t,
            ),
            init_state: Some(
                sobol_init
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_uint,
                    ) -> libc::c_int,
            ),
            get: Some(
                sobol_get
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
pub static mut gsl_qrng_sobol: *const gsl_qrng_type = unsafe {
    &sobol_type as *const gsl_qrng_type
};
static mut primitive_polynomials: [libc::c_int; 40] = [
    1 as libc::c_int,
    3 as libc::c_int,
    7 as libc::c_int,
    11 as libc::c_int,
    13 as libc::c_int,
    19 as libc::c_int,
    25 as libc::c_int,
    37 as libc::c_int,
    59 as libc::c_int,
    47 as libc::c_int,
    61 as libc::c_int,
    55 as libc::c_int,
    41 as libc::c_int,
    67 as libc::c_int,
    97 as libc::c_int,
    91 as libc::c_int,
    109 as libc::c_int,
    103 as libc::c_int,
    115 as libc::c_int,
    131 as libc::c_int,
    193 as libc::c_int,
    137 as libc::c_int,
    145 as libc::c_int,
    143 as libc::c_int,
    241 as libc::c_int,
    157 as libc::c_int,
    185 as libc::c_int,
    167 as libc::c_int,
    229 as libc::c_int,
    171 as libc::c_int,
    213 as libc::c_int,
    191 as libc::c_int,
    253 as libc::c_int,
    203 as libc::c_int,
    211 as libc::c_int,
    239 as libc::c_int,
    247 as libc::c_int,
    285 as libc::c_int,
    369 as libc::c_int,
    299 as libc::c_int,
];
static mut degree_table: [libc::c_int; 40] = [
    0 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    8 as libc::c_int,
    8 as libc::c_int,
];
static mut v_init: [[libc::c_int; 40]; 8] = [
    [
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        7 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        7 as libc::c_int,
        5 as libc::c_int,
        5 as libc::c_int,
        7 as libc::c_int,
        7 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        7 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        5 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        7 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
        7 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        5 as libc::c_int,
        7 as libc::c_int,
        7 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        7 as libc::c_int,
        9 as libc::c_int,
        13 as libc::c_int,
        11 as libc::c_int,
        1 as libc::c_int,
        3 as libc::c_int,
        7 as libc::c_int,
        9 as libc::c_int,
        5 as libc::c_int,
        13 as libc::c_int,
        13 as libc::c_int,
        11 as libc::c_int,
        3 as libc::c_int,
        15 as libc::c_int,
        5 as libc::c_int,
        3 as libc::c_int,
        15 as libc::c_int,
        7 as libc::c_int,
        9 as libc::c_int,
        13 as libc::c_int,
        9 as libc::c_int,
        1 as libc::c_int,
        11 as libc::c_int,
        7 as libc::c_int,
        5 as libc::c_int,
        15 as libc::c_int,
        1 as libc::c_int,
        15 as libc::c_int,
        11 as libc::c_int,
        5 as libc::c_int,
        3 as libc::c_int,
        1 as libc::c_int,
        7 as libc::c_int,
        9 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        9 as libc::c_int,
        3 as libc::c_int,
        27 as libc::c_int,
        15 as libc::c_int,
        29 as libc::c_int,
        21 as libc::c_int,
        23 as libc::c_int,
        19 as libc::c_int,
        11 as libc::c_int,
        25 as libc::c_int,
        7 as libc::c_int,
        13 as libc::c_int,
        17 as libc::c_int,
        1 as libc::c_int,
        25 as libc::c_int,
        29 as libc::c_int,
        3 as libc::c_int,
        31 as libc::c_int,
        11 as libc::c_int,
        5 as libc::c_int,
        23 as libc::c_int,
        27 as libc::c_int,
        19 as libc::c_int,
        21 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        17 as libc::c_int,
        13 as libc::c_int,
        7 as libc::c_int,
        15 as libc::c_int,
        9 as libc::c_int,
        31 as libc::c_int,
        9 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        37 as libc::c_int,
        33 as libc::c_int,
        7 as libc::c_int,
        5 as libc::c_int,
        11 as libc::c_int,
        39 as libc::c_int,
        63 as libc::c_int,
        27 as libc::c_int,
        17 as libc::c_int,
        15 as libc::c_int,
        23 as libc::c_int,
        29 as libc::c_int,
        3 as libc::c_int,
        21 as libc::c_int,
        13 as libc::c_int,
        31 as libc::c_int,
        25 as libc::c_int,
        9 as libc::c_int,
        49 as libc::c_int,
        33 as libc::c_int,
        19 as libc::c_int,
        29 as libc::c_int,
        11 as libc::c_int,
        19 as libc::c_int,
        27 as libc::c_int,
        15 as libc::c_int,
        25 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        13 as libc::c_int,
        33 as libc::c_int,
        115 as libc::c_int,
        41 as libc::c_int,
        79 as libc::c_int,
        17 as libc::c_int,
        29 as libc::c_int,
        119 as libc::c_int,
        75 as libc::c_int,
        73 as libc::c_int,
        105 as libc::c_int,
        7 as libc::c_int,
        59 as libc::c_int,
        65 as libc::c_int,
        21 as libc::c_int,
        3 as libc::c_int,
        113 as libc::c_int,
        61 as libc::c_int,
        89 as libc::c_int,
        45 as libc::c_int,
        107 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        7 as libc::c_int,
        23 as libc::c_int,
        39 as libc::c_int,
    ],
];
unsafe extern "C" fn sobol_state_size(mut dimension: libc::c_uint) -> size_t {
    return ::std::mem::size_of::<sobol_state_t>() as libc::c_ulong;
}
unsafe extern "C" fn sobol_init(
    mut state: *mut libc::c_void,
    mut dimension: libc::c_uint,
) -> libc::c_int {
    let mut s_state: *mut sobol_state_t = state as *mut sobol_state_t;
    let mut i_dim: libc::c_uint = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ell: libc::c_int = 0;
    if dimension < 1 as libc::c_int as libc::c_uint
        || dimension > 40 as libc::c_int as libc::c_uint
    {
        return GSL_EINVAL as libc::c_int;
    }
    k = 0 as libc::c_int;
    while k < 30 as libc::c_int {
        (*s_state).v_direction[k as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
        k += 1;
        k;
    }
    i_dim = 1 as libc::c_int as libc::c_uint;
    while i_dim < dimension {
        let poly_index: libc::c_int = i_dim as libc::c_int;
        let degree_i: libc::c_int = degree_table[poly_index as usize];
        let mut includ: [libc::c_int; 8] = [0; 8];
        let mut p_i: libc::c_int = primitive_polynomials[poly_index as usize];
        k = degree_i - 1 as libc::c_int;
        while k >= 0 as libc::c_int {
            includ[k
                as usize] = (p_i % 2 as libc::c_int == 1 as libc::c_int) as libc::c_int;
            p_i /= 2 as libc::c_int;
            k -= 1;
            k;
        }
        j = 0 as libc::c_int;
        while j < degree_i {
            (*s_state)
                .v_direction[j
                as usize][i_dim as usize] = v_init[j as usize][i_dim as usize];
            j += 1;
            j;
        }
        j = degree_i;
        while j < 30 as libc::c_int {
            let mut newv: libc::c_int = (*s_state)
                .v_direction[(j - degree_i) as usize][i_dim as usize];
            ell = 1 as libc::c_int;
            k = 0 as libc::c_int;
            while k < degree_i {
                ell *= 2 as libc::c_int;
                if includ[k as usize] != 0 {
                    newv
                        ^= ell
                            * (*s_state)
                                .v_direction[(j - k - 1 as libc::c_int)
                                as usize][i_dim as usize];
                }
                k += 1;
                k;
            }
            (*s_state).v_direction[j as usize][i_dim as usize] = newv;
            j += 1;
            j;
        }
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    ell = 1 as libc::c_int;
    j = 30 as libc::c_int - 1 as libc::c_int - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        ell *= 2 as libc::c_int;
        i_dim = 0 as libc::c_int as libc::c_uint;
        while i_dim < dimension {
            (*s_state).v_direction[j as usize][i_dim as usize] *= ell;
            i_dim = i_dim.wrapping_add(1);
            i_dim;
        }
        j -= 1;
        j;
    }
    (*s_state).last_denominator_inv = 1.0f64 / (2.0f64 * ell as libc::c_double);
    (*s_state).sequence_count = 0 as libc::c_int as libc::c_uint;
    i_dim = 0 as libc::c_int as libc::c_uint;
    while i_dim < dimension {
        (*s_state).last_numerator_vec[i_dim as usize] = 0 as libc::c_int;
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn sobol_get(
    mut state: *mut libc::c_void,
    mut dimension: libc::c_uint,
    mut v: *mut libc::c_double,
) -> libc::c_int {
    let mut s_state: *mut sobol_state_t = state as *mut sobol_state_t;
    let mut i_dimension: libc::c_uint = 0;
    let mut ell: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = (*s_state).sequence_count as libc::c_int;
    loop {
        ell += 1;
        ell;
        if !(c % 2 as libc::c_int == 1 as libc::c_int) {
            break;
        }
        c /= 2 as libc::c_int;
    }
    if ell > 30 as libc::c_int {
        return GSL_EFAILED as libc::c_int;
    }
    i_dimension = 0 as libc::c_int as libc::c_uint;
    while i_dimension < dimension {
        let direction_i: libc::c_int = (*s_state)
            .v_direction[(ell - 1 as libc::c_int) as usize][i_dimension as usize];
        let old_numerator_i: libc::c_int = (*s_state)
            .last_numerator_vec[i_dimension as usize];
        let new_numerator_i: libc::c_int = old_numerator_i ^ direction_i;
        (*s_state).last_numerator_vec[i_dimension as usize] = new_numerator_i;
        *v
            .offset(
                i_dimension as isize,
            ) = new_numerator_i as libc::c_double * (*s_state).last_denominator_inv;
        i_dimension = i_dimension.wrapping_add(1);
        i_dimension;
    }
    (*s_state).sequence_count = ((*s_state).sequence_count).wrapping_add(1);
    (*s_state).sequence_count;
    return GSL_SUCCESS as libc::c_int;
}
