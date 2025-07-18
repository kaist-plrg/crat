use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_sin_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_cos_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
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
pub type gsl_mode_t = libc::c_uint;
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
unsafe extern "C" fn GSL_MODE_PREC(mut mt: gsl_mode_t) -> libc::c_uint {
    return mt & 7 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn cheb_eval_mode_e(
    mut cs: *const cheb_series,
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut d: libc::c_double = 0.0f64;
    let mut dd: libc::c_double = 0.0f64;
    let mut y: libc::c_double = (2.0f64 * x - (*cs).a - (*cs).b) / ((*cs).b - (*cs).a);
    let mut y2: libc::c_double = 2.0f64 * y;
    let mut eval_order: libc::c_int = 0;
    if GSL_MODE_PREC(mode) == 0 as libc::c_int as libc::c_uint {
        eval_order = (*cs).order;
    } else {
        eval_order = (*cs).order_sp;
    }
    j = eval_order;
    while j >= 1 as libc::c_int {
        let mut temp: libc::c_double = d;
        d = y2 * d - dd + *((*cs).c).offset(j as isize);
        dd = temp;
        j -= 1;
        j;
    }
    (*result).val = y * d - dd + 0.5f64 * *((*cs).c).offset(0 as libc::c_int as isize);
    (*result)
        .err = 2.2204460492503131e-16f64 * fabs((*result).val)
        + fabs(*((*cs).c).offset(eval_order as isize));
    return GSL_SUCCESS as libc::c_int;
}
static mut am21_data: [libc::c_double; 37] = [
    0.0065809191761485f64,
    0.0023675984685722f64,
    0.0001324741670371f64,
    0.0000157600904043f64,
    0.0000027529702663f64,
    0.0000006102679017f64,
    0.0000001595088468f64,
    0.0000000471033947f64,
    0.0000000152933871f64,
    0.0000000053590722f64,
    0.0000000020000910f64,
    0.0000000007872292f64,
    0.0000000003243103f64,
    0.0000000001390106f64,
    0.0000000000617011f64,
    0.0000000000282491f64,
    0.0000000000132979f64,
    0.0000000000064188f64,
    0.0000000000031697f64,
    0.0000000000015981f64,
    0.0000000000008213f64,
    0.0000000000004296f64,
    0.0000000000002284f64,
    0.0000000000001232f64,
    0.0000000000000675f64,
    0.0000000000000374f64,
    0.0000000000000210f64,
    0.0000000000000119f64,
    0.0000000000000068f64,
    0.0000000000000039f64,
    0.0000000000000023f64,
    0.0000000000000013f64,
    0.0000000000000008f64,
    0.0000000000000005f64,
    0.0000000000000003f64,
    0.0000000000000001f64,
    0.0000000000000001f64,
];
static mut am21_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: am21_data.as_ptr() as *mut _,
            order: 36 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 20 as libc::c_int,
        };
        init
    }
};
static mut ath1_data: [libc::c_double; 36] = [
    -0.07125837815669365f64,
    -0.00590471979831451f64,
    -0.00012114544069499f64,
    -0.00000988608542270f64,
    -0.00000138084097352f64,
    -0.00000026142640172f64,
    -0.00000006050432589f64,
    -0.00000001618436223f64,
    -0.00000000483464911f64,
    -0.00000000157655272f64,
    -0.00000000055231518f64,
    -0.00000000020545441f64,
    -0.00000000008043412f64,
    -0.00000000003291252f64,
    -0.00000000001399875f64,
    -0.00000000000616151f64,
    -0.00000000000279614f64,
    -0.00000000000130428f64,
    -0.00000000000062373f64,
    -0.00000000000030512f64,
    -0.00000000000015239f64,
    -0.00000000000007758f64,
    -0.00000000000004020f64,
    -0.00000000000002117f64,
    -0.00000000000001132f64,
    -0.00000000000000614f64,
    -0.00000000000000337f64,
    -0.00000000000000188f64,
    -0.00000000000000105f64,
    -0.00000000000000060f64,
    -0.00000000000000034f64,
    -0.00000000000000020f64,
    -0.00000000000000011f64,
    -0.00000000000000007f64,
    -0.00000000000000004f64,
    -0.00000000000000002f64,
];
static mut ath1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ath1_data.as_ptr() as *mut _,
            order: 35 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 15 as libc::c_int,
        };
        init
    }
};
static mut am22_data: [libc::c_double; 33] = [
    -0.01562844480625341f64,
    0.00778336445239681f64,
    0.00086705777047718f64,
    0.00015696627315611f64,
    0.00003563962571432f64,
    0.00000924598335425f64,
    0.00000262110161850f64,
    0.00000079188221651f64,
    0.00000025104152792f64,
    0.00000008265223206f64,
    0.00000002805711662f64,
    0.00000000976821090f64,
    0.00000000347407923f64,
    0.00000000125828132f64,
    0.00000000046298826f64,
    0.00000000017272825f64,
    0.00000000006523192f64,
    0.00000000002490471f64,
    0.00000000000960156f64,
    0.00000000000373448f64,
    0.00000000000146417f64,
    0.00000000000057826f64,
    0.00000000000022991f64,
    0.00000000000009197f64,
    0.00000000000003700f64,
    0.00000000000001496f64,
    0.00000000000000608f64,
    0.00000000000000248f64,
    0.00000000000000101f64,
    0.00000000000000041f64,
    0.00000000000000017f64,
    0.00000000000000007f64,
    0.00000000000000002f64,
];
static mut am22_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: am22_data.as_ptr() as *mut _,
            order: 32 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 15 as libc::c_int,
        };
        init
    }
};
static mut ath2_data: [libc::c_double; 32] = [
    0.00440527345871877f64,
    -0.03042919452318455f64,
    -0.00138565328377179f64,
    -0.00018044439089549f64,
    -0.00003380847108327f64,
    -0.00000767818353522f64,
    -0.00000196783944371f64,
    -0.00000054837271158f64,
    -0.00000016254615505f64,
    -0.00000005053049981f64,
    -0.00000001631580701f64,
    -0.00000000543420411f64,
    -0.00000000185739855f64,
    -0.00000000064895120f64,
    -0.00000000023105948f64,
    -0.00000000008363282f64,
    -0.00000000003071196f64,
    -0.00000000001142367f64,
    -0.00000000000429811f64,
    -0.00000000000163389f64,
    -0.00000000000062693f64,
    -0.00000000000024260f64,
    -0.00000000000009461f64,
    -0.00000000000003716f64,
    -0.00000000000001469f64,
    -0.00000000000000584f64,
    -0.00000000000000233f64,
    -0.00000000000000093f64,
    -0.00000000000000037f64,
    -0.00000000000000015f64,
    -0.00000000000000006f64,
    -0.00000000000000002f64,
];
static mut ath2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ath2_data.as_ptr() as *mut _,
            order: 31 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 16 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn airy_mod_phase(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut mod_0: *mut gsl_sf_result,
    mut phase: *mut gsl_sf_result,
) -> libc::c_int {
    let mut result_m: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut result_p: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut m: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut sqx: libc::c_double = 0.;
    if x < -2.0f64 {
        let mut z: libc::c_double = 16.0f64 / (x * x * x) + 1.0f64;
        cheb_eval_mode_e(&mut am21_cs, z, mode, &mut result_m);
        cheb_eval_mode_e(&mut ath1_cs, z, mode, &mut result_p);
    } else if x <= -1.0f64 {
        let mut z_0: libc::c_double = (16.0f64 / (x * x * x) + 9.0f64) / 7.0f64;
        cheb_eval_mode_e(&mut am22_cs, z_0, mode, &mut result_m);
        cheb_eval_mode_e(&mut ath2_cs, z_0, mode, &mut result_p);
    } else {
        (*mod_0).val = 0.0f64;
        (*mod_0).err = 0.0f64;
        (*phase).val = 0.0f64;
        (*phase).err = 0.0f64;
        gsl_error(
            b"x is greater than 1.0\0" as *const u8 as *const libc::c_char,
            b"airy.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    m = 0.3125f64 + result_m.val;
    p = -0.625f64 + result_p.val;
    sqx = sqrt(-x);
    (*mod_0).val = sqrt(m / sqx);
    (*mod_0)
        .err = fabs((*mod_0).val)
        * (2.2204460492503131e-16f64 + fabs(result_m.err / result_m.val));
    (*phase).val = 0.78539816339744830962f64 - x * sqx * p;
    (*phase)
        .err = fabs((*phase).val)
        * (2.2204460492503131e-16f64 + fabs(result_p.err / result_p.val));
    return GSL_SUCCESS as libc::c_int;
}
static mut ai_data_f: [libc::c_double; 9] = [
    -0.03797135849666999750f64,
    0.05919188853726363857f64,
    0.00098629280577279975f64,
    0.00000684884381907656f64,
    0.00000002594202596219f64,
    0.00000000006176612774f64,
    0.00000000000010092454f64,
    0.00000000000000012014f64,
    0.00000000000000000010f64,
];
static mut aif_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ai_data_f.as_ptr() as *mut _,
            order: 8 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
static mut ai_data_g: [libc::c_double; 8] = [
    0.01815236558116127f64,
    0.02157256316601076f64,
    0.00025678356987483f64,
    0.00000142652141197f64,
    0.00000000457211492f64,
    0.00000000000952517f64,
    0.00000000000001392f64,
    0.00000000000000001f64,
];
static mut aig_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: ai_data_g.as_ptr() as *mut _,
            order: 7 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 7 as libc::c_int,
        };
        init
    }
};
static mut data_bif: [libc::c_double; 9] = [
    -0.01673021647198664948f64,
    0.10252335834249445610f64,
    0.00170830925073815165f64,
    0.00001186254546774468f64,
    0.00000004493290701779f64,
    0.00000000010698207143f64,
    0.00000000000017480643f64,
    0.00000000000000020810f64,
    0.00000000000000000018f64,
];
static mut bif_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: data_bif.as_ptr() as *mut _,
            order: 8 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
