use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_airy_Ai_e(
        x: libc::c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_airy_Bi_e(
        x: libc::c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_airy_Ai_deriv_e(
        x: libc::c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_airy_Bi_deriv_e(
        x: libc::c_double,
        mode: gsl_mode_t,
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
pub type gsl_mode_t = libc::c_uint;
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
static mut zofmzeta_a_data: [libc::c_double; 20] = [
    2.9332563730829348990f64,
    0.4896518224847036624f64,
    0.0228637617355380860f64,
    -0.0001715731377284693f64,
    -0.0000105927538148751f64,
    1.0595602530419e-6f64,
    -4.68016051691e-8f64,
    5.8310020e-12f64,
    1.766537581e-10f64,
    -1.45034640e-11f64,
    4.357772e-13f64,
    4.60971e-14f64,
    -2.57571e-14f64,
    2.26468e-14f64,
    -2.22053e-14f64,
    2.08593e-14f64,
    -1.84454e-14f64,
    1.50150e-14f64,
    -1.06506e-14f64,
    5.5375e-15f64,
];
static mut zofmzeta_a_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: zofmzeta_a_data.as_ptr() as *mut _,
            order: 19 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
static mut zofmzeta_b_data: [libc::c_double; 30] = [
    22.40725276466303489f64,
    10.39808258825165581f64,
    1.092050144486018425f64,
    -0.071111274777921604f64,
    0.008990125336059704f64,
    -0.001201950338088875f64,
    0.000106686807968315f64,
    0.000017406491576830f64,
    -0.000014946669657805f64,
    6.189984487752e-6f64,
    -2.049466715178e-6f64,
    5.87189458020e-7f64,
    -1.46077514157e-7f64,
    2.9803936132e-8f64,
    -3.817692108e-9f64,
    -4.66980416e-10f64,
    5.83860334e-10f64,
    -2.78825299e-10f64,
    1.01682688e-10f64,
    -3.1209928e-11f64,
    8.111122e-12f64,
    -1.663986e-12f64,
    1.81364e-13f64,
    5.3414e-14f64,
    -4.7234e-14f64,
    2.1689e-14f64,
    -7.815e-15f64,
    2.371e-15f64,
    -6.04e-16f64,
    1.20e-16f64,
];
static mut zofmzeta_b_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: zofmzeta_b_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 15 as libc::c_int,
        };
        init
    }
};
static mut zofmzeta_c_data: [libc::c_double; 11] = [
    1.3824761227122911500f64,
    0.0244856101686774245f64,
    -0.0000842866496282540f64,
    1.4656076569771e-6f64,
    -3.14874099476e-8f64,
    7.561134833e-10f64,
    -1.94531643e-11f64,
    5.245878e-13f64,
    -1.46380e-14f64,
    4.192e-16f64,
    -1.23e-17f64,
];
static mut zofmzeta_c_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: zofmzeta_c_data.as_ptr() as *mut _,
            order: 10 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 6 as libc::c_int,
        };
        init
    }
};
pub unsafe extern "C" fn gsl_sf_bessel_Olver_zofmzeta(
    mut minus_zeta: libc::c_double,
) -> libc::c_double {
    if minus_zeta < 1.0f64 {
        let x: libc::c_double = 2.0f64 * minus_zeta - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut zofmzeta_a_cs, x, &mut c);
        return c.val;
    } else if minus_zeta < 10.0f64 {
        let x_0: libc::c_double = (2.0f64 * minus_zeta - 11.0f64) / 9.0f64;
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut zofmzeta_b_cs, x_0, &mut c_0);
        return c_0.val;
    } else {
        let TEN_32: libc::c_double = 31.62277660168379332f64;
        let p: libc::c_double = pow(minus_zeta, 3.0f64 / 2.0f64);
        let x_1: libc::c_double = 2.0f64 * TEN_32 / p - 1.0f64;
        let mut c_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut zofmzeta_c_cs, x_1, &mut c_1);
        return c_1.val * p;
    };
}
static mut A3_gt1_data: [libc::c_double; 31] = [
    -0.123783199829515294670493131190f64,
    0.104636462534700704670877382304f64,
    -0.067500816575851826744877535903f64,
    0.035563362418888483652711005520f64,
    -0.0160738524035979408472979609051f64,
    0.0064497878252851092073278056238f64,
    -0.00235408261133449663958121821593f64,
    0.00079545702851302155411892534965f64,
    -0.00025214920745855079895784825637f64,
    0.00007574004596069392921153301833f64,
    -0.00002172917966339623434407978263f64,
    5.9914810727868915476543145465e-06f64,
    -1.5958781571808992162953719817e-06f64,
    4.1232986512903717525448312012e-07f64,
    -1.0369725993417659101913919101e-07f64,
    2.5457982304266541145999235022e-08f64,
    -6.1161715053791743082427422443e-09f64,
    1.4409346199138658887871461320e-09f64,
    -3.3350445956255561668232014995e-10f64,
    7.5950686572918996453336138108e-11f64,
    -1.7042296334409430377389900278e-11f64,
    3.7723525020626230919721640081e-12f64,
    -8.2460237635733980528416501227e-13f64,
    1.7816961527997797696251868875e-13f64,
    -3.8084101506541792942694560802e-14f64,
    8.0593669930916099079755351563e-15f64,
    -1.6896565961641739017452636964e-15f64,
    3.5115651805888443184822853595e-16f64,
    -7.2384771938569255638904297651e-17f64,
    1.4806598977677176106283840244e-17f64,
    -3.0069285750787303634897997963e-18f64,
];
static mut A3_gt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: A3_gt1_data.as_ptr() as *mut _,
            order: 30 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 17 as libc::c_int,
        };
        init
    }
};
static mut A4_gt1_data: [libc::c_double; 30] = [
    1.15309329391198493586724229008f64,
    -1.01812701728669338904729927846f64,
    0.71964022270555684403652781941f64,
    -0.42359963977172689685150061355f64,
    0.215024488759339557817435404261f64,
    -0.096751915348145944032096342479f64,
    0.039413982058824310099856035361f64,
    -0.014775225692561697963781115014f64,
    0.005162114514159370516947823271f64,
    -0.00169783446445524322560925166335f64,
    0.00052995667873006847211519193478f64,
    -0.00015802027574996477115667974856f64,
    0.000045254366680989687988902825193f64,
    -0.000012503722965474638015488600967f64,
    3.3457656998119148699124716204e-06f64,
    -8.6981575241150758412492331833e-07f64,
    2.2030895484325645640823940625e-07f64,
    -5.4493369492600677068285936533e-08f64,
    1.3190457281724829107139385556e-08f64,
    -3.1301560183377379158951191769e-09f64,
    7.2937802527123344842593076131e-10f64,
    -1.6712080137945140407348940109e-10f64,
    3.7700053248213600430503521194e-11f64,
    -8.3824538848817227637828899571e-12f64,
    1.8388741910049766865274037194e-12f64,
    -3.9835919980753778560117573063e-13f64,
    8.5288827136546615604290389711e-14f64,
    -1.8060227869114416998653266836e-14f64,
    3.7849342199690728470461022877e-15f64,
    -7.8552867468122209577151823365e-16f64,
];
static mut A4_gt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: A4_gt1_data.as_ptr() as *mut _,
            order: 17 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 17 as libc::c_int,
        };
        init
    }
};
static mut B2_gt1_data: [libc::c_double; 40] = [
    0.00118587147272683864479328868589f64,
    0.00034820459990648274622193981840f64,
    -0.00030411304425639768103075864567f64,
    0.00002812066284012343531484682886f64,
    0.00004493525295901613184489898748f64,
    -0.00003037629997093072196779489677f64,
    0.00001125979647123875721949743970f64,
    -2.4832533969517775991951008218e-06f64,
    -9.9003813640537799587086928278e-08f64,
    4.9259859656183110299492296029e-07f64,
    -3.7644120964426705960749504975e-07f64,
    2.2887828521334625189639122509e-07f64,
    -1.3202687370822203731489855050e-07f64,
    7.7019669092537400811434860763e-08f64,
    -4.6589706973010511603890144294e-08f64,
    2.9396476233013923711978522963e-08f64,
    -1.9293230611988282919101954538e-08f64,
    1.3099107013728717842406906896e-08f64,
    -9.1509111940885962831104149355e-09f64,
    6.5483472971925614347299375295e-09f64,
    -4.7831253582139967461241674569e-09f64,
    3.5562625457426178152760148639e-09f64,
    -2.6853389444008414186916562103e-09f64,
    2.0554738667134200145781857289e-09f64,
    -1.5923172019517426277886522758e-09f64,
    1.2465923213464381457319481498e-09f64,
    -9.8494846881180588507969988989e-10f64,
    7.8438674499372126663957464312e-10f64,
    -6.2877567918342950225937136855e-10f64,
    5.0662318868755257959686944117e-10f64,
    -4.0962270881243451160378710952e-10f64,
    3.3168684677374908553161911299e-10f64,
    -2.6829406619847450633596163305e-10f64,
    2.1603988122184568375561077873e-10f64,
    -1.7232373309560278402012124481e-10f64,
    1.3512709089611470626617830434e-10f64,
    -1.0285354732538663013167579792e-10f64,
    7.4211345443901713467637018423e-11f64,
    -4.8124980266864320351456993068e-11f64,
    2.3666534694476306077416831958e-11f64,
];
static mut B2_gt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: B2_gt1_data.as_ptr() as *mut _,
            order: 39 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 30 as libc::c_int,
        };
        init
    }
};
static mut B3_gt1_data: [libc::c_double; 30] = [
    -0.0102445379362695740863663926486f64,
    0.0036618484329295342954730801917f64,
    0.0026154252498599303282569321117f64,
    -0.0036187389410353156728771706336f64,
    0.0021878564157692275944613452462f64,
    -0.0008219952303590803584426516821f64,
    0.0001281773889155631494321316520f64,
    0.0001000944653368032985720548637f64,
    -0.0001288293344663774273453147788f64,
    0.00010136264202696513867821487205f64,
    -0.00007000275849659556221916572733f64,
    0.00004694886396757430431607955146f64,
    -0.00003190003869717837686356945696f64,
    0.00002231453668447775219665947479f64,
    -0.00001611102197712439539300336438f64,
    0.00001196634424990735214466633513f64,
    -9.0986920398931223804111374679e-06f64,
    7.0492613694235423068926562567e-06f64,
    -5.5425216624642184684300615394e-06f64,
    4.4071884714230296614449244106e-06f64,
    -3.5328595506791663127928952625e-06f64,
    2.84594975572077091520522824686e-06f64,
    -2.29592697828824392391071619788e-06f64,
    1.84714740375289956396370322228e-06f64,
    -1.47383331248116454652025598620e-06f64,
    1.15687781098593231076084710267e-06f64,
    -8.8174688524627071175315084910e-07f64,
    6.3705856964426840441434605593e-07f64,
    -4.1358791499961929237755474814e-07f64,
    2.0354151158738819867477996807e-07f64,
];
static mut B3_gt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: B3_gt1_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 29 as libc::c_int,
        };
        init
    }
};
static mut B2_lt1_data: [libc::c_double; 40] = [
    0.00073681565841337130021924199490f64,
    0.00033803599647571227535304316937f64,
    -0.00008251723219239754024210552679f64,
    -0.00003390879948656432545900779710f64,
    0.00001961398056848881816694014889f64,
    -2.35593745904151401624656805567e-06f64,
    -1.79055017080406086541563835433e-06f64,
    1.33129571185610681090725934031e-06f64,
    -5.38879444715436544130673956170e-07f64,
    1.49603056041381416881299945557e-07f64,
    -1.83377228267274327911131293091e-08f64,
    -1.33191430762944336526965187651e-08f64,
    1.60642096463700438411396889489e-08f64,
    -1.28932576330421806740136816643e-08f64,
    9.6169275086179165484403221944e-09f64,
    -7.1818502280703532276832887290e-09f64,
    5.4744009217215145730697754561e-09f64,
    -4.2680446690508456935030086136e-09f64,
    3.3941665009266174865683284781e-09f64,
    -2.7440714072221673882163135170e-09f64,
    2.2488361522108255229193038962e-09f64,
    -1.8638240716608748862087923337e-09f64,
    1.5592350940805373500866440401e-09f64,
    -1.3145743937732330609242633070e-09f64,
    1.1153716777215047842790244968e-09f64,
    -9.5117576805266622854647303110e-10f64,
    8.1428799553234876296804561100e-10f64,
    -6.9893770813548773664326279169e-10f64,
    6.0073113636087448745018831981e-10f64,
    -5.1627434258513453901420776514e-10f64,
    4.4290993195074905891788459756e-10f64,
    -3.7852978599966867611179315200e-10f64,
    3.2143959338863177145307610452e-10f64,
    -2.7025926680620777594992221143e-10f64,
    2.2384857772457918539228234321e-10f64,
    -1.8125071664276678046551271701e-10f64,
    1.4164870008713668767293008546e-10f64,
    -1.0433101857132782485813325981e-10f64,
    6.8663910168392483929411418190e-11f64,
    -3.4068313177952244040559740439e-11f64,
];
static mut B2_lt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: B2_lt1_data.as_ptr() as *mut _,
            order: 39 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 39 as libc::c_int,
        };
        init
    }
};
static mut B3_lt1_data: [libc::c_double; 40] = [
    -0.00137160820526992057354001614451f64,
    -0.00025474937951101049982680561302f64,
    0.00024762975547895881652073467771f64,
    0.00005229657281480196749313930265f64,
    -0.00007488354272621512385016593760f64,
    0.00001416880012891046449980449746f64,
    0.00001528986060172183690742576230f64,
    -0.00001668672297078590514293325326f64,
    0.00001061765189536459018739585094f64,
    -5.8220577442406209989680801335e-06f64,
    3.3322423743855900506302033234e-06f64,
    -2.23292405803003860894449897815e-06f64,
    1.74816651036678291794777245325e-06f64,
    -1.49581306041395051804547535093e-06f64,
    1.32759146107893129050610165582e-06f64,
    -1.19376077392564467408373553343e-06f64,
    1.07878303863211630544654040875e-06f64,
    -9.7743335011819134006676476250e-07f64,
    8.8729318903693324226127054792e-07f64,
    -8.0671146292125665050876015280e-07f64,
    7.3432860378667354971042255937e-07f64,
    -6.6897926072697370325310483359e-07f64,
    6.0966619703735610352576581485e-07f64,
    -5.5554095284507959561958605420e-07f64,
    5.0588335673197236002812826526e-07f64,
    -4.6008146297767601862670079590e-07f64,
    4.1761348515688145911438168306e-07f64,
    -3.7803230006989446874174476515e-07f64,
    3.4095248501364300041684648230e-07f64,
    -3.0603959751354749520615015472e-07f64,
    2.7300134179365690589640458993e-07f64,
    -2.4158028250762304756044254231e-07f64,
    2.1154781038298751985689113868e-07f64,
    -1.8269911328756771201465223313e-07f64,
    1.5484895085808513749026173074e-07f64,
    -1.2782806851555809369226440495e-07f64,
    1.0148011725394892565174207341e-07f64,
    -7.5658969771439627809239950461e-08f64,
    5.0226342286491286957075289622e-08f64,
    -2.5049645660259882970547555831e-08f64,
];
static mut B3_lt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: B3_lt1_data.as_ptr() as *mut _,
            order: 39 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 39 as libc::c_int,
        };
        init
    }
};
static mut A3_lt1_data: [libc::c_double; 40] = [
    -0.00017982561472134418587634980117f64,
    -0.00036558603837525275836608884064f64,
    -0.00002819398055929628850294406363f64,
    0.00016704539863875736769812786067f64,
    -0.00007098969970347674307623044850f64,
    -8.4470843942344237748899879940e-06f64,
    0.0000273413090343147765148014327150f64,
    -0.0000199073838489821681991178018081f64,
    0.0000100004176278235088881096950105f64,
    -3.9739852013143676487867902026e-06f64,
    1.2265357766449574306882693267e-06f64,
    -1.88755584306424047416914864854e-07f64,
    -1.37482206060161206336523452036e-07f64,
    2.10326379301853336795686477738e-07f64,
    -2.05583778245412633433934301948e-07f64,
    1.82377384812654863038691147988e-07f64,
    -1.58130247846381041027699152436e-07f64,
    1.36966982725588978654041029615e-07f64,
    -1.19250280944620257443805710485e-07f64,
    1.04477169029350256435316644493e-07f64,
    -9.2064832489437534542041040184e-08f64,
    8.1523798290458784610230199344e-08f64,
    -7.2471794980050867512294061891e-08f64,
    6.4614432955971132569968860233e-08f64,
    -5.7724095125560946811081322985e-08f64,
    5.1623107567436835158110947901e-08f64,
    -4.6171250746798606260216486042e-08f64,
    4.1256621998650164023254101585e-08f64,
    -3.6788925543159819135102047082e-08f64,
    3.2694499457951844422299750661e-08f64,
    -2.89125899697964696586521743928e-08f64,
    2.53925288725374047626589488217e-08f64,
    -2.20915707933726481321465184207e-08f64,
    1.89732166352720474944407102940e-08f64,
    -1.60058977893259856012119939554e-08f64,
    1.31619294542205876946742394494e-08f64,
    -1.04166651771938038563454275883e-08f64,
    7.7478015858156185064152078434e-09f64,
    -5.1347942579352613057675111787e-09f64,
    2.5583541594586723967261504321e-09f64,
];
static mut A3_lt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: A3_lt1_data.as_ptr() as *mut _,
            order: 39 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 39 as libc::c_int,
        };
        init
    }
};
static mut A4_lt1_data: [libc::c_double; 30] = [
    0.00009054703770051610946958226736f64,
    0.00033066000498098017589672988293f64,
    0.00019737453734363989127226073272f64,
    -0.00015490809725932037720034762889f64,
    -0.00004514948935538730085479280454f64,
    0.00007976881782603940889444573924f64,
    -0.00003314566154544740986264993251f64,
    -1.88212148790135672249935711657e-06f64,
    0.0000114788756505519986352882940648f64,
    -9.2263039911196207101468331210e-06f64,
    5.1401128250377780476084336340e-06f64,
    -2.38418218951722002658891397905e-06f64,
    1.00664292214481531598338960828e-06f64,
    -4.23224678096490060264249970540e-07f64,
    2.00132031535793489976535190025e-07f64,
    -1.18689501178886741400633921047e-07f64,
    8.7819524319114212999768013738e-08f64,
    -7.3964150324206644900787216386e-08f64,
    6.5780431507637165113885884236e-08f64,
    -5.9651053193022652369837650411e-08f64,
    5.4447762662767276209052293773e-08f64,
    -4.9802057381568863702541294988e-08f64,
    4.5571368194694340198117635845e-08f64,
    -4.1682117173547642845382848197e-08f64,
    3.8084701352766049815367147717e-08f64,
    -3.4740302885185237434662649907e-08f64,
    3.1616557064701510611273692060e-08f64,
    -2.8685739487689556252374879267e-08f64,
    2.5923752117132254429002796600e-08f64,
    -2.3309428552190587304662883477e-08f64,
];
static mut A4_lt1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: A4_lt1_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 29 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn olver_B0(
    mut z: libc::c_double,
    mut abs_zeta: libc::c_double,
) -> libc::c_double {
    if z < 0.98f64 {
        let t: libc::c_double = 1.0f64 / sqrt(1.0f64 - z * z);
        return -5.0f64 / (48.0f64 * abs_zeta * abs_zeta)
            + t * (-3.0f64 + 5.0f64 * t * t) / (24.0f64 * sqrt(abs_zeta));
    } else if z < 1.02f64 {
        let a: libc::c_double = 1.0f64 - z;
        let c0: libc::c_double = 0.0179988721413553309252458658183f64;
        let c1: libc::c_double = 0.0111992982212877614645974276203f64;
        let c2: libc::c_double = 0.0059404069786014304317781160605f64;
        let c3: libc::c_double = 0.0028676724516390040844556450173f64;
        let c4: libc::c_double = 0.0012339189052567271708525111185f64;
        let c5: libc::c_double = 0.0004169250674535178764734660248f64;
        let c6: libc::c_double = 0.0000330173385085949806952777365f64;
        let c7: libc::c_double = -0.0001318076238578203009990106425f64;
        let c8: libc::c_double = -0.0001906870370050847239813945647f64;
        return c0
            + a
                * (c1
                    + a
                        * (c2
                            + a
                                * (c3
                                    + a * (c4 + a * (c5 + a * (c6 + a * (c7 + a * c8)))))));
    } else {
        let t_0: libc::c_double = 1.0f64 / (z * sqrt(1.0f64 - 1.0f64 / (z * z)));
        return -5.0f64 / (48.0f64 * abs_zeta * abs_zeta)
            + t_0 * (3.0f64 + 5.0f64 * t_0 * t_0) / (24.0f64 * sqrt(abs_zeta));
    };
}
unsafe extern "C" fn olver_B1(
    mut z: libc::c_double,
    mut abs_zeta: libc::c_double,
) -> libc::c_double {
    if z < 0.88f64 {
        let t: libc::c_double = 1.0f64 / sqrt(1.0f64 - z * z);
        let t2: libc::c_double = t * t;
        let rz: libc::c_double = sqrt(abs_zeta);
        let z32: libc::c_double = rz * rz * rz;
        let z92: libc::c_double = z32 * z32 * z32;
        let term1: libc::c_double = t * t * t
            * (30375.0f64 - 369603.0f64 * t2 + 765765.0f64 * t2 * t2
                - 425425.0f64 * t2 * t2 * t2) / 414720.0f64;
        let term2: libc::c_double = 85085.0f64 / (663552.0f64 * z92);
        let term3: libc::c_double = 385.0f64 / 110592.0f64 * t * (3.0f64 - 5.0f64 * t2)
            / (abs_zeta * abs_zeta * abs_zeta);
        let term4: libc::c_double = 5.0f64 / 55296.0f64 * t2
            * (81.0f64 - 462.0f64 * t2 + 385.0f64 * t2 * t2) / z32;
        return -(term1 + term2 + term3 + term4) / rz;
    } else if z < 1.12f64 {
        let a: libc::c_double = 1.0f64 - z;
        let c0: libc::c_double = -0.00149282953213429172050073403334f64;
        let c1: libc::c_double = -0.00175640941909277865678308358128f64;
        let c2: libc::c_double = -0.00113346148874174912576929663517f64;
        let c3: libc::c_double = -0.00034691090981382974689396961817f64;
        let c4: libc::c_double = 0.00022752516104839243675693256916f64;
        let c5: libc::c_double = 0.00051764145724244846447294636552f64;
        let c6: libc::c_double = 0.00058906174858194233998714243010f64;
        let c7: libc::c_double = 0.00053485514521888073087240392846f64;
        let c8: libc::c_double = 0.00042891792986220150647633418796f64;
        let c9: libc::c_double = 0.00031639765900613633260381972850f64;
        let c10: libc::c_double = 0.00021908147678699592975840749194f64;
        return c0
            + a
                * (c1
                    + a
                        * (c2
                            + a
                                * (c3
                                    + a
                                        * (c4
                                            + a
                                                * (c5
                                                    + a * (c6 + a * (c7 + a * (c8 + a * (c9 + a * c10)))))))));
    } else {
        let t_0: libc::c_double = 1.0f64 / (z * sqrt(1.0f64 - 1.0f64 / (z * z)));
        let t2_0: libc::c_double = t_0 * t_0;
        let rz_0: libc::c_double = sqrt(abs_zeta);
        let z32_0: libc::c_double = rz_0 * rz_0 * rz_0;
        let z92_0: libc::c_double = z32_0 * z32_0 * z32_0;
        let term1_0: libc::c_double = -t2_0 * t_0
            * (30375.0f64 + 369603.0f64 * t2_0 + 765765.0f64 * t2_0 * t2_0
                + 425425.0f64 * t2_0 * t2_0 * t2_0) / 414720.0f64;
        let term2_0: libc::c_double = 85085.0f64 / (663552.0f64 * z92_0);
        let term3_0: libc::c_double = -385.0f64 / 110592.0f64 * t_0
            * (3.0f64 + 5.0f64 * t2_0) / (abs_zeta * abs_zeta * abs_zeta);
        let term4_0: libc::c_double = 5.0f64 / 55296.0f64 * t2_0
            * (81.0f64 + 462.0f64 * t2_0 + 385.0f64 * t2_0 * t2_0) / z32_0;
        return (term1_0 + term2_0 + term3_0 + term4_0) / rz_0;
    };
}
unsafe extern "C" fn olver_B2(mut z: libc::c_double) -> libc::c_double {
    if z < 0.8f64 {
        let x: libc::c_double = 5.0f64 * z / 2.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut B2_lt1_cs, x, &mut c);
        return c.val / z;
    } else if z <= 1.2f64 {
        let a: libc::c_double = 1.0f64 - z;
        let c0: libc::c_double = 0.00055221307672129279005986982501f64;
        let c1: libc::c_double = 0.00089586516310476929281129228969f64;
        let c2: libc::c_double = 0.00067015003441569770883539158863f64;
        let c3: libc::c_double = 0.00010166263361949045682945811828f64;
        let c4: libc::c_double = -0.00044086345133806887291336488582f64;
        let c5: libc::c_double = -0.00073963081508788743392883072523f64;
        let c6: libc::c_double = -0.00076745494377839561259903887331f64;
        let c7: libc::c_double = -0.00060829038106040362291568012663f64;
        let c8: libc::c_double = -0.00037128707528893496121336168683f64;
        let c9: libc::c_double = -0.00014116325105702609866850307176f64;
        return c0
            + a
                * (c1
                    + a
                        * (c2
                            + a
                                * (c3
                                    + a
                                        * (c4
                                            + a * (c5 + a * (c6 + a * (c7 + a * (c8 + a * c9))))))));
    } else {
        let zi: libc::c_double = 1.0f64 / z;
        let x_0: libc::c_double = 12.0f64 / 5.0f64 * zi - 1.0f64;
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut B2_gt1_cs, x_0, &mut c_0);
        return c_0.val * zi * zi * zi;
    };
}
unsafe extern "C" fn olver_B3(mut z: libc::c_double) -> libc::c_double {
    if z < 0.8f64 {
        let x: libc::c_double = 5.0f64 * z / 2.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut B3_lt1_cs, x, &mut c);
        return c.val;
    } else if z < 1.2f64 {
        let a: libc::c_double = 1.0f64 - z;
        let c0: libc::c_double = -0.00047461779655995980754441833105f64;
        let c1: libc::c_double = -0.00095572913429464297452176811898f64;
        let c2: libc::c_double = -0.00080369634512082892655558133973f64;
        let c3: libc::c_double = -0.00000727921669154784138080600339f64;
        let c4: libc::c_double = 0.00093162500331581345235746518994f64;
        let c5: libc::c_double = 0.00149848796913751497227188612403f64;
        let c6: libc::c_double = 0.00148406039675949727870390426462f64;
        return c0 + a * (c1 + a * (c2 + a * (c3 + a * (c4 + a * (c5 + a * c6)))));
    } else {
        let x_0: libc::c_double = 12.0f64 / (5.0f64 * z) - 1.0f64;
        let zi2: libc::c_double = 1.0f64 / (z * z);
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut B3_gt1_cs, x_0, &mut c_0);
        return c_0.val * zi2 * zi2 * zi2;
    };
}
unsafe extern "C" fn olver_A1(
    mut z: libc::c_double,
    mut abs_zeta: libc::c_double,
    mut err: *mut libc::c_double,
) -> libc::c_double {
    if z < 0.98f64 {
        let mut t: libc::c_double = 1.0f64 / sqrt(1.0f64 - z * z);
        let mut rz: libc::c_double = sqrt(abs_zeta);
        let mut t2: libc::c_double = t * t;
        let mut term1: libc::c_double = t2
            * (81.0f64 - 462.0f64 * t2 + 385.0f64 * t2 * t2) / 1152.0f64;
        let mut term2: libc::c_double = -455.0f64
            / (4608.0f64 * abs_zeta * abs_zeta * abs_zeta);
        let mut term3: libc::c_double = 7.0f64 * t * (-3.0f64 + 5.0f64 * t2)
            / (1152.0f64 * rz * rz * rz);
        *err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(term1) + fabs(term2) + fabs(term3));
        return term1 + term2 + term3;
    } else if z < 1.02f64 {
        let a: libc::c_double = 1.0f64 - z;
        let c0: libc::c_double = -0.00444444444444444444444444444444f64;
        let c1: libc::c_double = -0.00184415584415584415584415584416f64;
        let c2: libc::c_double = 0.00056812076812076812076812076812f64;
        let c3: libc::c_double = 0.00168137865661675185484709294233f64;
        let c4: libc::c_double = 0.00186744042139000122193399504324f64;
        let c5: libc::c_double = 0.00161330105833747826430066790326f64;
        let c6: libc::c_double = 0.00123177312220625816558607537838f64;
        let c7: libc::c_double = 0.00087334711007377573881689318421f64;
        let c8: libc::c_double = 0.00059004942455353250141217015410f64;
        let sum: libc::c_double = c0
            + a
                * (c1
                    + a
                        * (c2
                            + a
                                * (c3
                                    + a * (c4 + a * (c5 + a * (c6 + a * (c7 + a * c8)))))));
        *err = 2.0f64 * 2.2204460492503131e-16f64 * fabs(sum);
        return sum;
    } else {
        let t_0: libc::c_double = 1.0f64 / (z * sqrt(1.0f64 - 1.0f64 / (z * z)));
        let rz_0: libc::c_double = sqrt(abs_zeta);
        let t2_0: libc::c_double = t_0 * t_0;
        let term1_0: libc::c_double = -t2_0
            * (81.0f64 + 462.0f64 * t2_0 + 385.0f64 * t2_0 * t2_0) / 1152.0f64;
        let term2_0: libc::c_double = 455.0f64
            / (4608.0f64 * abs_zeta * abs_zeta * abs_zeta);
        let term3_0: libc::c_double = -7.0f64 * t_0 * (3.0f64 + 5.0f64 * t2_0)
            / (1152.0f64 * rz_0 * rz_0 * rz_0);
        *err = 2.0f64 * 2.2204460492503131e-16f64
            * (fabs(term1_0) + fabs(term2_0) + fabs(term3_0));
        return term1_0 + term2_0 + term3_0;
    };
}
unsafe extern "C" fn olver_A2(
    mut z: libc::c_double,
    mut abs_zeta: libc::c_double,
) -> libc::c_double {
    if z < 0.88f64 {
        let mut t: libc::c_double = 1.0f64 / sqrt(1.0f64 - z * z);
        let mut t2: libc::c_double = t * t;
        let mut t4: libc::c_double = t2 * t2;
        let mut t6: libc::c_double = t4 * t2;
        let mut t8: libc::c_double = t4 * t4;
        let mut rz: libc::c_double = sqrt(abs_zeta);
        let mut z3: libc::c_double = abs_zeta * abs_zeta * abs_zeta;
        let mut z32: libc::c_double = rz * rz * rz;
        let mut z92: libc::c_double = z3 * z32;
        let mut term1: libc::c_double = t4
            * (4465125.0f64 - 94121676.0f64 * t2 + 349922430.0f64 * t4
                - 446185740.0f64 * t6 + 185910725.0f64 * t8) / 39813120.0f64;
        let mut term2: libc::c_double = -40415375.0f64 / (127401984.0f64 * z3 * z3);
        let mut term3: libc::c_double = -95095.0f64 / 15925248.0f64 * t
            * (3.0f64 - 5.0f64 * t2) / z92;
        let mut term4: libc::c_double = -455.0f64 / 5308416.0f64 * t2
            * (81.0f64 - 462.0f64 * t2 + 385.0f64 * t4) / z3;
        let mut term5: libc::c_double = -7.0f64 / 19906560.0f64 * t * t2
            * (30375.0f64 - 369603.0f64 * t2 + 765765.0f64 * t4 - 425425.0f64 * t6)
            / z32;
        return term1 + term2 + term3 + term4 + term5;
    } else if z < 1.12f64 {
        let mut a: libc::c_double = 1.0f64 - z;
        let c0: libc::c_double = 0.000693735541354588973636592684210f64;
        let c1: libc::c_double = 0.000464483490365843307019777608010f64;
        let c2: libc::c_double = -0.000289036254605598132482570468291f64;
        let c3: libc::c_double = -0.000874764943953712638574497548110f64;
        let c4: libc::c_double = -0.001029716376139865629968584679350f64;
        let c5: libc::c_double = -0.000836857329713810600584714031650f64;
        let c6: libc::c_double = -0.000488910893527218954998270124540f64;
        let c7: libc::c_double = -0.000144236747940817220502256810151f64;
        let c8: libc::c_double = 0.000114363800986163478038576460325f64;
        let c9: libc::c_double = 0.000266806881492777536223944807117f64;
        let c10: libc::c_double = -0.011975517576151069627471048587000f64;
        return c0
            + a
                * (c1
                    + a
                        * (c2
                            + a
                                * (c3
                                    + a
                                        * (c4
                                            + a
                                                * (c5
                                                    + a * (c6 + a * (c7 + a * (c8 + a * (c9 + a * c10)))))))));
    } else {
        let t_0: libc::c_double = 1.0f64 / (z * sqrt(1.0f64 - 1.0f64 / (z * z)));
        let t2_0: libc::c_double = t_0 * t_0;
        let t4_0: libc::c_double = t2_0 * t2_0;
        let t6_0: libc::c_double = t4_0 * t2_0;
        let t8_0: libc::c_double = t4_0 * t4_0;
        let rz_0: libc::c_double = sqrt(abs_zeta);
        let z3_0: libc::c_double = abs_zeta * abs_zeta * abs_zeta;
        let z32_0: libc::c_double = rz_0 * rz_0 * rz_0;
        let z92_0: libc::c_double = z3_0 * z32_0;
        let term1_0: libc::c_double = t4_0
            * (4465125.0f64 + 94121676.0f64 * t2_0 + 349922430.0f64 * t4_0
                + 446185740.0f64 * t6_0 + 185910725.0f64 * t8_0) / 39813120.0f64;
        let term2_0: libc::c_double = -40415375.0f64 / (127401984.0f64 * z3_0 * z3_0);
        let term3_0: libc::c_double = 95095.0f64 / 15925248.0f64 * t_0
            * (3.0f64 + 5.0f64 * t2_0) / z92_0;
        let term4_0: libc::c_double = -455.0f64 / 5308416.0f64 * t2_0
            * (81.0f64 + 462.0f64 * t2_0 + 385.0f64 * t4_0) / z3_0;
        let term5_0: libc::c_double = 7.0f64 / 19906560.0f64 * t_0 * t2_0
            * (30375.0f64 + 369603.0f64 * t2_0 + 765765.0f64 * t4_0 + 425425.0f64 * t6_0)
            / z32_0;
        return term1_0 + term2_0 + term3_0 + term4_0 + term5_0;
    };
}
unsafe extern "C" fn olver_A3(mut z: libc::c_double) -> libc::c_double {
    if z < 0.9f64 {
        let x: libc::c_double = 20.0f64 * z / 9.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut A3_lt1_cs, x, &mut c);
        return c.val;
    } else if z < 1.1f64 {
        let mut a: libc::c_double = 1.0f64 - z;
        let c0: libc::c_double = -0.000354211971457743840771125759200f64;
        let c1: libc::c_double = -0.000312322527890318832782774881353f64;
        let c2: libc::c_double = 0.000277947465383133980329617631915f64;
        let c3: libc::c_double = 0.000919803044747966977054155192400f64;
        let c4: libc::c_double = 0.001147600388275977640983696906320f64;
        let c5: libc::c_double = 0.000869239326123625742931772044544f64;
        let c6: libc::c_double = 0.000287392257282507334785281718027f64;
        return c0 + a * (c1 + a * (c2 + a * (c3 + a * (c4 + a * (c5 + a * c6)))));
    } else {
        let x_0: libc::c_double = 11.0f64 / (5.0f64 * z) - 1.0f64;
        let zi2: libc::c_double = 1.0f64 / (z * z);
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut A3_gt1_cs, x_0, &mut c_0);
        return c_0.val * zi2 * zi2 * zi2;
    };
}
unsafe extern "C" fn olver_A4(mut z: libc::c_double) -> libc::c_double {
    if z < 0.8f64 {
        let x: libc::c_double = 5.0f64 * z / 2.0f64 - 1.0f64;
        let mut c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut A4_lt1_cs, x, &mut c);
        return c.val;
    } else if z < 1.2f64 {
        let mut a: libc::c_double = 1.0f64 - z;
        let c0: libc::c_double = 0.00037819419920177291402661228437f64;
        let c1: libc::c_double = 0.00040494390552363233477213857527f64;
        let c2: libc::c_double = -0.00045764735528936113047289344569f64;
        let c3: libc::c_double = -0.00165361044229650225813161341879f64;
        let c4: libc::c_double = -0.00217527517983360049717137015539f64;
        let c5: libc::c_double = -0.00152003287866490735107772795537f64;
        return c0 + a * (c1 + a * (c2 + a * (c3 + a * (c4 + a * c5))));
    } else {
        let x_0: libc::c_double = 12.0f64 / (5.0f64 * z) - 1.0f64;
        let zi2: libc::c_double = 1.0f64 / (z * z);
        let mut c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut A4_gt1_cs, x_0, &mut c_0);
        return c_0.val * zi2 * zi2 * zi2 * zi2;
    };
}
#[inline]
unsafe extern "C" fn olver_Asum(
    mut nu: libc::c_double,
    mut z: libc::c_double,
    mut abs_zeta: libc::c_double,
    mut err: *mut libc::c_double,
) -> libc::c_double {
    let mut nu2: libc::c_double = nu * nu;
    let mut A1_err: libc::c_double = 0.;
    let mut A1: libc::c_double = olver_A1(z, abs_zeta, &mut A1_err);
    let mut A2: libc::c_double = olver_A2(z, abs_zeta);
    let mut A3: libc::c_double = olver_A3(z);
    let mut A4: libc::c_double = olver_A4(z);
    *err = A1_err / nu2 + 2.2204460492503131e-16f64;
    return 1.0f64 + A1 / nu2 + A2 / (nu2 * nu2) + A3 / (nu2 * nu2 * nu2)
        + A4 / (nu2 * nu2 * nu2 * nu2);
}
#[inline]
unsafe extern "C" fn olver_Bsum(
    mut nu: libc::c_double,
    mut z: libc::c_double,
    mut abs_zeta: libc::c_double,
) -> libc::c_double {
    let mut nu2: libc::c_double = nu * nu;
    let mut B0: libc::c_double = olver_B0(z, abs_zeta);
    let mut B1: libc::c_double = olver_B1(z, abs_zeta);
    let mut B2: libc::c_double = olver_B2(z);
    let mut B3: libc::c_double = olver_B3(z);
    return B0 + B1 / nu2 + B2 / (nu2 * nu2) + B3 / (nu2 * nu2 * nu2 * nu2);
}
pub unsafe extern "C" fn gsl_sf_bessel_Jnu_asymp_Olver_e(
    mut nu: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 || nu <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_olver.c\0" as *const u8 as *const libc::c_char,
            847 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut zeta: libc::c_double = 0.;
        let mut abs_zeta: libc::c_double = 0.;
        let mut arg: libc::c_double = 0.;
        let mut pre: libc::c_double = 0.;
        let mut asum: libc::c_double = 0.;
        let mut bsum: libc::c_double = 0.;
        let mut asum_err: libc::c_double = 0.;
        let mut ai: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut aip: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut z: libc::c_double = x / nu;
        let mut crnu: libc::c_double = pow(nu, 1.0f64 / 3.0f64);
        let mut nu3: libc::c_double = nu * nu * nu;
        let mut nu11: libc::c_double = nu3 * nu3 * nu3 * nu * nu;
        let mut stat_a: libc::c_int = 0;
        let mut stat_ap: libc::c_int = 0;
        if fabs(1.0f64 - z) < 0.02f64 {
            let a: libc::c_double = 1.0f64 - z;
            let c0: libc::c_double = 1.25992104989487316476721060728f64;
            let c1: libc::c_double = 0.37797631496846194943016318218f64;
            let c2: libc::c_double = 0.230385563409348235843147082474f64;
            let c3: libc::c_double = 0.165909603649648694839821892031f64;
            let c4: libc::c_double = 0.12931387086451008907f64;
            let c5: libc::c_double = 0.10568046188858133991f64;
            let c6: libc::c_double = 0.08916997952268186978f64;
            let c7: libc::c_double = 0.07700014900618802456f64;
            pre = c0
                + a
                    * (c1
                        + a * (c2 + a * (c3 + a * (c4 + a * (c5 + a * (c6 + a * c7))))));
            zeta = a * pre;
            pre = sqrt(2.0f64 * sqrt(pre / (1.0f64 + z)));
            abs_zeta = fabs(zeta);
        } else if z < 1.0f64 {
            let mut rt: libc::c_double = sqrt(1.0f64 - z * z);
            abs_zeta = pow(1.5f64 * (log((1.0f64 + rt) / z) - rt), 2.0f64 / 3.0f64);
            zeta = abs_zeta;
            pre = sqrt(2.0f64 * sqrt(abs_zeta / (rt * rt)));
        } else {
            let mut rt_0: libc::c_double = z * sqrt(1.0f64 - 1.0f64 / (z * z));
            abs_zeta = pow(1.5f64 * (rt_0 - acos(1.0f64 / z)), 2.0f64 / 3.0f64);
            zeta = -abs_zeta;
            pre = sqrt(2.0f64 * sqrt(abs_zeta / (rt_0 * rt_0)));
        }
        asum = olver_Asum(nu, z, abs_zeta, &mut asum_err);
        bsum = olver_Bsum(nu, z, abs_zeta);
        arg = crnu * crnu * zeta;
        stat_a = gsl_sf_airy_Ai_e(arg, 0 as libc::c_int as gsl_mode_t, &mut ai);
        stat_ap = gsl_sf_airy_Ai_deriv_e(arg, 0 as libc::c_int as gsl_mode_t, &mut aip);
        (*result)
            .val = pre * (ai.val * asum / crnu + aip.val * bsum / (nu * crnu * crnu));
        (*result).err = pre * (ai.err * fabs(asum / crnu));
        (*result).err += pre * fabs(ai.val) * asum_err / crnu;
        (*result).err += pre * fabs(ai.val * asum) / (crnu * nu11);
        (*result).err += 8.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_a != GSL_SUCCESS as libc::c_int {
            stat_a
        } else if stat_ap != GSL_SUCCESS as libc::c_int {
            stat_ap
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_Ynu_asymp_Olver_e(
    mut nu: libc::c_double,
    mut x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x <= 0.0f64 || nu <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_olver.c\0" as *const u8 as *const libc::c_char,
            922 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut zeta: libc::c_double = 0.;
        let mut abs_zeta: libc::c_double = 0.;
        let mut arg: libc::c_double = 0.;
        let mut pre: libc::c_double = 0.;
        let mut asum: libc::c_double = 0.;
        let mut bsum: libc::c_double = 0.;
        let mut asum_err: libc::c_double = 0.;
        let mut bi: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut bip: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut z: libc::c_double = x / nu;
        let mut crnu: libc::c_double = pow(nu, 1.0f64 / 3.0f64);
        let mut nu3: libc::c_double = nu * nu * nu;
        let mut nu11: libc::c_double = nu3 * nu3 * nu3 * nu * nu;
        let mut stat_b: libc::c_int = 0;
        let mut stat_d: libc::c_int = 0;
        if fabs(1.0f64 - z) < 0.02f64 {
            let a: libc::c_double = 1.0f64 - z;
            let c0: libc::c_double = 1.25992104989487316476721060728f64;
            let c1: libc::c_double = 0.37797631496846194943016318218f64;
            let c2: libc::c_double = 0.230385563409348235843147082474f64;
            let c3: libc::c_double = 0.165909603649648694839821892031f64;
            let c4: libc::c_double = 0.12931387086451008907f64;
            let c5: libc::c_double = 0.10568046188858133991f64;
            let c6: libc::c_double = 0.08916997952268186978f64;
            let c7: libc::c_double = 0.07700014900618802456f64;
            pre = c0
                + a
                    * (c1
                        + a * (c2 + a * (c3 + a * (c4 + a * (c5 + a * (c6 + a * c7))))));
            zeta = a * pre;
            pre = sqrt(2.0f64 * sqrt(pre / (1.0f64 + z)));
            abs_zeta = fabs(zeta);
        } else if z < 1.0f64 {
            let mut rt: libc::c_double = sqrt(1.0f64 - z * z);
            abs_zeta = pow(1.5f64 * (log((1.0f64 + rt) / z) - rt), 2.0f64 / 3.0f64);
            zeta = abs_zeta;
            pre = sqrt(2.0f64 * sqrt(abs_zeta / (rt * rt)));
        } else {
            let mut rt_0: libc::c_double = z * sqrt(1.0f64 - 1.0f64 / (z * z));
            let mut ac: libc::c_double = acos(1.0f64 / z);
            abs_zeta = pow(1.5f64 * (rt_0 - ac), 2.0f64 / 3.0f64);
            zeta = -abs_zeta;
            pre = sqrt(2.0f64 * sqrt(abs_zeta) / rt_0);
        }
        asum = olver_Asum(nu, z, abs_zeta, &mut asum_err);
        bsum = olver_Bsum(nu, z, abs_zeta);
        arg = crnu * crnu * zeta;
        stat_b = gsl_sf_airy_Bi_e(arg, 0 as libc::c_int as gsl_mode_t, &mut bi);
        stat_d = gsl_sf_airy_Bi_deriv_e(arg, 0 as libc::c_int as gsl_mode_t, &mut bip);
        (*result)
            .val = -pre * (bi.val * asum / crnu + bip.val * bsum / (nu * crnu * crnu));
        (*result).err = pre * (bi.err * fabs(asum / crnu));
        (*result).err += pre * fabs(bi.val) * asum_err / crnu;
        (*result).err += pre * fabs(bi.val * asum) / (crnu * nu11);
        (*result).err += 8.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return if stat_b != GSL_SUCCESS as libc::c_int {
            stat_b
        } else if stat_d != GSL_SUCCESS as libc::c_int {
            stat_d
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
