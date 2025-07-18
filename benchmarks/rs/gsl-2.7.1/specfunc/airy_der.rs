use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
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
static mut aif_data: [libc::c_double; 8] = [
    0.10527461226531408809f64,
    0.01183613628152997844f64,
    0.00012328104173225664f64,
    0.00000062261225638140f64,
    0.00000000185298887844f64,
    0.00000000000363328873f64,
    0.00000000000000504622f64,
    0.00000000000000000522f64,
];
static mut aif_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: aif_data.as_ptr() as *mut _,
            order: 7 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 7 as libc::c_int,
        };
        init
    }
};
static mut aig_data: [libc::c_double; 9] = [
    0.021233878150918666852f64,
    0.086315930335214406752f64,
    0.001797594720383231358f64,
    0.000014265499875550693f64,
    0.000000059437995283683f64,
    0.000000000152403366479f64,
    0.000000000000264587660f64,
    0.000000000000000331562f64,
    0.000000000000000000314f64,
];
static mut aig_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: aig_data.as_ptr() as *mut _,
            order: 8 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
static mut aip2_data: [libc::c_double; 15] = [
    0.0065457691989713757f64,
    0.0023833724120774592f64,
    -0.0000430700770220586f64,
    0.0000015629125858629f64,
    -0.0000000815417186163f64,
    0.0000000054103738057f64,
    -0.0000000004284130883f64,
    0.0000000000389497963f64,
    -0.0000000000039623161f64,
    0.0000000000004428184f64,
    -0.0000000000000536297f64,
    0.0000000000000069650f64,
    -0.0000000000000009620f64,
    0.0000000000000001403f64,
    -0.0000000000000000215f64,
];
static mut aip2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: aip2_data.as_ptr() as *mut _,
            order: 14 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut aip1_data: [libc::c_double; 25] = [
    0.0358865097808301538f64,
    0.0114668575627764899f64,
    -0.0007592073583861400f64,
    0.0000869517610893841f64,
    -0.0000128237294298592f64,
    0.0000022062695681038f64,
    -0.0000004222295185921f64,
    0.0000000874686415726f64,
    -0.0000000192773588418f64,
    0.0000000044668460054f64,
    -0.0000000010790108052f64,
    0.0000000002700029447f64,
    -0.0000000000696480108f64,
    0.0000000000184489907f64,
    -0.0000000000050027817f64,
    0.0000000000013852243f64,
    -0.0000000000003908218f64,
    0.0000000000001121536f64,
    -0.0000000000000326862f64,
    0.0000000000000096619f64,
    -0.0000000000000028935f64,
    0.0000000000000008770f64,
    -0.0000000000000002688f64,
    0.0000000000000000832f64,
    -0.0000000000000000260f64,
];
static mut aip1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: aip1_data.as_ptr() as *mut _,
            order: 24 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 14 as libc::c_int,
        };
        init
    }
};
static mut bif_data: [libc::c_double; 8] = [
    0.1153536790828570243f64,
    0.0205007894049192875f64,
    0.0002135290278902876f64,
    0.0000010783960614677f64,
    0.0000000032094708833f64,
    0.0000000000062930407f64,
    0.0000000000000087403f64,
    0.0000000000000000090f64,
];
static mut bif_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bif_data.as_ptr() as *mut _,
            order: 7 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 7 as libc::c_int,
        };
        init
    }
};
static mut big_data: [libc::c_double; 9] = [
    -0.097196440416443537390f64,
    0.149503576843167066571f64,
    0.003113525387121326042f64,
    0.000024708570579821297f64,
    0.000000102949627731379f64,
    0.000000000263970373987f64,
    0.000000000000458279271f64,
    0.000000000000000574283f64,
    0.000000000000000000544f64,
];
static mut big_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: big_data.as_ptr() as *mut _,
            order: 8 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
