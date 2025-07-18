use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_sin_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_cos_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
}
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
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
pub type cheb_series = cheb_series_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cheb_series_struct {
    pub c: *mut libc::c_double,
    pub order: libc::c_int,
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub order_sp: libc::c_int,
}
#[inline]
unsafe extern "C" fn cheb_eval_e(
    mut cs: *const cheb_series,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut d: libc::c_double = 0.0f64;
    let mut dd: libc::c_double = 0.0f64;
    let mut y: libc::c_double = (2.0f64 * x - (*cs).a - (*cs).b) / ((*cs).b - (*cs).a);
    let mut y2: libc::c_double = 2.0f64 * y;
    let mut e: libc::c_double = 0.0f64;
    j = (*cs).order;
    while j >= 1 as libc::c_int {
        let mut temp: libc::c_double = d;
        d = y2 * d - dd + *((*cs).c).offset(j as isize);
        e += fabs(y2 * temp) + fabs(dd) + fabs(*((*cs).c).offset(j as isize));
        dd = temp;
        j -= 1;
        j;
    }
    let mut temp_0: libc::c_double = d;
    d = y * d - dd + 0.5f64 * *((*cs).c).offset(0 as libc::c_int as isize);
    e
        += fabs(y * temp_0) + fabs(dd)
            + 0.5f64 * fabs(*((*cs).c).offset(0 as libc::c_int as isize));
    (*result).val = d;
    (*result)
        .err = 2.2204460492503131e-16f64 * e
        + fabs(*((*cs).c).offset((*cs).order as isize));
    return GSL_SUCCESS as libc::c_int;
}
static mut f1_data: [libc::c_double; 20] = [
    -0.1191081969051363610f64,
    -0.0247823144996236248f64,
    0.0011910281453357821f64,
    -0.0000927027714388562f64,
    0.0000093373141568271f64,
    -0.0000011058287820557f64,
    0.0000001464772071460f64,
    -0.0000000210694496288f64,
    0.0000000032293492367f64,
    -0.0000000005206529618f64,
    0.0000000000874878885f64,
    -0.0000000000152176187f64,
    0.0000000000027257192f64,
    -0.0000000000005007053f64,
    0.0000000000000940241f64,
    -0.0000000000000180014f64,
    0.0000000000000035063f64,
    -0.0000000000000006935f64,
    0.0000000000000001391f64,
    -0.0000000000000000282f64,
];
static mut f1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: f1_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut f2_data: [libc::c_double; 29] = [
    -0.0348409253897013234f64,
    -0.0166842205677959686f64,
    0.0006752901241237738f64,
    -0.0000535066622544701f64,
    0.0000062693421779007f64,
    -0.0000009526638801991f64,
    0.0000001745629224251f64,
    -0.0000000368795403065f64,
    0.0000000087202677705f64,
    -0.0000000022601970392f64,
    0.0000000006324624977f64,
    -0.0000000001888911889f64,
    0.0000000000596774674f64,
    -0.0000000000198044313f64,
    0.0000000000068641396f64,
    -0.0000000000024731020f64,
    0.0000000000009226360f64,
    -0.0000000000003552364f64,
    0.0000000000001407606f64,
    -0.0000000000000572623f64,
    0.0000000000000238654f64,
    -0.0000000000000101714f64,
    0.0000000000000044259f64,
    -0.0000000000000019634f64,
    0.0000000000000008868f64,
    -0.0000000000000004074f64,
    0.0000000000000001901f64,
    -0.0000000000000000900f64,
    0.0000000000000000432f64,
];
static mut f2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: f2_data.as_ptr() as *mut _,
            order: 28 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 14 as libc::c_int,
        };
        init
    }
};
static mut g1_data: [libc::c_double; 21] = [
    -0.3040578798253495954f64,
    -0.0566890984597120588f64,
    0.0039046158173275644f64,
    -0.0003746075959202261f64,
    0.0000435431556559844f64,
    -0.0000057417294453025f64,
    0.0000008282552104503f64,
    -0.0000001278245892595f64,
    0.0000000207978352949f64,
    -0.0000000035313205922f64,
    0.0000000006210824236f64,
    -0.0000000001125215474f64,
    0.0000000000209088918f64,
    -0.0000000000039715832f64,
    0.0000000000007690431f64,
    -0.0000000000001514697f64,
    0.0000000000000302892f64,
    -0.0000000000000061400f64,
    0.0000000000000012601f64,
    -0.0000000000000002615f64,
    0.0000000000000000548f64,
];
static mut g1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: g1_data.as_ptr() as *mut _,
            order: 20 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 13 as libc::c_int,
        };
        init
    }
};
static mut g2_data: [libc::c_double; 34] = [
    -0.0967329367532432218f64,
    -0.0452077907957459871f64,
    0.0028190005352706523f64,
    -0.0002899167740759160f64,
    0.0000407444664601121f64,
    -0.0000071056382192354f64,
    0.0000014534723163019f64,
    -0.0000003364116512503f64,
    0.0000000859774367886f64,
    -0.0000000238437656302f64,
    0.0000000070831906340f64,
    -0.0000000022318068154f64,
    0.0000000007401087359f64,
    -0.0000000002567171162f64,
    0.0000000000926707021f64,
    -0.0000000000346693311f64,
    0.0000000000133950573f64,
    -0.0000000000053290754f64,
    0.0000000000021775312f64,
    -0.0000000000009118621f64,
    0.0000000000003905864f64,
    -0.0000000000001708459f64,
    0.0000000000000762015f64,
    -0.0000000000000346151f64,
    0.0000000000000159996f64,
    -0.0000000000000075213f64,
    0.0000000000000035970f64,
    -0.0000000000000017530f64,
    0.0000000000000008738f64,
    -0.0000000000000004487f64,
    0.0000000000000002397f64,
    -0.0000000000000001347f64,
    0.0000000000000000801f64,
    -0.0000000000000000501f64,
];
static mut g2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: g2_data.as_ptr() as *mut _,
            order: 33 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 20 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn fg_asymp(
    x: libc::c_double,
    mut f: *mut gsl_sf_result,
    mut g: *mut gsl_sf_result,
) {
    let xbig: libc::c_double = 1.0f64 / 1.4901161193847656e-08f64;
    let xmaxf: libc::c_double = 1.0f64 / 2.2250738585072014e-308f64;
    let xmaxg: libc::c_double = 1.0f64 / 1.4916681462400413e-154f64;
    let xbnd: libc::c_double = 7.07106781187f64;
    let x2: libc::c_double = x * x;
    if x <= xbnd {
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut f1_cs, (1.0f64 / x2 - 0.04125f64) / 0.02125f64, &mut result_c1);
        cheb_eval_e(&mut g1_cs, (1.0f64 / x2 - 0.04125f64) / 0.02125f64, &mut result_c2);
        (*f).val = (1.0f64 + result_c1.val) / x;
        (*g).val = (1.0f64 + result_c2.val) / x2;
        (*f)
            .err = result_c1.err / x
            + 2.0f64 * 2.2204460492503131e-16f64 * fabs((*f).val);
        (*g)
            .err = result_c2.err / x2
            + 2.0f64 * 2.2204460492503131e-16f64 * fabs((*g).val);
    } else if x <= xbig {
        let mut result_c1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c2_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut f2_cs, 100.0f64 / x2 - 1.0f64, &mut result_c1_0);
        cheb_eval_e(&mut g2_cs, 100.0f64 / x2 - 1.0f64, &mut result_c2_0);
        (*f).val = (1.0f64 + result_c1_0.val) / x;
        (*g).val = (1.0f64 + result_c2_0.val) / x2;
        (*f)
            .err = result_c1_0.err / x
            + 2.0f64 * 2.2204460492503131e-16f64 * fabs((*f).val);
        (*g)
            .err = result_c2_0.err / x2
            + 2.0f64 * 2.2204460492503131e-16f64 * fabs((*g).val);
    } else {
        (*f).val = if x < xmaxf { 1.0f64 / x } else { 0.0f64 };
        (*g).val = if x < xmaxg { 1.0f64 / x2 } else { 0.0f64 };
        (*f).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*f).val);
        (*g).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*g).val);
    };
}
static mut si_data: [libc::c_double; 12] = [
    -0.1315646598184841929f64,
    -0.2776578526973601892f64,
    0.0354414054866659180f64,
    -0.0025631631447933978f64,
    0.0001162365390497009f64,
    -0.0000035904327241606f64,
    0.0000000802342123706f64,
    -0.0000000013562997693f64,
    0.0000000000179440722f64,
    -0.0000000000001908387f64,
    0.0000000000000016670f64,
    -0.0000000000000000122f64,
];
static mut si_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: si_data.as_ptr() as *mut _,
            order: 11 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut ci_data: [libc::c_double; 13] = [
    -0.34004281856055363156f64,
    -1.03302166401177456807f64,
    0.19388222659917082877f64,
    -0.01918260436019865894f64,
    0.00110789252584784967f64,
    -0.00004157234558247209f64,
    0.00000109278524300229f64,
    -0.00000002123285954183f64,
    0.00000000031733482164f64,
    -0.00000000000376141548f64,
    0.00000000000003622653f64,
    -0.00000000000000028912f64,
    0.00000000000000000194f64,
];
static mut ci_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ci_data.as_ptr() as *mut _,
            order: 12 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_Si_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut ax: libc::c_double = fabs(x);
    if ax < 1.4901161193847656e-08f64 {
        (*result).val = x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if ax <= 4.0f64 {
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut si_cs, (x * x - 8.0f64) * 0.125f64, &mut result_c);
        (*result).val = x * (0.75f64 + result_c.val);
        (*result).err = ax * result_c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut f: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut g: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        fg_asymp(ax, &mut f, &mut g);
        (*result)
            .val = 0.5f64 * 3.14159265358979323846f64 - f.val * cos(ax)
            - g.val * sin(ax);
        (*result).err = f.err + g.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        if x < 0.0f64 {
            (*result).val = -(*result).val;
        }
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_Ci_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"sinint.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x <= 4.0f64 {
        let lx: libc::c_double = log(x);
        let y: libc::c_double = (x * x - 8.0f64) * 0.125f64;
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut ci_cs, y, &mut result_c);
        (*result).val = lx - 0.5f64 + result_c.val;
        (*result)
            .err = 2.0f64 * 2.2204460492503131e-16f64 * (fabs(lx) + 0.5f64)
            + result_c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut sin_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_sin: libc::c_int = gsl_sf_sin_e(x, &mut sin_result);
        let mut stat_cos: libc::c_int = gsl_sf_cos_e(x, &mut cos_result);
        let mut f: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut g: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        fg_asymp(x, &mut f, &mut g);
        (*result).val = f.val * sin_result.val - g.val * cos_result.val;
        (*result).err = fabs(f.err * sin_result.val);
        (*result).err += fabs(g.err * cos_result.val);
        (*result).err += fabs(f.val * sin_result.err);
        (*result).err += fabs(g.val * cos_result.err);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_sin != GSL_SUCCESS as libc::c_int {
            stat_sin
        } else if stat_cos != GSL_SUCCESS as libc::c_int {
            stat_cos
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_Si(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_Si_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_Si_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"sinint.c\0" as *const u8 as *const libc::c_char,
            396 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_Ci(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_Ci_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_Ci_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"sinint.c\0" as *const u8 as *const libc::c_char,
            401 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