static mut data_big: [libc::c_double; 8] = [
    0.02246622324857452f64,
    0.03736477545301955f64,
    0.00044476218957212f64,
    0.00000247080756363f64,
    0.00000000791913533f64,
    0.00000000001649807f64,
    0.00000000000002411f64,
    0.00000000000000002f64,
];
static mut big_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: data_big.as_ptr() as *mut _,
            order: 7 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 7 as libc::c_int,
        };
        init
    }
};
static mut data_bif2: [libc::c_double; 10] = [
    0.0998457269381604100f64,
    0.4786249778630055380f64,
    0.0251552119604330118f64,
    0.0005820693885232645f64,
    0.0000074997659644377f64,
    0.0000000613460287034f64,
    0.0000000003462753885f64,
    0.0000000000014288910f64,
    0.0000000000000044962f64,
    0.0000000000000000111f64,
];
static mut bif2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: data_bif2.as_ptr() as *mut _,
            order: 9 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut data_big2: [libc::c_double; 10] = [
    0.033305662145514340f64,
    0.161309215123197068f64,
    0.0063190073096134286f64,
    0.0001187904568162517f64,
    0.0000013045345886200f64,
    0.0000000093741259955f64,
    0.0000000000474580188f64,
    0.0000000000001783107f64,
    0.0000000000000005167f64,
    0.0000000000000000011f64,
];
static mut big2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: data_big2.as_ptr() as *mut _,
            order: 9 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut data_aip: [libc::c_double; 36] = [
    -0.0187519297793867540198f64,
    -0.0091443848250055004725f64,
    0.0009010457337825074652f64,
    -0.0001394184127221491507f64,
    0.0000273815815785209370f64,
    -0.0000062750421119959424f64,
    0.0000016064844184831521f64,
    -0.0000004476392158510354f64,
    0.0000001334635874651668f64,
    -0.0000000420735334263215f64,
    0.0000000139021990246364f64,
    -0.0000000047831848068048f64,
    0.0000000017047897907465f64,
    -0.0000000006268389576018f64,
    0.0000000002369824276612f64,
    -0.0000000000918641139267f64,
    0.0000000000364278543037f64,
    -0.0000000000147475551725f64,
    0.0000000000060851006556f64,
    -0.0000000000025552772234f64,
    0.0000000000010906187250f64,
    -0.0000000000004725870319f64,
    0.0000000000002076969064f64,
    -0.0000000000000924976214f64,
    0.0000000000000417096723f64,
    -0.0000000000000190299093f64,
    0.0000000000000087790676f64,
    -0.0000000000000040927557f64,
    0.0000000000000019271068f64,
    -0.0000000000000009160199f64,
    0.0000000000000004393567f64,
    -0.0000000000000002125503f64,
    0.0000000000000001036735f64,
    -0.0000000000000000509642f64,
    0.0000000000000000252377f64,
    -0.0000000000000000125793f64,
];
static mut aip_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: data_aip.as_ptr() as *mut _,
            order: 35 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 17 as libc::c_int,
        };
        init
    }
};
static mut data_bip: [libc::c_double; 24] = [
    -0.08322047477943447f64,
    0.01146118927371174f64,
    0.00042896440718911f64,
    -0.00014906639379950f64,
    -0.00001307659726787f64,
    0.00000632759839610f64,
    -0.00000042226696982f64,
    -0.00000019147186298f64,
    0.00000006453106284f64,
    -0.00000000784485467f64,
    -0.00000000096077216f64,
    0.00000000070004713f64,
    -0.00000000017731789f64,
    0.00000000002272089f64,
    0.00000000000165404f64,
    -0.00000000000185171f64,
    0.00000000000059576f64,
    -0.00000000000012194f64,
    0.00000000000001334f64,
    0.00000000000000172f64,
    -0.00000000000000145f64,
    0.00000000000000049f64,
    -0.00000000000000011f64,
    0.00000000000000001f64,
];
static mut bip_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: data_bip.as_ptr() as *mut _,
            order: 23 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 14 as libc::c_int,
        };
        init
    }
};
static mut data_bip2: [libc::c_double; 29] = [
    -0.113596737585988679f64,
    0.0041381473947881595f64,
    0.0001353470622119332f64,
    0.0000104273166530153f64,
    0.0000013474954767849f64,
    0.0000001696537405438f64,
    -0.0000000100965008656f64,
    -0.0000000167291194937f64,
    -0.0000000045815364485f64,
    0.0000000003736681366f64,
    0.0000000005766930320f64,
    0.0000000000621812650f64,
    -0.0000000000632941202f64,
    -0.0000000000149150479f64,
    0.0000000000078896213f64,
    0.0000000000024960513f64,
    -0.0000000000012130075f64,
    -0.0000000000003740493f64,
    0.0000000000002237727f64,
    0.0000000000000474902f64,
    -0.0000000000000452616f64,
    -0.0000000000000030172f64,
    0.0000000000000091058f64,
    -0.0000000000000009814f64,
    -0.0000000000000016429f64,
    0.0000000000000005533f64,
    0.0000000000000002175f64,
    -0.0000000000000001737f64,
    -0.0000000000000000010f64,
];
static mut bip2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: data_bip2.as_ptr() as *mut _,
            order: 28 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
