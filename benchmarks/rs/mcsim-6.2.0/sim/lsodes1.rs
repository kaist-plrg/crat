use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn jgroup_(
        n: *mut libc::c_long,
        ia: *mut libc::c_long,
        ja: *mut libc::c_long,
        maxg: *mut libc::c_long,
        ngrp: *mut libc::c_long,
        igp: *mut libc::c_long,
        jgp: *mut libc::c_long,
        incl: *mut libc::c_long,
        jdone: *mut libc::c_long,
        ier: *mut libc::c_long,
    ) -> libc::c_int;
    fn odrv_(
        n: *mut libc::c_long,
        ia: *mut libc::c_long,
        ja: *mut libc::c_long,
        a: *mut libc::c_double,
        p: *mut libc::c_long,
        ip: *mut libc::c_long,
        nsp: *mut libc::c_long,
        isp: *mut libc::c_long,
        path: libc::c_long,
        flag_: *mut libc::c_long,
    ) -> libc::c_int;
    fn ewset_(
        _: *mut libc::c_long,
        _: *mut libc::c_long,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
    ) -> libc::c_int;
    fn nroc_(
        n: *mut libc::c_long,
        ic: *mut libc::c_long,
        ia: *mut libc::c_long,
        ja: *mut libc::c_long,
        a: *mut libc::c_double,
        jar: *mut libc::c_long,
        ar: *mut libc::c_double,
        p: *mut libc::c_long,
        flag_: *mut libc::c_long,
    ) -> libc::c_int;
    fn nsfc_(
        n: *mut libc::c_long,
        r: *mut libc::c_long,
        ic: *mut libc::c_long,
        ia: *mut libc::c_long,
        ja: *mut libc::c_long,
        jlmax: *mut libc::c_long,
        il: *mut libc::c_long,
        jl: *mut libc::c_long,
        ijl: *mut libc::c_long,
        jumax: *mut libc::c_long,
        iu: *mut libc::c_long,
        ju: *mut libc::c_long,
        iju: *mut libc::c_long,
        q: *mut libc::c_long,
        ira: *mut libc::c_long,
        jra: *mut libc::c_long,
        irac: *mut libc::c_long,
        irl: *mut libc::c_long,
        jrl: *mut libc::c_long,
        iru: *mut libc::c_long,
        jru: *mut libc::c_long,
        flag_: *mut libc::c_long,
    ) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn abort() -> !;
    fn CalcDeriv(
        rgModelVars: *mut libc::c_double,
        rgDerivs: *mut libc::c_double,
        pdTime: PDOUBLE,
    );
    fn CalcJacob(
        pdTime: PDOUBLE,
        rgModelVars: *mut libc::c_double,
        column: libc::c_long,
        rgdJac: *mut libc::c_double,
    );
    fn StoreDelayed(t: libc::c_double);
    static mut bDelays: BOOL;
    fn DoStep_by_Step();
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub conit: libc::c_double,
    pub crate_0: libc::c_double,
    pub el: [libc::c_double; 13],
    pub elco: [libc::c_double; 156],
    pub hold: libc::c_double,
    pub rmax: libc::c_double,
    pub tesco: [libc::c_double; 36],
    pub ccmax: libc::c_double,
    pub el0: libc::c_double,
    pub h: libc::c_double,
    pub hmin: libc::c_double,
    pub hmxi: libc::c_double,
    pub hu: libc::c_double,
    pub rc: libc::c_double,
    pub tn: libc::c_double,
    pub uround: libc::c_double,
    pub con0: libc::c_double,
    pub conmin: libc::c_double,
    pub ccmxj: libc::c_double,
    pub psmall: libc::c_double,
    pub rbig: libc::c_double,
    pub seth: libc::c_double,
    pub iplost: libc::c_long,
    pub iesp: libc::c_long,
    pub istatc: libc::c_long,
    pub iys: libc::c_long,
    pub iba: libc::c_long,
    pub ibian: libc::c_long,
    pub ibjan: libc::c_long,
    pub ibjgp: libc::c_long,
    pub ipian: libc::c_long,
    pub ipjan: libc::c_long,
    pub ipjgp: libc::c_long,
    pub ipigp: libc::c_long,
    pub ipr: libc::c_long,
    pub ipc: libc::c_long,
    pub ipic: libc::c_long,
    pub ipisp: libc::c_long,
    pub iprsp: libc::c_long,
    pub ipa: libc::c_long,
    pub lenyh: libc::c_long,
    pub lenyhm: libc::c_long,
    pub lenwk: libc::c_long,
    pub lreq: libc::c_long,
    pub lrat: libc::c_long,
    pub lrest: libc::c_long,
    pub lwmin: libc::c_long,
    pub moss: libc::c_long,
    pub msbj: libc::c_long,
    pub nslj: libc::c_long,
    pub ngp: libc::c_long,
    pub nlu: libc::c_long,
    pub nnz: libc::c_long,
    pub nsp: libc::c_long,
    pub nzl: libc::c_long,
    pub nzu: libc::c_long,
    pub illin: libc::c_long,
    pub init: libc::c_long,
    pub lyh: libc::c_long,
    pub lewt: libc::c_long,
    pub lacor: libc::c_long,
    pub lsavf: libc::c_long,
    pub lwm: libc::c_long,
    pub mxstep: libc::c_long,
    pub mxhnil: libc::c_long,
    pub nhnil: libc::c_long,
    pub ntrep: libc::c_long,
    pub nslast: libc::c_long,
    pub nyh: libc::c_long,
    pub ialth: libc::c_long,
    pub ipup: libc::c_long,
    pub lmax: libc::c_long,
    pub meo: libc::c_long,
    pub nqnyh: libc::c_long,
    pub nslp: libc::c_long,
    pub icf: libc::c_long,
    pub ierpj: libc::c_long,
    pub iersl: libc::c_long,
    pub jcur: libc::c_long,
    pub jstart: libc::c_long,
    pub kflag: libc::c_long,
    pub l: libc::c_long,
    pub meth: libc::c_long,
    pub miter: libc::c_long,
    pub maxord: libc::c_long,
    pub maxcor: libc::c_long,
    pub msbp: libc::c_long,
    pub mxncf: libc::c_long,
    pub n: libc::c_long,
    pub nq: libc::c_long,
    pub nst: libc::c_long,
    pub nfe: libc::c_long,
    pub nje: libc::c_long,
    pub nqu: libc::c_long,
}
pub type PDOUBLE = *mut libc::c_double;
pub type BOOL = libc::c_int;
pub static mut IGS: C2RustUnnamed = C2RustUnnamed {
    conit: 0.,
    crate_0: 0.,
    el: [0.; 13],
    elco: [0.; 156],
    hold: 0.,
    rmax: 0.,
    tesco: [0.; 36],
    ccmax: 0.,
    el0: 0.,
    h: 0.,
    hmin: 0.,
    hmxi: 0.,
    hu: 0.,
    rc: 0.,
    tn: 0.,
    uround: 0.,
    con0: 0.,
    conmin: 0.,
    ccmxj: 0.,
    psmall: 0.,
    rbig: 0.,
    seth: 0.,
    iplost: 0,
    iesp: 0,
    istatc: 0,
    iys: 0,
    iba: 0,
    ibian: 0,
    ibjan: 0,
    ibjgp: 0,
    ipian: 0,
    ipjan: 0,
    ipjgp: 0,
    ipigp: 0,
    ipr: 0,
    ipc: 0,
    ipic: 0,
    ipisp: 0,
    iprsp: 0,
    ipa: 0,
    lenyh: 0,
    lenyhm: 0,
    lenwk: 0,
    lreq: 0,
    lrat: 0,
    lrest: 0,
    lwmin: 0,
    moss: 0,
    msbj: 0,
    nslj: 0,
    ngp: 0,
    nlu: 0,
    nnz: 0,
    nsp: 0,
    nzl: 0,
    nzu: 0,
    illin: 0,
    init: 0,
    lyh: 0,
    lewt: 0,
    lacor: 0,
    lsavf: 0,
    lwm: 0,
    mxstep: 0,
    mxhnil: 0,
    nhnil: 0,
    ntrep: 0,
    nslast: 0,
    nyh: 0,
    ialth: 0,
    ipup: 0,
    lmax: 0,
    meo: 0,
    nqnyh: 0,
    nslp: 0,
    icf: 0,
    ierpj: 0,
    iersl: 0,
    jcur: 0,
    jstart: 0,
    kflag: 0,
    l: 0,
    meth: 0,
    miter: 0,
    maxord: 0,
    maxcor: 0,
    msbp: 0,
    mxncf: 0,
    n: 0,
    nq: 0,
    nst: 0,
    nfe: 0,
    nje: 0,
    nqu: 0,
};
pub unsafe extern "C" fn d_sign(
    mut pa1: *mut libc::c_double,
    mut pa2: *mut libc::c_double,
) -> libc::c_double {
    let mut a3: libc::c_double = 0.;
    a3 = if *pa1 >= 0 as libc::c_int as libc::c_double { *pa1 } else { -*pa1 };
    if *pa2 >= 0 as libc::c_int as libc::c_double {
        return a3;
    }
    return -a3;
}
pub unsafe extern "C" fn lsodes_(
    mut neq: *mut libc::c_long,
    mut y: *mut libc::c_double,
    mut t: *mut libc::c_double,
    mut tout: *mut libc::c_double,
    mut itol: *mut libc::c_long,
    mut rtol: *mut libc::c_double,
    mut atol: *mut libc::c_double,
    mut itask: *mut libc::c_long,
    mut istate: *mut libc::c_long,
    mut iopt: *mut libc::c_long,
    mut rwork: *mut libc::c_double,
    mut lrw: *mut libc::c_long,
    mut iwork: *mut libc::c_long,
    mut liw: *mut libc::c_long,
    mut mf: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    static mut mord: [libc::c_long; 2] = [
        12 as libc::c_int as libc::c_long,
        5 as libc::c_int as libc::c_long,
    ];
    static mut mxstp0: libc::c_long = 500 as libc::c_int as libc::c_long;
    static mut mxhnl0: libc::c_long = 10 as libc::c_int as libc::c_long;
    let mut i__1: libc::c_long = 0;
    let mut i__2: libc::c_long = 0;
    let mut d__1: libc::c_double = 0.;
    let mut d__2: libc::c_double = 0.;
    let mut irem: libc::c_long = 0;
    let mut imax: libc::c_long = 0;
    let mut imul: libc::c_long = 0;
    let mut ipgo: libc::c_long = 0;
    let mut lyhd: libc::c_long = 0;
    let mut lyhn: libc::c_long = 0;
    let mut ewti: libc::c_double = 0.;
    let mut hmax: libc::c_double = 0.;
    let mut size: libc::c_double = 0.;
    let mut ihit: libc::c_long = 1 as libc::c_int as libc::c_long;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut iflag: libc::c_long = 0;
    let mut leniw: libc::c_long = 0;
    let mut imxer: libc::c_long = 0;
    let mut lenrw: libc::c_long = 0;
    let mut i1: libc::c_long = 0;
    let mut i2: libc::c_long = 0;
    let mut lrtem: libc::c_long = 0;
    let mut lwtem: libc::c_long = 0;
    let mut ncolm: libc::c_long = 0;
    let mut atoli: libc::c_double = 0.;
    let mut h0: libc::c_double = 0.;
    let mut rtoli: libc::c_double = 0.;
    let mut tcrit: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tdist: libc::c_double = 0.;
    let mut tnext: libc::c_double = 0.;
    let mut tolsf: libc::c_double = 0.;
    let mut w0: libc::c_double = 0.;
    let mut rh: libc::c_double = 0.;
    let mut ipflag: libc::c_long = 0;
    let mut tp: libc::c_double = 0.;
    let mut lf0: libc::c_long = 0;
    let mut lenyht: libc::c_long = 0;
    let mut mf1: libc::c_long = 0;
    let mut big: libc::c_double = 0.;
    let mut lia: libc::c_long = 0;
    let mut lja: libc::c_long = 0;
    let mut kgo: libc::c_long = 0;
    let mut ayi: libc::c_double = 0.;
    let mut hmx: libc::c_double = 0.;
    let mut tol: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut lenrat: libc::c_long = 0;
    lenrat = (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
        as libc::c_long;
    if lenrat < 1 as libc::c_int as libc::c_long {
        lenrat = 1 as libc::c_int as libc::c_long;
    }
    iwork = iwork.offset(-1);
    iwork;
    rwork = rwork.offset(-1);
    rwork;
    atol = atol.offset(-1);
    atol;
    rtol = rtol.offset(-1);
    rtol;
    y = y.offset(-1);
    y;
    neq = neq.offset(-1);
    neq;
    if *istate < 1 as libc::c_int as libc::c_long
        || *istate > 3 as libc::c_int as libc::c_long
    {
        printf(
            b"lsodes: istate = %ld is illegal.\n\0" as *const u8 as *const libc::c_char,
            *istate,
        );
        current_block = 2196110567356697731;
    } else if *itask < 1 as libc::c_int as libc::c_long
        || *itask > 5 as libc::c_int as libc::c_long
    {
        printf(
            b"lsodes: itask = %ld illegal.\n\0" as *const u8 as *const libc::c_char,
            *itask,
        );
        current_block = 2196110567356697731;
    } else {
        if *istate == 1 as libc::c_int as libc::c_long {
            IGS.init = 0 as libc::c_int as libc::c_long;
            if *tout == *t {
                IGS.ntrep += 1;
                IGS.ntrep;
                if IGS.ntrep < 5 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int;
                }
                printf(
                    b"lsodes: repeated calls with istate = 1 and tout = t = %21.13f.\n\0"
                        as *const u8 as *const libc::c_char,
                    *t,
                );
                current_block = 14574620217021738995;
            } else {
                current_block = 17396894775415404740;
            }
        } else if IGS.init == 0 as libc::c_int as libc::c_long {
            printf(
                b"lsodes: istate is > 1 but lsodes is not initialized.\n\0" as *const u8
                    as *const libc::c_char,
            );
            current_block = 2196110567356697731;
        } else if *istate == 2 as libc::c_int as libc::c_long {
            current_block = 16977092573463915408;
        } else {
            current_block = 17396894775415404740;
        }
        match current_block {
            14574620217021738995 => {}
            2196110567356697731 => {}
            _ => {
                match current_block {
                    17396894775415404740 => {
                        IGS.ntrep = 0 as libc::c_int as libc::c_long;
                        if *neq.offset(1 as libc::c_int as isize)
                            <= 0 as libc::c_int as libc::c_long
                        {
                            printf(
                                b"lsodes: neq = %ld is lower than 1.\n\0" as *const u8
                                    as *const libc::c_char,
                                *neq.offset(1 as libc::c_int as isize),
                            );
                            current_block = 2196110567356697731;
                        } else {
                            if *istate == 1 as libc::c_int as libc::c_long {
                                current_block = 1120863650783119692;
                            } else if *neq.offset(1 as libc::c_int as isize) > IGS.n {
                                printf(
                                    b"lsodes: istate = 3 and neq increased from %ld to %ld.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    IGS.n,
                                    *neq.offset(1 as libc::c_int as isize),
                                );
                                current_block = 2196110567356697731;
                            } else {
                                current_block = 1120863650783119692;
                            }
                            match current_block {
                                2196110567356697731 => {}
                                _ => {
                                    IGS.n = *neq.offset(1 as libc::c_int as isize);
                                    if *itol < 1 as libc::c_int as libc::c_long
                                        || *itol > 4 as libc::c_int as libc::c_long
                                    {
                                        printf(
                                            b"lsodes: itol = %ld is illegal.\n\0" as *const u8
                                                as *const libc::c_char,
                                            *itol,
                                        );
                                        current_block = 2196110567356697731;
                                    } else if *iopt < 0 as libc::c_int as libc::c_long
                                        || *iopt > 1 as libc::c_int as libc::c_long
                                    {
                                        printf(
                                            b"lsodes: iopt = %ld is illegal.\n\0" as *const u8
                                                as *const libc::c_char,
                                            *iopt,
                                        );
                                        current_block = 2196110567356697731;
                                    } else {
                                        IGS.moss = *mf / 100 as libc::c_int as libc::c_long;
                                        mf1 = *mf - IGS.moss * 100 as libc::c_int as libc::c_long;
                                        IGS.meth = mf1 / 10 as libc::c_int as libc::c_long;
                                        IGS
                                            .miter = mf1 - IGS.meth * 10 as libc::c_int as libc::c_long;
                                        if IGS.moss < 0 as libc::c_int as libc::c_long
                                            || IGS.moss > 2 as libc::c_int as libc::c_long
                                        {
                                            current_block = 5402624238003992709;
                                        } else if IGS.meth < 1 as libc::c_int as libc::c_long
                                            || IGS.meth > 2 as libc::c_int as libc::c_long
                                        {
                                            current_block = 5402624238003992709;
                                        } else if IGS.miter < 0 as libc::c_int as libc::c_long
                                            || IGS.miter > 3 as libc::c_int as libc::c_long
                                        {
                                            current_block = 5402624238003992709;
                                        } else {
                                            if IGS.miter == 0 as libc::c_int as libc::c_long
                                                || IGS.miter == 3 as libc::c_int as libc::c_long
                                            {
                                                IGS.moss = 0 as libc::c_int as libc::c_long;
                                            }
                                            if *iopt == 1 as libc::c_int as libc::c_long {
                                                IGS.maxord = *iwork.offset(5 as libc::c_int as isize);
                                                if IGS.maxord < 0 as libc::c_int as libc::c_long {
                                                    printf(
                                                        b"lsodes: maxord = %ld is lower than 0.\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        IGS.maxord,
                                                    );
                                                    current_block = 2196110567356697731;
                                                } else {
                                                    if IGS.maxord == 0 as libc::c_int as libc::c_long {
                                                        IGS.maxord = 100 as libc::c_int as libc::c_long;
                                                    }
                                                    IGS
                                                        .maxord = if IGS.maxord
                                                        <= mord[(IGS.meth - 1 as libc::c_int as libc::c_long)
                                                            as usize]
                                                    {
                                                        IGS.maxord
                                                    } else {
                                                        mord[(IGS.meth - 1 as libc::c_int as libc::c_long) as usize]
                                                    };
                                                    IGS.mxstep = *iwork.offset(6 as libc::c_int as isize);
                                                    if IGS.mxstep < 0 as libc::c_int as libc::c_long {
                                                        printf(
                                                            b"lsodes: mxstep = %ld is lower than 0.\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            IGS.mxstep,
                                                        );
                                                        current_block = 2196110567356697731;
                                                    } else {
                                                        if IGS.mxstep == 0 as libc::c_int as libc::c_long {
                                                            IGS.mxstep = mxstp0;
                                                        }
                                                        IGS.mxhnil = *iwork.offset(7 as libc::c_int as isize);
                                                        if IGS.mxhnil < 0 as libc::c_int as libc::c_long {
                                                            printf(
                                                                b"lsodes: mxhnil = %ld is lower than 0.\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                                IGS.mxhnil,
                                                            );
                                                            current_block = 2196110567356697731;
                                                        } else {
                                                            if IGS.mxhnil == 0 as libc::c_int as libc::c_long {
                                                                IGS.mxhnil = mxhnl0;
                                                            }
                                                            if *istate != 1 as libc::c_int as libc::c_long {
                                                                current_block = 18302919043162154060;
                                                            } else {
                                                                h0 = *rwork.offset(5 as libc::c_int as isize);
                                                                if (*tout - *t) * h0 < 0.0f64 {
                                                                    printf(
                                                                        b"lsodes: tout = %21.13f is behind t = %21.13f\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        *tout,
                                                                        *t,
                                                                    );
                                                                    printf(
                                                                        b"        while integration direction is given by h0 = %21.13f.\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        h0,
                                                                    );
                                                                    current_block = 2196110567356697731;
                                                                } else {
                                                                    current_block = 18302919043162154060;
                                                                }
                                                            }
                                                            match current_block {
                                                                2196110567356697731 => {}
                                                                _ => {
                                                                    hmax = *rwork.offset(6 as libc::c_int as isize);
                                                                    if hmax < 0.0f64 {
                                                                        printf(
                                                                            b"lsodes: hmax = %21.13f, lower than 0.\n\0" as *const u8
                                                                                as *const libc::c_char,
                                                                            hmax,
                                                                        );
                                                                        current_block = 2196110567356697731;
                                                                    } else {
                                                                        IGS.hmxi = 0.0f64;
                                                                        if hmax > 0.0f64 {
                                                                            IGS.hmxi = 1.0f64 / hmax;
                                                                        }
                                                                        IGS.hmin = *rwork.offset(7 as libc::c_int as isize);
                                                                        if IGS.hmin < 0.0f64 {
                                                                            printf(
                                                                                b"lsodes: hmin = %21.13f, lower than 0.\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                                IGS.hmin,
                                                                            );
                                                                            current_block = 2196110567356697731;
                                                                        } else {
                                                                            IGS.seth = *rwork.offset(8 as libc::c_int as isize);
                                                                            if IGS.seth < 0.0f64 {
                                                                                printf(
                                                                                    b"lsodes: seth = %21.13f is lower than 0.\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    IGS.seth,
                                                                                );
                                                                                current_block = 2196110567356697731;
                                                                            } else {
                                                                                current_block = 6275242790444837149;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                IGS
                                                    .maxord = mord[(IGS.meth - 1 as libc::c_int as libc::c_long)
                                                    as usize];
                                                IGS.mxstep = mxstp0;
                                                IGS.mxhnil = mxhnl0;
                                                if *istate == 1 as libc::c_int as libc::c_long {
                                                    h0 = 0.0f64;
                                                }
                                                IGS.hmxi = 0.0f64;
                                                IGS.hmin = 0.0f64;
                                                IGS.seth = 0.0f64;
                                                current_block = 6275242790444837149;
                                            }
                                            match current_block {
                                                2196110567356697731 => {}
                                                _ => {
                                                    rtoli = *rtol.offset(1 as libc::c_int as isize);
                                                    atoli = *atol.offset(1 as libc::c_int as isize);
                                                    i__1 = IGS.n;
                                                    i = 1 as libc::c_int as libc::c_long;
                                                    loop {
                                                        if !(i <= i__1) {
                                                            current_block = 7419121793134201633;
                                                            break;
                                                        }
                                                        if *itol >= 3 as libc::c_int as libc::c_long {
                                                            rtoli = *rtol.offset(i as isize);
                                                        }
                                                        if *itol == 2 as libc::c_int as libc::c_long
                                                            || *itol == 4 as libc::c_int as libc::c_long
                                                        {
                                                            atoli = *atol.offset(i as isize);
                                                        }
                                                        if rtoli < 0.0f64 {
                                                            current_block = 182237522261645391;
                                                            break;
                                                        }
                                                        if atoli < 0.0f64 {
                                                            current_block = 5621912852155580917;
                                                            break;
                                                        }
                                                        i += 1;
                                                        i;
                                                    }
                                                    match current_block {
                                                        5621912852155580917 => {
                                                            printf(
                                                                b"lsodes: atol[%ld] = %21.13f, lower than 0.\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                i,
                                                                atoli,
                                                            );
                                                            current_block = 2196110567356697731;
                                                        }
                                                        182237522261645391 => {
                                                            printf(
                                                                b"lsodes: rtol[%ld] = %21.13f, lower than 0.\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                i,
                                                                rtoli,
                                                            );
                                                            current_block = 2196110567356697731;
                                                        }
                                                        _ => {
                                                            IGS.lrat = lenrat;
                                                            if *istate == 1 as libc::c_int as libc::c_long {
                                                                IGS.nyh = IGS.n;
                                                            }
                                                            IGS.lwmin = 0 as libc::c_int as libc::c_long;
                                                            if IGS.miter == 1 as libc::c_int as libc::c_long {
                                                                IGS
                                                                    .lwmin = ((IGS.n << 2 as libc::c_int) as libc::c_double
                                                                    + (IGS.n * 10 as libc::c_int as libc::c_long)
                                                                        as libc::c_double / IGS.lrat as libc::c_double)
                                                                    as libc::c_long;
                                                            }
                                                            if IGS.miter == 2 as libc::c_int as libc::c_long {
                                                                IGS
                                                                    .lwmin = ((IGS.n << 2 as libc::c_int) as libc::c_double
                                                                    + (IGS.n * 11 as libc::c_int as libc::c_long)
                                                                        as libc::c_double / IGS.lrat as libc::c_double)
                                                                    as libc::c_long;
                                                            }
                                                            if IGS.miter == 3 as libc::c_int as libc::c_long {
                                                                IGS.lwmin = IGS.n + 2 as libc::c_int as libc::c_long;
                                                            }
                                                            IGS
                                                                .lenyh = (IGS.maxord + 1 as libc::c_int as libc::c_long)
                                                                * IGS.nyh;
                                                            IGS
                                                                .lrest = IGS.lenyh
                                                                + IGS.n * 3 as libc::c_int as libc::c_long;
                                                            lenrw = IGS.lwmin + 20 as libc::c_int as libc::c_long
                                                                + IGS.lrest;
                                                            *iwork.offset(17 as libc::c_int as isize) = lenrw;
                                                            leniw = 30 as libc::c_int as libc::c_long;
                                                            if IGS.moss == 0 as libc::c_int as libc::c_long
                                                                && IGS.miter != 0 as libc::c_int as libc::c_long
                                                                && IGS.miter != 3 as libc::c_int as libc::c_long
                                                            {
                                                                leniw = leniw + IGS.n + 1 as libc::c_int as libc::c_long;
                                                            }
                                                            *iwork.offset(18 as libc::c_int as isize) = leniw;
                                                            if lenrw > *lrw {
                                                                current_block = 17976554964722057093;
                                                            } else {
                                                                if leniw > *liw {
                                                                    current_block = 7085547678065013391;
                                                                } else {
                                                                    lia = 31 as libc::c_int as libc::c_long;
                                                                    if IGS.moss == 0 as libc::c_int as libc::c_long
                                                                        && IGS.miter != 0 as libc::c_int as libc::c_long
                                                                        && IGS.miter != 3 as libc::c_int as libc::c_long
                                                                    {
                                                                        leniw = leniw + *iwork.offset((lia + IGS.n) as isize)
                                                                            - 1 as libc::c_int as libc::c_long;
                                                                    }
                                                                    *iwork.offset(18 as libc::c_int as isize) = leniw;
                                                                    if leniw > *liw {
                                                                        current_block = 7085547678065013391;
                                                                    } else {
                                                                        lja = lia + IGS.n + 1 as libc::c_int as libc::c_long;
                                                                        lia = if lia <= *liw { lia } else { *liw };
                                                                        lja = if lja <= *liw { lja } else { *liw };
                                                                        IGS.lwm = 21 as libc::c_int as libc::c_long;
                                                                        if *istate == 1 as libc::c_int as libc::c_long {
                                                                            IGS.nq = 1 as libc::c_int as libc::c_long;
                                                                        }
                                                                        i__1 = IGS.nq + 1 as libc::c_int as libc::c_long;
                                                                        i__2 = IGS.maxord + 2 as libc::c_int as libc::c_long;
                                                                        ncolm = if i__1 <= i__2 { i__1 } else { i__2 };
                                                                        IGS.lenyhm = ncolm * IGS.nyh;
                                                                        lenyht = IGS.lenyh;
                                                                        if IGS.miter == 1 as libc::c_int as libc::c_long
                                                                            || IGS.miter == 2 as libc::c_int as libc::c_long
                                                                        {
                                                                            lenyht = IGS.lenyhm;
                                                                        }
                                                                        imul = 2 as libc::c_int as libc::c_long;
                                                                        if *istate == 3 as libc::c_int as libc::c_long {
                                                                            imul = IGS.moss;
                                                                        }
                                                                        if IGS.moss == 2 as libc::c_int as libc::c_long {
                                                                            imul = 3 as libc::c_int as libc::c_long;
                                                                        }
                                                                        lrtem = lenyht + imul * IGS.n;
                                                                        lwtem = IGS.lwmin;
                                                                        if IGS.miter == 1 as libc::c_int as libc::c_long
                                                                            || IGS.miter == 2 as libc::c_int as libc::c_long
                                                                        {
                                                                            lwtem = *lrw - 20 as libc::c_int as libc::c_long - lrtem;
                                                                        }
                                                                        IGS.lenwk = lwtem;
                                                                        lyhn = IGS.lwm + lwtem;
                                                                        IGS.lsavf = lyhn + lenyht;
                                                                        IGS.lewt = IGS.lsavf + IGS.n;
                                                                        IGS.lacor = IGS.lewt + IGS.n;
                                                                        IGS.istatc = *istate;
                                                                        if *istate == 1 as libc::c_int as libc::c_long {
                                                                            IGS.lyh = lyhn;
                                                                            *iwork.offset(22 as libc::c_int as isize) = IGS.lyh;
                                                                            IGS.tn = *t;
                                                                            IGS.nst = 0 as libc::c_int as libc::c_long;
                                                                            IGS.h = 1.0f64;
                                                                            IGS.nnz = 0 as libc::c_int as libc::c_long;
                                                                            IGS.ngp = 0 as libc::c_int as libc::c_long;
                                                                            IGS.nzl = 0 as libc::c_int as libc::c_long;
                                                                            IGS.nzu = 0 as libc::c_int as libc::c_long;
                                                                            i__1 = IGS.n;
                                                                            i = 1 as libc::c_int as libc::c_long;
                                                                            while i <= i__1 {
                                                                                *rwork
                                                                                    .offset(
                                                                                        (i + IGS.lyh - 1 as libc::c_int as libc::c_long) as isize,
                                                                                    ) = *y.offset(i as isize);
                                                                                i += 1;
                                                                                i;
                                                                            }
                                                                            lf0 = IGS.lyh + IGS.nyh;
                                                                            CalcDeriv(
                                                                                &mut *y.offset(1 as libc::c_int as isize),
                                                                                &mut *rwork.offset(lf0 as isize),
                                                                                t,
                                                                            );
                                                                            IGS.nfe = 1 as libc::c_int as libc::c_long;
                                                                            ewset_(
                                                                                &mut IGS.n,
                                                                                itol,
                                                                                &mut *rtol.offset(1 as libc::c_int as isize),
                                                                                &mut *atol.offset(1 as libc::c_int as isize),
                                                                                &mut *rwork.offset(IGS.lyh as isize),
                                                                                &mut *rwork.offset(IGS.lewt as isize),
                                                                            );
                                                                            i__1 = IGS.n;
                                                                            i = 1 as libc::c_int as libc::c_long;
                                                                            loop {
                                                                                if !(i <= i__1) {
                                                                                    current_block = 18416742981416489973;
                                                                                    break;
                                                                                }
                                                                                if *rwork
                                                                                    .offset(
                                                                                        (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                                    ) <= 0.0f64
                                                                                {
                                                                                    current_block = 948988363597225076;
                                                                                    break;
                                                                                }
                                                                                *rwork
                                                                                    .offset(
                                                                                        (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                                    ) = 1.0f64
                                                                                    / *rwork
                                                                                        .offset(
                                                                                            (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                                        );
                                                                                i += 1;
                                                                                i;
                                                                            }
                                                                            match current_block {
                                                                                948988363597225076 => {}
                                                                                _ => {
                                                                                    if IGS.miter == 0 as libc::c_int as libc::c_long
                                                                                        || IGS.miter == 3 as libc::c_int as libc::c_long
                                                                                    {
                                                                                        current_block = 6992595839166662032;
                                                                                    } else {
                                                                                        IGS
                                                                                            .lacor = if IGS.lacor <= *lrw { IGS.lacor } else { *lrw };
                                                                                        iprep_(
                                                                                            &mut *neq.offset(1 as libc::c_int as isize),
                                                                                            &mut *y.offset(1 as libc::c_int as isize),
                                                                                            &mut *rwork.offset(1 as libc::c_int as isize),
                                                                                            &mut *iwork.offset(lia as isize),
                                                                                            &mut *iwork.offset(lja as isize),
                                                                                            &mut ipflag,
                                                                                        );
                                                                                        lenrw = IGS.lwm - 1 as libc::c_int as libc::c_long
                                                                                            + IGS.lenwk + IGS.lrest;
                                                                                        *iwork.offset(17 as libc::c_int as isize) = lenrw;
                                                                                        if ipflag != -(1 as libc::c_int) as libc::c_long {
                                                                                            *iwork.offset(23 as libc::c_int as isize) = IGS.ipian;
                                                                                        }
                                                                                        if ipflag != -(1 as libc::c_int) as libc::c_long {
                                                                                            *iwork.offset(24 as libc::c_int as isize) = IGS.ipjan;
                                                                                        }
                                                                                        ipgo = -ipflag + 1 as libc::c_int as libc::c_long;
                                                                                        match ipgo {
                                                                                            2 => {
                                                                                                current_block = 16556930902621764451;
                                                                                            }
                                                                                            3 => {
                                                                                                current_block = 9970882338518894152;
                                                                                            }
                                                                                            4 => {
                                                                                                current_block = 11025910637373094823;
                                                                                            }
                                                                                            5 => {
                                                                                                current_block = 12178837017827381628;
                                                                                            }
                                                                                            6 => {
                                                                                                current_block = 10972642675681460258;
                                                                                            }
                                                                                            7 => {
                                                                                                current_block = 10882754252140161242;
                                                                                            }
                                                                                            1 | _ => {
                                                                                                *iwork.offset(22 as libc::c_int as isize) = IGS.lyh;
                                                                                                if lenrw > *lrw {
                                                                                                    current_block = 17976554964722057093;
                                                                                                } else {
                                                                                                    current_block = 6992595839166662032;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    match current_block {
                                                                                        17976554964722057093 => {}
                                                                                        16556930902621764451 => {}
                                                                                        9970882338518894152 => {}
                                                                                        11025910637373094823 => {}
                                                                                        12178837017827381628 => {}
                                                                                        10972642675681460258 => {}
                                                                                        10882754252140161242 => {}
                                                                                        _ => {
                                                                                            if *itask != 4 as libc::c_int as libc::c_long
                                                                                                && *itask != 5 as libc::c_int as libc::c_long
                                                                                            {
                                                                                                current_block = 16520105677914726267;
                                                                                            } else {
                                                                                                tcrit = *rwork.offset(1 as libc::c_int as isize);
                                                                                                if (tcrit - *tout) * (*tout - *t) < 0.0f64 {
                                                                                                    current_block = 15113272348145639663;
                                                                                                } else {
                                                                                                    if h0 != 0.0f64 && (*t + h0 - tcrit) * h0 > 0.0f64 {
                                                                                                        h0 = tcrit - *t;
                                                                                                    }
                                                                                                    current_block = 16520105677914726267;
                                                                                                }
                                                                                            }
                                                                                            match current_block {
                                                                                                15113272348145639663 => {}
                                                                                                _ => {
                                                                                                    IGS.uround = 2.2204460492503131e-16f64;
                                                                                                    IGS.jstart = 0 as libc::c_int as libc::c_long;
                                                                                                    if IGS.miter != 0 as libc::c_int as libc::c_long {
                                                                                                        *rwork.offset(IGS.lwm as isize) = sqrt(IGS.uround);
                                                                                                    }
                                                                                                    IGS.msbj = 50 as libc::c_int as libc::c_long;
                                                                                                    IGS.nslj = 0 as libc::c_int as libc::c_long;
                                                                                                    IGS.ccmxj = 0.2f64;
                                                                                                    IGS.psmall = IGS.uround * 1e3f64;
                                                                                                    IGS.rbig = 0.01f64 / IGS.psmall;
                                                                                                    IGS.nhnil = 0 as libc::c_int as libc::c_long;
                                                                                                    IGS.nje = 0 as libc::c_int as libc::c_long;
                                                                                                    IGS.nlu = 0 as libc::c_int as libc::c_long;
                                                                                                    IGS.nslast = 0 as libc::c_int as libc::c_long;
                                                                                                    IGS.hu = 0.0f64;
                                                                                                    IGS.nqu = 0 as libc::c_int as libc::c_long;
                                                                                                    IGS.ccmax = 0.3f64;
                                                                                                    IGS.maxcor = 3 as libc::c_int as libc::c_long;
                                                                                                    IGS.msbp = 20 as libc::c_int as libc::c_long;
                                                                                                    IGS.mxncf = 10 as libc::c_int as libc::c_long;
                                                                                                    lf0 = IGS.lyh + IGS.nyh;
                                                                                                    if h0 != 0.0f64 {
                                                                                                        current_block = 12720067915845509315;
                                                                                                    } else {
                                                                                                        tdist = fabs(*tout - *t);
                                                                                                        w0 = if fabs(*t) >= fabs(*tout) {
                                                                                                            fabs(*t)
                                                                                                        } else {
                                                                                                            fabs(*tout)
                                                                                                        };
                                                                                                        if tdist < IGS.uround * 2.0f64 * w0 {
                                                                                                            printf(
                                                                                                                b"lsodes: tout = %21.13f too close to t = %21.13f to start integration.\n\0"
                                                                                                                    as *const u8 as *const libc::c_char,
                                                                                                                *tout,
                                                                                                                *t,
                                                                                                            );
                                                                                                            current_block = 2196110567356697731;
                                                                                                        } else {
                                                                                                            tol = *rtol.offset(1 as libc::c_int as isize);
                                                                                                            if !(*itol <= 2 as libc::c_int as libc::c_long) {
                                                                                                                i__1 = IGS.n;
                                                                                                                i = 1 as libc::c_int as libc::c_long;
                                                                                                                while i <= i__1 {
                                                                                                                    d__1 = tol;
                                                                                                                    d__2 = *rtol.offset(i as isize);
                                                                                                                    tol = if d__1 >= d__2 { d__1 } else { d__2 };
                                                                                                                    i += 1;
                                                                                                                    i;
                                                                                                                }
                                                                                                            }
                                                                                                            if !(tol > 0.0f64) {
                                                                                                                atoli = *atol.offset(1 as libc::c_int as isize);
                                                                                                                i__1 = IGS.n;
                                                                                                                i = 1 as libc::c_int as libc::c_long;
                                                                                                                while i <= i__1 {
                                                                                                                    if *itol == 2 as libc::c_int as libc::c_long
                                                                                                                        || *itol == 4 as libc::c_int as libc::c_long
                                                                                                                    {
                                                                                                                        atoli = *atol.offset(i as isize);
                                                                                                                    }
                                                                                                                    ayi = fabs(*y.offset(i as isize));
                                                                                                                    if ayi != 0.0f64 {
                                                                                                                        d__1 = tol;
                                                                                                                        d__2 = atoli / ayi;
                                                                                                                        tol = if d__1 >= d__2 { d__1 } else { d__2 };
                                                                                                                    }
                                                                                                                    i += 1;
                                                                                                                    i;
                                                                                                                }
                                                                                                            }
                                                                                                            d__1 = tol;
                                                                                                            d__2 = IGS.uround * 100.0f64;
                                                                                                            tol = if d__1 >= d__2 { d__1 } else { d__2 };
                                                                                                            tol = if tol <= 0.001f64 { tol } else { 0.001f64 };
                                                                                                            sum = vnorm_(
                                                                                                                &mut IGS.n,
                                                                                                                &mut *rwork.offset(lf0 as isize),
                                                                                                                &mut *rwork.offset(IGS.lewt as isize),
                                                                                                            );
                                                                                                            d__1 = sum;
                                                                                                            sum = 1.0f64 / (tol * w0 * w0) + tol * (d__1 * d__1);
                                                                                                            h0 = 1.0f64 / sqrt(sum);
                                                                                                            h0 = if h0 <= tdist { h0 } else { tdist };
                                                                                                            d__1 = *tout - *t;
                                                                                                            h0 = d_sign(&mut h0, &mut d__1);
                                                                                                            current_block = 12720067915845509315;
                                                                                                        }
                                                                                                    }
                                                                                                    match current_block {
                                                                                                        2196110567356697731 => {}
                                                                                                        _ => {
                                                                                                            rh = fabs(h0) * IGS.hmxi;
                                                                                                            if rh > 1.0f64 {
                                                                                                                h0 /= rh;
                                                                                                            }
                                                                                                            IGS.h = h0;
                                                                                                            i__1 = IGS.n;
                                                                                                            i = 1 as libc::c_int as libc::c_long;
                                                                                                            while i <= i__1 {
                                                                                                                *rwork
                                                                                                                    .offset(
                                                                                                                        (i + lf0 - 1 as libc::c_int as libc::c_long) as isize,
                                                                                                                    ) = h0
                                                                                                                    * *rwork
                                                                                                                        .offset(
                                                                                                                            (i + lf0 - 1 as libc::c_int as libc::c_long) as isize,
                                                                                                                        );
                                                                                                                i += 1;
                                                                                                                i;
                                                                                                            }
                                                                                                            current_block = 10812852962309229047;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            lyhd = IGS.lyh - lyhn;
                                                                            imax = lyhn - 1 as libc::c_int as libc::c_long + IGS.lenyhm;
                                                                            if lyhd < 0 as libc::c_int as libc::c_long {
                                                                                i__1 = imax;
                                                                                i = lyhn;
                                                                                while i <= i__1 {
                                                                                    j = imax + lyhn - i;
                                                                                    *rwork
                                                                                        .offset(j as isize) = *rwork.offset((j + lyhd) as isize);
                                                                                    i += 1;
                                                                                    i;
                                                                                }
                                                                            } else if !(lyhd == 0 as libc::c_int as libc::c_long) {
                                                                                i__1 = imax;
                                                                                i = lyhn;
                                                                                while i <= i__1 {
                                                                                    *rwork
                                                                                        .offset(i as isize) = *rwork.offset((i + lyhd) as isize);
                                                                                    i += 1;
                                                                                    i;
                                                                                }
                                                                            }
                                                                            IGS.lyh = lyhn;
                                                                            *iwork.offset(22 as libc::c_int as isize) = IGS.lyh;
                                                                            if IGS.miter == 0 as libc::c_int as libc::c_long
                                                                                || IGS.miter == 3 as libc::c_int as libc::c_long
                                                                            {
                                                                                current_block = 8583961259489240085;
                                                                            } else {
                                                                                if IGS.moss != 2 as libc::c_int as libc::c_long {
                                                                                    current_block = 4522553794717406153;
                                                                                } else {
                                                                                    ewset_(
                                                                                        &mut IGS.n,
                                                                                        itol,
                                                                                        &mut *rtol.offset(1 as libc::c_int as isize),
                                                                                        &mut *atol.offset(1 as libc::c_int as isize),
                                                                                        &mut *rwork.offset(IGS.lyh as isize),
                                                                                        &mut *rwork.offset(IGS.lewt as isize),
                                                                                    );
                                                                                    i__1 = IGS.n;
                                                                                    i = 1 as libc::c_int as libc::c_long;
                                                                                    loop {
                                                                                        if !(i <= i__1) {
                                                                                            current_block = 4522553794717406153;
                                                                                            break;
                                                                                        }
                                                                                        if *rwork
                                                                                            .offset(
                                                                                                (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                                            ) <= 0.0f64
                                                                                        {
                                                                                            current_block = 948988363597225076;
                                                                                            break;
                                                                                        }
                                                                                        *rwork
                                                                                            .offset(
                                                                                                (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                                            ) = 1.0f64
                                                                                            / *rwork
                                                                                                .offset(
                                                                                                    (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                                                );
                                                                                        i += 1;
                                                                                        i;
                                                                                    }
                                                                                }
                                                                                match current_block {
                                                                                    948988363597225076 => {}
                                                                                    _ => {
                                                                                        IGS
                                                                                            .lsavf = if IGS.lsavf <= *lrw { IGS.lsavf } else { *lrw };
                                                                                        IGS.lewt = if IGS.lewt <= *lrw { IGS.lewt } else { *lrw };
                                                                                        IGS
                                                                                            .lacor = if IGS.lacor <= *lrw { IGS.lacor } else { *lrw };
                                                                                        iprep_(
                                                                                            &mut *neq.offset(1 as libc::c_int as isize),
                                                                                            &mut *y.offset(1 as libc::c_int as isize),
                                                                                            &mut *rwork.offset(1 as libc::c_int as isize),
                                                                                            &mut *iwork.offset(lia as isize),
                                                                                            &mut *iwork.offset(lja as isize),
                                                                                            &mut ipflag,
                                                                                        );
                                                                                        lenrw = IGS.lwm - 1 as libc::c_int as libc::c_long
                                                                                            + IGS.lenwk + IGS.lrest;
                                                                                        *iwork.offset(17 as libc::c_int as isize) = lenrw;
                                                                                        if ipflag != -(1 as libc::c_int) as libc::c_long {
                                                                                            *iwork.offset(23 as libc::c_int as isize) = IGS.ipian;
                                                                                        }
                                                                                        if ipflag != -(1 as libc::c_int) as libc::c_long {
                                                                                            *iwork.offset(24 as libc::c_int as isize) = IGS.ipjan;
                                                                                        }
                                                                                        ipgo = -ipflag + 1 as libc::c_int as libc::c_long;
                                                                                        match ipgo {
                                                                                            2 => {
                                                                                                current_block = 16556930902621764451;
                                                                                            }
                                                                                            3 => {
                                                                                                current_block = 9970882338518894152;
                                                                                            }
                                                                                            4 => {
                                                                                                current_block = 11025910637373094823;
                                                                                            }
                                                                                            5 => {
                                                                                                current_block = 12178837017827381628;
                                                                                            }
                                                                                            6 => {
                                                                                                current_block = 10972642675681460258;
                                                                                            }
                                                                                            7 => {
                                                                                                current_block = 10882754252140161242;
                                                                                            }
                                                                                            1 | _ => {
                                                                                                *iwork.offset(22 as libc::c_int as isize) = IGS.lyh;
                                                                                                if lenrw > *lrw {
                                                                                                    current_block = 17976554964722057093;
                                                                                                } else {
                                                                                                    current_block = 8583961259489240085;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            match current_block {
                                                                                948988363597225076 => {}
                                                                                16556930902621764451 => {}
                                                                                9970882338518894152 => {}
                                                                                11025910637373094823 => {}
                                                                                12178837017827381628 => {}
                                                                                10972642675681460258 => {}
                                                                                10882754252140161242 => {}
                                                                                17976554964722057093 => {}
                                                                                _ => {
                                                                                    IGS.jstart = -(1 as libc::c_int) as libc::c_long;
                                                                                    if IGS.n == IGS.nyh {
                                                                                        current_block = 16977092573463915408;
                                                                                    } else {
                                                                                        i1 = IGS.lyh + IGS.l * IGS.nyh;
                                                                                        i2 = IGS.lyh
                                                                                            + (IGS.maxord + 1 as libc::c_int as libc::c_long) * IGS.nyh
                                                                                            - 1 as libc::c_int as libc::c_long;
                                                                                        if i1 > i2 {
                                                                                            current_block = 16977092573463915408;
                                                                                        } else {
                                                                                            i__1 = i2;
                                                                                            i = i1;
                                                                                            while i <= i__1 {
                                                                                                *rwork.offset(i as isize) = 0.0f64;
                                                                                                i += 1;
                                                                                                i;
                                                                                            }
                                                                                            current_block = 16977092573463915408;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block {
                                                                            2196110567356697731 => {}
                                                                            10812852962309229047 => {}
                                                                            15113272348145639663 => {}
                                                                            17976554964722057093 => {}
                                                                            16977092573463915408 => {}
                                                                            _ => {
                                                                                match current_block {
                                                                                    10972642675681460258 => {
                                                                                        printf(
                                                                                            b"lsodes: rwork length insufficient (for subroutine cdrv),\n        length needed is at least %ld, exceeds lrw (= %ld).\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                            lenrw,
                                                                                            *lrw,
                                                                                        );
                                                                                    }
                                                                                    12178837017827381628 => {
                                                                                        imul = (IGS.iys - 1 as libc::c_int as libc::c_long) / IGS.n;
                                                                                        irem = IGS.iys - imul * IGS.n;
                                                                                        printf(
                                                                                            b"lsodes: error from odrv in yale sparse matrix package,\n        at t = %21.13f, odrv returned error flag = %ld*neq + %ld:\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                            IGS.tn,
                                                                                            imul,
                                                                                            irem,
                                                                                        );
                                                                                    }
                                                                                    10882754252140161242 => {
                                                                                        printf(
                                                                                            b"lsodes: error from cdrv in yale sparse matrix package,\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                        );
                                                                                        imul = (IGS.iys - 1 as libc::c_int as libc::c_long) / IGS.n;
                                                                                        irem = IGS.iys - imul * IGS.n;
                                                                                        printf(
                                                                                            b"        at t =%21.13f, cdrv returned error flag = %ld*neq + %ld:\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                            IGS.tn,
                                                                                            imul,
                                                                                            irem,
                                                                                        );
                                                                                        if imul == 2 as libc::c_int as libc::c_long {
                                                                                            printf(
                                                                                                b"        duplicate entry in sparsity structure descriptors.\n\0"
                                                                                                    as *const u8 as *const libc::c_char,
                                                                                            );
                                                                                        }
                                                                                        if imul == 3 as libc::c_int as libc::c_long
                                                                                            || imul == 6 as libc::c_int as libc::c_long
                                                                                        {
                                                                                            printf(
                                                                                                b"        insufficient storage for nsfc (called by cdrv).\n\0"
                                                                                                    as *const u8 as *const libc::c_char,
                                                                                            );
                                                                                        }
                                                                                    }
                                                                                    16556930902621764451 => {
                                                                                        printf(
                                                                                            b"lsodes: rwork length insufficient (for subroutine prep),\n        length needed is at least %ld, exceeds lrw (= %ld).\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                            lenrw,
                                                                                            *lrw,
                                                                                        );
                                                                                    }
                                                                                    9970882338518894152 => {
                                                                                        printf(
                                                                                            b"lsodes: rwork length insufficient (for subroutine jgroup),\n        length needed is at least %ld, exceeds lrw (= %ld).\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                            lenrw,
                                                                                            *lrw,
                                                                                        );
                                                                                    }
                                                                                    11025910637373094823 => {
                                                                                        printf(
                                                                                            b"lsodes: rwork length insufficient (for subroutine odrv),\n        length needed is at least %ld, exceeds lrw (= %ld).\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                            lenrw,
                                                                                            *lrw,
                                                                                        );
                                                                                    }
                                                                                    _ => {
                                                                                        ewti = *rwork
                                                                                            .offset(
                                                                                                (IGS.lewt + i - 1 as libc::c_int as libc::c_long) as isize,
                                                                                            );
                                                                                        printf(
                                                                                            b"lsodes: ewt[%ld] = %21.13f, lower or equal to 0.\n\0"
                                                                                                as *const u8 as *const libc::c_char,
                                                                                            i,
                                                                                            ewti,
                                                                                        );
                                                                                    }
                                                                                }
                                                                                current_block = 2196110567356697731;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                match current_block {
                                                                    2196110567356697731 => {}
                                                                    10812852962309229047 => {}
                                                                    17976554964722057093 => {}
                                                                    16977092573463915408 => {}
                                                                    15113272348145639663 => {}
                                                                    _ => {
                                                                        printf(
                                                                            b"lsodes: iwork length is insufficient to proceed,\n         length needed is at least %ld, exceeds liw (= %ld).\n\0"
                                                                                as *const u8 as *const libc::c_char,
                                                                            leniw,
                                                                            *liw,
                                                                        );
                                                                        current_block = 2196110567356697731;
                                                                    }
                                                                }
                                                            }
                                                            match current_block {
                                                                2196110567356697731 => {}
                                                                10812852962309229047 => {}
                                                                16977092573463915408 => {}
                                                                15113272348145639663 => {}
                                                                _ => {
                                                                    printf(
                                                                        b"lsodes: rwork length is insufficient to proceed,\n         length needed is at least %ld, exceeds liw (= %ld).\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        lenrw,
                                                                        *lrw,
                                                                    );
                                                                    current_block = 2196110567356697731;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        match current_block {
                                            2196110567356697731 => {}
                                            16977092573463915408 => {}
                                            15113272348145639663 => {}
                                            10812852962309229047 => {}
                                            _ => {
                                                printf(
                                                    b"lsodes: mf = %ld is illegal.\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    *mf,
                                                );
                                                current_block = 2196110567356697731;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    2196110567356697731 => {}
                    _ => {
                        match current_block {
                            16977092573463915408 => {
                                IGS.nslast = IGS.nst;
                                match *itask {
                                    2 => {
                                        current_block = 2953975522857967725;
                                    }
                                    3 => {
                                        current_block = 16472680688353791607;
                                        match current_block {
                                            17186749766321373789 => {
                                                if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 2953975522857967725;
                                                } else {
                                                    intdy_(
                                                        tout,
                                                        0 as libc::c_int as libc::c_long,
                                                        &mut *rwork.offset(IGS.lyh as isize),
                                                        &mut IGS.nyh,
                                                        &mut *y.offset(1 as libc::c_int as isize),
                                                        &mut iflag,
                                                    );
                                                    if iflag != 0 as libc::c_int as libc::c_long {
                                                        current_block = 12428118024966215402;
                                                    } else {
                                                        *t = *tout;
                                                        current_block = 4298192726394742527;
                                                    }
                                                }
                                            }
                                            7750364551164556572 => {
                                                tcrit = *rwork.offset(1 as libc::c_int as isize);
                                                if (IGS.tn - tcrit) * IGS.h > 0.0f64 {
                                                    current_block = 5487427803874674771;
                                                } else if (tcrit - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 15113272348145639663;
                                                } else if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 17660875221514469833;
                                                } else {
                                                    intdy_(
                                                        tout,
                                                        0 as libc::c_int as libc::c_long,
                                                        &mut *rwork.offset(IGS.lyh as isize),
                                                        &mut IGS.nyh,
                                                        &mut *y.offset(1 as libc::c_int as isize),
                                                        &mut iflag,
                                                    );
                                                    if iflag != 0 as libc::c_int as libc::c_long {
                                                        current_block = 12428118024966215402;
                                                    } else {
                                                        *t = *tout;
                                                        current_block = 4298192726394742527;
                                                    }
                                                }
                                            }
                                            16472680688353791607 => {
                                                tp = IGS.tn - IGS.hu * (IGS.uround * 100.0f64 + 1.0f64);
                                                if (tp - *tout) * IGS.h > 0.0f64 {
                                                    printf(
                                                        b"lsodes: itask = %ld and tout = %21.13f is behind tcur - hu (= %21.13f).\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        *itask,
                                                        *tout,
                                                        tp,
                                                    );
                                                    current_block = 2196110567356697731;
                                                } else if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 2953975522857967725;
                                                } else {
                                                    current_block = 14098009084916107386;
                                                }
                                            }
                                            _ => {
                                                tcrit = *rwork.offset(1 as libc::c_int as isize);
                                                if (IGS.tn - tcrit) * IGS.h > 0.0f64 {
                                                    current_block = 5487427803874674771;
                                                } else {
                                                    current_block = 17660875221514469833;
                                                }
                                            }
                                        }
                                        match current_block {
                                            15113272348145639663 => {}
                                            4298192726394742527 => {}
                                            2196110567356697731 => {}
                                            2953975522857967725 => {}
                                            14098009084916107386 => {}
                                            _ => {
                                                match current_block {
                                                    12428118024966215402 => {
                                                        printf(
                                                            b"lsodes: trouble from intdy. itask = %ld, tout = %21.13f.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            *itask,
                                                            *tout,
                                                        );
                                                        current_block = 2196110567356697731;
                                                    }
                                                    5487427803874674771 => {
                                                        printf(
                                                            b"lsodes: itask = 4 or 5 and tcrit = %21.13f, behind tcur (= %21.13f).\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            tcrit,
                                                            IGS.tn,
                                                        );
                                                        current_block = 2196110567356697731;
                                                    }
                                                    _ => {
                                                        hmx = fabs(IGS.tn) + fabs(IGS.h);
                                                        ihit = (fabs(IGS.tn - tcrit) <= IGS.uround * 100.0f64 * hmx)
                                                            as libc::c_int as libc::c_long;
                                                        if ihit != 0 {
                                                            current_block = 14098009084916107386;
                                                        } else {
                                                            tnext = IGS.tn + IGS.h * (IGS.uround * 4.0f64 + 1.0f64);
                                                            if (tnext - tcrit) * IGS.h <= 0.0f64 {
                                                                current_block = 2953975522857967725;
                                                            } else {
                                                                IGS.h = (tcrit - IGS.tn) * (1.0f64 - IGS.uround * 4.0f64);
                                                                if *istate == 2 as libc::c_int as libc::c_long {
                                                                    IGS.jstart = -(2 as libc::c_int) as libc::c_long;
                                                                }
                                                                current_block = 2953975522857967725;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    4 => {
                                        current_block = 7750364551164556572;
                                        match current_block {
                                            17186749766321373789 => {
                                                if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 2953975522857967725;
                                                } else {
                                                    intdy_(
                                                        tout,
                                                        0 as libc::c_int as libc::c_long,
                                                        &mut *rwork.offset(IGS.lyh as isize),
                                                        &mut IGS.nyh,
                                                        &mut *y.offset(1 as libc::c_int as isize),
                                                        &mut iflag,
                                                    );
                                                    if iflag != 0 as libc::c_int as libc::c_long {
                                                        current_block = 12428118024966215402;
                                                    } else {
                                                        *t = *tout;
                                                        current_block = 4298192726394742527;
                                                    }
                                                }
                                            }
                                            7750364551164556572 => {
                                                tcrit = *rwork.offset(1 as libc::c_int as isize);
                                                if (IGS.tn - tcrit) * IGS.h > 0.0f64 {
                                                    current_block = 5487427803874674771;
                                                } else if (tcrit - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 15113272348145639663;
                                                } else if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 17660875221514469833;
                                                } else {
                                                    intdy_(
                                                        tout,
                                                        0 as libc::c_int as libc::c_long,
                                                        &mut *rwork.offset(IGS.lyh as isize),
                                                        &mut IGS.nyh,
                                                        &mut *y.offset(1 as libc::c_int as isize),
                                                        &mut iflag,
                                                    );
                                                    if iflag != 0 as libc::c_int as libc::c_long {
                                                        current_block = 12428118024966215402;
                                                    } else {
                                                        *t = *tout;
                                                        current_block = 4298192726394742527;
                                                    }
                                                }
                                            }
                                            16472680688353791607 => {
                                                tp = IGS.tn - IGS.hu * (IGS.uround * 100.0f64 + 1.0f64);
                                                if (tp - *tout) * IGS.h > 0.0f64 {
                                                    printf(
                                                        b"lsodes: itask = %ld and tout = %21.13f is behind tcur - hu (= %21.13f).\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        *itask,
                                                        *tout,
                                                        tp,
                                                    );
                                                    current_block = 2196110567356697731;
                                                } else if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 2953975522857967725;
                                                } else {
                                                    current_block = 14098009084916107386;
                                                }
                                            }
                                            _ => {
                                                tcrit = *rwork.offset(1 as libc::c_int as isize);
                                                if (IGS.tn - tcrit) * IGS.h > 0.0f64 {
                                                    current_block = 5487427803874674771;
                                                } else {
                                                    current_block = 17660875221514469833;
                                                }
                                            }
                                        }
                                        match current_block {
                                            15113272348145639663 => {}
                                            4298192726394742527 => {}
                                            2196110567356697731 => {}
                                            2953975522857967725 => {}
                                            14098009084916107386 => {}
                                            _ => {
                                                match current_block {
                                                    12428118024966215402 => {
                                                        printf(
                                                            b"lsodes: trouble from intdy. itask = %ld, tout = %21.13f.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            *itask,
                                                            *tout,
                                                        );
                                                        current_block = 2196110567356697731;
                                                    }
                                                    5487427803874674771 => {
                                                        printf(
                                                            b"lsodes: itask = 4 or 5 and tcrit = %21.13f, behind tcur (= %21.13f).\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            tcrit,
                                                            IGS.tn,
                                                        );
                                                        current_block = 2196110567356697731;
                                                    }
                                                    _ => {
                                                        hmx = fabs(IGS.tn) + fabs(IGS.h);
                                                        ihit = (fabs(IGS.tn - tcrit) <= IGS.uround * 100.0f64 * hmx)
                                                            as libc::c_int as libc::c_long;
                                                        if ihit != 0 {
                                                            current_block = 14098009084916107386;
                                                        } else {
                                                            tnext = IGS.tn + IGS.h * (IGS.uround * 4.0f64 + 1.0f64);
                                                            if (tnext - tcrit) * IGS.h <= 0.0f64 {
                                                                current_block = 2953975522857967725;
                                                            } else {
                                                                IGS.h = (tcrit - IGS.tn) * (1.0f64 - IGS.uround * 4.0f64);
                                                                if *istate == 2 as libc::c_int as libc::c_long {
                                                                    IGS.jstart = -(2 as libc::c_int) as libc::c_long;
                                                                }
                                                                current_block = 2953975522857967725;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    5 => {
                                        current_block = 2560043871774763239;
                                        match current_block {
                                            17186749766321373789 => {
                                                if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 2953975522857967725;
                                                } else {
                                                    intdy_(
                                                        tout,
                                                        0 as libc::c_int as libc::c_long,
                                                        &mut *rwork.offset(IGS.lyh as isize),
                                                        &mut IGS.nyh,
                                                        &mut *y.offset(1 as libc::c_int as isize),
                                                        &mut iflag,
                                                    );
                                                    if iflag != 0 as libc::c_int as libc::c_long {
                                                        current_block = 12428118024966215402;
                                                    } else {
                                                        *t = *tout;
                                                        current_block = 4298192726394742527;
                                                    }
                                                }
                                            }
                                            7750364551164556572 => {
                                                tcrit = *rwork.offset(1 as libc::c_int as isize);
                                                if (IGS.tn - tcrit) * IGS.h > 0.0f64 {
                                                    current_block = 5487427803874674771;
                                                } else if (tcrit - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 15113272348145639663;
                                                } else if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 17660875221514469833;
                                                } else {
                                                    intdy_(
                                                        tout,
                                                        0 as libc::c_int as libc::c_long,
                                                        &mut *rwork.offset(IGS.lyh as isize),
                                                        &mut IGS.nyh,
                                                        &mut *y.offset(1 as libc::c_int as isize),
                                                        &mut iflag,
                                                    );
                                                    if iflag != 0 as libc::c_int as libc::c_long {
                                                        current_block = 12428118024966215402;
                                                    } else {
                                                        *t = *tout;
                                                        current_block = 4298192726394742527;
                                                    }
                                                }
                                            }
                                            16472680688353791607 => {
                                                tp = IGS.tn - IGS.hu * (IGS.uround * 100.0f64 + 1.0f64);
                                                if (tp - *tout) * IGS.h > 0.0f64 {
                                                    printf(
                                                        b"lsodes: itask = %ld and tout = %21.13f is behind tcur - hu (= %21.13f).\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        *itask,
                                                        *tout,
                                                        tp,
                                                    );
                                                    current_block = 2196110567356697731;
                                                } else if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 2953975522857967725;
                                                } else {
                                                    current_block = 14098009084916107386;
                                                }
                                            }
                                            _ => {
                                                tcrit = *rwork.offset(1 as libc::c_int as isize);
                                                if (IGS.tn - tcrit) * IGS.h > 0.0f64 {
                                                    current_block = 5487427803874674771;
                                                } else {
                                                    current_block = 17660875221514469833;
                                                }
                                            }
                                        }
                                        match current_block {
                                            15113272348145639663 => {}
                                            4298192726394742527 => {}
                                            2196110567356697731 => {}
                                            2953975522857967725 => {}
                                            14098009084916107386 => {}
                                            _ => {
                                                match current_block {
                                                    12428118024966215402 => {
                                                        printf(
                                                            b"lsodes: trouble from intdy. itask = %ld, tout = %21.13f.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            *itask,
                                                            *tout,
                                                        );
                                                        current_block = 2196110567356697731;
                                                    }
                                                    5487427803874674771 => {
                                                        printf(
                                                            b"lsodes: itask = 4 or 5 and tcrit = %21.13f, behind tcur (= %21.13f).\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            tcrit,
                                                            IGS.tn,
                                                        );
                                                        current_block = 2196110567356697731;
                                                    }
                                                    _ => {
                                                        hmx = fabs(IGS.tn) + fabs(IGS.h);
                                                        ihit = (fabs(IGS.tn - tcrit) <= IGS.uround * 100.0f64 * hmx)
                                                            as libc::c_int as libc::c_long;
                                                        if ihit != 0 {
                                                            current_block = 14098009084916107386;
                                                        } else {
                                                            tnext = IGS.tn + IGS.h * (IGS.uround * 4.0f64 + 1.0f64);
                                                            if (tnext - tcrit) * IGS.h <= 0.0f64 {
                                                                current_block = 2953975522857967725;
                                                            } else {
                                                                IGS.h = (tcrit - IGS.tn) * (1.0f64 - IGS.uround * 4.0f64);
                                                                if *istate == 2 as libc::c_int as libc::c_long {
                                                                    IGS.jstart = -(2 as libc::c_int) as libc::c_long;
                                                                }
                                                                current_block = 2953975522857967725;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    1 | _ => {
                                        current_block = 17186749766321373789;
                                        match current_block {
                                            17186749766321373789 => {
                                                if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 2953975522857967725;
                                                } else {
                                                    intdy_(
                                                        tout,
                                                        0 as libc::c_int as libc::c_long,
                                                        &mut *rwork.offset(IGS.lyh as isize),
                                                        &mut IGS.nyh,
                                                        &mut *y.offset(1 as libc::c_int as isize),
                                                        &mut iflag,
                                                    );
                                                    if iflag != 0 as libc::c_int as libc::c_long {
                                                        current_block = 12428118024966215402;
                                                    } else {
                                                        *t = *tout;
                                                        current_block = 4298192726394742527;
                                                    }
                                                }
                                            }
                                            7750364551164556572 => {
                                                tcrit = *rwork.offset(1 as libc::c_int as isize);
                                                if (IGS.tn - tcrit) * IGS.h > 0.0f64 {
                                                    current_block = 5487427803874674771;
                                                } else if (tcrit - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 15113272348145639663;
                                                } else if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 17660875221514469833;
                                                } else {
                                                    intdy_(
                                                        tout,
                                                        0 as libc::c_int as libc::c_long,
                                                        &mut *rwork.offset(IGS.lyh as isize),
                                                        &mut IGS.nyh,
                                                        &mut *y.offset(1 as libc::c_int as isize),
                                                        &mut iflag,
                                                    );
                                                    if iflag != 0 as libc::c_int as libc::c_long {
                                                        current_block = 12428118024966215402;
                                                    } else {
                                                        *t = *tout;
                                                        current_block = 4298192726394742527;
                                                    }
                                                }
                                            }
                                            16472680688353791607 => {
                                                tp = IGS.tn - IGS.hu * (IGS.uround * 100.0f64 + 1.0f64);
                                                if (tp - *tout) * IGS.h > 0.0f64 {
                                                    printf(
                                                        b"lsodes: itask = %ld and tout = %21.13f is behind tcur - hu (= %21.13f).\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        *itask,
                                                        *tout,
                                                        tp,
                                                    );
                                                    current_block = 2196110567356697731;
                                                } else if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                    current_block = 2953975522857967725;
                                                } else {
                                                    current_block = 14098009084916107386;
                                                }
                                            }
                                            _ => {
                                                tcrit = *rwork.offset(1 as libc::c_int as isize);
                                                if (IGS.tn - tcrit) * IGS.h > 0.0f64 {
                                                    current_block = 5487427803874674771;
                                                } else {
                                                    current_block = 17660875221514469833;
                                                }
                                            }
                                        }
                                        match current_block {
                                            15113272348145639663 => {}
                                            4298192726394742527 => {}
                                            2196110567356697731 => {}
                                            2953975522857967725 => {}
                                            14098009084916107386 => {}
                                            _ => {
                                                match current_block {
                                                    12428118024966215402 => {
                                                        printf(
                                                            b"lsodes: trouble from intdy. itask = %ld, tout = %21.13f.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            *itask,
                                                            *tout,
                                                        );
                                                        current_block = 2196110567356697731;
                                                    }
                                                    5487427803874674771 => {
                                                        printf(
                                                            b"lsodes: itask = 4 or 5 and tcrit = %21.13f, behind tcur (= %21.13f).\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            tcrit,
                                                            IGS.tn,
                                                        );
                                                        current_block = 2196110567356697731;
                                                    }
                                                    _ => {
                                                        hmx = fabs(IGS.tn) + fabs(IGS.h);
                                                        ihit = (fabs(IGS.tn - tcrit) <= IGS.uround * 100.0f64 * hmx)
                                                            as libc::c_int as libc::c_long;
                                                        if ihit != 0 {
                                                            current_block = 14098009084916107386;
                                                        } else {
                                                            tnext = IGS.tn + IGS.h * (IGS.uround * 4.0f64 + 1.0f64);
                                                            if (tnext - tcrit) * IGS.h <= 0.0f64 {
                                                                current_block = 2953975522857967725;
                                                            } else {
                                                                IGS.h = (tcrit - IGS.tn) * (1.0f64 - IGS.uround * 4.0f64);
                                                                if *istate == 2 as libc::c_int as libc::c_long {
                                                                    IGS.jstart = -(2 as libc::c_int) as libc::c_long;
                                                                }
                                                                current_block = 2953975522857967725;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            2196110567356697731 => {}
                            _ => {
                                match current_block {
                                    15113272348145639663 => {
                                        printf(
                                            b"lsodes: itask = 4 or 5 and tcrit = %21.13f, behind tout = %21.13f.\n\0"
                                                as *const u8 as *const libc::c_char,
                                            tcrit,
                                            *tout,
                                        );
                                        current_block = 2196110567356697731;
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    2196110567356697731 => {}
                                    _ => {
                                        '_L270: loop {
                                            match current_block {
                                                14098009084916107386 => {
                                                    i__1 = IGS.n;
                                                    i = 1 as libc::c_int as libc::c_long;
                                                    while i <= i__1 {
                                                        *y
                                                            .offset(
                                                                i as isize,
                                                            ) = *rwork
                                                            .offset(
                                                                (i + IGS.lyh - 1 as libc::c_int as libc::c_long) as isize,
                                                            );
                                                        i += 1;
                                                        i;
                                                    }
                                                    *t = IGS.tn;
                                                    if *itask != 4 as libc::c_int as libc::c_long
                                                        && *itask != 5 as libc::c_int as libc::c_long
                                                    {
                                                        current_block = 4298192726394742527;
                                                        continue;
                                                    }
                                                    if ihit != 0 {
                                                        *t = tcrit;
                                                    }
                                                    current_block = 4298192726394742527;
                                                }
                                                10812852962309229047 => {
                                                    tolsf = IGS.uround
                                                        * vnorm_(
                                                            &mut IGS.n,
                                                            &mut *rwork.offset(IGS.lyh as isize),
                                                            &mut *rwork.offset(IGS.lewt as isize),
                                                        );
                                                    if tolsf <= 1.0f64 {
                                                        if !(IGS.tn + IGS.h != IGS.tn) {
                                                            IGS.nhnil += 1;
                                                            IGS.nhnil;
                                                            if !(IGS.nhnil > IGS.mxhnil) {
                                                                printf(
                                                                    b"lsodes: internal t (=%21.13f) and h (=%21.13f) are such that\n        t + h = t on the next step (h = step size).\n        Solver will continue anyway.\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    IGS.tn,
                                                                    IGS.h,
                                                                );
                                                                if !(IGS.nhnil < IGS.mxhnil) {
                                                                    printf(
                                                                        b"lsodes: above warning has been issued %ld times.\n        It will not be issued again\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        IGS.mxhnil,
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        if bDelays != 0 {
                                                            StoreDelayed(IGS.tn);
                                                        }
                                                        DoStep_by_Step();
                                                        stode_(
                                                            &mut *neq.offset(0 as libc::c_int as isize),
                                                            &mut *y.offset(0 as libc::c_int as isize),
                                                            &mut *rwork.offset(IGS.lyh as isize),
                                                            &mut IGS.nyh,
                                                            &mut *rwork.offset(IGS.lyh as isize),
                                                            &mut *rwork.offset(IGS.lewt as isize),
                                                            &mut *rwork.offset(IGS.lsavf as isize),
                                                            &mut *rwork.offset(IGS.lacor as isize),
                                                            &mut *rwork.offset(IGS.lwm as isize),
                                                            &mut *rwork.offset(IGS.lwm as isize) as *mut libc::c_double
                                                                as *mut libc::c_long,
                                                        );
                                                        kgo = 1 as libc::c_int as libc::c_long - IGS.kflag;
                                                        match kgo {
                                                            2 => {
                                                                printf(
                                                                    b"lsodes: at t = %21.13f and step size h = %21.13f, the error\n        test failed repeatedly or with abs(h) = hmin.\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    IGS.tn,
                                                                    IGS.h,
                                                                );
                                                                *istate = -(4 as libc::c_int) as libc::c_long;
                                                                current_block = 5193462963640342065;
                                                                break;
                                                            }
                                                            3 => {
                                                                printf(
                                                                    b"lsodes: at t = %21.13f and step size h = %21.13f, the corrector\n        convergence failed repeatedly or with abs(h) = hmin.\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    IGS.tn,
                                                                    IGS.h,
                                                                );
                                                                *istate = -(5 as libc::c_int) as libc::c_long;
                                                                current_block = 5193462963640342065;
                                                                break;
                                                            }
                                                            4 => {
                                                                printf(
                                                                    b"lsodes: at t = %21.13f and step size h = %21.13f, a fatal error flag\n        was returned by cdrv (by way of subroutine prjs or slss.\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    IGS.tn,
                                                                    IGS.h,
                                                                );
                                                                *istate = -(7 as libc::c_int) as libc::c_long;
                                                                current_block = 14184701559676929220;
                                                                break;
                                                            }
                                                            1 | _ => {
                                                                IGS.init = 1 as libc::c_int as libc::c_long;
                                                                match *itask {
                                                                    2 => {
                                                                        current_block = 14098009084916107386;
                                                                    }
                                                                    3 => {
                                                                        if (IGS.tn - *tout) * IGS.h >= 0.0f64 {
                                                                            current_block = 14098009084916107386;
                                                                        } else {
                                                                            current_block = 2953975522857967725;
                                                                        }
                                                                    }
                                                                    4 => {
                                                                        if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                                            hmx = fabs(IGS.tn) + fabs(IGS.h);
                                                                            ihit = (fabs(IGS.tn - tcrit) <= IGS.uround * 100.0f64 * hmx)
                                                                                as libc::c_int as libc::c_long;
                                                                            if ihit != 0 {
                                                                                current_block = 14098009084916107386;
                                                                                continue;
                                                                            }
                                                                            tnext = IGS.tn + IGS.h * (IGS.uround * 4.0f64 + 1.0f64);
                                                                            if (tnext - tcrit) * IGS.h <= 0.0f64 {
                                                                                current_block = 2953975522857967725;
                                                                                continue;
                                                                            }
                                                                            IGS.h = (tcrit - IGS.tn) * (1.0f64 - IGS.uround * 4.0f64);
                                                                            IGS.jstart = -(2 as libc::c_int) as libc::c_long;
                                                                            current_block = 2953975522857967725;
                                                                        } else {
                                                                            intdy_(
                                                                                tout,
                                                                                0 as libc::c_int as libc::c_long,
                                                                                &mut *rwork.offset(IGS.lyh as isize),
                                                                                &mut IGS.nyh,
                                                                                &mut *y.offset(1 as libc::c_int as isize),
                                                                                &mut iflag,
                                                                            );
                                                                            *t = *tout;
                                                                            current_block = 4298192726394742527;
                                                                        }
                                                                    }
                                                                    5 => {
                                                                        hmx = fabs(IGS.tn) + fabs(IGS.h);
                                                                        ihit = (fabs(IGS.tn - tcrit) <= IGS.uround * 100.0f64 * hmx)
                                                                            as libc::c_int as libc::c_long;
                                                                        current_block = 14098009084916107386;
                                                                    }
                                                                    1 | _ => {
                                                                        if (IGS.tn - *tout) * IGS.h < 0.0f64 {
                                                                            current_block = 2953975522857967725;
                                                                            continue;
                                                                        }
                                                                        intdy_(
                                                                            tout,
                                                                            0 as libc::c_int as libc::c_long,
                                                                            &mut *rwork.offset(IGS.lyh as isize),
                                                                            &mut IGS.nyh,
                                                                            &mut *y.offset(1 as libc::c_int as isize),
                                                                            &mut iflag,
                                                                        );
                                                                        *t = *tout;
                                                                        current_block = 4298192726394742527;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        tolsf *= 2.0f64;
                                                        if IGS.nst == 0 as libc::c_int as libc::c_long {
                                                            current_block = 3383546865078064135;
                                                            break;
                                                        } else {
                                                            current_block = 18199692590428512734;
                                                            break;
                                                        }
                                                    }
                                                }
                                                2953975522857967725 => {
                                                    if IGS.nst - IGS.nslast >= IGS.mxstep {
                                                        printf(
                                                            b"lsodes: at t = %21.13f, mxstep (= %ld) steps\n        taken before reaching tout.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            IGS.tn,
                                                            IGS.mxstep,
                                                        );
                                                        *istate = -(1 as libc::c_int) as libc::c_long;
                                                        current_block = 14184701559676929220;
                                                        break;
                                                    } else {
                                                        ewset_(
                                                            &mut IGS.n,
                                                            itol,
                                                            &mut *rtol.offset(1 as libc::c_int as isize),
                                                            &mut *atol.offset(1 as libc::c_int as isize),
                                                            &mut *rwork.offset(IGS.lyh as isize),
                                                            &mut *rwork.offset(IGS.lewt as isize),
                                                        );
                                                        i__1 = IGS.n;
                                                        i = 1 as libc::c_int as libc::c_long;
                                                        loop {
                                                            if !(i <= i__1) {
                                                                current_block = 10812852962309229047;
                                                                continue '_L270;
                                                            }
                                                            if *rwork
                                                                .offset(
                                                                    (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                ) <= 0.0f64
                                                            {
                                                                break;
                                                            }
                                                            *rwork
                                                                .offset(
                                                                    (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                ) = 1.0f64
                                                                / *rwork
                                                                    .offset(
                                                                        (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                    );
                                                            i += 1;
                                                            i;
                                                        }
                                                        ewti = *rwork
                                                            .offset(
                                                                (IGS.lewt + i - 1 as libc::c_int as libc::c_long) as isize,
                                                            );
                                                        printf(
                                                            b"lsodes: at t = %21.13f, ewt (%ld) has become %21.13f <= 0.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            IGS.tn,
                                                            i,
                                                            ewti,
                                                        );
                                                        *istate = -(6 as libc::c_int) as libc::c_long;
                                                        current_block = 14184701559676929220;
                                                        break;
                                                    }
                                                }
                                                _ => {
                                                    *istate = 2 as libc::c_int as libc::c_long;
                                                    IGS.illin = 0 as libc::c_int as libc::c_long;
                                                    *rwork.offset(11 as libc::c_int as isize) = IGS.hu;
                                                    *rwork.offset(12 as libc::c_int as isize) = IGS.h;
                                                    *rwork.offset(13 as libc::c_int as isize) = IGS.tn;
                                                    *iwork.offset(11 as libc::c_int as isize) = IGS.nst;
                                                    *iwork.offset(12 as libc::c_int as isize) = IGS.nfe;
                                                    *iwork.offset(13 as libc::c_int as isize) = IGS.nje;
                                                    *iwork.offset(14 as libc::c_int as isize) = IGS.nqu;
                                                    *iwork.offset(15 as libc::c_int as isize) = IGS.nq;
                                                    *iwork.offset(19 as libc::c_int as isize) = IGS.nnz;
                                                    *iwork.offset(20 as libc::c_int as isize) = IGS.ngp;
                                                    *iwork.offset(21 as libc::c_int as isize) = IGS.nlu;
                                                    *iwork.offset(25 as libc::c_int as isize) = IGS.nzl;
                                                    *iwork.offset(26 as libc::c_int as isize) = IGS.nzu;
                                                    return 0 as libc::c_int;
                                                }
                                            }
                                        }
                                        match current_block {
                                            18199692590428512734 => {
                                                printf(
                                                    b"lsodes: at t = %21.13f, too much accuracy requested\n        for precision of machine (tolsf = %21.13f).\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    IGS.tn,
                                                    tolsf,
                                                );
                                                *rwork.offset(14 as libc::c_int as isize) = tolsf;
                                                *istate = -(2 as libc::c_int) as libc::c_long;
                                                current_block = 14184701559676929220;
                                            }
                                            5193462963640342065 => {
                                                big = 0.0f64;
                                                imxer = 1 as libc::c_int as libc::c_long;
                                                i__1 = IGS.n;
                                                i = 1 as libc::c_int as libc::c_long;
                                                while i <= i__1 {
                                                    size = fabs(
                                                        *rwork
                                                            .offset(
                                                                (i + IGS.lacor - 1 as libc::c_int as libc::c_long) as isize,
                                                            )
                                                            * *rwork
                                                                .offset(
                                                                    (i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize,
                                                                ),
                                                    );
                                                    if !(big >= size) {
                                                        big = size;
                                                        imxer = i;
                                                    }
                                                    i += 1;
                                                    i;
                                                }
                                                *iwork.offset(16 as libc::c_int as isize) = imxer;
                                                current_block = 14184701559676929220;
                                            }
                                            3383546865078064135 => {
                                                printf(
                                                    b"lsodes: at start of the problem, too much accuracy requested\n        for precision of machine (tolsf = %21.13f).\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    tolsf,
                                                );
                                                *rwork.offset(14 as libc::c_int as isize) = tolsf;
                                                current_block = 2196110567356697731;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            2196110567356697731 => {}
                                            _ => {
                                                i__1 = IGS.n;
                                                i = 1 as libc::c_int as libc::c_long;
                                                while i <= i__1 {
                                                    *y
                                                        .offset(
                                                            i as isize,
                                                        ) = *rwork
                                                        .offset(
                                                            (i + IGS.lyh - 1 as libc::c_int as libc::c_long) as isize,
                                                        );
                                                    i += 1;
                                                    i;
                                                }
                                                *t = IGS.tn;
                                                IGS.illin = 0 as libc::c_int as libc::c_long;
                                                *rwork.offset(11 as libc::c_int as isize) = IGS.hu;
                                                *rwork.offset(12 as libc::c_int as isize) = IGS.h;
                                                *rwork.offset(13 as libc::c_int as isize) = IGS.tn;
                                                *iwork.offset(11 as libc::c_int as isize) = IGS.nst;
                                                *iwork.offset(12 as libc::c_int as isize) = IGS.nfe;
                                                *iwork.offset(13 as libc::c_int as isize) = IGS.nje;
                                                *iwork.offset(14 as libc::c_int as isize) = IGS.nqu;
                                                *iwork.offset(15 as libc::c_int as isize) = IGS.nq;
                                                *iwork.offset(19 as libc::c_int as isize) = IGS.nnz;
                                                *iwork.offset(20 as libc::c_int as isize) = IGS.ngp;
                                                *iwork.offset(21 as libc::c_int as isize) = IGS.nlu;
                                                *iwork.offset(25 as libc::c_int as isize) = IGS.nzl;
                                                *iwork.offset(26 as libc::c_int as isize) = IGS.nzu;
                                                return 0 as libc::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        2196110567356697731 => {
            if IGS.illin == 5 as libc::c_int as libc::c_long {
                printf(
                    b"lsodes: repeated occurrences of illegal input.\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                IGS.illin += 1;
                IGS.illin;
                *istate = -(3 as libc::c_int) as libc::c_long;
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    printf(
        b"lsodes: run aborted, apparent infinite loop.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    abort();
}
pub unsafe extern "C" fn slss_(
    mut wk: *mut libc::c_double,
    mut iwk: *mut libc::c_long,
    mut x: *mut libc::c_double,
    mut tem: *mut libc::c_double,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__1: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut r: libc::c_double = 0.;
    let mut di: libc::c_double = 0.;
    let mut hl0: libc::c_double = 0.;
    let mut phl0: libc::c_double = 0.;
    tem = tem.offset(-1);
    tem;
    x = x.offset(-1);
    x;
    iwk = iwk.offset(-1);
    iwk;
    wk = wk.offset(-1);
    wk;
    IGS.iersl = 0 as libc::c_int as libc::c_long;
    match IGS.miter {
        3 => {
            phl0 = *wk.offset(2 as libc::c_int as isize);
            hl0 = IGS.h * IGS.el0;
            *wk.offset(2 as libc::c_int as isize) = hl0;
            if !(hl0 == phl0) {
                r = hl0 / phl0;
                i__1 = IGS.n;
                i = 1 as libc::c_int as libc::c_long;
                loop {
                    if !(i <= i__1) {
                        current_block = 7171884347553071733;
                        break;
                    }
                    di = 1.0f64
                        - r
                            * (1.0f64
                                - 1.0f64
                                    / *wk
                                        .offset((i + 2 as libc::c_int as libc::c_long) as isize));
                    if fabs(di) == 0.0f64 {
                        current_block = 12545189862335960440;
                        break;
                    }
                    *wk
                        .offset(
                            (i + 2 as libc::c_int as libc::c_long) as isize,
                        ) = 1.0f64 / di;
                    i += 1;
                    i;
                }
                match current_block {
                    7171884347553071733 => {}
                    _ => {
                        IGS.iersl = 1 as libc::c_int as libc::c_long;
                        return 0 as libc::c_int;
                    }
                }
            }
            i__1 = IGS.n;
            i = 1 as libc::c_int as libc::c_long;
            while i <= i__1 {
                *x
                    .offset(
                        i as isize,
                    ) = *wk.offset((i + 2 as libc::c_int as libc::c_long) as isize)
                    * *x.offset(i as isize);
                i += 1;
                i;
            }
            return 0 as libc::c_int;
        }
        1 | 2 | _ => {
            cdrv_(
                &mut IGS.n,
                &mut *iwk.offset(IGS.ipr as isize),
                &mut *iwk.offset(IGS.ipc as isize),
                &mut *iwk.offset(IGS.ipic as isize),
                &mut *iwk.offset(IGS.ipian as isize),
                &mut *iwk.offset(IGS.ipjan as isize),
                &mut *wk.offset(IGS.ipa as isize),
                &mut *x.offset(1 as libc::c_int as isize),
                &mut *x.offset(1 as libc::c_int as isize),
                &mut IGS.nsp,
                &mut *iwk.offset(IGS.ipisp as isize),
                &mut *wk.offset(IGS.iprsp as isize),
                &mut IGS.iesp,
                4 as libc::c_int as libc::c_long,
                &mut IGS.iersl,
            );
            if IGS.iersl != 0 as libc::c_int as libc::c_long {
                IGS.iersl = -(1 as libc::c_int) as libc::c_long;
            }
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn prjs_(
    mut neq: *mut libc::c_long,
    mut y: *mut libc::c_double,
    mut yh: *mut libc::c_double,
    mut nyh: *mut libc::c_long,
    mut ewt: *mut libc::c_double,
    mut ftem: *mut libc::c_double,
    mut savf: *mut libc::c_double,
    mut wk: *mut libc::c_double,
    mut iwk: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut yh_dim1: libc::c_long = 0;
    let mut yh_offset: libc::c_long = 0;
    let mut i__1: libc::c_long = 0;
    let mut i__2: libc::c_long = 0;
    let mut i__3: libc::c_long = 0;
    let mut d__1: libc::c_double = 0.;
    let mut d__2: libc::c_double = 0.;
    let mut d__3: libc::c_double = 0.;
    let mut jmin: libc::c_long = 0;
    let mut jmax: libc::c_long = 0;
    let mut imul: libc::c_long = 0;
    let mut kmax: libc::c_long = 0;
    let mut kmin: libc::c_long = 0;
    let mut rcon: libc::c_double = 0.;
    let mut srur: libc::c_double = 0.;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut r: libc::c_double = 0.;
    let mut rcont: libc::c_double = 0.;
    let mut r0: libc::c_double = 0.;
    let mut di: libc::c_double = 0.;
    let mut jj: libc::c_long = 0;
    let mut ng: libc::c_long = 0;
    let mut hl0: libc::c_double = 0.;
    let mut fac: libc::c_double = 0.;
    let mut con: libc::c_double = 0.;
    let mut pij: libc::c_double = 0.;
    let mut jok: libc::c_long = 0;
    iwk = iwk.offset(-1);
    iwk;
    wk = wk.offset(-1);
    wk;
    savf = savf.offset(-1);
    savf;
    ftem = ftem.offset(-1);
    ftem;
    ewt = ewt.offset(-1);
    ewt;
    yh_dim1 = *nyh;
    yh_offset = yh_dim1 + 1 as libc::c_int as libc::c_long;
    yh = yh.offset(-(yh_offset as isize));
    y = y.offset(-1);
    y;
    neq = neq.offset(-1);
    neq;
    hl0 = IGS.h * IGS.el0;
    con = -hl0;
    if IGS.miter == 3 as libc::c_int as libc::c_long {
        IGS.jcur = 1 as libc::c_int as libc::c_long;
        IGS.nje += 1;
        IGS.nje;
        *wk.offset(2 as libc::c_int as isize) = hl0;
        IGS.ierpj = 0 as libc::c_int as libc::c_long;
        r = IGS.el0 * 0.1f64;
        i = 1 as libc::c_int as libc::c_long;
        while i <= IGS.n {
            *y.offset(i as isize)
                += r
                    * (IGS.h * *savf.offset(i as isize)
                        - *yh.offset((i + (yh_dim1 << 1 as libc::c_int)) as isize));
            i += 1;
            i;
        }
        CalcDeriv(
            &mut *y.offset(1 as libc::c_int as isize),
            &mut *wk.offset(3 as libc::c_int as isize),
            &mut IGS.tn,
        );
        IGS.nfe += 1;
        IGS.nfe;
        i = 1 as libc::c_int as libc::c_long;
        while i <= IGS.n {
            r0 = IGS.h * *savf.offset(i as isize)
                - *yh.offset((i + (yh_dim1 << 1 as libc::c_int)) as isize);
            di = r0 * 0.1f64
                - IGS.h
                    * (*wk.offset((i + 2 as libc::c_int as libc::c_long) as isize)
                        - *savf.offset(i as isize));
            *wk.offset((i + 2 as libc::c_int as libc::c_long) as isize) = 1.0f64;
            if fabs(r0) >= IGS.uround / *ewt.offset(i as isize) {
                if fabs(di) == 0.0f64 {
                    IGS.ierpj = 2 as libc::c_int as libc::c_long;
                    return 0 as libc::c_int;
                }
                *wk
                    .offset(
                        (i + 2 as libc::c_int as libc::c_long) as isize,
                    ) = r0 * 0.1f64 / di;
            }
            i += 1;
            i;
        }
        return 0 as libc::c_int;
    }
    jok = 1 as libc::c_int as libc::c_long;
    if IGS.nst == 0 as libc::c_int as libc::c_long || IGS.nst >= IGS.nslj + IGS.msbj {
        jok = 0 as libc::c_int as libc::c_long;
    }
    if IGS.icf == 1 as libc::c_int as libc::c_long
        && fabs(IGS.rc - 1 as libc::c_int as libc::c_double) < IGS.ccmxj
    {
        jok = 0 as libc::c_int as libc::c_long;
    }
    if IGS.icf == 2 as libc::c_int as libc::c_long {
        jok = 0 as libc::c_int as libc::c_long;
    }
    if jok == 1 as libc::c_int as libc::c_long {
        IGS.jcur = 0 as libc::c_int as libc::c_long;
        rcon = con / IGS.con0;
        rcont = fabs(con) / IGS.conmin;
        if rcont > IGS.rbig && IGS.iplost == 1 as libc::c_int as libc::c_long {
            current_block = 15161419463650305643;
        } else {
            kmin = *iwk.offset(IGS.ipian as isize);
            i__1 = IGS.n;
            j = 1 as libc::c_int as libc::c_long;
            while j <= i__1 {
                kmax = *iwk.offset((IGS.ipian + j) as isize)
                    - 1 as libc::c_int as libc::c_long;
                i__2 = kmax;
                k = kmin;
                while k <= i__2 {
                    i = *iwk.offset((IGS.ibjan + k) as isize);
                    pij = *wk.offset((IGS.iba + k) as isize);
                    if !(i != j) {
                        pij += -1.0f64;
                        if !(fabs(pij) >= IGS.psmall) {
                            IGS.iplost = 1 as libc::c_int as libc::c_long;
                            d__1 = fabs(IGS.con0);
                            IGS
                                .conmin = if d__1 <= IGS.conmin {
                                d__1
                            } else {
                                IGS.conmin
                            };
                        }
                    }
                    pij *= rcon;
                    if i == j {
                        pij += 1.0f64;
                    }
                    *wk.offset((IGS.iba + k) as isize) = pij;
                    k += 1;
                    k;
                }
                kmin = kmax + 1 as libc::c_int as libc::c_long;
                j += 1;
                j;
            }
            current_block = 16339807805050138676;
        }
    } else {
        current_block = 15161419463650305643;
    }
    match current_block {
        15161419463650305643 => {
            IGS.jcur = 1 as libc::c_int as libc::c_long;
            IGS.nje += 1;
            IGS.nje;
            IGS.nslj = IGS.nst;
            IGS.iplost = 0 as libc::c_int as libc::c_long;
            IGS.conmin = fabs(con);
            match IGS.miter {
                2 => {
                    fac = vnorm_(
                        &mut IGS.n,
                        &mut *savf.offset(1 as libc::c_int as isize),
                        &mut *ewt.offset(1 as libc::c_int as isize),
                    );
                    r0 = fabs(IGS.h) * 1e3f64 * IGS.uround * IGS.n as libc::c_double
                        * fac;
                    if r0 == 0.0f64 {
                        r0 = 1.0f64;
                    }
                    srur = *wk.offset(1 as libc::c_int as isize);
                    jmin = *iwk.offset(IGS.ipigp as isize);
                    i__1 = IGS.ngp;
                    ng = 1 as libc::c_int as libc::c_long;
                    while ng <= i__1 {
                        jmax = *iwk.offset((IGS.ipigp + ng) as isize)
                            - 1 as libc::c_int as libc::c_long;
                        i__2 = jmax;
                        j = jmin;
                        while j <= i__2 {
                            jj = *iwk.offset((IGS.ibjgp + j) as isize);
                            d__2 = srur * fabs(*y.offset(jj as isize));
                            d__3 = r0 / *ewt.offset(jj as isize);
                            r = if d__2 >= d__3 { d__2 } else { d__3 };
                            *y.offset(jj as isize) += r;
                            j += 1;
                            j;
                        }
                        CalcDeriv(
                            &mut *y.offset(1 as libc::c_int as isize),
                            &mut *ftem.offset(1 as libc::c_int as isize),
                            &mut IGS.tn,
                        );
                        i__2 = jmax;
                        j = jmin;
                        while j <= i__2 {
                            jj = *iwk.offset((IGS.ibjgp + j) as isize);
                            *y.offset(jj as isize) = *yh.offset((jj + yh_dim1) as isize);
                            d__2 = srur * fabs(*y.offset(jj as isize));
                            d__3 = r0 / *ewt.offset(jj as isize);
                            r = if d__2 >= d__3 { d__2 } else { d__3 };
                            fac = -hl0 / r;
                            kmin = *iwk.offset((IGS.ibian + jj) as isize);
                            kmax = *iwk
                                .offset(
                                    (IGS.ibian + jj + 1 as libc::c_int as libc::c_long) as isize,
                                ) - 1 as libc::c_int as libc::c_long;
                            i__3 = kmax;
                            k = kmin;
                            while k <= i__3 {
                                i = *iwk.offset((IGS.ibjan + k) as isize);
                                *wk
                                    .offset(
                                        (IGS.iba + k) as isize,
                                    ) = (*ftem.offset(i as isize) - *savf.offset(i as isize))
                                    * fac;
                                if i == jj {
                                    *wk.offset((IGS.iba + k) as isize) += 1.0f64;
                                }
                                k += 1;
                                k;
                            }
                            j += 1;
                            j;
                        }
                        jmin = jmax + 1 as libc::c_int as libc::c_long;
                        ng += 1;
                        ng;
                    }
                    IGS.nfe += IGS.ngp;
                }
                1 | _ => {
                    kmin = *iwk.offset(IGS.ipian as isize);
                    i__1 = IGS.n;
                    j = 1 as libc::c_int as libc::c_long;
                    while j <= i__1 {
                        kmax = *iwk.offset((IGS.ipian + j) as isize)
                            - 1 as libc::c_int as libc::c_long;
                        i = 1 as libc::c_int as libc::c_long;
                        while i <= IGS.n {
                            *ftem.offset(i as isize) = 0.0f64;
                            i += 1;
                            i;
                        }
                        CalcJacob(
                            &mut IGS.tn,
                            &mut *y.offset(1 as libc::c_int as isize),
                            j,
                            &mut *ftem.offset(1 as libc::c_int as isize),
                        );
                        i__2 = kmax;
                        k = kmin;
                        while k <= i__2 {
                            i = *iwk.offset((IGS.ibjan + k) as isize);
                            *wk
                                .offset(
                                    (IGS.iba + k) as isize,
                                ) = *ftem.offset(i as isize) * con;
                            if i == j {
                                *wk.offset((IGS.iba + k) as isize) += 1.0f64;
                            }
                            k += 1;
                            k;
                        }
                        kmin = kmax + 1 as libc::c_int as libc::c_long;
                        j += 1;
                        j;
                    }
                }
            }
        }
        _ => {}
    }
    IGS.nlu += 1;
    IGS.nlu;
    IGS.con0 = con;
    IGS.ierpj = 0 as libc::c_int as libc::c_long;
    i__1 = IGS.n;
    i = 1 as libc::c_int as libc::c_long;
    while i <= i__1 {
        *ftem.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    cdrv_(
        &mut IGS.n,
        &mut *iwk.offset(IGS.ipr as isize),
        &mut *iwk.offset(IGS.ipc as isize),
        &mut *iwk.offset(IGS.ipic as isize),
        &mut *iwk.offset(IGS.ipian as isize),
        &mut *iwk.offset(IGS.ipjan as isize),
        &mut *wk.offset(IGS.ipa as isize),
        &mut *ftem.offset(1 as libc::c_int as isize),
        &mut *ftem.offset(1 as libc::c_int as isize),
        &mut IGS.nsp,
        &mut *iwk.offset(IGS.ipisp as isize),
        &mut *wk.offset(IGS.iprsp as isize),
        &mut IGS.iesp,
        2 as libc::c_int as libc::c_long,
        &mut IGS.iys,
    );
    if IGS.iys == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    imul = (IGS.iys - 1 as libc::c_int as libc::c_long) / IGS.n;
    IGS.ierpj = -(2 as libc::c_int) as libc::c_long;
    if imul == 8 as libc::c_int as libc::c_long {
        IGS.ierpj = 1 as libc::c_int as libc::c_long;
    }
    if imul == 10 as libc::c_int as libc::c_long {
        IGS.ierpj = -(1 as libc::c_int) as libc::c_long;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn stode_(
    mut neq: *mut libc::c_long,
    mut y: *mut libc::c_double,
    mut yh: *mut libc::c_double,
    mut nyh: *mut libc::c_long,
    mut yh1: *mut libc::c_double,
    mut ewt: *mut libc::c_double,
    mut savf: *mut libc::c_double,
    mut acor: *mut libc::c_double,
    mut wm: *mut libc::c_double,
    mut iwm: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut yh_dim1: libc::c_long = 0;
    let mut yh_offset: libc::c_long = 0;
    let mut i__1: libc::c_long = 0;
    let mut i__2: libc::c_long = 0;
    let mut d__1: libc::c_double = 0.;
    let mut d__2: libc::c_double = 0.;
    let mut d__3: libc::c_double = 0.;
    let mut dcon: libc::c_double = 0.;
    let mut delp: libc::c_double = 0.;
    let mut rhdn: libc::c_double = 0.;
    let mut exdn: libc::c_double = 0.;
    let mut iret: libc::c_long = 0;
    let mut told: libc::c_double = 0.;
    let mut rhsm: libc::c_double = 0.;
    let mut newq: libc::c_long = 0;
    let mut exsm: libc::c_double = 0.;
    let mut rhup: libc::c_double = 0.;
    let mut exup: libc::c_double = 0.;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    let mut r: libc::c_double = 0.;
    let mut i1: libc::c_long = 0;
    let mut iredo: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut jb: libc::c_long = 0;
    let mut del: libc::c_double = 0.;
    let mut ddn: libc::c_double = 0.;
    let mut rh: libc::c_double = 0.0f64;
    let mut ncf: libc::c_long = 0;
    let mut dup: libc::c_double = 0.;
    let mut dsm: libc::c_double = 0.0f64;
    iwm = iwm.offset(-1);
    iwm;
    wm = wm.offset(-1);
    wm;
    acor = acor.offset(-1);
    acor;
    savf = savf.offset(-1);
    savf;
    ewt = ewt.offset(-1);
    ewt;
    yh1 = yh1.offset(-1);
    yh1;
    yh_dim1 = *nyh;
    yh_offset = yh_dim1 + 1 as libc::c_int as libc::c_long;
    yh = yh.offset(-(yh_offset as isize));
    IGS.kflag = 0 as libc::c_int as libc::c_long;
    told = IGS.tn;
    ncf = 0 as libc::c_int as libc::c_long;
    IGS.ierpj = 0 as libc::c_int as libc::c_long;
    IGS.iersl = 0 as libc::c_int as libc::c_long;
    IGS.jcur = 0 as libc::c_int as libc::c_long;
    IGS.icf = 0 as libc::c_int as libc::c_long;
    delp = 0.0f64;
    if IGS.jstart > 0 as libc::c_int as libc::c_long {
        current_block = 17586091195355511184;
    } else if IGS.jstart == -(1 as libc::c_int) as libc::c_long {
        IGS.ipup = IGS.miter;
        IGS.lmax = IGS.maxord + 1 as libc::c_int as libc::c_long;
        if IGS.ialth == 1 as libc::c_int as libc::c_long {
            IGS.ialth = 2 as libc::c_int as libc::c_long;
        }
        if IGS.meth == IGS.meo {
            if IGS.nq <= IGS.maxord {
                current_block = 12450414705071241475;
            } else {
                current_block = 18350126124679100982;
            }
        } else {
            cfode_(&mut IGS.meth, (IGS.elco).as_mut_ptr(), (IGS.tesco).as_mut_ptr());
            IGS.meo = IGS.meth;
            if IGS.nq > IGS.maxord {
                current_block = 18350126124679100982;
            } else {
                IGS.ialth = IGS.l;
                iret = 1 as libc::c_int as libc::c_long;
                current_block = 10139636376225292192;
            }
        }
        match current_block {
            12450414705071241475 => {}
            10139636376225292192 => {}
            _ => {
                IGS.nq = IGS.maxord;
                IGS.l = IGS.lmax;
                i__1 = IGS.l;
                i = 0 as libc::c_int as libc::c_long;
                while i < i__1 {
                    IGS
                        .el[i
                        as usize] = IGS
                        .elco[(i + IGS.nq * 13 as libc::c_int as libc::c_long
                        - 13 as libc::c_int as libc::c_long) as usize];
                    i += 1;
                    i;
                }
                IGS.nqnyh = IGS.nq * *nyh;
                IGS.rc = IGS.rc * IGS.el[0 as libc::c_int as usize] / IGS.el0;
                IGS.el0 = IGS.el[0 as libc::c_int as usize];
                IGS
                    .conit = 0.5f64
                    / (IGS.nq + 2 as libc::c_int as libc::c_long) as libc::c_double;
                ddn = vnorm_(
                    &mut IGS.n,
                    &mut *savf.offset(1 as libc::c_int as isize),
                    &mut *ewt.offset(1 as libc::c_int as isize),
                )
                    / IGS
                        .tesco[(IGS.l * 3 as libc::c_int as libc::c_long
                        - 3 as libc::c_int as libc::c_long) as usize];
                exdn = 1.0f64 / IGS.l as libc::c_double;
                rhdn = 1.0f64 / (pow(ddn, exdn) * 1.3f64 + 1.3e-6f64);
                rh = if rhdn <= 1.0f64 { rhdn } else { 1.0f64 };
                iredo = 3 as libc::c_int as libc::c_long;
                if IGS.h == IGS.hold {
                    current_block = 3642853594056902449;
                } else {
                    d__3 = fabs(IGS.h / IGS.hold);
                    rh = if rh <= d__3 { rh } else { d__3 };
                    IGS.h = IGS.hold;
                    current_block = 8190416105714964230;
                }
            }
        }
    } else if IGS.jstart == -(2 as libc::c_int) as libc::c_long {
        current_block = 12450414705071241475;
    } else {
        IGS.lmax = IGS.maxord + 1 as libc::c_int as libc::c_long;
        IGS.nq = 1 as libc::c_int as libc::c_long;
        IGS.l = 2 as libc::c_int as libc::c_long;
        IGS.ialth = 2 as libc::c_int as libc::c_long;
        IGS.rmax = 1e4f64;
        IGS.rc = 0.0f64;
        IGS.el0 = 1.0f64;
        IGS.crate_0 = 0.7f64;
        IGS.hold = IGS.h;
        IGS.meo = IGS.meth;
        IGS.nslp = 0 as libc::c_int as libc::c_long;
        IGS.ipup = IGS.miter;
        iret = 3 as libc::c_int as libc::c_long;
        cfode_(&mut IGS.meth, (IGS.elco).as_mut_ptr(), (IGS.tesco).as_mut_ptr());
        current_block = 10139636376225292192;
    }
    loop {
        match current_block {
            10139636376225292192 => {
                i__1 = IGS.l;
                i = 1 as libc::c_int as libc::c_long;
                while i <= i__1 {
                    IGS
                        .el[(i - 1 as libc::c_int as libc::c_long)
                        as usize] = IGS
                        .elco[(i + IGS.nq * 13 as libc::c_int as libc::c_long
                        - 14 as libc::c_int as libc::c_long) as usize];
                    i += 1;
                    i;
                }
                IGS.nqnyh = IGS.nq * *nyh;
                IGS.rc = IGS.rc * IGS.el[0 as libc::c_int as usize] / IGS.el0;
                IGS.el0 = IGS.el[0 as libc::c_int as usize];
                IGS
                    .conit = 0.5f64
                    / (IGS.nq + 2 as libc::c_int as libc::c_long) as libc::c_double;
                match iret {
                    2 => {
                        current_block = 3642853594056902449;
                    }
                    3 => {
                        current_block = 17586091195355511184;
                    }
                    1 | _ => {
                        current_block = 12450414705071241475;
                    }
                }
            }
            8190416105714964230 => {
                rh = if rh <= IGS.rmax { rh } else { IGS.rmax };
                d__2 = fabs(IGS.h) * IGS.hmxi * rh;
                rh /= if 1.0f64 >= d__2 { 1.0f64 } else { d__2 };
                r = 1.0f64;
                i__1 = IGS.l;
                j = 2 as libc::c_int as libc::c_long;
                while j <= i__1 {
                    r *= rh;
                    i = 1 as libc::c_int as libc::c_long;
                    while i <= IGS.n {
                        *yh.offset((i + j * yh_dim1) as isize) *= r;
                        i += 1;
                        i;
                    }
                    j += 1;
                    j;
                }
                IGS.h *= rh;
                IGS.rc *= rh;
                IGS.ialth = IGS.l;
                if !(iredo == 0 as libc::c_int as libc::c_long) {
                    current_block = 17586091195355511184;
                    continue;
                }
                IGS.rmax = 10.0f64;
                current_block = 11681593226181908859;
                break;
            }
            3642853594056902449 => {
                d__2 = IGS.hmin / fabs(IGS.h);
                rh = if rh >= d__2 { rh } else { d__2 };
                current_block = 8190416105714964230;
            }
            12450414705071241475 => {
                if IGS.h == IGS.hold {
                    current_block = 17586091195355511184;
                    continue;
                }
                rh = IGS.h / IGS.hold;
                IGS.h = IGS.hold;
                iredo = 3 as libc::c_int as libc::c_long;
                current_block = 8190416105714964230;
            }
            _ => {
                if fabs(IGS.rc - 1 as libc::c_int as libc::c_double) > IGS.ccmax {
                    IGS.ipup = IGS.miter;
                }
                if IGS.nst >= IGS.nslp + IGS.msbp {
                    IGS.ipup = IGS.miter;
                }
                IGS.tn += IGS.h;
                i1 = IGS.nqnyh + 1 as libc::c_int as libc::c_long;
                i__2 = IGS.nq;
                jb = 1 as libc::c_int as libc::c_long;
                while jb <= i__2 {
                    i1 -= *nyh;
                    i__1 = IGS.nqnyh;
                    i = i1;
                    while i <= i__1 {
                        *yh1.offset(i as isize) += *yh1.offset((i + *nyh) as isize);
                        i += 1;
                        i;
                    }
                    jb += 1;
                    jb;
                }
                '_L220: loop {
                    m = 0 as libc::c_int as libc::c_long;
                    i__2 = IGS.n;
                    i = 1 as libc::c_int as libc::c_long;
                    while i <= i__2 {
                        *y.offset(i as isize) = *yh.offset((i + yh_dim1) as isize);
                        i += 1;
                        i;
                    }
                    CalcDeriv(
                        &mut *y.offset(1 as libc::c_int as isize),
                        &mut *savf.offset(1 as libc::c_int as isize),
                        &mut IGS.tn,
                    );
                    IGS.nfe += 1;
                    IGS.nfe;
                    if !(IGS.ipup <= 0 as libc::c_int as libc::c_long) {
                        prjs_(
                            &mut *neq.offset(1 as libc::c_int as isize),
                            &mut *y.offset(1 as libc::c_int as isize),
                            &mut *yh.offset(yh_offset as isize),
                            nyh,
                            &mut *ewt.offset(1 as libc::c_int as isize),
                            &mut *acor.offset(1 as libc::c_int as isize),
                            &mut *savf.offset(1 as libc::c_int as isize),
                            &mut *wm.offset(1 as libc::c_int as isize),
                            &mut *iwm.offset(1 as libc::c_int as isize),
                        );
                        IGS.ipup = 0 as libc::c_int as libc::c_long;
                        IGS.rc = 1.0f64;
                        IGS.nslp = IGS.nst;
                        IGS.crate_0 = 0.7f64;
                        if IGS.ierpj != 0 as libc::c_int as libc::c_long {
                            current_block = 6535090061594612395;
                            break;
                        }
                    }
                    i = 1 as libc::c_int as libc::c_long;
                    while i <= IGS.n {
                        *acor.offset(i as isize) = 0.0f64;
                        i += 1;
                        i;
                    }
                    loop {
                        if IGS.miter != 0 as libc::c_int as libc::c_long {
                            i__2 = IGS.n;
                            i = 1 as libc::c_int as libc::c_long;
                            while i <= i__2 {
                                *y
                                    .offset(
                                        i as isize,
                                    ) = IGS.h * *savf.offset(i as isize)
                                    - (*yh.offset((i + (yh_dim1 << 1 as libc::c_int)) as isize)
                                        + *acor.offset(i as isize));
                                i += 1;
                                i;
                            }
                            slss_(
                                &mut *wm.offset(1 as libc::c_int as isize),
                                &mut *iwm.offset(1 as libc::c_int as isize),
                                &mut *y.offset(1 as libc::c_int as isize),
                                &mut *savf.offset(1 as libc::c_int as isize),
                            );
                            if IGS.iersl < 0 as libc::c_int as libc::c_long {
                                current_block = 6535090061594612395;
                                break '_L220;
                            }
                            if IGS.iersl > 0 as libc::c_int as libc::c_long {
                                break;
                            }
                            del = vnorm_(
                                &mut IGS.n,
                                &mut *y.offset(1 as libc::c_int as isize),
                                &mut *ewt.offset(1 as libc::c_int as isize),
                            );
                            i__2 = IGS.n;
                            i = 1 as libc::c_int as libc::c_long;
                            while i <= i__2 {
                                *acor.offset(i as isize) += *y.offset(i as isize);
                                *y
                                    .offset(
                                        i as isize,
                                    ) = *yh.offset((i + yh_dim1) as isize)
                                    + IGS.el[0 as libc::c_int as usize]
                                        * *acor.offset(i as isize);
                                i += 1;
                                i;
                            }
                        } else {
                            i__2 = IGS.n;
                            i = 1 as libc::c_int as libc::c_long;
                            while i <= i__2 {
                                *savf
                                    .offset(
                                        i as isize,
                                    ) = IGS.h * *savf.offset(i as isize)
                                    - *yh.offset((i + (yh_dim1 << 1 as libc::c_int)) as isize);
                                *y
                                    .offset(
                                        i as isize,
                                    ) = *savf.offset(i as isize) - *acor.offset(i as isize);
                                i += 1;
                                i;
                            }
                            del = vnorm_(
                                &mut IGS.n,
                                &mut *y.offset(1 as libc::c_int as isize),
                                &mut *ewt.offset(1 as libc::c_int as isize),
                            );
                            i = 1 as libc::c_int as libc::c_long;
                            while i <= IGS.n {
                                *y
                                    .offset(
                                        i as isize,
                                    ) = *yh.offset((i + yh_dim1) as isize)
                                    + IGS.el[0 as libc::c_int as usize]
                                        * *savf.offset(i as isize);
                                *acor.offset(i as isize) = *savf.offset(i as isize);
                                i += 1;
                                i;
                            }
                        }
                        if m != 0 as libc::c_int as libc::c_long {
                            d__1 = IGS.crate_0 * 0.2f64;
                            d__2 = del / delp;
                            IGS.crate_0 = if d__1 >= d__2 { d__1 } else { d__2 };
                        }
                        d__2 = IGS.crate_0 * 1.5f64;
                        dcon = del * (if 1.0f64 <= d__2 { 1.0f64 } else { d__2 })
                            / (IGS
                                .tesco[(IGS.nq * 3 as libc::c_int as libc::c_long
                                - 2 as libc::c_int as libc::c_long) as usize] * IGS.conit);
                        if dcon <= 1.0f64 {
                            IGS.jcur = 0 as libc::c_int as libc::c_long;
                            if m == 0 as libc::c_int as libc::c_long {
                                dsm = del
                                    / IGS
                                        .tesco[(IGS.nq * 3 as libc::c_int as libc::c_long
                                        - 2 as libc::c_int as libc::c_long) as usize];
                            }
                            if m > 0 as libc::c_int as libc::c_long {
                                dsm = vnorm_(
                                    &mut IGS.n,
                                    &mut *acor.offset(1 as libc::c_int as isize),
                                    &mut *ewt.offset(1 as libc::c_int as isize),
                                )
                                    / IGS
                                        .tesco[(IGS.nq * 3 as libc::c_int as libc::c_long
                                        - 2 as libc::c_int as libc::c_long) as usize];
                            }
                            if dsm > 1.0f64 {
                                current_block = 1920808331654117614;
                                break '_L220;
                            } else {
                                current_block = 7198204517578597543;
                                break '_L220;
                            }
                        } else {
                            m += 1;
                            m;
                            if m == IGS.maxcor {
                                break;
                            }
                            if m >= 2 as libc::c_int as libc::c_long
                                && del > delp * 2.0f64
                            {
                                break;
                            }
                            delp = del;
                            CalcDeriv(
                                &mut *y.offset(1 as libc::c_int as isize),
                                &mut *savf.offset(1 as libc::c_int as isize),
                                &mut IGS.tn,
                            );
                            IGS.nfe += 1;
                            IGS.nfe;
                        }
                    }
                    if IGS.miter == 0 as libc::c_int as libc::c_long
                        || IGS.jcur == 1 as libc::c_int as libc::c_long
                    {
                        current_block = 6535090061594612395;
                        break;
                    }
                    IGS.icf = 1 as libc::c_int as libc::c_long;
                    IGS.ipup = IGS.miter;
                }
                match current_block {
                    6535090061594612395 => {
                        IGS.icf = 2 as libc::c_int as libc::c_long;
                        ncf += 1;
                        ncf;
                        IGS.rmax = 2.0f64;
                        IGS.tn = told;
                        i1 = IGS.nqnyh + 1 as libc::c_int as libc::c_long;
                        jb = 1 as libc::c_int as libc::c_long;
                        while jb <= IGS.nq {
                            i1 -= *nyh;
                            i = i1;
                            while i <= IGS.nqnyh {
                                *yh1.offset(i as isize) -= *yh1.offset((i + *nyh) as isize);
                                i += 1;
                                i;
                            }
                            jb += 1;
                            jb;
                        }
                        if IGS.ierpj < 0 as libc::c_int as libc::c_long
                            || IGS.iersl < 0 as libc::c_int as libc::c_long
                        {
                            IGS.kflag = -(3 as libc::c_int) as libc::c_long;
                            current_block = 11802380472089656092;
                            break;
                        } else {
                            if fabs(IGS.h) <= IGS.hmin * 1.00001f64 {
                                current_block = 14836854278375750052;
                                break;
                            }
                            if ncf == IGS.mxncf {
                                current_block = 14836854278375750052;
                                break;
                            }
                            rh = 0.25f64;
                            IGS.ipup = IGS.miter;
                            iredo = 1 as libc::c_int as libc::c_long;
                            current_block = 3642853594056902449;
                            continue;
                        }
                    }
                    1920808331654117614 => {
                        IGS.kflag -= 1;
                        IGS.kflag;
                        IGS.tn = told;
                        i1 = IGS.nqnyh + 1 as libc::c_int as libc::c_long;
                        i__1 = IGS.nq;
                        jb = 1 as libc::c_int as libc::c_long;
                        while jb <= i__1 {
                            i1 -= *nyh;
                            i__2 = IGS.nqnyh;
                            i = i1;
                            while i <= i__2 {
                                *yh1.offset(i as isize) -= *yh1.offset((i + *nyh) as isize);
                                i += 1;
                                i;
                            }
                            jb += 1;
                            jb;
                        }
                        IGS.rmax = 2.0f64;
                        if fabs(IGS.h) <= IGS.hmin * 1.00001f64 {
                            current_block = 15578571638613445014;
                            break;
                        }
                        if IGS.kflag <= -(3 as libc::c_int) as libc::c_long {
                            if IGS.kflag == -(10 as libc::c_int) as libc::c_long {
                                current_block = 15578571638613445014;
                                break;
                            }
                            d__1 = IGS.hmin / fabs(IGS.h);
                            rh = if d__1 >= 0.1f64 { d__1 } else { 0.1f64 };
                            IGS.h *= rh;
                            i = 1 as libc::c_int as libc::c_long;
                            while i <= IGS.n {
                                *y.offset(i as isize) = *yh.offset((i + yh_dim1) as isize);
                                i += 1;
                                i;
                            }
                            CalcDeriv(
                                &mut *y.offset(1 as libc::c_int as isize),
                                &mut *savf.offset(1 as libc::c_int as isize),
                                &mut IGS.tn,
                            );
                            IGS.nfe += 1;
                            IGS.nfe;
                            i = 1 as libc::c_int as libc::c_long;
                            while i <= IGS.n {
                                *yh
                                    .offset(
                                        (i + (yh_dim1 << 1 as libc::c_int)) as isize,
                                    ) = IGS.h * *savf.offset(i as isize);
                                i += 1;
                                i;
                            }
                            IGS.ipup = IGS.miter;
                            IGS.ialth = 5 as libc::c_int as libc::c_long;
                            if IGS.nq == 1 as libc::c_int as libc::c_long {
                                current_block = 17586091195355511184;
                                continue;
                            }
                            IGS.nq = 1 as libc::c_int as libc::c_long;
                            IGS.l = 2 as libc::c_int as libc::c_long;
                            iret = 3 as libc::c_int as libc::c_long;
                            current_block = 10139636376225292192;
                            continue;
                        } else {
                            iredo = 2 as libc::c_int as libc::c_long;
                            rhup = 0.0f64;
                        }
                    }
                    _ => {
                        IGS.kflag = 0 as libc::c_int as libc::c_long;
                        iredo = 0 as libc::c_int as libc::c_long;
                        IGS.nst += 1;
                        IGS.nst;
                        IGS.hu = IGS.h;
                        IGS.nqu = IGS.nq;
                        j = 1 as libc::c_int as libc::c_long;
                        while j <= IGS.l {
                            i = 1 as libc::c_int as libc::c_long;
                            while i <= IGS.n {
                                *yh.offset((i + j * yh_dim1) as isize)
                                    += IGS.el[(j - 1 as libc::c_int as libc::c_long) as usize]
                                        * *acor.offset(i as isize);
                                i += 1;
                                i;
                            }
                            j += 1;
                            j;
                        }
                        IGS.ialth -= 1;
                        IGS.ialth;
                        if IGS.ialth == 0 as libc::c_int as libc::c_long {
                            rhup = 0.0f64;
                            if !(IGS.l == IGS.lmax) {
                                i__1 = IGS.n;
                                i = 1 as libc::c_int as libc::c_long;
                                while i <= i__1 {
                                    *savf
                                        .offset(
                                            i as isize,
                                        ) = *acor.offset(i as isize)
                                        - *yh.offset((i + IGS.lmax * yh_dim1) as isize);
                                    i += 1;
                                    i;
                                }
                                dup = vnorm_(
                                    &mut IGS.n,
                                    &mut *savf.offset(1 as libc::c_int as isize),
                                    &mut *ewt.offset(1 as libc::c_int as isize),
                                )
                                    / IGS
                                        .tesco[(IGS.nq * 3 as libc::c_int as libc::c_long
                                        - 1 as libc::c_int as libc::c_long) as usize];
                                exup = 1.0f64
                                    / (IGS.l + 1 as libc::c_int as libc::c_long)
                                        as libc::c_double;
                                rhup = 1.0f64 / (pow(dup, exup) * 1.4f64 + 1.4e-6f64);
                            }
                        } else if IGS.ialth > 1 as libc::c_int as libc::c_long {
                            current_block = 11681593226181908859;
                            break;
                        } else {
                            current_block = 8466485602140941539;
                            break;
                        }
                    }
                }
                exsm = 1.0f64 / IGS.l as libc::c_double;
                rhsm = 1.0f64 / (pow(dsm, exsm) * 1.2f64 + 1.2e-6f64);
                rhdn = 0.0f64;
                if !(IGS.nq == 1 as libc::c_int as libc::c_long) {
                    ddn = vnorm_(
                        &mut IGS.n,
                        &mut *yh
                            .offset(
                                (IGS.l * yh_dim1 + 1 as libc::c_int as libc::c_long)
                                    as isize,
                            ),
                        &mut *ewt.offset(1 as libc::c_int as isize),
                    )
                        / IGS
                            .tesco[(IGS.nq * 3 as libc::c_int as libc::c_long
                            - 3 as libc::c_int as libc::c_long) as usize];
                    exdn = 1.0f64 / IGS.nq as libc::c_double;
                    rhdn = 1.0f64 / (pow(ddn, exdn) * 1.3f64 + 1.3e-6f64);
                }
                if rhsm >= rhup {
                    if rhsm < rhdn {
                        current_block = 8445735333551943489;
                    } else {
                        newq = IGS.nq;
                        rh = rhsm;
                        current_block = 10098927251562842406;
                    }
                } else if rhup > rhdn {
                    newq = IGS.l;
                    rh = rhup;
                    if rh < 1.1f64 {
                        current_block = 8194941730993924067;
                        break;
                    }
                    r = IGS.el[(IGS.l - 1 as libc::c_int as libc::c_long) as usize]
                        / IGS.l as libc::c_double;
                    i__1 = IGS.n;
                    i = 1 as libc::c_int as libc::c_long;
                    while i <= i__1 {
                        *yh
                            .offset(
                                (i + (newq + 1 as libc::c_int as libc::c_long) * yh_dim1)
                                    as isize,
                            ) = *acor.offset(i as isize) * r;
                        i += 1;
                        i;
                    }
                    current_block = 11074306488306276270;
                } else {
                    current_block = 8445735333551943489;
                }
                match current_block {
                    8445735333551943489 => {
                        newq = IGS.nq - 1 as libc::c_int as libc::c_long;
                        rh = rhdn;
                        if IGS.kflag < 0 as libc::c_int as libc::c_long && rh > 1.0f64 {
                            rh = 1.0f64;
                        }
                        current_block = 10098927251562842406;
                    }
                    _ => {}
                }
                match current_block {
                    10098927251562842406 => {
                        if IGS.kflag == 0 as libc::c_int as libc::c_long && rh < 1.1f64 {
                            current_block = 8194941730993924067;
                            break;
                        }
                        if IGS.kflag <= -(2 as libc::c_int) as libc::c_long {
                            rh = if rh <= 0.2f64 { rh } else { 0.2f64 };
                        }
                        if newq == IGS.nq {
                            current_block = 3642853594056902449;
                            continue;
                        }
                    }
                    _ => {}
                }
                IGS.nq = newq;
                IGS.l = IGS.nq + 1 as libc::c_int as libc::c_long;
                iret = 2 as libc::c_int as libc::c_long;
                current_block = 10139636376225292192;
            }
        }
    }
    match current_block {
        8466485602140941539 => {
            if IGS.l == IGS.lmax {
                current_block = 11681593226181908859;
            } else {
                i = 1 as libc::c_int as libc::c_long;
                while i <= IGS.n {
                    *yh
                        .offset(
                            (i + IGS.lmax * yh_dim1) as isize,
                        ) = *acor.offset(i as isize);
                    i += 1;
                    i;
                }
                current_block = 11681593226181908859;
            }
        }
        15578571638613445014 => {
            IGS.kflag = -(1 as libc::c_int) as libc::c_long;
            current_block = 11802380472089656092;
        }
        8194941730993924067 => {
            IGS.ialth = 3 as libc::c_int as libc::c_long;
            current_block = 11681593226181908859;
        }
        14836854278375750052 => {
            IGS.kflag = -(2 as libc::c_int) as libc::c_long;
            current_block = 11802380472089656092;
        }
        _ => {}
    }
    match current_block {
        11681593226181908859 => {
            r = 1.0f64
                / IGS
                    .tesco[(IGS.nqu * 3 as libc::c_int as libc::c_long
                    - 2 as libc::c_int as libc::c_long) as usize];
            i = 1 as libc::c_int as libc::c_long;
            while i <= IGS.n {
                *acor.offset(i as isize) *= r;
                i += 1;
                i;
            }
        }
        _ => {}
    }
    IGS.hold = IGS.h;
    IGS.jstart = 1 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn vnorm_(
    mut n: *mut libc::c_long,
    mut v: *mut libc::c_double,
    mut w: *mut libc::c_double,
) -> libc::c_double {
    let mut d__1: libc::c_double = 0.;
    let mut i: libc::c_long = 0;
    let mut sum: libc::c_double = 0.;
    sum = 0.0f64;
    i = 0 as libc::c_int as libc::c_long;
    while i < *n {
        d__1 = *v.offset(i as isize) * *w.offset(i as isize);
        sum += d__1 * d__1;
        i += 1;
        i;
    }
    return sqrt(sum / *n as libc::c_double);
}
pub unsafe extern "C" fn cfode_(
    mut meth: *mut libc::c_long,
    mut elco: *mut libc::c_double,
    mut tesco: *mut libc::c_double,
) -> libc::c_int {
    let mut ragq: libc::c_double = 0.;
    let mut pint: libc::c_double = 0.;
    let mut xpin: libc::c_double = 0.;
    let mut fnqm1: libc::c_double = 0.;
    let mut i: libc::c_long = 0;
    let mut agamq: libc::c_double = 0.;
    let mut rqfac: libc::c_double = 0.;
    let mut tsign: libc::c_double = 0.;
    let mut rq1fac: libc::c_double = 0.;
    let mut ib: libc::c_long = 0;
    let mut pc: [libc::c_double; 12] = [0.; 12];
    let mut nq: libc::c_long = 0;
    let mut fnq: libc::c_double = 0.;
    let mut nqm1: libc::c_long = 0;
    let mut nqp1: libc::c_long = 0;
    tesco = tesco.offset(-(4 as libc::c_int as isize));
    elco = elco.offset(-(14 as libc::c_int as isize));
    match *meth {
        2 => {
            pc[0 as libc::c_int as usize] = 1.0f64;
            rq1fac = 1.0f64;
            nq = 1 as libc::c_int as libc::c_long;
            while nq <= 5 as libc::c_int as libc::c_long {
                fnq = nq as libc::c_double;
                nqp1 = nq + 1 as libc::c_int as libc::c_long;
                pc[(nqp1 - 1 as libc::c_int as libc::c_long) as usize] = 0.0f64;
                ib = 1 as libc::c_int as libc::c_long;
                while ib <= nq {
                    i = nq + 2 as libc::c_int as libc::c_long - ib;
                    pc[(i - 1 as libc::c_int as libc::c_long)
                        as usize] = pc[(i - 2 as libc::c_int as libc::c_long) as usize]
                        + fnq * pc[(i - 1 as libc::c_int as libc::c_long) as usize];
                    ib += 1;
                    ib;
                }
                pc[0 as libc::c_int as usize] = fnq * pc[0 as libc::c_int as usize];
                i = 1 as libc::c_int as libc::c_long;
                while i <= nqp1 {
                    *elco
                        .offset(
                            (i + nq * 13 as libc::c_int as libc::c_long) as isize,
                        ) = pc[(i - 1 as libc::c_int as libc::c_long) as usize]
                        / pc[1 as libc::c_int as usize];
                    i += 1;
                    i;
                }
                *elco
                    .offset(
                        (nq * 13 as libc::c_int as libc::c_long
                            + 2 as libc::c_int as libc::c_long) as isize,
                    ) = 1.0f64;
                *tesco
                    .offset(
                        (nq * 3 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as isize,
                    ) = rq1fac;
                *tesco
                    .offset(
                        (nq * 3 as libc::c_int as libc::c_long
                            + 2 as libc::c_int as libc::c_long) as isize,
                    ) = nqp1 as libc::c_double
                    / *elco
                        .offset(
                            (nq * 13 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as isize,
                        );
                *tesco
                    .offset(
                        (nq * 3 as libc::c_int as libc::c_long
                            + 3 as libc::c_int as libc::c_long) as isize,
                    ) = (nq + 2 as libc::c_int as libc::c_long) as libc::c_double
                    / *elco
                        .offset(
                            (nq * 13 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as isize,
                        );
                rq1fac /= fnq;
                nq += 1;
                nq;
            }
            return 0 as libc::c_int;
        }
        1 | _ => {
            *elco.offset(14 as libc::c_int as isize) = 1.0f64;
            *elco.offset(15 as libc::c_int as isize) = 1.0f64;
            *tesco.offset(4 as libc::c_int as isize) = 0.0f64;
            *tesco.offset(5 as libc::c_int as isize) = 2.0f64;
            *tesco.offset(7 as libc::c_int as isize) = 1.0f64;
            *tesco.offset(39 as libc::c_int as isize) = 0.0f64;
            pc[0 as libc::c_int as usize] = 1.0f64;
            rqfac = 1.0f64;
            nq = 2 as libc::c_int as libc::c_long;
            while nq <= 12 as libc::c_int as libc::c_long {
                rq1fac = rqfac;
                rqfac /= nq as libc::c_double;
                nqm1 = nq - 1 as libc::c_int as libc::c_long;
                fnqm1 = nqm1 as libc::c_double;
                nqp1 = nq + 1 as libc::c_int as libc::c_long;
                pc[(nq - 1 as libc::c_int as libc::c_long) as usize] = 0.0f64;
                ib = 1 as libc::c_int as libc::c_long;
                while ib <= nqm1 {
                    i = nqp1 - ib;
                    pc[(i - 1 as libc::c_int as libc::c_long)
                        as usize] = pc[(i - 2 as libc::c_int as libc::c_long) as usize]
                        + fnqm1 * pc[(i - 1 as libc::c_int as libc::c_long) as usize];
                    ib += 1;
                    ib;
                }
                pc[0 as libc::c_int as usize] = fnqm1 * pc[0 as libc::c_int as usize];
                pint = pc[0 as libc::c_int as usize];
                xpin = pc[0 as libc::c_int as usize] / 2.0f64;
                tsign = 1.0f64;
                i = 2 as libc::c_int as libc::c_long;
                while i <= nq {
                    tsign = -tsign;
                    pint
                        += tsign * pc[(i - 1 as libc::c_int as libc::c_long) as usize]
                            / i as libc::c_double;
                    xpin
                        += tsign * pc[(i - 1 as libc::c_int as libc::c_long) as usize]
                            / (i + 1 as libc::c_int as libc::c_long) as libc::c_double;
                    i += 1;
                    i;
                }
                *elco
                    .offset(
                        (nq * 13 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as isize,
                    ) = pint * rq1fac;
                *elco
                    .offset(
                        (nq * 13 as libc::c_int as libc::c_long
                            + 2 as libc::c_int as libc::c_long) as isize,
                    ) = 1.0f64;
                i = 2 as libc::c_int as libc::c_long;
                while i <= nq {
                    *elco
                        .offset(
                            (i + 1 as libc::c_int as libc::c_long
                                + nq * 13 as libc::c_int as libc::c_long) as isize,
                        ) = rq1fac * pc[(i - 1 as libc::c_int as libc::c_long) as usize]
                        / i as libc::c_double;
                    i += 1;
                    i;
                }
                agamq = rqfac * xpin;
                ragq = 1.0f64 / agamq;
                *tesco
                    .offset(
                        (nq * 3 as libc::c_int as libc::c_long
                            + 2 as libc::c_int as libc::c_long) as isize,
                    ) = ragq;
                if nq < 12 as libc::c_int as libc::c_long {
                    *tesco
                        .offset(
                            (nqp1 * 3 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as isize,
                        ) = ragq * rqfac / nqp1 as libc::c_double;
                }
                *tesco
                    .offset(
                        (nqm1 * 3 as libc::c_int as libc::c_long
                            + 3 as libc::c_int as libc::c_long) as isize,
                    ) = ragq;
                nq += 1;
                nq;
            }
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn intdy_(
    mut t: *mut libc::c_double,
    mut k: libc::c_long,
    mut yh: *mut libc::c_double,
    mut nyh: *mut libc::c_long,
    mut dky: *mut libc::c_double,
    mut iflag: *mut libc::c_long,
) -> libc::c_int {
    let mut yh_dim1: libc::c_long = 0;
    let mut yh_offset: libc::c_long = 0;
    let mut i__1: libc::c_long = 0;
    let mut c: libc::c_double = 0.;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut r: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut ic: libc::c_long = 0;
    let mut jb: libc::c_long = 0;
    let mut jj: libc::c_long = 0;
    let mut tp: libc::c_double = 0.;
    let mut jb2: libc::c_long = 0;
    let mut jj1: libc::c_long = 0;
    let mut jp1: libc::c_long = 0;
    dky = dky.offset(-1);
    dky;
    yh_dim1 = *nyh;
    yh_offset = yh_dim1 + 1 as libc::c_int as libc::c_long;
    yh = yh.offset(-(yh_offset as isize));
    *iflag = 0 as libc::c_int as libc::c_long;
    if k < 0 as libc::c_int as libc::c_long || k > IGS.nq {
        printf(b"intdy: k = %ld is illegal.\n\0" as *const u8 as *const libc::c_char, k);
        *iflag = -(1 as libc::c_int) as libc::c_long;
        return 0 as libc::c_int;
    } else {
        tp = IGS.tn - IGS.hu - IGS.uround * 100.0f64 * (IGS.tn + IGS.hu);
        if (*t - tp) * (*t - IGS.tn) > 0.0f64 {
            printf(
                b"intdy: t = %21.13f is illegal, it is not in the interval\0"
                    as *const u8 as *const libc::c_char,
                *t,
            );
            printf(
                b"       tcur - hu (= %21.13f) to tcur (= %21.13f).\n\0" as *const u8
                    as *const libc::c_char,
                tp,
                IGS.tn,
            );
            *iflag = -(2 as libc::c_int) as libc::c_long;
            return 0 as libc::c_int;
        } else {
            s = (*t - IGS.tn) / IGS.h;
            ic = 1 as libc::c_int as libc::c_long;
            if !(k == 0 as libc::c_int as libc::c_long) {
                jj1 = IGS.l - k;
                i__1 = IGS.nq;
                jj = jj1;
                while jj <= i__1 {
                    ic *= jj;
                    jj += 1;
                    jj;
                }
            }
            c = ic as libc::c_double;
            i__1 = IGS.n;
            i = 1 as libc::c_int as libc::c_long;
            while i <= i__1 {
                *dky.offset(i as isize) = c * *yh.offset((i + IGS.l * yh_dim1) as isize);
                i += 1;
                i;
            }
            if !(k == IGS.nq) {
                jb2 = IGS.nq - k;
                jb = 1 as libc::c_int as libc::c_long;
                while jb <= jb2 {
                    j = IGS.nq - jb;
                    jp1 = j + 1 as libc::c_int as libc::c_long;
                    ic = 1 as libc::c_int as libc::c_long;
                    if k != 0 as libc::c_int as libc::c_long {
                        jj1 = jp1 - k;
                        jj = jj1;
                        while jj <= j {
                            ic *= jj;
                            jj += 1;
                            jj;
                        }
                    }
                    c = ic as libc::c_double;
                    i = 1 as libc::c_int as libc::c_long;
                    while i <= IGS.n {
                        *dky
                            .offset(
                                i as isize,
                            ) = c * *yh.offset((i + jp1 * yh_dim1) as isize)
                            + s * *dky.offset(i as isize);
                        i += 1;
                        i;
                    }
                    jb += 1;
                    jb;
                }
                if k == 0 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int;
                }
            }
            r = pow(IGS.h, -k as libc::c_double);
            i = 1 as libc::c_int as libc::c_long;
            while i <= IGS.n {
                *dky.offset(i as isize) = r * *dky.offset(i as isize);
                i += 1;
                i;
            }
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn iprep_(
    mut neq: *mut libc::c_long,
    mut y: *mut libc::c_double,
    mut rwork: *mut libc::c_double,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut ipflag: *mut libc::c_long,
) -> libc::c_int {
    let mut i__1: libc::c_long = 0;
    let mut imax: libc::c_long = 0;
    let mut lyhd: libc::c_long = 0;
    let mut lyhn: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut lewtn: libc::c_long = 0;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    rwork = rwork.offset(-1);
    rwork;
    y = y.offset(-1);
    y;
    neq = neq.offset(-1);
    neq;
    *ipflag = 0 as libc::c_int as libc::c_long;
    prep_(
        &mut *neq.offset(1 as libc::c_int as isize),
        &mut *y.offset(1 as libc::c_int as isize),
        &mut *rwork.offset(IGS.lyh as isize),
        &mut *rwork.offset(IGS.lsavf as isize),
        &mut *rwork.offset(IGS.lewt as isize),
        &mut *rwork.offset(IGS.lacor as isize),
        &mut *ia.offset(1 as libc::c_int as isize),
        &mut *ja.offset(1 as libc::c_int as isize),
        &mut *rwork.offset(IGS.lwm as isize),
        &mut *rwork.offset(IGS.lwm as isize) as *mut libc::c_double as *mut libc::c_long,
        ipflag,
    );
    IGS.lenwk = if IGS.lreq >= IGS.lwmin { IGS.lreq } else { IGS.lwmin };
    if *ipflag < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    lyhn = IGS.lwm + IGS.lenwk;
    if lyhn > IGS.lyh {
        return 0 as libc::c_int;
    }
    lyhd = IGS.lyh - lyhn;
    if !(lyhd == 0 as libc::c_int as libc::c_long) {
        imax = lyhn - 1 as libc::c_int as libc::c_long + IGS.lenyhm;
        i__1 = imax;
        i = lyhn;
        while i <= i__1 {
            *rwork.offset(i as isize) = *rwork.offset((i + lyhd) as isize);
            i += 1;
            i;
        }
        IGS.lyh = lyhn;
    }
    IGS.lsavf = IGS.lyh + IGS.lenyh;
    lewtn = IGS.lsavf + IGS.n;
    IGS.lacor = lewtn + IGS.n;
    if !(IGS.istatc == 3 as libc::c_int as libc::c_long) {
        if lewtn > IGS.lewt {
            return 0 as libc::c_int;
        }
        i__1 = IGS.n;
        i = 1 as libc::c_int as libc::c_long;
        while i <= i__1 {
            *rwork
                .offset(
                    (i + lewtn - 1 as libc::c_int as libc::c_long) as isize,
                ) = *rwork
                .offset((i + IGS.lewt - 1 as libc::c_int as libc::c_long) as isize);
            i += 1;
            i;
        }
    }
    IGS.lewt = lewtn;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn prep_(
    mut neq: *mut libc::c_long,
    mut y: *mut libc::c_double,
    mut yh: *mut libc::c_double,
    mut savf: *mut libc::c_double,
    mut ewt: *mut libc::c_double,
    mut ftem: *mut libc::c_double,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut wk: *mut libc::c_double,
    mut iwk: *mut libc::c_long,
    mut ipper: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__1: libc::c_long = 0;
    let mut i__2: libc::c_long = 0;
    let mut ldif: libc::c_long = 0;
    let mut ipil: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut ipiu: libc::c_long = 0;
    let mut kmax: libc::c_long = 0;
    let mut kmin: libc::c_long = 0;
    let mut liwk: libc::c_long = 0;
    let mut maxg: libc::c_long = 0;
    let mut erwt: libc::c_double = 0.;
    let mut iptt1: libc::c_long = 0;
    let mut iptt2: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut nzsut: libc::c_long = 0;
    let mut dq: libc::c_double = 0.;
    let mut yj: libc::c_double = 0.;
    let mut lenigp: libc::c_long = 0;
    let mut jfound: libc::c_long = 0;
    let mut np1: libc::c_long = 0;
    let mut fac: libc::c_double = 0.;
    let mut ibr: libc::c_long = 0;
    let mut ier: libc::c_long = 0;
    let mut dyj: libc::c_double = 0.;
    iwk = iwk.offset(-1);
    iwk;
    wk = wk.offset(-1);
    wk;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    ftem = ftem.offset(-1);
    ftem;
    ewt = ewt.offset(-1);
    ewt;
    savf = savf.offset(-1);
    savf;
    yh = yh.offset(-1);
    yh;
    y = y.offset(-1);
    y;
    neq = neq.offset(-1);
    neq;
    IGS.ibian = IGS.lrat << 1 as libc::c_int;
    IGS.ipian = IGS.ibian + 1 as libc::c_int as libc::c_long;
    np1 = IGS.n + 1 as libc::c_int as libc::c_long;
    IGS.ipjan = IGS.ipian + np1;
    IGS.ibjan = IGS.ipjan - 1 as libc::c_int as libc::c_long;
    liwk = IGS.lenwk * IGS.lrat;
    if !(IGS.ipjan + IGS.n - 1 as libc::c_int as libc::c_long > liwk) {
        if IGS.moss == 0 as libc::c_int as libc::c_long {
            current_block = 6150346664290372554;
        } else {
            if IGS.istatc == 3 as libc::c_int as libc::c_long {
                current_block = 8670819487267933155;
            } else {
                i__1 = IGS.n;
                i = 1 as libc::c_int as libc::c_long;
                while i <= i__1 {
                    erwt = 1.0f64 / *ewt.offset(i as isize);
                    fac = 1.0f64 / (i as libc::c_double + 1.0f64) + 1.0f64;
                    *y.offset(i as isize)
                        += fac * d_sign(&mut erwt, &mut *y.offset(i as isize));
                    i += 1;
                    i;
                }
                match IGS.moss {
                    1 => {
                        current_block = 11602090681525859000;
                    }
                    2 => {
                        current_block = 867404828073960170;
                    }
                    _ => {
                        current_block = 8670819487267933155;
                    }
                }
            }
            match current_block {
                8670819487267933155 => {
                    i = 1 as libc::c_int as libc::c_long;
                    while i <= IGS.n {
                        *y.offset(i as isize) = *yh.offset(i as isize);
                        i += 1;
                        i;
                    }
                    match IGS.moss {
                        1 => {
                            current_block = 11602090681525859000;
                        }
                        2 => {
                            current_block = 867404828073960170;
                        }
                        _ => {
                            current_block = 6150346664290372554;
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                6150346664290372554 => {}
                _ => {
                    match current_block {
                        11602090681525859000 => {
                            CalcDeriv(
                                &mut *y.offset(1 as libc::c_int as isize),
                                &mut *savf.offset(1 as libc::c_int as isize),
                                &mut IGS.tn,
                            );
                            k = IGS.ipjan;
                            *iwk
                                .offset(
                                    IGS.ipian as isize,
                                ) = 1 as libc::c_int as libc::c_long;
                            i__1 = IGS.n;
                            j = 1 as libc::c_int as libc::c_long;
                            's_273: loop {
                                if !(j <= i__1) {
                                    current_block = 5622442387111943004;
                                    break;
                                }
                                if k > liwk {
                                    current_block = 7742784732132560523;
                                    break;
                                }
                                *iwk.offset(k as isize) = j;
                                k += 1;
                                k;
                                i = 1 as libc::c_int as libc::c_long;
                                while i <= IGS.n {
                                    *savf.offset(i as isize) = 0.0f64;
                                    i += 1;
                                    i;
                                }
                                CalcJacob(
                                    &mut IGS.tn,
                                    &mut *y.offset(1 as libc::c_int as isize),
                                    j,
                                    &mut *savf.offset(1 as libc::c_int as isize),
                                );
                                i__2 = IGS.n;
                                i = 1 as libc::c_int as libc::c_long;
                                while i <= i__2 {
                                    if !(fabs(*savf.offset(i as isize)) <= IGS.seth) {
                                        if !(i == j) {
                                            if k > liwk {
                                                current_block = 7742784732132560523;
                                                break 's_273;
                                            }
                                            *iwk.offset(k as isize) = i;
                                            k += 1;
                                            k;
                                        }
                                    }
                                    i += 1;
                                    i;
                                }
                                *iwk
                                    .offset(
                                        (IGS.ipian + j) as isize,
                                    ) = k + 1 as libc::c_int as libc::c_long - IGS.ipjan;
                                j += 1;
                                j;
                            }
                        }
                        _ => {
                            k = IGS.ipjan;
                            *iwk
                                .offset(
                                    IGS.ipian as isize,
                                ) = 1 as libc::c_int as libc::c_long;
                            CalcDeriv(
                                &mut *y.offset(1 as libc::c_int as isize),
                                &mut *savf.offset(1 as libc::c_int as isize),
                                &mut IGS.tn,
                            );
                            i__1 = IGS.n;
                            j = 1 as libc::c_int as libc::c_long;
                            's_375: loop {
                                if !(j <= i__1) {
                                    current_block = 5622442387111943004;
                                    break;
                                }
                                if k > liwk {
                                    current_block = 7742784732132560523;
                                    break;
                                }
                                *iwk.offset(k as isize) = j;
                                k += 1;
                                k;
                                yj = *y.offset(j as isize);
                                erwt = 1.0f64 / *ewt.offset(j as isize);
                                dyj = d_sign(&mut erwt, &mut yj);
                                *y.offset(j as isize) = yj + dyj;
                                CalcDeriv(
                                    &mut *y.offset(1 as libc::c_int as isize),
                                    &mut *ftem.offset(1 as libc::c_int as isize),
                                    &mut IGS.tn,
                                );
                                *y.offset(j as isize) = yj;
                                i__2 = IGS.n;
                                i = 1 as libc::c_int as libc::c_long;
                                while i <= i__2 {
                                    dq = (*ftem.offset(i as isize) - *savf.offset(i as isize))
                                        / dyj;
                                    if !(fabs(dq) <= IGS.seth) {
                                        if !(i == j) {
                                            if k > liwk {
                                                current_block = 7742784732132560523;
                                                break 's_375;
                                            }
                                            *iwk.offset(k as isize) = i;
                                            k += 1;
                                            k;
                                        }
                                    }
                                    i += 1;
                                    i;
                                }
                                *iwk
                                    .offset(
                                        (IGS.ipian + j) as isize,
                                    ) = k + 1 as libc::c_int as libc::c_long - IGS.ipjan;
                                j += 1;
                                j;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            7742784732132560523 => {}
            _ => {
                match current_block {
                    6150346664290372554 => {
                        knew = IGS.ipjan;
                        kmin = *ia.offset(1 as libc::c_int as isize);
                        *iwk
                            .offset(
                                IGS.ipian as isize,
                            ) = 1 as libc::c_int as libc::c_long;
                        i__1 = IGS.n;
                        j = 1 as libc::c_int as libc::c_long;
                        's_169: loop {
                            if !(j <= i__1) {
                                current_block = 5622442387111943004;
                                break;
                            }
                            jfound = 0 as libc::c_int as libc::c_long;
                            kmax = *ia
                                .offset((j + 1 as libc::c_int as libc::c_long) as isize)
                                - 1 as libc::c_int as libc::c_long;
                            if kmin > kmax {
                                current_block = 3274468813261518059;
                            } else {
                                i__2 = kmax;
                                k = kmin;
                                while k <= i__2 {
                                    i = *ja.offset(k as isize);
                                    if i == j {
                                        jfound = 1 as libc::c_int as libc::c_long;
                                    }
                                    if knew > liwk {
                                        current_block = 7742784732132560523;
                                        break 's_169;
                                    }
                                    *iwk.offset(knew as isize) = i;
                                    knew += 1;
                                    knew;
                                    k += 1;
                                    k;
                                }
                                if jfound == 1 as libc::c_int as libc::c_long {
                                    current_block = 5797527028290667957;
                                } else {
                                    current_block = 3274468813261518059;
                                }
                            }
                            match current_block {
                                3274468813261518059 => {
                                    if knew > liwk {
                                        current_block = 7742784732132560523;
                                        break;
                                    }
                                    *iwk.offset(knew as isize) = j;
                                    knew += 1;
                                    knew;
                                }
                                _ => {}
                            }
                            *iwk
                                .offset(
                                    (IGS.ipian + j) as isize,
                                ) = knew + 1 as libc::c_int as libc::c_long - IGS.ipjan;
                            kmin = kmax + 1 as libc::c_int as libc::c_long;
                            j += 1;
                            j;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    7742784732132560523 => {}
                    _ => {
                        if !(IGS.moss == 0 as libc::c_int as libc::c_long
                            || IGS.istatc != 1 as libc::c_int as libc::c_long)
                        {
                            i = 1 as libc::c_int as libc::c_long;
                            while i <= IGS.n {
                                *y.offset(i as isize) = *yh.offset(i as isize);
                                i += 1;
                                i;
                            }
                        }
                        IGS
                            .nnz = *iwk.offset((IGS.ipian + IGS.n) as isize)
                            - 1 as libc::c_int as libc::c_long;
                        lenigp = 0 as libc::c_int as libc::c_long;
                        IGS.ipigp = IGS.ipjan + IGS.nnz;
                        if !(IGS.miter != 2 as libc::c_int as libc::c_long) {
                            maxg = np1;
                            IGS.ipjgp = IGS.ipjan + IGS.nnz;
                            IGS.ibjgp = IGS.ipjgp - 1 as libc::c_int as libc::c_long;
                            IGS.ipigp = IGS.ipjgp + IGS.n;
                            iptt1 = IGS.ipigp + np1;
                            iptt2 = iptt1 + IGS.n;
                            IGS.lreq = iptt2 + IGS.n - 1 as libc::c_int as libc::c_long;
                            if IGS.lreq > liwk {
                                current_block = 8206453749173344948;
                            } else {
                                jgroup_(
                                    &mut IGS.n,
                                    &mut *iwk.offset(IGS.ipian as isize),
                                    &mut *iwk.offset(IGS.ipjan as isize),
                                    &mut maxg,
                                    &mut IGS.ngp,
                                    &mut *iwk.offset(IGS.ipigp as isize),
                                    &mut *iwk.offset(IGS.ipjgp as isize),
                                    &mut *iwk.offset(iptt1 as isize),
                                    &mut *iwk.offset(iptt2 as isize),
                                    &mut ier,
                                );
                                if ier != 0 as libc::c_int as libc::c_long {
                                    current_block = 8206453749173344948;
                                } else {
                                    lenigp = IGS.ngp + 1 as libc::c_int as libc::c_long;
                                    current_block = 4973706409159467350;
                                }
                            }
                            match current_block {
                                4973706409159467350 => {}
                                _ => {
                                    *ipper = -(2 as libc::c_int) as libc::c_long;
                                    IGS
                                        .lreq = (IGS.lreq - 1 as libc::c_int as libc::c_long)
                                        / IGS.lrat + 1 as libc::c_int as libc::c_long;
                                    return 0 as libc::c_int;
                                }
                            }
                        }
                        IGS.ipr = IGS.ipigp + lenigp;
                        IGS.ipc = IGS.ipr;
                        IGS.ipic = IGS.ipc + IGS.n;
                        IGS.ipisp = IGS.ipic + IGS.n;
                        IGS
                            .iprsp = (IGS.ipisp - 2 as libc::c_int as libc::c_long)
                            / IGS.lrat + 2 as libc::c_int as libc::c_long;
                        IGS
                            .iesp = IGS.lenwk + 1 as libc::c_int as libc::c_long
                            - IGS.iprsp;
                        if !(IGS.iesp < 0 as libc::c_int as libc::c_long) {
                            ibr = IGS.ipr - 1 as libc::c_int as libc::c_long;
                            i = 1 as libc::c_int as libc::c_long;
                            while i <= IGS.n {
                                *iwk.offset((ibr + i) as isize) = i;
                                i += 1;
                                i;
                            }
                            IGS
                                .nsp = liwk + 1 as libc::c_int as libc::c_long - IGS.ipisp;
                            odrv_(
                                &mut IGS.n,
                                &mut *iwk.offset(IGS.ipian as isize),
                                &mut *iwk.offset(IGS.ipjan as isize),
                                &mut *wk.offset(1 as libc::c_int as isize),
                                &mut *iwk.offset(IGS.ipr as isize),
                                &mut *iwk.offset(IGS.ipic as isize),
                                &mut IGS.nsp,
                                &mut *iwk.offset(IGS.ipisp as isize),
                                1 as libc::c_int as libc::c_long,
                                &mut IGS.iys,
                            );
                            if IGS.iys
                                == IGS.n * 11 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            {
                                *ipper = -(4 as libc::c_int) as libc::c_long;
                                return 0 as libc::c_int;
                            } else if !(IGS.iys != 0 as libc::c_int as libc::c_long) {
                                IGS
                                    .ipa = IGS.lenwk + 1 as libc::c_int as libc::c_long
                                    - IGS.nnz;
                                IGS.nsp = IGS.ipa - IGS.iprsp;
                                i__1 = IGS.n * 12 as libc::c_int as libc::c_long / IGS.lrat;
                                i__2 = IGS.n * 6 as libc::c_int as libc::c_long / IGS.lrat
                                    + (IGS.n << 1 as libc::c_int) + IGS.nnz;
                                IGS
                                    .lreq = (if i__1 >= i__2 { i__1 } else { i__2 })
                                    + 3 as libc::c_int as libc::c_long;
                                IGS
                                    .lreq = IGS.lreq + IGS.iprsp
                                    - 1 as libc::c_int as libc::c_long + IGS.nnz;
                                if !(IGS.lreq > IGS.lenwk) {
                                    IGS.iba = IGS.ipa - 1 as libc::c_int as libc::c_long;
                                    i = 1 as libc::c_int as libc::c_long;
                                    while i <= IGS.nnz {
                                        *wk.offset((IGS.iba + i) as isize) = 0.0f64;
                                        i += 1;
                                        i;
                                    }
                                    IGS
                                        .ipisp = IGS.lrat
                                        * (IGS.iprsp - 1 as libc::c_int as libc::c_long)
                                        + 1 as libc::c_int as libc::c_long;
                                    cdrv_(
                                        &mut IGS.n,
                                        &mut *iwk.offset(IGS.ipr as isize),
                                        &mut *iwk.offset(IGS.ipc as isize),
                                        &mut *iwk.offset(IGS.ipic as isize),
                                        &mut *iwk.offset(IGS.ipian as isize),
                                        &mut *iwk.offset(IGS.ipjan as isize),
                                        &mut *wk.offset(IGS.ipa as isize),
                                        &mut *wk.offset(IGS.ipa as isize),
                                        &mut *wk.offset(IGS.ipa as isize),
                                        &mut IGS.nsp,
                                        &mut *iwk.offset(IGS.ipisp as isize),
                                        &mut *wk.offset(IGS.iprsp as isize),
                                        &mut IGS.iesp,
                                        5 as libc::c_int as libc::c_long,
                                        &mut IGS.iys,
                                    );
                                    IGS.lreq = IGS.lenwk - IGS.iesp;
                                    if !(IGS.iys
                                        == IGS.n * 10 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long)
                                    {
                                        if IGS.iys != 0 as libc::c_int as libc::c_long {
                                            *ipper = -(6 as libc::c_int) as libc::c_long;
                                            IGS.lreq = IGS.lenwk;
                                            return 0 as libc::c_int;
                                        } else {
                                            ipil = IGS.ipisp;
                                            ipiu = ipil + (IGS.n << 1 as libc::c_int)
                                                + 1 as libc::c_int as libc::c_long;
                                            IGS
                                                .nzu = *iwk.offset((ipil + IGS.n) as isize)
                                                - *iwk.offset(ipil as isize);
                                            IGS
                                                .nzl = *iwk.offset((ipiu + IGS.n) as isize)
                                                - *iwk.offset(ipiu as isize);
                                            if !(IGS.lrat > 1 as libc::c_int as libc::c_long) {
                                                adjlr_(
                                                    &mut IGS.n,
                                                    &mut *iwk.offset(IGS.ipisp as isize),
                                                    &mut ldif,
                                                );
                                                IGS.lreq += ldif;
                                            }
                                            if IGS.lrat == 2 as libc::c_int as libc::c_long
                                                && IGS.nnz == IGS.n
                                            {
                                                IGS.lreq += 1;
                                                IGS.lreq;
                                            }
                                            IGS.nsp = IGS.nsp + IGS.lreq - IGS.lenwk;
                                            IGS
                                                .ipa = IGS.lreq + 1 as libc::c_int as libc::c_long
                                                - IGS.nnz;
                                            IGS.iba = IGS.ipa - 1 as libc::c_int as libc::c_long;
                                            *ipper = 0 as libc::c_int as libc::c_long;
                                            return 0 as libc::c_int;
                                        }
                                    }
                                }
                                *ipper = -(5 as libc::c_int) as libc::c_long;
                                return 0 as libc::c_int;
                            }
                        }
                        *ipper = -(3 as libc::c_int) as libc::c_long;
                        cntnzu_(
                            &mut IGS.n,
                            &mut *iwk.offset(IGS.ipian as isize),
                            &mut *iwk.offset(IGS.ipjan as isize),
                            &mut nzsut,
                        );
                        IGS
                            .lreq = IGS.lenwk - IGS.iesp
                            + (IGS.n * 3 as libc::c_int as libc::c_long
                                + (nzsut << 2 as libc::c_int)
                                - 1 as libc::c_int as libc::c_long) / IGS.lrat
                            + 1 as libc::c_int as libc::c_long;
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    *ipper = -(1 as libc::c_int) as libc::c_long;
    IGS
        .lreq = ((IGS.n << 1 as libc::c_int) + 1 as libc::c_int as libc::c_long)
        / IGS.lrat + 2 as libc::c_int as libc::c_long;
    i__1 = IGS.lenwk + 1 as libc::c_int as libc::c_long;
    IGS.lreq = if i__1 >= IGS.lreq { i__1 } else { IGS.lreq };
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cntnzu_(
    mut n: *mut libc::c_long,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut nzsut: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i__1: libc::c_long = 0;
    let mut i__2: libc::c_long = 0;
    let mut i__3: libc::c_long = 0;
    let mut jmin: libc::c_long = 0;
    let mut kmin: libc::c_long = 0;
    let mut jmax: libc::c_long = 0;
    let mut kmax: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut ii: libc::c_long = 0;
    let mut jj: libc::c_long = 0;
    let mut num: libc::c_long = 0;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    num = 0 as libc::c_int as libc::c_long;
    i__1 = *n;
    ii = 1 as libc::c_int as libc::c_long;
    while ii <= i__1 {
        jmin = *ia.offset(ii as isize);
        jmax = *ia.offset((ii + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        if !(jmin > jmax) {
            i__2 = jmax;
            j = jmin;
            while j <= i__2 {
                i__3 = *ja.offset(j as isize) - ii;
                if i__3 < 0 as libc::c_int as libc::c_long {
                    jj = *ja.offset(j as isize);
                    kmin = *ia.offset(jj as isize);
                    kmax = *ia.offset((jj + 1 as libc::c_int as libc::c_long) as isize)
                        - 1 as libc::c_int as libc::c_long;
                    if kmin > kmax {
                        current_block = 12875127467136764031;
                    } else {
                        i__3 = kmax;
                        k = kmin;
                        loop {
                            if !(k <= i__3) {
                                current_block = 12875127467136764031;
                                break;
                            }
                            if *ja.offset(k as isize) == ii {
                                current_block = 17216689946888361452;
                                break;
                            }
                            k += 1;
                            k;
                        }
                    }
                } else if i__3 == 0 as libc::c_int as libc::c_long {
                    current_block = 17216689946888361452;
                } else {
                    current_block = 12875127467136764031;
                }
                match current_block {
                    12875127467136764031 => {
                        num += 1;
                        num;
                    }
                    _ => {}
                }
                j += 1;
                j;
            }
        }
        ii += 1;
        ii;
    }
    *nzsut = num;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn adjlr_(
    mut n: *mut libc::c_long,
    mut isp: *mut libc::c_long,
    mut ldif: *mut libc::c_long,
) -> libc::c_int {
    let mut lnfc: libc::c_long = 0;
    let mut lsfc: libc::c_long = 0;
    let mut nzlu: libc::c_long = 0;
    let mut jlmax: libc::c_long = 0;
    let mut jumax: libc::c_long = 0;
    let mut ip: libc::c_long = 0;
    isp = isp.offset(-1);
    isp;
    ip = (*n << 1 as libc::c_int) + 1 as libc::c_int as libc::c_long;
    jlmax = *isp.offset(ip as isize);
    jumax = *isp.offset((ip + ip) as isize);
    nzlu = *isp.offset((*n + 1 as libc::c_int as libc::c_long) as isize)
        - *isp.offset(1 as libc::c_int as isize)
        + *isp.offset((ip + *n + 1 as libc::c_int as libc::c_long) as isize)
        - *isp.offset((ip + 1 as libc::c_int as libc::c_long) as isize);
    lsfc = *n * 12 as libc::c_int as libc::c_long + 3 as libc::c_int as libc::c_long
        + ((if jlmax >= jumax { jlmax } else { jumax }) << 1 as libc::c_int);
    lnfc = *n * 9 as libc::c_int as libc::c_long + 2 as libc::c_int as libc::c_long
        + jlmax + jumax + nzlu;
    *ldif = lsfc - lnfc;
    if *ldif < 0 as libc::c_int as libc::c_long {
        *ldif = 0 as libc::c_int as libc::c_long;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cdrv_(
    mut n: *mut libc::c_long,
    mut r: *mut libc::c_long,
    mut c: *mut libc::c_long,
    mut ic: *mut libc::c_long,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut z: *mut libc::c_double,
    mut nsp: *mut libc::c_long,
    mut isp: *mut libc::c_long,
    mut rsp: *mut libc::c_double,
    mut esp: *mut libc::c_long,
    mut path: libc::c_long,
    mut flag_: *mut libc::c_long,
) -> libc::c_int {
    let mut current_block: u64;
    let mut irac: libc::c_long = 0;
    let mut lmax: libc::c_long = 0;
    let mut umax: libc::c_long = 0;
    let mut d: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut q: libc::c_long = 0;
    let mut u: libc::c_long = 0;
    let mut jlmax: libc::c_long = 0;
    let mut jumax: libc::c_long = 0;
    let mut jutmp: libc::c_long = 0;
    let mut ar: libc::c_long = 0;
    let mut il: libc::c_long = 0;
    let mut jl: libc::c_long = 0;
    let mut iu: libc::c_long = 0;
    let mut ju: libc::c_long = 0;
    let mut ira: libc::c_long = 0;
    let mut jra: libc::c_long = 0;
    let mut ijl: libc::c_long = 0;
    let mut max_: libc::c_long = 0;
    let mut irl: libc::c_long = 0;
    let mut iju: libc::c_long = 0;
    let mut jrl: libc::c_long = 0;
    let mut iru: libc::c_long = 0;
    let mut tmp: libc::c_long = 0;
    let mut jru: libc::c_long = 0;
    let mut row: libc::c_long = 0;
    let mut lratio: libc::c_long = 0;
    lratio = (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
        as libc::c_long;
    if lratio < 1 as libc::c_int as libc::c_long {
        lratio = 1 as libc::c_int as libc::c_long;
    }
    rsp = rsp.offset(-1);
    rsp;
    isp = isp.offset(-1);
    isp;
    z = z.offset(-1);
    z;
    b = b.offset(-1);
    b;
    a = a.offset(-1);
    a;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    ic = ic.offset(-1);
    ic;
    c = c.offset(-1);
    c;
    r = r.offset(-1);
    r;
    if path < 1 as libc::c_int as libc::c_long
        || (5 as libc::c_int as libc::c_long) < path
    {
        *flag_ = *n * 11 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long;
        return 0 as libc::c_int;
    }
    il = 1 as libc::c_int as libc::c_long;
    ijl = il + (*n + 1 as libc::c_int as libc::c_long);
    iu = ijl + *n;
    iju = iu + (*n + 1 as libc::c_int as libc::c_long);
    irl = iju + *n;
    jrl = irl + *n;
    jl = jrl + *n;
    if (path - 1 as libc::c_int as libc::c_long)
        * (path - 5 as libc::c_int as libc::c_long) != 0 as libc::c_int as libc::c_long
    {
        current_block = 7688714797631891549;
    } else {
        max_ = lratio * *nsp + 1 as libc::c_int as libc::c_long - jl
            - (*n + 1 as libc::c_int as libc::c_long)
            - *n * 5 as libc::c_int as libc::c_long;
        jlmax = max_ / 2 as libc::c_int as libc::c_long;
        q = jl + jlmax;
        ira = q + (*n + 1 as libc::c_int as libc::c_long);
        jra = ira + *n;
        irac = jra + *n;
        iru = irac + *n;
        jru = iru + *n;
        jutmp = jru + *n;
        jumax = lratio * *nsp + 1 as libc::c_int as libc::c_long - jutmp;
        *esp = max_ / lratio;
        if jlmax <= 0 as libc::c_int as libc::c_long
            || jumax <= 0 as libc::c_int as libc::c_long
        {
            current_block = 640560946461574241;
        } else {
            i = 1 as libc::c_int as libc::c_long;
            loop {
                if !(i <= *n) {
                    current_block = 14067288136855562193;
                    break;
                }
                if *c.offset(i as isize) != i {
                    current_block = 572494604199905725;
                    break;
                }
                i += 1;
                i;
            }
            match current_block {
                572494604199905725 => {
                    ar = *nsp + 1 as libc::c_int as libc::c_long - *n;
                    nroc_(
                        n,
                        &mut *ic.offset(1 as libc::c_int as isize),
                        &mut *ia.offset(1 as libc::c_int as isize),
                        &mut *ja.offset(1 as libc::c_int as isize),
                        &mut *a.offset(1 as libc::c_int as isize),
                        &mut *isp.offset(il as isize),
                        &mut *rsp.offset(ar as isize),
                        &mut *isp.offset(iu as isize),
                        flag_,
                    );
                    if *flag_ != 0 as libc::c_int as libc::c_long {
                        current_block = 9520247013168877810;
                    } else {
                        current_block = 14067288136855562193;
                    }
                }
                _ => {}
            }
            match current_block {
                9520247013168877810 => {}
                _ => {
                    nsfc_(
                        n,
                        &mut *r.offset(1 as libc::c_int as isize),
                        &mut *ic.offset(1 as libc::c_int as isize),
                        &mut *ia.offset(1 as libc::c_int as isize),
                        &mut *ja.offset(1 as libc::c_int as isize),
                        &mut jlmax,
                        &mut *isp.offset(il as isize),
                        &mut *isp.offset(jl as isize),
                        &mut *isp.offset(ijl as isize),
                        &mut jumax,
                        &mut *isp.offset(iu as isize),
                        &mut *isp.offset(jutmp as isize),
                        &mut *isp.offset(iju as isize),
                        &mut *isp.offset(q as isize),
                        &mut *isp.offset(ira as isize),
                        &mut *isp.offset(jra as isize),
                        &mut *isp.offset(irac as isize),
                        &mut *isp.offset(irl as isize),
                        &mut *isp.offset(jrl as isize),
                        &mut *isp.offset(iru as isize),
                        &mut *isp.offset(jru as isize),
                        flag_,
                    );
                    if *flag_ != 0 as libc::c_int as libc::c_long {
                        current_block = 9520247013168877810;
                    } else {
                        jlmax = *isp
                            .offset(
                                (ijl + *n - 1 as libc::c_int as libc::c_long) as isize,
                            );
                        ju = jl + jlmax;
                        jumax = *isp
                            .offset(
                                (iju + *n - 1 as libc::c_int as libc::c_long) as isize,
                            );
                        if jumax <= 0 as libc::c_int as libc::c_long {
                            current_block = 7688714797631891549;
                        } else {
                            j = 1 as libc::c_int as libc::c_long;
                            while j <= jumax {
                                *isp
                                    .offset(
                                        (ju + j - 1 as libc::c_int as libc::c_long) as isize,
                                    ) = *isp
                                    .offset(
                                        (jutmp + j - 1 as libc::c_int as libc::c_long) as isize,
                                    );
                                j += 1;
                                j;
                            }
                            current_block = 7688714797631891549;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        7688714797631891549 => {
            jlmax = *isp.offset((ijl + *n - 1 as libc::c_int as libc::c_long) as isize);
            ju = jl + jlmax;
            jumax = *isp.offset((iju + *n - 1 as libc::c_int as libc::c_long) as isize);
            l = (ju + jumax - 2 as libc::c_int as libc::c_long + lratio) / lratio
                + 1 as libc::c_int as libc::c_long;
            lmax = *isp.offset((il + *n) as isize) - 1 as libc::c_int as libc::c_long;
            d = l + lmax;
            u = d + *n;
            row = *nsp + 1 as libc::c_int as libc::c_long - *n;
            tmp = row - *n;
            umax = tmp - u;
            *esp = umax
                - (*isp.offset((iu + *n) as isize) - 1 as libc::c_int as libc::c_long);
            if (path - 1 as libc::c_int as libc::c_long)
                * (path - 2 as libc::c_int as libc::c_long)
                != 0 as libc::c_int as libc::c_long
            {
                current_block = 11626300471535822927;
            } else if umax < 0 as libc::c_int as libc::c_long {
                current_block = 640560946461574241;
            } else {
                nnfc_(
                    n,
                    &mut *r.offset(1 as libc::c_int as isize),
                    &mut *c.offset(1 as libc::c_int as isize),
                    &mut *ic.offset(1 as libc::c_int as isize),
                    &mut *ia.offset(1 as libc::c_int as isize),
                    &mut *ja.offset(1 as libc::c_int as isize),
                    &mut *a.offset(1 as libc::c_int as isize),
                    &mut *z.offset(1 as libc::c_int as isize),
                    &mut *b.offset(1 as libc::c_int as isize),
                    &mut lmax,
                    &mut *isp.offset(il as isize),
                    &mut *isp.offset(jl as isize),
                    &mut *isp.offset(ijl as isize),
                    &mut *rsp.offset(l as isize),
                    &mut *rsp.offset(d as isize),
                    &mut umax,
                    &mut *isp.offset(iu as isize),
                    &mut *isp.offset(ju as isize),
                    &mut *isp.offset(iju as isize),
                    &mut *rsp.offset(u as isize),
                    &mut *rsp.offset(row as isize),
                    &mut *rsp.offset(tmp as isize),
                    &mut *isp.offset(irl as isize),
                    &mut *isp.offset(jrl as isize),
                    flag_,
                );
                if *flag_ != 0 as libc::c_int as libc::c_long {
                    current_block = 9520247013168877810;
                } else {
                    current_block = 11626300471535822927;
                }
            }
            match current_block {
                9520247013168877810 => {}
                640560946461574241 => {}
                _ => {
                    if !(path - 3 as libc::c_int as libc::c_long
                        != 0 as libc::c_int as libc::c_long)
                    {
                        nnsc_(
                            n,
                            &mut *r.offset(1 as libc::c_int as isize),
                            &mut *c.offset(1 as libc::c_int as isize),
                            &mut *isp.offset(il as isize),
                            &mut *isp.offset(jl as isize),
                            &mut *isp.offset(ijl as isize),
                            &mut *rsp.offset(l as isize),
                            &mut *rsp.offset(d as isize),
                            &mut *isp.offset(iu as isize),
                            &mut *isp.offset(ju as isize),
                            &mut *isp.offset(iju as isize),
                            &mut *rsp.offset(u as isize),
                            &mut *z.offset(1 as libc::c_int as isize),
                            &mut *b.offset(1 as libc::c_int as isize),
                            &mut *rsp.offset(tmp as isize),
                        );
                    }
                    if !(path - 4 as libc::c_int as libc::c_long
                        != 0 as libc::c_int as libc::c_long)
                    {
                        nntc_(
                            n,
                            &mut *r.offset(1 as libc::c_int as isize),
                            &mut *c.offset(1 as libc::c_int as isize),
                            &mut *isp.offset(il as isize),
                            &mut *isp.offset(jl as isize),
                            &mut *isp.offset(ijl as isize),
                            &mut *rsp.offset(l as isize),
                            &mut *rsp.offset(d as isize),
                            &mut *isp.offset(iu as isize),
                            &mut *isp.offset(ju as isize),
                            &mut *isp.offset(iju as isize),
                            &mut *rsp.offset(u as isize),
                            &mut *z.offset(1 as libc::c_int as isize),
                            &mut *b.offset(0 as libc::c_int as isize),
                            &mut *rsp.offset(tmp as isize),
                        );
                    }
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    match current_block {
        9520247013168877810 => return 0 as libc::c_int,
        _ => {
            *flag_ = *n * 10 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn nntc_(
    mut n: *mut libc::c_long,
    mut r: *mut libc::c_long,
    mut c: *mut libc::c_long,
    mut il: *mut libc::c_long,
    mut jl: *mut libc::c_long,
    mut ijl: *mut libc::c_long,
    mut l: *mut libc::c_double,
    mut d: *mut libc::c_double,
    mut iu: *mut libc::c_long,
    mut ju: *mut libc::c_long,
    mut iju: *mut libc::c_long,
    mut u: *mut libc::c_double,
    mut z: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut tmp: *mut libc::c_double,
) -> libc::c_int {
    let mut jmin: libc::c_long = 0;
    let mut jmax: libc::c_long = 0;
    let mut tmpk: libc::c_double = 0.;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut ml: libc::c_long = 0;
    let mut mu: libc::c_long = 0;
    let mut sum: libc::c_double = 0.;
    k = 0 as libc::c_int as libc::c_long;
    while k < *n {
        *tmp.offset(k as isize) = *b.offset(*c.offset(k as isize) as isize);
        k += 1;
        k;
    }
    k = 0 as libc::c_int as libc::c_long;
    while k < *n {
        jmin = *iu.offset(k as isize);
        jmax = *iu.offset((k + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        tmpk = -*tmp.offset(k as isize);
        if jmin <= jmax {
            mu = *iju.offset(k as isize) - jmin;
            j = jmin - 1 as libc::c_int as libc::c_long;
            while j < jmax {
                *tmp
                    .offset(
                        (*ju.offset((mu + j) as isize)
                            - 1 as libc::c_int as libc::c_long) as isize,
                    ) += tmpk * *u.offset(j as isize);
                j += 1;
                j;
            }
        }
        k += 1;
        k;
    }
    k = *n - 1 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int as libc::c_long;
    while i < *n {
        sum = -*tmp.offset(k as isize);
        jmin = *il.offset(k as isize);
        jmax = *il.offset((k + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        if jmin <= jmax {
            ml = *ijl.offset(k as isize) - jmin;
            j = jmin - 1 as libc::c_int as libc::c_long;
            while j < jmax {
                sum
                    += *l.offset(j as isize)
                        * *tmp
                            .offset(
                                (*jl.offset((ml + j) as isize)
                                    - 1 as libc::c_int as libc::c_long) as isize,
                            );
                j += 1;
                j;
            }
        }
        *tmp.offset(k as isize) = -sum * *d.offset(k as isize);
        *z
            .offset(
                (*r.offset(k as isize) - 1 as libc::c_int as libc::c_long) as isize,
            ) = *tmp.offset(k as isize);
        k -= 1;
        k;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn nnsc_(
    mut n: *mut libc::c_long,
    mut r: *mut libc::c_long,
    mut c: *mut libc::c_long,
    mut il: *mut libc::c_long,
    mut jl: *mut libc::c_long,
    mut ijl: *mut libc::c_long,
    mut l: *mut libc::c_double,
    mut d: *mut libc::c_double,
    mut iu: *mut libc::c_long,
    mut ju: *mut libc::c_long,
    mut iju: *mut libc::c_long,
    mut u: *mut libc::c_double,
    mut z: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut tmp: *mut libc::c_double,
) -> libc::c_int {
    let mut jmin: libc::c_long = 0;
    let mut jmax: libc::c_long = 0;
    let mut tmpk: libc::c_double = 0.;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut ml: libc::c_long = 0;
    let mut mu: libc::c_long = 0;
    let mut sum: libc::c_double = 0.;
    tmp = tmp.offset(-1);
    tmp;
    b = b.offset(-1);
    b;
    z = z.offset(-1);
    z;
    u = u.offset(-1);
    u;
    iju = iju.offset(-1);
    iju;
    ju = ju.offset(-1);
    ju;
    iu = iu.offset(-1);
    iu;
    d = d.offset(-1);
    d;
    l = l.offset(-1);
    l;
    ijl = ijl.offset(-1);
    ijl;
    jl = jl.offset(-1);
    jl;
    il = il.offset(-1);
    il;
    c = c.offset(-1);
    c;
    r = r.offset(-1);
    r;
    k = 1 as libc::c_int as libc::c_long;
    while k <= *n {
        *tmp.offset(k as isize) = *b.offset(*r.offset(k as isize) as isize);
        k += 1;
        k;
    }
    k = 1 as libc::c_int as libc::c_long;
    while k <= *n {
        jmin = *il.offset(k as isize);
        jmax = *il.offset((k + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        tmpk = -*d.offset(k as isize) * *tmp.offset(k as isize);
        *tmp.offset(k as isize) = -tmpk;
        if jmin <= jmax {
            ml = *ijl.offset(k as isize) - jmin;
            j = jmin;
            while j <= jmax {
                *tmp.offset(*jl.offset((ml + j) as isize) as isize)
                    += tmpk * *l.offset(j as isize);
                j += 1;
                j;
            }
        }
        k += 1;
        k;
    }
    k = *n;
    i = 1 as libc::c_int as libc::c_long;
    while i <= *n {
        sum = -*tmp.offset(k as isize);
        jmin = *iu.offset(k as isize);
        jmax = *iu.offset((k + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        if jmin <= jmax {
            mu = *iju.offset(k as isize) - jmin;
            j = jmin;
            while j <= jmax {
                sum
                    += *u.offset(j as isize)
                        * *tmp.offset(*ju.offset((mu + j) as isize) as isize);
                j += 1;
                j;
            }
        }
        *tmp.offset(k as isize) = -sum;
        *z.offset(*c.offset(k as isize) as isize) = -sum;
        k -= 1;
        k;
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn nnfc_(
    mut n: *mut libc::c_long,
    mut r: *mut libc::c_long,
    mut c: *mut libc::c_long,
    mut ic: *mut libc::c_long,
    mut ia: *mut libc::c_long,
    mut ja: *mut libc::c_long,
    mut a: *mut libc::c_double,
    mut z: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut lmax: *mut libc::c_long,
    mut il: *mut libc::c_long,
    mut jl: *mut libc::c_long,
    mut ijl: *mut libc::c_long,
    mut l: *mut libc::c_double,
    mut d: *mut libc::c_double,
    mut umax: *mut libc::c_long,
    mut iu: *mut libc::c_long,
    mut ju: *mut libc::c_long,
    mut iju: *mut libc::c_long,
    mut u: *mut libc::c_double,
    mut row: *mut libc::c_double,
    mut tmp: *mut libc::c_double,
    mut irl: *mut libc::c_long,
    mut jrl: *mut libc::c_long,
    mut flag_: *mut libc::c_long,
) -> libc::c_int {
    let mut ijlb: libc::c_long = 0;
    let mut jmin: libc::c_long = 0;
    let mut jmax: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut i1: libc::c_long = 0;
    let mut i2: libc::c_long = 0;
    let mut dk: libc::c_double = 0.;
    let mut rk: libc::c_long = 0;
    let mut mu: libc::c_long = 0;
    let mut lki: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    jrl = jrl.offset(-1);
    jrl;
    irl = irl.offset(-1);
    irl;
    tmp = tmp.offset(-1);
    tmp;
    row = row.offset(-1);
    row;
    u = u.offset(-1);
    u;
    iju = iju.offset(-1);
    iju;
    ju = ju.offset(-1);
    ju;
    iu = iu.offset(-1);
    iu;
    d = d.offset(-1);
    d;
    l = l.offset(-1);
    l;
    ijl = ijl.offset(-1);
    ijl;
    jl = jl.offset(-1);
    jl;
    il = il.offset(-1);
    il;
    b = b.offset(-1);
    b;
    z = z.offset(-1);
    z;
    a = a.offset(-1);
    a;
    ja = ja.offset(-1);
    ja;
    ia = ia.offset(-1);
    ia;
    ic = ic.offset(-1);
    ic;
    c = c.offset(-1);
    c;
    r = r.offset(-1);
    r;
    if *il.offset((*n + 1 as libc::c_int as libc::c_long) as isize)
        - 1 as libc::c_int as libc::c_long > *lmax
    {
        *flag_ = (*n << 2 as libc::c_int) + 1 as libc::c_int as libc::c_long;
        return 0 as libc::c_int;
    }
    if *iu.offset((*n + 1 as libc::c_int as libc::c_long) as isize)
        - 1 as libc::c_int as libc::c_long > *umax
    {
        *flag_ = *n * 7 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long;
        return 0 as libc::c_int;
    }
    k = 1 as libc::c_int as libc::c_long;
    while k <= *n {
        *irl.offset(k as isize) = *il.offset(k as isize);
        *jrl.offset(k as isize) = 0 as libc::c_int as libc::c_long;
        k += 1;
        k;
    }
    let mut current_block_97: u64;
    k = 1 as libc::c_int as libc::c_long;
    while k <= *n {
        *row.offset(k as isize) = 0.0f64;
        i1 = 0 as libc::c_int as libc::c_long;
        if !(*jrl.offset(k as isize) == 0 as libc::c_int as libc::c_long) {
            i = *jrl.offset(k as isize);
            loop {
                i2 = *jrl.offset(i as isize);
                *jrl.offset(i as isize) = i1;
                i1 = i;
                *row.offset(i as isize) = 0.0f64;
                i = i2;
                if !(i != 0 as libc::c_int as libc::c_long) {
                    break;
                }
            }
        }
        jmin = *iju.offset(k as isize);
        jmax = jmin + *iu.offset((k + 1 as libc::c_int as libc::c_long) as isize)
            - *iu.offset(k as isize) - 1 as libc::c_int as libc::c_long;
        if jmin <= jmax {
            j = jmin;
            while j <= jmax {
                *row.offset(*ju.offset(j as isize) as isize) = 0.0f64;
                j += 1;
                j;
            }
        }
        rk = *r.offset(k as isize);
        jmin = *ia.offset(rk as isize);
        jmax = *ia.offset((rk + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        j = jmin;
        while j <= jmax {
            *row
                .offset(
                    *ic.offset(*ja.offset(j as isize) as isize) as isize,
                ) = *a.offset(j as isize);
            j += 1;
            j;
        }
        sum = *b.offset(rk as isize);
        i = i1;
        if i == 0 as libc::c_int as libc::c_long {
            current_block_97 = 7994950529128894910;
        } else {
            current_block_97 = 14898944158652026007;
        }
        loop {
            match current_block_97 {
                7994950529128894910 => {
                    if *row.offset(k as isize) == 0.0f64 {
                        *flag_ = (*n << 3 as libc::c_int) + k;
                        return 0 as libc::c_int;
                    }
                    break;
                }
                _ => {
                    lki = -*row.offset(i as isize);
                    *l.offset(*irl.offset(i as isize) as isize) = -lki;
                    sum += lki * *tmp.offset(i as isize);
                    jmin = *iu.offset(i as isize);
                    jmax = *iu.offset((i + 1 as libc::c_int as libc::c_long) as isize)
                        - 1 as libc::c_int as libc::c_long;
                    if jmin <= jmax {
                        mu = *iju.offset(i as isize) - jmin;
                        j = jmin;
                        while j <= jmax {
                            *row.offset(*ju.offset((mu + j) as isize) as isize)
                                += lki * *u.offset(j as isize);
                            j += 1;
                            j;
                        }
                    }
                    i = *jrl.offset(i as isize);
                    if i != 0 as libc::c_int as libc::c_long {
                        current_block_97 = 14898944158652026007;
                    } else {
                        current_block_97 = 7994950529128894910;
                    }
                }
            }
        }
        dk = 1.0f64 / *row.offset(k as isize);
        *d.offset(k as isize) = dk;
        *tmp.offset(k as isize) = sum * dk;
        if k == *n {
            break;
        }
        jmin = *iu.offset(k as isize);
        jmax = *iu.offset((k + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        if jmin <= jmax {
            mu = *iju.offset(k as isize) - jmin;
            j = jmin;
            while j <= jmax {
                *u
                    .offset(
                        j as isize,
                    ) = *row.offset(*ju.offset((mu + j) as isize) as isize) * dk;
                j += 1;
                j;
            }
        }
        i = i1;
        if i == 0 as libc::c_int as libc::c_long {
            current_block_97 = 5873822016354245970;
        } else {
            current_block_97 = 9190534984756623203;
        }
        loop {
            match current_block_97 {
                5873822016354245970 => {
                    if *irl.offset(k as isize)
                        < *il.offset((k + 1 as libc::c_int as libc::c_long) as isize)
                    {
                        j = *jl.offset(*ijl.offset(k as isize) as isize);
                        *jrl.offset(k as isize) = *jrl.offset(j as isize);
                        *jrl.offset(j as isize) = k;
                    }
                    break;
                }
                _ => {
                    let ref mut fresh0 = *irl.offset(i as isize);
                    *fresh0 += 1;
                    *fresh0;
                    i1 = *jrl.offset(i as isize);
                    if !(*irl.offset(i as isize)
                        >= *il.offset((i + 1 as libc::c_int as libc::c_long) as isize))
                    {
                        ijlb = *irl.offset(i as isize) - *il.offset(i as isize)
                            + *ijl.offset(i as isize);
                        j = *jl.offset(ijlb as isize);
                        while !(i > *jrl.offset(j as isize)) {
                            j = *jrl.offset(j as isize);
                        }
                        *jrl.offset(i as isize) = *jrl.offset(j as isize);
                        *jrl.offset(j as isize) = i;
                    }
                    i = i1;
                    if i != 0 as libc::c_int as libc::c_long {
                        current_block_97 = 9190534984756623203;
                    } else {
                        current_block_97 = 5873822016354245970;
                    }
                }
            }
        }
        k += 1;
        k;
    }
    k = *n;
    i = 1 as libc::c_int as libc::c_long;
    while i <= *n {
        sum = *tmp.offset(k as isize);
        jmin = *iu.offset(k as isize);
        jmax = *iu.offset((k + 1 as libc::c_int as libc::c_long) as isize)
            - 1 as libc::c_int as libc::c_long;
        if jmin <= jmax {
            mu = *iju.offset(k as isize) - jmin;
            j = jmin;
            while j <= jmax {
                sum
                    -= *u.offset(j as isize)
                        * *tmp.offset(*ju.offset((mu + j) as isize) as isize);
                j += 1;
                j;
            }
        }
        *tmp.offset(k as isize) = sum;
        *z.offset(*c.offset(k as isize) as isize) = sum;
        k -= 1;
        k;
        i += 1;
        i;
    }
    *flag_ = 0 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
