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
static mut xgk: [libc::c_double; 11] = [
    0.995657163025808080735527280689003f64,
    0.973906528517171720077964012084452f64,
    0.930157491355708226001207180059508f64,
    0.865063366688984510732096688423493f64,
    0.780817726586416897063717578345042f64,
    0.679409568299024406234327365114874f64,
    0.562757134668604683339000099272694f64,
    0.433395394129247190799265943165784f64,
    0.294392862701460198131126603103866f64,
    0.148874338981631210884826001129720f64,
    0.000000000000000000000000000000000f64,
];
static mut wg: [libc::c_double; 5] = [
    0.066671344308688137593568809893332f64,
    0.149451349150580593145776339657697f64,
    0.219086362515982043995534934228163f64,
    0.269266719309996355091226921569469f64,
    0.295524224714752870173892994651338f64,
];
static mut wgk: [libc::c_double; 11] = [
    0.011694638867371874278064396062192f64,
    0.032558162307964727478818972459390f64,
    0.054755896574351996031381300244580f64,
    0.075039674810919952767043140916190f64,
    0.093125454583697605535065465083366f64,
    0.109387158802297641899210590325805f64,
    0.123491976262065851077958109831074f64,
    0.134709217311473325928054001771707f64,
    0.142775938577060080797094273138717f64,
    0.147739104901338491374841515972068f64,
    0.149445554002916905664936468389821f64,
];
pub unsafe extern "C" fn gsl_integration_qk21(
    mut f: *const gsl_function,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
    mut resabs: *mut libc::c_double,
    mut resasc: *mut libc::c_double,
) {
    let mut fv1: [libc::c_double; 11] = [0.; 11];
    let mut fv2: [libc::c_double; 11] = [0.; 11];
    gsl_integration_qk(
        11 as libc::c_int,
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