#[inline]
unsafe extern "C" fn airy_aie(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut sqx: libc::c_double = sqrt(x);
    let mut z: libc::c_double = 2.0f64 / (x * sqx) - 1.0f64;
    let mut y: libc::c_double = sqrt(sqx);
    let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    cheb_eval_mode_e(&mut aip_cs, z, mode, &mut result_c);
    (*result).val = (0.28125f64 + result_c.val) / y;
    (*result).err = result_c.err / y + 2.2204460492503131e-16f64 * fabs((*result).val);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn airy_bie(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ATR: libc::c_double = 8.7506905708484345f64;
    let BTR: libc::c_double = -2.0938363213560543f64;
    if x < 4.0f64 {
        let mut sqx: libc::c_double = sqrt(x);
        let mut z: libc::c_double = ATR / (x * sqx) + BTR;
        let mut y: libc::c_double = sqrt(sqx);
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bip_cs, z, mode, &mut result_c);
        (*result).val = (0.625f64 + result_c.val) / y;
        (*result)
            .err = result_c.err / y + 2.2204460492503131e-16f64 * fabs((*result).val);
    } else {
        let mut sqx_0: libc::c_double = sqrt(x);
        let mut z_0: libc::c_double = 16.0f64 / (x * sqx_0) - 1.0f64;
        let mut y_0: libc::c_double = sqrt(sqx_0);
        let mut result_c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bip2_cs, z_0, mode, &mut result_c_0);
        (*result).val = (0.625f64 + result_c_0.val) / y_0;
        (*result)
            .err = result_c_0.err / y_0
            + 2.2204460492503131e-16f64 * fabs((*result).val);
    }
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_airy_Ai_e(
    x: libc::c_double,
    mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -1.0f64 {
        let mut mod_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut theta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_mp: libc::c_int = airy_mod_phase(x, mode, &mut mod_0, &mut theta);
        let mut stat_cos: libc::c_int = gsl_sf_cos_err_e(
            theta.val,
            theta.err,
            &mut cos_result,
        );
        (*result).val = mod_0.val * cos_result.val;
        (*result)
            .err = fabs(mod_0.val * cos_result.err) + fabs(cos_result.val * mod_0.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_mp != GSL_SUCCESS as libc::c_int {
            stat_mp
        } else if stat_cos != GSL_SUCCESS as libc::c_int {
            stat_cos
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x <= 1.0f64 {
        let z: libc::c_double = x * x * x;
        let mut result_c0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut aif_cs, z, mode, &mut result_c0);
        cheb_eval_mode_e(&mut aig_cs, z, mode, &mut result_c1);
        (*result).val = 0.375f64 + (result_c0.val - x * (0.25f64 + result_c1.val));
        (*result).err = result_c0.err + fabs(x * result_c1.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut x32: libc::c_double = x * sqrt(x);
        let mut s: libc::c_double = exp(-2.0f64 * x32 / 3.0f64);
        let mut result_aie: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_aie: libc::c_int = airy_aie(x, mode, &mut result_aie);
        (*result).val = result_aie.val * s;
        (*result)
            .err = result_aie.err * s + (*result).val * x32 * 2.2204460492503131e-16f64;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        if fabs((*result).val) < 2.2250738585072014e-308f64 {
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"airy.c\0" as *const u8 as *const libc::c_char,
                693 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        }
        return stat_aie;
    };
}
pub unsafe extern "C" fn gsl_sf_airy_Ai_scaled_e(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -1.0f64 {
        let mut mod_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut theta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut cos_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_mp: libc::c_int = airy_mod_phase(x, mode, &mut mod_0, &mut theta);
        let mut stat_cos: libc::c_int = gsl_sf_cos_err_e(
            theta.val,
            theta.err,
            &mut cos_result,
        );
        (*result).val = mod_0.val * cos_result.val;
        (*result)
            .err = fabs(mod_0.val * cos_result.err) + fabs(cos_result.val * mod_0.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_mp != GSL_SUCCESS as libc::c_int {
            stat_mp
        } else if stat_cos != GSL_SUCCESS as libc::c_int {
            stat_cos
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x <= 1.0f64 {
        let z: libc::c_double = x * x * x;
        let mut result_c0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut aif_cs, z, mode, &mut result_c0);
        cheb_eval_mode_e(&mut aig_cs, z, mode, &mut result_c1);
        (*result).val = 0.375f64 + (result_c0.val - x * (0.25f64 + result_c1.val));
        (*result).err = result_c0.err + fabs(x * result_c1.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        if x > 0.0f64 {
            let scale: libc::c_double = exp(2.0f64 / 3.0f64 * sqrt(z));
            (*result).val *= scale;
            (*result).err *= scale;
        }
        return GSL_SUCCESS as libc::c_int;
    } else {
        return airy_aie(x, mode, result)
    };
}
pub unsafe extern "C" fn gsl_sf_airy_Bi_e(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -1.0f64 {
        let mut mod_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut theta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sin_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_mp: libc::c_int = airy_mod_phase(x, mode, &mut mod_0, &mut theta);
        let mut stat_sin: libc::c_int = gsl_sf_sin_err_e(
            theta.val,
            theta.err,
            &mut sin_result,
        );
        (*result).val = mod_0.val * sin_result.val;
        (*result)
            .err = fabs(mod_0.val * sin_result.err) + fabs(sin_result.val * mod_0.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_mp != GSL_SUCCESS as libc::c_int {
            stat_mp
        } else if stat_sin != GSL_SUCCESS as libc::c_int {
            stat_sin
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x < 1.0f64 {
        let z: libc::c_double = x * x * x;
        let mut result_c0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bif_cs, z, mode, &mut result_c0);
        cheb_eval_mode_e(&mut big_cs, z, mode, &mut result_c1);
        (*result).val = 0.625f64 + result_c0.val + x * (0.4375f64 + result_c1.val);
        (*result).err = result_c0.err + fabs(x * result_c1.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 2.0f64 {
        let z_0: libc::c_double = (2.0f64 * x * x * x - 9.0f64) / 7.0f64;
        let mut result_c0_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bif2_cs, z_0, mode, &mut result_c0_0);
        cheb_eval_mode_e(&mut big2_cs, z_0, mode, &mut result_c1_0);
        (*result).val = 1.125f64 + result_c0_0.val + x * (0.625f64 + result_c1_0.val);
        (*result).err = result_c0_0.err + fabs(x * result_c1_0.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let y: libc::c_double = 2.0f64 * x * sqrt(x) / 3.0f64;
        let s: libc::c_double = exp(y);
        if y > 7.0978271289338397e+02f64 - 1.0f64 {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"airy.c\0" as *const u8 as *const libc::c_char,
                780 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else {
            let mut result_bie: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let mut stat_bie: libc::c_int = airy_bie(x, mode, &mut result_bie);
            (*result).val = result_bie.val * s;
            (*result)
                .err = result_bie.err * s
                + fabs(1.5f64 * y * (2.2204460492503131e-16f64 * (*result).val));
            (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
            return stat_bie;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_airy_Bi_scaled_e(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -1.0f64 {
        let mut mod_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut theta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut sin_result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_mp: libc::c_int = airy_mod_phase(x, mode, &mut mod_0, &mut theta);
        let mut stat_sin: libc::c_int = gsl_sf_sin_err_e(
            theta.val,
            theta.err,
            &mut sin_result,
        );
        (*result).val = mod_0.val * sin_result.val;
        (*result)
            .err = fabs(mod_0.val * sin_result.err) + fabs(sin_result.val * mod_0.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_mp != GSL_SUCCESS as libc::c_int {
            stat_mp
        } else if stat_sin != GSL_SUCCESS as libc::c_int {
            stat_sin
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else if x < 1.0f64 {
        let z: libc::c_double = x * x * x;
        let mut result_c0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bif_cs, z, mode, &mut result_c0);
        cheb_eval_mode_e(&mut big_cs, z, mode, &mut result_c1);
        (*result).val = 0.625f64 + result_c0.val + x * (0.4375f64 + result_c1.val);
        (*result).err = result_c0.err + fabs(x * result_c1.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        if x > 0.0f64 {
            let scale: libc::c_double = exp(-2.0f64 / 3.0f64 * sqrt(z));
            (*result).val *= scale;
            (*result).err *= scale;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 2.0f64 {
        let x3: libc::c_double = x * x * x;
        let z_0: libc::c_double = (2.0f64 * x3 - 9.0f64) / 7.0f64;
        let s: libc::c_double = exp(-2.0f64 / 3.0f64 * sqrt(x3));
        let mut result_c0_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bif2_cs, z_0, mode, &mut result_c0_0);
        cheb_eval_mode_e(&mut big2_cs, z_0, mode, &mut result_c1_0);
        (*result)
            .val = s * (1.125f64 + result_c0_0.val + x * (0.625f64 + result_c1_0.val));
        (*result).err = s * (result_c0_0.err + fabs(x * result_c1_0.err));
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        return airy_bie(x, mode, result)
    };
}
pub unsafe extern "C" fn gsl_sf_airy_Ai(
    x: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_Ai_e(x, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_Ai_e(x, mode, &result)\0" as *const u8 as *const libc::c_char,
            b"airy.c\0" as *const u8 as *const libc::c_char,
            851 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_airy_Ai_scaled(
    x: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_Ai_scaled_e(x, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_Ai_scaled_e(x, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"airy.c\0" as *const u8 as *const libc::c_char,
            856 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_airy_Bi(
    x: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_Bi_e(x, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_Bi_e(x, mode, &result)\0" as *const u8 as *const libc::c_char,
            b"airy.c\0" as *const u8 as *const libc::c_char,
            861 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_airy_Bi_scaled(
    x: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_Bi_scaled_e(x, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_Bi_scaled_e(x, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"airy.c\0" as *const u8 as *const libc::c_char,
            866 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