static mut bif2_data: [libc::c_double; 10] = [
    0.323493987603522033521f64,
    0.086297871535563559139f64,
    0.002994025552655397426f64,
    0.000051430528364661637f64,
    0.000000525840250036811f64,
    0.000000003561751373958f64,
    0.000000000017146864007f64,
    0.000000000000061663520f64,
    0.000000000000000171911f64,
    0.000000000000000000382f64,
];
static mut bif2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bif2_data.as_ptr() as *mut _,
            order: 9 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut big2_data: [libc::c_double; 10] = [
    1.6062999463621294578f64,
    0.7449088819876088652f64,
    0.0470138738610277380f64,
    0.0012284422062548239f64,
    0.0000173222412256624f64,
    0.0000001521901652368f64,
    0.0000000009113560249f64,
    0.0000000000039547918f64,
    0.0000000000000130017f64,
    0.0000000000000000335f64,
];
static mut big2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: big2_data.as_ptr() as *mut _,
            order: 9 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 9 as libc::c_int,
        };
        init
    }
};
static mut bip2_data: [libc::c_double; 29] = [
    -0.13269705443526630495f64,
    -0.00568443626045977481f64,
    -0.00015643601119611610f64,
    -0.00001136737203679562f64,
    -0.00000143464350991284f64,
    -0.00000018098531185164f64,
    0.00000000926177343611f64,
    0.00000001710005490721f64,
    0.00000000476698163504f64,
    -0.00000000035195022023f64,
    -0.00000000058890614316f64,
    -0.00000000006678499608f64,
    0.00000000006395565102f64,
    0.00000000001554529427f64,
    -0.00000000000792397000f64,
    -0.00000000000258326243f64,
    0.00000000000121655048f64,
    0.00000000000038707207f64,
    -0.00000000000022487045f64,
    -0.00000000000004953477f64,
    0.00000000000004563782f64,
    0.00000000000000332998f64,
    -0.00000000000000921750f64,
    0.00000000000000094157f64,
    0.00000000000000167154f64,
    -0.00000000000000055134f64,
    -0.00000000000000022369f64,
    0.00000000000000017487f64,
    0.00000000000000000207f64,
];
static mut bip2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bip2_data.as_ptr() as *mut _,
            order: 28 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 14 as libc::c_int,
        };
        init
    }
};
static mut bip1_data: [libc::c_double; 24] = [
    -0.1729187351079553719f64,
    -0.0149358492984694364f64,
    -0.0005471104951678566f64,
    0.0001537966292958408f64,
    0.0000154353476192179f64,
    -0.0000065434113851906f64,
    0.0000003728082407879f64,
    0.0000002072078388189f64,
    -0.0000000658173336470f64,
    0.0000000074926746354f64,
    0.0000000011101336884f64,
    -0.0000000007265140553f64,
    0.0000000001782723560f64,
    -0.0000000000217346352f64,
    -0.0000000000020302035f64,
    0.0000000000019311827f64,
    -0.0000000000006044953f64,
    0.0000000000001209450f64,
    -0.0000000000000125109f64,
    -0.0000000000000019917f64,
    0.0000000000000015154f64,
    -0.0000000000000004977f64,
    0.0000000000000001155f64,
    -0.0000000000000000186f64,
];
static mut bip1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: bip1_data.as_ptr() as *mut _,
            order: 23 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 13 as libc::c_int,
        };
        init
    }
};
static mut an22_data: [libc::c_double; 33] = [
    0.0537418629629794329f64,
    -0.0126661435859883193f64,
    -0.0011924334106593007f64,
    -0.0002032327627275655f64,
    -0.0000446468963075164f64,
    -0.0000113359036053123f64,
    -0.0000031641352378546f64,
    -0.0000009446708886149f64,
    -0.0000002966562236472f64,
    -0.0000000969118892024f64,
    -0.0000000326822538653f64,
    -0.0000000113144618964f64,
    -0.0000000040042691002f64,
    -0.0000000014440333684f64,
    -0.0000000005292853746f64,
    -0.0000000001967763374f64,
    -0.0000000000740800096f64,
    -0.0000000000282016314f64,
    -0.0000000000108440066f64,
    -0.0000000000042074801f64,
    -0.0000000000016459150f64,
    -0.0000000000006486827f64,
    -0.0000000000002574095f64,
    -0.0000000000001027889f64,
    -0.0000000000000412846f64,
    -0.0000000000000166711f64,
    -0.0000000000000067657f64,
    -0.0000000000000027585f64,
    -0.0000000000000011296f64,
    -0.0000000000000004645f64,
    -0.0000000000000001917f64,
    -0.0000000000000000794f64,
    -0.0000000000000000330f64,
];
static mut an22_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: an22_data.as_ptr() as *mut _,
            order: 32 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 18 as libc::c_int,
        };
        init
    }
};
static mut an21_data: [libc::c_double; 24] = [
    0.0198313155263169394f64,
    -0.0029376249067087533f64,
    -0.0001136260695958196f64,
    -0.0000100554451087156f64,
    -0.0000013048787116563f64,
    -0.0000002123881993151f64,
    -0.0000000402270833384f64,
    -0.0000000084996745953f64,
    -0.0000000019514839426f64,
    -0.0000000004783865344f64,
    -0.0000000001236733992f64,
    -0.0000000000334137486f64,
    -0.0000000000093702824f64,
    -0.0000000000027130128f64,
    -0.0000000000008075954f64,
    -0.0000000000002463214f64,
    -0.0000000000000767656f64,
    -0.0000000000000243883f64,
    -0.0000000000000078831f64,
    -0.0000000000000025882f64,
    -0.0000000000000008619f64,
    -0.0000000000000002908f64,
    -0.0000000000000000993f64,
    -0.0000000000000000343f64,
];
static mut an21_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: an21_data.as_ptr() as *mut _,
            order: 23 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 12 as libc::c_int,
        };
        init
    }
};
static mut an20_data: [libc::c_double; 16] = [
    0.0126732217145738027f64,
    -0.0005212847072615621f64,
    -0.0000052672111140370f64,
    -0.0000001628202185026f64,
    -0.0000000090991442687f64,
    -0.0000000007438647126f64,
    -0.0000000000795494752f64,
    -0.0000000000104050944f64,
    -0.0000000000015932426f64,
    -0.0000000000002770648f64,
    -0.0000000000000535343f64,
    -0.0000000000000113062f64,
    -0.0000000000000025772f64,
    -0.0000000000000006278f64,
    -0.0000000000000001621f64,
    -0.0000000000000000441f64,
];
static mut an20_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: an20_data.as_ptr() as *mut _,
            order: 15 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 8 as libc::c_int,
        };
        init
    }
};
static mut aph2_data: [libc::c_double; 32] = [
    -0.2057088719781465107f64,
    0.0422196961357771922f64,
    0.0020482560511207275f64,
    0.0002607800735165006f64,
    0.0000474824268004729f64,
    0.0000105102756431612f64,
    0.0000026353534014668f64,
    0.0000007208824863499f64,
    0.0000002103236664473f64,
    0.0000000644975634555f64,
    0.0000000205802377264f64,
    0.0000000067836273921f64,
    0.0000000022974015284f64,
    0.0000000007961306765f64,
    0.0000000002813860610f64,
    0.0000000001011749057f64,
    0.0000000000369306738f64,
    0.0000000000136615066f64,
    0.0000000000051142751f64,
    0.0000000000019351689f64,
    0.0000000000007393607f64,
    0.0000000000002849792f64,
    0.0000000000001107281f64,
    0.0000000000000433412f64,
    0.0000000000000170801f64,
    0.0000000000000067733f64,
    0.0000000000000027017f64,
    0.0000000000000010835f64,
    0.0000000000000004367f64,
    0.0000000000000001769f64,
    0.0000000000000000719f64,
    0.0000000000000000294f64,
];
static mut aph2_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: aph2_data.as_ptr() as *mut _,
            order: 31 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 16 as libc::c_int,
        };
        init
    }
};
static mut aph1_data: [libc::c_double; 22] = [
    -0.1024172908077571694f64,
    0.0071697275146591248f64,
    0.0001209959363122329f64,
    0.0000073361512841220f64,
    0.0000007535382954272f64,
    0.0000001041478171741f64,
    0.0000000174358728519f64,
    0.0000000033399795033f64,
    0.0000000007073075174f64,
    0.0000000001619187515f64,
    0.0000000000394539982f64,
    0.0000000000101192282f64,
    0.0000000000027092778f64,
    0.0000000000007523806f64,
    0.0000000000002156369f64,
    0.0000000000000635283f64,
    0.0000000000000191757f64,
    0.0000000000000059143f64,
    0.0000000000000018597f64,
    0.0000000000000005950f64,
    0.0000000000000001934f64,
    0.0000000000000000638f64,
];
static mut aph1_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: aph1_data.as_ptr() as *mut _,
            order: 21 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 10 as libc::c_int,
        };
        init
    }
};
static mut aph0_data: [libc::c_double; 15] = [
    -0.0855849241130933257f64,
    0.0011214378867065261f64,
    0.0000042721029353664f64,
    0.0000000817607381483f64,
    0.0000000033907645000f64,
    0.0000000002253264423f64,
    0.0000000000206284209f64,
    0.0000000000023858763f64,
    0.0000000000003301618f64,
    0.0000000000000527010f64,
    0.0000000000000094555f64,
    0.0000000000000018709f64,
    0.0000000000000004024f64,
    0.0000000000000000930f64,
    0.0000000000000000229f64,
];
static mut aph0_cs: cheb_series = unsafe {
    {
        let mut init = cheb_series_struct {
            c: aph0_data.as_ptr() as *mut _,
            order: 14 as libc::c_int,
            a: -(1 as libc::c_int) as libc::c_double,
            b: 1 as libc::c_int as libc::c_double,
            order_sp: 7 as libc::c_int,
        };
        init
    }
};
unsafe extern "C" fn airy_deriv_mod_phase(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut ampl: *mut gsl_sf_result,
    mut phi: *mut gsl_sf_result,
) -> libc::c_int {
    let pi34: libc::c_double = 2.356194490192344928847f64;
    let mut result_a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut result_p: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut a: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut sqx: libc::c_double = 0.;
    if x <= -4.0f64 {
        let mut z: libc::c_double = 128.0f64 / (x * x * x) + 1.0f64;
        cheb_eval_mode_e(&mut an20_cs, z, mode, &mut result_a);
        cheb_eval_mode_e(&mut aph0_cs, z, mode, &mut result_p);
    } else if x <= -2.0f64 {
        let mut z_0: libc::c_double = (128.0f64 / (x * x * x) + 9.0f64) / 7.0f64;
        cheb_eval_mode_e(&mut an21_cs, z_0, mode, &mut result_a);
        cheb_eval_mode_e(&mut aph1_cs, z_0, mode, &mut result_p);
    } else if x <= -1.0f64 {
        let mut z_1: libc::c_double = (16.0f64 / (x * x * x) + 9.0f64) / 7.0f64;
        cheb_eval_mode_e(&mut an22_cs, z_1, mode, &mut result_a);
        cheb_eval_mode_e(&mut aph2_cs, z_1, mode, &mut result_p);
    } else {
        (*ampl).val = 0.0f64;
        (*ampl).err = 0.0f64;
        (*phi).val = 0.0f64;
        (*phi).err = 0.0f64;
        gsl_error(
            b"x is greater than 1.0\0" as *const u8 as *const libc::c_char,
            b"airy_der.c\0" as *const u8 as *const libc::c_char,
            616 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    a = 0.3125f64 + result_a.val;
    p = -0.625f64 + result_p.val;
    sqx = sqrt(-x);
    (*ampl).val = sqrt(a * sqx);
    (*ampl)
        .err = fabs((*ampl).val)
        * (2.2204460492503131e-16f64 + fabs(result_a.err / result_a.val));
    (*phi).val = pi34 - x * sqx * p;
    (*phi)
        .err = fabs((*phi).val)
        * (2.2204460492503131e-16f64 + fabs(result_p.err / result_p.val));
    return GSL_SUCCESS as libc::c_int;
}
pub unsafe extern "C" fn gsl_sf_airy_Ai_deriv_scaled_e(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -1.0f64 {
        let mut a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut p: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status_ap: libc::c_int = airy_deriv_mod_phase(x, mode, &mut a, &mut p);
        let mut c: libc::c_double = cos(p.val);
        (*result).val = a.val * c;
        (*result).err = fabs((*result).val * p.err) + fabs(c * a.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return status_ap;
    } else if x <= 1.0f64 {
        let x3: libc::c_double = x * x * x;
        let x2: libc::c_double = x * x;
        let mut result_c0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut aif_cs, x3, mode, &mut result_c0);
        cheb_eval_mode_e(&mut aig_cs, x3, mode, &mut result_c1);
        (*result).val = x2 * (0.125f64 + result_c0.val) - result_c1.val - 0.25f64;
        (*result).err = fabs(x2 * result_c0.val) + result_c1.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        if x > 6.0554544523933429e-06f64 * 6.0554544523933429e-06f64 {
            let mut s: libc::c_double = exp(2.0f64 * x * sqrt(x) / 3.0f64);
            (*result).val *= s;
            (*result).err *= s;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x <= 4.0f64 {
        let sqrtx: libc::c_double = sqrt(x);
        let z: libc::c_double = (16.0f64 / (x * sqrtx) - 9.0f64) / 7.0f64;
        let s_0: libc::c_double = sqrt(sqrtx);
        let mut result_c0_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut aip1_cs, z, mode, &mut result_c0_0);
        (*result).val = -(0.28125f64 + result_c0_0.val) * s_0;
        (*result).err = result_c0_0.err * s_0;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let sqrtx_0: libc::c_double = sqrt(x);
        let z_0: libc::c_double = 16.0f64 / (x * sqrtx_0) - 1.0f64;
        let s_1: libc::c_double = sqrt(sqrtx_0);
        let mut result_c0_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut aip2_cs, z_0, mode, &mut result_c0_1);
        (*result).val = -(0.28125f64 + result_c0_1.val) * s_1;
        (*result).err = result_c0_1.err * s_1;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_airy_Ai_deriv_e(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -1.0f64 {
        let mut a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut p: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status_ap: libc::c_int = airy_deriv_mod_phase(x, mode, &mut a, &mut p);
        let mut c: libc::c_double = cos(p.val);
        (*result).val = a.val * c;
        (*result).err = fabs((*result).val * p.err) + fabs(c * a.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return status_ap;
    } else if x < 1.0f64 {
        let x3: libc::c_double = x * x * x;
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut aif_cs, x3, mode, &mut result_c1);
        cheb_eval_mode_e(&mut aig_cs, x3, mode, &mut result_c2);
        (*result).val = x * x * (0.125f64 + result_c1.val) - result_c2.val - 0.25f64;
        (*result).err = fabs(x * x * result_c1.err) + result_c2.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x * x * x
        < 9.0f64 / 4.0f64 * -7.0839641853226408e+02f64 * -7.0839641853226408e+02f64
    {
        let mut result_aps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let arg: libc::c_double = -2.0f64 * x * sqrt(x) / 3.0f64;
        let stat_a: libc::c_int = gsl_sf_airy_Ai_deriv_scaled_e(
            x,
            mode,
            &mut result_aps,
        );
        let stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            arg,
            1.5f64 * fabs(arg * 2.2204460492503131e-16f64),
            result_aps.val,
            result_aps.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_a != GSL_SUCCESS as libc::c_int {
            stat_a
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        (*result).val = 0.0f64;
        (*result).err = 2.2250738585072014e-308f64;
        gsl_error(
            b"underflow\0" as *const u8 as *const libc::c_char,
            b"airy_der.c\0" as *const u8 as *const libc::c_char,
            732 as libc::c_int,
            GSL_EUNDRFLW as libc::c_int,
        );
        return GSL_EUNDRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_airy_Bi_deriv_scaled_e(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    let atr: libc::c_double = 8.7506905708484345f64;
    let btr: libc::c_double = -2.0938363213560543f64;
    if x < -1.0f64 {
        let mut a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut p: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status_ap: libc::c_int = airy_deriv_mod_phase(x, mode, &mut a, &mut p);
        let mut s: libc::c_double = sin(p.val);
        (*result).val = a.val * s;
        (*result).err = fabs((*result).val * p.err) + fabs(s * a.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return status_ap;
    } else if x < 1.0f64 {
        let x3: libc::c_double = x * x * x;
        let x2: libc::c_double = x * x;
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bif_cs, x3, mode, &mut result_c1);
        cheb_eval_mode_e(&mut big_cs, x3, mode, &mut result_c2);
        (*result).val = x2 * (result_c1.val + 0.25f64) + result_c2.val + 0.5f64;
        (*result).err = x2 * result_c1.err + result_c2.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        if x > 6.0554544523933429e-06f64 * 6.0554544523933429e-06f64 {
            let s_0: libc::c_double = exp(-2.0f64 * x * sqrt(x) / 3.0f64);
            (*result).val *= s_0;
            (*result).err *= s_0;
        }
        return GSL_SUCCESS as libc::c_int;
    } else if x < 2.0f64 {
        let z: libc::c_double = (2.0f64 * x * x * x - 9.0f64) / 7.0f64;
        let s_1: libc::c_double = exp(-2.0f64 * x * sqrt(x) / 3.0f64);
        let mut result_c0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bif2_cs, z, mode, &mut result_c0);
        cheb_eval_mode_e(&mut big2_cs, z, mode, &mut result_c1_0);
        (*result)
            .val = s_1 * (x * x * (0.25f64 + result_c0.val) + 0.5f64 + result_c1_0.val);
        (*result).err = s_1 * (x * x * result_c0.err + result_c1_0.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 4.0f64 {
        let sqrtx: libc::c_double = sqrt(x);
        let z_0: libc::c_double = atr / (x * sqrtx) + btr;
        let s_2: libc::c_double = sqrt(sqrtx);
        let mut result_c0_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bip1_cs, z_0, mode, &mut result_c0_0);
        (*result).val = s_2 * (0.625f64 + result_c0_0.val);
        (*result).err = s_2 * result_c0_0.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else {
        let sqrtx_0: libc::c_double = sqrt(x);
        let z_1: libc::c_double = 16.0f64 / (x * sqrtx_0) - 1.0f64;
        let s_3: libc::c_double = sqrt(sqrtx_0);
        let mut result_c0_1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bip2_cs, z_1, mode, &mut result_c0_1);
        (*result).val = s_3 * (0.625f64 + result_c0_1.val);
        (*result).err = s_3 * result_c0_1.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_airy_Bi_deriv_e(
    x: libc::c_double,
    mut mode: gsl_mode_t,
    mut result: *mut gsl_sf_result,
) -> libc::c_int {
    if x < -1.0f64 {
        let mut a: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut p: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut status_ap: libc::c_int = airy_deriv_mod_phase(x, mode, &mut a, &mut p);
        let mut s: libc::c_double = sin(p.val);
        (*result).val = a.val * s;
        (*result).err = fabs((*result).val * p.err) + fabs(s * a.err);
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return status_ap;
    } else if x < 1.0f64 {
        let x3: libc::c_double = x * x * x;
        let x2: libc::c_double = x * x;
        let mut result_c1: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c2: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bif_cs, x3, mode, &mut result_c1);
        cheb_eval_mode_e(&mut big_cs, x3, mode, &mut result_c2);
        (*result).val = x2 * (result_c1.val + 0.25f64) + result_c2.val + 0.5f64;
        (*result).err = x2 * result_c1.err + result_c2.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 2.0f64 {
        let z: libc::c_double = (2.0f64 * x * x * x - 9.0f64) / 7.0f64;
        let mut result_c1_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let mut result_c2_0: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        cheb_eval_mode_e(&mut bif2_cs, z, mode, &mut result_c1_0);
        cheb_eval_mode_e(&mut big2_cs, z, mode, &mut result_c2_0);
        (*result).val = x * x * (result_c1_0.val + 0.25f64) + result_c2_0.val + 0.5f64;
        (*result).err = x * x * result_c1_0.err + result_c2_0.err;
        (*result).err += 2.2204460492503131e-16f64 * fabs((*result).val);
        return GSL_SUCCESS as libc::c_int;
    } else if x < 5.6438030941222897e+102f64 * 5.6438030941222897e+102f64 {
        let mut result_bps: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
        let arg: libc::c_double = 2.0f64 * (x * sqrt(x) / 3.0f64);
        let mut stat_b: libc::c_int = gsl_sf_airy_Bi_deriv_scaled_e(
            x,
            mode,
            &mut result_bps,
        );
        let mut stat_e: libc::c_int = gsl_sf_exp_mult_err_e(
            arg,
            1.5f64 * fabs(arg * 2.2204460492503131e-16f64),
            result_bps.val,
            result_bps.err,
            result,
        );
        return if stat_e != GSL_SUCCESS as libc::c_int {
            stat_e
        } else if stat_b != GSL_SUCCESS as libc::c_int {
            stat_b
        } else {
            GSL_SUCCESS as libc::c_int
        };
    } else {
        (*result).val = ::std::f32::INFINITY as libc::c_double;
        (*result).err = ::std::f32::INFINITY as libc::c_double;
        gsl_error(
            b"overflow\0" as *const u8 as *const libc::c_char,
            b"airy_der.c\0" as *const u8 as *const libc::c_char,
            860 as libc::c_int,
            GSL_EOVRFLW as libc::c_int,
        );
        return GSL_EOVRFLW as libc::c_int;
    };
}
pub unsafe extern "C" fn gsl_sf_airy_Ai_deriv_scaled(
    x: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_Ai_deriv_scaled_e(x, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_Ai_deriv_scaled_e(x, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"airy_der.c\0" as *const u8 as *const libc::c_char,
            870 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_airy_Ai_deriv(
    x: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_Ai_deriv_e(x, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_Ai_deriv_e(x, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"airy_der.c\0" as *const u8 as *const libc::c_char,
            875 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_airy_Bi_deriv_scaled(
    x: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_Bi_deriv_scaled_e(x, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_Bi_deriv_scaled_e(x, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"airy_der.c\0" as *const u8 as *const libc::c_char,
            880 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
pub unsafe extern "C" fn gsl_sf_airy_Bi_deriv(
    x: libc::c_double,
    mut mode: gsl_mode_t,
) -> libc::c_double {
    let mut result: gsl_sf_result = gsl_sf_result { val: 0., err: 0. };
    let mut status: libc::c_int = gsl_sf_airy_Bi_deriv_e(x, mode, &mut result);
    if status != GSL_SUCCESS as libc::c_int {
        gsl_error(
            b"gsl_sf_airy_Bi_deriv_e(x, mode, &result)\0" as *const u8
                as *const libc::c_char,
            b"airy_der.c\0" as *const u8 as *const libc::c_char,
            885 as libc::c_int,
            status,
        );
        return result.val;
    }
    return result.val;
}
