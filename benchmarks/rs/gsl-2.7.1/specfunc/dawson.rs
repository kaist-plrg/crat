use ::libc;
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
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
static mut daw_data: [libc::c_double; 21] = [
    -0.6351734375145949201065127736293e-02f64,
    -0.2294071479677386939899824125866e+00f64,
    0.2213050093908476441683979161786e-01f64,
    -0.1549265453892985046743057753375e-02f64,
    0.8497327715684917456777542948066e-04f64,
    -0.3828266270972014924994099521309e-05f64,
    0.1462854806250163197757148949539e-06f64,
    -0.4851982381825991798846715425114e-08f64,
    0.1421463577759139790347568183304e-09f64,
    -0.3728836087920596525335493054088e-11f64,
    0.8854942961778203370194565231369e-13f64,
    -0.1920757131350206355421648417493e-14f64,
    0.3834325867246327588241074439253e-16f64,
    -0.7089154168175881633584099327999e-18f64,
    0.1220552135889457674416901120000e-19f64,
    -0.1966204826605348760299451733333e-21f64,
    0.2975845541376597189113173333333e-23f64,
    -0.4247069514800596951039999999999e-25f64,
    0.5734270767391742798506666666666e-27f64,
    -0.7345836823178450261333333333333e-29f64,
    0.8951937667516552533333333333333e-31f64,
];
static mut daw_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: daw_data.as_ptr() as *mut _,
            order: 15 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut daw2_data: [libc::c_double; 45] = [
    -0.56886544105215527114160533733674e-01f64,
    -0.31811346996168131279322878048822e+00f64,
    0.20873845413642236789741580198858e+00f64,
    -0.12475409913779131214073498314784e+00f64,
    0.67869305186676777092847516423676e-01f64,
    -0.33659144895270939503068230966587e-01f64,
    0.15260781271987971743682460381640e-01f64,
    -0.63483709625962148230586094788535e-02f64,
    0.24326740920748520596865966109343e-02f64,
    -0.86219541491065032038526983549637e-03f64,
    0.28376573336321625302857636538295e-03f64,
    -0.87057549874170423699396581464335e-04f64,
    0.24986849985481658331800044137276e-04f64,
    -0.67319286764160294344603050339520e-05f64,
    0.17078578785573543710504524047844e-05f64,
    -0.40917551226475381271896592490038e-06f64,
    0.92828292216755773260751785312273e-07f64,
    -0.19991403610147617829845096332198e-07f64,
    0.40963490644082195241210487868917e-08f64,
    -0.80032409540993168075706781753561e-09f64,
    0.14938503128761465059143225550110e-09f64,
    -0.26687999885622329284924651063339e-10f64,
    0.45712216985159458151405617724103e-11f64,
    -0.75187305222043565872243727326771e-12f64,
    0.11893100052629681879029828987302e-12f64,
    -0.18116907933852346973490318263084e-13f64,
    0.26611733684358969193001612199626e-14f64,
    -0.37738863052129419795444109905930e-15f64,
    0.51727953789087172679680082229329e-16f64,
    -0.68603684084077500979419564670102e-17f64,
    0.88123751354161071806469337321745e-18f64,
    -0.10974248249996606292106299624652e-18f64,
    0.13261199326367178513595545891635e-19f64,
    -0.15562732768137380785488776571562e-20f64,
    0.17751425583655720607833415570773e-21f64,
    -0.19695006967006578384953608765439e-22f64,
    0.21270074896998699661924010120533e-23f64,
    -0.22375398124627973794182113962666e-24f64,
    0.22942768578582348946971383125333e-25f64,
    -0.22943788846552928693329592319999e-26f64,
    0.22391702100592453618342297600000e-27f64,
    -0.21338230616608897703678225066666e-28f64,
    0.19866196585123531518028458666666e-29f64,
    -0.18079295866694391771955199999999e-30f64,
    0.16090686015283030305450666666666e-31f64,
];
static mut daw2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: daw2_data.as_ptr() as *mut _,
            order: 32 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 21 as libc::c_int,
        };
        init
    }
};
static mut dawa_data: [libc::c_double; 75] = [
    0.1690485637765703755422637438849e-01f64,
    0.8683252278406957990536107850768e-02f64,
    0.2424864042417715453277703459889e-03f64,
    0.1261182399572690001651949240377e-04f64,
    0.1066453314636176955705691125906e-05f64,
    0.1358159794790727611348424505728e-06f64,
    0.2171042356577298398904312744743e-07f64,
    0.2867010501805295270343676804813e-08f64,
    -0.1901336393035820112282492378024e-09f64,
    -0.3097780484395201125532065774268e-09f64,
    -0.1029414876057509247398132286413e-09f64,
    -0.6260356459459576150417587283121e-11f64,
    0.8563132497446451216262303166276e-11f64,
    0.3033045148075659292976266276257e-11f64,
    -0.2523618306809291372630886938826e-12f64,
    -0.4210604795440664513175461934510e-12f64,
    -0.4431140826646238312143429452036e-13f64,
    0.4911210272841205205940037065117e-13f64,
    0.1235856242283903407076477954739e-13f64,
    -0.5788733199016569246955765071069e-14f64,
    -0.2282723294807358620978183957030e-14f64,
    0.7637149411014126476312362917590e-15f64,
    0.3851546883566811728777594002095e-15f64,
    -0.1199932056928290592803237283045e-15f64,
    -0.6313439150094572347334270285250e-16f64,
    0.2239559965972975375254912790237e-16f64,
    0.9987925830076495995132891200749e-17f64,
    -0.4681068274322495334536246507252e-17f64,
    -0.1436303644349721337241628751534e-17f64,
    0.1020822731410541112977908032130e-17f64,
    0.1538908873136092072837389822372e-18f64,
    -0.2189157877645793888894790926056e-18f64,
    0.2156879197938651750392359152517e-20f64,
    0.4370219827442449851134792557395e-19f64,
    -0.8234581460977207241098927905177e-20f64,
    -0.7498648721256466222903202835420e-20f64,
    0.3282536720735671610957612930039e-20f64,
    0.8858064309503921116076561515151e-21f64,
    -0.9185087111727002988094460531485e-21f64,
    0.2978962223788748988314166045791e-22f64,
    0.1972132136618471883159505468041e-21f64,
    -0.5974775596362906638089584995117e-22f64,
    -0.2834410031503850965443825182441e-22f64,
    0.2209560791131554514777150489012e-22f64,
    -0.5439955741897144300079480307711e-25f64,
    -0.5213549243294848668017136696470e-23f64,
    0.1702350556813114199065671499076e-23f64,
    0.6917400860836148343022185660197e-24f64,
    -0.6540941793002752512239445125802e-24f64,
    0.6093576580439328960371824654636e-25f64,
    0.1408070432905187461501945080272e-24f64,
    -0.6785886121054846331167674943755e-25f64,
    -0.9799732036214295711741583102225e-26f64,
    0.2121244903099041332598960939160e-25f64,
    -0.5954455022548790938238802154487e-26f64,
    -0.3093088861875470177838847232049e-26f64,
    0.2854389216344524682400691986104e-26f64,
    -0.3951289447379305566023477271811e-27f64,
    -0.5906000648607628478116840894453e-27f64,
    0.3670236964668687003647889980609e-27f64,
    -0.4839958238042276256598303038941e-29f64,
    -0.9799265984210443869597404017022e-28f64,
    0.4684773732612130606158908804300e-28f64,
    0.5030877696993461051647667603155e-29f64,
    -0.1547395051706028239247552068295e-28f64,
    0.6112180185086419243976005662714e-29f64,
    0.1357913399124811650343602736158e-29f64,
    -0.2417687752768673088385304299044e-29f64,
    0.8369074582074298945292887587291e-30f64,
    0.2665413042788979165838319401566e-30f64,
    -0.3811653692354890336935691003712e-30f64,
    0.1230054721884951464371706872585e-30f64,
    0.4622506399041493508805536929983e-31f64,
    -0.6120087296881677722911435593001e-31f64,
    0.1966024640193164686956230217896e-31f64,
];
static mut dawa_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: dawa_data.as_ptr() as *mut _,
            order: 34 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_dawson_e(
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let xsml: libc::c_double = 1.225f64 * 1.4901161193847656e-08f64;
    let xbig: libc::c_double = 1.0f64
        / (1.41421356237309504880f64 * 1.4901161193847656e-08f64);
    let xmax: libc::c_double = 0.1f64 * 1.7976931348623157e+308f64;
    let y: libc::c_double = fabs(x);
    if y < xsml {
        (*result).val = x;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else if y < 1.0f64 {
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut daw_cs, 2.0f64 * y * y - 1.0f64, &mut result_c);
        (*result).val = x * (0.75f64 + result_c.val);
        (*result).err = y * result_c.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if y < 4.0f64 {
        let mut result_c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut daw2_cs, 0.125f64 * y * y - 1.0f64, &mut result_c_0);
        (*result).val = x * (0.25f64 + result_c_0.val);
        (*result).err = y * result_c_0.err;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if y < xbig {
        let mut result_c_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut dawa_cs, 32.0f64 / (y * y) - 1.0f64, &mut result_c_1);
        (*result).val = (0.5f64 + result_c_1.val) / x;
        (*result).err = result_c_1.err / y;
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if y < xmax {
        (*result).val = 0.5f64 / x;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"dawson.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_dawson(mut x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_dawson_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_dawson_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"dawson.c\0" as *const u8 as *const libc::c_char,
            280 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
