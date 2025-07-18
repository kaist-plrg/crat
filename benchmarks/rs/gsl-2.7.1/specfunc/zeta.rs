use ::libc;
extern "C" {
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_multiply_e(
        x: libc::c_double,
        y: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_exp_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_gamma_e(x: libc::c_double, result: *mut gsl_sf_result) -> libc::c_int;
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
static mut zeta_xlt1_data: [libc::c_double; 14] = [
    1.48018677156931561235192914649f64,
    0.25012062539889426471999938167f64,
    0.00991137502135360774243761467f64,
    -0.00012084759656676410329833091f64,
    -4.7585866367662556504652535281e-06f64,
    2.2229946694466391855561441361e-07f64,
    -2.2237496498030257121309056582e-09f64,
    -1.0173226513229028319420799028e-10f64,
    4.3756643450424558284466248449e-12f64,
    -6.2229632593100551465504090814e-14f64,
    -6.6116201003272207115277520305e-16f64,
    4.9477279533373912324518463830e-17f64,
    -1.0429819093456189719660003522e-18f64,
    6.9925216166580021051464412040e-21f64,
];
static mut zeta_xlt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: zeta_xlt1_data.as_ptr() as *mut _,
            order: 13 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
static mut zeta_xgt1_data: [libc::c_double; 30] = [
    19.3918515726724119415911269006f64,
    9.1525329692510756181581271500f64,
    0.2427897658867379985365270155f64,
    -0.1339000688262027338316641329f64,
    0.0577827064065028595578410202f64,
    -0.0187625983754002298566409700f64,
    0.0039403014258320354840823803f64,
    -0.0000581508273158127963598882f64,
    -0.0003756148907214820704594549f64,
    0.0001892530548109214349092999f64,
    -0.0000549032199695513496115090f64,
    8.7086484008939038610413331863e-6f64,
    6.4609477924811889068410083425e-7f64,
    -9.6749773915059089205835337136e-7f64,
    3.6585400766767257736982342461e-7f64,
    -8.4592516427275164351876072573e-8f64,
    9.9956786144497936572288988883e-9f64,
    1.4260036420951118112457144842e-9f64,
    -1.1761968823382879195380320948e-9f64,
    3.7114575899785204664648987295e-10f64,
    -7.4756855194210961661210215325e-11f64,
    7.8536934209183700456512982968e-12f64,
    9.9827182259685539619810406271e-13f64,
    -7.5276687030192221587850302453e-13f64,
    2.1955026393964279988917878654e-13f64,
    -4.1934859852834647427576319246e-14f64,
    4.6341149635933550715779074274e-15f64,
    2.3742488509048340106830309402e-16f64,
    -2.7276516388124786119323824391e-16f64,
    7.8473570134636044722154797225e-17f64,
];
static mut zeta_xgt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: zeta_xgt1_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 17 as libc::c_int,
        };
        init
    }
};
static mut zetam1_inter_data: [libc::c_double; 24] = [
    -21.7509435653088483422022339374f64,
    -5.63036877698121782876372020472f64,
    0.0528041358684229425504861579635f64,
    -0.0156381809179670789342700883562f64,
    0.00408218474372355881195080781927f64,
    -0.0010264867349474874045036628282f64,
    0.000260469880409886900143834962387f64,
    -0.0000676175847209968878098566819447f64,
    0.0000179284472587833525426660171124f64,
    -4.83238651318556188834107605116e-6f64,
    1.31913788964999288471371329447e-6f64,
    -3.63760500656329972578222188542e-7f64,
    1.01146847513194744989748396574e-7f64,
    -2.83215225141806501619105289509e-8f64,
    7.97733710252021423361012829496e-9f64,
    -2.25850168553956886676250696891e-9f64,
    6.42269392950164306086395744145e-10f64,
    -1.83363861846127284505060843614e-10f64,
    5.25309763895283179960368072104e-11f64,
    -1.50958687042589821074710575446e-11f64,
    4.34997545516049244697776942981e-12f64,
    -1.25597782748190416118082322061e-12f64,
    3.61280740072222650030134104162e-13f64,
    -9.66437239205745207188920348801e-14f64,
];
static mut zetam1_inter_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: zetam1_inter_data.as_ptr() as *mut _,
            order: 22 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
