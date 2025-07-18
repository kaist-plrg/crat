use ::libc;
extern "C" {
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub unsafe extern "C" fn ti_msw_start(
    mut options: *const libc::c_double,
) -> libc::c_int {
    return *options.offset(0 as libc::c_int as isize) as libc::c_int;
}
pub unsafe extern "C" fn ti_msw(
    mut size: libc::c_int,
    mut inputs: *const *const libc::c_double,
    mut options: *const libc::c_double,
    mut outputs: *const *mut libc::c_double,
) -> libc::c_int {
    let mut input: *const libc::c_double = *inputs.offset(0 as libc::c_int as isize);
    let mut sine: *mut libc::c_double = *outputs.offset(0 as libc::c_int as isize);
    let mut lead: *mut libc::c_double = *outputs.offset(1 as libc::c_int as isize);
    let period: libc::c_int = *options.offset(0 as libc::c_int as isize) as libc::c_int;
    if period < 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if size <= ti_msw_start(options) {
        return 0 as libc::c_int;
    }
    let pi: libc::c_double = 3.1415926f64;
    let tpi: libc::c_double = 2 as libc::c_int as libc::c_double * pi;
    let mut weight: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut phase: libc::c_double = 0.;
    let mut rp: libc::c_double = 0.;
    let mut ip: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = period;
    while i < size {
        rp = 0 as libc::c_int as libc::c_double;
        ip = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int;
        while j < period {
            weight = *input.offset((i - j) as isize);
            rp = rp + cos(tpi * j as libc::c_double / period as libc::c_double) * weight;
            ip = ip + sin(tpi * j as libc::c_double / period as libc::c_double) * weight;
            j += 1;
            j;
        }
        if fabs(rp) > 0.001f64 {
            phase = atan(ip / rp);
        } else {
            phase = tpi / 2.0f64
                * (if ip < 0 as libc::c_int as libc::c_double {
                    -1.0f64
                } else {
                    1.0f64
                });
        }
        if rp < 0.0f64 {
            phase += pi;
        }
        phase += pi / 2.0f64;
        if phase < 0.0f64 {
            phase += tpi;
        }
        if phase > tpi {
            phase -= tpi;
        }
        let fresh0 = sine;
        sine = sine.offset(1);
        *fresh0 = sin(phase);
        let fresh1 = lead;
        lead = lead.offset(1);
        *fresh1 = sin(phase + pi / 4.0f64);
        i += 1;
        i;
    }
    if sine.offset_from(*outputs.offset(0 as libc::c_int as isize)) as libc::c_long
        == (size - ti_msw_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"sine - outputs[0] == size - ti_msw_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/msw.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_msw(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    if lead.offset_from(*outputs.offset(1 as libc::c_int as isize)) as libc::c_long
        == (size - ti_msw_start(options)) as libc::c_long
    {} else {
        __assert_fail(
            b"lead - outputs[1] == size - ti_msw_start(options)\0" as *const u8
                as *const libc::c_char,
            b"indicator/msw.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"int ti_msw(int, const double *const *, const double *, double *const *)\0",
            ))
                .as_ptr(),
        );
    };
    return 0 as libc::c_int;
}
