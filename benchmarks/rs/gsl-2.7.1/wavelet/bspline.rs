use ::libc;
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
pub struct gsl_wavelet_type {
    pub name: *const libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut *const libc::c_double,
            *mut *const libc::c_double,
            *mut *const libc::c_double,
            *mut *const libc::c_double,
            *mut size_t,
            *mut size_t,
            size_t,
        ) -> libc::c_int,
    >,
}
static mut h1_103: [libc::c_double; 6] = [
    -0.0883883476483184405501055452631f64,
    0.0883883476483184405501055452631f64,
    0.70710678118654752440f64,
    0.70710678118654752440f64,
    0.0883883476483184405501055452631f64,
    -0.0883883476483184405501055452631f64,
];
static mut g2_103: [libc::c_double; 6] = [
    -0.0883883476483184405501055452631f64,
    -0.0883883476483184405501055452631f64,
    0.70710678118654752440f64,
    -0.70710678118654752440f64,
    0.0883883476483184405501055452631f64,
    0.0883883476483184405501055452631f64,
];
static mut h1_105: [libc::c_double; 10] = [
    0.0165728151840597076031447897368f64,
    -0.0165728151840597076031447897368f64,
    -0.1215339780164378557563951247368f64,
    0.1215339780164378557563951247368f64,
    0.70710678118654752440f64,
    0.70710678118654752440f64,
    0.1215339780164378557563951247368f64,
    -0.1215339780164378557563951247368f64,
    -0.0165728151840597076031447897368f64,
    0.0165728151840597076031447897368f64,
];
static mut g2_105: [libc::c_double; 10] = [
    0.0165728151840597076031447897368f64,
    0.0165728151840597076031447897368f64,
    -0.1215339780164378557563951247368f64,
    -0.1215339780164378557563951247368f64,
    0.70710678118654752440f64,
    -0.70710678118654752440f64,
    0.1215339780164378557563951247368f64,
    0.1215339780164378557563951247368f64,
    -0.0165728151840597076031447897368f64,
    -0.0165728151840597076031447897368f64,
];
static mut g1_1: [libc::c_double; 10] = [
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.70710678118654752440f64,
    -0.70710678118654752440f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
];
static mut h2_1: [libc::c_double; 10] = [
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.70710678118654752440f64,
    0.70710678118654752440f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
];
static mut h1_202: [libc::c_double; 6] = [
    -0.1767766952966368811002110905262f64,
    0.3535533905932737622004221810524f64,
    1.0606601717798212866012665431573f64,
    0.3535533905932737622004221810524f64,
    -0.1767766952966368811002110905262f64,
    0.0f64,
];
static mut g2_202: [libc::c_double; 6] = [
    0.0f64,
    -0.1767766952966368811002110905262f64,
    -0.3535533905932737622004221810524f64,
    1.0606601717798212866012665431573f64,
    -0.3535533905932737622004221810524f64,
    -0.1767766952966368811002110905262f64,
];
static mut h1_204: [libc::c_double; 10] = [
    0.0331456303681194152062895794737f64,
    -0.0662912607362388304125791589473f64,
    -0.1767766952966368811002110905262f64,
    0.4198446513295125926130013399998f64,
    0.9943689110435824561886873842099f64,
    0.4198446513295125926130013399998f64,
    -0.1767766952966368811002110905262f64,
    -0.0662912607362388304125791589473f64,
    0.0331456303681194152062895794737f64,
    0.0f64,
];
static mut g2_204: [libc::c_double; 10] = [
    0.0f64,
    0.0331456303681194152062895794737f64,
    0.0662912607362388304125791589473f64,
    -0.1767766952966368811002110905262f64,
    -0.4198446513295125926130013399998f64,
    0.9943689110435824561886873842099f64,
    -0.4198446513295125926130013399998f64,
    -0.1767766952966368811002110905262f64,
    0.0662912607362388304125791589473f64,
    0.0331456303681194152062895794737f64,
];
static mut h1_206: [libc::c_double; 14] = [
    -0.0069053396600248781679769957237f64,
    0.0138106793200497563359539914474f64,
    0.0469563096881691715422435709210f64,
    -0.1077232986963880994204411332894f64,
    -0.1698713556366120029322340948025f64,
    0.4474660099696121052849093228945f64,
    0.9667475524034829435167794013152f64,
    0.4474660099696121052849093228945f64,
    -0.1698713556366120029322340948025f64,
    -0.1077232986963880994204411332894f64,
    0.0469563096881691715422435709210f64,
    0.0138106793200497563359539914474f64,
    -0.0069053396600248781679769957237f64,
    0.0f64,
];
static mut g2_206: [libc::c_double; 14] = [
    0.0f64,
    -0.0069053396600248781679769957237f64,
    -0.0138106793200497563359539914474f64,
    0.0469563096881691715422435709210f64,
    0.1077232986963880994204411332894f64,
    -0.1698713556366120029322340948025f64,
    -0.4474660099696121052849093228945f64,
    0.9667475524034829435167794013152f64,
    -0.4474660099696121052849093228945f64,
    -0.1698713556366120029322340948025f64,
    0.1077232986963880994204411332894f64,
    0.0469563096881691715422435709210f64,
    -0.0138106793200497563359539914474f64,
    -0.0069053396600248781679769957237f64,
];
static mut h1_208: [libc::c_double; 18] = [
    0.0015105430506304420992449678146f64,
    -0.0030210861012608841984899356291f64,
    -0.0129475118625466465649568669819f64,
    0.0289161098263541773284036695929f64,
    0.0529984818906909399392234421792f64,
    -0.1349130736077360572068505539514f64,
    -0.1638291834340902345352542235443f64,
    0.4625714404759165262773590010400f64,
    0.9516421218971785225243297231697f64,
    0.4625714404759165262773590010400f64,
    -0.1638291834340902345352542235443f64,
    -0.1349130736077360572068505539514f64,
    0.0529984818906909399392234421792f64,
    0.0289161098263541773284036695929f64,
    -0.0129475118625466465649568669819f64,
    -0.0030210861012608841984899356291f64,
    0.0015105430506304420992449678146f64,
    0.0f64,
];
static mut g2_208: [libc::c_double; 18] = [
    0.0f64,
    0.0015105430506304420992449678146f64,
    0.0030210861012608841984899356291f64,
    -0.0129475118625466465649568669819f64,
    -0.0289161098263541773284036695929f64,
    0.0529984818906909399392234421792f64,
    0.1349130736077360572068505539514f64,
    -0.1638291834340902345352542235443f64,
    -0.4625714404759165262773590010400f64,
    0.9516421218971785225243297231697f64,
    -0.4625714404759165262773590010400f64,
    -0.1638291834340902345352542235443f64,
    0.1349130736077360572068505539514f64,
    0.0529984818906909399392234421792f64,
    -0.0289161098263541773284036695929f64,
    -0.0129475118625466465649568669819f64,
    0.0030210861012608841984899356291f64,
    0.0015105430506304420992449678146f64,
];
static mut h2_2: [libc::c_double; 18] = [
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.3535533905932737622004221810524f64,
    0.7071067811865475244008443621048f64,
    0.3535533905932737622004221810524f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
];
static mut g1_2: [libc::c_double; 18] = [
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    -0.3535533905932737622004221810524f64,
    0.7071067811865475244008443621048f64,
    -0.3535533905932737622004221810524f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
];
static mut h1_301: [libc::c_double; 4] = [
    -0.3535533905932737622004221810524f64,
    1.0606601717798212866012665431573f64,
    1.0606601717798212866012665431573f64,
    -0.3535533905932737622004221810524f64,
];
static mut g2_301: [libc::c_double; 4] = [
    0.3535533905932737622004221810524f64,
    1.0606601717798212866012665431573f64,
    -1.0606601717798212866012665431573f64,
    -0.3535533905932737622004221810524f64,
];
static mut h1_303: [libc::c_double; 8] = [
    0.0662912607362388304125791589473f64,
    -0.1988737822087164912377374768420f64,
    -0.1546796083845572709626847042104f64,
    0.9943689110435824561886873842099f64,
    0.9943689110435824561886873842099f64,
    -0.1546796083845572709626847042104f64,
    -0.1988737822087164912377374768420f64,
    0.0662912607362388304125791589473f64,
];
static mut g2_303: [libc::c_double; 8] = [
    -0.0662912607362388304125791589473f64,
    -0.1988737822087164912377374768420f64,
    0.1546796083845572709626847042104f64,
    0.9943689110435824561886873842099f64,
    -0.9943689110435824561886873842099f64,
    -0.1546796083845572709626847042104f64,
    0.1988737822087164912377374768420f64,
    0.0662912607362388304125791589473f64,
];
static mut h1_305: [libc::c_double; 12] = [
    -0.0138106793200497563359539914474f64,
    0.0414320379601492690078619743421f64,
    0.0524805814161890740766251675000f64,
    -0.2679271788089652729175074340788f64,
    -0.0718155324642587329469607555263f64,
    0.9667475524034829435167794013152f64,
    0.9667475524034829435167794013152f64,
    -0.0718155324642587329469607555263f64,
    -0.2679271788089652729175074340788f64,
    0.0524805814161890740766251675000f64,
    0.0414320379601492690078619743421f64,
    -0.0138106793200497563359539914474f64,
];
static mut g2_305: [libc::c_double; 12] = [
    0.0138106793200497563359539914474f64,
    0.0414320379601492690078619743421f64,
    -0.0524805814161890740766251675000f64,
    -0.2679271788089652729175074340788f64,
    0.0718155324642587329469607555263f64,
    0.9667475524034829435167794013152f64,
    -0.9667475524034829435167794013152f64,
    -0.0718155324642587329469607555263f64,
    0.2679271788089652729175074340788f64,
    0.0524805814161890740766251675000f64,
    -0.0414320379601492690078619743421f64,
    -0.0138106793200497563359539914474f64,
];
static mut h1_307: [libc::c_double; 16] = [
    0.0030210861012608841984899356291f64,
    -0.0090632583037826525954698068873f64,
    -0.0168317654213106405344439270765f64,
    0.0746639850740189951912512662623f64,
    0.0313329787073628846871956180962f64,
    -0.3011591259228349991008967259990f64,
    -0.0264992409453454699696117210896f64,
    0.9516421218971785225243297231697f64,
    0.9516421218971785225243297231697f64,
    -0.0264992409453454699696117210896f64,
    -0.3011591259228349991008967259990f64,
    0.0313329787073628846871956180962f64,
    0.0746639850740189951912512662623f64,
    -0.0168317654213106405344439270765f64,
    -0.0090632583037826525954698068873f64,
    0.0030210861012608841984899356291f64,
];
static mut g2_307: [libc::c_double; 16] = [
    -0.0030210861012608841984899356291f64,
    -0.0090632583037826525954698068873f64,
    0.0168317654213106405344439270765f64,
    0.0746639850740189951912512662623f64,
    -0.0313329787073628846871956180962f64,
    -0.3011591259228349991008967259990f64,
    0.0264992409453454699696117210896f64,
    0.9516421218971785225243297231697f64,
    -0.9516421218971785225243297231697f64,
    -0.0264992409453454699696117210896f64,
    0.3011591259228349991008967259990f64,
    0.0313329787073628846871956180962f64,
    -0.0746639850740189951912512662623f64,
    -0.0168317654213106405344439270765f64,
    0.0090632583037826525954698068873f64,
    0.0030210861012608841984899356291f64,
];
static mut h1_309: [libc::c_double; 20] = [
    -0.0006797443727836989446602355165f64,
    0.0020392331183510968339807065496f64,
    0.0050603192196119810324706421788f64,
    -0.0206189126411055346546938106687f64,
    -0.0141127879301758447558029850103f64,
    0.0991347824942321571990197448581f64,
    0.0123001362694193142367090236328f64,
    -0.3201919683607785695513833204624f64,
    0.0020500227115698857061181706055f64,
    0.9421257006782067372990864259380f64,
    0.9421257006782067372990864259380f64,
    0.0020500227115698857061181706055f64,
    -0.3201919683607785695513833204624f64,
    0.0123001362694193142367090236328f64,
    0.0991347824942321571990197448581f64,
    -0.0141127879301758447558029850103f64,
    -0.0206189126411055346546938106687f64,
    0.0050603192196119810324706421788f64,
    0.0020392331183510968339807065496f64,
    -0.0006797443727836989446602355165f64,
];
static mut g2_309: [libc::c_double; 20] = [
    0.0006797443727836989446602355165f64,
    0.0020392331183510968339807065496f64,
    -0.0050603192196119810324706421788f64,
    -0.0206189126411055346546938106687f64,
    0.0141127879301758447558029850103f64,
    0.0991347824942321571990197448581f64,
    -0.0123001362694193142367090236328f64,
    -0.3201919683607785695513833204624f64,
    -0.0020500227115698857061181706055f64,
    0.9421257006782067372990864259380f64,
    -0.9421257006782067372990864259380f64,
    0.0020500227115698857061181706055f64,
    0.3201919683607785695513833204624f64,
    0.0123001362694193142367090236328f64,
    -0.0991347824942321571990197448581f64,
    -0.0141127879301758447558029850103f64,
    0.0206189126411055346546938106687f64,
    0.0050603192196119810324706421788f64,
    -0.0020392331183510968339807065496f64,
    -0.0006797443727836989446602355165f64,
];
static mut h2_3: [libc::c_double; 20] = [
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.1767766952966368811002110905262f64,
    0.5303300858899106433006332715786f64,
    0.5303300858899106433006332715786f64,
    0.1767766952966368811002110905262f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
];
static mut g1_3: [libc::c_double; 20] = [
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    -0.1767766952966368811002110905262f64,
    0.5303300858899106433006332715786f64,
    -0.5303300858899106433006332715786f64,
    0.1767766952966368811002110905262f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
    0.0f64,
];
unsafe extern "C" fn bspline_init(
    mut h1: *mut *const libc::c_double,
    mut g1: *mut *const libc::c_double,
    mut h2: *mut *const libc::c_double,
    mut g2: *mut *const libc::c_double,
    mut nc: *mut size_t,
    mut offset: *mut size_t,
    mut member: size_t,
) -> libc::c_int {
    match member {
        103 => {
            *nc = 6 as libc::c_int as size_t;
            *h1 = h1_103.as_ptr();
            *g1 = &*g1_1.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_1.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_103.as_ptr();
        }
        105 => {
            *nc = 10 as libc::c_int as size_t;
            *h1 = h1_105.as_ptr();
            *g1 = g1_1.as_ptr();
            *h2 = h2_1.as_ptr();
            *g2 = g2_105.as_ptr();
        }
        202 => {
            *nc = 6 as libc::c_int as size_t;
            *h1 = h1_202.as_ptr();
            *g1 = &*g1_2.as_ptr().offset(6 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_2.as_ptr().offset(6 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_202.as_ptr();
        }
        204 => {
            *nc = 10 as libc::c_int as size_t;
            *h1 = h1_204.as_ptr();
            *g1 = &*g1_2.as_ptr().offset(4 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_2.as_ptr().offset(4 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_204.as_ptr();
        }
        206 => {
            *nc = 14 as libc::c_int as size_t;
            *h1 = h1_206.as_ptr();
            *g1 = &*g1_2.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_2.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_206.as_ptr();
        }
        208 => {
            *nc = 18 as libc::c_int as size_t;
            *h1 = h1_208.as_ptr();
            *g1 = g1_2.as_ptr();
            *h2 = h2_2.as_ptr();
            *g2 = g2_208.as_ptr();
        }
        301 => {
            *nc = 4 as libc::c_int as size_t;
            *h1 = h1_301.as_ptr();
            *g1 = &*g1_3.as_ptr().offset(8 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_3.as_ptr().offset(8 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_301.as_ptr();
        }
        303 => {
            *nc = 8 as libc::c_int as size_t;
            *h1 = h1_303.as_ptr();
            *g1 = &*g1_3.as_ptr().offset(6 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_3.as_ptr().offset(6 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_303.as_ptr();
        }
        305 => {
            *nc = 12 as libc::c_int as size_t;
            *h1 = h1_305.as_ptr();
            *g1 = &*g1_3.as_ptr().offset(4 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_3.as_ptr().offset(4 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_305.as_ptr();
        }
        307 => {
            *nc = 16 as libc::c_int as size_t;
            *h1 = h1_307.as_ptr();
            *g1 = &*g1_3.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_3.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_307.as_ptr();
        }
        309 => {
            *nc = 20 as libc::c_int as size_t;
            *h1 = h1_309.as_ptr();
            *g1 = g1_3.as_ptr();
            *h2 = h2_3.as_ptr();
            *g2 = g2_309.as_ptr();
        }
        _ => return GSL_FAILURE as libc::c_int,
    }
    *offset = 0 as libc::c_int as size_t;
    return GSL_SUCCESS as libc::c_int;
}
unsafe extern "C" fn bspline_centered_init(
    mut h1: *mut *const libc::c_double,
    mut g1: *mut *const libc::c_double,
    mut h2: *mut *const libc::c_double,
    mut g2: *mut *const libc::c_double,
    mut nc: *mut size_t,
    mut offset: *mut size_t,
    mut member: size_t,
) -> libc::c_int {
    match member {
        103 => {
            *nc = 6 as libc::c_int as size_t;
            *h1 = h1_103.as_ptr();
            *g1 = &*g1_1.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_1.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_103.as_ptr();
        }
        105 => {
            *nc = 10 as libc::c_int as size_t;
            *h1 = h1_105.as_ptr();
            *g1 = g1_1.as_ptr();
            *h2 = h2_1.as_ptr();
            *g2 = g2_105.as_ptr();
        }
        202 => {
            *nc = 6 as libc::c_int as size_t;
            *h1 = h1_202.as_ptr();
            *g1 = &*g1_2.as_ptr().offset(6 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_2.as_ptr().offset(6 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_202.as_ptr();
        }
        204 => {
            *nc = 10 as libc::c_int as size_t;
            *h1 = h1_204.as_ptr();
            *g1 = &*g1_2.as_ptr().offset(4 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_2.as_ptr().offset(4 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_204.as_ptr();
        }
        206 => {
            *nc = 14 as libc::c_int as size_t;
            *h1 = h1_206.as_ptr();
            *g1 = &*g1_2.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_2.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_206.as_ptr();
        }
        208 => {
            *nc = 18 as libc::c_int as size_t;
            *h1 = h1_208.as_ptr();
            *g1 = g1_2.as_ptr();
            *h2 = h2_2.as_ptr();
            *g2 = g2_208.as_ptr();
        }
        301 => {
            *nc = 4 as libc::c_int as size_t;
            *h1 = h1_301.as_ptr();
            *g1 = &*g1_3.as_ptr().offset(8 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_3.as_ptr().offset(8 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_301.as_ptr();
        }
        303 => {
            *nc = 8 as libc::c_int as size_t;
            *h1 = h1_303.as_ptr();
            *g1 = &*g1_3.as_ptr().offset(6 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_3.as_ptr().offset(6 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_303.as_ptr();
        }
        305 => {
            *nc = 12 as libc::c_int as size_t;
            *h1 = h1_305.as_ptr();
            *g1 = &*g1_3.as_ptr().offset(4 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_3.as_ptr().offset(4 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_305.as_ptr();
        }
        307 => {
            *nc = 16 as libc::c_int as size_t;
            *h1 = h1_307.as_ptr();
            *g1 = &*g1_3.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *h2 = &*h2_3.as_ptr().offset(2 as libc::c_int as isize)
                as *const libc::c_double;
            *g2 = g2_307.as_ptr();
        }
        309 => {
            *nc = 20 as libc::c_int as size_t;
            *h1 = h1_309.as_ptr();
            *g1 = g1_3.as_ptr();
            *h2 = h2_3.as_ptr();
            *g2 = g2_309.as_ptr();
        }
        _ => return GSL_FAILURE as libc::c_int,
    }
    *offset = *nc >> 1 as libc::c_int;
    return GSL_SUCCESS as libc::c_int;
}
static mut bspline_type: gsl_wavelet_type = {
    let mut init = gsl_wavelet_type {
        name: b"bspline\0" as *const u8 as *const libc::c_char,
        init: Some(
            bspline_init
                as unsafe extern "C" fn(
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut size_t,
                    *mut size_t,
                    size_t,
                ) -> libc::c_int,
        ),
    };
    init
};
static mut bspline_centered_type: gsl_wavelet_type = {
    let mut init = gsl_wavelet_type {
        name: b"bspline-centered\0" as *const u8 as *const libc::c_char,
        init: Some(
            bspline_centered_init
                as unsafe extern "C" fn(
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut *const libc::c_double,
                    *mut size_t,
                    *mut size_t,
                    size_t,
                ) -> libc::c_int,
        ),
    };
    init
};
pub static mut gsl_wavelet_bspline: *const gsl_wavelet_type = unsafe {
    &bspline_type as *const gsl_wavelet_type
};
pub static mut gsl_wavelet_bspline_centered: *const gsl_wavelet_type = unsafe {
    &bspline_centered_type as *const gsl_wavelet_type
};
