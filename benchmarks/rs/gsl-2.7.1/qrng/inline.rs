use ::libc;
pub type size_t = libc::c_ulong;
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
pub struct gsl_qrng {
    pub type_0: *const gsl_qrng_type,
    pub dimension: libc::c_uint,
    pub state_size: size_t,
    pub state: *mut libc::c_void,
}
pub unsafe extern "C" fn gsl_qrng_get(
    mut q: *const gsl_qrng,
    mut x: *mut libc::c_double,
) -> libc::c_int {
    return ((*(*q).type_0).get).unwrap()((*q).state, (*q).dimension, x);
}
