use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn ReportRunTimeError(_: PANALYSIS, _: WORD, _: ...);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type BOOL = libc::c_int;
pub type WORD = libc::c_uint;
pub type DWORD = libc::c_ulong;
pub type HANDLE = DWORD;
pub type PSTR = *mut libc::c_char;
pub type PINT = *mut libc::c_int;
pub type PLONG = *mut libc::c_long;
pub type PVOID = *mut libc::c_void;
pub type PDOUBLE = *mut libc::c_double;
pub type PFILE = *mut FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagIFN {
    pub iType: libc::c_int,
    pub bOn: BOOL,
    pub dTStartPeriod: libc::c_double,
    pub dVal: libc::c_double,
    pub dMag: libc::c_double,
    pub dTper: libc::c_double,
    pub dT0: libc::c_double,
    pub dTexp: libc::c_double,
    pub dDecay: libc::c_double,
    pub dNcpt: libc::c_double,
    pub hMag: HANDLE,
    pub hTper: HANDLE,
    pub hT0: HANDLE,
    pub hTexp: HANDLE,
    pub hDecay: HANDLE,
    pub hNcpt: HANDLE,
    pub nDoses: libc::c_int,
    pub iDoseCur: libc::c_int,
    pub rgT0s: PDOUBLE,
    pub rgMags: PDOUBLE,
    pub rghT0s: *mut HANDLE,
    pub rghMags: *mut HANDLE,
    pub target_state: HANDLE,
    pub rgOper: PINT,
}
pub type PIFN = *mut tagIFN;
pub type HVAR = HANDLE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagLISTELEM {
    pub pData: PVOID,
    pub pleNext: *mut tagLISTELEM,
}
pub type PLISTELEM = *mut tagLISTELEM;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagLIST {
    pub pleHead: PLISTELEM,
    pub pleTail: PLISTELEM,
    pub iSize: libc::c_int,
}
pub type PLIST = *mut tagLIST;
pub type RANDREC = tagRANDREC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagRANDREC {
    pub seed: libc::c_double,
}
pub type PANALYSIS = *mut tagANALYSIS;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagANALYSIS {
    pub rank: libc::c_int,
    pub size: libc::c_int,
    pub bDependents: BOOL,
    pub bOutputIter: BOOL,
    pub nOutputFreq: libc::c_int,
    pub bPrintConvergence: BOOL,
    pub iType: libc::c_int,
    pub wContext: WORD,
    pub dSeed: libc::c_double,
    pub modelinfo: MODELINFO,
    pub iDepth: libc::c_int,
    pub iCurrentDepth: libc::c_int,
    pub iInstances: libc::c_int,
    pub iExpts: libc::c_int,
    pub pLevels: [PLEVEL; 20000],
    pub pCurrentLevel: [PLEVEL; 10],
    pub iInstance: [libc::c_int; 10],
    pub expGlobal: EXPERIMENT,
    pub szOutfilename: PSTR,
    pub pfileOut: PFILE,
    pub bCommandLineSpec: BOOL,
    pub bAllocatedFileName: BOOL,
    pub rgpExps: [PEXPERIMENT; 20000],
    pub pexpCurrent: PEXPERIMENT,
    pub plistVars: PLIST,
    pub mc: MONTECARLO,
    pub gd: GIBBSDATA,
}
pub type GIBBSDATA = tagGIBBSDATA;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagGIBBSDATA {
    pub nMaxIter: libc::c_long,
    pub nSimTypeFlag: libc::c_long,
    pub nPrintFreq: libc::c_long,
    pub nPrintIter: libc::c_long,
    pub nMaxPerkSetIter: libc::c_long,
    pub szGout: PSTR,
    pub pfileOut: PFILE,
    pub szGrestart: PSTR,
    pub pfileRestart: PFILE,
    pub szGdata: PSTR,
    pub pfilePerks: PFILE,
    pub nPerks: libc::c_int,
    pub rgdPerks: PDOUBLE,
    pub rglTransAttempts: PLONG,
    pub rglTransAccepts: PLONG,
    pub indexT: libc::c_int,
    pub rgdlnPi: PDOUBLE,
    pub rglCount: PLONG,
    pub dCZero: libc::c_double,
    pub dNZero: libc::c_double,
    pub startT: libc::c_int,
    pub endT: libc::c_int,
}
pub type MONTECARLO = tagMONTECARLO;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagMONTECARLO {
    pub nRuns: libc::c_long,
    pub lRun: libc::c_long,
    pub szMCOutfilename: PSTR,
    pub pfileMCOut: PFILE,
    pub szSetPointsFilename: PSTR,
    pub pfileSetPoints: PFILE,
    pub plistMCVars: PLIST,
    pub nParms: libc::c_long,
    pub rgdParms: *mut libc::c_double,
    pub rghvar: *mut HVAR,
    pub rgpMCVar: *mut *mut MCVAR,
    pub nSetParms: libc::c_long,
    pub style: libc::c_int,
}
pub type MCVAR = tagMCVAR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagMCVAR {
    pub pszName: PSTR,
    pub hvar: HVAR,
    pub dVal: libc::c_double,
    pub pdVal: PDOUBLE,
    pub dVal_mean: libc::c_double,
    pub dVal_var: libc::c_double,
    pub iDepth: libc::c_int,
    pub iType: libc::c_int,
    pub hParm: [HVAR; 4],
    pub dParm: [libc::c_double; 4],
    pub pdParm: [PDOUBLE; 4],
    pub iParmType: [libc::c_int; 4],
    pub pMCVParent: [*mut tagMCVAR; 4],
    pub plistDependents: PLIST,
    pub nDependents: libc::c_long,
    pub rgpDependents: *mut *mut tagMCVAR,
    pub bExptIsDep: BOOL,
    pub bIsFixed: BOOL,
    pub bGibbs: BOOL,
    pub lJumps: libc::c_long,
    pub lCount: libc::c_long,
    pub dKernelSD: libc::c_double,
    pub dMaxKernelSD: libc::c_double,
    pub pdSum: PDOUBLE,
    pub pdSumSq: PDOUBLE,
}
pub type PEXPERIMENT = *mut tagEXPERIMENT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagEXPERIMENT {
    pub iExp: libc::c_int,
    pub dT0: libc::c_double,
    pub hT0: HANDLE,
    pub dTfinal: libc::c_double,
    pub dTime: libc::c_double,
    pub pmodelinfo: PMODELINFO,
    pub plistParmMods: PLIST,
    pub is: INTSPEC,
    pub os: OUTSPEC,
    pub dLnLike: libc::c_double,
    pub dLnLikeSave: libc::c_double,
}
pub type OUTSPEC = tagOUTSPEC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagOUTSPEC {
    pub nOutputs: libc::c_int,
    pub plistPrintRecs: PLIST,
    pub pszOutputNames: *mut PSTR,
    pub phvar_out: *mut HVAR,
    pub nData: libc::c_int,
    pub plistDataRecs: PLIST,
    pub pszDataNames: *mut PSTR,
    pub phvar_dat: *mut HVAR,
    pub pcOutputTimes: PINT,
    pub piCurrentOut: PINT,
    pub prgdOutputTimes: *mut PDOUBLE,
    pub prgdOutputVals: *mut PDOUBLE,
    pub cDistinctTimes: libc::c_int,
    pub rgdDistinctTimes: PDOUBLE,
    pub pcData: PINT,
    pub prgdDataVals: *mut PDOUBLE,
}
pub type INTSPEC = tagINTSPEC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagINTSPEC {
    pub iAlgo: libc::c_int,
    pub dRtol: libc::c_double,
    pub dAtol: libc::c_double,
    pub iopt: libc::c_long,
    pub itask: libc::c_long,
    pub itol: libc::c_long,
    pub iMf: libc::c_long,
    pub iDSFlag: libc::c_long,
    pub liw: libc::c_long,
    pub lrw: libc::c_long,
    pub iwork: PLONG,
    pub rwork: PDOUBLE,
    pub maxsteps: libc::c_int,
    pub maxnef: libc::c_int,
    pub maxcor: libc::c_int,
    pub maxncf: libc::c_int,
    pub nlscoef: libc::c_double,
    pub dTStep: libc::c_double,
}
pub type PMODELINFO = *mut tagMODELINFO;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagMODELINFO {
    pub nStates: libc::c_long,
    pub nModelVars: libc::c_long,
    pub pStateHvar: *mut HVAR,
    pub pdModelVars: PDOUBLE,
}
pub type EXPERIMENT = tagEXPERIMENT;
pub type PLEVEL = *mut tagLEVEL;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagLEVEL {
    pub iDepth: libc::c_int,
    pub iSequence: libc::c_int,
    pub iInstances: libc::c_int,
    pub pLevels: [*mut tagLEVEL; 20000],
    pub plistVars: PLIST,
    pub nFixedVars: libc::c_long,
    pub rgpFixedVars: *mut PVARMOD,
    pub plistMCVars: PLIST,
    pub nMCVars: libc::c_long,
    pub rgpMCVars: *mut PMCVAR,
    pub plistLikes: PLIST,
    pub nLikes: libc::c_long,
    pub rgpLikes: *mut PMCVAR,
    pub pexpt: PEXPERIMENT,
}
pub type PMCVAR = *mut tagMCVAR;
pub type PVARMOD = *mut tagVARMODIFICATION;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagVARMODIFICATION {
    pub hvar: HVAR,
    pub uvar: UVAR,
}
pub type UVAR = tagUVAR;
#[derive(Copy, Clone)]
#[repr(C)]
pub union tagUVAR {
    pub dVal: libc::c_double,
    pub pifn: PIFN,
}
pub type MODELINFO = tagMODELINFO;
static mut vbSwitchGauss: BOOL = 0 as libc::c_int;
static mut vbNoSeed: BOOL = 1 as libc::c_int;
static mut vbNotInitd: BOOL = 1 as libc::c_int;
static mut vRandRec: RANDREC = RANDREC { seed: 0. };
pub unsafe extern "C" fn GetSeed() -> libc::c_double {
    return vRandRec.seed;
}
pub unsafe extern "C" fn SetSeed(mut dSeed: libc::c_double) {
    let mut bCorrected: libc::c_int = 0 as libc::c_int;
    if dSeed == 0.0f64 {
        dSeed = 314159265.3589793f64;
        bCorrected += 1;
        bCorrected;
    }
    if dSeed < 0 as libc::c_int as libc::c_double {
        dSeed = -dSeed;
    }
    if dSeed < 1.0f64 {
        dSeed = 1.0f64 + dSeed / 1.0f64 / (2147483646.0f64 - 1.0f64);
        bCorrected += 1;
        bCorrected;
    }
    if dSeed > 2147483646.0f64 {
        dSeed = 1.0f64 + 2147483646.0f64 / dSeed / (2147483646.0f64 - 1.0f64);
        bCorrected += 1;
        bCorrected;
    }
    if dSeed >= 1.0f64 && dSeed <= 2147483646.0f64 {} else {
        __assert_fail(
            b"( dSeed >= SEED_MIN && dSeed <= SEED_MAX)\0" as *const u8
                as *const libc::c_char,
            b"random.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"void SetSeed(double)\0"))
                .as_ptr(),
        );
    }
    'c_3886: {
        if dSeed >= 1.0f64 && dSeed <= 2147483646.0f64 {} else {
            __assert_fail(
                b"( dSeed >= SEED_MIN && dSeed <= SEED_MAX)\0" as *const u8
                    as *const libc::c_char,
                b"random.c\0" as *const u8 as *const libc::c_char,
                248 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"void SetSeed(double)\0"))
                    .as_ptr(),
            );
        }
    };
    if bCorrected != 0 {
        printf(
            b"SetSeed():  corrected out of range random number seed\nSeed must lie in the range [%g, %g]\nNew seed --> %g\n\0"
                as *const u8 as *const libc::c_char,
            1.0f64,
            2147483646.0f64,
            dSeed,
        );
    }
    vRandRec.seed = dSeed;
    vbNoSeed = 0 as libc::c_int;
    vbSwitchGauss = 0 as libc::c_int;
}
pub unsafe extern "C" fn InitRandom(
    mut rank: libc::c_int,
    mut dSeed: libc::c_double,
    mut bWarmUp: libc::c_int,
) {
    let mut i: libc::c_long = 0;
    if vbNoSeed != 0 || dSeed != 314159265.3589793f64 {
        SetSeed(dSeed);
    }
    if bWarmUp != 0 {
        i = 0 as libc::c_int as libc::c_long;
        while i < 50 as libc::c_int as libc::c_long {
            Randoms();
            i += 1;
            i;
        }
        vbNotInitd = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn Randoms() -> libc::c_double {
    let mut hi: libc::c_double = 0.;
    let mut test: libc::c_double = 0.;
    if vbNoSeed != 0 {
        SetSeed(314159265.3589793f64);
    }
    hi = (vRandRec.seed / 127773.0f64) as libc::c_long as libc::c_double;
    test = 16807.0f64 * (vRandRec.seed - 127773.0f64 * hi) - 2836.0f64 * hi;
    if test > 0.0f64 {
        vRandRec.seed = test;
    } else {
        vRandRec.seed = test + 2147483647.0f64;
    }
    return vRandRec.seed / 2147483647.0f64;
}
pub unsafe extern "C" fn BetaRandom(
    mut alpha: libc::c_double,
    mut beta: libc::c_double,
    mut a_0: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let mut u1: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    if b <= a_0 || alpha <= 0 as libc::c_int as libc::c_double
        || beta <= 0 as libc::c_int as libc::c_double
    {
        printf(
            b"Error: bad shape or range for a beta variate - Exiting\n\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if alpha < 1 as libc::c_int as libc::c_double
        && beta < 1 as libc::c_int as libc::c_double
    {
        loop {
            u1 = Randoms();
            u2 = Randoms();
            u1 = pow(u1, 1 as libc::c_int as libc::c_double / alpha);
            u2 = pow(u2, 1 as libc::c_int as libc::c_double / beta);
            w = u1 + u2;
            if !(w > 1.0f64) {
                break;
            }
        }
    } else {
        u1 = GammaRandom(alpha);
        u2 = GammaRandom(beta);
        w = u1 + u2;
    }
    return a_0 + u1 / w * (b - a_0);
}
pub unsafe extern "C" fn BinomialBetaRandom(
    mut Expectation: libc::c_double,
    mut alpha: libc::c_double,
    mut beta: libc::c_double,
) -> libc::c_double {
    let mut dTmp: libc::c_double = Expectation + Expectation * beta / alpha;
    if dTmp < 9223372036854775807 as libc::c_long as libc::c_double {
        return BinomialRandom(
            BetaRandom(
                alpha,
                beta,
                0 as libc::c_int as libc::c_double,
                1 as libc::c_int as libc::c_double,
            ),
            dTmp as libc::c_long,
        )
    } else {
        printf(
            b"BinomialBetaRandom: N (= %g) too large - Exiting...\0" as *const u8
                as *const libc::c_char,
            dTmp,
        );
        exit(0 as libc::c_int);
    };
}
pub unsafe extern "C" fn BinomialRandom(
    mut p: libc::c_double,
    mut N: libc::c_long,
) -> libc::c_double {
    let mut j: libc::c_long = 0;
    static mut iOldN: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    let mut dAngle: libc::c_double = 0.;
    let mut dDeviate: libc::c_double = 0.;
    let mut dMean: libc::c_double = 0.;
    let mut dPtemp: libc::c_double = 0.;
    let mut dSqrt: libc::c_double = 0.;
    let mut dTangent: libc::c_double = 0.;
    let mut dTemp1: libc::c_double = 0.;
    let mut dTemp2: libc::c_double = 0.;
    static mut dLnFactN: libc::c_double = 0.;
    static mut dPold: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    static mut dLnP: libc::c_double = 0.;
    static mut dQ: libc::c_double = 0.;
    static mut dLnQ: libc::c_double = 0.;
    if p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double
        || N < 0 as libc::c_int as libc::c_long
    {
        printf(
            b"Error: parameters out of bounds for a binomial variate - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    dPtemp = if p <= 0.5f64 { p } else { 1 as libc::c_int as libc::c_double - p };
    dMean = N as libc::c_double * dPtemp;
    if N < 25 as libc::c_int as libc::c_long {
        dDeviate = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int as libc::c_long;
        while j < N {
            if Randoms() < dPtemp {
                dDeviate = dDeviate + 1 as libc::c_int as libc::c_double;
            }
            j += 1;
            j;
        }
    } else if dMean < 1 as libc::c_int as libc::c_double {
        dTemp1 = exp(-dMean);
        dTemp2 = 1.0f64;
        j = 0 as libc::c_int as libc::c_long;
        while j <= N {
            dTemp2 = dTemp2 * Randoms();
            if dTemp2 < dTemp1 {
                break;
            }
            j += 1;
            j;
        }
        dDeviate = (if j <= N { j } else { N }) as libc::c_double;
    } else {
        if N != iOldN {
            dLnFactN = lnGamma(N as libc::c_double + 1 as libc::c_int as libc::c_double);
            iOldN = N;
        }
        if dPtemp != dPold {
            dPold = dPtemp;
            dQ = 1 as libc::c_int as libc::c_double - dPtemp;
            dLnP = log(dPtemp);
            dLnQ = log(dQ);
        }
        dSqrt = sqrt(2 as libc::c_int as libc::c_double * dMean * dQ);
        loop {
            loop {
                dAngle = 3.1415926535897932384626433f64 * Randoms();
                dTangent = tan(dAngle);
                dTemp1 = dSqrt * dTangent + dMean;
                if !(dTemp1 < 0 as libc::c_int as libc::c_double
                    || dTemp1
                        >= (N + 1 as libc::c_int as libc::c_long) as libc::c_double)
                {
                    break;
                }
            }
            dTemp1 = floor(dTemp1);
            dTemp2 = 1.2f64 * dSqrt
                * (1 as libc::c_int as libc::c_double + dTangent * dTangent)
                * exp(
                    dLnFactN - lnGamma(dTemp1 + 1 as libc::c_int as libc::c_double)
                        - lnGamma(
                            N as libc::c_double - dTemp1
                                + 1 as libc::c_int as libc::c_double,
                        ) + dTemp1 * dLnP + (N as libc::c_double - dTemp1) * dLnQ,
                );
            if !(Randoms() > dTemp2) {
                break;
            }
        }
        dDeviate = dTemp1;
    }
    if dPtemp != p {
        dDeviate = N as libc::c_double - dDeviate;
    }
    return dDeviate;
}
pub unsafe extern "C" fn CauchyRandom(mut dScale: libc::c_double) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    z = NormalRandom(0 as libc::c_int as libc::c_double, dScale);
    x = GGammaRandom(0.5f64, 0.5f64);
    return z / sqrt(x);
}
pub unsafe extern "C" fn Chi2Random(mut dof: libc::c_double) -> libc::c_double {
    return GGammaRandom(dof / 2.0f64, 0.5f64);
}
pub unsafe extern "C" fn ExpRandom(mut beta: libc::c_double) -> libc::c_double {
    if beta <= 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: negative or null inverse scale for an exponential variate - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    return -log(Randoms()) / beta;
}
pub unsafe extern "C" fn GammaRandom(mut alpha: libc::c_double) -> libc::c_double {
    static mut aprev: libc::c_double = 0.0f64;
    static mut c1: libc::c_double = 0.;
    static mut c2: libc::c_double = 0.;
    static mut c3: libc::c_double = 0.;
    static mut c4: libc::c_double = 0.;
    static mut c5: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut u1: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    if alpha <= 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: negative or null shape parameter for a gamma variate - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    } else if alpha < 1 as libc::c_int as libc::c_double {
        b = (alpha + 2.718281828459f64) / 2.718281828459f64;
        loop {
            u1 = b * Randoms();
            if u1 <= 1.0f64 {
                x = pow(u1, 1.0f64 / alpha);
                if x > 2.2250738585072014e-308f64 && x <= -log(Randoms()) {
                    return x;
                }
            } else {
                x = -log((b - u1) / alpha);
                if pow(x, alpha - 1 as libc::c_int as libc::c_double) >= Randoms() {
                    return x;
                }
            }
            if !(1 as libc::c_int == 1 as libc::c_int) {
                break;
            }
        }
    } else if alpha > 1 as libc::c_int as libc::c_double {
        if alpha != aprev {
            aprev = alpha;
            c1 = alpha - 1 as libc::c_int as libc::c_double;
            b = 1.0f64 / c1;
            c2 = b * (alpha - 1 as libc::c_int as libc::c_double / (6.0f64 * alpha));
            c3 = 2 as libc::c_int as libc::c_double * b;
            c4 = c3 + 2.0f64;
            if alpha > 2.5f64 {
                c5 = 1.0f64 / sqrt(alpha);
            }
        }
        loop {
            loop {
                u1 = Randoms();
                u2 = Randoms();
                if alpha > 2.5f64 {
                    u1 = u2 + c5 * (1 as libc::c_int as libc::c_double - 1.86f64 * u1);
                }
                if !(u1 >= 1 as libc::c_int as libc::c_double
                    || u1 <= 0 as libc::c_int as libc::c_double)
                {
                    break;
                }
            }
            w = c2 * u2 / u1;
            if c3 * u1 + w + 1 as libc::c_int as libc::c_double / w <= c4
                || c3 * log(u1) - log(w) + w < 1 as libc::c_int as libc::c_double
            {
                return c1 * w;
            }
            if !(1 as libc::c_int == 1 as libc::c_int) {
                break;
            }
        }
    } else {
        return ExpRandom(1.0f64)
    }
    panic!();
}
pub unsafe extern "C" fn GenLogNormalRandom(
    mut dMean: libc::c_double,
    mut dSDNorm: libc::c_double,
    mut dSDLogNorm: libc::c_double,
) -> libc::c_double {
    let mut dmuz: libc::c_double = 0.;
    let mut dSLogNorm: libc::c_double = 0.;
    let mut dLambda: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    if dMean < 0 as libc::c_int as libc::c_double {
        let mut str: [libc::c_char; 10] = [0; 10];
        sprintf(str.as_mut_ptr(), b"%5.2e\0" as *const u8 as *const libc::c_char, dMean);
        ReportRunTimeError(
            0 as PANALYSIS,
            (0x303 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"\0" as *const u8 as *const libc::c_char,
            str.as_mut_ptr(),
            b"GenLogNormalRandom\0" as *const u8 as *const libc::c_char,
        );
    } else if dSDLogNorm <= 0 as libc::c_int as libc::c_double {
        let mut str_0: [libc::c_char; 10] = [0; 10];
        sprintf(
            str_0.as_mut_ptr(),
            b"%5.2e\0" as *const u8 as *const libc::c_char,
            dSDLogNorm,
        );
        ReportRunTimeError(
            0 as PANALYSIS,
            (0x302 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"\0" as *const u8 as *const libc::c_char,
            str_0.as_mut_ptr(),
            b"GenLogNormalRandom\0" as *const u8 as *const libc::c_char,
        );
    }
    dSLogNorm = sqrt(
        exp(pow(dSDLogNorm, 2 as libc::c_int as libc::c_double))
            * (exp(pow(dSDLogNorm, 2 as libc::c_int as libc::c_double))
                - 1 as libc::c_int as libc::c_double),
    );
    dLambda = pow(dSDNorm / dSLogNorm, 2 as libc::c_int as libc::c_double);
    dmuz = log(dMean + sqrt(pow(dMean, 2 as libc::c_int as libc::c_double) + dLambda));
    dz = NormalRandom(dmuz, dSLogNorm);
    return (exp(dz) - dLambda * exp(-dz)) / 2 as libc::c_int as libc::c_double;
}
pub unsafe extern "C" fn GGammaRandom(
    mut alpha: libc::c_double,
    mut beta: libc::c_double,
) -> libc::c_double {
    if beta <= 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: negative or null inverse scale for a gamma variate - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    return GammaRandom(alpha) / beta;
}
pub unsafe extern "C" fn InvGGammaRandom(
    mut alpha: libc::c_double,
    mut beta: libc::c_double,
) -> libc::c_double {
    if beta <= 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: negative or null scale for an inverse gamma variate - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    return beta / GammaRandom(alpha);
}
pub unsafe extern "C" fn LogNormalRandom(
    mut dGeoMean: libc::c_double,
    mut dGeoSD: libc::c_double,
) -> libc::c_double {
    if dGeoMean <= 0 as libc::c_int as libc::c_double {
        let mut str: [libc::c_char; 10] = [0; 10];
        sprintf(
            str.as_mut_ptr(),
            b"%5.2e\0" as *const u8 as *const libc::c_char,
            dGeoMean,
        );
        ReportRunTimeError(
            0 as PANALYSIS,
            (0x303 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"\0" as *const u8 as *const libc::c_char,
            str.as_mut_ptr(),
            b"LogNormalRandom\0" as *const u8 as *const libc::c_char,
        );
    } else if dGeoSD < 1 as libc::c_int as libc::c_double {
        let mut str_0: [libc::c_char; 10] = [0; 10];
        sprintf(
            str_0.as_mut_ptr(),
            b"%5.2e\0" as *const u8 as *const libc::c_char,
            dGeoSD,
        );
        ReportRunTimeError(
            0 as PANALYSIS,
            (0x302 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"\0" as *const u8 as *const libc::c_char,
            str_0.as_mut_ptr(),
            b"LogNormalRandom\0" as *const u8 as *const libc::c_char,
        );
    }
    return exp(NormalRandom(log(dGeoMean), log(dGeoSD)));
}
pub unsafe extern "C" fn LogUniformRandom(
    mut a_0: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    if b < a_0 {
        printf(
            b"Error: bad range a for uniform variate - Exiting\n\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    return a_0 * pow(b / a_0, Randoms());
}
pub unsafe extern "C" fn Multinomial(
    mut n: libc::c_long,
    mut dim: libc::c_int,
    mut p: *mut libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut sum: libc::c_double = 0.;
    let mut ptemp: libc::c_double = 0.;
    sum = 1 as libc::c_int as libc::c_double;
    i = 1 as libc::c_int;
    while i <= dim {
        if *p.offset(i as isize) != 0. {
            ptemp = *p.offset(i as isize) / sum;
            *x.offset(i as isize) = BinomialRandom(ptemp, n);
            n = n - *x.offset(i as isize) as libc::c_long;
            sum = sum - *p.offset(i as isize);
        } else {
            *x.offset(i as isize) = 0.0f64;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn NegativeBinomialRandom(
    mut r_0: libc::c_double,
    mut p: libc::c_double,
) -> libc::c_long {
    if p < 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: parameter p negative for a negative binomial variate - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if p >= 1 as libc::c_int as libc::c_double {
        printf(
            b"Error: parameter p >= 1 for a negative binomial variate - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if r_0 < 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: parameter r negative for a negative binomial variate - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if r_0 == 0 as libc::c_int as libc::c_double
        || p == 0 as libc::c_int as libc::c_double
    {
        return 0 as libc::c_int as libc::c_long;
    }
    return PoissonRandom(
        GGammaRandom(r_0, (1 as libc::c_int as libc::c_double - p) / p),
    );
}
pub unsafe extern "C" fn NormalRandom(
    mut dMean: libc::c_double,
    mut dSD: libc::c_double,
) -> libc::c_double {
    let mut dRacine: libc::c_double = 0.;
    let mut dTemp1: libc::c_double = 0.;
    let mut dTemp2: libc::c_double = 0.;
    let mut dTemp3: libc::c_double = 0.;
    static mut memGauss: libc::c_double = 0.;
    if vbSwitchGauss != 0 {
        vbSwitchGauss = 0 as libc::c_int;
        return dMean + dSD * memGauss;
    }
    loop {
        dTemp1 = 2 as libc::c_int as libc::c_double * Randoms()
            - 1 as libc::c_int as libc::c_double;
        dTemp2 = 2 as libc::c_int as libc::c_double * Randoms()
            - 1 as libc::c_int as libc::c_double;
        dRacine = dTemp1 * dTemp1 + dTemp2 * dTemp2;
        if !(dRacine >= 1 as libc::c_int as libc::c_double
            || dRacine == 0 as libc::c_int as libc::c_double)
        {
            break;
        }
    }
    dTemp3 = sqrt(-(2 as libc::c_int) as libc::c_double * log(dRacine) / dRacine);
    vbSwitchGauss = 1 as libc::c_int;
    memGauss = dTemp1 * dTemp3;
    return dMean + dSD * (dTemp2 * dTemp3);
}
pub unsafe extern "C" fn PiecewiseRandom(
    mut min: libc::c_double,
    mut a_0: libc::c_double,
    mut b: libc::c_double,
    mut max: libc::c_double,
) -> libc::c_double {
    let mut dTemp: libc::c_double = 0.;
    static mut Grille: [libc::c_double; 4] = [0.; 4];
    static mut densite: [libc::c_double; 4] = [0.; 4];
    static mut densiteCum: [libc::c_double; 4] = [0.; 4];
    let mut nvlle_densite: libc::c_double = 0.;
    Grille[0 as libc::c_int as usize] = min;
    Grille[1 as libc::c_int as usize] = a_0;
    Grille[2 as libc::c_int as usize] = b;
    Grille[3 as libc::c_int as usize] = max;
    densite[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    densite[1 as libc::c_int
        as usize] = 1 as libc::c_int as libc::c_double
        / (max / 2 as libc::c_int as libc::c_double
            + b / 2 as libc::c_int as libc::c_double
            - a_0 / 2 as libc::c_int as libc::c_double
            - min / 2 as libc::c_int as libc::c_double);
    densite[2 as libc::c_int
        as usize] = 1 as libc::c_int as libc::c_double
        / (max / 2 as libc::c_int as libc::c_double
            + b / 2 as libc::c_int as libc::c_double
            - a_0 / 2 as libc::c_int as libc::c_double
            - min / 2 as libc::c_int as libc::c_double);
    densite[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
    CalcCumulative(
        4 as libc::c_int as libc::c_long,
        Grille.as_mut_ptr(),
        densite.as_mut_ptr(),
        densiteCum.as_mut_ptr(),
        1 as libc::c_int,
    );
    dTemp = PiecewiseVariate(
        4 as libc::c_int as libc::c_long,
        Grille.as_mut_ptr(),
        densite.as_mut_ptr(),
        densiteCum.as_mut_ptr(),
        1 as libc::c_int,
        &mut nvlle_densite,
    );
    return dTemp;
}
pub unsafe extern "C" fn PiecewiseVariate(
    mut cDim: libc::c_long,
    mut rg_x: *mut libc::c_double,
    mut rg_pdf: *mut libc::c_double,
    mut rg_Cdf: *mut libc::c_double,
    mut iOrder: libc::c_int,
    mut pVal_pdf: *mut libc::c_double,
) -> libc::c_double {
    let mut dPWVariate: libc::c_double = 0.;
    let mut dValPdf: libc::c_double = 0.;
    let mut dUniform: libc::c_double = UniformRandom(
        0 as libc::c_int as libc::c_double,
        *rg_Cdf.offset((cDim - 1 as libc::c_int as libc::c_long) as isize),
    );
    let mut lUpper: libc::c_long = 0;
    let mut lLower: libc::c_long = 0;
    let mut lIndex: libc::c_long = 0;
    if iOrder > 1 as libc::c_int {
        printf(
            b"CalcCumulative: Order %d not supported-> using piecewise-linear\n\0"
                as *const u8 as *const libc::c_char,
            iOrder,
        );
        iOrder = 1 as libc::c_int;
    }
    lUpper = cDim;
    lLower = 0 as libc::c_int as libc::c_long;
    lIndex = 0 as libc::c_int as libc::c_long;
    while lUpper - lLower > 1 as libc::c_int as libc::c_long {
        lIndex = (lUpper + lLower) / 2 as libc::c_int as libc::c_long;
        if dUniform > *rg_Cdf.offset(lIndex as isize) {
            lLower = lIndex;
        } else if dUniform < *rg_Cdf.offset(lIndex as isize) {
            lUpper = lIndex;
        } else {
            lLower = lIndex;
            lUpper = lLower;
        }
    }
    if lUpper == lLower {
        dValPdf = *rg_pdf.offset(lLower as isize);
        dPWVariate = *rg_x.offset(lLower as isize);
    } else {
        match iOrder {
            0 => {
                dValPdf = *rg_pdf.offset(lLower as isize);
                dPWVariate = InterpolateX(rg_x, rg_Cdf, lLower, dUniform);
            }
            1 => {
                if *rg_pdf.offset(lLower as isize) == *rg_pdf.offset(lUpper as isize) {
                    dValPdf = *rg_pdf.offset(lLower as isize);
                    dPWVariate = InterpolateX(rg_x, rg_Cdf, lLower, dUniform);
                } else {
                    let mut a_0: libc::c_double = 0.;
                    let mut b: libc::c_double = 0.;
                    let mut c: libc::c_double = 0.;
                    let mut dRadical: libc::c_double = 0.;
                    a_0 = (*rg_pdf.offset(lUpper as isize)
                        - *rg_pdf.offset(lLower as isize))
                        / (*rg_x.offset(lUpper as isize)
                            - *rg_x.offset(lLower as isize));
                    b = *rg_pdf.offset(lLower as isize)
                        - a_0 * *rg_x.offset(lLower as isize);
                    c = *rg_Cdf.offset(lLower as isize)
                        - (a_0 * *rg_x.offset(lLower as isize) / 2.0f64 + b)
                            * *rg_x.offset(lLower as isize);
                    dRadical = sqrt(
                        b * b - 2 as libc::c_int as libc::c_double * a_0 * (c - dUniform),
                    );
                    dPWVariate = (-b + dRadical) / a_0;
                    if dPWVariate >= *rg_x.offset(lLower as isize)
                        && dPWVariate <= *rg_x.offset(lUpper as isize)
                    {} else {
                        __assert_fail(
                            b"dPWVariate >= rg_x[lLower] && dPWVariate <= rg_x[lUpper]\0"
                                as *const u8 as *const libc::c_char,
                            b"random.c\0" as *const u8 as *const libc::c_char,
                            1025 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 75],
                                &[libc::c_char; 75],
                            >(
                                b"double PiecewiseVariate(long, double *, double *, double *, int, double *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_6794: {
                        if dPWVariate >= *rg_x.offset(lLower as isize)
                            && dPWVariate <= *rg_x.offset(lUpper as isize)
                        {} else {
                            __assert_fail(
                                b"dPWVariate >= rg_x[lLower] && dPWVariate <= rg_x[lUpper]\0"
                                    as *const u8 as *const libc::c_char,
                                b"random.c\0" as *const u8 as *const libc::c_char,
                                1025 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 75],
                                    &[libc::c_char; 75],
                                >(
                                    b"double PiecewiseVariate(long, double *, double *, double *, int, double *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    dValPdf = a_0 * dPWVariate + b;
                    if a_0 > 0 as libc::c_int as libc::c_double {
                        if dValPdf >= *rg_pdf.offset(lLower as isize)
                            && dValPdf <= *rg_pdf.offset(lUpper as isize)
                        {} else {
                            __assert_fail(
                                b"dValPdf >= rg_pdf[lLower] && dValPdf <= rg_pdf[lUpper]\0"
                                    as *const u8 as *const libc::c_char,
                                b"random.c\0" as *const u8 as *const libc::c_char,
                                1030 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 75],
                                    &[libc::c_char; 75],
                                >(
                                    b"double PiecewiseVariate(long, double *, double *, double *, int, double *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_6712: {
                            if dValPdf >= *rg_pdf.offset(lLower as isize)
                                && dValPdf <= *rg_pdf.offset(lUpper as isize)
                            {} else {
                                __assert_fail(
                                    b"dValPdf >= rg_pdf[lLower] && dValPdf <= rg_pdf[lUpper]\0"
                                        as *const u8 as *const libc::c_char,
                                    b"random.c\0" as *const u8 as *const libc::c_char,
                                    1030 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 75],
                                        &[libc::c_char; 75],
                                    >(
                                        b"double PiecewiseVariate(long, double *, double *, double *, int, double *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                    } else {
                        if dValPdf <= *rg_pdf.offset(lLower as isize)
                            && dValPdf >= *rg_pdf.offset(lUpper as isize)
                        {} else {
                            __assert_fail(
                                b"dValPdf <= rg_pdf[lLower] && dValPdf >= rg_pdf[lUpper]\0"
                                    as *const u8 as *const libc::c_char,
                                b"random.c\0" as *const u8 as *const libc::c_char,
                                1032 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 75],
                                    &[libc::c_char; 75],
                                >(
                                    b"double PiecewiseVariate(long, double *, double *, double *, int, double *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_6643: {
                            if dValPdf <= *rg_pdf.offset(lLower as isize)
                                && dValPdf >= *rg_pdf.offset(lUpper as isize)
                            {} else {
                                __assert_fail(
                                    b"dValPdf <= rg_pdf[lLower] && dValPdf >= rg_pdf[lUpper]\0"
                                        as *const u8 as *const libc::c_char,
                                    b"random.c\0" as *const u8 as *const libc::c_char,
                                    1032 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 75],
                                        &[libc::c_char; 75],
                                    >(
                                        b"double PiecewiseVariate(long, double *, double *, double *, int, double *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                    }
                }
            }
            _ => {
                dValPdf = 0 as libc::c_int as libc::c_double;
                dPWVariate = 0 as libc::c_int as libc::c_double;
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"random.c\0" as *const u8 as *const libc::c_char,
                    1042 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 75],
                        &[libc::c_char; 75],
                    >(
                        b"double PiecewiseVariate(long, double *, double *, double *, int, double *)\0",
                    ))
                        .as_ptr(),
                );
                'c_6584: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"random.c\0" as *const u8 as *const libc::c_char,
                        1042 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 75],
                            &[libc::c_char; 75],
                        >(
                            b"double PiecewiseVariate(long, double *, double *, double *, int, double *)\0",
                        ))
                            .as_ptr(),
                    );
                };
            }
        }
    }
    if !pVal_pdf.is_null() {
        *pVal_pdf = dValPdf;
    }
    return dPWVariate;
}
pub unsafe extern "C" fn PoissonRandom(mut mu: libc::c_double) -> libc::c_long {
    let mut u1: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut lnfact: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    static mut prev_mu: libc::c_double = 0 as libc::c_int as libc::c_double;
    static mut c: libc::c_double = 0.;
    static mut beta: libc::c_double = 0.;
    static mut alpha: libc::c_double = 0.;
    static mut k: libc::c_double = 0.;
    let mut n: libc::c_long = 0 as libc::c_int as libc::c_long;
    if mu <= 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: negative or null rate for a Poisson variate - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if mu <= 60 as libc::c_int as libc::c_double {
        s = 1 as libc::c_int as libc::c_double;
        t = 1 as libc::c_int as libc::c_double;
        u1 = Randoms() * exp(mu);
        while s < u1 {
            n += 1;
            n;
            t = t * mu / n as libc::c_double;
            s = s + t;
        }
    } else {
        if mu != prev_mu {
            c = 0.767f64 - 3.36f64 / mu;
            beta = 3.1415926535897932384626433f64
                / sqrt(3 as libc::c_int as libc::c_double * mu);
            alpha = beta * mu;
            k = log(c) - mu - log(beta);
        }
        loop {
            loop {
                u1 = Randoms();
                x = (alpha - log((1 as libc::c_int as libc::c_double - u1) / u1)) / beta;
                if !(x <= -0.5f64) {
                    break;
                }
            }
            n = (x + 0.5f64) as libc::c_long;
            u2 = Randoms();
            lnfact = 0.918938533f64 - n as libc::c_double
                + (n as libc::c_double + 0.5f64) * log(n as libc::c_double);
            if !(alpha - beta * x
                + log(
                    u2
                        / pow(
                            1 as libc::c_int as libc::c_double + exp(alpha - beta * x),
                            2 as libc::c_int as libc::c_double,
                        ),
                ) > k + n as libc::c_double * log(mu) - lnfact)
            {
                break;
            }
        }
    }
    return n;
}
pub unsafe extern "C" fn StudentTRandom(
    mut dof: libc::c_double,
    mut dMean: libc::c_double,
    mut dSD: libc::c_double,
) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    if dof <= 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: StudentTRandom: dof <= 0\n\0" as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    z = NormalRandom(
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    );
    x = Chi2Random(dof);
    return dMean + dSD * z * sqrt(dof / x);
}
pub unsafe extern "C" fn TruncInvGGammaRandom(
    mut alpha: libc::c_double,
    mut beta: libc::c_double,
    mut a_0: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let mut X: libc::c_double = 0.0f64;
    let mut iter: libc::c_int = 0 as libc::c_int;
    if a_0 >= b {
        printf(
            b"TruncLogNormalRandom: min >= max  [%g %g]\n\0" as *const u8
                as *const libc::c_char,
            a_0,
            b,
        );
    } else {
        loop {
            iter += 1;
            if iter == 25 as libc::c_int {
                printf(
                    b"TruncInvGGammaRandom: problem with range: \0" as *const u8
                        as *const libc::c_char,
                );
                printf(
                    b"min %g, max %g, alpha %g, beta %g\n\0" as *const u8
                        as *const libc::c_char,
                    a_0,
                    b,
                    alpha,
                    beta,
                );
            }
            X = InvGGammaRandom(alpha, beta);
            if !(X < a_0 || X > b) {
                break;
            }
        }
    }
    return X;
}
pub unsafe extern "C" fn TruncLogNormalRandom(
    mut dGeoMean: libc::c_double,
    mut dGeoSD: libc::c_double,
    mut a_0: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return exp(TruncNormalRandom(log(dGeoMean), log(dGeoSD), log(a_0), log(b)));
}
pub unsafe extern "C" fn TruncLogNormalRandom_old(
    mut dGeoMean: libc::c_double,
    mut dGeoSD: libc::c_double,
    mut a_0: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let mut X: libc::c_double = 0.0f64;
    let mut iter: libc::c_int = 0 as libc::c_int;
    if a_0 >= b {
        printf(
            b"TruncLogNormalRandom: min >= max  [%g %g]\n\0" as *const u8
                as *const libc::c_char,
            a_0,
            b,
        );
    } else {
        loop {
            iter += 1;
            if iter == 25 as libc::c_int {
                printf(
                    b"TruncLogNormalRandom: problem with range: \0" as *const u8
                        as *const libc::c_char,
                );
                printf(
                    b"min %g, max %g, ave %g, sd %g\n\0" as *const u8
                        as *const libc::c_char,
                    a_0,
                    b,
                    dGeoMean,
                    dGeoSD,
                );
            }
            X = LogNormalRandom(dGeoMean, dGeoSD);
            if !(X < a_0 || X > b) {
                break;
            }
        }
    }
    return X;
}
pub unsafe extern "C" fn TruncNormalRandom(
    mut dMean: libc::c_double,
    mut dSD: libc::c_double,
    mut a_0: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let mut lower: libc::c_double = (a_0 - dMean) / dSD;
    let mut upper: libc::c_double = (b - dMean) / dSD;
    let mut dTmp: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if a_0 >= b {
        printf(
            b"Error: TruncNormalRandom: min >= max  [%g %g]\n\0" as *const u8
                as *const libc::c_char,
            a_0,
            b,
        );
        exit(0 as libc::c_int);
    }
    if lower < 0 as libc::c_int as libc::c_double
        && upper > 0 as libc::c_int as libc::c_double
        && upper - lower > 2.506628274631000241612f64
    {
        loop {
            y = NormalRandom(
                0 as libc::c_int as libc::c_double,
                1 as libc::c_int as libc::c_double,
            );
            if !(y < lower || y > upper) {
                break;
            }
        }
    } else {
        dTmp = lower + sqrt(lower * lower + 4 as libc::c_int as libc::c_double);
        if lower >= 0 as libc::c_int as libc::c_double
            && upper
                > lower
                    + 3.297442541400256388329f64 / dTmp
                        * exp(
                            lower
                                * (2 as libc::c_int as libc::c_double
                                    - sqrt(lower * lower + 4 as libc::c_int as libc::c_double))
                                * 0.25f64,
                        )
        {
            dTmp = dTmp * 0.5f64;
            loop {
                y = ExpRandom(dTmp) + lower;
                if !(Randoms() > exp(-(y - dTmp) * (y - dTmp) * 0.5f64) || y > upper) {
                    break;
                }
            }
        } else {
            dTmp = -upper + sqrt(upper * upper + 4 as libc::c_int as libc::c_double);
            if upper <= 0 as libc::c_int as libc::c_double
                && lower
                    < upper
                        - 3.297442541400256388329f64 / dTmp
                            * exp(
                                upper
                                    * (2 as libc::c_int as libc::c_double
                                        + sqrt(upper * upper + 4 as libc::c_int as libc::c_double))
                                    * 0.25f64,
                            )
            {
                dTmp = dTmp * 0.5f64;
                loop {
                    y = ExpRandom(dTmp) - upper;
                    if !(Randoms() > exp(-(y - dTmp) * (y - dTmp) * 0.5f64)
                        || y > -lower)
                    {
                        break;
                    }
                }
                y = -y;
            } else {
                loop {
                    y = UniformRandom(lower, upper);
                    if lower > 0 as libc::c_int as libc::c_double {
                        dTmp = exp((lower * lower - y * y) * 0.5f64);
                    } else if upper < 0 as libc::c_int as libc::c_double {
                        dTmp = exp((upper * upper - y * y) * 0.5f64);
                    } else {
                        dTmp = exp(-y * y * 0.5f64);
                    }
                    if !(Randoms() > dTmp) {
                        break;
                    }
                }
            }
        }
    }
    return y * dSD + dMean;
}
pub unsafe extern "C" fn TruncNormalRandom_old(
    mut dMean: libc::c_double,
    mut dSD: libc::c_double,
    mut a_0: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let mut X: libc::c_double = 0.0f64;
    let mut unif_density: libc::c_double = 0.;
    let mut normConstant: libc::c_double = 0.;
    let mut imp_ratio: libc::c_double = 0.;
    if a_0 >= b {
        printf(
            b"Error: TruncNormalRandom: min >= max  [%g %g]\n\0" as *const u8
                as *const libc::c_char,
            a_0,
            b,
        );
        exit(0 as libc::c_int);
    } else if (b - a_0) / dSD > 1.5f64 {
        loop {
            X = NormalRandom(dMean, dSD);
            if !(X < a_0 || X > b) {
                break;
            }
        }
    } else {
        normConstant = CDFNormal((b - dMean) / dSD) - CDFNormal((a_0 - dMean) / dSD);
        unif_density = if dMean < a_0 {
            DFNormal(a_0, dMean, dSD) / normConstant
        } else if dMean < b {
            DFNormal(dMean, dMean, dSD) / normConstant
        } else {
            DFNormal(b, dMean, dSD) / normConstant
        };
        loop {
            X = UniformRandom(a_0, b);
            imp_ratio = DFNormal(X, dMean, dSD) / (normConstant * unif_density);
            if !(imp_ratio < 1 as libc::c_int as libc::c_double && Randoms() > imp_ratio)
            {
                break;
            }
        }
    }
    return X;
}
pub unsafe extern "C" fn UniformRandom(
    mut a_0: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    if b < a_0 {
        printf(
            b"Error: bad range a for uniform variate - Exiting\n\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    return Randoms() * (b - a_0) + a_0;
}
pub unsafe extern "C" fn WishartRandom(
    mut n: libc::c_long,
    mut p: libc::c_long,
    mut t: *mut libc::c_double,
    mut w: *mut libc::c_double,
    mut work: *mut libc::c_double,
) {
    let mut eta: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut m_0: libc::c_long = 0;
    let mut k1: libc::c_long = 0;
    let mut k2: libc::c_long = 0;
    let mut k3: libc::c_long = 0;
    printf(
        b"WishartRandom not tested - Exiting...\0" as *const u8 as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn and(mut A: BOOL, mut B: BOOL) -> BOOL {
    return (A != 0 && B != 0) as libc::c_int;
}
pub unsafe extern "C" fn CalcCumulative(
    mut cDim: libc::c_long,
    mut rg_x: *mut libc::c_double,
    mut rg_pdf: *mut libc::c_double,
    mut rg_Cdf: *mut libc::c_double,
    mut iOrder: libc::c_int,
) {
    let mut i: libc::c_long = 0;
    if iOrder > 1 as libc::c_int {
        printf(
            b"CalcCumulative: Order %d not supported-> using piecewise-linear\n\0"
                as *const u8 as *const libc::c_char,
            iOrder,
        );
        iOrder = 1 as libc::c_int;
    }
    *rg_Cdf.offset(0 as libc::c_int as isize) = 0.0f64;
    match iOrder {
        0 => {
            i = 1 as libc::c_int as libc::c_long;
            while i < cDim {
                *rg_Cdf
                    .offset(
                        i as isize,
                    ) = *rg_Cdf.offset((i - 1 as libc::c_int as libc::c_long) as isize)
                    + *rg_pdf.offset(i as isize)
                        * (*rg_x.offset(i as isize)
                            - *rg_x
                                .offset((i - 1 as libc::c_int as libc::c_long) as isize));
                i += 1;
                i;
            }
        }
        1 => {
            i = 1 as libc::c_int as libc::c_long;
            while i < cDim {
                *rg_Cdf
                    .offset(
                        i as isize,
                    ) = *rg_Cdf.offset((i - 1 as libc::c_int as libc::c_long) as isize)
                    + (*rg_x.offset(i as isize)
                        - *rg_x.offset((i - 1 as libc::c_int as libc::c_long) as isize))
                        * (*rg_pdf.offset(i as isize)
                            + *rg_pdf
                                .offset((i - 1 as libc::c_int as libc::c_long) as isize))
                        / 2 as libc::c_int as libc::c_double;
                i += 1;
                i;
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"random.c\0" as *const u8 as *const libc::c_char,
                1476 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"void CalcCumulative(long, double *, double *, double *, int)\0"))
                    .as_ptr(),
            );
            'c_7297: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"random.c\0" as *const u8 as *const libc::c_char,
                    1476 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 61],
                        &[libc::c_char; 61],
                    >(b"void CalcCumulative(long, double *, double *, double *, int)\0"))
                        .as_ptr(),
                );
            };
        }
    };
}
pub unsafe extern "C" fn CDFNormal(mut z: libc::c_double) -> libc::c_double {
    let mut tmp: libc::c_double = 0.;
    tmp = z / 1.414213562373095145475f64;
    if tmp >= 0 as libc::c_int as libc::c_double {
        return 0.5f64 * (2 as libc::c_int as libc::c_double - erfc(tmp))
    } else {
        return 0.5f64 * erfc(-tmp)
    };
}
pub unsafe extern "C" fn erfc(mut x: libc::c_double) -> libc::c_double {
    let mut dAbsX: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut dVal: libc::c_double = 0.;
    dAbsX = fabs(x);
    if dAbsX > 20 as libc::c_int as libc::c_double {
        return (if x >= 0 as libc::c_int as libc::c_double {
            0 as libc::c_int
        } else {
            2 as libc::c_int
        }) as libc::c_double
    } else {
        t = 1 as libc::c_int as libc::c_double
            / (1 as libc::c_int as libc::c_double + 0.5f64 * dAbsX);
        dVal = t
            * exp(
                -dAbsX * dAbsX - 1.26551223f64
                    + t
                        * (1.00002368f64
                            + t
                                * (0.37409196f64
                                    + t
                                        * (0.09678418f64
                                            + t
                                                * (-0.18628806f64
                                                    + t
                                                        * (0.27886807f64
                                                            + t
                                                                * (-1.13520398f64
                                                                    + t
                                                                        * (1.48851587f64
                                                                            + t * (-0.82215223f64 + t * 0.17087277f64)))))))),
            );
        return if x >= 0 as libc::c_int as libc::c_double {
            dVal
        } else {
            2 as libc::c_int as libc::c_double - dVal
        };
    };
}
pub unsafe extern "C" fn DFNormal(
    mut x: libc::c_double,
    mut mu: libc::c_double,
    mut sd: libc::c_double,
) -> libc::c_double {
    if sd <= 0.0f64 {
        printf(
            b"Error: negative or null SD in DFNormal\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    let mut tmp: libc::c_double = (mu - x) / sd;
    return 0.398942280401432702863f64 / sd * exp(-0.5f64 * tmp * tmp);
}
pub unsafe extern "C" fn InterpolateX(
    mut rgX: *mut libc::c_double,
    mut rgY: *mut libc::c_double,
    mut lLower: libc::c_long,
    mut dY: libc::c_double,
) -> libc::c_double {
    return *rgX.offset(lLower as isize)
        + (dY - *rgY.offset(lLower as isize))
            * (*rgX.offset((lLower + 1 as libc::c_int as libc::c_long) as isize)
                - *rgX.offset(lLower as isize))
            / (*rgY.offset((lLower + 1 as libc::c_int as libc::c_long) as isize)
                - *rgY.offset(lLower as isize));
}
pub unsafe extern "C" fn lnDFBeta(
    mut x: libc::c_double,
    mut alpha: libc::c_double,
    mut beta: libc::c_double,
    mut min: libc::c_double,
    mut max: libc::c_double,
) -> libc::c_double {
    if max <= min {
        printf(
            b"Error: bad range for beta variate in lnDFBeta\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if alpha <= 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: bad alpha for beta variate in LnDensity\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if beta <= 0 as libc::c_int as libc::c_double {
        printf(
            b"Error: bad beta for beta variate in LnDensity\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    x = (x - min) / (max - min);
    return (alpha - 1 as libc::c_int as libc::c_double) * log(x)
        + (beta - 1 as libc::c_int as libc::c_double)
            * log(1 as libc::c_int as libc::c_double - x) + lnGamma(alpha + beta)
        - lnGamma(alpha) - lnGamma(beta) - log(max - min);
}
pub unsafe extern "C" fn lnDFNormal(
    mut x: libc::c_double,
    mut mu: libc::c_double,
    mut sd: libc::c_double,
) -> libc::c_double {
    if sd <= 0.0f64 {
        printf(
            b"Error: negative or null SD in lnDFNormal\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    let mut tmp: libc::c_double = (mu - x) / sd;
    return -0.918938533204672669541f64 - log(sd) - 0.5f64 * tmp * tmp;
}
pub unsafe extern "C" fn lnGamma(mut x: libc::c_double) -> libc::c_double {
    let mut dSeries: libc::c_double = 0.;
    let mut dTemp: libc::c_double = 0.;
    if x <= 0.0f64 {
        printf(
            b"Error: negative or null parameter for lnGamma function\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    dSeries = 1.000000000190015f64 + 76.18009172947146f64 / x
        - 86.50532032141677f64 / (x + 1 as libc::c_int as libc::c_double)
        + 24.01409824083091f64 / (x + 2 as libc::c_int as libc::c_double)
        - 1.231739572450155f64 / (x + 3 as libc::c_int as libc::c_double)
        + 1.20865097386617E-3f64 / (x + 4 as libc::c_int as libc::c_double)
        - 5.39523938495E-6f64 / (x + 5 as libc::c_int as libc::c_double);
    dTemp = x + 4.5f64;
    dTemp = -dTemp + (x - 0.5f64) * log(dTemp) + log(2.50662827465f64 * dSeries);
    return dTemp;
}
