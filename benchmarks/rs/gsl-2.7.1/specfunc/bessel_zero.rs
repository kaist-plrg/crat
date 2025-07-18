use ::libc;
extern "C" {
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_sf_airy_zero_Ai_e(s: libc::c_uint, result: *mut gsl_sf_result) -> libc::c_int;
    fn gsl_sf_pow_int_e(
        x: libc::c_double,
        n: libc::c_int,
        result: *mut gsl_sf_result,
    ) -> libc::c_int;
    fn gsl_sf_bessel_Olver_zofmzeta(minus_zeta: libc::c_double) -> libc::c_double;
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
pub struct gsl_sf_result_struct {
    pub val: libc::c_double,
    pub err: libc::c_double,
}
pub type gsl_sf_result = gsl_sf_result_struct;
static mut coef_jnu1_a: [libc::c_double; 24] = [
    3.801775243633476f64,
    1.360704737511120f64,
    -0.030707710261106f64,
    0.004526823746202f64,
    -0.000808682832134f64,
    0.000159218792489f64,
    -0.000033225189761f64,
    0.000007205599763f64,
    -0.000001606110397f64,
    0.000000365439424f64,
    -0.000000084498039f64,
    0.000000019793815f64,
    -0.000000004687054f64,
    0.000000001120052f64,
    -0.000000000269767f64,
    0.000000000065420f64,
    -0.000000000015961f64,
    0.000000000003914f64,
    -0.000000000000965f64,
    0.000000000000239f64,
    -0.000000000000059f64,
    0.000000000000015f64,
    -0.000000000000004f64,
    0.000000000000001f64,
];
static mut coef_jnu1_b: [libc::c_double; 15] = [
    1.735063412537096f64,
    0.784478100951978f64,
    0.048881473180370f64,
    -0.000578279783021f64,
    -0.000038984957864f64,
    0.000005758297879f64,
    -0.000000327583229f64,
    -0.000000003853878f64,
    0.000000002284653f64,
    -0.000000000153079f64,
    -0.000000000000895f64,
    0.000000000000283f64,
    0.000000000000043f64,
    0.000000000000010f64,
    -0.000000000000003f64,
];
static mut coef_jnu2_a: [libc::c_double; 19] = [
    6.992370244046161f64,
    1.446379282056534f64,
    -0.023458616207293f64,
    0.002172149448700f64,
    -0.000246262775620f64,
    0.000030990180959f64,
    -0.000004154183047f64,
    0.000000580766328f64,
    -0.000000083648175f64,
    0.000000012317355f64,
    -0.000000001844887f64,
    0.000000000280076f64,
    -0.000000000042986f64,
    0.000000000006658f64,
    -0.000000000001039f64,
    0.000000000000163f64,
    -0.000000000000026f64,
    0.000000000000004f64,
    -0.000000000000001f64,
];
static mut coef_jnu2_b: [libc::c_double; 19] = [
    2.465611864263400f64,
    1.607952988471069f64,
    0.138758034431497f64,
    -0.003687791182054f64,
    -0.000051276007868f64,
    0.000045113570749f64,
    -0.000007579172152f64,
    0.000000736469208f64,
    -0.000000011118527f64,
    -0.000000011919884f64,
    0.000000002696788f64,
    -0.000000000314488f64,
    0.000000000008124f64,
    0.000000000005211f64,
    -0.000000000001292f64,
    0.000000000000158f64,
    -0.000000000000004f64,
    -0.000000000000003f64,
    0.000000000000001f64,
];
static mut coef_jnu3_a: [libc::c_double; 19] = [
    10.869647065239236f64,
    2.177524286141710f64,
    -0.034822817125293f64,
    0.003167249102413f64,
    -0.000353960349344f64,
    0.000044039086085f64,
    -0.000005851380981f64,
    0.000000812575483f64,
    -0.000000116463617f64,
    0.000000017091246f64,
    -0.000000002554376f64,
    0.000000000387335f64,
    -0.000000000059428f64,
    0.000000000009207f64,
    -0.000000000001438f64,
    0.000000000000226f64,
    -0.000000000000036f64,
    0.000000000000006f64,
    -0.000000000000001f64,
];
static mut coef_jnu3_b: [libc::c_double; 19] = [
    2.522816775173244f64,
    1.673199424973720f64,
    0.146431617506314f64,
    -0.004049001763912f64,
    -0.000039517767244f64,
    0.000048781729288f64,
    -0.000008729705695f64,
    0.000000928737310f64,
    -0.000000028388244f64,
    -0.000000012927432f64,
    0.000000003441008f64,
    -0.000000000471695f64,
    0.000000000025590f64,
    0.000000000005502f64,
    -0.000000000001881f64,
    0.000000000000295f64,
    -0.000000000000020f64,
    -0.000000000000003f64,
    0.000000000000001f64,
];
static mut coef_jnu4_a: [libc::c_double; 19] = [
    14.750310252773009f64,
    2.908010932941708f64,
    -0.046093293420315f64,
    0.004147172321412f64,
    -0.000459092310473f64,
    0.000056646951906f64,
    -0.000007472351546f64,
    0.000001031210065f64,
    -0.000000147008137f64,
    0.000000021475218f64,
    -0.000000003197208f64,
    0.000000000483249f64,
    -0.000000000073946f64,
    0.000000000011431f64,
    -0.000000000001782f64,
    0.000000000000280f64,
    -0.000000000000044f64,
    0.000000000000007f64,
    -0.000000000000001f64,
];
static mut coef_jnu4_b: [libc::c_double; 19] = [
    2.551681323117914f64,
    1.706177978336572f64,
    0.150357658406131f64,
    -0.004234001378590f64,
    -0.000033854229898f64,
    0.000050763551485f64,
    -0.000009337464057f64,
    0.000001029717834f64,
    -0.000000037474196f64,
    -0.000000013450153f64,
    0.000000003836180f64,
    -0.000000000557404f64,
    0.000000000035748f64,
    0.000000000005487f64,
    -0.000000000002187f64,
    0.000000000000374f64,
    -0.000000000000031f64,
    -0.000000000000003f64,
    0.000000000000001f64,
];
static mut coef_jnu5_a: [libc::c_double; 19] = [
    18.632261081028211f64,
    3.638249012596966f64,
    -0.057329705998828f64,
    0.005121709126820f64,
    -0.000563325259487f64,
    0.000069100826174f64,
    -0.000009066603030f64,
    0.000001245181383f64,
    -0.000000176737282f64,
    0.000000025716695f64,
    -0.000000003815184f64,
    0.000000000574839f64,
    -0.000000000087715f64,
    0.000000000013526f64,
    -0.000000000002104f64,
    0.000000000000330f64,
    -0.000000000000052f64,
    0.000000000000008f64,
    -0.000000000000001f64,
];
static mut coef_jnu5_b: [libc::c_double; 19] = [
    2.569079487591442f64,
    1.726073360882134f64,
    0.152740776809531f64,
    -0.004346449660148f64,
    -0.000030512461856f64,
    0.000052000821080f64,
    -0.000009713343981f64,
    0.000001091997863f64,
    -0.000000043061707f64,
    -0.000000013779413f64,
    0.000000004082870f64,
    -0.000000000611259f64,
    0.000000000042242f64,
    0.000000000005448f64,
    -0.000000000002377f64,
    0.000000000000424f64,
    -0.000000000000038f64,
    -0.000000000000002f64,
    0.000000000000002f64,
];
static mut coef_jnu6_a: [libc::c_double; 19] = [
    22.514836143374042f64,
    4.368367257557198f64,
    -0.068550155285562f64,
    0.006093776505822f64,
    -0.000667152784957f64,
    0.000081486022398f64,
    -0.000010649011647f64,
    0.000001457089679f64,
    -0.000000206105082f64,
    0.000000029894724f64,
    -0.000000004422012f64,
    0.000000000664471f64,
    -0.000000000101140f64,
    0.000000000015561f64,
    -0.000000000002416f64,
    0.000000000000378f64,
    -0.000000000000060f64,
    0.000000000000009f64,
    -0.000000000000002f64,
];
static mut coef_jnu6_b: [libc::c_double; 19] = [
    2.580710285494837f64,
    1.739380728566154f64,
    0.154340696401691f64,
    -0.004422028860168f64,
    -0.000028305272624f64,
    0.000052845975269f64,
    -0.000009968794373f64,
    0.000001134252926f64,
    -0.000000046841241f64,
    -0.000000014007555f64,
    0.000000004251816f64,
    -0.000000000648213f64,
    0.000000000046728f64,
    0.000000000005414f64,
    -0.000000000002508f64,
    0.000000000000459f64,
    -0.000000000000043f64,
    -0.000000000000002f64,
    0.000000000000002f64,
];
static mut coef_jnu7_a: [libc::c_double; 19] = [
    26.397760539730869f64,
    5.098418721711790f64,
    -0.079761896398948f64,
    0.007064521280487f64,
    -0.000770766522482f64,
    0.000093835449636f64,
    -0.000012225308542f64,
    0.000001667939800f64,
    -0.000000235288157f64,
    0.000000034040347f64,
    -0.000000005023142f64,
    0.000000000753101f64,
    -0.000000000114389f64,
    0.000000000017564f64,
    -0.000000000002722f64,
    0.000000000000425f64,
    -0.000000000000067f64,
    0.000000000000011f64,
    -0.000000000000002f64,
];
static mut coef_jnu7_b: [libc::c_double; 19] = [
    2.589033335856773f64,
    1.748907007612678f64,
    0.155488900387653f64,
    -0.004476317805688f64,
    -0.000026737952924f64,
    0.000053459680946f64,
    -0.000010153699240f64,
    0.000001164804272f64,
    -0.000000049566917f64,
    -0.000000014175403f64,
    0.000000004374840f64,
    -0.000000000675135f64,
    0.000000000050004f64,
    0.000000000005387f64,
    -0.000000000002603f64,
    0.000000000000485f64,
    -0.000000000000047f64,
    -0.000000000000002f64,
    0.000000000000002f64,
];
static mut coef_jnu8_a: [libc::c_double; 19] = [
    30.280900001606662f64,
    5.828429205461221f64,
    -0.090968381181069f64,
    0.008034479731033f64,
    -0.000874254899080f64,
    0.000106164151611f64,
    -0.000013798098749f64,
    0.000001878187386f64,
    -0.000000264366627f64,
    0.000000038167685f64,
    -0.000000005621060f64,
    0.000000000841165f64,
    -0.000000000127538f64,
    0.000000000019550f64,
    -0.000000000003025f64,
    0.000000000000472f64,
    -0.000000000000074f64,
    0.000000000000012f64,
    -0.000000000000002f64,
];
static mut coef_jnu8_b: [libc::c_double; 19] = [
    2.595283877150078f64,
    1.756063044986928f64,
    0.156352972371030f64,
    -0.004517201896761f64,
    -0.000025567187878f64,
    0.000053925472558f64,
    -0.000010293734486f64,
    0.000001187923085f64,
    -0.000000051625122f64,
    -0.000000014304212f64,
    0.000000004468450f64,
    -0.000000000695620f64,
    0.000000000052500f64,
    0.000000000005367f64,
    -0.000000000002676f64,
    0.000000000000505f64,
    -0.000000000000050f64,
    -0.000000000000002f64,
    0.000000000000002f64,
];
static mut coef_jnu9_a: [libc::c_double; 19] = [
    34.164181213238386f64,
    6.558412747925228f64,
    -0.102171455365016f64,
    0.009003934361201f64,
    -0.000977663914535f64,
    0.000118479876579f64,
    -0.000015368714220f64,
    0.000002088064285f64,
    -0.000000293381154f64,
    0.000000042283900f64,
    -0.000000006217033f64,
    0.000000000928887f64,
    -0.000000000140627f64,
    0.000000000021526f64,
    -0.000000000003326f64,
    0.000000000000518f64,
    -0.000000000000081f64,
    0.000000000000013f64,
    -0.000000000000002f64,
];
static mut coef_jnu9_b: [libc::c_double; 19] = [
    2.600150240905079f64,
    1.761635491694032f64,
    0.157026743724010f64,
    -0.004549100368716f64,
    -0.000024659248617f64,
    0.000054291035068f64,
    -0.000010403464334f64,
    0.000001206027524f64,
    -0.000000053234089f64,
    -0.000000014406241f64,
    0.000000004542078f64,
    -0.000000000711728f64,
    0.000000000054464f64,
    0.000000000005350f64,
    -0.000000000002733f64,
    0.000000000000521f64,
    -0.000000000000052f64,
    -0.000000000000002f64,
    0.000000000000002f64,
];
static mut coef_jnu10_a: [libc::c_double; 20] = [
    38.047560766184647f64,
    7.288377637926008f64,
    -0.113372193277897f64,
    0.009973047509098f64,
    -0.001081019701335f64,
    0.000130786983847f64,
    -0.000016937898538f64,
    0.000002297699179f64,
    -0.000000322354218f64,
    0.000000046392941f64,
    -0.000000006811759f64,
    0.000000001016395f64,
    -0.000000000153677f64,
    0.000000000023486f64,
    -0.000000000003616f64,
    0.000000000000561f64,
    -0.000000000000095f64,
    0.000000000000027f64,
    -0.000000000000013f64,
    0.000000000000005f64,
];
static mut coef_jnu10_b: [libc::c_double; 19] = [
    2.604046346867949f64,
    1.766097596481182f64,
    0.157566834446511f64,
    -0.004574682244089f64,
    -0.000023934500688f64,
    0.000054585558231f64,
    -0.000010491765415f64,
    0.000001220589364f64,
    -0.000000054526331f64,
    -0.000000014489078f64,
    0.000000004601510f64,
    -0.000000000724727f64,
    0.000000000056049f64,
    0.000000000005337f64,
    -0.000000000002779f64,
    0.000000000000533f64,
    -0.000000000000054f64,
    -0.000000000000002f64,
    0.000000000000002f64,
];
static mut coef_jnu11_a: [libc::c_double; 27] = [
    49.5054081076848637f64,
    15.33692279367165101f64,
    -0.33677234163517130f64,
    0.04623235772920729f64,
    -0.00781084960665093f64,
    0.00147217395434708f64,
    -0.00029695043846867f64,
    0.00006273356860235f64,
    -0.00001370575125628f64,
    3.07171282012e-6f64,
    -7.0235041249e-7f64,
    1.6320559339e-7f64,
    -3.843117306e-8f64,
    9.15083800e-9f64,
    -2.19957642e-9f64,
    5.3301703e-10f64,
    -1.3007541e-10f64,
    3.193827e-11f64,
    -7.88605e-12f64,
    1.95918e-12f64,
    -4.9020e-13f64,
    1.2207e-13f64,
    -2.820e-14f64,
    5.25e-15f64,
    -1.88e-15f64,
    2.80e-15f64,
    -2.45e-15f64,
];
static mut coef_jnu12_a: [libc::c_double; 27] = [
    54.0787833216641519f64,
    16.7336367772863598f64,
    -0.36718411124537953f64,
    0.05035523375053820f64,
    -0.00849884978867533f64,
    0.00160027692813434f64,
    -0.00032248114889921f64,
    0.00006806354127199f64,
    -0.00001485665901339f64,
    3.32668783672e-6f64,
    -7.5998952729e-7f64,
    1.7644939709e-7f64,
    -4.151538210e-8f64,
    9.87722772e-9f64,
    -2.37230133e-9f64,
    5.7442875e-10f64,
    -1.4007767e-10f64,
    3.437166e-11f64,
    -8.48215e-12f64,
    2.10554e-12f64,
    -5.2623e-13f64,
    1.3189e-13f64,
    -3.175e-14f64,
    5.73e-15f64,
    5.6e-16f64,
    -8.7e-16f64,
    -6.5e-16f64,
];
static mut coef_jnu13_a: [libc::c_double; 30] = [
    58.6521941921708890f64,
    18.1303398137970284f64,
    -0.39759381380126650f64,
    0.05447765240465494f64,
    -0.00918674227679980f64,
    0.00172835361420579f64,
    -0.00034800528297612f64,
    0.00007339183835188f64,
    -0.00001600713368099f64,
    3.58154960392e-6f64,
    -8.1759873497e-7f64,
    1.8968523220e-7f64,
    -4.459745253e-8f64,
    1.060304419e-8f64,
    -2.54487624e-9f64,
    6.1580214e-10f64,
    -1.5006751e-10f64,
    3.679707e-11f64,
    -9.07159e-12f64,
    2.24713e-12f64,
    -5.5943e-13f64,
    1.4069e-13f64,
    -3.679e-14f64,
    1.119e-14f64,
    -4.99e-15f64,
    3.43e-15f64,
    -2.85e-15f64,
    2.3e-15f64,
    -1.7e-15f64,
    8.7e-16f64,
];
static mut coef_jnu14_a: [libc::c_double; 26] = [
    63.2256329577315566f64,
    19.5270342832914901f64,
    -0.42800190567884337f64,
    0.05859971627729398f64,
    -0.00987455163523582f64,
    0.00185641011402081f64,
    -0.00037352439419968f64,
    0.00007871886257265f64,
    -0.00001715728110045f64,
    3.83632624437e-6f64,
    -8.7518558668e-7f64,
    2.0291515353e-7f64,
    -4.767795233e-8f64,
    1.132844415e-8f64,
    -2.71734219e-9f64,
    6.5714886e-10f64,
    -1.6005342e-10f64,
    3.922557e-11f64,
    -9.66637e-12f64,
    2.39379e-12f64,
    -5.9541e-13f64,
    1.4868e-13f64,
    -3.726e-14f64,
    9.37e-15f64,
    -2.36e-15f64,
    6.0e-16f64,
];
static mut coef_jnu15_a: [libc::c_double; 26] = [
    67.7990939565631635f64,
    20.9237219226859859f64,
    -0.45840871823085836f64,
    0.06272149946755639f64,
    -0.01056229551143042f64,
    0.00198445078693100f64,
    -0.00039903958650729f64,
    0.00008404489865469f64,
    -0.00001830717574922f64,
    4.09103745566e-6f64,
    -9.3275533309e-7f64,
    2.1614056403e-7f64,
    -5.075725222e-8f64,
    1.205352081e-8f64,
    -2.88971837e-9f64,
    6.9846848e-10f64,
    -1.7002946e-10f64,
    4.164941e-11f64,
    -1.025859e-11f64,
    2.53921e-12f64,
    -6.3128e-13f64,
    1.5757e-13f64,
    -3.947e-14f64,
    9.92e-15f64,
    -2.50e-15f64,
    6.3e-16f64,
];
static mut coef_jnu16_a: [libc::c_double; 26] = [
    72.3725729616724770f64,
    22.32040402918608585f64,
    -0.48881449782358690f64,
    0.06684305681828766f64,
    -0.01124998690363398f64,
    0.00211247882775445f64,
    -0.00042455166484632f64,
    0.00008937015316346f64,
    -0.00001945687139551f64,
    4.34569739281e-6f64,
    -9.9031173548e-7f64,
    2.2936247195e-7f64,
    -5.383562595e-8f64,
    1.277835103e-8f64,
    -3.06202860e-9f64,
    7.3977037e-10f64,
    -1.8000071e-10f64,
    4.407196e-11f64,
    -1.085046e-11f64,
    2.68453e-12f64,
    -6.6712e-13f64,
    1.6644e-13f64,
    -4.168e-14f64,
    1.047e-14f64,
    -2.64e-15f64,
    6.7e-16f64,
];
static mut coef_jnu17_a: [libc::c_double; 26] = [
    76.9460667535209549f64,
    23.71708159112252670f64,
    -0.51921943142405352f64,
    0.07096442978067622f64,
    -0.01193763559341369f64,
    0.00224049662974902f64,
    -0.00045006122941781f64,
    0.00009469477941684f64,
    -0.00002060640777107f64,
    4.60031647195e-6f64,
    -1.04785755046e-6f64,
    2.4258161247e-7f64,
    -5.691327087e-8f64,
    1.350298805e-8f64,
    -3.23428733e-9f64,
    7.8105847e-10f64,
    -1.8996825e-10f64,
    4.649350e-11f64,
    -1.144205e-11f64,
    2.82979e-12f64,
    -7.0294e-13f64,
    1.7531e-13f64,
    -4.388e-14f64,
    1.102e-14f64,
    -2.78e-15f64,
    7.0e-16f64,
];
static mut coef_jnu18_a: [libc::c_double; 26] = [
    81.5195728368096659f64,
    25.11375537470259305f64,
    -0.54962366347317668f64,
    0.07508565026117689f64,
    -0.01262524908033818f64,
    0.00236850602019778f64,
    -0.00047556873651929f64,
    0.00010001889347161f64,
    -0.00002175581482429f64,
    4.85490251239e-6f64,
    -1.10539483940e-6f64,
    2.5579853343e-7f64,
    -5.999033352e-8f64,
    1.422747129e-8f64,
    -3.40650521e-9f64,
    8.2233565e-10f64,
    -1.9993286e-10f64,
    4.891426e-11f64,
    -1.203343e-11f64,
    2.97498e-12f64,
    -7.3875e-13f64,
    1.8418e-13f64,
    -4.608e-14f64,
    1.157e-14f64,
    -2.91e-15f64,
    7.4e-16f64,
];
static mut coef_jnu19_a: [libc::c_double; 26] = [
    86.0930892477047512f64,
    26.51042598308271729f64,
    -0.58002730731948358f64,
    0.07920674321589394f64,
    -0.01331283320930301f64,
    0.00249650841778073f64,
    -0.00050107453900793f64,
    0.00010534258471335f64,
    -0.00002290511552874f64,
    5.10946148897e-6f64,
    -1.16292517157e-6f64,
    2.6901365037e-7f64,
    -6.306692473e-8f64,
    1.495183048e-8f64,
    -3.57869025e-9f64,
    8.6360410e-10f64,
    -2.0989514e-10f64,
    5.133439e-11f64,
    -1.262465e-11f64,
    3.12013e-12f64,
    -7.7455e-13f64,
    1.9304e-13f64,
    -4.829e-14f64,
    1.212e-14f64,
    -3.05e-15f64,
    7.7e-16f64,
];
static mut coef_jnu20_a: [libc::c_double; 27] = [
    90.6666144195163770f64,
    27.9070938975436823f64,
    -0.61043045315390591f64,
    0.08332772844325554f64,
    -0.01400039260208282f64,
    0.00262450494035660f64,
    -0.00052657891389470f64,
    0.00011066592304919f64,
    -0.00002405432778364f64,
    5.36399803946e-6f64,
    -1.22044976064e-6f64,
    2.8222728362e-7f64,
    -6.614312964e-8f64,
    1.567608839e-8f64,
    -3.75084856e-9f64,
    9.0486546e-10f64,
    -2.1985553e-10f64,
    5.375401e-11f64,
    -1.321572e-11f64,
    3.26524e-12f64,
    -8.1033e-13f64,
    2.0190e-13f64,
    -5.049e-14f64,
    1.267e-14f64,
    -3.19e-15f64,
    8.0e-16f64,
    -2.0e-16f64,
];
static mut coef_jnu_a: [*const libc::c_double; 21] = unsafe {
    [
        0 as *const libc::c_double,
        coef_jnu1_a.as_ptr(),
        coef_jnu2_a.as_ptr(),
        coef_jnu3_a.as_ptr(),
        coef_jnu4_a.as_ptr(),
        coef_jnu5_a.as_ptr(),
        coef_jnu6_a.as_ptr(),
        coef_jnu7_a.as_ptr(),
        coef_jnu8_a.as_ptr(),
        coef_jnu9_a.as_ptr(),
        coef_jnu10_a.as_ptr(),
        coef_jnu11_a.as_ptr(),
        coef_jnu12_a.as_ptr(),
        coef_jnu13_a.as_ptr(),
        coef_jnu14_a.as_ptr(),
        coef_jnu15_a.as_ptr(),
        coef_jnu16_a.as_ptr(),
        coef_jnu17_a.as_ptr(),
        coef_jnu18_a.as_ptr(),
        coef_jnu19_a.as_ptr(),
        coef_jnu20_a.as_ptr(),
    ]
};
static mut size_jnu_a: [size_t; 21] = [0; 21];
static mut coef_jnu_b: [*const libc::c_double; 11] = unsafe {
    [
        0 as *const libc::c_double,
        coef_jnu1_b.as_ptr(),
        coef_jnu2_b.as_ptr(),
        coef_jnu3_b.as_ptr(),
        coef_jnu4_b.as_ptr(),
        coef_jnu5_b.as_ptr(),
        coef_jnu6_b.as_ptr(),
        coef_jnu7_b.as_ptr(),
        coef_jnu8_b.as_ptr(),
        coef_jnu9_b.as_ptr(),
        coef_jnu10_b.as_ptr(),
    ]
};
static mut size_jnu_b: [size_t; 11] = [0; 11];
unsafe extern "C" fn clenshaw(
    mut c: *const libc::c_double,
    mut N: libc::c_int,
    mut u: libc::c_double,
) -> libc::c_double {
    let mut B_np1: libc::c_double = 0.0f64;
    let mut B_n: libc::c_double = *c.offset(N as isize);
    let mut B_nm1: libc::c_double = 0.;
    let mut n: libc::c_int = 0;
    n = N;
    while n > 0 as libc::c_int {
        B_nm1 = 2.0f64 * (2.0f64 * u - 1.0f64) * B_n - B_np1
            + *c.offset((n - 1 as libc::c_int) as isize);
        B_np1 = B_n;
        B_n = B_nm1;
        n -= 1;
        n;
    }
    return B_n - (2.0f64 * u - 1.0f64) * B_np1;
}
unsafe extern "C" fn mcmahon_correction(
    mu: libc::c_double,
    beta: libc::c_double,
) -> libc::c_double {
    let eb: libc::c_double = 8.0f64 * beta;
    let ebsq: libc::c_double = eb * eb;
    if mu < 2.2204460492503131e-16f64 {
        let term1: libc::c_double = 1.0f64 / ebsq;
        let term2: libc::c_double = -4.0f64 * 31.0f64
            / (3 as libc::c_int as libc::c_double * ebsq * ebsq);
        let term3: libc::c_double = 32.0f64 * 3779.0f64 / (15.0f64 * ebsq * ebsq * ebsq);
        let term4: libc::c_double = -64.0f64 * 6277237.0f64
            / (105.0f64 * ebsq * ebsq * ebsq * ebsq);
        let term5: libc::c_double = 512.0f64 * 2092163573.0f64
            / (315.0f64 * ebsq * ebsq * ebsq * ebsq * ebsq);
        return 1.0f64 + 8.0f64 * (term1 + term2 + term3 + term4 + term5);
    } else {
        let mi: libc::c_double = 1.0f64 / mu;
        let r: libc::c_double = mu / ebsq;
        let n2: libc::c_double = 4.0f64 / 3.0f64 * (7.0f64 - 31.0f64 * mi);
        let n3: libc::c_double = 32.0f64 / 15.0f64
            * (83.0f64 + (-982.0f64 + 3779.0f64 * mi) * mi);
        let n4: libc::c_double = 64.0f64 / 105.0f64
            * (6949.0f64
                + (-153855.0f64 + (1585743.0f64 - 6277237.0f64 * mi) * mi) * mi);
        let n5: libc::c_double = 512.0f64 / 315.0f64
            * (70197.0f64
                + (-2479316.0f64
                    + (48010494.0f64 + (-512062548.0f64 + 2092163573.0f64 * mi) * mi)
                        * mi) * mi);
        let n6: libc::c_double = 2048.0f64 / 3465.0f64
            * (5592657.0f64
                + (-287149133.0f64
                    + (8903961290.0f64
                        + (-179289628602.0f64
                            + (1982611456181.0f64 - 8249725736393.0f64 * mi) * mi) * mi)
                        * mi) * mi);
        let term1_0: libc::c_double = (1.0f64 - mi) * r;
        let term2_0: libc::c_double = term1_0 * n2 * r;
        let term3_0: libc::c_double = term1_0 * n3 * r * r;
        let term4_0: libc::c_double = term1_0 * n4 * r * r * r;
        let term5_0: libc::c_double = term1_0 * n5 * r * r * r * r;
        let term6: libc::c_double = term1_0 * n6 * r * r * r * r * r;
        return 1.0f64
            - 8.0f64 * (term1_0 + term2_0 + term3_0 + term4_0 + term5_0 + term6);
    };
}
unsafe extern "C" fn olver_b0(
    mut z: libc::c_double,
    mut minus_zeta: libc::c_double,
) -> libc::c_double {
    if z < 1.02f64 {
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
        let abs_zeta: libc::c_double = minus_zeta;
        let t: libc::c_double = 1.0f64 / (z * sqrt(1.0f64 - 1.0f64 / (z * z)));
        return -5.0f64 / (48.0f64 * abs_zeta * abs_zeta)
            + t * (3.0f64 + 5.0f64 * t * t) / (24.0f64 * sqrt(abs_zeta));
    };
}
#[inline]
unsafe extern "C" fn olver_f1(
    mut z: libc::c_double,
    mut minus_zeta: libc::c_double,
) -> libc::c_double {
    let b0: libc::c_double = olver_b0(z, minus_zeta);
    let h2: libc::c_double = sqrt(4.0f64 * minus_zeta / (z * z - 1.0f64));
    return 0.5f64 * z * h2 * b0;
}
pub unsafe extern "C" fn gsl_sf_bessel_zero_J0_e(
    mut s: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s == 0 as libc::c_int as libc::c_uint {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        gsl_error(
            b"error\0" as *const u8 as *const libc::c_char,
            b"bessel_zero.c\0" as *const u8 as *const libc::c_char,
            1031 as libc::c_int,
            GSL_EINVAL as libc::c_int,
        );
        return GSL_EINVAL as libc::c_int;
    } else {
        static mut P: [libc::c_double; 4] = [
            1567450796.0f64 / 12539606369.0f64,
            8903660.0f64 / 2365861.0f64,
            10747040.0f64 / 536751.0f64,
            17590991.0f64 / 1696654.0f64,
        ];
        static mut Q: [libc::c_double; 4] = [
            1.0f64,
            29354255.0f64 / 954518.0f64,
            76900001.0f64 / 431847.0f64,
            67237052.0f64 / 442411.0f64,
        ];
        let beta: libc::c_double = (s as libc::c_double - 0.25f64)
            * 3.14159265358979323846f64;
        let bi2: libc::c_double = 1.0f64 / (beta * beta);
        let R33num: libc::c_double = P[0 as libc::c_int as usize]
            + bi2
                * (P[1 as libc::c_int as usize]
                    + bi2
                        * (P[2 as libc::c_int as usize]
                            + P[3 as libc::c_int as usize] * bi2));
        let R33den: libc::c_double = Q[0 as libc::c_int as usize]
            + bi2
                * (Q[1 as libc::c_int as usize]
                    + bi2
                        * (Q[2 as libc::c_int as usize]
                            + Q[3 as libc::c_int as usize] * bi2));
        let R33: libc::c_double = R33num / R33den;
        (*result).val = beta + R33 / beta;
        (*result).err = fabs(3.0e-15f64 * (*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_zero_J1_e(
    mut s: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s == 0 as libc::c_int as libc::c_uint {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        return GSL_SUCCESS as libc::c_int;
    } else {
        static mut a: [libc::c_double; 4] = [
            -0.362804405737084f64,
            0.120341279038597f64,
            0.439454547101171e-01f64,
            0.159340088474713e-02f64,
        ];
        static mut b: [libc::c_double; 4] = [
            1.0f64,
            -0.325641790801361f64,
            -0.117453445968927f64,
            -0.424906902601794e-02f64,
        ];
        let beta: libc::c_double = (s as libc::c_double + 0.25f64)
            * 3.14159265358979323846f64;
        let bi2: libc::c_double = 1.0f64 / (beta * beta);
        let Rnum: libc::c_double = a[3 as libc::c_int as usize]
            + bi2
                * (a[2 as libc::c_int as usize]
                    + bi2
                        * (a[1 as libc::c_int as usize]
                            + bi2 * a[0 as libc::c_int as usize]));
        let Rden: libc::c_double = b[3 as libc::c_int as usize]
            + bi2
                * (b[2 as libc::c_int as usize]
                    + bi2
                        * (b[1 as libc::c_int as usize]
                            + bi2 * b[0 as libc::c_int as usize]));
        let R: libc::c_double = Rnum / Rden;
        (*result).val = beta * (1.0f64 + R * bi2);
        (*result).err = fabs(2.0e-14f64 * (*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_zero_Jnu_e(
    mut nu: libc::c_double,
    mut s: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if nu <= -1.0f64 {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"domain error\0" as *const u8 as *const libc::c_char,
            b"bessel_zero.c\0" as *const u8 as *const libc::c_char,
            1101 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if s == 0 as libc::c_int as libc::c_uint {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        if nu == 0.0f64 {
            gsl_error(
                b"no zero-th root for nu = 0.0\0" as *const u8 as *const libc::c_char,
                b"bessel_zero.c\0" as *const u8 as *const libc::c_char,
                1107 as libc::c_int,
                GSL_EINVAL as libc::c_int,
            );
            return GSL_EINVAL as libc::c_int;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if nu < 0.0f64 {
        (*result).val = 0.0f64;
        (*result).err = 0.0f64;
        gsl_error(
            b"unimplemented\0" as *const u8 as *const libc::c_char,
            b"bessel_zero.c\0" as *const u8 as *const libc::c_char,
            1115 as libc::c_int,
            GSL_EUNIMPL as libc::c_int,
        );
        return GSL_EUNIMPL as libc::c_int;
    } else if s == 1 as libc::c_int as libc::c_uint {
        if nu < 2.0f64 {
            let mut c: *const libc::c_double = coef_jnu_a[s as usize];
            let L: size_t = size_jnu_a[s as usize];
            let arg: libc::c_double = nu / 2.0f64;
            let chb: libc::c_double = clenshaw(
                c,
                L.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                arg,
            );
            (*result).val = chb;
            (*result).err = 2.0e-15f64 * (*result).val;
        } else {
            let mut c_0: *const libc::c_double = coef_jnu_b[s as usize];
            let L_0: size_t = size_jnu_b[s as usize];
            let arg_0: libc::c_double = pow(2.0f64 / nu, 2.0f64 / 3.0f64);
            let chb_0: libc::c_double = clenshaw(
                c_0,
                L_0.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                arg_0,
            );
            (*result).val = nu * chb_0;
            (*result).err = 2.0e-15f64 * (*result).val;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if s <= 10 as libc::c_int as libc::c_uint {
        if nu < s as libc::c_double {
            let mut c_1: *const libc::c_double = coef_jnu_a[s as usize];
            let L_1: size_t = size_jnu_a[s as usize];
            let arg_1: libc::c_double = nu / s as libc::c_double;
            let chb_1: libc::c_double = clenshaw(
                c_1,
                L_1.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                arg_1,
            );
            (*result).val = chb_1;
            (*result).err = 2.0e-15f64 * (*result).val;
        } else {
            let mut c_2: *const libc::c_double = coef_jnu_b[s as usize];
            let L_2: size_t = size_jnu_b[s as usize];
            let arg_2: libc::c_double = pow(s as libc::c_double / nu, 2.0f64 / 3.0f64);
            let chb_2: libc::c_double = clenshaw(
                c_2,
                L_2.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                arg_2,
            );
            (*result).val = nu * chb_2;
            (*result).err = 2.0e-15f64 * (*result).val;
            if s == 5 as libc::c_int as libc::c_uint {
                (*result).err *= 5.0e+06f64;
            }
        }
        return GSL_SUCCESS as libc::c_int;
    } else if s as libc::c_double > 0.5f64 * nu && s <= 20 as libc::c_int as libc::c_uint
    {
        let mut c_3: *const libc::c_double = coef_jnu_a[s as usize];
        let L_3: size_t = size_jnu_a[s as usize];
        let arg_3: libc::c_double = nu / (2.0f64 * s as libc::c_double);
        let chb_3: libc::c_double = clenshaw(
            c_3,
            L_3.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            arg_3,
        );
        (*result).val = chb_3;
        (*result).err = 4.0e-15f64 * chb_3;
        return GSL_SUCCESS as libc::c_int;
    } else if s as libc::c_double > 2.0f64 * nu {
        let beta: libc::c_double = (s as libc::c_double + 0.5f64 * nu - 0.25f64)
            * 3.14159265358979323846f64;
        let mc: libc::c_double = mcmahon_correction(4.0f64 * nu * nu, beta);
        let mut rat12: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        gsl_sf_pow_int_e(nu / beta, 14 as libc::c_int, &mut rat12);
        (*result).val = beta * mc;
        (*result).err = 4.0f64 * fabs(beta) * rat12.val;
        (*result).err += 4.0f64 * fabs(2.2204460492503131e-16f64 * (*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let mut as_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let stat_as: libc::c_int = gsl_sf_airy_zero_Ai_e(s, &mut as_0);
        let minus_zeta: libc::c_double = -pow(nu, -2.0f64 / 3.0f64) * as_0.val;
        let z: libc::c_double = gsl_sf_bessel_Olver_zofmzeta(minus_zeta);
        let f1: libc::c_double = olver_f1(z, minus_zeta);
        (*result).val = nu * (z + f1 / (nu * nu));
        (*result).err = 0.001f64 / (nu * nu * nu);
        (*result).err += 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return stat_as;
    };
}
pub unsafe extern "C" fn gsl_sf_bessel_zero_J0(mut s: libc::c_uint) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_zero_J0_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_zero_J0_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_zero.c\0" as *const u8 as *const libc::c_char,
            1208 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_zero_J1(mut s: libc::c_uint) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_zero_J1_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_zero_J1_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"bessel_zero.c\0" as *const u8 as *const libc::c_char,
            1213 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_bessel_zero_Jnu(
    mut nu: libc::c_double,
    mut s: libc::c_uint,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_bessel_zero_Jnu_e(nu, s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_bessel_zero_Jnu_e(nu, s, &result)\0" as *const u8
                as *const libc::c_char,
            b"bessel_zero.c\0" as *const u8 as *const libc::c_char,
            1218 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
unsafe extern "C" fn run_static_initializers() {
    size_jnu_a = [
        0 as libc::c_int as size_t,
        (::std::mem::size_of::<[libc::c_double; 24]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 20]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 27]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 27]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 30]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 26]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 27]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ];
    size_jnu_b = [
        0 as libc::c_int as size_t,
        (::std::mem::size_of::<[libc::c_double; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        (::std::mem::size_of::<[libc::c_double; 19]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
