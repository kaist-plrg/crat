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
static mut zero_Ai: [libc::c_double; 101] = [
    0 as libc::c_int as libc::c_double,
    -2.3381074104597670385f64,
    -4.087949444130970617f64,
    -5.520559828095551059f64,
    -6.786708090071758999f64,
    -7.944133587120853123f64,
    -9.022650853340980380f64,
    -10.04017434155808593f64,
    -11.00852430373326289f64,
    -11.93601556323626252f64,
    -12.82877675286575720f64,
    -13.69148903521071793f64,
    -14.52782995177533498f64,
    -15.34075513597799686f64,
    -16.13268515694577144f64,
    -16.90563399742994263f64,
    -17.661300105697057509f64,
    -18.401132599207115416f64,
    -19.126380474246952144f64,
    -19.838129891721499701f64,
    -20.537332907677566360f64,
    -21.224829943642096955f64,
    -21.901367595585130707f64,
    -22.567612917496502831f64,
    -23.224165001121681061f64,
    -23.871564455535918567f64,
    -24.510301236589677490f64,
    -25.140821166148963748f64,
    -25.763531400982756459f64,
    -26.378805052137232374f64,
    -26.986985111606367686f64,
    -27.588387809882444812f64,
    -28.183305502632644923f64,
    -28.772009165237435382f64,
    -29.354750558766287963f64,
    -29.931764119086555913f64,
    -30.503268611418505287f64,
    -31.069468585183755604f64,
    -31.63055565801265934f64,
    -32.18670965295205069f64,
    -32.73809960900026913f64,
    -33.28488468190140188f64,
    -33.82721494950865194f64,
    -34.36523213386365906f64,
    -34.89907025034531210f64,
    -35.42885619274788846f64,
    -35.95471026189862926f64,
    -36.47674664437480896f64,
    -36.99507384699450161f64,
    -37.50979509200501613f64,
    -38.02100867725525443f64,
    -38.52880830509424882f64,
    -39.03328338327251391f64,
    -39.53451930072301805f64,
    -40.03259768075417603f64,
    -40.52759661388971821f64,
    -41.01959087233248966f64,
    -41.50865210780525018f64,
    -41.99484903432643004f64,
    -42.47824759730839188f64,
    -42.95891113021656009f64,
    -43.43690049989685412f64,
    -43.91227424156370168f64,
    -44.38508868433939023f64,
    -44.85539806814583243f64,
    -45.32325465267043011f64,
    -45.78870881905730086f64,
    -46.25180916491254629f64,
    -46.71260259315651633f64,
    -47.17113439520631705f64,
    -47.62744832892739292f64,
    -48.08158669175325711f64,
    -48.53359038933679845f64,
    -48.98349900006458366f64,
    -49.43135083573678341f64,
    -49.87718299868941729f64,
    -50.32103143561221860f64,
    -50.76293098829428522f64,
    -51.20291544151056412f64,
    -51.64101756824489758f64,
    -52.07726917242964943f64,
    -52.51170112936766183f64,
    -52.94434342398931824f64,
    -53.37522518708567514f64,
    -53.80437472964785717f64,
    -54.23181957543308298f64,
    -54.65758649186871111f64,
    -55.08170151939748312f64,
    -55.50418999935962251f64,
    -55.92507660050055598f64,
    -56.34438534418670066f64,
    -56.76213962840595327f64,
    -57.17836225062417808f64,
    -57.59307542956407782f64,
    -58.00630082596830627f64,
    -58.41805956240450934f64,
    -58.82837224216613231f64,
    -59.23725896731927534f64,
    -59.64473935594259360f64,
    -60.05083255860419805f64,
    -60.45555727411669871f64,
];
static mut size_zero_Ai: size_t = 0;
static mut zero_Bi: [libc::c_double; 101] = [
    0 as libc::c_int as libc::c_double,
    -1.173713222709127925f64,
    -3.271093302836352716f64,
    -4.830737841662015933f64,
    -6.169852128310251260f64,
    -7.376762079367763714f64,
    -8.491948846509388013f64,
    -9.538194379346238887f64,
    -10.52991350670535792f64,
    -11.47695355127877944f64,
    -12.38641713858273875f64,
    -13.26363952294180555f64,
    -14.11275680906865779f64,
    -14.93705741215416404f64,
    -15.739210351190482771f64,
    -16.521419550634379054f64,
    -17.285531624581242533f64,
    -18.033113287225001572f64,
    -18.765508284480081041f64,
    -19.483880132989234014f64,
    -20.189244785396202420f64,
    -20.882495994193175768f64,
    -21.564425284712977653f64,
    -22.235737881803385167f64,
    -22.897065554219793474f64,
    -23.548977079642448269f64,
    -24.191986850649000086f64,
    -24.826562012152892172f64,
    -25.453128427085131994f64,
    -26.072075698466804494f64,
    -26.683761425120990449f64,
    -27.288514830076298204f64,
    -27.886639871735962459f64,
    -28.478417925678661737f64,
    -29.064110107777650305f64,
    -29.643959295918396591f64,
    -30.218191897047274645f64,
    -30.787019397921766297f64,
    -31.350639731255585371f64,
    -31.90923848358456965f64,
    -32.46298996683685318f64,
    -33.01205817205683814f64,
    -33.55659762084006113f64,
    -34.09675412765602851f64,
    -34.63266548426775468f64,
    -35.16446207582101720f64,
    -35.69226743681080479f64,
    -36.21619875398748222f64,
    -36.73636732230120657f64,
    -37.25287895916828697f64,
    -37.76583438165180116f64,
    -38.27532955056003997f64,
    -38.78145598496327279f64,
    -39.28430105019802461f64,
    -39.78394822205711298f64,
    -40.28047732954369150f64,
    -40.77396477829068148f64,
    -41.26448375650675678f64,
    -41.75210442510106287f64,
    -42.23689409345656643f64,
    -42.71891738216253539f64,
    -43.19823637387693118f64,
    -43.67491075336673948f64,
    -44.14899793766617113f64,
    -44.62055319719727274f64,
    -45.08962976861312825f64,
    -45.55627896004907928f64,
    -46.02055024940102076f64,
    -46.48249137619078661f64,
    -46.94214842752602207f64,
    -47.39956591861496210f64,
    -47.85478686825452176f64,
    -48.30785286967246692f64,
    -48.75880415707066192f64,
    -49.20767966818603897f64,
    -49.65451710315861501f64,
    -50.09935297997125482f64,
    -50.54222268670364757f64,
    -50.98316053082286586f64,
    -51.42219978571468262f64,
    -51.85937273464332870f64,
    -52.29471071231240525f64,
    -52.72824414418606069f64,
    -53.16000258371716397f64,
    -53.59001474761792882f64,
    -54.01830854929815828f64,
    -54.44491113058688729f64,
    -54.86984889184461534f64,
    -55.29314752056546491f64,
    -55.71483201856140365f64,
    -56.13492672781406761f64,
    -56.55345535507366411f64,
    -56.97044099527886475f64,
    -57.38590615386647834f64,
    -57.79987276803497897f64,
    -58.21236222702161974f64,
    -58.62339539144885603f64,
    -59.03299261179210306f64,
    -59.44117374601743460f64,
    -59.84795817643466996f64,
    -60.25336482580837088f64,
];
static mut size_zero_Bi: size_t = 0;
static mut zero_Aip: [libc::c_double; 101] = [
    0 as libc::c_int as libc::c_double,
    -1.018792971647471089f64,
    -3.248197582179836738f64,
    -4.820099211178735639f64,
    -6.163307355639486822f64,
    -7.372177255047770177f64,
    -8.488486734019722133f64,
    -9.535449052433547471f64,
    -10.52766039695740728f64,
    -11.47505663348024529f64,
    -12.384788371845747325f64,
    -13.262218961665210382f64,
    -14.111501970462995282f64,
    -14.935937196720517467f64,
    -15.738201373692538303f64,
    -16.520503825433793542f64,
    -17.284695050216437357f64,
    -18.032344622504393395f64,
    -18.764798437665954740f64,
    -19.483221656567231178f64,
    -20.188631509463373154f64,
    -20.881922755516737701f64,
    -21.563887723198974958f64,
    -22.235232285348913331f64,
    -22.896588738874619001f64,
    -23.548526295928801574f64,
    -24.191559709526353841f64,
    -24.826156425921155001f64,
    -25.452742561777649948f64,
    -26.071707935173912515f64,
    -26.683410328322449767f64,
    -27.288179121523985029f64,
    -27.886318408768461192f64,
    -28.478109683102278108f64,
    -29.063814162638199090f64,
    -29.643674814632015921f64,
    -30.217918124468574603f64,
    -30.786755648012502519f64,
    -31.350385379083034671f64,
    -31.90899295843046320f64,
    -32.46275274623847982f64,
    -33.01182877663428709f64,
    -33.55637560978942190f64,
    -34.09653909480913771f64,
    -34.63245705463586589f64,
    -35.16425990255340758f64,
    -35.69207119851046870f64,
    -36.21600815233519918f64,
    -36.73618207994680321f64,
    -37.25269881785414827f64,
    -37.76565910053887108f64,
    -38.27515890473087933f64,
    -38.78128976408036876f64,
    -39.28413905729859644f64,
    -39.78379027246823278f64,
    -40.28032324990371935f64,
    -40.77381440566486637f64,
    -41.26433693758643383f64,
    -41.75196101547722703f64,
    -42.23675395695976012f64,
    -42.71878039026198233f64,
    -43.19810240513270670f64,
    -43.67477969292950869f64,
    -44.14886967681966886f64,
    -44.62042763293925724f64,
    -45.08950680327102630f64,
    -45.55615850092696446f64,
    -46.02043220845493728f64,
    -46.48237566972975615f64,
    -46.94203497593635633f64,
    -47.39945464610575493f64,
    -47.85467770262241617f64,
    -48.30774574208398774f64,
    -48.75869900186057804f64,
    -49.20757642267037247f64,
    -49.65441570746105074f64,
    -50.09925337686182515f64,
    -50.54212482144867502f64,
    -50.98306435104524282f64,
    -51.42210524126365311f64,
    -51.85927977747301469f64,
    -52.29461929636838876f64,
    -52.72815422529939506f64,
    -53.15991411950524351f64,
    -53.58992769739169611f64,
    -54.01822287397517367f64,
    -54.44482679260982599f64,
    -54.86976585510479430f64,
    -55.29306575033103518f64,
    -55.71475148140987392f64,
    -56.13484739156885235f64,
    -56.55337718874437424f64,
    -56.97036396900508167f64,
    -57.38583023886477265f64,
    -57.79979793654895377f64,
    -58.21228845227477578f64,
    -58.62332264760009139f64,
    -59.03292087389367419f64,
    -59.44110298997521892f64,
    -59.84788837897058171f64,
    -60.25329596442479317f64,
];
static mut size_zero_Aip: size_t = 0;
static mut zero_Bip: [libc::c_double; 51] = [
    0 as libc::c_int as libc::c_double,
    -2.294439682614123247f64,
    -4.073155089071828216f64,
    -5.512395729663599496f64,
    -6.781294445990305390f64,
    -7.940178689168578927f64,
    -9.019583358794239067f64,
    -10.037696334908545802f64,
    -11.006462667712289940f64,
    -11.934261645014844663f64,
    -12.827258309177217640f64,
    -13.690155826835049101f64,
    -14.526645763485711410f64,
    -15.339693082242404109f64,
    -16.131724782385900578f64,
    -16.904759411889649958f64,
    -17.660498743114976102f64,
    -18.400394367181703280f64,
    -19.125697156412638066f64,
    -19.837494718415910503f64,
    -20.536740241453273980f64,
    -21.224275044889266569f64,
    -21.900846445139208281f64,
    -22.567122080497200470f64,
    -23.223701521208962116f64,
    -23.871125771677973595f64,
    -24.509885117016242729f64,
    -25.140425655367878908f64,
    -25.763154776913454319f64,
    -26.378445791146615697f64,
    -26.986641859775034987f64,
    -27.588059359225600573f64,
    -28.182990771292975456f64,
    -28.771707180886056250f64,
    -29.354460444612957224f64,
    -29.931485082026055160f64,
    -30.502999931936645516f64,
    -31.069209608721234058f64,
    -31.63030578754333679f64,
    -32.18646834257807369f64,
    -32.73786635840274752f64,
    -33.28465903151424981f64,
    -33.82699647630635587f64,
    -34.36502044767239631f64,
    -34.89886499060196419f64,
    -35.42865702564380962f64,
    -35.95451687785511190f64,
    -36.47655875580547918f64,
    -36.99489118631672770f64,
    -37.50961740986809593f64,
    -38.02083574095788210f64,
];
static mut size_zero_Bip: size_t = 0;
unsafe extern "C" fn zero_f(mut z: libc::c_double) -> libc::c_double {
    let pre: libc::c_double = pow(z, 2.0f64 / 3.0f64);
    let zi2: libc::c_double = 1.0f64 / (z * z);
    let zi4: libc::c_double = zi2 * zi2;
    let t1: libc::c_double = 5.0f64 / 48.0f64 * zi2;
    let t2: libc::c_double = -5.0f64 / 36.0f64 * zi4;
    let t3: libc::c_double = 77125.0f64 / 82944.0f64 * zi4 * zi2;
    let t4: libc::c_double = -108056875.0f64 / 6967296.0f64 * zi4 * zi4;
    return pre * (1.0f64 + t1 + t2 + t3 + t4);
}
unsafe extern "C" fn zero_g(mut z: libc::c_double) -> libc::c_double {
    let pre: libc::c_double = pow(z, 2.0f64 / 3.0f64);
    let zi2: libc::c_double = 1.0f64 / (z * z);
    let zi4: libc::c_double = zi2 * zi2;
    let t1: libc::c_double = -7.0f64 / 48.0f64 * zi2;
    let t2: libc::c_double = 35.0f64 / 288.0f64 * zi4;
    let t3: libc::c_double = -181223.0f64 / 207360.0f64 * zi4 * zi2;
    let t4: libc::c_double = 18683371.0f64 / 1244160.0f64 * zi4 * zi4;
    return pre * (1.0f64 + t1 + t2 + t3 + t4);
}
pub unsafe extern "C" fn gsl_sf_airy_zero_Ai_e(
    mut s: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s < 1 as libc::c_int as libc::c_uint {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"s is less than 1\0" as *const u8 as *const libc::c_char,
            b"airy_zero.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (s as libc::c_ulong) < size_zero_Ai {
        (*result).val = zero_Ai[s as usize];
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let z: libc::c_double = 3.0f64 * 3.14159265358979323846f64 / 8.0f64
            * (4.0f64 * s as libc::c_double - 1.0f64);
        let f: libc::c_double = zero_f(z);
        (*result).val = -f;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_airy_zero_Bi_e(
    mut s: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s < 1 as libc::c_int as libc::c_uint {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"s is less than 1\0" as *const u8 as *const libc::c_char,
            b"airy_zero.c\0" as *const u8 as *const libc::c_char,
            462 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (s as libc::c_ulong) < size_zero_Bi {
        (*result).val = zero_Bi[s as usize];
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let z: libc::c_double = 3.0f64 * 3.14159265358979323846f64 / 8.0f64
            * (4.0f64 * s as libc::c_double - 3.0f64);
        let f: libc::c_double = zero_f(z);
        (*result).val = -f;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_airy_zero_Ai_deriv_e(
    mut s: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s < 1 as libc::c_int as libc::c_uint {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"s is less than 1\0" as *const u8 as *const libc::c_char,
            b"airy_zero.c\0" as *const u8 as *const libc::c_char,
            485 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (s as libc::c_ulong) < size_zero_Aip {
        (*result).val = zero_Aip[s as usize];
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let z: libc::c_double = 3.0f64 * 3.14159265358979323846f64 / 8.0f64
            * (4.0f64 * s as libc::c_double - 3.0f64);
        let g: libc::c_double = zero_g(z);
        (*result).val = -g;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_airy_zero_Bi_deriv_e(
    mut s: libc::c_uint,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if s < 1 as libc::c_int as libc::c_uint {
        (*result).val = ::std::f32::NAN as libc::c_double;
        (*result).err = ::std::f32::NAN as libc::c_double;
        gsl_error(
            b"s is less than 1\0" as *const u8 as *const libc::c_char,
            b"airy_zero.c\0" as *const u8 as *const libc::c_char,
            508 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else if (s as libc::c_ulong) < size_zero_Bip {
        (*result).val = zero_Bip[s as usize];
        (*result).err = 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let z: libc::c_double = 3.0f64 * 3.14159265358979323846f64 / 8.0f64
            * (4.0f64 * s as libc::c_double - 1.0f64);
        let g: libc::c_double = zero_g(z);
        (*result).val = -g;
        (*result).err = 2.0f64 * 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_airy_zero_Ai(mut s: libc::c_uint) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_zero_Ai_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_zero_Ai_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"airy_zero.c\0" as *const u8 as *const libc::c_char,
            530 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_airy_zero_Bi(mut s: libc::c_uint) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_zero_Bi_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_zero_Bi_e(s, &result)\0" as *const u8 as *const libc::c_char,
            b"airy_zero.c\0" as *const u8 as *const libc::c_char,
            535 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_airy_zero_Ai_deriv(
    mut s: libc::c_uint,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_zero_Ai_deriv_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_zero_Ai_deriv_e(s, &result)\0" as *const u8
                as *const libc::c_char,
            b"airy_zero.c\0" as *const u8 as *const libc::c_char,
            540 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_airy_zero_Bi_deriv(
    mut s: libc::c_uint,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_zero_Bi_deriv_e(s, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_zero_Bi_deriv_e(s, &result)\0" as *const u8
                as *const libc::c_char,
            b"airy_zero.c\0" as *const u8 as *const libc::c_char,
            545 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
unsafe extern "C" fn run_static_initializers() {
    size_zero_Ai = (::std::mem::size_of::<[libc::c_double; 101]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong);
    size_zero_Bi = (::std::mem::size_of::<[libc::c_double; 101]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong);
    size_zero_Aip = (::std::mem::size_of::<[libc::c_double; 101]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong);
    size_zero_Bip = (::std::mem::size_of::<[libc::c_double; 51]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
