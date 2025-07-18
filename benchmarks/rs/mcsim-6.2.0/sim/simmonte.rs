use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn SetVar(hVar: HVAR, dVal: libc::c_double) -> BOOL;
    fn BetaRandom(
        alpha: libc::c_double,
        beta: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn BinomialBetaRandom(
        Expectation: libc::c_double,
        alpha: libc::c_double,
        beta: libc::c_double,
    ) -> libc::c_double;
    fn BinomialRandom(p: libc::c_double, n: libc::c_long) -> libc::c_double;
    fn CauchyRandom(dScale: libc::c_double) -> libc::c_double;
    fn Chi2Random(dof: libc::c_double) -> libc::c_double;
    fn ExpRandom(beta: libc::c_double) -> libc::c_double;
    fn InvGGammaRandom(alpha: libc::c_double, beta: libc::c_double) -> libc::c_double;
    fn TruncInvGGammaRandom(
        alpha: libc::c_double,
        beta: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn GGammaRandom(alpha: libc::c_double, beta: libc::c_double) -> libc::c_double;
    fn LogNormalRandom(
        dGeoMean: libc::c_double,
        dGeoSD: libc::c_double,
    ) -> libc::c_double;
    fn GenLogNormalRandom(
        dMean: libc::c_double,
        dSDNorm: libc::c_double,
        dSDLogNorm: libc::c_double,
    ) -> libc::c_double;
    fn StudentTRandom(
        dof: libc::c_double,
        dMean: libc::c_double,
        dSD: libc::c_double,
    ) -> libc::c_double;
    fn LogUniformRandom(a: libc::c_double, b: libc::c_double) -> libc::c_double;
    fn NegativeBinomialRandom(r: libc::c_double, p: libc::c_double) -> libc::c_long;
    fn NormalRandom(dMean: libc::c_double, dSD: libc::c_double) -> libc::c_double;
    fn PiecewiseRandom(
        min: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
        max: libc::c_double,
    ) -> libc::c_double;
    fn PoissonRandom(mu: libc::c_double) -> libc::c_long;
    fn TruncLogNormalRandom(
        dGeoMean: libc::c_double,
        dGeoSD: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn TruncNormalRandom(
        dMean: libc::c_double,
        dSD: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn UniformRandom(a: libc::c_double, b: libc::c_double) -> libc::c_double;
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
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
pub type PMONTECARLO = *mut tagMONTECARLO;
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
pub unsafe extern "C" fn SetParms(
    mut cParms: libc::c_long,
    mut rghvar: *mut HVAR,
    mut rgdParm: *mut libc::c_double,
) {
    let mut i: libc::c_long = 0;
    i = 0 as libc::c_int as libc::c_long;
    while i < cParms {
        SetVar(*rghvar.offset(i as isize), *rgdParm.offset(i as isize));
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn SetParmsLog(
    mut cParms: libc::c_long,
    mut rghvar: *mut HVAR,
    mut rgdParm: *mut libc::c_double,
) {
    let mut i: libc::c_long = 0;
    i = 0 as libc::c_int as libc::c_long;
    while i < cParms {
        SetVar(*rghvar.offset(i as isize), log(*rgdParm.offset(i as isize)));
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn SetParmsExp(
    mut cParms: libc::c_long,
    mut rghvar: *mut HVAR,
    mut rgdParm: *mut libc::c_double,
) {
    let mut i: libc::c_long = 0;
    i = 0 as libc::c_int as libc::c_long;
    while i < cParms {
        SetVar(*rghvar.offset(i as isize), exp(*rgdParm.offset(i as isize)));
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn CalculateOneMCParm(mut pMCVar: PMCVAR) -> libc::c_int {
    let mut dParm1: libc::c_double = 0.;
    let mut dParm2: libc::c_double = 0.;
    let mut dMin: libc::c_double = 0.;
    let mut dMax: libc::c_double = 0.;
    dParm1 = *(*pMCVar).pdParm[0 as libc::c_int as usize];
    dParm2 = *(*pMCVar).pdParm[1 as libc::c_int as usize];
    dMin = *(*pMCVar).pdParm[2 as libc::c_int as usize];
    dMax = *(*pMCVar).pdParm[3 as libc::c_int as usize];
    match (*pMCVar).iType {
        1 => {
            (*pMCVar).dVal = LogUniformRandom(dParm1, dParm2);
        }
        2 => {
            (*pMCVar).dVal = BetaRandom(dParm1, dParm2, dMin, dMax);
        }
        19 => {
            (*pMCVar).dVal = fabs(NormalRandom(dParm1, dParm2));
        }
        3 => {
            (*pMCVar).dVal = NormalRandom(dParm1, dParm2);
        }
        25 => {
            (*pMCVar).dVal = NormalRandom(dParm1, fabs(dParm1 * dParm2));
        }
        14 => {
            (*pMCVar).dVal = NormalRandom(dParm1, sqrt(dParm2));
        }
        5 => {
            (*pMCVar).dVal = TruncNormalRandom(dParm1, dParm2, dMin, dMax);
        }
        26 => {
            (*pMCVar)
                .dVal = TruncNormalRandom(dParm1, fabs(dParm1 * dParm2), dMin, dMax);
        }
        16 => {
            (*pMCVar).dVal = TruncNormalRandom(dParm1, sqrt(dParm2), dMin, dMax);
        }
        4 => {
            (*pMCVar).dVal = LogNormalRandom(dParm1, dParm2);
        }
        6 => {
            (*pMCVar).dVal = TruncLogNormalRandom(dParm1, dParm2, dMin, dMax);
        }
        15 => {
            (*pMCVar).dVal = LogNormalRandom(dParm1, exp(sqrt(dParm2)));
        }
        17 => {
            (*pMCVar).dVal = TruncLogNormalRandom(dParm1, exp(sqrt(dParm2)), dMin, dMax);
        }
        7 => {
            (*pMCVar).dVal = Chi2Random(dParm1);
        }
        8 => {
            (*pMCVar).dVal = BinomialRandom(dParm1, dParm2 as libc::c_long);
        }
        9 => {
            (*pMCVar).dVal = PiecewiseRandom(dMin, dParm1, dParm2, dMax);
        }
        10 => {
            (*pMCVar).dVal = ExpRandom(dParm1);
        }
        11 => {
            (*pMCVar).dVal = GGammaRandom(dParm1, dParm2);
        }
        13 => {
            (*pMCVar).dVal = InvGGammaRandom(dParm1, dParm2);
        }
        20 => {
            (*pMCVar).dVal = TruncInvGGammaRandom(dParm1, dParm2, dMin, dMax);
        }
        12 => {
            (*pMCVar).dVal = PoissonRandom(dParm1) as libc::c_double;
        }
        18 => {
            (*pMCVar).dVal = BinomialBetaRandom(dParm1, dParm2, dMin);
        }
        21 => {
            (*pMCVar).dVal = GenLogNormalRandom(dParm1, dParm2, dMin);
        }
        22 => {
            (*pMCVar).dVal = StudentTRandom(dParm1, dParm2, dMin);
        }
        23 => {
            (*pMCVar).dVal = CauchyRandom(dParm1);
        }
        24 => {
            (*pMCVar).dVal = fabs(CauchyRandom(dParm1));
        }
        27 => {
            ReportError(
                0 as PINPUTBUF,
                (0x101 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"UserSpecifiedLL\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        28 => {
            (*pMCVar).dVal = NegativeBinomialRandom(dParm1, dParm2) as libc::c_double;
        }
        0 | _ => {
            (*pMCVar).dVal = UniformRandom(dParm1, dParm2);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn CalcMCParms(
    mut pMC: PMONTECARLO,
    mut rgParms: *mut libc::c_double,
    mut iStart: libc::c_long,
) {
    let mut i: libc::c_long = 0;
    if rgParms.is_null() {
        rgParms = (*pMC).rgdParms;
    }
    i = iStart;
    while i < (*pMC).nParms {
        CalculateOneMCParm(*((*pMC).rgpMCVar).offset(i as isize));
        *rgParms.offset(i as isize) = (**((*pMC).rgpMCVar).offset(i as isize)).dVal;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn InitSetPoints(mut pMC: PMONTECARLO) -> BOOL {
    let mut c: libc::c_char = 0;
    let mut pfile: PFILE = 0 as *mut FILE;
    pfile = fopen(
        (*pMC).szSetPointsFilename as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if pfile.is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x3 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            (*pMC).szSetPointsFilename,
            0 as PSTR,
        );
    }
    (*pMC).pfileSetPoints = pfile;
    loop {
        c = getc((*pMC).pfileSetPoints) as libc::c_char;
        if !(c as libc::c_int != '\n' as i32) {
            break;
        }
    }
    if feof((*pMC).pfileSetPoints) != 0 {
        ReportError(
            0 as PINPUTBUF,
            (0x208 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            (*pMC).szSetPointsFilename,
            0 as PSTR,
        );
    }
    return pfile.is_null() as libc::c_int;
}
pub unsafe extern "C" fn ReadSetPoints(
    mut pMC: PMONTECARLO,
    mut rgParms: *mut libc::c_double,
) -> BOOL {
    let mut current_block: u64;
    let mut bReturn: BOOL = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut i: libc::c_long = 0;
    if rgParms.is_null() {
        rgParms = (*pMC).rgdParms;
    }
    loop {
        c = getc((*pMC).pfileSetPoints) as libc::c_char;
        if feof((*pMC).pfileSetPoints) != 0 {
            current_block = 8831408221741692167;
            break;
        }
        if !(c as libc::c_int != '\t' as i32 && c as libc::c_int != ' ' as i32) {
            current_block = 11875828834189669668;
            break;
        }
    }
    match current_block {
        11875828834189669668 => {
            i = 0 as libc::c_int as libc::c_long;
            loop {
                if !(i < (*pMC).nSetParms) {
                    current_block = 7651349459974463963;
                    break;
                }
                if feof((*pMC).pfileSetPoints) != 0
                    || fscanf(
                        (*pMC).pfileSetPoints,
                        b"%lg\0" as *const u8 as *const libc::c_char,
                        &mut (**((*pMC).rgpMCVar).offset(i as isize)).dVal
                            as *mut libc::c_double,
                    ) == -(1 as libc::c_int)
                {
                    if (*pMC).nRuns != 0 {
                        ReportError(
                            0 as PINPUTBUF,
                            (0x208 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            (*pMC).szSetPointsFilename,
                            0 as PSTR,
                        );
                    }
                    current_block = 8831408221741692167;
                    break;
                } else {
                    *rgParms
                        .offset(
                            i as isize,
                        ) = (**((*pMC).rgpMCVar).offset(i as isize)).dVal;
                    i += 1;
                    i;
                }
            }
            match current_block {
                8831408221741692167 => {}
                _ => {
                    bReturn = 1 as libc::c_int;
                    loop {
                        c = getc((*pMC).pfileSetPoints) as libc::c_char;
                        if !(c as libc::c_int != '\n' as i32
                            && feof((*pMC).pfileSetPoints) == 0)
                        {
                            break;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return bReturn;
}
pub unsafe extern "C" fn GetSPMods(
    mut panal: PANALYSIS,
    mut rgParms: *mut libc::c_double,
) -> BOOL {
    let mut bOK: BOOL = 0;
    bOK = ReadSetPoints(&mut (*panal).mc, rgParms);
    CalcMCParms(&mut (*panal).mc, rgParms, (*panal).mc.nSetParms);
    return bOK;
}
pub unsafe extern "C" fn SetParents(mut pMC: PMONTECARLO, mut iStart: libc::c_long) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut pMCVar1: PMCVAR = 0 as *mut tagMCVAR;
    let mut pMCVar2: PMCVAR = 0 as *mut tagMCVAR;
    let mut bFound: BOOL = 0;
    i = iStart;
    while i < (*pMC).nParms {
        pMCVar1 = *((*pMC).rgpMCVar).offset(i as isize);
        j = 0 as libc::c_int as libc::c_long;
        while j < 4 as libc::c_int as libc::c_long {
            if (*pMCVar1).iParmType[j as usize] == 1 as libc::c_int {
                bFound = 0 as libc::c_int;
                k = 0 as libc::c_int as libc::c_long;
                while k < i {
                    pMCVar2 = *((*pMC).rgpMCVar).offset(k as isize);
                    if (*pMCVar1).hParm[j as usize] == (*pMCVar2).hvar {
                        (*pMCVar1).pdParm[j as usize] = &mut (*pMCVar2).dVal;
                        bFound = 1 as libc::c_int;
                    }
                    k += 1;
                    k;
                }
                if bFound == 0 {
                    printf(
                        b"\nError: parents must be declared before childrens when\n       creating sampling dependencies - Exiting.\n\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
