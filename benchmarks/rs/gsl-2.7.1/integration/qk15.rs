use ::libc;
extern "C" {
    fn gsl_integration_qk(
        n: libc::c_int,
        xgk_0: *const libc::c_double,
        wg_0: *const libc::c_double,
        wgk_0: *const libc::c_double,
        fv1: *mut libc::c_double,
        fv2: *mut libc::c_double,
        f: *const gsl_function,
        a: libc::c_double,
        b: libc::c_double,
        result: *mut libc::c_double,
        abserr: *mut libc::c_double,
        resabs: *mut libc::c_double,
        resasc: *mut libc::c_double,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_function_struct {
    pub function: Option::<
        unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
pub type gsl_function = gsl_function_struct;
static mut xgk: [libc::c_double; 8] = [
    0.991455371120812639206854697526329f64,
    0.949107912342758524526189684047851f64,
    0.864864423359769072789712788640926f64,
    0.741531185599394439863864773280788f64,
    0.586087235467691130294144838258730f64,
    0.405845151377397166906606412076961f64,
    0.207784955007898467600689403773245f64,
    0.000000000000000000000000000000000f64,
];
static mut wg: [libc::c_double; 4] = [
    0.129484966168869693270611432679082f64,
    0.279705391489276667901467771423780f64,
    0.381830050505118944950369775488975f64,
    0.417959183673469387755102040816327f64,
];
static mut wgk: [libc::c_double; 8] = [
    0.022935322010529224963732008058970f64,
    0.063092092629978553290700663189204f64,
    0.104790010322250183839876322541518f64,
    0.140653259715525918745189590510238f64,
    0.169004726639267902826583426598550f64,
    0.190350578064785409913256402421014f64,
    0.204432940075298892414161999234649f64,
    0.209482141084727828012999174891714f64,
];
pub unsafe extern "C" fn gsl_integration_qk15(
    mut f: *const gsl_function,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
    mut resabs: *mut libc::c_double,
    mut resasc: *mut libc::c_double,
) {
    let mut fv1: [libc::c_double; 8] = [0.; 8];
    let mut fv2: [libc::c_double; 8] = [0.; 8];
    gsl_integration_qk(
        8 as libc::c_int,
        xgk.as_ptr(),
        wg.as_ptr(),
        wgk.as_ptr(),
        fv1.as_mut_ptr(),
        fv2.as_mut_ptr(),
        f,
        a,
        b,
        result,
        abserr,
        resabs,
        resasc,
    );
}
