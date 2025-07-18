use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn InitiVector(cVectors: libc::c_long) -> *mut libc::c_int;
    fn InitpdVector(cVectors: libc::c_long) -> *mut *mut libc::c_double;
    fn InitlVector(cVectors: libc::c_long) -> *mut libc::c_long;
    fn InitdVector(cVectors: libc::c_long) -> *mut libc::c_double;
    fn InitdMatrix(
        cVectors: libc::c_long,
        cElemsEach: libc::c_long,
    ) -> *mut *mut libc::c_double;
    fn Cholesky(
        prgdVariance: *mut PDOUBLE,
        prgdComponent: *mut PDOUBLE,
        lNparams: libc::c_long,
    ) -> libc::c_int;
    fn QueueListItem(plist: PLIST, pData: PVOID);
    fn InitList() -> PLIST;
    fn ForAllList3(
        plist: PLIST,
        pfiCallback: PFI_FORLISTCALLBACK3,
        pUserInfo1: PVOID,
        pUserInfo2: PVOID,
        pUserInfo3: PVOID,
    );
    fn SetVar(hVar: HVAR, dVal: libc::c_double) -> BOOL;
    fn SetInput(hVar: HVAR, pInputFnRecord: PIFN) -> BOOL;
    fn IsParm(hVar: HVAR) -> BOOL;
    fn IsInput(hVar: HVAR) -> BOOL;
    fn InitModel();
    fn Randoms() -> libc::c_double;
    fn NormalRandom(dMean: libc::c_double, dSD: libc::c_double) -> libc::c_double;
    fn TruncNormalRandom(
        dMean: libc::c_double,
        dSD: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn UniformRandom(a: libc::c_double, b: libc::c_double) -> libc::c_double;
    fn CDFNormal(z: libc::c_double) -> libc::c_double;
    fn lnDFNormal(
        x: libc::c_double,
        mu: libc::c_double,
        sd: libc::c_double,
    ) -> libc::c_double;
    fn lnGamma(x: libc::c_double) -> libc::c_double;
    fn lnDFBeta(
        x: libc::c_double,
        alpha: libc::c_double,
        beta: libc::c_double,
        min: libc::c_double,
        max: libc::c_double,
    ) -> libc::c_double;
    fn DoOneExperiment(pexp: PEXPERIMENT) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn ReportRunTimeError(_: PANALYSIS, _: WORD, _: ...);
    fn CalculateOneMCParm(pMCVar: PMCVAR) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type intptr_t = libc::c_long;
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
pub type PBUF = PSTR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagINPUTBUF {
    pub pfileIn: PFILE,
    pub pbufOrg: PBUF,
    pub pbufCur: PBUF,
    pub iLineNum: libc::c_int,
    pub iLNPrev: libc::c_int,
    pub cErrors: libc::c_int,
    pub pInfo: PVOID,
}
pub type PINPUTBUF = *mut tagINPUTBUF;
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
pub type PFI_FORLISTCALLBACK3 = Option::<
    unsafe extern "C" fn(PVOID, PVOID, PVOID, PVOID) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union tagUVAR {
    pub dVal: libc::c_double,
    pub pifn: PIFN,
}
pub type UVAR = tagUVAR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagVARMODIFICATION {
    pub hvar: HVAR,
    pub uvar: UVAR,
}
pub type PVARMOD = *mut tagVARMODIFICATION;
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
pub type INTSPEC = tagINTSPEC;
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
pub type OUTSPEC = tagOUTSPEC;
pub type POUTSPEC = *mut tagOUTSPEC;
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
pub type MCVAR = tagMCVAR;
pub type PMCVAR = *mut tagMCVAR;
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
pub type GIBBSDATA = tagGIBBSDATA;
pub type PGIBBSDATA = *mut tagGIBBSDATA;
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
pub type MONTECARLO = tagMONTECARLO;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagMODELINFO {
    pub nStates: libc::c_long,
    pub nModelVars: libc::c_long,
    pub pStateHvar: *mut HVAR,
    pub pdModelVars: PDOUBLE,
}
pub type MODELINFO = tagMODELINFO;
pub type PMODELINFO = *mut tagMODELINFO;
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
pub type EXPERIMENT = tagEXPERIMENT;
pub type PEXPERIMENT = *mut tagEXPERIMENT;
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
pub type PLEVEL = *mut tagLEVEL;
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
pub type PANALYSIS = *mut tagANALYSIS;
pub unsafe extern "C" fn CalculateMeanAndVariance(
    mut n: libc::c_long,
    mut x: libc::c_double,
    mut xi_bari: *mut libc::c_double,
    mut si_2i: *mut libc::c_double,
) {
    if n == 1 as libc::c_int as libc::c_long {
        *xi_bari = x;
        *si_2i = 0 as libc::c_int as libc::c_double;
        return;
    }
    let mut mTmp: libc::c_double = *xi_bari;
    *xi_bari = *xi_bari + (x - *xi_bari) / n as libc::c_double;
    *si_2i = *si_2i + (x - mTmp) * (x - *xi_bari);
}
pub unsafe extern "C" fn checkConvergence(
    mut nOut: libc::c_int,
    mut variableCount: libc::c_int,
    mut p_count: libc::c_int,
    mut meansForAll: *mut *mut libc::c_double,
    mut varsForAll: *mut *mut libc::c_double,
    mut Rhat: *mut libc::c_double,
) -> libc::c_int {
    let mut vi: libc::c_int = 0;
    let mut pi: libc::c_int = 0;
    let mut converged: libc::c_int = 0 as libc::c_int;
    let mut varsofvars: libc::c_double = 0.;
    let mut meansofmeans: libc::c_double = 0.;
    let mut varsofmeans: libc::c_double = 0.;
    let mut meansofvars: libc::c_double = 0.;
    vi = 0 as libc::c_int;
    while vi < variableCount {
        meansofmeans = 0.0f64;
        pi = 0 as libc::c_int;
        while pi < p_count {
            CalculateMeanAndVariance(
                (pi + 1 as libc::c_int) as libc::c_long,
                *(*meansForAll.offset(pi as isize)).offset(vi as isize),
                &mut meansofmeans,
                &mut varsofmeans,
            );
            CalculateMeanAndVariance(
                (pi + 1 as libc::c_int) as libc::c_long,
                *(*varsForAll.offset(pi as isize)).offset(vi as isize),
                &mut meansofvars,
                &mut varsofvars,
            );
            pi += 1;
            pi;
        }
        if meansofvars == 0 as libc::c_int as libc::c_double
            && varsofmeans == 0 as libc::c_int as libc::c_double
        {
            *Rhat = 1 as libc::c_int as libc::c_double;
            converged += 1;
            converged;
        } else {
            let mut s2: libc::c_double = ((nOut - 1 as libc::c_int) as libc::c_double
                * meansofvars + varsofmeans) / nOut as libc::c_double;
            let mut Vhat: libc::c_double = s2
                + varsofmeans / (nOut * p_count) as libc::c_double;
            *Rhat = Vhat / meansofvars;
            if *Rhat < 1.05f64 {
                converged += 1;
                converged;
            }
        }
        vi += 1;
        vi;
    }
    return converged;
}
pub unsafe extern "C" fn CollectConvInfo(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut mean_dest: *mut *mut libc::c_double = *args.offset(0 as libc::c_int as isize)
        as *mut *mut libc::c_double;
    let mut var_dest: *mut *mut libc::c_double = *args.offset(1 as libc::c_int as isize)
        as *mut *mut libc::c_double;
    let mut n: *mut libc::c_long = *args.offset(2 as libc::c_int as isize)
        as *mut libc::c_long;
    let mut i: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    i = 0 as libc::c_int as libc::c_long;
    while i < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(i as isize);
        **mean_dest = (*pMCVar).dVal_mean;
        **var_dest = (*pMCVar).dVal_var
            / (*n - 1 as libc::c_int as libc::c_long) as libc::c_double;
        *mean_dest = (*mean_dest).offset(1);
        *mean_dest;
        *var_dest = (*var_dest).offset(1);
        *var_dest;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn AnnounceMarkov(
    mut size: libc::c_int,
    mut nSimTypeFlag: libc::c_int,
    mut nIter: libc::c_long,
) {
    match nSimTypeFlag {
        0 => {
            printf(
                b"\nDoing %ld Metropolis within Gibbs simulation\0" as *const u8
                    as *const libc::c_char,
                nIter,
            );
            printf(
                if nIter != 1 as libc::c_int as libc::c_long {
                    b"s\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if size > 1 as libc::c_int {
                printf(
                    b" on each of %d processors\n\0" as *const u8 as *const libc::c_char,
                    size,
                );
            } else {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        1 => {
            printf(
                b"\nPrinting data and predictions for the last line of the restart file\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            printf(
                b"\nDoing %ld Metropolis-Hastings simulation\0" as *const u8
                    as *const libc::c_char,
                nIter,
            );
            printf(
                if nIter != 1 as libc::c_int as libc::c_long {
                    b"s\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if size > 1 as libc::c_int {
                printf(
                    b" on each of %d processors\n\0" as *const u8 as *const libc::c_char,
                    size,
                );
            } else {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        3 => {
            printf(
                b"\nDoing %ld Metropolis within Gibbs posterior tempered simulation\0"
                    as *const u8 as *const libc::c_char,
                nIter,
            );
            printf(
                if nIter != 1 as libc::c_int as libc::c_long {
                    b"s\n\0" as *const u8 as *const libc::c_char
                } else {
                    b"\n\0" as *const u8 as *const libc::c_char
                },
            );
        }
        4 => {
            printf(
                b"\nDoing %ld Metropolis within Gibbs likelihood tempered simulation\0"
                    as *const u8 as *const libc::c_char,
                nIter,
            );
            printf(
                if nIter != 1 as libc::c_int as libc::c_long {
                    b"s\n\0" as *const u8 as *const libc::c_char
                } else {
                    b"\n\0" as *const u8 as *const libc::c_char
                },
            );
        }
        5 => {
            printf(
                b"\nDoing Stochastic optimization\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn CalculateTotals(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut pdLnPrior: *mut libc::c_double = *args.offset(1 as libc::c_int as isize)
        as *mut libc::c_double;
    let mut n: libc::c_long = 0;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        *pdLnPrior += LnDensity(*((*plevel).rgpMCVars).offset(n as isize), panal);
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn CheckForFixed(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut n: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut pFVar: PVARMOD = 0 as *mut tagVARMODIFICATION;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
        m = 0 as libc::c_int as libc::c_long;
        while m < (*plevel).nFixedVars {
            pFVar = *((*plevel).rgpFixedVars).offset(m as isize);
            if (*pMCVar).hvar == (*pFVar).hvar {
                (*pMCVar).bIsFixed = 1 as libc::c_int;
                if IsInput((*pFVar).hvar) != 0 {
                    printf(
                        b"Error: a sampled parameter cannot be assigned an input\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                } else {
                    (*pMCVar).dVal = (*pFVar).uvar.dVal;
                }
            }
            m += 1;
            m;
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn CheckPrintStatements(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut pos: POUTSPEC = 0 as *mut tagOUTSPEC;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    if ((*plevel).pexpt).is_null() {
        return;
    }
    pos = &mut (*(*plevel).pexpt).os;
    j = 0 as libc::c_int as libc::c_long;
    while j < (*pos).nOutputs as libc::c_long {
        i = j + 1 as libc::c_int as libc::c_long;
        while i < (*pos).nOutputs as libc::c_long {
            if *((*pos).phvar_out).offset(j as isize)
                == *((*pos).phvar_out).offset(i as isize)
            {
                ReportRunTimeError(
                    panal,
                    (0x121 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    *((*pos).pszOutputNames).offset(i as isize),
                    b"Print\0" as *const u8 as *const libc::c_char,
                );
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    j = 0 as libc::c_int as libc::c_long;
    while j < (*pos).nData as libc::c_long {
        i = j + 1 as libc::c_int as libc::c_long;
        while i < (*pos).nData as libc::c_long {
            if *((*pos).phvar_dat).offset(j as isize)
                == *((*pos).phvar_dat).offset(i as isize)
            {
                ReportRunTimeError(
                    panal,
                    (0x121 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    *((*pos).pszDataNames).offset(i as isize),
                    b"Data\0" as *const u8 as *const libc::c_char,
                );
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    j = 0 as libc::c_int as libc::c_long;
    while j < (*pos).nOutputs as libc::c_long {
        i = 0 as libc::c_int as libc::c_long;
        while i < (*pos).nData as libc::c_long {
            if *((*pos).phvar_out).offset(j as isize)
                == *((*pos).phvar_dat).offset(i as isize)
                && *((*pos).pcOutputTimes).offset(j as isize)
                    != *((*pos).pcData).offset(i as isize)
            {
                printf(
                    b"\nError: unequal times in Print and Data statements for %s\nExiting.\n\n\0"
                        as *const u8 as *const libc::c_char,
                    *((*pos).pszOutputNames).offset(j as isize),
                );
                exit(0 as libc::c_int);
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn CheckAllTransitions(mut pgd: PGIBBSDATA) -> BOOL {
    let mut bOK: BOOL = 0;
    let mut AcceptRate: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    i = (*pgd).startT;
    bOK = 1 as libc::c_int;
    while i <= (*pgd).endT - 1 as libc::c_int && bOK != 0 {
        if *((*pgd).rglTransAttempts).offset(i as isize)
            < 10 as libc::c_int as libc::c_long
        {
            bOK = 0 as libc::c_int;
            break;
        } else {
            AcceptRate = *((*pgd).rglTransAccepts).offset(i as isize) as libc::c_double
                / *((*pgd).rglTransAttempts).offset(i as isize) as libc::c_double;
            bOK = (AcceptRate > 0.15f64) as libc::c_int;
            i += 1;
            i;
        }
    }
    return bOK;
}
pub unsafe extern "C" fn CheckTransitions(mut pgd: PGIBBSDATA) -> libc::c_int {
    let mut AcceptRate: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    i = (*pgd).startT;
    if *((*pgd).rglTransAttempts).offset(i as isize) < 10 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int)
    } else {
        AcceptRate = *((*pgd).rglTransAccepts).offset(i as isize) as libc::c_double
            / *((*pgd).rglTransAttempts).offset(i as isize) as libc::c_double;
    }
    if AcceptRate < 0.30f64 {
        return -(1 as libc::c_int)
    } else if AcceptRate < 1 as libc::c_int as libc::c_double {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
pub unsafe extern "C" fn CloneLikes(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut nLikes: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut pLower: PLEVEL = 0 as *mut tagLEVEL;
    let mut pClone: PMCVAR = 0 as *mut tagMCVAR;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut bFound: BOOL = 0;
    i = 0 as libc::c_int as libc::c_long;
    while i < (*plevel).iInstances as libc::c_long {
        pLower = (*plevel).pLevels[i as usize];
        nLikes = (*plevel).nLikes
            + (if !((*plevel).plistLikes).is_null() {
                (*(*plevel).plistLikes).iSize
            } else {
                0 as libc::c_int
            }) as libc::c_long;
        (*pLower).nLikes = nLikes;
        if (*pLower).nLikes != 0 as libc::c_int as libc::c_long {
            (*pLower)
                .rgpLikes = malloc(
                ((*pLower).nLikes as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<PMCVAR>() as libc::c_ulong),
            ) as *mut PMCVAR;
            if ((*pLower).rgpLikes).is_null() {
                ReportError(
                    0 as PINPUTBUF,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"CloneLikes\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
        }
        i += 1;
        i;
    }
    nLikes = 0 as libc::c_int as libc::c_long;
    ForAllList3(
        (*plevel).plistLikes,
        Some(CloneLikesL as unsafe extern "C" fn(PVOID, PVOID, PVOID, PVOID) -> ()),
        plevel as PVOID,
        &mut nLikes as *mut libc::c_long as PVOID,
        0 as *mut libc::c_void,
    );
    i = 0 as libc::c_int as libc::c_long;
    while i < (*plevel).iInstances as libc::c_long {
        pLower = (*plevel).pLevels[i as usize];
        j = 0 as libc::c_int as libc::c_long;
        while j < (*plevel).nLikes {
            pMCVar = *((*plevel).rgpLikes).offset(j as isize);
            bFound = 0 as libc::c_int;
            k = 0 as libc::c_int as libc::c_long;
            while k < nLikes && bFound == 0 {
                bFound = ((*pMCVar).hvar
                    == (**((*pLower).rgpLikes).offset(k as isize)).hvar) as libc::c_int;
                if bFound == 0 {
                    k += 1;
                    k;
                }
            }
            if bFound == 0 {
                pClone = malloc(::std::mem::size_of::<MCVAR>() as libc::c_ulong)
                    as PMCVAR;
                if pClone.is_null() {
                    ReportError(
                        0 as PINPUTBUF,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"CloneLikes\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                memcpy(
                    pClone as *mut libc::c_void,
                    pMCVar as *const libc::c_void,
                    ::std::mem::size_of::<MCVAR>() as libc::c_ulong,
                );
                let ref mut fresh0 = *((*pLower).rgpLikes).offset((nLikes + j) as isize);
                *fresh0 = pClone;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn CloneLikesL(
    mut pData: PVOID,
    mut pUser1: PVOID,
    mut pUser2: PVOID,
    mut pUser3: PVOID,
) {
    let mut pMCVar: PMCVAR = pData as PMCVAR;
    let mut plevel: PLEVEL = pUser1 as PLEVEL;
    let mut pnLikes: *mut libc::c_long = pUser2 as *mut libc::c_long;
    let mut i: libc::c_long = 0;
    let mut pLower: PLEVEL = 0 as *mut tagLEVEL;
    let mut pClone: PMCVAR = 0 as *mut tagMCVAR;
    (*pMCVar).iDepth += 1;
    (*pMCVar).iDepth;
    i = 0 as libc::c_int as libc::c_long;
    while i < (*plevel).iInstances as libc::c_long {
        pLower = (*plevel).pLevels[i as usize];
        pClone = malloc(::std::mem::size_of::<MCVAR>() as libc::c_ulong) as PMCVAR;
        if pClone.is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"CloneLikeL\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        memcpy(
            pClone as *mut libc::c_void,
            pMCVar as *const libc::c_void,
            ::std::mem::size_of::<MCVAR>() as libc::c_ulong,
        );
        let ref mut fresh1 = *((*pLower).rgpLikes).offset(*pnLikes as isize);
        *fresh1 = pClone;
        i += 1;
        i;
    }
    *pnLikes += 1;
    *pnLikes;
}
pub unsafe extern "C" fn CloneMCVars(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut nMCVars: libc::c_long = (if !((*plevel).plistMCVars).is_null() {
        (*(*plevel).plistMCVars).iSize
    } else {
        0 as libc::c_int
    }) as libc::c_long;
    let mut n: libc::c_long = 0;
    let mut pLower: PLEVEL = 0 as *mut tagLEVEL;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).iInstances as libc::c_long {
        pLower = (*plevel).pLevels[n as usize];
        (*pLower).nMCVars = nMCVars;
        if nMCVars != 0 as libc::c_int as libc::c_long {
            (*pLower)
                .rgpMCVars = malloc(
                (nMCVars as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<PMCVAR>() as libc::c_ulong),
            ) as *mut PMCVAR;
            if ((*pLower).rgpMCVars).is_null() {
                ReportError(
                    0 as PINPUTBUF,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"CloneMCVars\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
        }
        n += 1;
        n;
    }
    nMCVars = 0 as libc::c_int as libc::c_long;
    ForAllList3(
        (*plevel).plistMCVars,
        Some(CloneMCVarsL as unsafe extern "C" fn(PVOID, PVOID, PVOID, PVOID) -> ()),
        plevel as PVOID,
        &mut nMCVars as *mut libc::c_long as PVOID,
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn CloneMCVarsL(
    mut pData: PVOID,
    mut pUser1: PVOID,
    mut pUser2: PVOID,
    mut pUser3: PVOID,
) {
    let mut pMCVar: PMCVAR = pData as PMCVAR;
    let mut plevel: PLEVEL = pUser1 as PLEVEL;
    let mut pnMCVars: *mut libc::c_long = pUser2 as *mut libc::c_long;
    let mut i: libc::c_long = 0;
    let mut pLower: PLEVEL = 0 as *mut tagLEVEL;
    let mut pClone: PMCVAR = 0 as *mut tagMCVAR;
    (*pMCVar).iDepth += 1;
    (*pMCVar).iDepth;
    i = 0 as libc::c_int as libc::c_long;
    while i < (*plevel).iInstances as libc::c_long {
        pLower = (*plevel).pLevels[i as usize];
        pClone = malloc(::std::mem::size_of::<MCVAR>() as libc::c_ulong) as PMCVAR;
        if pClone.is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"CloneMCVarsL\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        memcpy(
            pClone as *mut libc::c_void,
            pMCVar as *const libc::c_void,
            ::std::mem::size_of::<MCVAR>() as libc::c_ulong,
        );
        (*pClone).plistDependents = InitList();
        let ref mut fresh2 = *((*pLower).rgpMCVars).offset(*pnMCVars as isize);
        *fresh2 = pClone;
        i += 1;
        i;
    }
    *pnMCVars += 1;
    *pnMCVars;
}
pub unsafe extern "C" fn CloseMarkovFiles(mut pgd: PGIBBSDATA) {
    if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long
        || (*pgd).nSimTypeFlag == 4 as libc::c_int as libc::c_long
    {
        let mut szFileName: [libc::c_char; 206] = [0; 206];
        sprintf(
            szFileName.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (*pgd).szGout,
            b".perks\0" as *const u8 as *const libc::c_char,
        );
        fclose((*pgd).pfilePerks);
        printf(
            b"\nWrote perks to \"%s\"\n\0" as *const u8 as *const libc::c_char,
            szFileName.as_mut_ptr(),
        );
    }
    if !((*pgd).pfileOut).is_null() {
        fclose((*pgd).pfileOut);
        printf(
            b"Wrote MCMC sample to \"%s\"\n\0" as *const u8 as *const libc::c_char,
            (*pgd).szGout,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn ConvertLists(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut m: libc::c_long = 0;
    let mut n: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    if ((*plevel).pexpt).is_null() {
        ListToPVArray(
            panal,
            (*plevel).plistVars,
            &mut (*plevel).nFixedVars,
            &mut (*plevel).rgpFixedVars,
        );
    } else {
        ListToPVArray(
            panal,
            (*(*plevel).pexpt).plistParmMods,
            &mut (*plevel).nFixedVars,
            &mut (*plevel).rgpFixedVars,
        );
    }
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
        ListToPMCArray(
            panal,
            (*pMCVar).plistDependents,
            &mut (*pMCVar).nDependents,
            &mut (*pMCVar).rgpDependents,
        );
        if (*pMCVar).nDependents == 0 as libc::c_int as libc::c_long {
            (*pMCVar).bExptIsDep = 1 as libc::c_int;
        } else {
            m = 0 as libc::c_int as libc::c_long;
            while m < (*pMCVar).nDependents
                && {
                    (*pMCVar)
                        .bExptIsDep = (if strcmp(
                        (**((*pMCVar).rgpDependents).offset(m as isize)).pszName
                            as *const libc::c_char,
                        (*pMCVar).pszName as *const libc::c_char,
                    ) != 0
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    });
                    (*pMCVar).bExptIsDep == 0
                }
            {
                m += 1;
                m;
            }
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn DoMarkov(mut panal: PANALYSIS) {
    let mut pgd: PGIBBSDATA = &mut (*panal).gd;
    let mut pLevel0: PLEVEL = (*panal).pLevels[0 as libc::c_int as usize];
    let mut nThetas: libc::c_long = 0;
    let mut nUpdateAt: libc::c_long = 0;
    let mut nTotal: libc::c_long = 0;
    let mut i: libc::c_long = 0;
    let mut iter: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut nIter: libc::c_long = (*pgd).nMaxIter;
    let mut pdMCVarVals: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdSum: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut prgdSumProd: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut dTmp: libc::c_double = 0.;
    let mut dLnPrior: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dLnData: libc::c_double = 0 as libc::c_int as libc::c_double;
    if (*panal).rank == 0 as libc::c_int {
        AnnounceMarkov((*panal).size, (*pgd).nSimTypeFlag as libc::c_int, nIter);
    }
    OpenMarkovFiles(panal);
    ReadDataFile(panal);
    TraverseLevels(
        pLevel0,
        Some(CloneMCVars as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> ()),
        0 as *mut libc::c_void,
    );
    TraverseLevels(
        pLevel0,
        Some(CloneLikes as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> ()),
        0 as *mut libc::c_void,
    );
    TraverseLevels(
        pLevel0,
        Some(
            FindMCParents as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
        ),
        panal,
        0 as *mut libc::c_void,
    );
    TraverseLevels(
        pLevel0,
        Some(
            FindMCDependents
                as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
        ),
        panal,
        0 as *mut libc::c_void,
    );
    TraverseLevels(
        pLevel0,
        Some(
            FindLikeParents as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
        ),
        panal,
        0 as *mut libc::c_void,
    );
    TraverseLevels(
        pLevel0,
        Some(ConvertLists as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> ()),
        panal,
        0 as *mut libc::c_void,
    );
    TraverseLevels(
        pLevel0,
        Some(
            CheckForFixed as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
        ),
        0 as *mut libc::c_void,
    );
    TraverseLevels(
        pLevel0,
        Some(
            CheckPrintStatements
                as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
        ),
        panal,
        0 as *mut libc::c_void,
    );
    if (*panal).rank == 0 as libc::c_int && (*panal).bDependents != 0 {
        printf(b"Hierarchical structure:\n\n\0" as *const u8 as *const libc::c_char);
        TraverseLevels(
            pLevel0,
            Some(
                PrintDeps as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            0 as *mut libc::c_void,
        );
        printf(b"\nDone.\n\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    TraverseLevels(
        pLevel0,
        Some(SetPointers as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> ()),
        panal,
        0 as *mut libc::c_void,
    );
    if !((*pgd).szGrestart).is_null() {
        nThetas = 0 as libc::c_int as libc::c_long;
        TraverseLevels(
            pLevel0,
            Some(
                GetNumberOfMCVars
                    as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            &mut nThetas as *mut libc::c_long,
        );
        if ((*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long
            || (*pgd).nSimTypeFlag == 4 as libc::c_int as libc::c_long)
            && (*pgd).nPerks != 0 as libc::c_int
        {
            ReadRestartTemper(
                (*pgd).pfileRestart,
                nThetas,
                (*pgd).nPerks,
                &mut pdMCVarVals,
                &mut pdSum,
                &mut prgdSumProd,
                &mut iter,
                &mut (*pgd).indexT,
                (*pgd).rgdlnPi,
            );
        } else {
            ReadRestart(
                (*pgd).pfileRestart,
                nThetas,
                &mut pdMCVarVals,
                &mut pdSum,
                &mut prgdSumProd,
                &mut iter,
            );
        }
        nThetas = 0 as libc::c_int as libc::c_long;
        if TraverseLevels1(
            pLevel0,
            Some(
                SetMCVars
                    as unsafe extern "C" fn(
                        PLEVEL,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            pdMCVarVals,
            &mut nThetas as *mut libc::c_long,
            0 as *mut libc::c_void,
        ) == 0
        {
            printf(
                b"\nError: some read-in parameters are out of bounds - Exiting\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
        if (*pgd).nSimTypeFlag == 1 as libc::c_int as libc::c_long {
            if (*panal).rank == 0 as libc::c_int {
                PrintAllExpts(pLevel0, panal, (*pgd).pfileOut);
                CloseMarkovFiles(pgd);
            }
            return;
        }
        if (*pgd).nSimTypeFlag == 0 as libc::c_int as libc::c_long
            || (*pgd).nSimTypeFlag >= 3 as libc::c_int as libc::c_long
        {
            let mut szKernelFile: [libc::c_char; 212] = [0; 212];
            if (*panal).size > 1 as libc::c_int {
                sprintf(
                    szKernelFile.as_mut_ptr(),
                    b"%04d_%s%s\0" as *const u8 as *const libc::c_char,
                    (*panal).rank,
                    (*panal).gd.szGrestart,
                    b".kernel\0" as *const u8 as *const libc::c_char,
                );
            } else {
                sprintf(
                    szKernelFile.as_mut_ptr(),
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    (*panal).gd.szGrestart,
                    b".kernel\0" as *const u8 as *const libc::c_char,
                );
            }
            let mut pfile: *mut FILE = fopen(
                szKernelFile.as_mut_ptr(),
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if !pfile.is_null() {
                printf(
                    b"Reading kernel file %s\n\0" as *const u8 as *const libc::c_char,
                    szKernelFile.as_mut_ptr(),
                );
                TraverseLevels(
                    pLevel0,
                    Some(
                        ReadKernel
                            as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
                    ),
                    pfile,
                    0 as *mut libc::c_void,
                );
            } else {
                TraverseLevels(
                    pLevel0,
                    Some(
                        SetKernel
                            as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
                    ),
                    1 as libc::c_int,
                    pdMCVarVals,
                    0 as *mut libc::c_void,
                );
                free(pdMCVarVals as *mut libc::c_void);
            }
        } else {
            TraverseLevels(
                pLevel0,
                Some(
                    CalculateTotals
                        as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
                ),
                panal,
                &mut dLnPrior as *mut libc::c_double,
                0 as *mut libc::c_void,
            );
        }
        if RunAllExpts(panal, &mut dLnData) == 0 {
            printf(
                b"\nError: cannot compute at the starting point - Exiting\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
        TraverseLevels1(
            pLevel0,
            Some(
                SaveLikelihoods
                    as unsafe extern "C" fn(
                        PLEVEL,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        InitPerks(panal);
        WriteHeader(panal);
    } else {
        if (*pgd).nSimTypeFlag == 1 as libc::c_int as libc::c_long {
            printf(
                b"\nError: a restart file must be given to print data and         predictions - Exiting.\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
        TraverseLevels(
            pLevel0,
            Some(
                SetKernel as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            2 as libc::c_int,
            pdMCVarVals,
            0 as *mut libc::c_void,
        );
        TraverseLevels(
            pLevel0,
            Some(
                InitMCVars as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            0 as *mut libc::c_void,
        );
        TraverseLevels(
            pLevel0,
            Some(
                CalculateTotals
                    as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            panal,
            &mut dLnPrior as *mut libc::c_double,
            0 as *mut libc::c_void,
        );
        if RunAllExpts(panal, &mut dLnData) == 0 {
            printf(
                b"\nError: cannot compute at the starting point - Exiting\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
        TraverseLevels1(
            pLevel0,
            Some(
                SaveLikelihoods
                    as unsafe extern "C" fn(
                        PLEVEL,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        InitPerks(panal);
        WriteHeader(panal);
        fprintf((*pgd).pfileOut, b"0\t\0" as *const u8 as *const libc::c_char);
        TraverseLevels(
            pLevel0,
            Some(
                WriteMCVars as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            (*pgd).pfileOut,
            0 as *mut libc::c_void,
        );
        if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long
            || (*pgd).nSimTypeFlag == 4 as libc::c_int as libc::c_long
        {
            fprintf(
                (*pgd).pfileOut,
                b"%d\t\0" as *const u8 as *const libc::c_char,
                (*pgd).indexT,
            );
            i = 0 as libc::c_int as libc::c_long;
            while i < (*pgd).nPerks as libc::c_long {
                fprintf(
                    (*pgd).pfileOut,
                    b"%e\t\0" as *const u8 as *const libc::c_char,
                    *((*pgd).rgdlnPi).offset(i as isize),
                );
                i += 1;
                i;
            }
        }
        fprintf(
            (*pgd).pfileOut,
            b"%e\t%e\t%e\n\0" as *const u8 as *const libc::c_char,
            dLnPrior,
            dLnData,
            dLnPrior + dLnData,
        );
        fflush((*pgd).pfileOut);
    }
    nTotal = 20 as libc::c_int as libc::c_long;
    nUpdateAt = iter + nTotal;
    while iter < nIter {
        if (*panal).bOutputIter != 0
            && (iter + 1 as libc::c_int as libc::c_long)
                % (*panal).nOutputFreq as libc::c_long
                == 0 as libc::c_int as libc::c_long
        {
            if (*panal).size > 1 as libc::c_int {
                printf(
                    b"Processor %d, Iteration %ld\n\0" as *const u8
                        as *const libc::c_char,
                    (*panal).rank,
                    iter + 1 as libc::c_int as libc::c_long,
                );
            } else {
                printf(
                    b"Iteration %ld\n\0" as *const u8 as *const libc::c_char,
                    iter + 1 as libc::c_int as libc::c_long,
                );
            }
            if iter == nIter - 1 as libc::c_int as libc::c_long {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        if (iter + 1 as libc::c_int as libc::c_long) % (*pgd).nPrintFreq
            == 0 as libc::c_int as libc::c_long
            && iter >= (*pgd).nMaxIter - (*pgd).nPrintIter
        {
            fprintf(
                (*pgd).pfileOut,
                b"%ld\t\0" as *const u8 as *const libc::c_char,
                iter + 1 as libc::c_int as libc::c_long,
            );
        }
        if (*pgd).nSimTypeFlag == 0 as libc::c_int as libc::c_long
            || (*pgd).nSimTypeFlag == 5 as libc::c_int as libc::c_long
        {
            TraverseLevels(
                pLevel0,
                Some(
                    SampleThetas
                        as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
                ),
                panal,
                pgd,
                &mut iter as *mut libc::c_long,
                &mut nUpdateAt as *mut libc::c_long,
                &mut nTotal as *mut libc::c_long,
                0 as *mut libc::c_void,
            );
            if (iter + 1 as libc::c_int as libc::c_long) % (*pgd).nPrintFreq
                == 0 as libc::c_int as libc::c_long
                && iter >= (*pgd).nMaxIter - (*pgd).nPrintIter
            {
                dLnPrior = 0.0f64;
                TraverseLevels(
                    pLevel0,
                    Some(
                        CalculateTotals
                            as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
                    ),
                    panal,
                    &mut dLnPrior as *mut libc::c_double,
                    0 as *mut libc::c_void,
                );
                dLnData = 0.0f64;
                TraverseLevels1(
                    pLevel0,
                    Some(
                        SumAllExpts
                            as unsafe extern "C" fn(
                                PLEVEL,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                    &mut dLnData as *mut libc::c_double,
                    0 as *mut libc::c_void,
                );
                fprintf(
                    (*pgd).pfileOut,
                    b"%e\t%e\t%e\n\0" as *const u8 as *const libc::c_char,
                    dLnPrior,
                    dLnData,
                    dLnPrior + dLnData,
                );
                fflush((*pgd).pfileOut);
            }
        } else if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long
            || (*pgd).nSimTypeFlag == 4 as libc::c_int as libc::c_long
        {
            TraverseLevels(
                pLevel0,
                Some(
                    SampleThetasTempered
                        as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
                ),
                panal,
                pgd,
                &mut iter as *mut libc::c_long,
                &mut nUpdateAt as *mut libc::c_long,
                &mut nTotal as *mut libc::c_long,
                &mut (*pgd).indexT as *mut libc::c_int,
                0 as *mut libc::c_void,
            );
            dLnPrior = 0.0f64;
            TraverseLevels(
                pLevel0,
                Some(
                    CalculateTotals
                        as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
                ),
                panal,
                &mut dLnPrior as *mut libc::c_double,
                0 as *mut libc::c_void,
            );
            dLnData = 0.0f64;
            TraverseLevels1(
                pLevel0,
                Some(
                    SumAllExpts
                        as unsafe extern "C" fn(
                            PLEVEL,
                            *mut *mut libc::c_char,
                        ) -> libc::c_int,
                ),
                &mut dLnData as *mut libc::c_double,
                0 as *mut libc::c_void,
            );
            dTmp = (*pgd).dCZero / (iter as libc::c_double + (*pgd).dNZero);
            i = 0 as libc::c_int as libc::c_long;
            while i < (*pgd).nPerks as libc::c_long {
                if i == (*pgd).indexT as libc::c_long {
                    *((*pgd).rgdlnPi).offset(i as isize) -= dTmp;
                } else {
                    *((*pgd).rgdlnPi).offset(i as isize)
                        += dTmp / (*pgd).nPerks as libc::c_double;
                }
                i += 1;
                i;
            }
            if (iter + 1 as libc::c_int as libc::c_long) % (*pgd).nPrintFreq
                == 0 as libc::c_int as libc::c_long
                && iter >= (*pgd).nMaxIter - (*pgd).nPrintIter
            {
                fprintf(
                    (*pgd).pfileOut,
                    b"%d\t\0" as *const u8 as *const libc::c_char,
                    (*pgd).indexT,
                );
                i = 0 as libc::c_int as libc::c_long;
                while i < (*pgd).nPerks as libc::c_long {
                    fprintf(
                        (*pgd).pfileOut,
                        b"%e\t\0" as *const u8 as *const libc::c_char,
                        *((*pgd).rgdlnPi).offset(i as isize),
                    );
                    i += 1;
                    i;
                }
                fprintf(
                    (*pgd).pfileOut,
                    b"%e\t%e\t%e\n\0" as *const u8 as *const libc::c_char,
                    dLnPrior,
                    dLnData,
                    dLnPrior + dLnData,
                );
                fflush((*pgd).pfileOut);
            }
            *((*pgd).rglCount)
                .offset(
                    (*pgd).indexT as isize,
                ) = *((*pgd).rglCount).offset((*pgd).indexT as isize)
                + 1 as libc::c_int as libc::c_long;
            (*pgd).indexT = SampleTemperature2(pgd, dLnPrior, dLnData) as libc::c_int;
        } else {
            SampleThetaVector(
                pLevel0,
                panal,
                nThetas,
                pdMCVarVals,
                pdSum,
                prgdSumProd,
                iter,
                nUpdateAt,
                nTotal,
                &mut dLnPrior,
                &mut dLnData,
            );
            if (iter + 1 as libc::c_int as libc::c_long) % (*pgd).nPrintFreq
                == 0 as libc::c_int as libc::c_long
                && iter >= (*pgd).nMaxIter - (*pgd).nPrintIter
            {
                i = 0 as libc::c_int as libc::c_long;
                while i < nThetas {
                    fprintf(
                        (*pgd).pfileOut,
                        b"%5g\t\0" as *const u8 as *const libc::c_char,
                        *pdMCVarVals.offset(i as isize),
                    );
                    i += 1;
                    i;
                }
                fprintf(
                    (*pgd).pfileOut,
                    b"%e\t%e\t%e\n\0" as *const u8 as *const libc::c_char,
                    dLnPrior,
                    dLnData,
                    dLnPrior + dLnData,
                );
                fflush((*pgd).pfileOut);
            }
        }
        if iter == nUpdateAt {
            nTotal = nTotal * 3 as libc::c_int as libc::c_long
                / 2 as libc::c_int as libc::c_long;
            nUpdateAt = iter + nTotal;
        }
        iter = iter + 1 as libc::c_int as libc::c_long;
    }
    if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long
        || (*pgd).nSimTypeFlag == 4 as libc::c_int as libc::c_long
    {
        PrintTemperatureDiagnostics((*pgd).pfilePerks, pgd);
    }
    if (*pgd).nSimTypeFlag == 0 as libc::c_int as libc::c_long
        || (*pgd).nSimTypeFlag >= 3 as libc::c_int as libc::c_long
    {
        let mut szKernelFile_0: [libc::c_char; 207] = [0; 207];
        sprintf(
            szKernelFile_0.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (*panal).gd.szGout,
            b".kernel\0" as *const u8 as *const libc::c_char,
        );
        let mut pfile_0: *mut FILE = fopen(
            szKernelFile_0.as_mut_ptr(),
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if pfile_0.is_null() {
            printf(
                b"Cannot create kernel saving file '%s'\n\0" as *const u8
                    as *const libc::c_char,
                (*panal).gd.szGdata,
            );
            exit(0 as libc::c_int);
        }
        TraverseLevels(
            pLevel0,
            Some(
                WriteKernel as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            pfile_0,
            0 as *mut libc::c_void,
        );
        fprintf(pfile_0, b"\n\0" as *const u8 as *const libc::c_char);
        fclose(pfile_0);
        printf(
            b"Wrote kernel SDs to \"%s\"\n\0" as *const u8 as *const libc::c_char,
            szKernelFile_0.as_mut_ptr(),
        );
    }
    CloseMarkovFiles(pgd);
}
pub unsafe extern "C" fn EqualSlopes(
    mut x: PDOUBLE,
    mut y: PDOUBLE,
    mut i: libc::c_int,
) -> BOOL {
    let mut s1: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    s1 = (*y.offset((i + 1 as libc::c_int) as isize) - *y.offset(i as isize))
        / (*x.offset((i + 1 as libc::c_int) as isize) - *x.offset(i as isize));
    s2 = (*y.offset((i + 2 as libc::c_int) as isize) - *y.offset(i as isize))
        / (*x.offset((i + 2 as libc::c_int) as isize) - *x.offset(i as isize));
    return (fabs(s2 / s1 - 1 as libc::c_int as libc::c_double) < 0.01f64) as libc::c_int;
}
pub unsafe extern "C" fn Extrapolate(
    mut pgd: PGIBBSDATA,
    mut dTargetX: libc::c_double,
    mut i1: libc::c_int,
    mut i2: libc::c_int,
) -> libc::c_double {
    return *((*pgd).rgdlnPi).offset(i1 as isize)
        - (*((*pgd).rgdPerks).offset(i1 as isize) - dTargetX)
            * (*((*pgd).rgdlnPi).offset(i2 as isize)
                - *((*pgd).rgdlnPi).offset(i1 as isize))
            / (*((*pgd).rgdPerks).offset(i2 as isize)
                - *((*pgd).rgdPerks).offset(i1 as isize));
}
pub unsafe extern "C" fn FindLikeParents(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    let mut n: libc::c_long = 0;
    let mut pPrevLev: PLEVEL = 0 as *mut tagLEVEL;
    let mut pMCVar1: PMCVAR = 0 as *mut tagMCVAR;
    let mut pMCVar2: PMCVAR = 0 as *mut tagMCVAR;
    let mut bFound: BOOL = 0;
    (*panal).pCurrentLevel[(*plevel).iDepth as usize] = plevel;
    k = 0 as libc::c_int as libc::c_long;
    while k < (*plevel).nLikes {
        pMCVar1 = *((*plevel).rgpLikes).offset(k as isize);
        l = 0 as libc::c_int as libc::c_long;
        while l < 4 as libc::c_int as libc::c_long {
            if (*pMCVar1).iParmType[l as usize] == 1 as libc::c_int {
                bFound = 0 as libc::c_int;
                m = 0 as libc::c_int as libc::c_long;
                while m < (*plevel).nMCVars {
                    pMCVar2 = *((*plevel).rgpMCVars).offset(m as isize);
                    if (*pMCVar1).hParm[l as usize] == (*pMCVar2).hvar {
                        (*pMCVar1).pMCVParent[l as usize] = pMCVar2;
                        bFound = 1 as libc::c_int;
                    }
                    m += 1;
                    m;
                }
                if bFound == 0 {
                    n = ((*plevel).iDepth - 1 as libc::c_int) as libc::c_long;
                    while n >= 0 as libc::c_int as libc::c_long {
                        pPrevLev = (*panal).pCurrentLevel[n as usize];
                        m = 0 as libc::c_int as libc::c_long;
                        while m < (*pPrevLev).nMCVars {
                            pMCVar2 = *((*pPrevLev).rgpMCVars).offset(m as isize);
                            if (*pMCVar1).hParm[l as usize] == (*pMCVar2).hvar {
                                (*pMCVar1).pMCVParent[l as usize] = pMCVar2;
                                bFound = 1 as libc::c_int;
                            }
                            m += 1;
                            m;
                        }
                        n -= 1;
                        n;
                    }
                }
                if bFound == 0 {
                    printf(
                        b"\nError: parent in position %ld of %s must be\n       declared before it when creating\n       sampling dependencies - Exiting.\n\n\0"
                            as *const u8 as *const libc::c_char,
                        l,
                        (*pMCVar1).pszName,
                    );
                    exit(0 as libc::c_int);
                }
            }
            l += 1;
            l;
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn FindMCDependents(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    i = 0 as libc::c_int as libc::c_long;
    while i < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(i as isize);
        j = 0 as libc::c_int as libc::c_long;
        while j < 4 as libc::c_int as libc::c_long {
            if !((*pMCVar).pMCVParent[j as usize]).is_null()
                && (*(*pMCVar).pMCVParent[j as usize]).hvar
                    == (*pMCVar).hParm[j as usize]
            {
                QueueListItem(
                    (*(*pMCVar).pMCVParent[j as usize]).plistDependents,
                    pMCVar as PVOID,
                );
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn FindMCParents(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    let mut n: libc::c_long = 0;
    let mut pPrevLev: PLEVEL = 0 as *mut tagLEVEL;
    let mut pMCVar1: PMCVAR = 0 as *mut tagMCVAR;
    let mut pMCVar2: PMCVAR = 0 as *mut tagMCVAR;
    let mut bFound: BOOL = 0;
    (*panal).pCurrentLevel[(*plevel).iDepth as usize] = plevel;
    k = 0 as libc::c_int as libc::c_long;
    while k < (*plevel).nMCVars {
        pMCVar1 = *((*plevel).rgpMCVars).offset(k as isize);
        l = 0 as libc::c_int as libc::c_long;
        while l < 4 as libc::c_int as libc::c_long {
            if (*pMCVar1).iParmType[l as usize] == 1 as libc::c_int {
                bFound = 0 as libc::c_int;
                m = 0 as libc::c_int as libc::c_long;
                while m < k {
                    pMCVar2 = *((*plevel).rgpMCVars).offset(m as isize);
                    if (*pMCVar1).hParm[l as usize] == (*pMCVar2).hvar {
                        (*pMCVar1).pMCVParent[l as usize] = pMCVar2;
                        bFound = 1 as libc::c_int;
                    }
                    m += 1;
                    m;
                }
                if bFound == 0 {
                    n = ((*plevel).iDepth - 1 as libc::c_int) as libc::c_long;
                    while n >= 0 as libc::c_int as libc::c_long {
                        pPrevLev = (*panal).pCurrentLevel[n as usize];
                        m = 0 as libc::c_int as libc::c_long;
                        while m < (*pPrevLev).nMCVars {
                            pMCVar2 = *((*pPrevLev).rgpMCVars).offset(m as isize);
                            if (*pMCVar1).hParm[l as usize] == (*pMCVar2).hvar {
                                (*pMCVar1).pMCVParent[l as usize] = pMCVar2;
                                bFound = 1 as libc::c_int;
                            }
                            m += 1;
                            m;
                        }
                        n -= 1;
                        n;
                    }
                }
                if bFound == 0 {
                    printf(
                        b"\nError: parent in position %ld of %s must be\n       declared before it when creating\n       sampling dependencies - Exiting.\n\n\0"
                            as *const u8 as *const libc::c_char,
                        l,
                        (*pMCVar1).pszName,
                    );
                    exit(0 as libc::c_int);
                }
            }
            l += 1;
            l;
        }
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn GetNumberOfMCVars(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut pnThetas: *mut libc::c_long = *args.offset(0 as libc::c_int as isize)
        as *mut libc::c_long;
    *pnThetas += (*plevel).nMCVars;
}
pub unsafe extern "C" fn InitMCVars(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut n: libc::c_long = 0;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        if (**((*plevel).rgpMCVars).offset(n as isize)).bIsFixed == 0 {
            CalculateOneMCParm(*((*plevel).rgpMCVars).offset(n as isize));
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn ListToPMCArray(
    mut panal: PANALYSIS,
    mut plist: PLIST,
    mut pnMCVars: *mut libc::c_long,
    mut rgpMCVars: *mut *mut PMCVAR,
) {
    *pnMCVars = (if !plist.is_null() { (*plist).iSize } else { 0 as libc::c_int })
        as libc::c_long;
    if *pnMCVars == 0 as libc::c_int as libc::c_long {
        return;
    }
    *rgpMCVars = malloc(
        (*pnMCVars as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<PMCVAR>() as libc::c_ulong),
    ) as *mut PMCVAR;
    if (*rgpMCVars).is_null() {
        ReportRunTimeError(
            panal,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ListToPMCArray\0" as *const u8 as *const libc::c_char,
        );
    }
    *pnMCVars = 0 as libc::c_int as libc::c_long;
    ForAllList3(
        plist,
        Some(ListToPMCArrayL as unsafe extern "C" fn(PVOID, PVOID, PVOID, PVOID) -> ()),
        pnMCVars as PVOID,
        *rgpMCVars as PVOID,
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn ListToPMCArrayL(
    mut pData: PVOID,
    mut pUser1: PVOID,
    mut pUser2: PVOID,
    mut pUser3: PVOID,
) {
    let mut pMCVar: PMCVAR = pData as PMCVAR;
    let mut pnMCVars: *mut libc::c_long = pUser1 as *mut libc::c_long;
    let mut rgpMCVars: *mut PMCVAR = pUser2 as *mut PMCVAR;
    let fresh3 = *pnMCVars;
    *pnMCVars = *pnMCVars + 1;
    let ref mut fresh4 = *rgpMCVars.offset(fresh3 as isize);
    *fresh4 = pMCVar;
}
pub unsafe extern "C" fn ListToPVArray(
    mut panal: PANALYSIS,
    mut plist: PLIST,
    mut pnFixedVars: *mut libc::c_long,
    mut rgpFixedVars: *mut *mut PVARMOD,
) {
    *pnFixedVars = (if !plist.is_null() { (*plist).iSize } else { 0 as libc::c_int })
        as libc::c_long;
    if *pnFixedVars == 0 as libc::c_int as libc::c_long {
        return;
    }
    *rgpFixedVars = malloc(
        (*pnFixedVars as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<PVARMOD>() as libc::c_ulong),
    ) as *mut PVARMOD;
    if (*rgpFixedVars).is_null() {
        ReportRunTimeError(
            panal,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ListToPVArray\0" as *const u8 as *const libc::c_char,
        );
    }
    *pnFixedVars = 0 as libc::c_int as libc::c_long;
    ForAllList3(
        plist,
        Some(ListToPVArrayL as unsafe extern "C" fn(PVOID, PVOID, PVOID, PVOID) -> ()),
        pnFixedVars as PVOID,
        *rgpFixedVars as PVOID,
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn ListToPVArrayL(
    mut pData: PVOID,
    mut pUser1: PVOID,
    mut pUser2: PVOID,
    mut pUser3: PVOID,
) {
    let mut pVar: PVARMOD = pData as PVARMOD;
    let mut pnFixedVars: *mut libc::c_long = pUser1 as *mut libc::c_long;
    let mut rgpFixedVars: *mut PVARMOD = pUser2 as *mut PVARMOD;
    let fresh5 = *pnFixedVars;
    *pnFixedVars = *pnFixedVars + 1;
    let ref mut fresh6 = *rgpFixedVars.offset(fresh5 as isize);
    *fresh6 = pVar;
}
pub unsafe extern "C" fn LnDensity(
    mut pMCVar: PMCVAR,
    mut panal: PANALYSIS,
) -> libc::c_double {
    let mut dTmp: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut density: libc::c_double = 0.;
    let mut dTmp2: libc::c_double = 0.;
    let mut dTmp3: libc::c_double = 0.;
    let mut dTmp4: libc::c_double = 0.;
    let mut dParm1: libc::c_double = *(*pMCVar).pdParm[0 as libc::c_int as usize];
    let mut dParm2: libc::c_double = *(*pMCVar).pdParm[1 as libc::c_int as usize];
    let mut dMin: libc::c_double = *(*pMCVar).pdParm[2 as libc::c_int as usize];
    let mut dMax: libc::c_double = *(*pMCVar).pdParm[3 as libc::c_int as usize];
    let mut dTheta: libc::c_double = (*pMCVar).dVal;
    let mut str: [libc::c_char; 10] = [0; 10];
    if (*pMCVar).iType == 18 as libc::c_int {
        if dTheta < 0 as libc::c_int as libc::c_double {
            printf(
                b"Error: variate out of bounds in LnDensity\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    } else if (*pMCVar).iType == 21 as libc::c_int
        || (*pMCVar).iType == 22 as libc::c_int
    {
        if dParm1 < 0 as libc::c_int as libc::c_double {
            printf(
                b"Error: parameter %g out of bounds in LnDensity\n\0" as *const u8
                    as *const libc::c_char,
                dParm1,
            );
            exit(0 as libc::c_int);
        }
    } else if dTheta > dMax || dTheta < dMin {
        return -1.0E+100f64
    }
    's_628: {
        let mut current_block_162: u64;
        match (*pMCVar).iType {
            0 => {
                if dTheta > dParm2 || dTheta < dParm1 {
                    return -1.0E+100f64;
                }
                if dParm2 <= dParm1 {
                    ReportRunTimeError(
                        panal,
                        (0x304 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        (*pMCVar).pszName,
                        b"LnDensity\0" as *const u8 as *const libc::c_char,
                    );
                }
                return -log(dParm2 - dParm1);
            }
            1 => {
                if dTheta > dParm2 || dTheta < dParm1 {
                    return -1.0E+100f64;
                }
                if dParm2 <= dParm1 {
                    ReportRunTimeError(
                        panal,
                        (0x304 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        (*pMCVar).pszName,
                        b"LnDensity\0" as *const u8 as *const libc::c_char,
                    );
                }
                return -log(dTheta * (dParm2 - dParm1));
            }
            14 => {
                dParm2 = sqrt(dParm2);
                return lnDFNormal(dTheta, dParm1, dParm2);
            }
            25 => {
                dParm2 = fabs(dParm1 * dParm2);
                return lnDFNormal(dTheta, dParm1, dParm2);
            }
            3 | 19 => return lnDFNormal(dTheta, dParm1, dParm2),
            15 => {
                dParm2 = exp(sqrt(dParm2));
                current_block_162 = 9100868043760823246;
            }
            4 => {
                current_block_162 = 9100868043760823246;
            }
            16 => {
                dParm2 = sqrt(dParm2);
                return lnDFNormal(dTheta, dParm1, dParm2)
                    - log(
                        CDFNormal((dMax - dParm1) / dParm2)
                            - CDFNormal((dMin - dParm1) / dParm2),
                    );
            }
            26 => {
                dParm2 = fabs(dParm1 * dParm2);
                return lnDFNormal(dTheta, dParm1, dParm2)
                    - log(
                        CDFNormal((dMax - dParm1) / dParm2)
                            - CDFNormal((dMin - dParm1) / dParm2),
                    );
            }
            5 => {
                if dParm2 <= 0.0f64 {
                    sprintf(
                        str.as_mut_ptr(),
                        b"%5.2e\0" as *const u8 as *const libc::c_char,
                        dParm2,
                    );
                    ReportRunTimeError(
                        panal,
                        (0x301 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        (*pMCVar).pszName,
                        str.as_mut_ptr(),
                        b"LnDensity\0" as *const u8 as *const libc::c_char,
                    );
                }
                return lnDFNormal(dTheta, dParm1, dParm2)
                    - log(
                        CDFNormal((dMax - dParm1) / dParm2)
                            - CDFNormal((dMin - dParm1) / dParm2),
                    );
            }
            17 => {
                dParm2 = exp(sqrt(dParm2));
                current_block_162 = 2694013637280298776;
            }
            6 => {
                current_block_162 = 2694013637280298776;
            }
            2 => return lnDFBeta(dTheta, dParm1, dParm2, dMin, dMax),
            7 => {
                dTmp = 0.5f64 * dParm1;
                return (dTmp - 1 as libc::c_int as libc::c_double) * log(dTheta)
                    - 0.5f64 * dTheta + dTmp * -6.9314718056E-01f64 - lnGamma(dTmp);
            }
            8 => {
                if dParm1 < 0 as libc::c_int as libc::c_double
                    || dParm1 > 1 as libc::c_int as libc::c_double
                {
                    printf(
                        b"Error: bad p for binomial variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                if dTheta > dParm2 {
                    return -1.0E+100f64;
                }
                dTmp = lnGamma(dParm2 + 1 as libc::c_int as libc::c_double)
                    - lnGamma(dTheta + 1 as libc::c_int as libc::c_double)
                    - lnGamma(dParm2 - dTheta + 1 as libc::c_int as libc::c_double);
                if dParm1 == 0 as libc::c_int as libc::c_double {
                    if dTheta != 0 as libc::c_int as libc::c_double {
                        return -1.0E+100f64;
                    }
                } else {
                    dTmp = dTmp + dTheta * log(dParm1);
                }
                if dParm1 == 1 as libc::c_int as libc::c_double {
                    if dParm2 - dTheta == 0 as libc::c_int as libc::c_double {
                        return dTmp
                    } else {
                        return -1.0E+100f64
                    }
                } else {
                    return dTmp
                        + (dParm2 - dTheta)
                            * log(1 as libc::c_int as libc::c_double - dParm1)
                }
            }
            28 => {
                if dParm2 < 0 as libc::c_int as libc::c_double
                    || dParm2 > 1 as libc::c_int as libc::c_double
                {
                    printf(
                        b"Error: bad p for negative binomial variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                dTmp = lnGamma(dTheta + dParm1)
                    - lnGamma(dTheta + 1 as libc::c_int as libc::c_double)
                    - lnGamma(dParm1);
                if dParm2 == 0 as libc::c_int as libc::c_double
                    && dTheta != 0 as libc::c_int as libc::c_double
                {
                    return -1.0E+100f64
                } else {
                    dTmp = dTmp + dTheta * log(dParm2);
                }
                if dParm2 == 1 as libc::c_int as libc::c_double {
                    if dParm1 == 0 as libc::c_int as libc::c_double {
                        return dTmp
                    } else {
                        return -1.0E+100f64
                    }
                } else {
                    return dTmp
                        + dParm1 * log(1 as libc::c_int as libc::c_double - dParm2)
                }
            }
            9 => {
                density = 2 as libc::c_int as libc::c_double
                    / (dMax + dParm2 - dParm1 - dMin);
                if dTheta <= dParm1 {
                    return log(density * (dTheta - dMin) / (dParm1 - dMin))
                } else if dTheta <= dParm2 {
                    return log(density)
                } else {
                    return log(density * (dMax - dTheta) / (dMax - dParm2))
                }
            }
            10 => {
                if dParm1 <= 0 as libc::c_int as libc::c_double {
                    printf(
                        b"Error: negative or null inverse scale (%g) for exponential variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                        dParm1,
                    );
                    exit(0 as libc::c_int);
                }
                return -dTheta * dParm1 + log(dParm1);
            }
            11 => {
                if dParm2 <= 0 as libc::c_int as libc::c_double {
                    printf(
                        b"Error: bad inv. scale for gamma variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                return (dParm1 - 1 as libc::c_int as libc::c_double) * log(dTheta)
                    - dParm2 * dTheta + dParm1 * log(dParm2) - lnGamma(dParm1);
            }
            20 => {
                printf(
                    b"Error: Truncated inverse gamma density cannot be evaluated\n\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(
                    b"       if the GNU Scientific Library is not installed\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            13 => {
                if dParm2 <= 0 as libc::c_int as libc::c_double {
                    printf(
                        b"Error: bad scale for inv. gamma variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                return (-dParm1 - 1 as libc::c_int as libc::c_double) * log(dTheta)
                    - dParm2 / dTheta + dParm1 * log(dParm2) - lnGamma(dParm1) - dTmp;
            }
            12 => {
                if dParm1 <= 0 as libc::c_int as libc::c_double {
                    printf(
                        b"Error: bad rate for Poisson variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                return dTheta * log(dParm1) - dParm1
                    - lnGamma(dTheta + 1 as libc::c_int as libc::c_double);
            }
            18 => {
                if dParm1 < 0 as libc::c_int as libc::c_double {
                    printf(
                        b"Error: bad expectation for BinomialBeta variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                if dParm2 <= 0 as libc::c_int as libc::c_double {
                    printf(
                        b"Error: bad alpha for BinomialBeta variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                if dMin <= 0 as libc::c_int as libc::c_double {
                    printf(
                        b"Error: bad beta for BinomialBeta variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                dTmp = floor(0.5f64 + dParm1 + dParm1 * dMin / dParm2);
                if dTheta > dTmp {
                    return -1.0E+100f64
                } else {
                    if dParm2 == 0.5f64 && dMin == 0.5f64 {
                        dTmp = lnGamma(0.5f64 + dTheta) + lnGamma(0.5f64 + dTmp - dTheta)
                            - lnGamma(dTheta + 1 as libc::c_int as libc::c_double)
                            - lnGamma(
                                dTmp - dTheta + 1 as libc::c_int as libc::c_double,
                            );
                    } else {
                        dTmp = lnGamma(dParm2 + dMin)
                            + lnGamma(dTmp + 1 as libc::c_int as libc::c_double)
                            + lnGamma(dParm2 + dTheta) + lnGamma(dMin + dTmp - dTheta)
                            - lnGamma(dTheta + 1 as libc::c_int as libc::c_double)
                            - lnGamma(dTmp - dTheta + 1 as libc::c_int as libc::c_double)
                            - lnGamma(dParm2) - lnGamma(dMin)
                            - lnGamma(dParm2 + dMin + dTmp);
                    }
                    return dTmp;
                }
            }
            21 => {
                if dParm1 < 0 as libc::c_int as libc::c_double {
                    printf(
                        b"Error: bad expectation for GenLogNormal variate in LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                dTmp = sqrt(
                    exp(pow(dMin, 2 as libc::c_int as libc::c_double))
                        * (exp(pow(dMin, 2 as libc::c_int as libc::c_double))
                            - 1 as libc::c_int as libc::c_double),
                );
                dTmp2 = pow(dParm2 / dTmp, 2 as libc::c_int as libc::c_double);
                dTmp3 = log(
                    dParm1
                        + sqrt(pow(dParm1, 2 as libc::c_int as libc::c_double) + dTmp2),
                );
                if dTheta < 0 as libc::c_int as libc::c_double
                    && dTmp2 < 0.01f64 * dTheta * dTheta
                {
                    dTmp4 = log(
                        dTmp2 / (-(2 as libc::c_int) as libc::c_double * dTheta)
                            * (1 as libc::c_int as libc::c_double
                                + 0.25f64 * dTmp2
                                    / pow(dTheta, 2 as libc::c_int as libc::c_double)),
                    );
                } else {
                    dTmp4 = log(
                        dTheta
                            + sqrt(
                                pow(dTheta, 2 as libc::c_int as libc::c_double) + dTmp2,
                            ),
                    );
                }
                return lnDFNormal(dTmp4, dTmp3, dTmp)
                    - 0.5f64
                        * log(pow(dTmp4, 2 as libc::c_int as libc::c_double) + dTmp2);
            }
            22 => {
                if dParm1 <= 0 as libc::c_int as libc::c_double {
                    printf(
                        b"Error: bad dof for Student-T variatein LnDensity\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                dTmp = (dParm1 + 1 as libc::c_int as libc::c_double)
                    / 2 as libc::c_int as libc::c_double;
                return lnGamma(dTmp)
                    - lnGamma(dParm1 / 2 as libc::c_int as libc::c_double)
                    - 0.5f64 * log(dParm1 * 3.1415926535897932384626433f64 * dMin * dMin)
                    - dTmp
                        * log(
                            1 as libc::c_int as libc::c_double
                                + pow(
                                    (dTheta - dParm2) / dMin,
                                    2 as libc::c_int as libc::c_double,
                                ) / dParm1,
                        );
            }
            23 => {
                return -1.144729885849400163877f64
                    - log(dParm1 + dTheta * dTheta / dParm1);
            }
            24 => {
                return -0.4515827052894548221396f64
                    - log(dParm1 + dTheta * dTheta / dParm1);
            }
            27 => return dParm1,
            _ => {
                ReportRunTimeError(
                    panal,
                    (0x305 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"LnDensity\0" as *const u8 as *const libc::c_char,
                );
                break 's_628;
            }
        }
        match current_block_162 {
            9100868043760823246 => {
                if dParm1 <= 0.0f64 {
                    sprintf(
                        str.as_mut_ptr(),
                        b"%5.2e\0" as *const u8 as *const libc::c_char,
                        dParm1,
                    );
                    ReportRunTimeError(
                        panal,
                        (0x303 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        (*pMCVar).pszName,
                        str.as_mut_ptr(),
                        b"LnDensity\0" as *const u8 as *const libc::c_char,
                    );
                }
                return lnDFNormal(log(dTheta), log(dParm1), log(dParm2)) - log(dTheta);
            }
            _ => {
                if dParm1 <= 0.0f64 {
                    sprintf(
                        str.as_mut_ptr(),
                        b"%5.2e\0" as *const u8 as *const libc::c_char,
                        dParm1,
                    );
                    ReportRunTimeError(
                        panal,
                        (0x303 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        (*pMCVar).pszName,
                        str.as_mut_ptr(),
                        b"LnDensity\0" as *const u8 as *const libc::c_char,
                    );
                }
                if dParm2 <= 1.0f64 {
                    sprintf(
                        str.as_mut_ptr(),
                        b"%5.2e\0" as *const u8 as *const libc::c_char,
                        dParm2,
                    );
                    ReportRunTimeError(
                        panal,
                        (0x302 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        (*pMCVar).pszName,
                        str.as_mut_ptr(),
                        b"LnDensity\0" as *const u8 as *const libc::c_char,
                    );
                }
                dTmp = log(dParm2);
                return lnDFNormal(log(dTheta), log(dParm1), dTmp) - log(dTheta)
                    - log(
                        CDFNormal(log(dMax / dParm1) / dTmp)
                            - CDFNormal(log(dMin / dParm1) / dTmp),
                    );
            }
        }
    }
    return 0.0f64;
}
pub unsafe extern "C" fn LnLike(
    mut pMCVar: PMCVAR,
    mut panal: PANALYSIS,
) -> libc::c_double {
    let mut n: libc::c_long = 0;
    let mut dDensity: libc::c_double = 0.;
    let mut dLnLike: libc::c_double = 0.0f64;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*pMCVar).nDependents {
        dDensity = LnDensity(*((*pMCVar).rgpDependents).offset(n as isize), panal);
        if dDensity == -1.0E+100f64 {
            return -1.0E+100f64
        } else {
            dLnLike += dDensity;
        }
        n += 1;
        n;
    }
    return dLnLike;
}
pub unsafe extern "C" fn LnLikeData(
    mut plevel: PLEVEL,
    mut panal: PANALYSIS,
) -> libc::c_double {
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut dLnLike: libc::c_double = 0.0f64;
    let mut dTmp: libc::c_double = 0.;
    let mut bMissData: BOOL = 0;
    let mut bMissOutp: BOOL = 0;
    static mut pdBase: [PDOUBLE; 4] = [0 as *const libc::c_double
        as *mut libc::c_double; 4];
    i = 0 as libc::c_int as libc::c_long;
    while i < (*plevel).nLikes {
        pMCVar = *((*plevel).rgpLikes).offset(i as isize);
        k = 0 as libc::c_int as libc::c_long;
        while k < 4 as libc::c_int as libc::c_long {
            pdBase[k as usize] = (*pMCVar).pdParm[k as usize];
            k += 1;
            k;
        }
        j = 0 as libc::c_int as libc::c_long;
        while j < (*pMCVar).lCount {
            (*pMCVar).dVal = *((*pMCVar).pdVal).offset(j as isize);
            if (*pMCVar).dVal != -(1 as libc::c_int) as libc::c_double {
                bMissData = 0 as libc::c_int;
                bMissOutp = 0 as libc::c_int;
                k = 0 as libc::c_int as libc::c_long;
                while k < 4 as libc::c_int as libc::c_long {
                    if (*pMCVar).iParmType[k as usize] == 2 as libc::c_int {
                        (*pMCVar)
                            .pdParm[k
                            as usize] = (pdBase[k as usize]).offset(j as isize);
                        bMissOutp = bMissOutp
                            + (*(*pMCVar).pdParm[k as usize]
                                == -1.7976931348623157e+308f64) as libc::c_int;
                    } else if (*pMCVar).iParmType[k as usize] == 3 as libc::c_int {
                        (*pMCVar)
                            .pdParm[k
                            as usize] = (pdBase[k as usize]).offset(j as isize);
                        bMissData = bMissData
                            + (*(*pMCVar).pdParm[k as usize]
                                == -(1 as libc::c_int) as libc::c_double) as libc::c_int;
                    }
                    k += 1;
                    k;
                }
                if bMissData == 0 as libc::c_int {
                    if bMissOutp == 0 as libc::c_int {
                        dTmp = LnDensity(pMCVar, panal);
                        if dTmp == -1.0E+100f64 {
                            k = 0 as libc::c_int as libc::c_long;
                            while k < 4 as libc::c_int as libc::c_long {
                                (*pMCVar).pdParm[k as usize] = pdBase[k as usize];
                                k += 1;
                                k;
                            }
                            return -1.0E+100f64;
                        } else {
                            dLnLike = dLnLike + dTmp;
                        }
                    } else {
                        ReportRunTimeError(
                            panal,
                            (0x307 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"LnLikeData\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            j += 1;
            j;
        }
        k = 0 as libc::c_int as libc::c_long;
        while k < 4 as libc::c_int as libc::c_long {
            (*pMCVar).pdParm[k as usize] = pdBase[k as usize];
            k += 1;
            k;
        }
        i += 1;
        i;
    }
    return dLnLike;
}
pub unsafe extern "C" fn MaxMCVar(mut pMCVar: PMCVAR) -> libc::c_double {
    if (*pMCVar).iType == 8 as libc::c_int || (*pMCVar).iType == 12 as libc::c_int {
        return *(*pMCVar).pdParm[3 as libc::c_int as usize]
    } else if (*pMCVar).iType == 0 as libc::c_int || (*pMCVar).iType == 1 as libc::c_int
    {
        return *(*pMCVar).pdParm[1 as libc::c_int as usize]
    } else {
        return *(*pMCVar).pdParm[3 as libc::c_int as usize]
    };
}
pub unsafe extern "C" fn MinMCVar(mut pMCVar: PMCVAR) -> libc::c_double {
    if (*pMCVar).iType == 8 as libc::c_int || (*pMCVar).iType == 12 as libc::c_int {
        return *(*pMCVar).pdParm[2 as libc::c_int as usize]
    } else if (*pMCVar).iType == 0 as libc::c_int || (*pMCVar).iType == 1 as libc::c_int
    {
        return *(*pMCVar).pdParm[0 as libc::c_int as usize]
    } else {
        return *(*pMCVar).pdParm[2 as libc::c_int as usize]
    };
}
pub unsafe extern "C" fn RunTemperingBlock(
    mut panal: PANALYSIS,
    mut lRunLength: libc::c_long,
    mut iter: PLONG,
) {
    let mut pgd: PGIBBSDATA = &mut (*panal).gd;
    let mut pLevel0: PLEVEL = (*panal).pLevels[0 as libc::c_int as usize];
    let mut dTmp: libc::c_double = 0.;
    let mut dLnPrior: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dLnData: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut nUpdateAt: libc::c_long = 0;
    let mut nTotal: libc::c_long = 0;
    i = 0 as libc::c_int as libc::c_long;
    while i < lRunLength {
        nTotal = 20 as libc::c_int as libc::c_long;
        nUpdateAt = *iter + nTotal;
        TraverseLevels(
            pLevel0,
            Some(
                SampleThetasTempered
                    as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            panal,
            pgd,
            &mut i as *mut libc::c_long,
            &mut nUpdateAt as *mut libc::c_long,
            &mut nTotal as *mut libc::c_long,
            &mut (*pgd).indexT as *mut libc::c_int,
            0 as *mut libc::c_void,
        );
        dLnPrior = 0.0f64;
        TraverseLevels(
            pLevel0,
            Some(
                CalculateTotals
                    as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            panal,
            &mut dLnPrior as *mut libc::c_double,
            0 as *mut libc::c_void,
        );
        dLnData = 0.0f64;
        TraverseLevels1(
            pLevel0,
            Some(
                SumAllExpts
                    as unsafe extern "C" fn(
                        PLEVEL,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            &mut dLnData as *mut libc::c_double,
            0 as *mut libc::c_void,
        );
        dTmp = (*pgd).dCZero / (i as libc::c_double + (*pgd).dNZero);
        j = (*pgd).startT as libc::c_long;
        while j <= (*pgd).endT as libc::c_long {
            if j == (*pgd).indexT as libc::c_long {
                *((*pgd).rgdlnPi).offset(j as isize) -= dTmp;
            } else {
                *((*pgd).rgdlnPi).offset(j as isize)
                    += dTmp / (*pgd).nPerks as libc::c_double;
            }
            j += 1;
            j;
        }
        *((*pgd).rglCount)
            .offset(
                (*pgd).indexT as isize,
            ) = *((*pgd).rglCount).offset((*pgd).indexT as isize)
            + 1 as libc::c_int as libc::c_long;
        (*pgd).indexT = SampleTemperature2(pgd, dLnPrior, dLnData) as libc::c_int;
        if i == nUpdateAt {
            nTotal = nTotal * 3 as libc::c_int as libc::c_long
                / 2 as libc::c_int as libc::c_long;
            nUpdateAt = i + nTotal;
        }
        *iter += 1;
        *iter;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn NextDown(mut Perk: libc::c_double) -> libc::c_double {
    let mut i: libc::c_int = 0;
    static mut PTable: [libc::c_double; 21] = [
        0 as libc::c_int as libc::c_double,
        1E-6f64,
        1E-5f64,
        1E-4f64,
        1E-3f64,
        1E-2f64,
        0.1f64,
        0.2f64,
        0.3f64,
        0.5f64,
        0.6f64,
        0.7f64,
        0.8f64,
        0.9f64,
        0.95f64,
        0.97f64,
        0.99f64,
        0.999f64,
        0.9999f64,
        0.99999f64,
        1 as libc::c_int as libc::c_double,
    ];
    i = 0 as libc::c_int;
    while Perk > PTable[i as usize] {
        i += 1;
        i;
    }
    return if i == 0 as libc::c_int {
        PTable[i as usize]
    } else {
        PTable[(i - 1 as libc::c_int) as usize]
    };
}
pub unsafe extern "C" fn InitPerks(mut panal: PANALYSIS) {
    let mut pgd: PGIBBSDATA = &mut (*panal).gd;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut iter: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut dTmp: libc::c_double = 0.;
    let mut bTrans: libc::c_int = 0;
    let mut bHappy: BOOL = 0;
    let mut bTooManyTrials: BOOL = 0;
    if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long
        || (*pgd).nSimTypeFlag == 4 as libc::c_int as libc::c_long
    {
        (*pgd).rglTransAttempts = InitlVector(50 as libc::c_int as libc::c_long);
        if ((*pgd).rglTransAttempts).is_null()
            || {
                (*pgd).rglTransAccepts = InitlVector(50 as libc::c_int as libc::c_long);
                ((*pgd).rglTransAccepts).is_null()
            }
        {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"InitPerks\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        i = 0 as libc::c_int as libc::c_long;
        while i < 50 as libc::c_int as libc::c_long {
            let ref mut fresh7 = *((*pgd).rglTransAccepts).offset(i as isize);
            *fresh7 = 0 as libc::c_int as libc::c_long;
            *((*pgd).rglTransAttempts).offset(i as isize) = *fresh7;
            i += 1;
            i;
        }
        if (*pgd).nPerks == 0 as libc::c_int {
            printf(
                b"Setting perks (inverse temperatures).\n\0" as *const u8
                    as *const libc::c_char,
            );
            (*pgd).nPerks = 50 as libc::c_int;
            (*pgd).rgdPerks = InitdVector(50 as libc::c_int as libc::c_long);
            if ((*pgd).rgdPerks).is_null()
                || {
                    (*pgd).rgdlnPi = InitdVector(50 as libc::c_int as libc::c_long);
                    ((*pgd).rgdlnPi).is_null()
                }
                || {
                    (*pgd).rglCount = InitlVector(50 as libc::c_int as libc::c_long);
                    ((*pgd).rglCount).is_null()
                }
            {
                ReportError(
                    0 as PINPUTBUF,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"InitPerks\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
            i = 0 as libc::c_int as libc::c_long;
            while i < 50 as libc::c_int as libc::c_long {
                let ref mut fresh8 = *((*pgd).rglCount).offset(i as isize);
                *fresh8 = 0 as libc::c_int as libc::c_long;
                let ref mut fresh9 = *((*pgd).rgdlnPi).offset(i as isize);
                *fresh9 = *fresh8 as libc::c_double;
                *((*pgd).rgdPerks).offset(i as isize) = *fresh9;
                i += 1;
                i;
            }
            (*pgd).endT = 50 as libc::c_int - 1 as libc::c_int;
            (*pgd).startT = 50 as libc::c_int - 2 as libc::c_int;
            (*pgd).indexT = (*pgd).startT;
            let mut dEPSILON: libc::c_double = 0.99f64;
            let mut dUP: libc::c_double = 2.0f64;
            *((*pgd).rgdPerks).offset((*pgd).startT as isize) = dEPSILON;
            *((*pgd).rgdPerks).offset((*pgd).endT as isize) = 1.00f64;
            let mut nOldPrintIter: libc::c_long = (*pgd).nPrintIter;
            (*pgd).nPrintIter = -(*pgd).nMaxPerkSetIter;
            let mut lRunLength: libc::c_int = 100 as libc::c_int;
            let mut dBoundary: libc::c_double = 0.0f64;
            loop {
                (*pgd).indexT = (*pgd).startT;
                RunTemperingBlock(panal, lRunLength as libc::c_long, &mut iter);
                PrintTemperatureDiagnostics(stdout, pgd);
                PrintTemperatureDiagnostics((*pgd).pfilePerks, pgd);
                bTrans = CheckTransitions(pgd);
                if *((*pgd).rgdPerks).offset((*pgd).startT as isize) == dBoundary {
                    bHappy = (bTrans > -(1 as libc::c_int)) as libc::c_int;
                } else {
                    bHappy = 0 as libc::c_int;
                }
                bTooManyTrials = (iter > (*pgd).nMaxPerkSetIter) as libc::c_int;
                if bHappy == 0 {
                    if bTrans == -(1 as libc::c_int) {
                        printf(
                            b"acceptance rate 1<->2 too low, stepping back up\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        dTmp = (*((*pgd).rgdPerks).offset((*pgd).startT as isize)
                            + *((*pgd).rgdPerks)
                                .offset(((*pgd).startT + 1 as libc::c_int) as isize)) / dUP;
                        *((*pgd).rgdlnPi)
                            .offset(
                                (*pgd).startT as isize,
                            ) = (*((*pgd).rgdlnPi).offset((*pgd).startT as isize)
                            + Extrapolate(
                                pgd,
                                dTmp,
                                (*pgd).startT,
                                (*pgd).startT + 1 as libc::c_int,
                            )) / 2 as libc::c_int as libc::c_double;
                        *((*pgd).rgdPerks).offset((*pgd).startT as isize) = dTmp;
                    }
                    if bTrans == 1 as libc::c_int {
                        printf(
                            b"acceptance rate 1<->2 too high, moving down\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        dTmp = NextDown(
                            *((*pgd).rgdPerks).offset((*pgd).startT as isize),
                        );
                        *((*pgd).rgdlnPi)
                            .offset(
                                (*pgd).startT as isize,
                            ) = (*((*pgd).rgdlnPi).offset((*pgd).startT as isize)
                            + Extrapolate(
                                pgd,
                                dTmp,
                                (*pgd).startT,
                                (*pgd).startT + 1 as libc::c_int,
                            )) / 2 as libc::c_int as libc::c_double;
                        *((*pgd).rgdPerks).offset((*pgd).startT as isize) = dTmp;
                    }
                    if bTrans == 0 as libc::c_int {
                        if (*pgd).startT > 0 as libc::c_int {
                            printf(
                                b"acceptance rate 1<->2 ok, adding a new point\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            if CheckAllTransitions(pgd) != 0 {
                                let mut i_0: libc::c_int = (*pgd).endT;
                                let mut j_0: libc::c_int = i_0 - 2 as libc::c_int;
                                while j_0 >= (*pgd).startT {
                                    if EqualSlopes((*pgd).rgdPerks, (*pgd).rgdlnPi, j_0) != 0 {
                                        k = j_0 as libc::c_long;
                                        while k >= (*pgd).startT as libc::c_long {
                                            *((*pgd).rgdPerks)
                                                .offset(
                                                    (k + 1 as libc::c_int as libc::c_long) as isize,
                                                ) = *((*pgd).rgdPerks).offset(k as isize);
                                            *((*pgd).rgdlnPi)
                                                .offset(
                                                    (k + 1 as libc::c_int as libc::c_long) as isize,
                                                ) = *((*pgd).rgdlnPi).offset(k as isize);
                                            *((*pgd).rglCount)
                                                .offset(
                                                    (k + 1 as libc::c_int as libc::c_long) as isize,
                                                ) = *((*pgd).rglCount).offset(k as isize);
                                            k -= 1;
                                            k;
                                        }
                                        (*pgd).startT += 1;
                                        (*pgd).startT;
                                        if (*pgd).indexT <= j_0 {
                                            (*pgd).indexT += 1;
                                            (*pgd).indexT;
                                        }
                                        lRunLength = lRunLength - 100 as libc::c_int;
                                        printf(
                                            b"Scale has been reduced.\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    } else {
                                        i_0 -= 1;
                                        i_0;
                                        j_0 -= 1;
                                        j_0;
                                    }
                                }
                            }
                            (*pgd).startT = (*pgd).startT - 1 as libc::c_int;
                            (*pgd).indexT = (*pgd).startT;
                            *((*pgd).rgdPerks)
                                .offset(
                                    (*pgd).startT as isize,
                                ) = NextDown(
                                *((*pgd).rgdPerks)
                                    .offset(((*pgd).startT + 1 as libc::c_int) as isize),
                            );
                            *((*pgd).rgdlnPi)
                                .offset(
                                    (*pgd).startT as isize,
                                ) = (*((*pgd).rgdlnPi)
                                .offset(((*pgd).startT + 1 as libc::c_int) as isize)
                                + Extrapolate(
                                    pgd,
                                    *((*pgd).rgdPerks).offset((*pgd).startT as isize),
                                    (*pgd).startT + 1 as libc::c_int,
                                    (*pgd).startT + 2 as libc::c_int,
                                )) / 2 as libc::c_int as libc::c_double;
                            lRunLength = lRunLength + 100 as libc::c_int;
                        } else {
                            bTooManyTrials = 1 as libc::c_int;
                        }
                    }
                    i = (*pgd).startT as libc::c_long;
                    while i <= (*pgd).endT as libc::c_long {
                        *((*pgd).rglCount)
                            .offset(i as isize) = 0 as libc::c_int as libc::c_long;
                        let ref mut fresh10 = *((*pgd).rglTransAccepts)
                            .offset(i as isize);
                        *fresh10 = 0 as libc::c_int as libc::c_long;
                        *((*pgd).rglTransAttempts).offset(i as isize) = *fresh10;
                        i += 1;
                        i;
                    }
                }
                if !(bHappy == 0 && bTooManyTrials == 0) {
                    break;
                }
            }
            if *((*pgd).rgdPerks).offset((*pgd).startT as isize) == dBoundary {
                printf(
                    b"Perk %lg reached in %ld iterations.\n\0" as *const u8
                        as *const libc::c_char,
                    dBoundary,
                    iter,
                );
            } else {
                printf(
                    b"Perk %lg not reached in %ld iterations...\n\0" as *const u8
                        as *const libc::c_char,
                    dBoundary,
                    iter,
                );
            }
            (*pgd).nPrintIter = nOldPrintIter;
            let mut iCount: libc::c_int = (*pgd).endT - (*pgd).startT + 1 as libc::c_int;
            if iCount != 50 as libc::c_int {
                (*pgd).nPerks = iCount;
                (*pgd).indexT = (*pgd).indexT - (*pgd).startT;
                i = 0 as libc::c_int as libc::c_long;
                while i < iCount as libc::c_long {
                    j = (*pgd).startT as libc::c_long + i;
                    *((*pgd).rgdPerks)
                        .offset(i as isize) = *((*pgd).rgdPerks).offset(j as isize);
                    *((*pgd).rgdlnPi)
                        .offset(i as isize) = *((*pgd).rgdlnPi).offset(j as isize);
                    *((*pgd).rglCount)
                        .offset(i as isize) = 0 as libc::c_int as libc::c_long;
                    i += 1;
                    i;
                }
                (*pgd).startT = 0 as libc::c_int;
                (*pgd).endT = iCount - 1 as libc::c_int;
            }
            printf(
                b"Done with InitPerks - Continuing.\n\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
}
pub unsafe extern "C" fn OpenMarkovFiles(mut panal: PANALYSIS) {
    let mut pgd: PGIBBSDATA = &mut (*panal).gd;
    let mut with_rank: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*panal).bDependents != 0 {
        return;
    }
    if (*panal).bCommandLineSpec != 0 {
        free((*pgd).szGout as *mut libc::c_void);
        (*panal).bAllocatedFileName = 0 as libc::c_int;
        (*pgd).szGout = (*panal).szOutfilename;
    } else if ((*pgd).szGout).is_null() {
        (*pgd)
            .szGout = b"MCMC.default.out\0" as *const u8 as *const libc::c_char as PSTR;
    }
    if (*panal).size > 1 as libc::c_int {
        with_rank = malloc(
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(
                    (6 as libc::c_int as libc::c_ulong)
                        .wrapping_add(strlen((*pgd).szGout as *const libc::c_char)),
                ),
        ) as *mut libc::c_char;
        sprintf(
            with_rank,
            b"%04d_%s\0" as *const u8 as *const libc::c_char,
            (*panal).rank,
            (*pgd).szGout,
        );
        (*pgd).szGout = with_rank;
    }
    if !((*pgd).szGrestart).is_null() {
        if ((*pgd).pfileRestart).is_null()
            && {
                (*pgd)
                    .pfileRestart = fopen(
                    (*pgd).szGrestart as *const libc::c_char,
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                ((*pgd).pfileRestart).is_null()
            }
        {
            ReportRunTimeError(
                panal,
                (0x8000 as libc::c_int | 0x3 as libc::c_int) as WORD,
                (*pgd).szGrestart,
                b"OpenMarkovFiles\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if ((*pgd).pfileOut).is_null()
        && {
            (*pgd)
                .pfileOut = fopen(
                (*pgd).szGout as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            ((*pgd).pfileOut).is_null()
        }
    {
        ReportRunTimeError(
            panal,
            (0x8000 as libc::c_int | 0x3 as libc::c_int) as WORD,
            (*pgd).szGout,
            b"OpenMarkovFiles\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long
        || (*pgd).nSimTypeFlag == 4 as libc::c_int as libc::c_long
    {
        let mut szFileName: [libc::c_char; 206] = [0; 206];
        sprintf(
            szFileName.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (*pgd).szGout,
            b".perks\0" as *const u8 as *const libc::c_char,
        );
        if ((*pgd).pfilePerks).is_null()
            && {
                (*pgd)
                    .pfilePerks = fopen(
                    szFileName.as_mut_ptr(),
                    b"w\0" as *const u8 as *const libc::c_char,
                );
                ((*pgd).pfilePerks).is_null()
            }
        {
            ReportRunTimeError(
                panal,
                (0x8000 as libc::c_int | 0x3 as libc::c_int) as WORD,
                szFileName.as_mut_ptr(),
                b"OpenMarkovFiles\0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
pub unsafe extern "C" fn PrintAllExpts(
    mut plevel: PLEVEL,
    mut panal: PANALYSIS,
    mut pOutFile: PFILE,
) {
    let mut n: libc::c_long = 0;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).iInstances as libc::c_long {
        TraverseLevels1(
            (*plevel).pLevels[n as usize],
            Some(
                PrintExpt
                    as unsafe extern "C" fn(
                        PLEVEL,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            panal,
            pOutFile,
            0 as *mut libc::c_void,
        );
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn PrintDeps(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut n: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    printf(
        b"Depth %d; Instance %d\n\0" as *const u8 as *const libc::c_char,
        (*plevel).iDepth,
        (*plevel).iSequence,
    );
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
        printf(
            b"Variable %s (%d) [%lx]\n\0" as *const u8 as *const libc::c_char,
            (*pMCVar).pszName,
            (*pMCVar).iDepth,
            pMCVar as intptr_t,
        );
        m = 0 as libc::c_int as libc::c_long;
        while m < 4 as libc::c_int as libc::c_long {
            if !((*pMCVar).pMCVParent[m as usize]).is_null() {
                printf(
                    b"  Parent %ld: %s (%d) [%lx]\n\0" as *const u8
                        as *const libc::c_char,
                    m,
                    (*(*pMCVar).pMCVParent[m as usize]).pszName,
                    (*(*pMCVar).pMCVParent[m as usize]).iDepth,
                    (*pMCVar).pMCVParent[m as usize] as intptr_t,
                );
            }
            m += 1;
            m;
        }
        m = 0 as libc::c_int as libc::c_long;
        while m < (*pMCVar).nDependents {
            printf(
                b"  Dependent: %s (%d) [%lx]\n\0" as *const u8 as *const libc::c_char,
                (**((*pMCVar).rgpDependents).offset(m as isize)).pszName,
                (**((*pMCVar).rgpDependents).offset(m as isize)).iDepth,
                *((*pMCVar).rgpDependents).offset(m as isize) as intptr_t,
            );
            m += 1;
            m;
        }
        if (*pMCVar).bExptIsDep != 0 {
            printf(
                b"  This variable influences experiments directly\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn PrintExpt(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut pOutFile: PFILE = *args.offset(1 as libc::c_int as isize) as PFILE;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    let mut n: libc::c_long = 0;
    let mut pExpt: PEXPERIMENT = (*plevel).pexpt;
    let mut pos: POUTSPEC = 0 as *mut tagOUTSPEC;
    static mut printed_head: libc::c_long = 0 as libc::c_int as libc::c_long;
    if printed_head == 0 {
        fprintf(
            pOutFile,
            b"Level\tSimulation\tOutput_Var\tTime\tData\tPrediction\n\0" as *const u8
                as *const libc::c_char,
        );
        printed_head = 1 as libc::c_int as libc::c_long;
    }
    (*panal).pCurrentLevel[(*plevel).iDepth as usize] = plevel;
    (*panal).iInstance[(*plevel).iDepth as usize] = (*plevel).iSequence;
    if !pExpt.is_null() {
        InitModel();
        n = 0 as libc::c_int as libc::c_long;
        while n <= (*plevel).iDepth as libc::c_long {
            SetModelVars((*panal).pCurrentLevel[n as usize]);
            SetFixedVars((*panal).pCurrentLevel[n as usize]);
            n += 1;
            n;
        }
        if DoOneExperiment(pExpt) == 0 {
            printf(
                b"Warning: DoOneExperiment failed\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int;
        } else {
            pos = &mut (*pExpt).os;
            m = 0 as libc::c_int as libc::c_long;
            while m < (*pos).nOutputs as libc::c_long {
                k = 0 as libc::c_int as libc::c_long;
                while k < (*pos).nData as libc::c_long {
                    if strcmp(
                        *((*pos).pszDataNames).offset(k as isize) as *const libc::c_char,
                        *((*pos).pszOutputNames).offset(m as isize)
                            as *const libc::c_char,
                    ) == 0
                    {
                        break;
                    }
                    k += 1;
                    k;
                }
                l = 0 as libc::c_int as libc::c_long;
                while l < *((*pos).pcOutputTimes).offset(m as isize) as libc::c_long {
                    n = 1 as libc::c_int as libc::c_long;
                    while n < (*plevel).iDepth as libc::c_long {
                        fprintf(
                            pOutFile,
                            b"%d_\0" as *const u8 as *const libc::c_char,
                            (*panal).iInstance[n as usize],
                        );
                        n += 1;
                        n;
                    }
                    fprintf(
                        pOutFile,
                        b"%d\t\0" as *const u8 as *const libc::c_char,
                        (*panal).iInstance[(*plevel).iDepth as usize],
                    );
                    if k != (*pos).nData as libc::c_long {
                        fprintf(
                            pOutFile,
                            b"%d\t%s\t%g\t%g\t%g\n\0" as *const u8
                                as *const libc::c_char,
                            (*pExpt).iExp,
                            *((*pos).pszOutputNames).offset(m as isize),
                            *(*((*pos).prgdOutputTimes).offset(m as isize))
                                .offset(l as isize),
                            *(*((*pos).prgdDataVals).offset(k as isize))
                                .offset(l as isize),
                            *(*((*pos).prgdOutputVals).offset(m as isize))
                                .offset(l as isize),
                        );
                    } else {
                        fprintf(
                            pOutFile,
                            b"%d\t%s\t%g\t\t%g\n\0" as *const u8 as *const libc::c_char,
                            (*pExpt).iExp,
                            *((*pos).pszOutputNames).offset(m as isize),
                            *(*((*pos).prgdOutputTimes).offset(m as isize))
                                .offset(l as isize),
                            *(*((*pos).prgdOutputVals).offset(m as isize))
                                .offset(l as isize),
                        );
                    }
                    l += 1;
                    l;
                }
                fprintf(pOutFile, b"\n\0" as *const u8 as *const libc::c_char);
                m += 1;
                m;
            }
            fprintf(pOutFile, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn PrintTemperatureDiagnostics(
    mut fOut: PFILE,
    mut pgd: PGIBBSDATA,
) {
    let mut i: libc::c_int = 0;
    fprintf(fOut, b"\nPerks:\0" as *const u8 as *const libc::c_char);
    i = (*pgd).startT;
    while i <= (*pgd).endT {
        fprintf(
            fOut,
            b"\t%g\0" as *const u8 as *const libc::c_char,
            *((*pgd).rgdPerks).offset(i as isize),
        );
        i += 1;
        i;
    }
    fprintf(fOut, b"\nCounts:\0" as *const u8 as *const libc::c_char);
    i = (*pgd).startT;
    while i <= (*pgd).endT {
        fprintf(
            fOut,
            b"\t%ld\0" as *const u8 as *const libc::c_char,
            *((*pgd).rglCount).offset(i as isize),
        );
        i += 1;
        i;
    }
    fprintf(fOut, b"\nLnPi(i):\0" as *const u8 as *const libc::c_char);
    i = (*pgd).startT;
    while i <= (*pgd).endT {
        fprintf(
            fOut,
            b"\t%g\0" as *const u8 as *const libc::c_char,
            *((*pgd).rgdlnPi).offset(i as isize),
        );
        i += 1;
        i;
    }
    fprintf(fOut, b"\nTried Jumps:\t\0" as *const u8 as *const libc::c_char);
    i = (*pgd).startT;
    while i <= (*pgd).endT - 1 as libc::c_int {
        fprintf(
            fOut,
            b"\t%ld\0" as *const u8 as *const libc::c_char,
            *((*pgd).rglTransAttempts).offset(i as isize),
        );
        i += 1;
        i;
    }
    fprintf(fOut, b"\nAccepted Jumps:\t\0" as *const u8 as *const libc::c_char);
    i = (*pgd).startT;
    while i <= (*pgd).endT - 1 as libc::c_int {
        fprintf(
            fOut,
            b"\t%ld\0" as *const u8 as *const libc::c_char,
            *((*pgd).rglTransAccepts).offset(i as isize),
        );
        i += 1;
        i;
    }
    fprintf(fOut, b"\n\n\0" as *const u8 as *const libc::c_char);
    fflush(fOut);
}
pub unsafe extern "C" fn ReadData(mut plevel: PLEVEL, mut args: *mut *mut libc::c_char) {
    let mut pfileData: *mut FILE = *args.offset(0 as libc::c_int as isize) as *mut FILE;
    let mut pos: POUTSPEC = 0 as *mut tagOUTSPEC;
    let mut cDat: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if ((*plevel).pexpt).is_null() {
        return;
    }
    pos = &mut (*(*plevel).pexpt).os;
    cDat = (*pos).nOutputs;
    (*pos).prgdDataVals = InitpdVector(cDat as libc::c_long);
    (*pos).pcData = InitiVector(cDat as libc::c_long);
    (*pos)
        .pszDataNames = malloc(
        (cDat as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<PSTR>() as libc::c_ulong),
    ) as *mut PSTR;
    (*pos)
        .phvar_dat = malloc(
        (cDat as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HVAR>() as libc::c_ulong),
    ) as *mut HVAR;
    if ((*pos).prgdDataVals).is_null() || ((*pos).phvar_dat).is_null()
        || ((*pos).pszDataNames).is_null() || ((*pos).pcData).is_null()
    {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ReadData()\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    } else {
        (*pos).nData = cDat;
        i = 0 as libc::c_int;
        while i < cDat {
            let ref mut fresh11 = *((*pos).prgdDataVals).offset(i as isize);
            *fresh11 = InitdVector(
                *((*pos).pcOutputTimes).offset(i as isize) as libc::c_long,
            );
            if (*fresh11).is_null() {
                ReportError(
                    0 as PINPUTBUF,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"ReadData()\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
            j = 0 as libc::c_int;
            while j < *((*pos).pcOutputTimes).offset(i as isize) {
                if fscanf(
                    pfileData,
                    b"%lg\0" as *const u8 as *const libc::c_char,
                    &mut *(*((*pos).prgdDataVals).offset(i as isize)).offset(j as isize)
                        as *mut libc::c_double,
                ) == -(1 as libc::c_int)
                {
                    printf(
                        b"Error: incorrect length for data file - Exiting\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                j += 1;
                j;
            }
            *((*pos).pcData).offset(i as isize) = j;
            *((*pos).phvar_dat)
                .offset(i as isize) = *((*pos).phvar_out).offset(i as isize);
            let ref mut fresh12 = *((*pos).pszDataNames).offset(i as isize);
            *fresh12 = *((*pos).pszOutputNames).offset(i as isize);
            i += 1;
            i;
        }
    };
}
pub unsafe extern "C" fn ReadDataFile(mut panal: PANALYSIS) {
    if !((*panal).gd.szGdata).is_null() {
        let mut c: libc::c_char = 0;
        let mut pfile: *mut FILE = fopen(
            (*panal).gd.szGdata as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if pfile.is_null() {
            printf(
                b"Cannot open data file '%s'\n\0" as *const u8 as *const libc::c_char,
                (*panal).gd.szGdata,
            );
            exit(0 as libc::c_int);
        }
        loop {
            c = getc(pfile) as libc::c_char;
            if !(c as libc::c_int != '\n' as i32) {
                break;
            }
        }
        TraverseLevels(
            (*panal).pLevels[0 as libc::c_int as usize],
            Some(ReadData as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> ()),
            pfile,
            0 as *mut libc::c_void,
        );
        fclose(pfile);
    }
}
pub unsafe extern "C" fn ReadRestart(
    mut pfileRestart: *mut FILE,
    mut nThetas: libc::c_long,
    mut pdTheta: *mut PDOUBLE,
    mut pdSum: *mut PDOUBLE,
    mut prgdSumProd: *mut *mut PDOUBLE,
    mut pIter: *mut libc::c_long,
) {
    let mut c: libc::c_char = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    if (*pdTheta).is_null() {
        *pdTheta = InitdVector(nThetas);
        if (*pdTheta).is_null() {
            ReportRunTimeError(
                0 as PANALYSIS,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"ReadRestart\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*pdSum).is_null() {
        *pdSum = InitdVector(nThetas);
        if (*pdSum).is_null() {
            ReportRunTimeError(
                0 as PANALYSIS,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"ReadRestart\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*prgdSumProd).is_null() {
        *prgdSumProd = InitdMatrix(nThetas, nThetas);
        if (*prgdSumProd).is_null() {
            ReportRunTimeError(
                0 as PANALYSIS,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"ReadRestart\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    *pIter = -(1 as libc::c_int) as libc::c_long;
    i = 0 as libc::c_int as libc::c_long;
    while i < nThetas {
        *(*pdSum).offset(i as isize) = 0.0f64;
        j = 0 as libc::c_int as libc::c_long;
        while j < nThetas {
            *(*(*prgdSumProd).offset(i as isize)).offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    loop {
        c = getc(pfileRestart) as libc::c_char;
        if !(c as libc::c_int != '\n' as i32) {
            break;
        }
    }
    while !(feof(pfileRestart) != 0
        || fscanf(pfileRestart, b"%*s\0" as *const u8 as *const libc::c_char)
            == -(1 as libc::c_int))
    {
        i = 0 as libc::c_int as libc::c_long;
        while i < nThetas {
            if fscanf(
                pfileRestart,
                b"%lg\0" as *const u8 as *const libc::c_char,
                &mut *(*pdTheta).offset(i as isize) as *mut libc::c_double,
            ) == -(1 as libc::c_int)
            {
                printf(
                    b"Error: incorrect length for restart file - Exiting\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            } else {
                *(*pdSum).offset(i as isize) += *(*pdTheta).offset(i as isize);
            }
            i += 1;
            i;
        }
        loop {
            c = getc(pfileRestart) as libc::c_char;
            if !(c as libc::c_int != '\n' as i32) {
                break;
            }
        }
        i = 0 as libc::c_int as libc::c_long;
        while i < nThetas {
            j = 0 as libc::c_int as libc::c_long;
            while j < nThetas {
                *(*(*prgdSumProd).offset(i as isize)).offset(j as isize)
                    += *(*pdTheta).offset(i as isize) * *(*pdTheta).offset(j as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        *pIter = *pIter + 1 as libc::c_int as libc::c_long;
    }
    fclose(pfileRestart);
}
pub unsafe extern "C" fn ReadRestartTemper(
    mut pfileRestart: *mut FILE,
    mut nThetas: libc::c_long,
    mut nPerks: libc::c_int,
    mut pdTheta: *mut PDOUBLE,
    mut pdSum: *mut PDOUBLE,
    mut prgdSumProd: *mut *mut PDOUBLE,
    mut pIter: *mut libc::c_long,
    mut pindexT: *mut libc::c_int,
    mut pdlnPi: *mut libc::c_double,
) {
    let mut c: libc::c_char = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    if (*pdTheta).is_null() {
        *pdTheta = InitdVector(nThetas);
        if (*pdTheta).is_null() {
            ReportRunTimeError(
                0 as PANALYSIS,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"ReadRestart\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*pdSum).is_null() {
        *pdSum = InitdVector(nThetas);
        if (*pdSum).is_null() {
            ReportRunTimeError(
                0 as PANALYSIS,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"ReadRestart\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*prgdSumProd).is_null() {
        *prgdSumProd = InitdMatrix(nThetas, nThetas);
        if (*prgdSumProd).is_null() {
            ReportRunTimeError(
                0 as PANALYSIS,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"ReadRestart\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    *pIter = -(1 as libc::c_int) as libc::c_long;
    i = 0 as libc::c_int as libc::c_long;
    while i < nThetas {
        *(*pdSum).offset(i as isize) = 0.0f64;
        j = 0 as libc::c_int as libc::c_long;
        while j < nThetas {
            *(*(*prgdSumProd).offset(i as isize)).offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    loop {
        c = getc(pfileRestart) as libc::c_char;
        if !(c as libc::c_int != '\n' as i32) {
            break;
        }
    }
    while !(feof(pfileRestart) != 0
        || fscanf(pfileRestart, b"%*s\0" as *const u8 as *const libc::c_char)
            == -(1 as libc::c_int))
    {
        i = 0 as libc::c_int as libc::c_long;
        while i < nThetas {
            if fscanf(
                pfileRestart,
                b"%lg\0" as *const u8 as *const libc::c_char,
                &mut *(*pdTheta).offset(i as isize) as *mut libc::c_double,
            ) == -(1 as libc::c_int)
            {
                printf(
                    b"Error: incorrect length for restart file - Exiting\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            } else {
                *(*pdSum).offset(i as isize) += *(*pdTheta).offset(i as isize);
            }
            i += 1;
            i;
        }
        if fscanf(pfileRestart, b"%d\0" as *const u8 as *const libc::c_char, pindexT)
            == -(1 as libc::c_int)
        {
            printf(
                b"Error: incorrect length for restart file - Exiting\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
        i = 0 as libc::c_int as libc::c_long;
        while i < nPerks as libc::c_long {
            if fscanf(
                pfileRestart,
                b"%lg\0" as *const u8 as *const libc::c_char,
                &mut *pdlnPi.offset(i as isize) as *mut libc::c_double,
            ) == -(1 as libc::c_int)
            {
                printf(
                    b"Error: incorrect length for restart file - Exiting\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            i += 1;
            i;
        }
        loop {
            c = getc(pfileRestart) as libc::c_char;
            if !(c as libc::c_int != '\n' as i32) {
                break;
            }
        }
        i = 0 as libc::c_int as libc::c_long;
        while i < nThetas {
            j = 0 as libc::c_int as libc::c_long;
            while j < nThetas {
                *(*(*prgdSumProd).offset(i as isize)).offset(j as isize)
                    += *(*pdTheta).offset(i as isize) * *(*pdTheta).offset(j as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        *pIter = *pIter + 1 as libc::c_int as libc::c_long;
    }
    fclose(pfileRestart);
}
pub unsafe extern "C" fn RestoreLikelihoods(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pExpt: PEXPERIMENT = (*plevel).pexpt;
    if !pExpt.is_null() {
        (*pExpt).dLnLike = (*pExpt).dLnLikeSave;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn RunAllExpts(
    mut panal: PANALYSIS,
    mut pdLnData: PDOUBLE,
) -> libc::c_int {
    let mut plevel0: PLEVEL = (*panal).pLevels[0 as libc::c_int as usize];
    let mut n: libc::c_long = 0;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel0).iInstances as libc::c_long {
        if TraverseLevels1(
            (*plevel0).pLevels[n as usize],
            Some(
                RunExpt
                    as unsafe extern "C" fn(
                        PLEVEL,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            panal,
            pdLnData,
            0 as *mut libc::c_void,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        n += 1;
        n;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn RunExpt(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut pdLnData: *mut libc::c_double = *args.offset(1 as libc::c_int as isize)
        as *mut libc::c_double;
    let mut i: libc::c_long = 0;
    let mut pExpt: PEXPERIMENT = (*plevel).pexpt;
    (*panal).pCurrentLevel[(*plevel).iDepth as usize] = plevel;
    if !pExpt.is_null() {
        InitModel();
        i = 0 as libc::c_int as libc::c_long;
        while i <= (*plevel).iDepth as libc::c_long {
            SetModelVars((*panal).pCurrentLevel[i as usize]);
            SetFixedVars((*panal).pCurrentLevel[i as usize]);
            i += 1;
            i;
        }
        if DoOneExperiment(pExpt) == 0 {
            printf(
                b"Warning: DoOneExperiment failed\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int;
        } else {
            (*pExpt).dLnLike = LnLikeData(plevel, panal);
            *pdLnData = *pdLnData + (*pExpt).dLnLike;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn SampleTemperature(
    mut pgd: PGIBBSDATA,
    mut dLnPrior: libc::c_double,
    mut dLnData: libc::c_double,
) -> libc::c_long {
    let mut indexT: libc::c_int = (*pgd).indexT;
    let mut indexT_new: libc::c_int = 0;
    if indexT == 0 as libc::c_int {
        indexT_new = 1 as libc::c_int;
    } else if indexT == (*pgd).nPerks - 1 as libc::c_int {
        indexT_new = indexT - 1 as libc::c_int;
    } else if Randoms() > 0.5f64 {
        indexT_new = indexT + 1 as libc::c_int;
    } else {
        indexT_new = indexT - 1 as libc::c_int;
    }
    if TestTemper(
        pgd,
        indexT as libc::c_long,
        indexT_new as libc::c_long,
        dLnPrior,
        dLnData,
        *((*pgd).rgdlnPi).offset(indexT as isize),
        *((*pgd).rgdlnPi).offset(indexT_new as isize),
    ) != 0
    {
        return indexT_new as libc::c_long
    } else {
        return indexT as libc::c_long
    };
}
pub unsafe extern "C" fn SampleTemperature2(
    mut pgd: PGIBBSDATA,
    mut dLnPrior: libc::c_double,
    mut dLnData: libc::c_double,
) -> libc::c_long {
    let mut indexT: libc::c_int = (*pgd).indexT;
    let mut indexT_new: libc::c_int = 0;
    if indexT == (*pgd).startT {
        indexT_new = indexT + 1 as libc::c_int;
    } else if indexT == (*pgd).endT {
        indexT_new = indexT - 1 as libc::c_int;
    } else if Randoms() > 0.5f64 {
        indexT_new = indexT + 1 as libc::c_int;
    } else {
        indexT_new = indexT - 1 as libc::c_int;
    }
    let mut minI: libc::c_int = if indexT < indexT_new { indexT } else { indexT_new };
    let ref mut fresh13 = *((*pgd).rglTransAttempts).offset(minI as isize);
    *fresh13 += 1;
    *fresh13;
    if TestTemper(
        pgd,
        indexT as libc::c_long,
        indexT_new as libc::c_long,
        dLnPrior,
        dLnData,
        *((*pgd).rgdlnPi).offset(indexT as isize),
        *((*pgd).rgdlnPi).offset(indexT_new as isize),
    ) != 0
    {
        let ref mut fresh14 = *((*pgd).rglTransAccepts).offset(minI as isize);
        *fresh14 += 1;
        *fresh14;
        return indexT_new as libc::c_long;
    } else {
        return indexT as libc::c_long
    };
}
pub unsafe extern "C" fn SampleTheta(mut pMCVar: PMCVAR) -> libc::c_double {
    if (*pMCVar).iType == 8 as libc::c_int || (*pMCVar).iType == 12 as libc::c_int {
        return floor(
            0.5f64
                + TruncNormalRandom(
                    (*pMCVar).dVal,
                    (*pMCVar).dKernelSD,
                    MinMCVar(pMCVar),
                    MaxMCVar(pMCVar),
                ),
        )
    } else {
        return TruncNormalRandom(
            (*pMCVar).dVal,
            (*pMCVar).dKernelSD,
            MinMCVar(pMCVar),
            MaxMCVar(pMCVar),
        )
    };
}
pub unsafe extern "C" fn SampleThetaUnif(mut pMCVar: PMCVAR) -> libc::c_double {
    if (*pMCVar).iType == 8 as libc::c_int || (*pMCVar).iType == 12 as libc::c_int {
        return floor(0.5f64 + UniformRandom(MinMCVar(pMCVar), MaxMCVar(pMCVar)))
    } else {
        return UniformRandom(MinMCVar(pMCVar), MaxMCVar(pMCVar))
    };
}
pub unsafe extern "C" fn SampleThetas(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut current_block: u64;
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut pgd: PGIBBSDATA = *args.offset(1 as libc::c_int as isize) as PGIBBSDATA;
    let mut pnIter: *mut libc::c_long = *args.offset(2 as libc::c_int as isize)
        as *mut libc::c_long;
    let mut pnUpdateAt: *mut libc::c_long = *args.offset(3 as libc::c_int as isize)
        as *mut libc::c_long;
    let mut pnTotal: *mut libc::c_long = *args.offset(4 as libc::c_int as isize)
        as *mut libc::c_long;
    let mut dLnPrior: libc::c_double = 0.;
    let mut dLnLike: libc::c_double = 0.;
    let mut dLnData: libc::c_double = 0.;
    let mut dLnKern: libc::c_double = 0.;
    let mut dLnPriorNew: libc::c_double = 0.;
    let mut dLnLikeNew: libc::c_double = 0.;
    let mut dLnDataNew: libc::c_double = 0.;
    let mut dLnKernNew: libc::c_double = 0.;
    let mut dTheta: libc::c_double = 0.;
    let mut dJumps: libc::c_double = 0.;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut n: libc::c_long = 0;
    (*panal).pCurrentLevel[(*plevel).iDepth as usize] = plevel;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
        if !((*pMCVar).bIsFixed != 0) {
            dLnPrior = LnDensity(pMCVar, panal);
            dLnLike = LnLike(pMCVar, panal);
            dLnData = 0.0f64;
            if (*pMCVar).bExptIsDep != 0 {
                TraverseLevels1(
                    plevel,
                    Some(
                        SumAllExpts
                            as unsafe extern "C" fn(
                                PLEVEL,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                    &mut dLnData as *mut libc::c_double,
                    0 as *mut libc::c_void,
                );
            }
            dTheta = (*pMCVar).dVal;
            if *pnIter == *pnUpdateAt {
                dJumps = (*pMCVar).lJumps as libc::c_double / *pnTotal as libc::c_double;
                if dJumps > 0.3f64 {
                    if dJumps == 1 as libc::c_int as libc::c_double {
                        if (*pMCVar).dKernelSD < sqrt(1.7976931348623157e+308f64) {
                            if (*pMCVar).dKernelSD > 2 as libc::c_int as libc::c_double {
                                (*pMCVar)
                                    .dKernelSD = (*pMCVar).dKernelSD * (*pMCVar).dKernelSD;
                            } else {
                                (*pMCVar)
                                    .dKernelSD = (*pMCVar).dKernelSD
                                    * 20 as libc::c_int as libc::c_double;
                            }
                        } else {
                            (*pMCVar).dKernelSD = (*pMCVar).dMaxKernelSD;
                        }
                    } else if (*pMCVar).dKernelSD
                        < 1.7976931348623157e+308f64 / 2 as libc::c_int as libc::c_double
                    {
                        (*pMCVar)
                            .dKernelSD = (*pMCVar).dKernelSD
                            * 2 as libc::c_int as libc::c_double;
                    } else {
                        (*pMCVar).dKernelSD = (*pMCVar).dMaxKernelSD;
                    }
                    if (*pMCVar).dKernelSD > (*pMCVar).dMaxKernelSD {
                        (*pMCVar).dKernelSD = (*pMCVar).dMaxKernelSD;
                    }
                } else if dJumps == 0 as libc::c_int as libc::c_double {
                    if (*pMCVar).dKernelSD > pow(2.2250738585072014e-308f64, 0.45f64) {
                        if (*pMCVar).dKernelSD > 2 as libc::c_int as libc::c_double {
                            (*pMCVar).dKernelSD = pow((*pMCVar).dKernelSD, 0.45f64);
                        } else {
                            (*pMCVar).dKernelSD = (*pMCVar).dKernelSD * 0.04f64;
                        }
                    } else {
                        (*pMCVar).dKernelSD = 2.2250738585072014e-308f64;
                    }
                } else if (*pMCVar).dKernelSD > 2.2250738585072014e-308f64 / 0.4f64 {
                    (*pMCVar).dKernelSD = (*pMCVar).dKernelSD * 0.4f64;
                } else {
                    (*pMCVar).dKernelSD = 2.2250738585072014e-308f64;
                }
                (*pMCVar).lJumps = 0 as libc::c_int as libc::c_long;
            }
            dLnKern = log(
                CDFNormal((MaxMCVar(pMCVar) - (*pMCVar).dVal) / (*pMCVar).dKernelSD)
                    - CDFNormal(
                        (MinMCVar(pMCVar) - (*pMCVar).dVal) / (*pMCVar).dKernelSD,
                    ),
            );
            (*pMCVar).dVal = SampleTheta(pMCVar);
            dLnKernNew = log(
                CDFNormal((MaxMCVar(pMCVar) - (*pMCVar).dVal) / (*pMCVar).dKernelSD)
                    - CDFNormal(
                        (MinMCVar(pMCVar) - (*pMCVar).dVal) / (*pMCVar).dKernelSD,
                    ),
            );
            dLnPriorNew = LnDensity(pMCVar, panal);
            dLnLikeNew = LnLike(pMCVar, panal);
            dLnDataNew = 0.0f64;
            if (*pMCVar).bExptIsDep != 0 {
                if TraverseLevels1(
                    plevel,
                    Some(
                        RunExpt
                            as unsafe extern "C" fn(
                                PLEVEL,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                    panal,
                    &mut dLnDataNew as *mut libc::c_double,
                    0 as *mut libc::c_void,
                ) == 0
                {
                    (*pMCVar).dVal = dTheta;
                    TraverseLevels1(
                        plevel,
                        Some(
                            RestoreLikelihoods
                                as unsafe extern "C" fn(
                                    PLEVEL,
                                    *mut *mut libc::c_char,
                                ) -> libc::c_int,
                        ),
                        0 as *mut libc::c_void,
                    );
                    current_block = 1198209117690636994;
                } else {
                    current_block = 13619784596304402172;
                }
            } else {
                current_block = 13619784596304402172;
            }
            match current_block {
                1198209117690636994 => {}
                _ => {
                    if TestImpRatio(
                        pgd,
                        (*pMCVar).bExptIsDep,
                        dLnKern,
                        dLnKernNew,
                        dLnPrior,
                        dLnPriorNew,
                        dLnLike,
                        dLnLikeNew,
                        dLnData,
                        dLnDataNew,
                    ) == 0
                    {
                        (*pMCVar).dVal = dTheta;
                        if (*pMCVar).bExptIsDep != 0 {
                            TraverseLevels1(
                                plevel,
                                Some(
                                    RestoreLikelihoods
                                        as unsafe extern "C" fn(
                                            PLEVEL,
                                            *mut *mut libc::c_char,
                                        ) -> libc::c_int,
                                ),
                                0 as *mut libc::c_void,
                            );
                        }
                    } else {
                        (*pMCVar)
                            .lJumps = (*pMCVar).lJumps
                            + 1 as libc::c_int as libc::c_long;
                        if (*pMCVar).bExptIsDep != 0 {
                            TraverseLevels1(
                                plevel,
                                Some(
                                    SaveLikelihoods
                                        as unsafe extern "C" fn(
                                            PLEVEL,
                                            *mut *mut libc::c_char,
                                        ) -> libc::c_int,
                                ),
                                0 as *mut libc::c_void,
                            );
                        }
                    }
                    CalculateMeanAndVariance(
                        *pnIter + 1 as libc::c_int as libc::c_long,
                        (*pMCVar).dVal,
                        &mut (*pMCVar).dVal_mean,
                        &mut (*pMCVar).dVal_var,
                    );
                }
            }
        }
        if (*pnIter + 1 as libc::c_int as libc::c_long) % (*pgd).nPrintFreq
            == 0 as libc::c_int as libc::c_long
            && *pnIter >= (*pgd).nMaxIter - (*pgd).nPrintIter
        {
            fprintf(
                (*pgd).pfileOut,
                b"%5g\t\0" as *const u8 as *const libc::c_char,
                (*pMCVar).dVal,
            );
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn SampleThetasTempered(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut current_block: u64;
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut pgd: PGIBBSDATA = *args.offset(1 as libc::c_int as isize) as PGIBBSDATA;
    let mut pnIter: *mut libc::c_long = *args.offset(2 as libc::c_int as isize)
        as *mut libc::c_long;
    let mut pnUpdateAt: *mut libc::c_long = *args.offset(3 as libc::c_int as isize)
        as *mut libc::c_long;
    let mut pnTotal: *mut libc::c_long = *args.offset(4 as libc::c_int as isize)
        as *mut libc::c_long;
    let mut pindexT: *mut libc::c_long = *args.offset(5 as libc::c_int as isize)
        as *mut libc::c_long;
    let mut dLnPrior: libc::c_double = 0.;
    let mut dLnLike: libc::c_double = 0.;
    let mut dLnData: libc::c_double = 0.;
    let mut dLnKern: libc::c_double = 0.;
    let mut dLnPriorNew: libc::c_double = 0.;
    let mut dLnLikeNew: libc::c_double = 0.;
    let mut dLnDataNew: libc::c_double = 0.;
    let mut dLnKernNew: libc::c_double = 0.;
    let mut dTheta: libc::c_double = 0.;
    let mut dJumps: libc::c_double = 0.;
    let mut old_dKernelSD: libc::c_double = 0.;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut n: libc::c_long = 0;
    (*panal).pCurrentLevel[(*plevel).iDepth as usize] = plevel;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
        if !((*pMCVar).bIsFixed != 0) {
            dLnPrior = LnDensity(pMCVar, panal);
            dLnLike = LnLike(pMCVar, panal);
            dLnData = 0.0f64;
            if (*pMCVar).bExptIsDep != 0 {
                TraverseLevels1(
                    plevel,
                    Some(
                        SumAllExpts
                            as unsafe extern "C" fn(
                                PLEVEL,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                    &mut dLnData as *mut libc::c_double,
                    0 as *mut libc::c_void,
                );
            }
            dTheta = (*pMCVar).dVal;
            if *pnIter == *pnUpdateAt {
                dJumps = (*pMCVar).lJumps as libc::c_double / *pnTotal as libc::c_double;
                if dJumps > 0.3f64 {
                    if dJumps == 1 as libc::c_int as libc::c_double {
                        if (*pMCVar).dKernelSD < sqrt(1.7976931348623157e+308f64) {
                            if (*pMCVar).dKernelSD > 2 as libc::c_int as libc::c_double {
                                (*pMCVar)
                                    .dKernelSD = (*pMCVar).dKernelSD * (*pMCVar).dKernelSD;
                            } else {
                                (*pMCVar)
                                    .dKernelSD = (*pMCVar).dKernelSD
                                    * 20 as libc::c_int as libc::c_double;
                            }
                        } else {
                            (*pMCVar).dKernelSD = 1.7976931348623157e+308f64;
                        }
                    } else if (*pMCVar).dKernelSD
                        < 1.7976931348623157e+308f64 / 2 as libc::c_int as libc::c_double
                    {
                        (*pMCVar)
                            .dKernelSD = (*pMCVar).dKernelSD
                            * 2 as libc::c_int as libc::c_double;
                    } else {
                        (*pMCVar).dKernelSD = 1.7976931348623157e+308f64;
                    }
                    if (*pMCVar).dKernelSD > (*pMCVar).dMaxKernelSD {
                        (*pMCVar).dKernelSD = (*pMCVar).dMaxKernelSD;
                    }
                } else if dJumps == 0 as libc::c_int as libc::c_double {
                    if (*pMCVar).dKernelSD > pow(2.2250738585072014e-308f64, 0.45f64) {
                        if (*pMCVar).dKernelSD > 2 as libc::c_int as libc::c_double {
                            (*pMCVar).dKernelSD = pow((*pMCVar).dKernelSD, 0.45f64);
                        } else {
                            (*pMCVar).dKernelSD = (*pMCVar).dKernelSD * 0.04f64;
                        }
                    } else {
                        (*pMCVar).dKernelSD = 2.2250738585072014e-308f64;
                    }
                } else if (*pMCVar).dKernelSD > 2.2250738585072014e-308f64 / 0.4f64 {
                    (*pMCVar).dKernelSD = (*pMCVar).dKernelSD * 0.4f64;
                } else {
                    (*pMCVar).dKernelSD = 2.2250738585072014e-308f64;
                }
                (*pMCVar).lJumps = 0 as libc::c_int as libc::c_long;
            }
            if *((*pgd).rgdPerks).offset(*pindexT as isize)
                > 0 as libc::c_int as libc::c_double
            {
                old_dKernelSD = (*pMCVar).dKernelSD;
                (*pMCVar)
                    .dKernelSD = (*pMCVar).dKernelSD
                    * exp(
                        (1 as libc::c_int as libc::c_double
                            - *((*pgd).rgdPerks).offset(*pindexT as isize))
                            * 1.837877066409345339082f64 * 0.25f64
                            - 0.75f64 * log(*((*pgd).rgdPerks).offset(*pindexT as isize)),
                    );
                if (*pMCVar).dKernelSD > (*pMCVar).dMaxKernelSD {
                    (*pMCVar).dKernelSD = (*pMCVar).dMaxKernelSD;
                }
                dLnKern = log(
                    CDFNormal((MaxMCVar(pMCVar) - (*pMCVar).dVal) / (*pMCVar).dKernelSD)
                        - CDFNormal(
                            (MinMCVar(pMCVar) - (*pMCVar).dVal) / (*pMCVar).dKernelSD,
                        ),
                );
                (*pMCVar).dVal = SampleTheta(pMCVar);
                dLnKernNew = log(
                    CDFNormal((MaxMCVar(pMCVar) - (*pMCVar).dVal) / (*pMCVar).dKernelSD)
                        - CDFNormal(
                            (MinMCVar(pMCVar) - (*pMCVar).dVal) / (*pMCVar).dKernelSD,
                        ),
                );
            } else if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long {
                (*pMCVar).dVal = SampleThetaUnif(pMCVar);
            } else {
                CalculateOneMCParm(pMCVar);
            }
            dLnPriorNew = LnDensity(pMCVar, panal);
            dLnLikeNew = LnLike(pMCVar, panal);
            dLnDataNew = 0.0f64;
            if (*pMCVar).bExptIsDep != 0 {
                if TraverseLevels1(
                    plevel,
                    Some(
                        RunExpt
                            as unsafe extern "C" fn(
                                PLEVEL,
                                *mut *mut libc::c_char,
                            ) -> libc::c_int,
                    ),
                    panal,
                    &mut dLnDataNew as *mut libc::c_double,
                    0 as *mut libc::c_void,
                ) == 0
                {
                    (*pMCVar).dVal = dTheta;
                    TraverseLevels1(
                        plevel,
                        Some(
                            RestoreLikelihoods
                                as unsafe extern "C" fn(
                                    PLEVEL,
                                    *mut *mut libc::c_char,
                                ) -> libc::c_int,
                        ),
                        0 as *mut libc::c_void,
                    );
                    current_block = 6043525425345113293;
                } else {
                    current_block = 4741994311446740739;
                }
            } else {
                current_block = 4741994311446740739;
            }
            match current_block {
                6043525425345113293 => {}
                _ => {
                    if TestImpRatioTemper(
                        pgd,
                        (*pMCVar).bExptIsDep,
                        dLnKern,
                        dLnKernNew,
                        dLnPrior,
                        dLnPriorNew,
                        dLnLike,
                        dLnLikeNew,
                        dLnData,
                        dLnDataNew,
                        *pindexT,
                    ) == 0
                    {
                        (*pMCVar).dVal = dTheta;
                        if (*pMCVar).bExptIsDep != 0 {
                            TraverseLevels1(
                                plevel,
                                Some(
                                    RestoreLikelihoods
                                        as unsafe extern "C" fn(
                                            PLEVEL,
                                            *mut *mut libc::c_char,
                                        ) -> libc::c_int,
                                ),
                                0 as *mut libc::c_void,
                            );
                        }
                    } else {
                        (*pMCVar)
                            .lJumps = (*pMCVar).lJumps
                            + 1 as libc::c_int as libc::c_long;
                        if (*pMCVar).bExptIsDep != 0 {
                            TraverseLevels1(
                                plevel,
                                Some(
                                    SaveLikelihoods
                                        as unsafe extern "C" fn(
                                            PLEVEL,
                                            *mut *mut libc::c_char,
                                        ) -> libc::c_int,
                                ),
                                0 as *mut libc::c_void,
                            );
                        }
                    }
                    if *((*pgd).rgdPerks).offset(*pindexT as isize)
                        > 0 as libc::c_int as libc::c_double
                    {
                        (*pMCVar).dKernelSD = old_dKernelSD;
                    }
                }
            }
        }
        if (*pnIter + 1 as libc::c_int as libc::c_long) % (*pgd).nPrintFreq
            == 0 as libc::c_int as libc::c_long
            && *pnIter >= (*pgd).nMaxIter - (*pgd).nPrintIter
        {
            fprintf(
                (*pgd).pfileOut,
                b"%5g\t\0" as *const u8 as *const libc::c_char,
                (*pMCVar).dVal,
            );
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn SampleThetaVector(
    mut pLevel: PLEVEL,
    mut panal: PANALYSIS,
    mut nThetas: libc::c_long,
    mut pdTheta: *mut libc::c_double,
    mut pdSum: *mut libc::c_double,
    mut prgdSumProd: *mut *mut libc::c_double,
    mut iter: libc::c_long,
    mut nUpdateAt: libc::c_long,
    mut nTotal: libc::c_long,
    mut pdLnPrior: PDOUBLE,
    mut pdLnData: PDOUBLE,
) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut dTmp: libc::c_double = 0.;
    let mut dAccept: libc::c_double = 0.;
    let mut dLnPrior_old: libc::c_double = 0.;
    let mut dLnData_old: libc::c_double = 0.;
    let mut bInBounds: BOOL = 0;
    static mut lAccepted: libc::c_long = 0 as libc::c_int as libc::c_long;
    static mut dJumpSpread: libc::c_double = 0.;
    static mut pdTheta_old: PDOUBLE = 0 as *const libc::c_double as PDOUBLE;
    static mut prgdComponent: *mut PDOUBLE = 0 as *const PDOUBLE as *mut PDOUBLE;
    static mut prgdVariance: *mut PDOUBLE = 0 as *const PDOUBLE as *mut PDOUBLE;
    static mut dNormVar: PDOUBLE = 0 as *const libc::c_double as *mut libc::c_double;
    if pdTheta_old.is_null() || iter == nUpdateAt {
        if pdTheta_old.is_null() {
            pdTheta_old = InitdVector(nThetas);
            if pdTheta_old.is_null()
                || {
                    dNormVar = InitdVector(nThetas);
                    dNormVar.is_null()
                }
                || {
                    prgdVariance = InitdMatrix(nThetas, nThetas);
                    prgdVariance.is_null()
                }
                || {
                    prgdComponent = InitdMatrix(nThetas, nThetas);
                    prgdComponent.is_null()
                }
            {
                ReportRunTimeError(
                    panal,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"SampleThetaVector\0" as *const u8 as *const libc::c_char,
                );
            }
            dJumpSpread = 2.4f64 / sqrt(nThetas as libc::c_double);
        } else {
            dAccept = lAccepted as libc::c_double / nTotal as libc::c_double;
            if dAccept > 0.3f64 {
                dJumpSpread = dJumpSpread * 1.5f64;
            } else if dAccept < 0.15f64 {
                dJumpSpread = dJumpSpread * 0.7f64;
            }
            printf(
                b"Monitoring: iter\t%ld\t\0" as *const u8 as *const libc::c_char,
                iter,
            );
            printf(
                b"success rate\t%g\tspread\t%g\n\0" as *const u8 as *const libc::c_char,
                dAccept,
                dJumpSpread,
            );
            lAccepted = 0 as libc::c_int as libc::c_long;
        }
        i = 0 as libc::c_int as libc::c_long;
        while i < nThetas {
            j = 0 as libc::c_int as libc::c_long;
            while j < nThetas {
                *(*prgdVariance.offset(i as isize))
                    .offset(
                        j as isize,
                    ) = (*(*prgdSumProd.offset(i as isize)).offset(j as isize)
                    - *pdSum.offset(i as isize) * *pdSum.offset(j as isize)
                        / (iter + 1 as libc::c_int as libc::c_long) as libc::c_double)
                    / iter as libc::c_double;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        if Cholesky(prgdVariance, prgdComponent, nThetas) == 0 {
            i = 0 as libc::c_int as libc::c_long;
            while i < nThetas {
                j = 0 as libc::c_int as libc::c_long;
                while j < nThetas {
                    if i == j {
                        *(*prgdVariance.offset(i as isize))
                            .offset(
                                j as isize,
                            ) = *(*prgdSumProd.offset(i as isize)).offset(j as isize)
                            / iter as libc::c_double;
                    } else {
                        *(*prgdVariance.offset(i as isize)).offset(j as isize) = 0.0f64;
                    }
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            if Cholesky(prgdVariance, prgdComponent, nThetas) == 0 {
                printf(
                    b"Error: impossible to compute a jumping kernel - Exiting.(You should check or change the restart file).\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
        }
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < nThetas {
        *pdTheta_old.offset(i as isize) = *pdTheta.offset(i as isize);
        i += 1;
        i;
    }
    dLnPrior_old = *pdLnPrior;
    dLnData_old = *pdLnData;
    i = 0 as libc::c_int as libc::c_long;
    while i < nThetas {
        *dNormVar.offset(i as isize) = NormalRandom(0.0f64, 1.0f64);
        i += 1;
        i;
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < nThetas {
        dTmp = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int as libc::c_long;
        while j <= i {
            dTmp = dTmp
                + *dNormVar.offset(j as isize)
                    * *(*prgdComponent.offset(i as isize)).offset(j as isize);
            j += 1;
            j;
        }
        *pdTheta
            .offset(i as isize) = *pdTheta_old.offset(i as isize) + dJumpSpread * dTmp;
        i += 1;
        i;
    }
    let mut iTmp: libc::c_long = 0 as libc::c_int as libc::c_long;
    bInBounds = TraverseLevels1(
        pLevel,
        Some(
            SetMCVars
                as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> libc::c_int,
        ),
        pdTheta,
        &mut iTmp as *mut libc::c_long,
        0 as *mut libc::c_void,
    );
    if bInBounds == 0 {
        i = 0 as libc::c_int as libc::c_long;
        while i < nThetas {
            *pdTheta.offset(i as isize) = *pdTheta_old.offset(i as isize);
            i += 1;
            i;
        }
    } else {
        *pdLnPrior = 0.0f64;
        TraverseLevels(
            pLevel,
            Some(
                CalculateTotals
                    as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
            ),
            panal,
            pdLnPrior,
            0 as *mut libc::c_void,
        );
        *pdLnData = 0.0f64;
        if RunAllExpts(panal, pdLnData) == 0 {
            i = 0 as libc::c_int as libc::c_long;
            while i < nThetas {
                *pdTheta.offset(i as isize) = *pdTheta_old.offset(i as isize);
                i += 1;
                i;
            }
            *pdLnPrior = dLnPrior_old;
            *pdLnData = dLnData_old;
        } else if log(Randoms()) > *pdLnPrior + *pdLnData - dLnPrior_old - dLnData_old {
            i = 0 as libc::c_int as libc::c_long;
            while i < nThetas {
                *pdTheta.offset(i as isize) = *pdTheta_old.offset(i as isize);
                i += 1;
                i;
            }
            *pdLnPrior = dLnPrior_old;
            *pdLnData = dLnData_old;
        } else {
            lAccepted += 1;
            lAccepted;
        }
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < nThetas {
        *pdSum.offset(i as isize) += *pdTheta.offset(i as isize);
        j = 0 as libc::c_int as libc::c_long;
        while j < nThetas {
            *(*prgdSumProd.offset(i as isize)).offset(j as isize)
                += *pdTheta.offset(i as isize) * *pdTheta.offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn SaveLikelihoods(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pExpt: PEXPERIMENT = (*plevel).pexpt;
    if !pExpt.is_null() {
        (*pExpt).dLnLikeSave = (*pExpt).dLnLike;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn SetFixedVars(mut plevel: PLEVEL) {
    let mut n: libc::c_long = 0;
    let mut pFVar: PVARMOD = 0 as *mut tagVARMODIFICATION;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nFixedVars {
        pFVar = *((*plevel).rgpFixedVars).offset(n as isize);
        if IsInput((*pFVar).hvar) != 0 {
            SetInput((*pFVar).hvar, (*pFVar).uvar.pifn);
        } else {
            SetVar((*pFVar).hvar, (*pFVar).uvar.dVal);
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn SetKernel(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut useMCVarVals: intptr_t = *args.offset(0 as libc::c_int as isize) as intptr_t;
    let mut pdMCVarVals: *mut libc::c_double = *args.offset(1 as libc::c_int as isize)
        as *mut libc::c_double;
    let mut dMin: libc::c_double = 0.;
    let mut dMax: libc::c_double = 0.;
    let mut dTmp: libc::c_double = 0.;
    let mut n: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    static mut nThetas: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        if (**((*plevel).rgpMCVars).offset(n as isize)).bIsFixed == 0 {
            pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
            CalculateOneMCParm(pMCVar);
            if (*pMCVar).iType == 0 as libc::c_int || (*pMCVar).iType == 1 as libc::c_int
            {
                (*pMCVar)
                    .dMaxKernelSD = *(*pMCVar).pdParm[1 as libc::c_int as usize] / 6.0f64
                    - *(*pMCVar).pdParm[0 as libc::c_int as usize] / 6.0f64;
            } else {
                (*pMCVar)
                    .dMaxKernelSD = *(*pMCVar).pdParm[3 as libc::c_int as usize] / 6.0f64
                    - *(*pMCVar).pdParm[2 as libc::c_int as usize] / 6.0f64;
            }
            dMax = (*pMCVar).dVal;
            dMin = dMax;
            m = 0 as libc::c_int as libc::c_long;
            while m < 3 as libc::c_int as libc::c_long {
                CalculateOneMCParm(pMCVar);
                dTmp = (*pMCVar).dVal;
                if dMin >= dTmp {
                    dMin = dTmp;
                } else if dMax < dTmp {
                    dMax = dTmp;
                }
                m += 1;
                m;
            }
            if *(*pMCVar).pdParm[2 as libc::c_int as usize]
                == -1.7976931348623157e+308f64
                || *(*pMCVar).pdParm[3 as libc::c_int as usize]
                    == 1.7976931348623157e+308f64
            {
                (*pMCVar).dKernelSD = 0.5f64 * dMax - 0.5f64 * dMin;
            } else {
                (*pMCVar).dKernelSD = dMax - dMin;
            }
            if (*pMCVar).dKernelSD == 0 as libc::c_int as libc::c_double {
                (*pMCVar).dKernelSD = (*pMCVar).dMaxKernelSD;
            }
            if (*pMCVar).dKernelSD > (*pMCVar).dMaxKernelSD {
                (*pMCVar).dKernelSD = (*pMCVar).dMaxKernelSD;
            }
        }
        if useMCVarVals == 1 as libc::c_int as libc::c_long {
            let fresh15 = nThetas;
            nThetas = nThetas + 1;
            (**((*plevel).rgpMCVars).offset(n as isize))
                .dVal = *pdMCVarVals.offset(fresh15 as isize);
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn WriteKernel(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut pfile: *mut FILE = *args.offset(0 as libc::c_int as isize) as *mut FILE;
    let mut n: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        if (**((*plevel).rgpMCVars).offset(n as isize)).bIsFixed == 0 {
            pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
            fprintf(
                pfile,
                b"%lg\t\0" as *const u8 as *const libc::c_char,
                (*pMCVar).dKernelSD,
            );
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn ReadKernel(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut pfile: *mut FILE = *args.offset(0 as libc::c_int as isize) as *mut FILE;
    let mut n: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        if (**((*plevel).rgpMCVars).offset(n as isize)).bIsFixed == 0 {
            pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
            (*pMCVar).dMaxKernelSD = (MaxMCVar(pMCVar) - MinMCVar(pMCVar)) / 6.0f64;
            if fscanf(
                pfile,
                b"%lg\0" as *const u8 as *const libc::c_char,
                &mut (*pMCVar).dKernelSD as *mut libc::c_double,
            ) == 0
            {
                ReportError(
                    0 as PINPUTBUF,
                    (0x5 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"kernel file\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn SetModelVars(mut plevel: PLEVEL) {
    let mut n: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
        if (*pMCVar).bIsFixed == 0 && IsParm((*pMCVar).hvar) != 0 {
            SetVar((*pMCVar).hvar, (*pMCVar).dVal);
        }
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn SetMCVars(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pdMCVarVals: *mut libc::c_double = *args.offset(0 as libc::c_int as isize)
        as *mut libc::c_double;
    let mut nThetas: *mut libc::c_long = *args.offset(1 as libc::c_int as isize)
        as *mut libc::c_long;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut dVar: libc::c_double = 0.;
    let mut n: libc::c_long = 0;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        let fresh16 = *nThetas;
        *nThetas = *nThetas + 1;
        dVar = *pdMCVarVals.offset(fresh16 as isize);
        pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
        if (*pMCVar).iType == 0 as libc::c_int || (*pMCVar).iType == 1 as libc::c_int {
            if dVar < *(*pMCVar).pdParm[0 as libc::c_int as usize]
                || dVar > *(*pMCVar).pdParm[1 as libc::c_int as usize]
            {
                return 0 as libc::c_int;
            }
        } else if dVar < *(*pMCVar).pdParm[2 as libc::c_int as usize]
            || dVar > *(*pMCVar).pdParm[3 as libc::c_int as usize]
        {
            return 0 as libc::c_int
        }
        (*pMCVar).dVal = dVar;
        n += 1;
        n;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn SetPointers(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut pos: POUTSPEC = 0 as *mut tagOUTSPEC;
    let mut bFound: BOOL = 0;
    i = 0 as libc::c_int as libc::c_long;
    while i < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(i as isize);
        j = 0 as libc::c_int as libc::c_long;
        while j < 4 as libc::c_int as libc::c_long {
            if ((*pMCVar).pMCVParent[j as usize]).is_null() {
                (*pMCVar)
                    .pdParm[j
                    as usize] = &mut *((*pMCVar).dParm).as_mut_ptr().offset(j as isize)
                    as *mut libc::c_double;
            } else {
                (*pMCVar)
                    .pdParm[j
                    as usize] = &mut (**((*pMCVar).pMCVParent)
                    .as_mut_ptr()
                    .offset(j as isize))
                    .dVal;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if !((*plevel).pexpt).is_null() {
        i = 0 as libc::c_int as libc::c_long;
        while i < (*plevel).nLikes {
            pMCVar = *((*plevel).rgpLikes).offset(i as isize);
            pos = &mut (*(*plevel).pexpt).os;
            bFound = 0 as libc::c_int;
            j = 0 as libc::c_int as libc::c_long;
            while j < (*pos).nData as libc::c_long && bFound == 0 {
                bFound = ((*pMCVar).hvar == *((*pos).phvar_dat).offset(j as isize))
                    as libc::c_int;
                if bFound == 0 {
                    j += 1;
                    j;
                }
            }
            if bFound != 0 {
                (*pMCVar).pdVal = *((*pos).prgdDataVals).offset(j as isize);
                (*pMCVar).lCount = *((*pos).pcData).offset(j as isize) as libc::c_long;
            } else {
                printf(
                    b"Error: no Data for %s in Simulation %d - Exiting.\n\n\0"
                        as *const u8 as *const libc::c_char,
                    (*pMCVar).pszName,
                    (*(*plevel).pexpt).iExp,
                );
                exit(0 as libc::c_int);
            }
            j = 0 as libc::c_int as libc::c_long;
            while j < 4 as libc::c_int as libc::c_long {
                if (*pMCVar).iParmType[j as usize] == 2 as libc::c_int {
                    bFound = 0 as libc::c_int;
                    k = 0 as libc::c_int as libc::c_long;
                    while k < (*pos).nOutputs as libc::c_long && bFound == 0 {
                        bFound = ((*pMCVar).hParm[j as usize]
                            == *((*pos).phvar_out).offset(k as isize)) as libc::c_int;
                        if bFound == 0 {
                            k += 1;
                            k;
                        }
                    }
                    if bFound != 0 {
                        (*pMCVar)
                            .pdParm[j
                            as usize] = &mut *(*((*pos).prgdOutputVals)
                            .offset(k as isize))
                            .offset(0 as libc::c_int as isize) as *mut libc::c_double;
                    } else {
                        printf(
                            b"Error: missing Print statement for parameter number %ld\nof %s distribution - Exiting.\n\n\0"
                                as *const u8 as *const libc::c_char,
                            j,
                            (*pMCVar).pszName,
                        );
                        exit(0 as libc::c_int);
                    }
                } else if (*pMCVar).iParmType[j as usize] == 3 as libc::c_int {
                    bFound = 0 as libc::c_int;
                    k = 0 as libc::c_int as libc::c_long;
                    while k < (*pos).nData as libc::c_long && bFound == 0 {
                        bFound = ((*pMCVar).hParm[j as usize]
                            == *((*pos).phvar_dat).offset(k as isize)) as libc::c_int;
                        if bFound == 0 {
                            k += 1;
                            k;
                        }
                    }
                    if bFound != 0 {
                        (*pMCVar)
                            .pdParm[j
                            as usize] = &mut *(*((*pos).prgdDataVals).offset(k as isize))
                            .offset(0 as libc::c_int as isize) as *mut libc::c_double;
                    } else {
                        printf(
                            b"Error: no Data for %s in Simulation %d - Exiting.\n\n\0"
                                as *const u8 as *const libc::c_char,
                            (*pMCVar).pszName,
                            (*(*plevel).pexpt).iExp,
                        );
                        exit(0 as libc::c_int);
                    }
                } else if ((*pMCVar).pMCVParent[j as usize]).is_null() {
                    (*pMCVar)
                        .pdParm[j
                        as usize] = &mut *((*pMCVar).dParm)
                        .as_mut_ptr()
                        .offset(j as isize) as *mut libc::c_double;
                } else {
                    (*pMCVar)
                        .pdParm[j
                        as usize] = &mut (**((*pMCVar).pMCVParent)
                        .as_mut_ptr()
                        .offset(j as isize))
                        .dVal;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
}
pub unsafe extern "C" fn SumAllExpts(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pdLnData: *mut libc::c_double = *args.offset(0 as libc::c_int as isize)
        as *mut libc::c_double;
    let mut pExpt: PEXPERIMENT = (*plevel).pexpt;
    if !pExpt.is_null() {
        *pdLnData += (*pExpt).dLnLike;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn TestImpRatio(
    mut pgd: PGIBBSDATA,
    mut bExptIsDep: BOOL,
    mut dLnKern: libc::c_double,
    mut dLnKernNew: libc::c_double,
    mut dLnPrior: libc::c_double,
    mut dLnPriorNew: libc::c_double,
    mut dLnLike: libc::c_double,
    mut dLnLikeNew: libc::c_double,
    mut dLnData: libc::c_double,
    mut dLnDataNew: libc::c_double,
) -> BOOL {
    let mut dPjump: libc::c_double = 0.;
    if dLnKernNew == -1.0E+100f64 || dLnPriorNew == -1.0E+100f64
        || dLnLikeNew == -1.0E+100f64 || dLnDataNew == -1.0E+100f64
    {
        return 0 as libc::c_int;
    }
    dPjump = dLnPriorNew - dLnPrior + dLnLikeNew - dLnLike + dLnKern - dLnKernNew;
    if bExptIsDep != 0 {
        dPjump += dLnDataNew - dLnData;
    }
    if (*pgd).nSimTypeFlag == 0 as libc::c_int as libc::c_long {
        return (log(Randoms()) <= dPjump) as libc::c_int
    } else if (*pgd).nSimTypeFlag == 5 as libc::c_int as libc::c_long {
        return (0 as libc::c_int as libc::c_double <= dPjump) as libc::c_int
    } else {
        printf(
            b"Error: simTypeFlag should be 0 or 5 in TestImpRatio - Exiting.\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    };
}
pub unsafe extern "C" fn TestImpRatioTemper(
    mut pgd: PGIBBSDATA,
    mut bExptIsDep: BOOL,
    mut dLnKern: libc::c_double,
    mut dLnKernNew: libc::c_double,
    mut dLnPrior: libc::c_double,
    mut dLnPriorNew: libc::c_double,
    mut dLnLike: libc::c_double,
    mut dLnLikeNew: libc::c_double,
    mut dLnData: libc::c_double,
    mut dLnDataNew: libc::c_double,
    mut indexT: libc::c_long,
) -> BOOL {
    let mut dPjump: libc::c_double = 0.;
    if dLnPriorNew == -1.0E+100f64 || dLnLikeNew == -1.0E+100f64
        || dLnDataNew == -1.0E+100f64
    {
        return 0 as libc::c_int;
    }
    if *((*pgd).rgdPerks).offset(indexT as isize) == 0 as libc::c_int as libc::c_double {
        return 1 as libc::c_int;
    }
    if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long {
        dPjump = *((*pgd).rgdPerks).offset(indexT as isize)
            * (dLnPriorNew - dLnPrior + dLnLikeNew - dLnLike) + dLnKern - dLnKernNew;
    } else {
        dPjump = dLnPriorNew - dLnPrior
            + *((*pgd).rgdPerks).offset(indexT as isize) * (dLnLikeNew - dLnLike)
            + dLnKern - dLnKernNew;
    }
    if bExptIsDep != 0 {
        dPjump += *((*pgd).rgdPerks).offset(indexT as isize) * (dLnDataNew - dLnData);
    }
    return (log(Randoms()) <= dPjump) as libc::c_int;
}
pub unsafe extern "C" fn TestTemper(
    mut pgd: PGIBBSDATA,
    mut indexT: libc::c_long,
    mut indexT_new: libc::c_long,
    mut dLnPrior: libc::c_double,
    mut dLnData: libc::c_double,
    mut pseudo: libc::c_double,
    mut pseudonew: libc::c_double,
) -> BOOL {
    let mut dPjump: libc::c_double = 0 as libc::c_int as libc::c_double;
    if dLnPrior + dLnData == -1.0E+100f64 {
        return 0 as libc::c_int;
    }
    if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long {
        dPjump = (*((*pgd).rgdPerks).offset(indexT_new as isize)
            - *((*pgd).rgdPerks).offset(indexT as isize)) * (dLnPrior + dLnData)
            + pseudonew - pseudo
            + (if indexT_new == 0 as libc::c_int as libc::c_long
                || indexT_new == ((*pgd).nPerks - 1 as libc::c_int) as libc::c_long
            {
                0 as libc::c_int as libc::c_double
            } else {
                -0.6931471805599452862268f64
            })
            - (if indexT == 0 as libc::c_int as libc::c_long
                || indexT == ((*pgd).nPerks - 1 as libc::c_int) as libc::c_long
            {
                0 as libc::c_int as libc::c_double
            } else {
                -0.6931471805599452862268f64
            });
    } else {
        dPjump = (*((*pgd).rgdPerks).offset(indexT_new as isize)
            - *((*pgd).rgdPerks).offset(indexT as isize)) * dLnData + pseudonew - pseudo
            + (if indexT_new == 0 as libc::c_int as libc::c_long
                || indexT_new == ((*pgd).nPerks - 1 as libc::c_int) as libc::c_long
            {
                0 as libc::c_int as libc::c_double
            } else {
                -0.6931471805599452862268f64
            })
            - (if indexT == 0 as libc::c_int as libc::c_long
                || indexT == ((*pgd).nPerks - 1 as libc::c_int) as libc::c_long
            {
                0 as libc::c_int as libc::c_double
            } else {
                -0.6931471805599452862268f64
            });
    }
    return (log(Randoms()) <= dPjump) as libc::c_int;
}
pub unsafe extern "C" fn TraverseLevels(
    mut plevel: PLEVEL,
    mut routinePtr: Option::<unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> ()>,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    static mut arg: [*mut libc::c_char; 25] = [0 as *const libc::c_char
        as *mut libc::c_char; 25];
    static mut args_0: *mut *mut libc::c_char = unsafe { arg.as_ptr() as *mut _ };
    let mut arg1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_long = 0;
    let mut nargs: libc::c_long = 0 as libc::c_int as libc::c_long;
    ap = args.clone();
    arg1 = ap.arg::<*mut libc::c_char>();
    if !arg1.is_null() {
        arg[0 as libc::c_int as usize] = arg1;
        loop {
            nargs += 1;
            arg[nargs as usize] = ap.arg::<*mut libc::c_char>();
            if (arg[nargs as usize]).is_null() {
                break;
            }
        }
    }
    routinePtr.unwrap()(plevel, args_0);
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).iInstances as libc::c_long {
        TraverseLevels(
            (*plevel).pLevels[n as usize],
            routinePtr,
            0 as *mut libc::c_void,
        );
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn TraverseLevels1(
    mut plevel: PLEVEL,
    mut routinePtr: Option::<
        unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> libc::c_int,
    >,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::std::ffi::VaListImpl;
    static mut arg: [*mut libc::c_char; 25] = [0 as *const libc::c_char
        as *mut libc::c_char; 25];
    static mut args_0: *mut *mut libc::c_char = unsafe { arg.as_ptr() as *mut _ };
    let mut arg1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_long = 0;
    let mut nargs: libc::c_long = 0 as libc::c_int as libc::c_long;
    ap = args.clone();
    arg1 = ap.arg::<*mut libc::c_char>();
    if !arg1.is_null() {
        arg[0 as libc::c_int as usize] = arg1;
        loop {
            nargs += 1;
            arg[nargs as usize] = ap.arg::<*mut libc::c_char>();
            if (arg[nargs as usize]).is_null() {
                break;
            }
        }
    }
    if routinePtr.unwrap()(plevel, args_0) != 0 {
        n = 0 as libc::c_int as libc::c_long;
        while n < (*plevel).iInstances as libc::c_long {
            if TraverseLevels1(
                (*plevel).pLevels[n as usize],
                routinePtr,
                0 as *mut libc::c_void,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            n += 1;
            n;
        }
    } else {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn WriteHeader(mut panal: PANALYSIS) {
    let mut pgd: PGIBBSDATA = &mut (*panal).gd;
    let mut i: libc::c_long = 0;
    fprintf((*pgd).pfileOut, b"iter\t\0" as *const u8 as *const libc::c_char);
    TraverseLevels(
        (*panal).pLevels[0 as libc::c_int as usize],
        Some(
            WriteParameterNames
                as unsafe extern "C" fn(PLEVEL, *mut *mut libc::c_char) -> (),
        ),
        panal,
        (*pgd).pfileOut,
        0 as *mut libc::c_void,
    );
    if (*pgd).nSimTypeFlag == 3 as libc::c_int as libc::c_long
        || (*pgd).nSimTypeFlag == 4 as libc::c_int as libc::c_long
    {
        fprintf((*pgd).pfileOut, b"IndexT\t\0" as *const u8 as *const libc::c_char);
        i = 0 as libc::c_int as libc::c_long;
        while i < (*pgd).nPerks as libc::c_long {
            fprintf(
                (*pgd).pfileOut,
                b"LnPseudoPrior(%ld)\t\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int as libc::c_long,
            );
            i += 1;
            i;
        }
    }
    fprintf(
        (*pgd).pfileOut,
        b"LnPrior\tLnData\tLnPosterior\n\0" as *const u8 as *const libc::c_char,
    );
    fflush((*pgd).pfileOut);
}
pub unsafe extern "C" fn WriteParameterNames(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut panal: PANALYSIS = *args.offset(0 as libc::c_int as isize) as PANALYSIS;
    let mut outFile: PFILE = *args.offset(1 as libc::c_int as isize) as *mut FILE;
    let mut n: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    (*panal).iInstance[(*plevel).iDepth as usize] = (*plevel).iSequence;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        fprintf(
            outFile,
            b"%s(\0" as *const u8 as *const libc::c_char,
            (**((*plevel).rgpMCVars).offset(n as isize)).pszName,
        );
        m = 1 as libc::c_int as libc::c_long;
        while m < (*plevel).iDepth as libc::c_long {
            fprintf(
                outFile,
                b"%d.\0" as *const u8 as *const libc::c_char,
                (*panal).iInstance[m as usize],
            );
            m += 1;
            m;
        }
        fprintf(
            outFile,
            b"%d)\t\0" as *const u8 as *const libc::c_char,
            (*panal).iInstance[(*plevel).iDepth as usize],
        );
        n += 1;
        n;
    }
}
pub unsafe extern "C" fn WriteMCVars(
    mut plevel: PLEVEL,
    mut args: *mut *mut libc::c_char,
) {
    let mut pOutFile: PFILE = *args.offset(0 as libc::c_int as isize) as PFILE;
    let mut n: libc::c_long = 0;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    n = 0 as libc::c_int as libc::c_long;
    while n < (*plevel).nMCVars {
        pMCVar = *((*plevel).rgpMCVars).offset(n as isize);
        fprintf(
            pOutFile,
            b"%5g\t\0" as *const u8 as *const libc::c_char,
            (*pMCVar).dVal,
        );
        n += 1;
        n;
    }
}
