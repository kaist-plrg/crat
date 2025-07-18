use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
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
unsafe extern "C" fn rescale_error(
    mut err: libc::c_double,
    result_abs: libc::c_double,
    result_asc: libc::c_double,
) -> libc::c_double {
    err = fabs(err);
    if result_asc != 0 as libc::c_int as libc::c_double
        && err != 0 as libc::c_int as libc::c_double
    {
        let mut scale: libc::c_double = pow(
            200 as libc::c_int as libc::c_double * err / result_asc,
            1.5f64,
        );
        if scale < 1 as libc::c_int as libc::c_double {
            err = result_asc * scale;
        } else {
            err = result_asc;
        }
    }
    if result_abs
        > 2.2250738585072014e-308f64
            / (50 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64)
    {
        let mut min_err: libc::c_double = 50 as libc::c_int as libc::c_double
            * 2.2204460492503131e-16f64 * result_abs;
        if min_err > err {
            err = min_err;
        }
    }
    return err;
}
static mut x1: [libc::c_double; 5] = [
    0.973906528517171720077964012084452f64,
    0.865063366688984510732096688423493f64,
    0.679409568299024406234327365114874f64,
    0.433395394129247190799265943165784f64,
    0.148874338981631210884826001129720f64,
];
static mut w10: [libc::c_double; 5] = [
    0.066671344308688137593568809893332f64,
    0.149451349150580593145776339657697f64,
    0.219086362515982043995534934228163f64,
    0.269266719309996355091226921569469f64,
    0.295524224714752870173892994651338f64,
];
static mut x2: [libc::c_double; 5] = [
    0.995657163025808080735527280689003f64,
    0.930157491355708226001207180059508f64,
    0.780817726586416897063717578345042f64,
    0.562757134668604683339000099272694f64,
    0.294392862701460198131126603103866f64,
];
static mut w21a: [libc::c_double; 5] = [
    0.032558162307964727478818972459390f64,
    0.075039674810919952767043140916190f64,
    0.109387158802297641899210590325805f64,
    0.134709217311473325928054001771707f64,
    0.147739104901338491374841515972068f64,
];
static mut w21b: [libc::c_double; 6] = [
    0.011694638867371874278064396062192f64,
    0.054755896574351996031381300244580f64,
    0.093125454583697605535065465083366f64,
    0.123491976262065851077958109831074f64,
    0.142775938577060080797094273138717f64,
    0.149445554002916905664936468389821f64,
];
static mut x3: [libc::c_double; 11] = [
    0.999333360901932081394099323919911f64,
    0.987433402908088869795961478381209f64,
    0.954807934814266299257919200290473f64,
    0.900148695748328293625099494069092f64,
    0.825198314983114150847066732588520f64,
    0.732148388989304982612354848755461f64,
    0.622847970537725238641159120344323f64,
    0.499479574071056499952214885499755f64,
    0.364901661346580768043989548502644f64,
    0.222254919776601296498260928066212f64,
    0.074650617461383322043914435796506f64,
];
static mut w43a: [libc::c_double; 10] = [
    0.016296734289666564924281974617663f64,
    0.037522876120869501461613795898115f64,
    0.054694902058255442147212685465005f64,
    0.067355414609478086075553166302174f64,
    0.073870199632393953432140695251367f64,
    0.005768556059769796184184327908655f64,
    0.027371890593248842081276069289151f64,
    0.046560826910428830743339154433824f64,
    0.061744995201442564496240336030883f64,
    0.071387267268693397768559114425516f64,
];
static mut w43b: [libc::c_double; 12] = [
    0.001844477640212414100389106552965f64,
    0.010798689585891651740465406741293f64,
    0.021895363867795428102523123075149f64,
    0.032597463975345689443882222526137f64,
    0.042163137935191811847627924327955f64,
    0.050741939600184577780189020092084f64,
    0.058379395542619248375475369330206f64,
    0.064746404951445885544689259517511f64,
    0.069566197912356484528633315038405f64,
    0.072824441471833208150939535192842f64,
    0.074507751014175118273571813842889f64,
    0.074722147517403005594425168280423f64,
];
static mut x4: [libc::c_double; 22] = [
    0.999902977262729234490529830591582f64,
    0.997989895986678745427496322365960f64,
    0.992175497860687222808523352251425f64,
    0.981358163572712773571916941623894f64,
    0.965057623858384619128284110607926f64,
    0.943167613133670596816416634507426f64,
    0.915806414685507209591826430720050f64,
    0.883221657771316501372117548744163f64,
    0.845710748462415666605902011504855f64,
    0.803557658035230982788739474980964f64,
    0.757005730685495558328942793432020f64,
    0.706273209787321819824094274740840f64,
    0.651589466501177922534422205016736f64,
    0.593223374057961088875273770349144f64,
    0.531493605970831932285268948562671f64,
    0.466763623042022844871966781659270f64,
    0.399424847859218804732101665817923f64,
    0.329874877106188288265053371824597f64,
    0.258503559202161551802280975429025f64,
    0.185695396568346652015917141167606f64,
    0.111842213179907468172398359241362f64,
    0.037352123394619870814998165437704f64,
];
static mut w87a: [libc::c_double; 21] = [
    0.008148377384149172900002878448190f64,
    0.018761438201562822243935059003794f64,
    0.027347451050052286161582829741283f64,
    0.033677707311637930046581056957588f64,
    0.036935099820427907614589586742499f64,
    0.002884872430211530501334156248695f64,
    0.013685946022712701888950035273128f64,
    0.023280413502888311123409291030404f64,
    0.030872497611713358675466394126442f64,
    0.035693633639418770719351355457044f64,
    0.000915283345202241360843392549948f64,
    0.005399280219300471367738743391053f64,
    0.010947679601118931134327826856808f64,
    0.016298731696787335262665703223280f64,
    0.021081568889203835112433060188190f64,
    0.025370969769253827243467999831710f64,
    0.029189697756475752501446154084920f64,
    0.032373202467202789685788194889595f64,
    0.034783098950365142750781997949596f64,
    0.036412220731351787562801163687577f64,
    0.037253875503047708539592001191226f64,
];
static mut w87b: [libc::c_double; 23] = [
    0.000274145563762072350016527092881f64,
    0.001807124155057942948341311753254f64,
    0.004096869282759164864458070683480f64,
    0.006758290051847378699816577897424f64,
    0.009549957672201646536053581325377f64,
    0.012329447652244853694626639963780f64,
    0.015010447346388952376697286041943f64,
    0.017548967986243191099665352925900f64,
    0.019938037786440888202278192730714f64,
    0.022194935961012286796332102959499f64,
    0.024339147126000805470360647041454f64,
    0.026374505414839207241503786552615f64,
    0.028286910788771200659968002987960f64,
    0.030052581128092695322521110347341f64,
    0.031646751371439929404586051078883f64,
    0.033050413419978503290785944862689f64,
    0.034255099704226061787082821046821f64,
    0.035262412660156681033782717998428f64,
    0.036076989622888701185500318003895f64,
    0.036698604498456094498018047441094f64,
    0.037120549269832576114119958413599f64,
    0.037334228751935040321235449094698f64,
    0.037361073762679023410321241766599f64,
];
pub unsafe extern "C" fn gsl_integration_qng(
    mut f: *const gsl_function,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut epsabs: libc::c_double,
    mut epsrel: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
    mut neval: *mut size_t,
) -> libc::c_int {
    let mut fv1: [libc::c_double; 5] = [0.; 5];
    let mut fv2: [libc::c_double; 5] = [0.; 5];
    let mut fv3: [libc::c_double; 5] = [0.; 5];
    let mut fv4: [libc::c_double; 5] = [0.; 5];
    let mut savfun: [libc::c_double; 21] = [0.; 21];
    let mut res10: libc::c_double = 0.;
    let mut res21: libc::c_double = 0.;
    let mut res43: libc::c_double = 0.;
    let mut res87: libc::c_double = 0.;
    let mut result_kronrod: libc::c_double = 0.;
    let mut err: libc::c_double = 0.;
    let mut resabs: libc::c_double = 0.;
    let mut resasc: libc::c_double = 0.;
    let half_length: libc::c_double = 0.5f64 * (b - a);
    let abs_half_length: libc::c_double = fabs(half_length);
    let center: libc::c_double = 0.5f64 * (b + a);
    let f_center: libc::c_double = (Some(((*f).function).unwrap()))
        .unwrap()(center, (*f).params);
    let mut k: libc::c_int = 0;
    if epsabs <= 0 as libc::c_int as libc::c_double
        && (epsrel < 50 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64
            || epsrel < 0.5e-28f64)
    {
        *result = 0 as libc::c_int as libc::c_double;
        *abserr = 0 as libc::c_int as libc::c_double;
        *neval = 0 as libc::c_int as size_t;
        gsl_error(
            b"tolerance cannot be achieved with given epsabs and epsrel\0" as *const u8
                as *const libc::c_char,
            b"qng.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_EBADTOL as libc::c_int,
        );
        return GSL_EBADTOL as libc::c_int;
    }
    res10 = 0 as libc::c_int as libc::c_double;
    res21 = w21b[5 as libc::c_int as usize] * f_center;
    resabs = w21b[5 as libc::c_int as usize] * fabs(f_center);
    k = 0 as libc::c_int;
    while k < 5 as libc::c_int {
        let abscissa: libc::c_double = half_length * x1[k as usize];
        let fval1: libc::c_double = (Some(((*f).function).unwrap()))
            .unwrap()(center + abscissa, (*f).params);
        let fval2: libc::c_double = (Some(((*f).function).unwrap()))
            .unwrap()(center - abscissa, (*f).params);
        let fval: libc::c_double = fval1 + fval2;
        res10 += w10[k as usize] * fval;
        res21 += w21a[k as usize] * fval;
        resabs += w21a[k as usize] * (fabs(fval1) + fabs(fval2));
        savfun[k as usize] = fval;
        fv1[k as usize] = fval1;
        fv2[k as usize] = fval2;
        k += 1;
        k;
    }
    k = 0 as libc::c_int;
    while k < 5 as libc::c_int {
        let abscissa_0: libc::c_double = half_length * x2[k as usize];
        let fval1_0: libc::c_double = (Some(((*f).function).unwrap()))
            .unwrap()(center + abscissa_0, (*f).params);
        let fval2_0: libc::c_double = (Some(((*f).function).unwrap()))
            .unwrap()(center - abscissa_0, (*f).params);
        let fval_0: libc::c_double = fval1_0 + fval2_0;
        res21 += w21b[k as usize] * fval_0;
        resabs += w21b[k as usize] * (fabs(fval1_0) + fabs(fval2_0));
        savfun[(k + 5 as libc::c_int) as usize] = fval_0;
        fv3[k as usize] = fval1_0;
        fv4[k as usize] = fval2_0;
        k += 1;
        k;
    }
    resabs *= abs_half_length;
    let mean: libc::c_double = 0.5f64 * res21;
    resasc = w21b[5 as libc::c_int as usize] * fabs(f_center - mean);
    k = 0 as libc::c_int;
    while k < 5 as libc::c_int {
        resasc
            += w21a[k as usize]
                * (fabs(fv1[k as usize] - mean) + fabs(fv2[k as usize] - mean))
                + w21b[k as usize]
                    * (fabs(fv3[k as usize] - mean) + fabs(fv4[k as usize] - mean));
        k += 1;
        k;
    }
    resasc *= abs_half_length;
    result_kronrod = res21 * half_length;
    err = rescale_error((res21 - res10) * half_length, resabs, resasc);
    if err < epsabs || err < epsrel * fabs(result_kronrod) {
        *result = result_kronrod;
        *abserr = err;
        *neval = 21 as libc::c_int as size_t;
        return GSL_SUCCESS as libc::c_int;
    }
    res43 = w43b[11 as libc::c_int as usize] * f_center;
    k = 0 as libc::c_int;
    while k < 10 as libc::c_int {
        res43 += savfun[k as usize] * w43a[k as usize];
        k += 1;
        k;
    }
    k = 0 as libc::c_int;
    while k < 11 as libc::c_int {
        let abscissa_1: libc::c_double = half_length * x3[k as usize];
        let fval_1: libc::c_double = (Some(((*f).function).unwrap()))
            .unwrap()(center + abscissa_1, (*f).params)
            + (Some(((*f).function).unwrap()))
                .unwrap()(center - abscissa_1, (*f).params);
        res43 += fval_1 * w43b[k as usize];
        savfun[(k + 10 as libc::c_int) as usize] = fval_1;
        k += 1;
        k;
    }
    result_kronrod = res43 * half_length;
    err = rescale_error((res43 - res21) * half_length, resabs, resasc);
    if err < epsabs || err < epsrel * fabs(result_kronrod) {
        *result = result_kronrod;
        *abserr = err;
        *neval = 43 as libc::c_int as size_t;
        return GSL_SUCCESS as libc::c_int;
    }
    res87 = w87b[22 as libc::c_int as usize] * f_center;
    k = 0 as libc::c_int;
    while k < 21 as libc::c_int {
        res87 += savfun[k as usize] * w87a[k as usize];
        k += 1;
        k;
    }
    k = 0 as libc::c_int;
    while k < 22 as libc::c_int {
        let abscissa_2: libc::c_double = half_length * x4[k as usize];
        res87
            += w87b[k as usize]
                * ((Some(((*f).function).unwrap()))
                    .unwrap()(center + abscissa_2, (*f).params)
                    + (Some(((*f).function).unwrap()))
                        .unwrap()(center - abscissa_2, (*f).params));
        k += 1;
        k;
    }
    result_kronrod = res87 * half_length;
    err = rescale_error((res87 - res43) * half_length, resabs, resasc);
    if err < epsabs || err < epsrel * fabs(result_kronrod) {
        *result = result_kronrod;
        *abserr = err;
        *neval = 87 as libc::c_int as size_t;
        return GSL_SUCCESS as libc::c_int;
    }
    *result = result_kronrod;
    *abserr = err;
    *neval = 87 as libc::c_int as size_t;
    gsl_error(
        b"failed to reach tolerance with highest-order rule\0" as *const u8
            as *const libc::c_char,
        b"qng.c\0" as *const u8 as *const libc::c_char,
        189 as libc::c_int,
        GSL_ETOL as libc::c_int,
    );
    return GSL_ETOL as libc::c_int;
}
