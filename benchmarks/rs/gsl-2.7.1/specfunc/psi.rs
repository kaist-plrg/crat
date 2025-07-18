use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn gsl_finite(x: libc::c_double) -> libc::c_int;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_exp_mult_err_e(
        x: libc::c_double,
        dx: libc::c_double,
        y: libc::c_double,
        dy: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_lnfact_e(n: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_hzeta_e(
        s: libc::c_double,
        q: libc::c_double,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_complex_add(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_sub(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_mul(a: gsl_complex, b: gsl_complex) -> gsl_complex;
    fn gsl_complex_add_real(a: gsl_complex, x: libc::c_double) -> gsl_complex;
    fn gsl_complex_mul_real(a: gsl_complex, x: libc::c_double) -> gsl_complex;
    fn gsl_complex_inverse(a: gsl_complex) -> gsl_complex;
    fn gsl_complex_log(a: gsl_complex) -> gsl_complex;
    fn gsl_complex_cot(a: gsl_complex) -> gsl_complex;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_complex {
    pub dat: [libc::c_double; 2],
}
#[inline]
unsafe extern "C" fn gsl_complex_rect(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> gsl_complex {
    let mut z: gsl_complex = gsl_complex { dat: [0.; 2] };
    z.dat[0 as libc::c_int as usize] = x;
    z.dat[1 as libc::c_int as usize] = y;
    return z;
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
static mut r1py_data: [libc::c_double; 30] = [
    1.59888328244976954803168395603f64,
    0.67905625353213463845115658455f64,
    -0.068485802980122530009506482524f64,
    -0.005788184183095866792008831182f64,
    0.008511258167108615980419855648f64,
    -0.004042656134699693434334556409f64,
    0.001352328406159402601778462956f64,
    -0.000311646563930660566674525382f64,
    0.000018507563785249135437219139f64,
    0.000028348705427529850296492146f64,
    -0.000019487536014574535567541960f64,
    8.0709788710834469408621587335e-06f64,
    -2.2983564321340518037060346561e-06f64,
    3.0506629599604749843855962658e-07f64,
    1.3042238632418364610774284846e-07f64,
    -1.2308657181048950589464690208e-07f64,
    5.7710855710682427240667414345e-08f64,
    -1.8275559342450963966092636354e-08f64,
    3.1020471300626589420759518930e-09f64,
    6.8989327480593812470039430640e-10f64,
    -8.7182290258923059852334818997e-10f64,
    4.4069147710243611798213548777e-10f64,
    -1.4727311099198535963467200277e-10f64,
    2.7589682523262644748825844248e-11f64,
    4.1871826756975856411554363568e-12f64,
    -6.5673460487260087541400767340e-12f64,
    3.4487900886723214020103638000e-12f64,
    -1.1807251417448690607973794078e-12f64,
    2.3798314343969589258709315574e-13f64,
    2.1663630410818831824259465821e-15f64,
];
static mut r1py_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: r1py_data.as_ptr() as *mut _,
            order: 29 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 18 as libc::c_int,
        };
        init
    }
};
static mut psics_data: [libc::c_double; 23] = [
    -0.038057080835217922f64,
    0.491415393029387130f64,
    -0.056815747821244730f64,
    0.008357821225914313f64,
    -0.001333232857994342f64,
    0.000220313287069308f64,
    -0.000037040238178456f64,
    0.000006283793654854f64,
    -0.000001071263908506f64,
    0.000000183128394654f64,
    -0.000000031353509361f64,
    0.000000005372808776f64,
    -0.000000000921168141f64,
    0.000000000157981265f64,
    -0.000000000027098646f64,
    0.000000000004648722f64,
    -0.000000000000797527f64,
    0.000000000000136827f64,
    -0.000000000000023475f64,
    0.000000000000004027f64,
    -0.000000000000000691f64,
    0.000000000000000118f64,
    -0.000000000000000020f64,
];
static mut psi_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: psics_data.as_ptr() as *mut _,
            order: 22 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 17 as libc::c_int,
        };
        init
    }
};
static mut apsics_data: [libc::c_double; 16] = [
    -0.0204749044678185f64,
    -0.0101801271534859f64,
    0.0000559718725387f64,
    -0.0000012917176570f64,
    0.0000000572858606f64,
    -0.0000000038213539f64,
    0.0000000003397434f64,
    -0.0000000000374838f64,
    0.0000000000048990f64,
    -0.0000000000007344f64,
    0.0000000000001233f64,
    -0.0000000000000228f64,
    0.0000000000000045f64,
    -0.0000000000000009f64,
    0.0000000000000002f64,
    -0.0000000000000000f64,
];
static mut apsi_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: apsics_data.as_ptr() as *mut _,
            order: 15 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut psi_table: [libc::c_double; 101] = [
    0.0f64,
    -0.57721566490153286060651209008f64,
    0.42278433509846713939348790992f64,
    0.92278433509846713939348790992f64,
    1.25611766843180047272682124325f64,
    1.50611766843180047272682124325f64,
    1.70611766843180047272682124325f64,
    1.87278433509846713939348790992f64,
    2.01564147795560999653634505277f64,
    2.14064147795560999653634505277f64,
    2.25175258906672110764745616389f64,
    2.35175258906672110764745616389f64,
    2.44266167997581201673836525479f64,
    2.52599501330914535007169858813f64,
    2.60291809023222227314862166505f64,
    2.67434666166079370172005023648f64,
    2.74101332832746036838671690315f64,
    2.80351332832746036838671690315f64,
    2.86233685773922507426906984432f64,
    2.91789241329478062982462539988f64,
    2.97052399224214905087725697883f64,
    3.02052399224214905087725697883f64,
    3.06814303986119666992487602645f64,
    3.11359758531574212447033057190f64,
    3.15707584618530734186163491973f64,
    3.1987425128519740085283015864f64,
    3.2387425128519740085283015864f64,
    3.2772040513135124700667631249f64,
    3.3142410883505495071038001619f64,
    3.3499553740648352213895144476f64,
    3.3844381326855248765619282407f64,
    3.4177714660188582098952615740f64,
    3.4500295305349872421533260902f64,
    3.4812795305349872421533260902f64,
    3.5115825608380175451836291205f64,
    3.5409943255438998981248055911f64,
    3.5695657541153284695533770196f64,
    3.5973435318931062473311547974f64,
    3.6243705589201332743581818244f64,
    3.6506863483938174848844976139f64,
    3.6763273740348431259101386396f64,
    3.7013273740348431259101386396f64,
    3.7257176179372821503003825420f64,
    3.7495271417468059598241920658f64,
    3.7727829557002943319172153216f64,
    3.7955102284275670591899425943f64,
    3.8177324506497892814121648166f64,
    3.8394715810845718901078169905f64,
    3.8607481768292527411716467777f64,
    3.8815815101625860745049801110f64,
    3.9019896734278921969539597029f64,
    3.9219896734278921969539597029f64,
    3.9415975165651470989147440166f64,
    3.9608282857959163296839747858f64,
    3.9796962103242182164764276160f64,
    3.9982147288427367349949461345f64,
    4.0163965470245549168131279527f64,
    4.0342536898816977739559850956f64,
    4.0517975495308205809735289552f64,
    4.0690389288411654085597358518f64,
    4.0859880813835382899156680552f64,
    4.1026547480502049565823347218f64,
    4.1190481906731557762544658694f64,
    4.1351772229312202923834981274f64,
    4.1510502388042361653993711433f64,
    4.1666752388042361653993711433f64,
    4.1820598541888515500147557587f64,
    4.1972113693403667015299072739f64,
    4.2121367424746950597388624977f64,
    4.2268426248276362362094507330f64,
    4.2413353784508246420065521823f64,
    4.2556210927365389277208378966f64,
    4.2697055997787924488475984600f64,
    4.2835944886676813377364873489f64,
    4.2972931188046676391063503626f64,
    4.3108066323181811526198638761f64,
    4.3241399656515144859531972094f64,
    4.3372978603883565912163551041f64,
    4.3502848733753695782293421171f64,
    4.3631053861958823987421626300f64,
    4.3757636140439836645649474401f64,
    4.3882636140439836645649474401f64,
    4.4006092930563293435772931191f64,
    4.4128044150075488557724150703f64,
    4.4248526077786331931218126607f64,
    4.4367573696833950978837174226f64,
    4.4485220755657480390601880108f64,
    4.4601499825424922251066996387f64,
    4.4716442354160554434975042364f64,
    4.4830078717796918071338678728f64,
    4.4942438268358715824147667492f64,
    4.5053549379469826935258778603f64,
    4.5163439489359936825368668713f64,
    4.5272135141533849868846929582f64,
    4.5379662023254279976373811303f64,
    4.5486045001977684231692960239f64,
    4.5591308159872421073798223397f64,
    4.5695474826539087740464890064f64,
    4.5798567610044242379640147796f64,
    4.5900608426370772991885045755f64,
    4.6001618527380874001986055856f64,
];
static mut psi_1_table: [libc::c_double; 101] = [
    0.0f64,
    3.14159265358979323846f64 * 3.14159265358979323846f64 / 6.0f64,
    0.644934066848226436472415f64,
    0.394934066848226436472415f64,
    0.2838229557371153253613041f64,
    0.2213229557371153253613041f64,
    0.1813229557371153253613041f64,
    0.1535451779593375475835263f64,
    0.1331370146940314251345467f64,
    0.1175120146940314251345467f64,
    0.1051663356816857461222010f64,
    0.0951663356816857461222010f64,
    0.0869018728717683907503002f64,
    0.0799574284273239463058557f64,
    0.0740402686640103368384001f64,
    0.0689382278476838062261552f64,
    0.0644937834032393617817108f64,
    0.0605875334032393617817108f64,
    0.0571273257907826143768665f64,
    0.0540409060376961946237801f64,
    0.0512708229352031198315363f64,
    0.0487708229352031198315363f64,
    0.0465032492390579951149830f64,
    0.0444371335365786562720078f64,
    0.0425467743683366902984728f64,
    0.0408106632572255791873617f64,
    0.0392106632572255791873617f64,
    0.0377313733163971768204978f64,
    0.0363596312039143235969038f64,
    0.0350841209998326909438426f64,
    0.0338950603577399442137594f64,
    0.0327839492466288331026483f64,
    0.0317433665203020901265817f64,
    0.03076680402030209012658168f64,
    0.02984853037475571730748159f64,
    0.02898347847164153045627052f64,
    0.02816715194102928555831133f64,
    0.02739554700275768062003973f64,
    0.02666508681283803124093089f64,
    0.02597256603721476254286995f64,
    0.02531510384129102815759710f64,
    0.02469010384129102815759710f64,
    0.02409521984367056414807896f64,
    0.02352832641963428296894063f64,
    0.02298749353699501850166102f64,
    0.02247096461137518379091722f64,
    0.02197713745088135663042339f64,
    0.02150454765882086513703965f64,
    0.02105185413233829383780923f64,
    0.02061782635456051606003145f64,
    0.02020133322669712580597065f64,
    0.01980133322669712580597065f64,
    0.01941686571420193164987683f64,
    0.01904704322899483105816086f64,
    0.01869104465298913508094477f64,
    0.01834810912486842177504628f64,
    0.01801753061247172756017024f64,
    0.01769865306145131939690494f64,
    0.01739086605006319997554452f64,
    0.01709360088954001329302371f64,
    0.01680632711763538818529605f64,
    0.01652854933985761040751827f64,
    0.01625980437882562975715546f64,
    0.01599965869724394401313881f64,
    0.01574770606433893015574400f64,
    0.01550356543933893015574400f64,
    0.01526687904880638577704578f64,
    0.01503731063741979257227076f64,
    0.01481454387422086185273411f64,
    0.01459828089844231513993134f64,
    0.01438824099085987447620523f64,
    0.01418415935820681325171544f64,
    0.01398578601958352422176106f64,
    0.01379288478501562298719316f64,
    0.01360523231738567365335942f64,
    0.01342261726990576130858221f64,
    0.01324483949212798353080444f64,
    0.01307170929822216635628920f64,
    0.01290304679189732236910755f64,
    0.01273868124291638877278934f64,
    0.01257845051066194236996928f64,
    0.01242220051066194236996928f64,
    0.01226978472038606978956995f64,
    0.01212106372098095378719041f64,
    0.01197590477193174490346273f64,
    0.01183418141592267460867815f64,
    0.01169577311142440471248438f64,
    0.01156056489076458859566448f64,
    0.01142844704164317229232189f64,
    0.01129931481023821361463594f64,
    0.01117306812421372175754719f64,
    0.01104961133409026496742374f64,
    0.01092885297157366069257770f64,
    0.01081070552355853781923177f64,
    0.01069508522063334415522437f64,
    0.01058191183901270133041676f64,
    0.01047110851491297833872701f64,
    0.01036260157046853389428257f64,
    0.01025632035036012704977199f64,
    0.01015219706839427948625679f64,
    0.01005016666333357139524567f64,
];
unsafe extern "C" fn psi_x(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let y: libc::c_double = fabs(x);
    if x == 0.0f64 || x == -1.0f64 || x == -2.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            385 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if y >= 2.0f64 {
        let t: libc::c_double = 8.0f64 / (y * y) - 1.0f64;
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut apsi_cs, t, &mut result_c);
        if x < 0.0f64 {
            let s: libc::c_double = sin(3.14159265358979323846f64 * x);
            let c: libc::c_double = cos(3.14159265358979323846f64 * x);
            if fabs(s) < 2.0f64 * 1.4916681462400413e-154f64 {
                (*result).val = ::std::f32::NAN as libc::c_double;
                (*result).err = ::std::f32::NAN as libc::c_double;
                gsl_error(
                    b"domain error\0" as *const u8 as *const libc::c_char,
                    b"psi.c\0" as *const u8 as *const libc::c_char,
                    395 as libc::c_int,
                    GSL_EDOM as libc::c_int,
                );
                return GSL_EDOM as libc::c_int;
            } else {
                (*result)
                    .val = log(y) - 0.5f64 / x + result_c.val
                    - 3.14159265358979323846f64 * c / s;
                (*result)
                    .err = 3.14159265358979323846f64 * fabs(x)
                    * 2.2204460492503131e-16f64 / (s * s);
                (*result).err += result_c.err;
                (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
                return GSL_SUCCESS as libc::c_int;
            }
        } else {
            (*result).val = log(y) - 0.5f64 / x + result_c.val;
            (*result).err = result_c.err;
            (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        }
    } else {
        let mut result_c_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        if x < -1.0f64 {
            let v: libc::c_double = x + 2.0f64;
            let t1: libc::c_double = 1.0f64 / x;
            let t2: libc::c_double = 1.0f64 / (x + 1.0f64);
            let t3: libc::c_double = 1.0f64 / v;
            cheb_eval_e(&mut psi_cs, 2.0f64 * v - 1.0f64, &mut result_c_0);
            (*result).val = -(t1 + t2 + t3) + result_c_0.val;
            (*result)
                .err = 2.2204460492503131e-16f64
                * (fabs(t1) + fabs(x / (t2 * t2)) + fabs(x / (t3 * t3)));
            (*result).err += result_c_0.err;
            (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else if x < 0.0f64 {
            let v_0: libc::c_double = x + 1.0f64;
            let t1_0: libc::c_double = 1.0f64 / x;
            let t2_0: libc::c_double = 1.0f64 / v_0;
            cheb_eval_e(&mut psi_cs, 2.0f64 * v_0 - 1.0f64, &mut result_c_0);
            (*result).val = -(t1_0 + t2_0) + result_c_0.val;
            (*result)
                .err = 2.2204460492503131e-16f64
                * (fabs(t1_0) + fabs(x / (t2_0 * t2_0)));
            (*result).err += result_c_0.err;
            (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else if x < 1.0f64 {
            let t1_1: libc::c_double = 1.0f64 / x;
            cheb_eval_e(&mut psi_cs, 2.0f64 * x - 1.0f64, &mut result_c_0);
            (*result).val = -t1_1 + result_c_0.val;
            (*result).err = 2.2204460492503131e-16f64 * t1_1;
            (*result).err += result_c_0.err;
            (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
            return GSL_SUCCESS as libc::c_int;
        } else {
            let v_1: libc::c_double = x - 1.0f64;
            return cheb_eval_e(&mut psi_cs, 2.0f64 * v_1 - 1.0f64, result);
        }
    };
}
unsafe extern "C" fn psi_complex_asymp(mut z: gsl_complex) -> gsl_complex {
    static mut c1: libc::c_double = -0.1f64;
    static mut c2: libc::c_double = 1.0f64 / 21.0f64;
    static mut c3: libc::c_double = -0.05f64;
    let mut zi: gsl_complex = gsl_complex_inverse(z);
    let mut w: gsl_complex = gsl_complex_mul(zi, zi);
    let mut cs: gsl_complex = gsl_complex { dat: [0.; 2] };
    let mut sum: gsl_complex = gsl_complex { dat: [0.; 2] };
    sum = gsl_complex_mul_real(w, c3 / c2);
    sum = gsl_complex_add_real(sum, 1.0f64);
    sum = gsl_complex_mul_real(sum, c2 / c1);
    sum = gsl_complex_mul(sum, w);
    sum = gsl_complex_add_real(sum, 1.0f64);
    sum = gsl_complex_mul_real(sum, c1);
    sum = gsl_complex_mul(sum, w);
    sum = gsl_complex_add_real(sum, 1.0f64);
    cs = gsl_complex_mul(sum, w);
    cs = gsl_complex_mul_real(cs, -1.0f64 / 12.0f64);
    cs = gsl_complex_add(cs, gsl_complex_mul_real(zi, -0.5f64));
    return gsl_complex_add(gsl_complex_log(z), cs);
}
unsafe extern "C" fn psi_complex_rhp(
    mut z: gsl_complex,
    mut result_re: *mut gsl_sf_result,
    mut result_im: *mut gsl_sf_result,
) -> libc::c_int {
    let mut n_recurse: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut a: gsl_complex = gsl_complex { dat: [0.; 2] };
    if z.dat[0 as libc::c_int as usize] == 0.0f64
        && z.dat[1 as libc::c_int as usize] == 0.0f64
    {
        (*result_re).val = 0.0f64;
        (*result_im).val = 0.0f64;
        (*result_re).err = 0.0f64;
        (*result_im).err = 0.0f64;
        return GSL_EDOM as libc::c_int;
    }
    if z.dat[0 as libc::c_int as usize] < 20.0f64
        && fabs(z.dat[1 as libc::c_int as usize]) < 20.0f64
    {
        let sp: libc::c_double = sqrt(20.0f64 + z.dat[1 as libc::c_int as usize]);
        let sn: libc::c_double = sqrt(20.0f64 - z.dat[1 as libc::c_int as usize]);
        let rhs: libc::c_double = sp * sn - z.dat[0 as libc::c_int as usize];
        if rhs > 0.0f64 {
            n_recurse = ceil(rhs) as libc::c_int;
        }
    }
    a = psi_complex_asymp(gsl_complex_add_real(z, n_recurse as libc::c_double));
    (*result_re)
        .err = 2.0f64 * 2.2204460492503131e-16f64
        * fabs(a.dat[0 as libc::c_int as usize]);
    (*result_im)
        .err = 2.0f64 * 2.2204460492503131e-16f64
        * fabs(a.dat[1 as libc::c_int as usize]);
    i = n_recurse;
    while i >= 1 as libc::c_int {
        let mut zn: gsl_complex = gsl_complex_add_real(z, i as libc::c_double - 1.0f64);
        let mut zn_inverse: gsl_complex = gsl_complex_inverse(zn);
        a = gsl_complex_sub(a, zn_inverse);
        (*result_re).err
            += 2.0f64 * 2.2204460492503131e-16f64
                * fabs(zn_inverse.dat[0 as libc::c_int as usize]);
        (*result_im).err
            += 2.0f64 * 2.2204460492503131e-16f64
                * fabs(zn_inverse.dat[1 as libc::c_int as usize]);
        i -= 1;
        i;
    }
    (*result_re).val = a.dat[0 as libc::c_int as usize];
    (*result_im).val = a.dat[1 as libc::c_int as usize];
    (*result_re).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result_re).val);
    (*result_im).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result_im).val);
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn psi_n_xg0(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n == 0 as libc::c_int {
        return gsl_sf_psi_e(x, result)
    } else {
        let mut ln_nf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut hzeta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_hz: libc::c_int = gsl_sf_hzeta_e(
            n as libc::c_double + 1.0f64,
            x,
            &mut hzeta,
        );
        let mut stat_nf: libc::c_int = gsl_sf_lnfact_e(n as libc::c_uint, &mut ln_nf);
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            ln_nf.val,
            ln_nf.err,
            hzeta.val,
            hzeta.err,
            result,
        );
        if n & 1 as libc::c_int == 0 {
            (*result).val = -(*result).val;
        }
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_nf != GSL_SUCCESS as libc::c_int {
            stat_nf
        } else if stat_hz != GSL_SUCCESS as libc::c_int {
            stat_hz
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_psi_int_e(
    n: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n <= 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            587 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n <= 100 as libc::c_int {
        (*result).val = psi_table[n as usize];
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let c2: libc::c_double = -1.0f64 / 12.0f64;
        let c3: libc::c_double = 1.0f64 / 120.0f64;
        let c4: libc::c_double = -1.0f64 / 252.0f64;
        let c5: libc::c_double = 1.0f64 / 240.0f64;
        let ni2: libc::c_double = 1.0f64 / n as libc::c_double
            * (1.0f64 / n as libc::c_double);
        let ser: libc::c_double = ni2 * (c2 + ni2 * (c3 + ni2 * (c4 + ni2 * c5)));
        (*result).val = log(n as libc::c_double) - 0.5f64 / n as libc::c_double + ser;
        (*result)
            .err = 2.2204460492503131e-16f64
            * (fabs(log(n as libc::c_double)) + fabs(0.5f64 / n as libc::c_double)
                + fabs(ser));
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_psi_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    return psi_x(x, result);
}
pub unsafe extern "C" fn gsl_sf_psi_1piy_e(
    y: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let ay: libc::c_double = fabs(y);
    if ay > 1000.0f64 {
        let yi2: libc::c_double = 1.0f64 / (ay * ay);
        let lny: libc::c_double = log(ay);
        let sum: libc::c_double = yi2
            * (1.0f64 / 12.0f64 + 1.0f64 / 120.0f64 * yi2
                + 1.0f64 / 252.0f64 * yi2 * yi2);
        (*result).val = lny + sum;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (fabs(lny) + fabs(sum));
        return GSL_SUCCESS as libc::c_int;
    } else if ay > 10.0f64 {
        let yi2_0: libc::c_double = 1.0f64 / (ay * ay);
        let lny_0: libc::c_double = log(ay);
        let sum_0: libc::c_double = yi2_0
            * (1.0f64 / 12.0f64
                + yi2_0
                    * (1.0f64 / 120.0f64
                        + yi2_0
                            * (1.0f64 / 252.0f64
                                + yi2_0
                                    * (1.0f64 / 240.0f64
                                        + yi2_0
                                            * (1.0f64 / 132.0f64 + 691.0f64 / 32760.0f64 * yi2_0)))));
        (*result).val = lny_0 + sum_0;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * (fabs(lny_0) + fabs(sum_0));
        return GSL_SUCCESS as libc::c_int;
    } else if ay > 1.0f64 {
        let y2: libc::c_double = ay * ay;
        let x: libc::c_double = (2.0f64 * ay - 11.0f64) / 9.0f64;
        let v: libc::c_double = y2 * (1.0f64 / (1.0f64 + y2) + 0.5f64 / (4.0f64 + y2));
        let mut result_c: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_e(&mut r1py_cs, x, &mut result_c);
        (*result).val = result_c.val - 0.57721566490153286060651209008f64 + v;
        (*result).err = result_c.err;
        (*result).err
            += 2.0f64 * 2.2204460492503131e-16f64
                * (fabs(v) + 0.57721566490153286060651209008f64 + fabs(result_c.val));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        (*result).err *= 5.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let M: libc::c_int = 50 as libc::c_int;
        let y2_0: libc::c_double = y * y;
        let c0: libc::c_double = 0.00019603999466879846570f64;
        let c2: libc::c_double = 3.8426659205114376860e-08f64;
        let c4: libc::c_double = 1.0041592839497643554e-11f64;
        let c6: libc::c_double = 2.9516743763500191289e-15f64;
        let p: libc::c_double = c0 + y2_0 * (-c2 + y2_0 * (c4 - y2_0 * c6));
        let mut sum_1: libc::c_double = 0.0f64;
        let mut v_0: libc::c_double = 0.;
        let mut n: libc::c_int = 0;
        n = 1 as libc::c_int;
        while n <= M {
            sum_1
                += 1.0f64 / (n as libc::c_double * ((n * n) as libc::c_double + y * y));
            n += 1;
            n;
        }
        v_0 = y2_0 * (sum_1 + p);
        (*result).val = -0.57721566490153286060651209008f64 + v_0;
        (*result)
            .err = 2.2204460492503131e-16f64
            * (0.57721566490153286060651209008f64 + fabs(v_0));
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_psi_1_int_e(
    n: libc::c_int,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n <= 0 as libc::c_int {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            699 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if n <= 100 as libc::c_int {
        (*result).val = psi_1_table[n as usize];
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    } else {
        let c0: libc::c_double = -1.0f64 / 30.0f64;
        let c1: libc::c_double = 1.0f64 / 42.0f64;
        let c2: libc::c_double = -1.0f64 / 30.0f64;
        let ni2: libc::c_double = 1.0f64 / n as libc::c_double
            * (1.0f64 / n as libc::c_double);
        let ser: libc::c_double = ni2 * ni2 * (c0 + ni2 * (c1 + c2 * ni2));
        (*result)
            .val = (1.0f64 + 0.5f64 / n as libc::c_double
            + 1.0f64 / (6.0f64 * n as libc::c_double * n as libc::c_double) + ser)
            / n as libc::c_double;
        (*result).err = 2.2204460492503131e-16f64 * (*result).val;
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_psi_1_e(
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x == 0.0f64 || x == -1.0f64 || x == -2.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            727 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if x > 0.0f64 {
        return psi_n_xg0(1 as libc::c_int, x, result)
    } else if x > -5.0f64 {
        let mut M: libc::c_int = -floor(x) as libc::c_int;
        let mut fx: libc::c_double = x + M as libc::c_double;
        let mut sum: libc::c_double = 0.0f64;
        let mut m: libc::c_int = 0;
        if fx == 0.0f64 {
            (*result).val = ::std::f32::NAN as libc::c_double;
            (*result).err = ::std::f32::NAN as libc::c_double;
            gsl_error(
                b"domain error\0" as *const u8 as *const libc::c_char,
                b"psi.c\0" as *const u8 as *const libc::c_char,
                742 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
        m = 0 as libc::c_int;
        while m < M {
            sum += 1.0f64 / ((x + m as libc::c_double) * (x + m as libc::c_double));
            m += 1;
            m;
        }
        let mut stat_psi: libc::c_int = psi_n_xg0(1 as libc::c_int, fx, result);
        (*result).val += sum;
        (*result).err += M as libc::c_double * 2.2204460492503131e-16f64 * sum;
        return stat_psi;
    } else {
        let sin_px: libc::c_double = sin(3.14159265358979323846f64 * x);
        let d: libc::c_double = 3.14159265358979323846f64 * 3.14159265358979323846f64
            / (sin_px * sin_px);
        let mut r: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_psi_0: libc::c_int = psi_n_xg0(
            1 as libc::c_int,
            1.0f64 - x,
            &mut r,
        );
        (*result).val = d - r.val;
        (*result).err = r.err + 2.0f64 * 2.2204460492503131e-16f64 * d;
        return stat_psi_0;
    };
}
pub unsafe extern "C" fn gsl_sf_psi_n_e(
    n: libc::c_int,
    x: libc::c_double,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if n == 0 as libc::c_int {
        return gsl_sf_psi_e(x, result)
    } else if n == 1 as libc::c_int {
        return gsl_sf_psi_1_e(x, result)
    } else if n < 0 as libc::c_int || x <= 0.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            781 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut ln_nf: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut hzeta: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut stat_hz: libc::c_int = gsl_sf_hzeta_e(
            n as libc::c_double + 1.0f64,
            x,
            &mut hzeta,
        );
        let mut stat_nf: libc::c_int = gsl_sf_lnfact_e(n as libc::c_uint, &mut ln_nf);
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            ln_nf.val,
            ln_nf.err,
            hzeta.val,
            hzeta.err,
            result,
        );
        if n & 1 as libc::c_int == 0 {
            (*result).val = -(*result).val;
        }
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_nf != GSL_SUCCESS as libc::c_int {
            stat_nf
        } else if stat_hz != GSL_SUCCESS as libc::c_int {
            stat_hz
        } else {
            GSL_SUCCESS as libc::c_int
        };
    };
}
pub unsafe extern "C" fn gsl_sf_complex_psi_e(
    x: libc::c_double,
    y: libc::c_double,
    mut result_re: *mut gsl_sf_result,
    mut result_im: *mut gsl_sf_result,
) -> libc::c_int {
    if x >= 0.0f64 {
        let mut z: gsl_complex = gsl_complex_rect(x, y);
        return psi_complex_rhp(z, result_re, result_im);
    } else {
        let mut z_0: gsl_complex = gsl_complex_rect(x, y);
        let mut omz: gsl_complex = gsl_complex_rect(1.0f64 - x, -y);
        let mut zpi: gsl_complex = gsl_complex_mul_real(z_0, 3.14159265358979323846f64);
        let mut cotzpi: gsl_complex = gsl_complex_cot(zpi);
        let mut ret_val: libc::c_int = psi_complex_rhp(omz, result_re, result_im);
        if gsl_finite(cotzpi.dat[0 as libc::c_int as usize]) != 0
            && gsl_finite(cotzpi.dat[1 as libc::c_int as usize]) != 0
        {
            (*result_re).val
                -= 3.14159265358979323846f64 * cotzpi.dat[0 as libc::c_int as usize];
            (*result_im).val
                -= 3.14159265358979323846f64 * cotzpi.dat[1 as libc::c_int as usize];
            return ret_val;
        } else {
            gsl_error(
                b"singularity\0" as *const u8 as *const libc::c_char,
                b"psi.c\0" as *const u8 as *const libc::c_char,
                827 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn gsl_sf_psi_int(n: libc::c_int) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_psi_int_e(n, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_psi_int_e(n, &result)\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            840 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_psi(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_psi_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_psi_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            845 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_psi_1piy(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_psi_1piy_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_psi_1piy_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            850 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_psi_1_int(n: libc::c_int) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_psi_1_int_e(n, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_psi_1_int_e(n, &result)\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            855 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_psi_1(x: libc::c_double) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_psi_1_e(x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_psi_1_e(x, &result)\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            860 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_psi_n(
    n: libc::c_int,
    x: libc::c_double,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_psi_n_e(n, x, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_psi_n_e(n, x, &result)\0" as *const u8 as *const libc::c_char,
            b"psi.c\0" as *const u8 as *const libc::c_char,
            865 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
