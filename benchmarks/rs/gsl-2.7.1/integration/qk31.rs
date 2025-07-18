use ::libc;
extern "C" {
    fn gsl_integration_qk(
        n: libc::c_int,
        xgk_0: *const libc::c_double,
        wg_0: *const libc::c_double,
        wgk_0: *const libc::c_double,
        fv1: *mut libc::c_double,
        fv2: *mut libc::c_double,
        f: *const gsl_function,
        a: libc::c_double,
        b: libc::c_double,
        result: *mut libc::c_double,
        abserr: *mut libc::c_double,
        resabs: *mut libc::c_double,
        resasc: *mut libc::c_double,
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
static mut xgk: [libc::c_double; 16] = [
    0.998002298693397060285172840152271f64,
    0.987992518020485428489565718586613f64,
    0.967739075679139134257347978784337f64,
    0.937273392400705904307758947710209f64,
    0.897264532344081900882509656454496f64,
    0.848206583410427216200648320774217f64,
    0.790418501442465932967649294817947f64,
    0.724417731360170047416186054613938f64,
    0.650996741297416970533735895313275f64,
    0.570972172608538847537226737253911f64,
    0.485081863640239680693655740232351f64,
    0.394151347077563369897207370981045f64,
    0.299180007153168812166780024266389f64,
    0.201194093997434522300628303394596f64,
    0.101142066918717499027074231447392f64,
    0.000000000000000000000000000000000f64,
];
static mut wg: [libc::c_double; 8] = [
    0.030753241996117268354628393577204f64,
    0.070366047488108124709267416450667f64,
    0.107159220467171935011869546685869f64,
    0.139570677926154314447804794511028f64,
    0.166269205816993933553200860481209f64,
    0.186161000015562211026800561866423f64,
    0.198431485327111576456118326443839f64,
    0.202578241925561272880620199967519f64,
];
static mut wgk: [libc::c_double; 16] = [
    0.005377479872923348987792051430128f64,
    0.015007947329316122538374763075807f64,
    0.025460847326715320186874001019653f64,
    0.035346360791375846222037948478360f64,
    0.044589751324764876608227299373280f64,
    0.053481524690928087265343147239430f64,
    0.062009567800670640285139230960803f64,
    0.069854121318728258709520077099147f64,
    0.076849680757720378894432777482659f64,
    0.083080502823133021038289247286104f64,
    0.088564443056211770647275443693774f64,
    0.093126598170825321225486872747346f64,
    0.096642726983623678505179907627589f64,
    0.099173598721791959332393173484603f64,
    0.100769845523875595044946662617570f64,
    0.101330007014791549017374792767493f64,
];
pub unsafe extern "C" fn gsl_integration_qk31(
    mut f: *const gsl_function,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut result: *mut libc::c_double,
    mut abserr: *mut libc::c_double,
    mut resabs: *mut libc::c_double,
    mut resasc: *mut libc::c_double,
) {
    let mut fv1: [libc::c_double; 16] = [0.; 16];
    let mut fv2: [libc::c_double; 16] = [0.; 16];
    gsl_integration_qk(
        16 as libc::c_int,
        xgk.as_ptr(),
        wg.as_ptr(),
        wgk.as_ptr(),
        fv1.as_mut_ptr(),
        fv2.as_mut_ptr(),
        f,
        a,
        b,
        result,
        abserr,
        resabs,
        resasc,
    );
}
