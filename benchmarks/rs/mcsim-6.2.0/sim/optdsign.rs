use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn InitiVector(cVectors: libc::c_long) -> *mut libc::c_int;
    fn InitdVector(cVectors: libc::c_long) -> *mut libc::c_double;
    fn InitdMatrix(
        cVectors: libc::c_long,
        cElemsEach: libc::c_long,
    ) -> *mut *mut libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn InitModel();
    fn Randoms() -> libc::c_double;
    fn DoOneExperiment(pexp: PEXPERIMENT) -> libc::c_int;
    fn ModifyParms(plistParmMods: PLIST);
    fn WriteArray(pfile: *mut FILE, cElems: libc::c_long, rg: *mut libc::c_double);
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn LnDensity(pMCVar: PMCVAR, panal: PANALYSIS) -> libc::c_double;
    fn CalculateOneMCParm(pMCVar: PMCVAR) -> libc::c_int;
    fn SetParents(pMC: PMONTECARLO, iStart: libc::c_long);
    fn SetParms(cParms: libc::c_long, rghvar: *mut HVAR, rgdParm: *mut libc::c_double);
}
pub type size_t = libc::c_ulong;
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
pub type PGIBBSDATA = *mut tagGIBBSDATA;
pub type C2RustUnnamed = libc::c_uint;
pub const backward: C2RustUnnamed = 1;
pub const forward: C2RustUnnamed = 0;
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
pub const Var_Reduction: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const Shannon: C2RustUnnamed_0 = 0;
pub unsafe extern "C" fn InitOptArrays(
    mut panal: PANALYSIS,
    mut piDesign_mask: *mut *mut libc::c_int,
    mut pnDesignPts: *mut libc::c_long,
    mut pdY: *mut *mut *mut libc::c_double,
    mut pnPreds: *mut libc::c_long,
    mut pnStartDecisionPts: *mut libc::c_long,
    mut pdVariance: *mut *mut libc::c_double,
    mut pdIR: *mut *mut libc::c_double,
    mut nSims: libc::c_long,
) {
    let mut bFound: BOOL = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pos: *mut OUTSPEC = 0 as *mut OUTSPEC;
    *pnDesignPts = 0 as libc::c_int as libc::c_long;
    *pnPreds = *pnDesignPts;
    i = 0 as libc::c_int;
    while i < (*panal).expGlobal.iExp {
        pos = &mut (**((*panal).rgpExps).as_mut_ptr().offset(i as isize)).os;
        bFound = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*pos).nOutputs {
            k = 0 as libc::c_int;
            while k < *((*pos).pcOutputTimes).offset(j as isize) {
                if !((*pos).prgdDataVals).is_null() {
                    *pnDesignPts += 1;
                    *pnDesignPts;
                    bFound = 1 as libc::c_int;
                }
                *pnPreds += 1;
                *pnPreds;
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        if bFound != 0 {
            *pnStartDecisionPts = *pnPreds;
        }
        i += 1;
        i;
    }
    if *pnDesignPts == 0 as libc::c_int as libc::c_long {
        printf(
            b"Error: you must provide Data Statements \0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"for at least one Simulation to define design points - Exiting.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if *pnPreds == *pnDesignPts {
        printf(
            b"Error: you must provide at least one Simulation \0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"without Data Statements for utility computations - Exiting.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    *piDesign_mask = InitiVector(*pnDesignPts);
    if (*piDesign_mask).is_null()
        || {
            *pdVariance = InitdVector(*pnDesignPts);
            (*pdVariance).is_null()
        }
        || {
            *pdIR = InitdVector(nSims);
            (*pdIR).is_null()
        }
        || {
            *pdY = InitdMatrix(nSims, *pnPreds);
            (*pdY).is_null()
        }
    {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"InitOptArrays\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
}
pub unsafe extern "C" fn OpenOptFiles(mut panal: PANALYSIS) {
    if (*panal).bCommandLineSpec != 0 {
        (*panal).gd.szGout = (*panal).szOutfilename;
    } else if ((*panal).gd.szGout).is_null() {
        (*panal)
            .gd
            .szGout = b"simopt.default.out\0" as *const u8 as *const libc::c_char
            as PSTR;
    }
    if ((*panal).gd.pfileOut).is_null()
        && {
            (*panal)
                .gd
                .pfileOut = fopen(
                (*panal).gd.szGout as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            ((*panal).gd.pfileOut).is_null()
        }
    {
        ReportError(
            0 as PINPUTBUF,
            (0x8000 as libc::c_int | 0x3 as libc::c_int) as WORD,
            (*panal).gd.szGout,
            b"[in OpenOptFiles()]\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
}
pub unsafe extern "C" fn WriteOutHeader(
    mut panal: PANALYSIS,
    mut criterion: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pos: *mut OUTSPEC = 0 as *mut OUTSPEC;
    fprintf((*panal).gd.pfileOut, b"iter\t\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*panal).expGlobal.iExp {
        pos = &mut (**((*panal).rgpExps).as_mut_ptr().offset(i as isize)).os;
        j = 0 as libc::c_int;
        while j < (*pos).nOutputs {
            k = 0 as libc::c_int;
            while k < *((*pos).pcOutputTimes).offset(j as isize) {
                if !((*pos).prgdDataVals).is_null() {
                    fprintf(
                        (*panal).gd.pfileOut,
                        b"T%g\t\0" as *const u8 as *const libc::c_char,
                        *(*((*pos).prgdOutputTimes).offset(j as isize))
                            .offset(k as isize),
                    );
                }
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    fprintf((*panal).gd.pfileOut, b"Chosen\t\0" as *const u8 as *const libc::c_char);
    if criterion == Var_Reduction as libc::c_int {
        fprintf(
            (*panal).gd.pfileOut,
            b"Variance\tSD\tUtility\n\0" as *const u8 as *const libc::c_char,
        );
    }
    fflush((*panal).gd.pfileOut);
}
pub unsafe extern "C" fn SetupLikes(
    mut panal: PANALYSIS,
    mut nPreds: libc::c_long,
    mut pLikes: *mut *mut PMCVAR,
) {
    let mut bFound: BOOL = 0;
    let mut bLikeFound: BOOL = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut m: libc::c_long = 0;
    let mut n: libc::c_long = 0;
    let mut nPts: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut pos: *mut OUTSPEC = 0 as *mut OUTSPEC;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut pMC: PMONTECARLO = &mut (*panal).mc;
    *pLikes = malloc(
        (nPreds as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<PMCVAR>() as libc::c_ulong),
    ) as *mut PMCVAR;
    if (*pLikes).is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"SetupLikes\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < (*panal).expGlobal.iExp as libc::c_long {
        pos = &mut (**((*panal).rgpExps).as_mut_ptr().offset(i as isize)).os;
        j = 0 as libc::c_int as libc::c_long;
        while j < (*pos).nOutputs as libc::c_long {
            k = 0 as libc::c_int as libc::c_long;
            while k < *((*pos).pcOutputTimes).offset(j as isize) as libc::c_long {
                let ref mut fresh0 = *(*pLikes).offset(nPts as isize);
                *fresh0 = malloc(::std::mem::size_of::<MCVAR>() as libc::c_ulong)
                    as PMCVAR;
                if (*fresh0).is_null() {
                    ReportError(
                        0 as PINPUTBUF,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"SetupLikes\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                if !((*pos).prgdDataVals).is_null() {
                    bLikeFound = 0 as libc::c_int;
                    m = (*pMC).nSetParms;
                    while bLikeFound == 0 {
                        bLikeFound = ((**((*pMC).rgpMCVar).offset(m as isize)).hvar
                            == *((*pos).phvar_out).offset(j as isize)) as libc::c_int;
                        if bLikeFound == 0 {
                            m += 1;
                            m;
                        }
                    }
                    if bLikeFound != 0 {
                        pMCVar = *((*pMC).rgpMCVar).offset(m as isize);
                        SetParents(pMC, 0 as libc::c_int as libc::c_long);
                        m = 0 as libc::c_int as libc::c_long;
                        while m < 4 as libc::c_int as libc::c_long {
                            if (*pMCVar).iParmType[m as usize] == 2 as libc::c_int {
                                bFound = 0 as libc::c_int;
                                n = 0 as libc::c_int as libc::c_long;
                                while n < (*pos).nOutputs as libc::c_long && bFound == 0 {
                                    bFound = ((*pMCVar).hParm[m as usize]
                                        == *((*pos).phvar_out).offset(n as isize)) as libc::c_int;
                                    if bFound == 0 {
                                        n += 1;
                                        n;
                                    }
                                }
                                if bFound != 0 {
                                    (*pMCVar)
                                        .pdParm[m
                                        as usize] = &mut *(*((*pos).prgdOutputVals)
                                        .offset(n as isize))
                                        .offset(k as isize) as *mut libc::c_double;
                                } else {
                                    printf(
                                        b"Error: missing Print statement for parameter number %ld of %s distribution - Exiting.\n\n\0"
                                            as *const u8 as *const libc::c_char,
                                        j,
                                        (*pMCVar).pszName,
                                    );
                                    exit(0 as libc::c_int);
                                }
                            } else if (*pMCVar).iParmType[m as usize] == 3 as libc::c_int
                            {
                                bFound = 0 as libc::c_int;
                                n = 0 as libc::c_int as libc::c_long;
                                while n < (*pos).nData as libc::c_long && bFound == 0 {
                                    bFound = ((*pMCVar).hParm[m as usize]
                                        == *((*pos).phvar_dat).offset(n as isize)) as libc::c_int;
                                    if bFound == 0 {
                                        n += 1;
                                        n;
                                    }
                                }
                                if bFound != 0 {
                                    (*pMCVar)
                                        .pdParm[m
                                        as usize] = &mut *(*((*pos).prgdDataVals)
                                        .offset(n as isize))
                                        .offset(k as isize) as *mut libc::c_double;
                                } else {
                                    printf(
                                        b"Error: no Data for %s in Simulation %ld - Exiting.\n\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*pMCVar).pszName,
                                        i,
                                    );
                                    exit(0 as libc::c_int);
                                }
                            }
                            m += 1;
                            m;
                        }
                    } else {
                        printf(
                            b"Error: missing Likelihood for %s - Exiting.\n\n\0"
                                as *const u8 as *const libc::c_char,
                            *((*pos).pszOutputNames).offset(j as isize),
                        );
                        exit(0 as libc::c_int);
                    }
                    memcpy(
                        *(*pLikes).offset(nPts as isize) as *mut libc::c_void,
                        pMCVar as *const libc::c_void,
                        ::std::mem::size_of::<MCVAR>() as libc::c_ulong,
                    );
                } else {
                    let ref mut fresh1 = *(*pLikes).offset(nPts as isize);
                    *fresh1 = 0 as PMCVAR;
                }
                nPts = nPts + 1 as libc::c_int as libc::c_long;
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn Estimate_y(
    mut panal: PANALYSIS,
    mut pdTheta: *mut libc::c_double,
    mut pdY: *mut libc::c_double,
) -> libc::c_int {
    let mut cNPred: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pos: *mut OUTSPEC = 0 as *mut OUTSPEC;
    i = 0 as libc::c_int;
    while i < (*panal).expGlobal.iExp {
        InitModel();
        ModifyParms((*panal).expGlobal.plistParmMods);
        SetParms((*panal).mc.nSetParms, (*panal).mc.rghvar, pdTheta);
        ModifyParms((*(*panal).rgpExps[i as usize]).plistParmMods);
        if DoOneExperiment((*panal).rgpExps[i as usize]) == 0 {
            printf(
                b"Warning: Can't estimate y with parameters:\n\0" as *const u8
                    as *const libc::c_char,
            );
            WriteArray(stdout, (*panal).mc.nSetParms, pdTheta);
            fputc('\n' as i32, stdout);
            return 0 as libc::c_int;
        }
        pos = &mut (**((*panal).rgpExps).as_mut_ptr().offset(i as isize)).os;
        j = 0 as libc::c_int;
        while j < (*pos).nOutputs {
            k = 0 as libc::c_int;
            while k < *((*pos).pcOutputTimes).offset(j as isize) {
                *pdY
                    .offset(
                        cNPred as isize,
                    ) = *(*((*pos).prgdOutputVals).offset(j as isize))
                    .offset(k as isize);
                cNPred += 1;
                cNPred;
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ReadAndSimulate(
    mut panal: PANALYSIS,
    mut nSetParms: libc::c_long,
    mut pdY: *mut *mut libc::c_double,
    mut nPreds: libc::c_long,
    mut pLikes: *mut PMCVAR,
    mut nSims: libc::c_long,
) {
    let mut c: libc::c_char = 0;
    let mut lDummy: libc::c_long = 0;
    let mut iter: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut j: libc::c_long = 0;
    let mut pdTheta: PDOUBLE = 0 as PDOUBLE;
    let mut pdTheta_0: PDOUBLE = 0 as PDOUBLE;
    let mut pdData: PDOUBLE = 0 as PDOUBLE;
    let mut pdData_old: PDOUBLE = 0 as PDOUBLE;
    let mut pfileRestart: *mut FILE = (*panal).gd.pfileRestart;
    pdTheta = InitdVector(nSetParms);
    if pdTheta.is_null()
        || {
            pdTheta_0 = InitdVector(nSetParms);
            pdTheta_0.is_null()
        }
        || {
            pdData_old = InitdVector(nPreds);
            pdData_old.is_null()
        }
        || {
            pdData = InitdVector(nPreds);
            pdData.is_null()
        }
    {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ReadAndSimulate\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    if pfileRestart.is_null()
        && {
            pfileRestart = fopen(
                (*panal).gd.szGrestart as *const libc::c_char,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            pfileRestart.is_null()
        }
    {
        ReportError(
            0 as PINPUTBUF,
            (0x8000 as libc::c_int | 0x3 as libc::c_int) as WORD,
            (*panal).gd.szGrestart,
            b"[in ReadAndSimulate()]\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
    loop {
        c = getc(pfileRestart) as libc::c_char;
        if !(c as libc::c_int != '\n' as i32) {
            break;
        }
    }
    while iter < nSims
        && !(feof(pfileRestart) != 0
            || fscanf(
                pfileRestart,
                b"%ld\0" as *const u8 as *const libc::c_char,
                &mut lDummy as *mut libc::c_long,
            ) == -(1 as libc::c_int))
    {
        j = 0 as libc::c_int as libc::c_long;
        while j < nSetParms {
            if fscanf(
                pfileRestart,
                b"%lg\0" as *const u8 as *const libc::c_char,
                &mut *pdTheta.offset(j as isize) as *mut libc::c_double,
            ) == 0
            {
                ReportError(
                    0 as PINPUTBUF,
                    (0x5 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    (*panal).gd.szGrestart,
                    0 as PSTR,
                );
            } else {
                (**((*panal).mc.rgpMCVar).offset(j as isize))
                    .dVal = *pdTheta.offset(j as isize);
            }
            j += 1;
            j;
        }
        loop {
            c = getc(pfileRestart) as libc::c_char;
            if !(c as libc::c_int != '\n' as i32) {
                break;
            }
        }
        Estimate_y(panal, pdTheta, *pdY.offset(iter as isize));
        if iter == 0 as libc::c_int as libc::c_long {
            j = 0 as libc::c_int as libc::c_long;
            while j < nSetParms {
                *pdTheta_0.offset(j as isize) = *pdTheta.offset(j as isize);
                j += 1;
                j;
            }
            j = 0 as libc::c_int as libc::c_long;
            while j < nPreds {
                if !(*pLikes.offset(j as isize)).is_null() {
                    CalculateOneMCParm(*pLikes.offset(j as isize));
                    *pdData_old.offset(j as isize) = (**pLikes.offset(j as isize)).dVal;
                }
                j += 1;
                j;
            }
        } else {
            j = 0 as libc::c_int as libc::c_long;
            while j < nPreds {
                if !(*pLikes.offset(j as isize)).is_null() {
                    CalculateOneMCParm(*pLikes.offset(j as isize));
                    *pdData.offset(j as isize) = (**pLikes.offset(j as isize)).dVal;
                }
                j += 1;
                j;
            }
            j = 0 as libc::c_int as libc::c_long;
            while j < nPreds {
                if !(*pLikes.offset(j as isize)).is_null() {
                    (**pLikes.offset(j as isize)).dVal = *pdData_old.offset(j as isize);
                    *(*pdY.offset(iter as isize))
                        .offset(
                            j as isize,
                        ) = LnDensity(*pLikes.offset(j as isize), panal);
                    *pdData_old.offset(j as isize) = *pdData.offset(j as isize);
                }
                j += 1;
                j;
            }
            if iter == nSims - 1 as libc::c_int as libc::c_long {
                Estimate_y(panal, pdTheta_0, *pdY.offset(0 as libc::c_int as isize));
                j = 0 as libc::c_int as libc::c_long;
                while j < nPreds {
                    if !(*pLikes.offset(j as isize)).is_null() {
                        (**pLikes.offset(j as isize))
                            .dVal = *pdData_old.offset(j as isize);
                        *(*pdY.offset(0 as libc::c_int as isize))
                            .offset(
                                j as isize,
                            ) = LnDensity(*pLikes.offset(j as isize), panal);
                    }
                    j += 1;
                    j;
                }
            }
        }
        iter += 1;
        iter;
    }
    if iter < nSims {
        printf(
            b"\nError: The number of lines in file %s is less than\n\0" as *const u8
                as *const libc::c_char,
            (*panal).gd.szGrestart,
        );
        printf(
            b"       the number of lines to read (%ld) - Exiting\n\0" as *const u8
                as *const libc::c_char,
            nSims,
        );
        exit(0 as libc::c_int);
    }
    fclose(pfileRestart);
}
pub unsafe extern "C" fn Do_Importance_Ratios(
    mut pdY: *mut *mut libc::c_double,
    mut pLikes: *mut PMCVAR,
    mut nSims: libc::c_long,
    mut nPreds: libc::c_long,
    mut nDesignPts: libc::c_long,
    mut piDesign_mask: *mut libc::c_int,
    mut nDesignPt_tried: libc::c_int,
    mut pdIR: *mut libc::c_double,
) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut dSumL: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut bOn: BOOL = 0;
    k = 0 as libc::c_int as libc::c_long;
    while k < nSims {
        *pdIR.offset(k as isize) = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int as libc::c_long;
        while i < nPreds {
            if !(*pLikes.offset(i as isize)).is_null() {
                if j == nDesignPt_tried as libc::c_long {
                    bOn = (*piDesign_mask.offset(j as isize) == 0) as libc::c_int;
                } else {
                    bOn = *piDesign_mask.offset(j as isize);
                }
                if bOn != 0 {
                    *pdIR
                        .offset(
                            k as isize,
                        ) = *pdIR.offset(k as isize)
                        + *(*pdY.offset(k as isize)).offset(i as isize);
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        *pdIR.offset(k as isize) = exp(*pdIR.offset(k as isize));
        dSumL = dSumL + *pdIR.offset(k as isize);
        k += 1;
        k;
    }
    k = 0 as libc::c_int as libc::c_long;
    while k < nSims {
        *pdIR.offset(k as isize) = *pdIR.offset(k as isize) / dSumL;
        k += 1;
        k;
    }
}
pub unsafe extern "C" fn DoVariance(
    mut nDim: libc::c_long,
    mut pdIR: *mut libc::c_double,
    mut pdX: *mut *mut libc::c_double,
    mut istart: libc::c_long,
    mut ifinish: libc::c_long,
) -> libc::c_double {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut ave: libc::c_double = 0.;
    let mut ss: libc::c_double = 0.;
    let mut dTmp: libc::c_double = 0.;
    ss = 0 as libc::c_int as libc::c_double;
    i = istart;
    while i < ifinish {
        ave = 0 as libc::c_int as libc::c_double;
        j = 0 as libc::c_int as libc::c_long;
        while j < nDim {
            ave = ave
                + *pdIR.offset(j as isize)
                    * log(*(*pdX.offset(j as isize)).offset(i as isize));
            j += 1;
            j;
        }
        j = 0 as libc::c_int as libc::c_long;
        while j < nDim {
            dTmp = log(*(*pdX.offset(j as isize)).offset(i as isize)) - ave;
            ss = ss + *pdIR.offset(j as isize) * dTmp * dTmp;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return ss / (ifinish - istart) as libc::c_double;
}
pub unsafe extern "C" fn Compute_utility(
    mut nDesignPts: libc::c_long,
    mut piDesign_mask: *mut libc::c_int,
    mut dUtility: *mut libc::c_double,
) {
    let mut nPts: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < nDesignPts {
        if *piDesign_mask.offset(i as isize) != 0 {
            nPts += 1;
            nPts;
        }
        i += 1;
        i;
    }
    *dUtility = (-(2 as libc::c_int) * nPts) as libc::c_double;
}
pub unsafe extern "C" fn WriteOptimOut(
    mut panal: PANALYSIS,
    mut iter: libc::c_long,
    mut nDesignPts: libc::c_long,
    mut criterion: libc::c_int,
    mut pdVariance: *mut libc::c_double,
    mut piDesign_mask: *mut libc::c_int,
    mut iCrit: libc::c_long,
    mut dCrit: libc::c_double,
    mut dUtility: libc::c_double,
) {
    let mut i: libc::c_long = 0;
    fprintf((*panal).gd.pfileOut, b"%ld\t\0" as *const u8 as *const libc::c_char, iter);
    if iCrit < nDesignPts {
        i = 0 as libc::c_int as libc::c_long;
        while i < nDesignPts {
            if (*panal).mc.style == forward as libc::c_int {
                if i == iCrit || *piDesign_mask.offset(i as isize) == 0 {
                    fprintf(
                        (*panal).gd.pfileOut,
                        b"%g\t\0" as *const u8 as *const libc::c_char,
                        *pdVariance.offset(i as isize),
                    );
                } else {
                    fprintf(
                        (*panal).gd.pfileOut,
                        b"0\t\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if *piDesign_mask.offset(i as isize) == 0 {
                fprintf(
                    (*panal).gd.pfileOut,
                    b"0\t\0" as *const u8 as *const libc::c_char,
                );
            } else {
                fprintf(
                    (*panal).gd.pfileOut,
                    b"%g\t\0" as *const u8 as *const libc::c_char,
                    *pdVariance.offset(i as isize),
                );
            }
            i += 1;
            i;
        }
        fprintf(
            (*panal).gd.pfileOut,
            b"%ld\t\0" as *const u8 as *const libc::c_char,
            iCrit + 1 as libc::c_int as libc::c_long,
        );
    } else {
        i = 0 as libc::c_int as libc::c_long;
        while i <= nDesignPts {
            fprintf((*panal).gd.pfileOut, b"0\t\0" as *const u8 as *const libc::c_char);
            i += 1;
            i;
        }
    }
    if criterion == Var_Reduction as libc::c_int {
        fprintf(
            (*panal).gd.pfileOut,
            b"%g\t%g\t%g\n\0" as *const u8 as *const libc::c_char,
            dCrit,
            sqrt(dCrit),
            dUtility,
        );
    }
    fflush((*panal).gd.pfileOut);
}
pub unsafe extern "C" fn Importance_Resample(
    mut nSims: libc::c_long,
    mut pIndex0: *mut libc::c_long,
    mut pIndex1: *mut libc::c_long,
    mut plDrawn: *mut libc::c_long,
    mut pdL: *mut libc::c_double,
    mut dSumL: libc::c_double,
) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    i = 0 as libc::c_int as libc::c_long;
    while i < nSims {
        *pdL.offset(i as isize) = *pdL.offset(i as isize) / dSumL;
        i += 1;
        i;
    }
    j = 0 as libc::c_int as libc::c_long;
    loop {
        i = floor(Randoms() * nSims as libc::c_double) as libc::c_long;
        if Randoms() < *pdL.offset(i as isize) {
            *pIndex1.offset(j as isize) = *pIndex0.offset(i as isize);
            j += 1;
            j;
            let ref mut fresh2 = *plDrawn.offset(*pIndex0.offset(i as isize) as isize);
            *fresh2 += 1;
            *fresh2;
        }
        if !(j < nSims) {
            break;
        }
    };
}
pub unsafe extern "C" fn CloseOptFiles(mut panal: PANALYSIS) {
    if !((*panal).gd.pfileOut).is_null() {
        fclose((*panal).gd.pfileOut);
        printf(
            b"\nWrote results to \"%s\"\n\0" as *const u8 as *const libc::c_char,
            (*panal).gd.szGout,
        );
    }
}
pub unsafe extern "C" fn DoOptimalDesign(mut panal: PANALYSIS) {
    let mut pgd: PGIBBSDATA = &mut (*panal).gd;
    let mut pmc: PMONTECARLO = &mut (*panal).mc;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut iBest: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut dim: libc::c_long = 0;
    let mut nDesignPts: libc::c_long = 0;
    let mut nPreds: libc::c_long = 0;
    let mut nSims: libc::c_long = (*pmc).nRuns;
    let mut piDesign_mask: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nDesignPt_tried: libc::c_long = 0;
    let mut nStartDecisionPts: libc::c_long = 0;
    let mut criterion: libc::c_int = 0;
    let mut dBest: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dUtility: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut pdIR: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdVariance: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut pdY: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut pLikes: *mut PMCVAR = 0 as *mut PMCVAR;
    criterion = Var_Reduction as libc::c_int;
    if criterion != Var_Reduction as libc::c_int {
        printf(
            b"Oooops, Shannon not implemented - exiting\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    printf(
        b"\nDoing analysis - Optimal Design %s %s - %d experiment%c\n\0" as *const u8
            as *const libc::c_char,
        if (*pmc).style == forward as libc::c_int {
            b"forward\0" as *const u8 as *const libc::c_char
        } else {
            b"backward\0" as *const u8 as *const libc::c_char
        },
        if criterion == Var_Reduction as libc::c_int {
            b"variance reduction\0" as *const u8 as *const libc::c_char
        } else {
            b"Shannon\0" as *const u8 as *const libc::c_char
        },
        (*panal).expGlobal.iExp,
        if (*panal).expGlobal.iExp > 1 as libc::c_int { 's' as i32 } else { ' ' as i32 },
    );
    OpenOptFiles(panal);
    InitOptArrays(
        panal,
        &mut piDesign_mask,
        &mut nDesignPts,
        &mut pdY,
        &mut nPreds,
        &mut nStartDecisionPts,
        &mut pdVariance,
        &mut pdIR,
        nSims,
    );
    SetupLikes(panal, nPreds, &mut pLikes);
    if ((*pgd).szGrestart).is_null() {
        printf(
            b"Error: there must be a parameter sample file - Exiting\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    } else {
        ReadAndSimulate(panal, (*pmc).nSetParms, pdY, nPreds, pLikes, nSims);
    }
    WriteOutHeader(panal, criterion);
    i = 0 as libc::c_int as libc::c_long;
    while i < nDesignPts {
        *piDesign_mask
            .offset(
                i as isize,
            ) = !((*pmc).style == forward as libc::c_int) as libc::c_int;
        i += 1;
        i;
    }
    if criterion == Var_Reduction as libc::c_int {
        if (*pmc).style == backward as libc::c_int {
            nDesignPt_tried = nDesignPts + 1 as libc::c_int as libc::c_long;
            Do_Importance_Ratios(
                pdY,
                pLikes,
                nSims,
                nPreds,
                nDesignPts,
                piDesign_mask,
                nDesignPt_tried as libc::c_int,
                pdIR,
            );
            dBest = DoVariance(nSims, pdIR, pdY, nStartDecisionPts, nPreds);
        } else {
            j = 0 as libc::c_int as libc::c_long;
            while j < nSims {
                *pdIR
                    .offset(
                        j as isize,
                    ) = 1 as libc::c_int as libc::c_double / nSims as libc::c_double;
                j += 1;
                j;
            }
            dBest = DoVariance(nSims, pdIR, pdY, nStartDecisionPts, nPreds);
        }
    }
    iBest = nDesignPts + 1 as libc::c_int as libc::c_long;
    WriteOptimOut(
        panal,
        0 as libc::c_int as libc::c_long,
        nDesignPts,
        criterion,
        pdVariance,
        piDesign_mask,
        iBest,
        dBest,
        dUtility,
    );
    dim = if (*pmc).style == backward as libc::c_int {
        nDesignPts - 1 as libc::c_int as libc::c_long
    } else {
        nDesignPts
    };
    i = 0 as libc::c_int as libc::c_long;
    while i < dim {
        if criterion == Var_Reduction as libc::c_int {
            dBest = 1.7976931348623157e+308f64;
        }
        j = 0 as libc::c_int as libc::c_long;
        while j < nDesignPts {
            if *piDesign_mask.offset(j as isize)
                == ((*pmc).style == backward as libc::c_int) as libc::c_int
            {
                nDesignPt_tried = j;
                *pdVariance.offset(j as isize) = 0 as libc::c_int as libc::c_double;
                if criterion == Var_Reduction as libc::c_int {
                    Do_Importance_Ratios(
                        pdY,
                        pLikes,
                        nSims,
                        nPreds,
                        nDesignPts,
                        piDesign_mask,
                        nDesignPt_tried as libc::c_int,
                        pdIR,
                    );
                    *pdVariance
                        .offset(
                            j as isize,
                        ) = DoVariance(nSims, pdIR, pdY, nStartDecisionPts, nPreds);
                }
                if dBest > *pdVariance.offset(j as isize) {
                    dBest = *pdVariance.offset(j as isize);
                    iBest = j;
                }
            }
            j += 1;
            j;
        }
        *piDesign_mask
            .offset(
                iBest as isize,
            ) = (*piDesign_mask.offset(iBest as isize) == 0) as libc::c_int;
        WriteOptimOut(
            panal,
            i + 1 as libc::c_int as libc::c_long,
            nDesignPts,
            criterion,
            pdVariance,
            piDesign_mask,
            iBest,
            dBest,
            dUtility,
        );
        printf(
            b"%ld points design done\n\0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int as libc::c_long,
        );
        i += 1;
        i;
    }
    if (*pmc).style == backward as libc::c_int {
        j = 0 as libc::c_int as libc::c_long;
        while j < nSims {
            *pdIR
                .offset(
                    j as isize,
                ) = 1 as libc::c_int as libc::c_double / nSims as libc::c_double;
            j += 1;
            j;
        }
        if criterion == Var_Reduction as libc::c_int {
            dBest = DoVariance(nSims, pdIR, pdY, nStartDecisionPts, nPreds);
            j = 0 as libc::c_int as libc::c_long;
            while *piDesign_mask.offset(j as isize) == 0 as libc::c_int {
                j += 1;
                j;
            }
            iBest = j;
        }
        WriteOptimOut(
            panal,
            nDesignPts,
            nDesignPts,
            criterion,
            pdVariance,
            piDesign_mask,
            iBest,
            dBest,
            0 as libc::c_int as libc::c_double,
        );
        printf(
            b"%ld points design done\n\0" as *const u8 as *const libc::c_char,
            nDesignPts,
        );
    }
    free(piDesign_mask as *mut libc::c_void);
    CloseOptFiles(panal);
}
