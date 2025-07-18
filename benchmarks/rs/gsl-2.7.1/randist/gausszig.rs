use ::libc;
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
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
pub struct gsl_rng_type {
    pub name: *const libc::c_char,
    pub max: libc::c_ulong,
    pub min: libc::c_ulong,
    pub size: size_t,
    pub set: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_ulong) -> ()>,
    pub get: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_ulong>,
    pub get_double: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform_int(
    mut r: *const gsl_rng,
    mut n: libc::c_ulong,
) -> libc::c_ulong {
    let mut offset: libc::c_ulong = (*(*r).type_0).min;
    let mut range: libc::c_ulong = ((*(*r).type_0).max).wrapping_sub(offset);
    let mut scale: libc::c_ulong = 0;
    let mut k: libc::c_ulong = 0;
    if n > range || n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"invalid n, either 0 or exceeds maximum value of generator\0" as *const u8
                as *const libc::c_char,
            b"../gsl/gsl_rng.h\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_ulong;
    }
    scale = range.wrapping_div(n);
    loop {
        k = (((*(*r).type_0).get).unwrap()((*r).state))
            .wrapping_sub(offset)
            .wrapping_div(scale);
        if !(k >= n) {
            break;
        }
    }
    return k;
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform(mut r: *const gsl_rng) -> libc::c_double {
    return ((*(*r).type_0).get_double).unwrap()((*r).state);
}
#[inline]
unsafe extern "C" fn gsl_rng_get(mut r: *const gsl_rng) -> libc::c_ulong {
    return ((*(*r).type_0).get).unwrap()((*r).state);
}
static mut ytab: [libc::c_double; 128] = [
    1 as libc::c_int as libc::c_double,
    0.963598623011f64,
    0.936280813353f64,
    0.913041104253f64,
    0.892278506696f64,
    0.873239356919f64,
    0.855496407634f64,
    0.838778928349f64,
    0.822902083699f64,
    0.807732738234f64,
    0.793171045519f64,
    0.779139726505f64,
    0.765577436082f64,
    0.752434456248f64,
    0.739669787677f64,
    0.727249120285f64,
    0.715143377413f64,
    0.703327646455f64,
    0.691780377035f64,
    0.68048276891f64,
    0.669418297233f64,
    0.65857233912f64,
    0.647931876189f64,
    0.637485254896f64,
    0.62722199145f64,
    0.617132611532f64,
    0.607208517467f64,
    0.597441877296f64,
    0.587825531465f64,
    0.578352913803f64,
    0.569017984198f64,
    0.559815170911f64,
    0.550739320877f64,
    0.541785656682f64,
    0.532949739145f64,
    0.524227434628f64,
    0.515614886373f64,
    0.507108489253f64,
    0.498704867478f64,
    0.490400854812f64,
    0.482193476986f64,
    0.47407993601f64,
    0.466057596125f64,
    0.458123971214f64,
    0.450276713467f64,
    0.442513603171f64,
    0.434832539473f64,
    0.427231532022f64,
    0.419708693379f64,
    0.41226223212f64,
    0.404890446548f64,
    0.397591718955f64,
    0.390364510382f64,
    0.383207355816f64,
    0.376118859788f64,
    0.369097692334f64,
    0.362142585282f64,
    0.355252328834f64,
    0.348425768415f64,
    0.341661801776f64,
    0.334959376311f64,
    0.328317486588f64,
    0.321735172063f64,
    0.31521151497f64,
    0.308745638367f64,
    0.302336704338f64,
    0.29598391232f64,
    0.289686497571f64,
    0.283443729739f64,
    0.27725491156f64,
    0.271119377649f64,
    0.265036493387f64,
    0.259005653912f64,
    0.253026283183f64,
    0.247097833139f64,
    0.241219782932f64,
    0.235391638239f64,
    0.229612930649f64,
    0.223883217122f64,
    0.218202079518f64,
    0.212569124201f64,
    0.206983981709f64,
    0.201446306496f64,
    0.195955776745f64,
    0.190512094256f64,
    0.185114984406f64,
    0.179764196185f64,
    0.174459502324f64,
    0.169200699492f64,
    0.1639876086f64,
    0.158820075195f64,
    0.153697969964f64,
    0.148621189348f64,
    0.143589656295f64,
    0.138603321143f64,
    0.133662162669f64,
    0.128766189309f64,
    0.123915440582f64,
    0.119109988745f64,
    0.114349940703f64,
    0.10963544023f64,
    0.104966670533f64,
    0.100343857232f64,
    0.0957672718266f64,
    0.0912372357329f64,
    0.0867541250127f64,
    0.082318375932f64,
    0.0779304915295f64,
    0.0735910494266f64,
    0.0693007111742f64,
    0.065060233529f64,
    0.0608704821745f64,
    0.056732448584f64,
    0.05264727098f64,
    0.0486162607163f64,
    0.0446409359769f64,
    0.0407230655415f64,
    0.0368647267386f64,
    0.0330683839378f64,
    0.0293369977411f64,
    0.0256741818288f64,
    0.0220844372634f64,
    0.0185735200577f64,
    0.0151490552854f64,
    0.0118216532614f64,
    0.00860719483079f64,
    0.00553245272614f64,
    0.00265435214565f64,
];
static mut ktab: [libc::c_ulong; 128] = [
    0 as libc::c_int as libc::c_ulong,
    12590644 as libc::c_int as libc::c_ulong,
    14272653 as libc::c_int as libc::c_ulong,
    14988939 as libc::c_int as libc::c_ulong,
    15384584 as libc::c_int as libc::c_ulong,
    15635009 as libc::c_int as libc::c_ulong,
    15807561 as libc::c_int as libc::c_ulong,
    15933577 as libc::c_int as libc::c_ulong,
    16029594 as libc::c_int as libc::c_ulong,
    16105155 as libc::c_int as libc::c_ulong,
    16166147 as libc::c_int as libc::c_ulong,
    16216399 as libc::c_int as libc::c_ulong,
    16258508 as libc::c_int as libc::c_ulong,
    16294295 as libc::c_int as libc::c_ulong,
    16325078 as libc::c_int as libc::c_ulong,
    16351831 as libc::c_int as libc::c_ulong,
    16375291 as libc::c_int as libc::c_ulong,
    16396026 as libc::c_int as libc::c_ulong,
    16414479 as libc::c_int as libc::c_ulong,
    16431002 as libc::c_int as libc::c_ulong,
    16445880 as libc::c_int as libc::c_ulong,
    16459343 as libc::c_int as libc::c_ulong,
    16471578 as libc::c_int as libc::c_ulong,
    16482744 as libc::c_int as libc::c_ulong,
    16492970 as libc::c_int as libc::c_ulong,
    16502368 as libc::c_int as libc::c_ulong,
    16511031 as libc::c_int as libc::c_ulong,
    16519039 as libc::c_int as libc::c_ulong,
    16526459 as libc::c_int as libc::c_ulong,
    16533352 as libc::c_int as libc::c_ulong,
    16539769 as libc::c_int as libc::c_ulong,
    16545755 as libc::c_int as libc::c_ulong,
    16551348 as libc::c_int as libc::c_ulong,
    16556584 as libc::c_int as libc::c_ulong,
    16561493 as libc::c_int as libc::c_ulong,
    16566101 as libc::c_int as libc::c_ulong,
    16570433 as libc::c_int as libc::c_ulong,
    16574511 as libc::c_int as libc::c_ulong,
    16578353 as libc::c_int as libc::c_ulong,
    16581977 as libc::c_int as libc::c_ulong,
    16585398 as libc::c_int as libc::c_ulong,
    16588629 as libc::c_int as libc::c_ulong,
    16591685 as libc::c_int as libc::c_ulong,
    16594575 as libc::c_int as libc::c_ulong,
    16597311 as libc::c_int as libc::c_ulong,
    16599901 as libc::c_int as libc::c_ulong,
    16602354 as libc::c_int as libc::c_ulong,
    16604679 as libc::c_int as libc::c_ulong,
    16606881 as libc::c_int as libc::c_ulong,
    16608968 as libc::c_int as libc::c_ulong,
    16610945 as libc::c_int as libc::c_ulong,
    16612818 as libc::c_int as libc::c_ulong,
    16614592 as libc::c_int as libc::c_ulong,
    16616272 as libc::c_int as libc::c_ulong,
    16617861 as libc::c_int as libc::c_ulong,
    16619363 as libc::c_int as libc::c_ulong,
    16620782 as libc::c_int as libc::c_ulong,
    16622121 as libc::c_int as libc::c_ulong,
    16623383 as libc::c_int as libc::c_ulong,
    16624570 as libc::c_int as libc::c_ulong,
    16625685 as libc::c_int as libc::c_ulong,
    16626730 as libc::c_int as libc::c_ulong,
    16627708 as libc::c_int as libc::c_ulong,
    16628619 as libc::c_int as libc::c_ulong,
    16629465 as libc::c_int as libc::c_ulong,
    16630248 as libc::c_int as libc::c_ulong,
    16630969 as libc::c_int as libc::c_ulong,
    16631628 as libc::c_int as libc::c_ulong,
    16632228 as libc::c_int as libc::c_ulong,
    16632768 as libc::c_int as libc::c_ulong,
    16633248 as libc::c_int as libc::c_ulong,
    16633671 as libc::c_int as libc::c_ulong,
    16634034 as libc::c_int as libc::c_ulong,
    16634340 as libc::c_int as libc::c_ulong,
    16634586 as libc::c_int as libc::c_ulong,
    16634774 as libc::c_int as libc::c_ulong,
    16634903 as libc::c_int as libc::c_ulong,
    16634972 as libc::c_int as libc::c_ulong,
    16634980 as libc::c_int as libc::c_ulong,
    16634926 as libc::c_int as libc::c_ulong,
    16634810 as libc::c_int as libc::c_ulong,
    16634628 as libc::c_int as libc::c_ulong,
    16634381 as libc::c_int as libc::c_ulong,
    16634066 as libc::c_int as libc::c_ulong,
    16633680 as libc::c_int as libc::c_ulong,
    16633222 as libc::c_int as libc::c_ulong,
    16632688 as libc::c_int as libc::c_ulong,
    16632075 as libc::c_int as libc::c_ulong,
    16631380 as libc::c_int as libc::c_ulong,
    16630598 as libc::c_int as libc::c_ulong,
    16629726 as libc::c_int as libc::c_ulong,
    16628757 as libc::c_int as libc::c_ulong,
    16627686 as libc::c_int as libc::c_ulong,
    16626507 as libc::c_int as libc::c_ulong,
    16625212 as libc::c_int as libc::c_ulong,
    16623794 as libc::c_int as libc::c_ulong,
    16622243 as libc::c_int as libc::c_ulong,
    16620548 as libc::c_int as libc::c_ulong,
    16618698 as libc::c_int as libc::c_ulong,
    16616679 as libc::c_int as libc::c_ulong,
    16614476 as libc::c_int as libc::c_ulong,
    16612071 as libc::c_int as libc::c_ulong,
    16609444 as libc::c_int as libc::c_ulong,
    16606571 as libc::c_int as libc::c_ulong,
    16603425 as libc::c_int as libc::c_ulong,
    16599973 as libc::c_int as libc::c_ulong,
    16596178 as libc::c_int as libc::c_ulong,
    16591995 as libc::c_int as libc::c_ulong,
    16587369 as libc::c_int as libc::c_ulong,
    16582237 as libc::c_int as libc::c_ulong,
    16576520 as libc::c_int as libc::c_ulong,
    16570120 as libc::c_int as libc::c_ulong,
    16562917 as libc::c_int as libc::c_ulong,
    16554758 as libc::c_int as libc::c_ulong,
    16545450 as libc::c_int as libc::c_ulong,
    16534739 as libc::c_int as libc::c_ulong,
    16522287 as libc::c_int as libc::c_ulong,
    16507638 as libc::c_int as libc::c_ulong,
    16490152 as libc::c_int as libc::c_ulong,
    16468907 as libc::c_int as libc::c_ulong,
    16442518 as libc::c_int as libc::c_ulong,
    16408804 as libc::c_int as libc::c_ulong,
    16364095 as libc::c_int as libc::c_ulong,
    16301683 as libc::c_int as libc::c_ulong,
    16207738 as libc::c_int as libc::c_ulong,
    16047994 as libc::c_int as libc::c_ulong,
    15704248 as libc::c_int as libc::c_ulong,
    15472926 as libc::c_int as libc::c_ulong,
];
static mut wtab: [libc::c_double; 128] = [
    1.62318314817e-08f64,
    2.16291505214e-08f64,
    2.54246305087e-08f64,
    2.84579525938e-08f64,
    3.10340022482e-08f64,
    3.33011726243e-08f64,
    3.53439060345e-08f64,
    3.72152672658e-08f64,
    3.8950989572e-08f64,
    4.05763964764e-08f64,
    4.21101548915e-08f64,
    4.35664624904e-08f64,
    4.49563968336e-08f64,
    4.62887864029e-08f64,
    4.75707945735e-08f64,
    4.88083237257e-08f64,
    5.00063025384e-08f64,
    5.11688950428e-08f64,
    5.22996558616e-08f64,
    5.34016475624e-08f64,
    5.44775307871e-08f64,
    5.55296344581e-08f64,
    5.65600111659e-08f64,
    5.75704813695e-08f64,
    5.85626690412e-08f64,
    5.95380306862e-08f64,
    6.04978791776e-08f64,
    6.14434034901e-08f64,
    6.23756851626e-08f64,
    6.32957121259e-08f64,
    6.42043903937e-08f64,
    6.51025540077e-08f64,
    6.59909735447e-08f64,
    6.68703634341e-08f64,
    6.77413882848e-08f64,
    6.8604668381e-08f64,
    6.94607844804e-08f64,
    7.03102820203e-08f64,
    7.11536748229e-08f64,
    7.1991448372e-08f64,
    7.2824062723e-08f64,
    7.36519550992e-08f64,
    7.44755422158e-08f64,
    7.52952223703e-08f64,
    7.61113773308e-08f64,
    7.69243740467e-08f64,
    7.77345662086e-08f64,
    7.85422956743e-08f64,
    7.93478937793e-08f64,
    8.01516825471e-08f64,
    8.09539758128e-08f64,
    8.17550802699e-08f64,
    8.25552964535e-08f64,
    8.33549196661e-08f64,
    8.41542408569e-08f64,
    8.49535474601e-08f64,
    8.57531242006e-08f64,
    8.65532538723e-08f64,
    8.73542180955e-08f64,
    8.8156298059e-08f64,
    8.89597752521e-08f64,
    8.97649321908e-08f64,
    9.05720531451e-08f64,
    9.138142487e-08f64,
    9.21933373471e-08f64,
    9.30080845407e-08f64,
    9.38259651738e-08f64,
    9.46472835298e-08f64,
    9.54723502847e-08f64,
    9.63014833769e-08f64,
    9.71350089201e-08f64,
    9.79732621669e-08f64,
    9.88165885297e-08f64,
    9.96653446693e-08f64,
    1.00519899658e-07f64,
    1.0138063623e-07f64,
    1.02247952126e-07f64,
    1.03122261554e-07f64,
    1.04003996769e-07f64,
    1.04893609795e-07f64,
    1.05791574313e-07f64,
    1.06698387725e-07f64,
    1.07614573423e-07f64,
    1.08540683296e-07f64,
    1.09477300508e-07f64,
    1.1042504257e-07f64,
    1.11384564771e-07f64,
    1.12356564007e-07f64,
    1.13341783071e-07f64,
    1.14341015475e-07f64,
    1.15355110887e-07f64,
    1.16384981291e-07f64,
    1.17431607977e-07f64,
    1.18496049514e-07f64,
    1.19579450872e-07f64,
    1.20683053909e-07f64,
    1.21808209468e-07f64,
    1.2295639141e-07f64,
    1.24129212952e-07f64,
    1.25328445797e-07f64,
    1.26556042658e-07f64,
    1.27814163916e-07f64,
    1.29105209375e-07f64,
    1.30431856341e-07f64,
    1.31797105598e-07f64,
    1.3320433736e-07f64,
    1.34657379914e-07f64,
    1.36160594606e-07f64,
    1.37718982103e-07f64,
    1.39338316679e-07f64,
    1.41025317971e-07f64,
    1.42787873535e-07f64,
    1.44635331499e-07f64,
    1.4657889173e-07f64,
    1.48632138436e-07f64,
    1.50811780719e-07f64,
    1.53138707402e-07f64,
    1.55639532047e-07f64,
    1.58348931426e-07f64,
    1.61313325908e-07f64,
    1.64596952856e-07f64,
    1.68292495203e-07f64,
    1.72541128694e-07f64,
    1.77574279496e-07f64,
    1.83813550477e-07f64,
    1.92166040885e-07f64,
    2.05295471952e-07f64,
    2.22600839893e-07f64,
];
pub unsafe extern "C" fn gsl_ran_gaussian_ziggurat(
    mut r: *const gsl_rng,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_ulong = 0;
    let mut j: libc::c_ulong = 0;
    let mut sign: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let range: libc::c_ulong = ((*(*r).type_0).max).wrapping_sub((*(*r).type_0).min);
    let offset: libc::c_ulong = (*(*r).type_0).min;
    loop {
        if range >= 0xffffffff as libc::c_uint as libc::c_ulong {
            let mut k: libc::c_ulong = (gsl_rng_get(r)).wrapping_sub(offset);
            i = k & 0xff as libc::c_int as libc::c_ulong;
            j = k >> 8 as libc::c_int & 0xffffff as libc::c_int as libc::c_ulong;
        } else if range >= 0xffffff as libc::c_int as libc::c_ulong {
            let mut k1: libc::c_ulong = (gsl_rng_get(r)).wrapping_sub(offset);
            let mut k2: libc::c_ulong = (gsl_rng_get(r)).wrapping_sub(offset);
            i = k1 & 0xff as libc::c_int as libc::c_ulong;
            j = k2 & 0xffffff as libc::c_int as libc::c_ulong;
        } else {
            i = gsl_rng_uniform_int(r, 256 as libc::c_int as libc::c_ulong);
            j = gsl_rng_uniform_int(r, 16777216 as libc::c_int as libc::c_ulong);
        }
        sign = if i & 0x80 as libc::c_int as libc::c_ulong != 0 {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        i &= 0x7f as libc::c_int as libc::c_ulong;
        x = j as libc::c_double * wtab[i as usize];
        if j < ktab[i as usize] {
            break;
        }
        if i < 127 as libc::c_int as libc::c_ulong {
            let mut y0: libc::c_double = 0.;
            let mut y1: libc::c_double = 0.;
            let mut U1: libc::c_double = 0.;
            y0 = ytab[i as usize];
            y1 = ytab[i.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize];
            U1 = gsl_rng_uniform(r);
            y = y1 + (y0 - y1) * U1;
        } else {
            let mut U1_0: libc::c_double = 0.;
            let mut U2: libc::c_double = 0.;
            U1_0 = 1.0f64 - gsl_rng_uniform(r);
            U2 = gsl_rng_uniform(r);
            x = 3.44428647676f64 - log(U1_0) / 3.44428647676f64;
            y = exp(-3.44428647676f64 * (x - 0.5f64 * 3.44428647676f64)) * U2;
        }
        if y < exp(-0.5f64 * x * x) {
            break;
        }
    }
    return sign as libc::c_double * sigma * x;
}
