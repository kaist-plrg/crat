use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn ForAllList(
        plist: PLIST,
        pfiForAllData: PFI_FORLISTCALLBACK,
        pInfo: PVOID,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn InitpdVector(cVectors: libc::c_long) -> *mut *mut libc::c_double;
    fn InitlVector(cVectors: libc::c_long) -> *mut libc::c_long;
    fn InitiVector(cVectors: libc::c_long) -> *mut libc::c_int;
    fn InitdVector(cVectors: libc::c_long) -> *mut libc::c_double;
    fn GetNStates() -> libc::c_int;
    fn GetNModelVars() -> libc::c_int;
    fn GetModelVector() -> PDOUBLE;
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn FreeList(pplist: *mut PLIST, pfvFreeData: PFV_FREELISTCALLBACK, bAndData: BOOL);
    fn InitList() -> PLIST;
    fn GetStateHandles(phvar: *mut HVAR);
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
pub type PFI_FORLISTCALLBACK = Option::<
    unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int,
>;
pub type PFV_FREELISTCALLBACK = Option::<unsafe extern "C" fn(PVOID) -> ()>;
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
pub type PINTSPEC = *mut tagINTSPEC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagPRINTREC {
    pub szOutputName: PSTR,
    pub hvar: HVAR,
    pub cTimes: libc::c_long,
    pub pdTimes: PDOUBLE,
}
pub type PPRINTREC = *mut tagPRINTREC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagDATAREC {
    pub szDataName: PSTR,
    pub hvar: HVAR,
    pub cData: libc::c_long,
    pub pdData: PDOUBLE,
}
pub type PDATAREC = *mut tagDATAREC;
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
pub unsafe extern "C" fn GetModelInfo(mut pmi: PMODELINFO) {
    (*pmi).nModelVars = GetNModelVars() as libc::c_long;
    (*pmi).pdModelVars = GetModelVector();
    (*pmi).nStates = GetNStates() as libc::c_long;
    if (*pmi).nStates != 0 as libc::c_int as libc::c_long {
        (*pmi)
            .pStateHvar = malloc(
            ((*pmi).nStates as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<HVAR>() as libc::c_ulong),
        ) as *mut HVAR;
        if ((*pmi).pStateHvar).is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"GetModelInfo\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        GetStateHandles((*pmi).pStateHvar);
    } else {
        (*pmi).pStateHvar = 0 as *mut HVAR;
    };
}
pub unsafe extern "C" fn InitIntegratorSpec(mut pis: PINTSPEC) {
    (*pis).iAlgo = 3 as libc::c_int;
    (*pis).dRtol = 1.0e-5f64;
    (*pis).dAtol = 1.0e-7f64;
    (*pis).iopt = 0 as libc::c_int as libc::c_long;
    (*pis).itask = 4 as libc::c_int as libc::c_long;
    (*pis).itol = 1 as libc::c_int as libc::c_long;
    (*pis).iMf = 222 as libc::c_int as libc::c_long;
    (*pis).liw = 1000 as libc::c_int as libc::c_long;
    (*pis).lrw = 60000 as libc::c_int as libc::c_long;
    (*pis).maxsteps = 500 as libc::c_int;
    (*pis).maxnef = 7 as libc::c_int;
    (*pis).maxcor = 3 as libc::c_int;
    (*pis).maxncf = 10 as libc::c_int;
    (*pis).nlscoef = 0.1f64;
    (*pis).iwork = InitlVector((*pis).liw);
    if ((*pis).iwork).is_null()
        || {
            (*pis).rwork = InitdVector((*pis).lrw);
            ((*pis).rwork).is_null()
        }
    {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"InitIntegratorSpec()\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    (*pis).dTStep = 1 as libc::c_int as libc::c_double;
}
pub unsafe extern "C" fn InitOutputSpec(mut pos: POUTSPEC) {
    (*pos).nOutputs = 0 as libc::c_int;
    (*pos).pszOutputNames = 0 as *mut PSTR;
    (*pos).phvar_out = 0 as *mut HVAR;
    (*pos).nData = 0 as libc::c_int;
    (*pos).pszDataNames = 0 as *mut PSTR;
    (*pos).phvar_dat = 0 as *mut HVAR;
    (*pos).plistPrintRecs = 0 as PLIST;
    (*pos).plistDataRecs = 0 as PLIST;
    (*pos).pcOutputTimes = 0 as PINT;
    (*pos).piCurrentOut = 0 as PINT;
    (*pos).prgdOutputTimes = 0 as *mut PDOUBLE;
    (*pos).prgdOutputVals = 0 as *mut PDOUBLE;
    (*pos).pcData = 0 as PINT;
    (*pos).prgdDataVals = 0 as *mut PDOUBLE;
    (*pos).cDistinctTimes = 0 as libc::c_int;
    (*pos).rgdDistinctTimes = 0 as PDOUBLE;
}
pub unsafe extern "C" fn InitExperiment(
    mut pexp: PEXPERIMENT,
    mut pmodelinfo: PMODELINFO,
) {
    (*pexp).iExp = 0 as libc::c_int;
    (*pexp).dT0 = 0.0f64;
    (*pexp).dTfinal = 0.0f64;
    (*pexp).dTime = 0.0f64;
    (*pexp).pmodelinfo = pmodelinfo;
    (*pexp).plistParmMods = InitList();
    InitIntegratorSpec(&mut (*pexp).is);
    InitOutputSpec(&mut (*pexp).os);
}
pub unsafe extern "C" fn InitMonteCarlo(mut pmc: PMONTECARLO) {
    (*pmc).nRuns = 0 as libc::c_int as libc::c_long;
    (*pmc).szMCOutfilename = 0 as PSTR;
    (*pmc).pfileMCOut = 0 as PFILE;
    (*pmc).szSetPointsFilename = 0 as PSTR;
    (*pmc).pfileSetPoints = 0 as PFILE;
    (*pmc).plistMCVars = 0 as PLIST;
    (*pmc).rgdParms = 0 as *mut libc::c_double;
    (*pmc).rghvar = 0 as *mut HVAR;
    (*pmc).rgpMCVar = 0 as *mut *mut MCVAR;
}
pub unsafe extern "C" fn InitGibbs(mut pgd: PGIBBSDATA) {
    (*pgd).nMaxIter = 0 as libc::c_int as libc::c_long;
    (*pgd).nSimTypeFlag = 0 as libc::c_int as libc::c_long;
    (*pgd).nPrintIter = 0 as libc::c_int as libc::c_long;
    (*pgd).nPrintFreq = 1 as libc::c_int as libc::c_long;
    (*pgd).nMaxPerkSetIter = 300000 as libc::c_int as libc::c_long;
    (*pgd).szGout = 0 as PSTR;
    (*pgd).pfileOut = 0 as PFILE;
    (*pgd).szGrestart = 0 as PSTR;
    (*pgd).pfileRestart = 0 as PFILE;
    (*pgd).szGdata = 0 as PSTR;
    (*pgd).nPerks = 0 as libc::c_int;
    (*pgd).indexT = 0 as libc::c_int;
    (*pgd).dCZero = 100 as libc::c_int as libc::c_double;
    (*pgd).dNZero = 100 as libc::c_int as libc::c_double;
    (*pgd).startT = 0 as libc::c_int;
    (*pgd).endT = 0 as libc::c_int;
    (*pgd).rglTransAttempts = 0 as PLONG;
    (*pgd).rglTransAccepts = 0 as PLONG;
}
pub unsafe extern "C" fn InitAnalysis(mut panal: PANALYSIS) {
    let mut i: libc::c_int = 0;
    if panal.is_null() {
        return;
    }
    (*panal).bDependents = 0 as libc::c_int;
    (*panal).bOutputIter = 0 as libc::c_int;
    (*panal).nOutputFreq = 0 as libc::c_int;
    (*panal).bPrintConvergence = 0 as libc::c_int;
    (*panal).iType = 1 as libc::c_int;
    (*panal).dSeed = 314159265.3589793f64;
    (*panal).wContext = 0x1 as libc::c_int as WORD;
    (*panal).pexpCurrent = &mut (*panal).expGlobal;
    GetModelInfo(&mut (*panal).modelinfo);
    InitExperiment(&mut (*panal).expGlobal, &mut (*panal).modelinfo);
    (*panal).szOutfilename = 0 as PSTR;
    (*panal).pfileOut = 0 as PFILE;
    (*panal).bCommandLineSpec = 0 as libc::c_int;
    (*panal).bAllocatedFileName = 0 as libc::c_int;
    (*panal).iExpts = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 20000 as libc::c_int {
        (*panal).rgpExps[i as usize] = 0 as PEXPERIMENT;
        i += 1;
        i;
    }
    InitMonteCarlo(&mut (*panal).mc);
    InitGibbs(&mut (*panal).gd);
}
pub unsafe extern "C" fn InitOutputs(
    mut pexp: PEXPERIMENT,
    mut piOut: PINT,
    mut pdTout: PDOUBLE,
) -> BOOL {
    let mut j: libc::c_int = 0;
    let mut bReturn: BOOL = 0 as libc::c_int;
    if (*pexp).os.nOutputs == 0 {
        ReportError(
            0 as PINPUTBUF,
            0x202 as libc::c_int as WORD,
            &mut (*pexp).iExp as *mut libc::c_int as PSTR,
            0 as PSTR,
        );
    } else {
        *piOut = 0 as libc::c_int;
        *pdTout = *((*pexp).os.rgdDistinctTimes).offset(0 as libc::c_int as isize);
        j = 0 as libc::c_int;
        while j < (*pexp).os.nOutputs {
            *((*pexp).os.piCurrentOut).offset(j as isize) = 0 as libc::c_int;
            j += 1;
            j;
        }
        bReturn = 1 as libc::c_int;
    }
    return bReturn;
}
pub unsafe extern "C" fn InitOneOutVar(
    mut pData: PVOID,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut ppr: PPRINTREC = pData as PPRINTREC;
    let mut pos: POUTSPEC = pInfo as POUTSPEC;
    let fresh0 = (*pos).nOutputs;
    (*pos).nOutputs = (*pos).nOutputs + 1;
    let mut i: libc::c_int = fresh0;
    let ref mut fresh1 = *((*pos).pszOutputNames).offset(i as isize);
    *fresh1 = (*ppr).szOutputName;
    *((*pos).phvar_out).offset(i as isize) = (*ppr).hvar;
    *((*pos).pcOutputTimes).offset(i as isize) = (*ppr).cTimes as libc::c_int;
    *((*pos).piCurrentOut).offset(i as isize) = 0 as libc::c_int;
    let ref mut fresh2 = *((*pos).prgdOutputTimes).offset(i as isize);
    *fresh2 = (*ppr).pdTimes;
    let ref mut fresh3 = *((*pos).prgdOutputVals).offset(i as isize);
    *fresh3 = InitdVector((*ppr).cTimes);
    if (*((*pos).prgdOutputTimes).offset(i as isize)).is_null()
        || (*((*pos).prgdOutputVals).offset(i as isize)).is_null()
    {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"InitOneOutVar()\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn InitOneDataVar(
    mut pData: PVOID,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut pda: PDATAREC = pData as PDATAREC;
    let mut pos: POUTSPEC = pInfo as POUTSPEC;
    let fresh4 = (*pos).nData;
    (*pos).nData = (*pos).nData + 1;
    let mut i: libc::c_int = fresh4;
    let ref mut fresh5 = *((*pos).prgdDataVals).offset(i as isize);
    *fresh5 = (*pda).pdData;
    *((*pos).pcData).offset(i as isize) = (*pda).cData as libc::c_int;
    *((*pos).phvar_dat).offset(i as isize) = (*pda).hvar;
    let ref mut fresh6 = *((*pos).pszDataNames).offset(i as isize);
    *fresh6 = (*pda).szDataName;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FindNewPoint(mut pos: POUTSPEC, mut piPoint: PINT) -> BOOL {
    *piPoint = 0 as libc::c_int;
    while *piPoint < (*pos).nOutputs {
        if *((*pos).piCurrentOut).offset(*piPoint as isize)
            < *((*pos).pcOutputTimes).offset(*piPoint as isize)
        {
            break;
        }
        *piPoint += 1;
        *piPoint;
    }
    return if *piPoint < (*pos).nOutputs { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub unsafe extern "C" fn CreateOutputSchedule(mut pos: POUTSPEC) {
    let mut i: libc::c_int = 0;
    let mut cTimes: libc::c_int = 0 as libc::c_int;
    let mut iPoint: libc::c_int = 0;
    let mut bCont: BOOL = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*pos).nOutputs {
        cTimes += *((*pos).pcOutputTimes).offset(i as isize);
        i += 1;
        i;
    }
    (*pos).rgdDistinctTimes = InitdVector(cTimes as libc::c_long);
    if ((*pos).rgdDistinctTimes).is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"CreateOutputSchedule()\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    cTimes = 0 as libc::c_int;
    FindNewPoint(pos, &mut iPoint);
    while bCont != 0 {
        i = 0 as libc::c_int;
        while i < (*pos).nOutputs {
            if i != iPoint
                && *((*pos).piCurrentOut).offset(i as isize)
                    < *((*pos).pcOutputTimes).offset(i as isize)
            {
                if *(*((*pos).prgdOutputTimes).offset(i as isize))
                    .offset(*((*pos).piCurrentOut).offset(i as isize) as isize)
                    < *(*((*pos).prgdOutputTimes).offset(iPoint as isize))
                        .offset(*((*pos).piCurrentOut).offset(iPoint as isize) as isize)
                {
                    iPoint = i;
                } else if *(*((*pos).prgdOutputTimes).offset(i as isize))
                    .offset(*((*pos).piCurrentOut).offset(i as isize) as isize)
                    == *(*((*pos).prgdOutputTimes).offset(iPoint as isize))
                        .offset(*((*pos).piCurrentOut).offset(iPoint as isize) as isize)
                {
                    let ref mut fresh7 = *((*pos).piCurrentOut).offset(i as isize);
                    *fresh7 += 1;
                    *fresh7;
                }
            }
            i += 1;
            i;
        }
        let fresh8 = cTimes;
        cTimes = cTimes + 1;
        *((*pos).rgdDistinctTimes)
            .offset(
                fresh8 as isize,
            ) = *(*((*pos).prgdOutputTimes).offset(iPoint as isize))
            .offset(*((*pos).piCurrentOut).offset(iPoint as isize) as isize);
        let ref mut fresh9 = *((*pos).piCurrentOut).offset(iPoint as isize);
        *fresh9 += 1;
        if *fresh9 >= *((*pos).pcOutputTimes).offset(iPoint as isize) {
            bCont = FindNewPoint(pos, &mut iPoint);
        }
    }
    (*pos).cDistinctTimes = cTimes;
}
pub unsafe extern "C" fn PrepareOutSpec(mut pexp: PEXPERIMENT) -> BOOL {
    let mut pos: POUTSPEC = &mut (*pexp).os;
    let mut bReturn: BOOL = 0 as libc::c_int;
    let mut cDat: libc::c_int = if !((*pos).plistDataRecs).is_null() {
        (*(*pos).plistDataRecs).iSize
    } else {
        0 as libc::c_int
    };
    let mut cOut: libc::c_int = if !((*pos).plistPrintRecs).is_null() {
        (*(*pos).plistPrintRecs).iSize
    } else {
        0 as libc::c_int
    };
    if cOut == 0 {
        ReportError(
            0 as PINPUTBUF,
            0x202 as libc::c_int as WORD,
            &mut (*pexp).iExp as *mut libc::c_int as PSTR,
            0 as PSTR,
        );
    } else {
        (*pos)
            .pszOutputNames = malloc(
            (cOut as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<PSTR>() as libc::c_ulong),
        ) as *mut PSTR;
        (*pos)
            .phvar_out = malloc(
            (cOut as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<HVAR>() as libc::c_ulong),
        ) as *mut HVAR;
        (*pos).pcOutputTimes = InitiVector(cOut as libc::c_long);
        (*pos).piCurrentOut = InitiVector(cOut as libc::c_long);
        (*pos).prgdOutputTimes = InitpdVector(cOut as libc::c_long);
        (*pos).prgdOutputVals = InitpdVector(cOut as libc::c_long);
        if ((*pos).pszOutputNames).is_null() || ((*pos).phvar_out).is_null()
            || ((*pos).pcOutputTimes).is_null() || ((*pos).piCurrentOut).is_null()
            || ((*pos).prgdOutputTimes).is_null() || ((*pos).prgdOutputVals).is_null()
        {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"PrepareOutSpec()\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        } else {
            (*pos).nOutputs = 0 as libc::c_int;
            ForAllList(
                (*pos).plistPrintRecs,
                Some(InitOneOutVar as unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int),
                &mut (*pexp).os as *mut OUTSPEC as PVOID,
            );
            (*pos).nOutputs = cOut;
            CreateOutputSchedule(pos);
            (*pexp)
                .dTfinal = *((*pos).rgdDistinctTimes)
                .offset(((*pos).cDistinctTimes - 1 as libc::c_int) as isize);
            if (*pexp).dTfinal == (*pexp).dT0 {
                printf(
                    b"\nError: starting and final times are equal in Simulation %d - Exiting.\n\n\0"
                        as *const u8 as *const libc::c_char,
                    (*pexp).iExp,
                );
                exit(0 as libc::c_int);
            }
            bReturn = 1 as libc::c_int;
        }
    }
    if !(cDat == 0) {
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
                b"PrepareOutSpec()\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        } else {
            (*pos).nData = 0 as libc::c_int;
            ForAllList(
                (*pos).plistDataRecs,
                Some(
                    InitOneDataVar as unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int,
                ),
                &mut (*pexp).os as *mut OUTSPEC as PVOID,
            );
            (*pos).nData = cDat;
            FreeList(&mut (*pos).plistDataRecs, None, 0 as libc::c_int);
        }
    }
    return bReturn;
}
pub unsafe extern "C" fn PrintOutSpec(mut pexp: PEXPERIMENT) -> BOOL {
    let mut pos: POUTSPEC = &mut (*pexp).os;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cOut: libc::c_int = (*pos).nOutputs;
    printf(b"%d Outputs:\n\0" as *const u8 as *const libc::c_char, cOut);
    i = 0 as libc::c_int;
    while i < cOut {
        printf(
            b"  %#0lx  %s: \0" as *const u8 as *const libc::c_char,
            *((*pos).phvar_out).offset(i as isize),
            *((*pos).pszOutputNames).offset(i as isize),
        );
        j = 0 as libc::c_int;
        while j < *((*pos).pcOutputTimes).offset(i as isize) {
            printf(
                b"%g \0" as *const u8 as *const libc::c_char,
                *(*((*pos).prgdOutputTimes).offset(i as isize)).offset(j as isize),
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
