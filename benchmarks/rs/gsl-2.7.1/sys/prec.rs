use ::libc;
pub type gsl_mode_t = libc::c_uint;
pub unsafe extern "C" fn GSL_MODE_PREC(mut mt: gsl_mode_t) -> libc::c_uint {
    return mt & 7 as libc::c_int as libc::c_uint;
}
pub static mut gsl_prec_eps: [libc::c_double; 3] = [
    2.2204460492503131e-16f64,
    1.1920928955078125e-07f64,
    4.8828125000000000e-04f64,
];
pub static mut gsl_prec_sqrt_eps: [libc::c_double; 3] = [
    1.4901161193847656e-08f64,
    3.4526698300124393e-04f64,
    2.2097086912079612e-02f64,
];
pub static mut gsl_prec_root3_eps: [libc::c_double; 3] = [
    6.0554544523933429e-06f64,
    4.9215666011518501e-03f64,
    7.8745065618429588e-02f64,
];
pub static mut gsl_prec_root4_eps: [libc::c_double; 3] = [
    1.2207031250000000e-04f64,
    1.8581361171917516e-02f64,
    1.4865088937534013e-01f64,
];
pub static mut gsl_prec_root5_eps: [libc::c_double; 3] = [
    7.4009597974140505e-04f64,
    4.1234622211652937e-02f64,
    2.1763764082403100e-01f64,
];
pub static mut gsl_prec_root6_eps: [libc::c_double; 3] = [
    2.4607833005759251e-03f64,
    7.0153878019335827e-02f64,
    2.8061551207734325e-01f64,
];