#[inline]
unsafe extern "C" fn riemann_zeta_sgt0(
    mut s: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s < 1.0f64 {
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut zeta_xlt1_cs, 2.0f64 * s - 1.0f64, &mut c);
        (*result).val = c.val / (s - 1.0f64);
        (*result)
            .err = c.err / fabs(s - 1.0f64)
            + 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if s <= 20.0f64 {
        let mut x: libc::c_double = (2.0f64 * s - 21.0f64) / 19.0f64;
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut zeta_xgt1_cs, x, &mut c_0);
        (*result).val = c_0.val / (s - 1.0f64);
        (*result)
            .err = c_0.err / (s - 1.0f64)
            + 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut f2: libc::c_double = 1.0f64 - pow(2.0f64, -s);
        let mut f3: libc::c_double = 1.0f64 - pow(3.0f64, -s);
        let mut f5: libc::c_double = 1.0f64 - pow(5.0f64, -s);
        let mut f7: libc::c_double = 1.0f64 - pow(7.0f64, -s);
        (*result).val = 1.0f64 / (f2 * f3 * f5 * f7);
        (*result).err = 3.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn riemann_zeta1ms_slt0(
    mut s: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s > -19.0f64 {
        let mut x: libc::c_double = (-(19 as libc::c_int) as libc::c_double - 2.0f64 * s)
            / 19.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut zeta_xgt1_cs, x, &mut c);
        (*result).val = c.val / -s;
        (*result).err = c.err / -s + 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut f2: libc::c_double = 1.0f64 - pow(2.0f64, -(1.0f64 - s));
        let mut f3: libc::c_double = 1.0f64 - pow(3.0f64, -(1.0f64 - s));
        let mut f5: libc::c_double = 1.0f64 - pow(5.0f64, -(1.0f64 - s));
        let mut f7: libc::c_double = 1.0f64 - pow(7.0f64, -(1.0f64 - s));
        (*result).val = 1.0f64 / (f2 * f3 * f5 * f7);
        (*result).err = 3.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn riemann_zeta_minus_1_intermediate_s(
    mut s: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut t: libc::c_double = (s - 10.0f64) / 5.0f64;
    let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    cheb_eval_e(&mut zetam1_inter_cs, t, &mut c);
    (*result).val = exp(c.val) + pow(2.0f64, -s);
    (*result).err = (c.err + 2.0f64 * 2.2204460492503131e-16f64) * (*result).val;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn riemann_zeta_minus1_large_s(
    mut s: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let mut a: libc::c_double = pow(2.0f64, -s);
    let mut b: libc::c_double = pow(3.0f64, -s);
    let mut c: libc::c_double = pow(5.0f64, -s);
    let mut d: libc::c_double = pow(7.0f64, -s);
    let mut e: libc::c_double = pow(11.0f64, -s);
    let mut f: libc::c_double = pow(13.0f64, -s);
    let mut t1: libc::c_double = a + b + c + d + e + f;
    let mut t2: libc::c_double = a * (b + c + d + e + f) + b * (c + d + e + f)
        + c * (d + e + f) + d * (e + f) + e * f;
    let mut numt: libc::c_double = t1 - t2;
    let mut zeta: libc::c_double = 1.0f64
        / ((1.0f64 - a) * (1.0f64 - b) * (1.0f64 - c) * (1.0f64 - d) * (1.0f64 - e)
            * (1.0f64 - f));
    (*result).val = numt * zeta;
    (*result)
        .err = (15.0f64 / s + 1.0f64) * 6.0f64 * 2.2204460492503131e-16f64
        * (*result).val;
    return GSL_SUCCESS as libc::c_int;
}
static mut zetam1_pos_int_table: [libc::c_double; 101] = [
    -1.5f64,
    0.0f64,
    0.644934066848226436472415166646f64,
    0.202056903159594285399738161511f64,
    0.082323233711138191516003696541f64,
    0.036927755143369926331365486457f64,
    0.017343061984449139714517929790f64,
    0.008349277381922826839797549849f64,
    0.004077356197944339378685238508f64,
    0.002008392826082214417852769232f64,
    0.000994575127818085337145958900f64,
    0.000494188604119464558702282526f64,
    0.000246086553308048298637998047f64,
    0.000122713347578489146751836526f64,
    0.000061248135058704829258545105f64,
    0.000030588236307020493551728510f64,
    0.000015282259408651871732571487f64,
    7.6371976378997622736002935630e-6f64,
    3.8172932649998398564616446219e-6f64,
    1.9082127165539389256569577951e-6f64,
    9.5396203387279611315203868344e-7f64,
    4.7693298678780646311671960437e-7f64,
    2.3845050272773299000364818675e-7f64,
    1.1921992596531107306778871888e-7f64,
    5.9608189051259479612440207935e-8f64,
    2.9803503514652280186063705069e-8f64,
    1.4901554828365041234658506630e-8f64,
    7.4507117898354294919810041706e-9f64,
    3.7253340247884570548192040184e-9f64,
    1.8626597235130490064039099454e-9f64,
    9.3132743241966818287176473502e-10f64,
    4.6566290650337840729892332512e-10f64,
    2.3283118336765054920014559759e-10f64,
    1.1641550172700519775929738354e-10f64,
    5.8207720879027008892436859891e-11f64,
    2.9103850444970996869294252278e-11f64,
    1.4551921891041984235929632245e-11f64,
    7.2759598350574810145208690123e-12f64,
    3.6379795473786511902372363558e-12f64,
    1.8189896503070659475848321007e-12f64,
    9.0949478402638892825331183869e-13f64,
    4.5474737830421540267991120294e-13f64,
    2.2737368458246525152268215779e-13f64,
    1.1368684076802278493491048380e-13f64,
    5.6843419876275856092771829675e-14f64,
    2.8421709768893018554550737049e-14f64,
    1.4210854828031606769834307141e-14f64,
    7.1054273952108527128773544799e-15f64,
    3.5527136913371136732984695340e-15f64,
    1.7763568435791203274733490144e-15f64,
    8.8817842109308159030960913863e-16f64,
    4.4408921031438133641977709402e-16f64,
    2.2204460507980419839993200942e-16f64,
    1.1102230251410661337205445699e-16f64,
    5.5511151248454812437237365905e-17f64,
    2.7755575621361241725816324538e-17f64,
    1.3877787809725232762839094906e-17f64,
    6.9388939045441536974460853262e-18f64,
    3.4694469521659226247442714961e-18f64,
    1.7347234760475765720489729699e-18f64,
    8.6736173801199337283420550673e-19f64,
    4.3368086900206504874970235659e-19f64,
    2.1684043449972197850139101683e-19f64,
    1.0842021724942414063012711165e-19f64,
    5.4210108624566454109187004043e-20f64,
    2.7105054312234688319546213119e-20f64,
    1.3552527156101164581485233996e-20f64,
    6.7762635780451890979952987415e-21f64,
    3.3881317890207968180857031004e-21f64,
    1.6940658945097991654064927471e-21f64,
    8.4703294725469983482469926091e-22f64,
    4.2351647362728333478622704833e-22f64,
    2.1175823681361947318442094398e-22f64,
    1.0587911840680233852265001539e-22f64,
    5.2939559203398703238139123029e-23f64,
    2.6469779601698529611341166842e-23f64,
    1.3234889800848990803094510250e-23f64,
    6.6174449004244040673552453323e-24f64,
    3.3087224502121715889469563843e-24f64,
    1.6543612251060756462299236771e-24f64,
    8.2718061255303444036711056167e-25f64,
    4.1359030627651609260093824555e-25f64,
    2.0679515313825767043959679193e-25f64,
    1.0339757656912870993284095591e-25f64,
    5.1698788284564313204101332166e-26f64,
    2.5849394142282142681277617708e-26f64,
    1.2924697071141066700381126118e-26f64,
    6.4623485355705318034380021611e-27f64,
    3.2311742677852653861348141180e-27f64,
    1.6155871338926325212060114057e-27f64,
    8.0779356694631620331587381863e-28f64,
    4.0389678347315808256222628129e-28f64,
    2.0194839173657903491587626465e-28f64,
    1.0097419586828951533619250700e-28f64,
    5.0487097934144756960847711725e-29f64,
    2.5243548967072378244674341938e-29f64,
    1.2621774483536189043753999660e-29f64,
    6.3108872417680944956826093943e-30f64,
    3.1554436208840472391098412184e-30f64,
    1.5777218104420236166444327830e-30f64,
    7.8886090522101180735205378276e-31f64,
];
static mut zeta_neg_int_table: [libc::c_double; 50] = [
    -0.083333333333333333333333333333f64,
    0.008333333333333333333333333333f64,
    -0.003968253968253968253968253968f64,
    0.004166666666666666666666666667f64,
    -0.007575757575757575757575757576f64,
    0.021092796092796092796092796093f64,
    -0.083333333333333333333333333333f64,
    0.44325980392156862745098039216f64,
    -3.05395433027011974380395433027f64,
    26.4562121212121212121212121212f64,
    -281.460144927536231884057971014f64,
    3607.5105463980463980463980464f64,
    -54827.583333333333333333333333f64,
    974936.82385057471264367816092f64,
    -2.0052695796688078946143462272e+07f64,
    4.7238486772162990196078431373e+08f64,
    -1.2635724795916666666666666667e+10f64,
    3.8087931125245368811553022079e+11f64,
    -1.2850850499305083333333333333e+13f64,
    4.8241448354850170371581670362e+14f64,
    -2.0040310656516252738108421663e+16f64,
    9.1677436031953307756992753623e+17f64,
    -4.5979888343656503490437943262e+19f64,
    2.5180471921451095697089023320e+21f64,
    -1.5001733492153928733711440151e+23f64,
    9.6899578874635940656497942895e+24f64,
    -6.7645882379292820990945242302e+26f64,
    5.0890659468662289689766332916e+28f64,
    -4.1147288792557978697665486068e+30f64,
    3.5666582095375556109684574609e+32f64,
    -3.3066089876577576725680214670e+34f64,
    3.2715634236478716264211227016e+36f64,
    -3.4473782558278053878256455080e+38f64,
    3.8614279832705258893092720200e+40f64,
    -4.5892974432454332168863989006e+42f64,
    5.7775386342770431824884825688e+44f64,
    -7.6919858759507135167410075972e+46f64,
    1.0813635449971654696354033351e+49f64,
    -1.6029364522008965406067102346e+51f64,
    2.5019479041560462843656661499e+53f64,
    -4.1067052335810212479752045004e+55f64,
    7.0798774408494580617452972433e+57f64,
    -1.2804546887939508790190849756e+60f64,
    2.4267340392333524078020892067e+62f64,
    -4.8143218874045769355129570066e+64f64,
    9.9875574175727530680652777408e+66f64,
    -2.1645634868435185631335136160e+69f64,
    4.8962327039620553206849224516e+71f64,
    -1.1549023923963519663954271692e+74f64,
    2.8382249570693706959264156336e+76f64,
];
static mut hzeta_c: [libc::c_double; 15] = [
    1.00000000000000000000000000000f64,
    0.083333333333333333333333333333f64,
    -0.00138888888888888888888888888889f64,
    0.000033068783068783068783068783069f64,
    -8.2671957671957671957671957672e-07f64,
    2.0876756987868098979210090321e-08f64,
    -5.2841901386874931848476822022e-10f64,
    1.3382536530684678832826980975e-11f64,
    -3.3896802963225828668301953912e-13f64,
    8.5860620562778445641359054504e-15f64,
    -2.1748686985580618730415164239e-16f64,
    5.5090028283602295152026526089e-18f64,
    -1.3954464685812523340707686264e-19f64,
    3.5347070396294674716932299778e-21f64,
    -8.9535174270375468504026113181e-23f64,
];
static mut eta_pos_int_table: [libc::c_double; 101] = [
    0.50000000000000000000000000000f64,
    0.69314718055994530942f64,
    0.82246703342411321823620758332f64,
    0.90154267736969571404980362113f64,
    0.94703282949724591757650323447f64,
    0.97211977044690930593565514355f64,
    0.98555109129743510409843924448f64,
    0.99259381992283028267042571313f64,
    0.99623300185264789922728926008f64,
    0.99809429754160533076778303185f64,
    0.99903950759827156563922184570f64,
    0.99951714349806075414409417483f64,
    0.99975768514385819085317967871f64,
    0.99987854276326511549217499282f64,
    0.99993917034597971817095419226f64,
    0.99996955121309923808263293263f64,
    0.99998476421490610644168277496f64,
    0.99999237829204101197693787224f64,
    0.99999618786961011347968922641f64,
    0.99999809350817167510685649297f64,
    0.99999904661158152211505084256f64,
    0.99999952325821554281631666433f64,
    0.99999976161323082254789720494f64,
    0.99999988080131843950322382485f64,
    0.99999994039889239462836140314f64,
    0.99999997019885696283441513311f64,
    0.99999998509923199656878766181f64,
    0.99999999254955048496351585274f64,
    0.99999999627475340010872752767f64,
    0.99999999813736941811218674656f64,
    0.99999999906868228145397862728f64,
    0.99999999953434033145421751469f64,
    0.99999999976716989595149082282f64,
    0.99999999988358485804603047265f64,
    0.99999999994179239904531592388f64,
    0.99999999997089618952980952258f64,
    0.99999999998544809143388476396f64,
    0.99999999999272404460658475006f64,
    0.99999999999636202193316875550f64,
    0.99999999999818101084320873555f64,
    0.99999999999909050538047887809f64,
    0.99999999999954525267653087357f64,
    0.99999999999977262633369589773f64,
    0.99999999999988631316532476488f64,
    0.99999999999994315658215465336f64,
    0.99999999999997157829090808339f64,
    0.99999999999998578914539762720f64,
    0.99999999999999289457268000875f64,
    0.99999999999999644728633373609f64,
    0.99999999999999822364316477861f64,
    0.99999999999999911182158169283f64,
    0.99999999999999955591079061426f64,
    0.99999999999999977795539522974f64,
    0.99999999999999988897769758908f64,
    0.99999999999999994448884878594f64,
    0.99999999999999997224442439010f64,
    0.99999999999999998612221219410f64,
    0.99999999999999999306110609673f64,
    0.99999999999999999653055304826f64,
    0.99999999999999999826527652409f64,
    0.99999999999999999913263826204f64,
    0.99999999999999999956631913101f64,
    0.99999999999999999978315956551f64,
    0.99999999999999999989157978275f64,
    0.99999999999999999994578989138f64,
    0.99999999999999999997289494569f64,
    0.99999999999999999998644747284f64,
    0.99999999999999999999322373642f64,
    0.99999999999999999999661186821f64,
    0.99999999999999999999830593411f64,
    0.99999999999999999999915296705f64,
    0.99999999999999999999957648353f64,
    0.99999999999999999999978824176f64,
    0.99999999999999999999989412088f64,
    0.99999999999999999999994706044f64,
    0.99999999999999999999997353022f64,
    0.99999999999999999999998676511f64,
    0.99999999999999999999999338256f64,
    0.99999999999999999999999669128f64,
    0.99999999999999999999999834564f64,
    0.99999999999999999999999917282f64,
    0.99999999999999999999999958641f64,
    0.99999999999999999999999979320f64,
    0.99999999999999999999999989660f64,
    0.99999999999999999999999994830f64,
    0.99999999999999999999999997415f64,
    0.99999999999999999999999998708f64,
    0.99999999999999999999999999354f64,
    0.99999999999999999999999999677f64,
    0.99999999999999999999999999838f64,
    0.99999999999999999999999999919f64,
    0.99999999999999999999999999960f64,
    0.99999999999999999999999999980f64,
    0.99999999999999999999999999990f64,
    0.99999999999999999999999999995f64,
    0.99999999999999999999999999997f64,
    0.99999999999999999999999999999f64,
    0.99999999999999999999999999999f64,
    1.00000000000000000000000000000f64,
    1.00000000000000000000000000000f64,
    1.00000000000000000000000000000f64,
];
static mut eta_neg_int_table: [libc::c_double; 50] = [
    0.25000000000000000000000000000f64,
    -0.12500000000000000000000000000f64,
    0.25000000000000000000000000000f64,
    -1.06250000000000000000000000000f64,
    7.75000000000000000000000000000f64,
    -86.3750000000000000000000000000f64,
    1365.25000000000000000000000000f64,
    -29049.0312500000000000000000000f64,
    800572.750000000000000000000000f64,
    -2.7741322625000000000000000000e+7f64,
    1.1805291302500000000000000000e+9f64,
    -6.0523980051687500000000000000e+10f64,
    3.6794167785377500000000000000e+12f64,
    -2.6170760990658387500000000000e+14f64,
    2.1531418140800295250000000000e+16f64,
    -2.0288775575173015930156250000e+18f64,
    2.1708009902623770590275000000e+20f64,
    -2.6173826968455814932120125000e+22f64,
    3.5324148876863877826668602500e+24f64,
    -5.3042033406864906641493838981e+26f64,
    8.8138218364311576767253114668e+28f64,
    -1.6128065107490778547354654864e+31f64,
    3.2355470001722734208527794569e+33f64,
    -7.0876727476537493198506645215e+35f64,
    1.6890450341293965779175629389e+38f64,
    -4.3639690731216831157655651358e+40f64,
    1.2185998827061261322605065672e+43f64,
    -3.6670584803153006180101262324e+45f64,
    1.1859898526302099104271449748e+48f64,
    -4.1120769493584015047981746438e+50f64,
    1.5249042436787620309090168687e+53f64,
    -6.0349693196941307074572991901e+55f64,
    2.5437161764210695823197691519e+58f64,
    -1.1396923802632287851130360170e+61f64,
    5.4180861064753979196802726455e+63f64,
    -2.7283654799994373847287197104e+66f64,
    1.4529750514918543238511171663e+69f64,
    -8.1705519371067450079777183386e+71f64,
    4.8445781606678367790247757259e+74f64,
    -3.0246694206649519336179448018e+77f64,
    1.9858807961690493054169047970e+80f64,
    -1.3694474620720086994386818232e+83f64,
    9.9070382984295807826303785989e+85f64,
    -7.5103780796592645925968460677e+88f64,
    5.9598418264260880840077992227e+91f64,
    -4.9455988887500020399263196307e+94f64,
    4.2873596927020241277675775935e+97f64,
    -3.8791952037716162900707994047e+100f64,
    3.6600317773156342245401829308e+103f64,
    -3.5978775704117283875784869570e+106f64,
];
pub unsafe extern "C" fn gsl_sf_hzeta_e(
    s: libc::c_double,
    q: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s <= 1.0f64 || q <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            719 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let max_bits: libc::c_double = 54.0f64;
        let ln_term0: libc::c_double = -s * log(q);
        if ln_term0 < -7.0839641853226408e+02f64 + 1.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 2.2250738585072014e-308f64;
            gsl_error(
                b"underflow\0" as *const u8 as *const libc::c_char,
                b"zeta.c\0" as *const u8 as *const libc::c_char,
                726 as libc::c_int,
                GSL_EUNDRFLW as libc::c_int,
            );
            return GSL_EUNDRFLW as libc::c_int;
        } else if ln_term0 > 7.0978271289338397e+02f64 - 1.0f64 {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"zeta.c\0" as *const u8 as *const libc::c_char,
                729 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        } else if s > max_bits && q < 1.0f64 || s > 0.5f64 * max_bits && q < 0.25f64 {
            (*result).val = pow(q, -s);
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else if s > 0.5f64 * max_bits && q < 1.0f64 {
            let p1: libc::c_double = pow(q, -s);
            let p2: libc::c_double = pow(q / (1.0f64 + q), s);
            let p3: libc::c_double = pow(q / (2.0f64 + q), s);
            (*result).val = p1 * (1.0f64 + p2 + p3);
            (*result)
                .err = 2.2204460492503131e-16f64 * (0.5f64 * s + 2.0f64)
                * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else {
            let jmax: libc::c_int = 12 as libc::c_int;
            let kmax: libc::c_int = 10 as libc::c_int;
            let mut j: libc::c_int = 0;
            let mut k: libc::c_int = 0;
            let pmax: libc::c_double = pow(kmax as libc::c_double + q, -s);
            let mut scp: libc::c_double = s;
            let mut pcp: libc::c_double = pmax / (kmax as libc::c_double + q);
            let mut ans: libc::c_double = pmax
                * ((kmax as libc::c_double + q) / (s - 1.0f64) + 0.5f64);
            k = 0 as libc::c_int;
            while k < kmax {
                ans += pow(k as libc::c_double + q, -s);
                k += 1;
                k;
            }
            j = 0 as libc::c_int;
            while j <= jmax {
                let mut delta: libc::c_double = hzeta_c[(j + 1 as libc::c_int) as usize]
                    * scp * pcp;
                ans += delta;
                if fabs(delta / ans) < 0.5f64 * 2.2204460492503131e-16f64 {
                    break;
                }
                scp
                    *= (s + (2 as libc::c_int * j) as libc::c_double
                        + 1 as libc::c_int as libc::c_double)
                        * (s + (2 as libc::c_int * j) as libc::c_double
                            + 2 as libc::c_int as libc::c_double);
                pcp /= (kmax as libc::c_double + q) * (kmax as libc::c_double + q);
                j += 1;
                j;
            }
            (*result).val = ans;
            (*result)
                .err = 2.0f64 * (jmax as libc::c_double + 1.0f64)
                * 2.2204460492503131e-16f64 * fabs(ans);
            return GSL_SUCCESS as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_zeta_e(
    s: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s == 1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            781 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if s >= 0.0f64 {
        return riemann_zeta_sgt0(s, result)
    } else {
        let mut zeta_one_minus_s: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_zoms: libc::c_int = riemann_zeta1ms_slt0(s, &mut zeta_one_minus_s);
        let sin_term: libc::c_double = if fmod(s, 2.0f64) == 0.0f64 {
            0.0f64
        } else {
            sin(0.5f64 * 3.14159265358979323846f64 * fmod(s, 4.0f64))
                / 3.14159265358979323846f64
        };
        if sin_term == 0.0f64 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else if s > -(170 as libc::c_int) as libc::c_double {
            let twopi_pow: [libc::c_double; 18] = [
                1.0f64,
                9.589560061550901348e+007f64,
                9.195966217409212684e+015f64,
                8.818527036583869903e+023f64,
                8.456579467173150313e+031f64,
                8.109487671573504384e+039f64,
                7.776641909496069036e+047f64,
                7.457457466828644277e+055f64,
                7.151373628461452286e+063f64,
                6.857852693272229709e+071f64,
                6.576379029540265771e+079f64,
                6.306458169130020789e+087f64,
                6.047615938853066678e+095f64,
                5.799397627482402614e+103f64,
                5.561367186955830005e+111f64,
                5.333106466365131227e+119f64,
                5.114214477385391780e+127f64,
                4.904306689854036836e+135f64,
            ];
            let n: libc::c_int = floor(-s / 10.0f64) as libc::c_int;
            let fs: libc::c_double = s + 10.0f64 * n as libc::c_double;
            let p: libc::c_double = pow(2.0f64 * 3.14159265358979323846f64, fs)
                / twopi_pow[n as usize];
            let mut g: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
            let stat_g: libc::c_int = gsl_sf_gamma_e(1.0f64 - s, &mut g);
            (*result).val = p * g.val * sin_term * zeta_one_minus_s.val;
            (*result).err = fabs(p * g.val * sin_term) * zeta_one_minus_s.err;
            (*result).err += fabs(p * sin_term * zeta_one_minus_s.val) * g.err;
            (*result).err
                += 2.2204460492503131e-16f64 * (fabs(s) + 2.0f64) * fabs((*result).val);
            return if stat_g != GSL_SUCCESS as libc::c_int {
                stat_g
            } else if stat_zoms != GSL_SUCCESS as libc::c_int {
                stat_zoms
            } else {
                GSL_SUCCESS as libc::c_int
            };
        } else {
            (*result).val = ::std::f32::INFINITY as libc::c_double;
            (*result).err = ::std::f32::INFINITY as libc::c_double;
            gsl_error(
                b"overflow\0" as *const u8 as *const libc::c_char,
                b"zeta.c\0" as *const u8 as *const libc::c_char,
                847 as libc::c_int,
                GSL_EOVRFLW as libc::c_int,
            );
            return GSL_EOVRFLW as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_zeta_int_e(
    n: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        if n & 1 as libc::c_int == 0 {
            (*result).val = 0.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else if n > -(99 as libc::c_int) {
            (*result)
                .val = zeta_neg_int_table[(-(n + 1 as libc::c_int) / 2 as libc::c_int)
                as usize];
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else {
            return gsl_sf_zeta_e(n as libc::c_double, result)
        }
    } else if n == 1 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            873 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n <= 100 as libc::c_int {
        (*result).val = 1.0f64 + zetam1_pos_int_table[n as usize];
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 1.0f64;
        (*result).err = 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_zetam1_e(
    s: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s <= 5.0f64 {
        let mut stat: libc::c_int = gsl_sf_zeta_e(s, result);
        (*result).val = (*result).val - 1.0f64;
        return stat;
    } else if s < 15.0f64 {
        return riemann_zeta_minus_1_intermediate_s(s, result)
    } else {
        return riemann_zeta_minus1_large_s(s, result)
    };
}
pub unsafe extern "C" fn gsl_sf_zetam1_int_e(
    n: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n < 0 as libc::c_int {
        if n & 1 as libc::c_int == 0 {
            (*result).val = -1.0f64;
            (*result).err = 0.0f64;
            return GSL_SUCCESS as libc::c_int;
        } else if n > -(99 as libc::c_int) {
            (*result)
                .val = zeta_neg_int_table[(-(n + 1 as libc::c_int) / 2 as libc::c_int)
                as usize] - 1.0f64;
            (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else {
            return gsl_sf_zeta_e(n as libc::c_double, result)
        }
    } else if n == 1 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            927 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n <= 100 as libc::c_int {
        (*result).val = zetam1_pos_int_table[n as usize];
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        return gsl_sf_zetam1_e(n as libc::c_double, result)
    };
}
pub unsafe extern "C" fn gsl_sf_eta_int_e(
    mut n: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n > 100 as libc::c_int {
        (*result).val = 1.0f64;
        (*result).err = 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n >= 0 as libc::c_int {
        (*result).val = eta_pos_int_table[n as usize];
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if n & 1 as libc::c_int == 0 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if n > -(99 as libc::c_int) {
        (*result)
            .val = eta_neg_int_table[(-(n + 1 as libc::c_int) / 2 as libc::c_int)
            as usize];
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut z: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut p: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_z: libc::c_int = gsl_sf_zeta_int_e(n, &mut z);
        let mut stat_p: libc::c_int = gsl_sf_exp_e(
            (1.0f64 - n as libc::c_double) * 0.69314718055994530942f64,
            &mut p,
        );
        let mut stat_m: libc::c_int = gsl_sf_multiply_e(-p.val, z.val, result);
        (*result)
            .err = fabs(
            p.err * (0.69314718055994530942f64 * (1.0f64 - n as libc::c_double)) * z.val,
        ) + z.err * fabs(p.val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_m != GSL_SUCCESS as libc::c_int {
            stat_m
        } else if stat_p != GSL_SUCCESS as libc::c_int {
            stat_p
        } else if stat_z != GSL_SUCCESS as libc::c_int {
            stat_z
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_eta_e(
    s: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s > 100.0f64 {
        (*result).val = 1.0f64;
        (*result).err = 2.2204460492503131e-16f64;
        return GSL_SUCCESS as libc::c_int;
    } else if fabs(s - 1.0f64) < 10.0f64 * 7.4009597974140505e-04f64 {
        let mut del: libc::c_double = s - 1.0f64;
        let mut c0: libc::c_double = 0.69314718055994530942f64;
        let mut c1: libc::c_double = 0.69314718055994530942f64
            * (0.57721566490153286060651209008f64 - 0.5f64 * 0.69314718055994530942f64);
        let mut c2: libc::c_double = -0.0326862962794492996f64;
        let mut c3: libc::c_double = 0.0015689917054155150f64;
        let mut c4: libc::c_double = 0.00074987242112047532f64;
        (*result).val = c0 + del * (c1 + del * (c2 + del * (c3 + del * c4)));
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut z: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut p: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_z: libc::c_int = gsl_sf_zeta_e(s, &mut z);
        let mut stat_p: libc::c_int = gsl_sf_exp_e(
            (1.0f64 - s) * 0.69314718055994530942f64,
            &mut p,
        );
        let mut stat_m: libc::c_int = gsl_sf_multiply_e(1.0f64 - p.val, z.val, result);
        (*result)
            .err = fabs(p.err * (0.69314718055994530942f64 * (1.0f64 - s)) * z.val)
            + z.err * fabs(p.val);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_m != GSL_SUCCESS as libc::c_int {
            stat_m
        } else if stat_p != GSL_SUCCESS as libc::c_int {
            stat_p
        } else if stat_z != GSL_SUCCESS as libc::c_int {
            stat_z
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_zeta(s: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_zeta_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_zeta_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            1019 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_hzeta(
    s: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_hzeta_e(s, a, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_hzeta_e(s, a, &result)\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            1024 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_zeta_int(s: libc::c_int) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_zeta_int_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_zeta_int_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            1029 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_zetam1(s: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_zetam1_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_zetam1_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            1034 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_zetam1_int(s: libc::c_int) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_zetam1_int_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_zetam1_int_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            1039 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_eta_int(s: libc::c_int) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_eta_int_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_eta_int_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            1044 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_eta(s: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_eta_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_eta_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"zeta.c\0" as *const u8 as *const libc::c_char,
            1049 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
