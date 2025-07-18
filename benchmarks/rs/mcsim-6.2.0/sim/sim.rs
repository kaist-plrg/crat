use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn InitRandom(rank: libc::c_int, dSeed: libc::c_double, bWarmUp: libc::c_int);
    fn InitBuffer(pibIn: PINPUTBUF, szFullPathname: PSTR) -> BOOL;
    static mut szModelDescFilename: [libc::c_char; 0];
    static mut szModelSourceFilename: [libc::c_char; 0];
    static mut szModelGenAndVersion: [libc::c_char; 0];
    fn InitModel();
    fn CalcDeriv(
        rgModelVars: *mut libc::c_double,
        rgDerivs: *mut libc::c_double,
        pdTime: PDOUBLE,
    );
    fn GetVarValue(hVar: HVAR) -> libc::c_double;
    fn IsInput(hVar: HVAR) -> BOOL;
    fn UpdateInputs(pdTime: PDOUBLE, pdNextTransTime: PDOUBLE);
    fn SetInput(hVar: HVAR, pInputFnRecord: PIFN) -> BOOL;
    fn SetVar(hVar: HVAR, dVal: libc::c_double) -> BOOL;
    fn ForAllList(
        plist: PLIST,
        pfiForAllData: PFI_FORLISTCALLBACK,
        pInfo: PVOID,
    ) -> libc::c_int;
    fn FreeList(pplist: *mut PLIST, pfvFreeData: PFV_FREELISTCALLBACK, bAndData: BOOL);
    fn InitdVector(cVectors: libc::c_long) -> *mut libc::c_double;
    fn StoreDelayed(t: libc::c_double);
    static mut bDelays: BOOL;
    fn InitDelays(dTime: libc::c_double);
    fn DoStep_by_Step();
    fn TransformPred(_: PANALYSIS, _: PMCPREDOUT);
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn _getopt(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn DoMarkov(panal: PANALYSIS);
    fn DoOptimalDesign(panal: PANALYSIS);
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn lsodes_(
        _: *mut libc::c_long,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: *mut libc::c_long,
        _: *mut libc::c_double,
        _: *mut libc::c_double,
        _: *mut libc::c_long,
        _: *mut libc::c_long,
        _: *mut libc::c_long,
        _: *mut libc::c_double,
        _: *mut libc::c_long,
        _: *mut libc::c_long,
        _: *mut libc::c_long,
        _: *mut libc::c_long,
    ) -> libc::c_int;
    fn FreeLevels(panal: PANALYSIS);
    fn FreeDataRec(pData: PVOID, pUserInfo: PVOID) -> libc::c_int;
    fn FreePrintRec(pData: PVOID, pUserInfo: PVOID) -> libc::c_int;
    fn ReadAnalysis(_: PINPUTBUF) -> BOOL;
    fn MyStrcmp(sz1: *const libc::c_char, sz2: *const libc::c_char) -> libc::c_int;
    fn InitAnalysis(panal: PANALYSIS);
    fn InitOutputs(pexp: PEXPERIMENT, piOut: PINT, pdTout: PDOUBLE) -> BOOL;
    fn CloseMCFiles(_: PANALYSIS);
    fn SaveOutputs(_: PEXPERIMENT, _: PDOUBLE);
    fn NextOutputTime(_: PEXPERIMENT, _: PDOUBLE, _: PINT);
    fn OpenMCFiles(panal: PANALYSIS);
    fn WriteMCOutput(_: PANALYSIS, _: PMCPREDOUT);
    fn WriteNormalOutput(_: PANALYSIS, _: PEXPERIMENT);
    fn CalcMCParms(pMC: PMONTECARLO, rgParms: *mut libc::c_double, iStart: libc::c_long);
    fn GetSPMods(panal: PANALYSIS, rgdOptionalParms: *mut libc::c_double) -> BOOL;
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
pub type INPUTBUF = tagINPUTBUF;
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
pub type ANALYSIS = tagANALYSIS;
pub type PANALYSIS = *mut tagANALYSIS;
pub type MCPREDOUT = tagMCPREDOUT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagMCPREDOUT {
    pub nbrdy: libc::c_long,
    pub pred: *mut libc::c_double,
    pub passflag: libc::c_int,
}
pub type PMCPREDOUT = *mut tagMCPREDOUT;
pub unsafe extern "C" fn CorrectInputToTransition(
    mut pexp: PEXPERIMENT,
    mut pdTtrans: PDOUBLE,
) {
    (*pexp).dTime = *pdTtrans;
    UpdateInputs(&mut (*pexp).dTime, pdTtrans);
}
pub unsafe extern "C" fn Euler(
    mut neq: libc::c_long,
    mut y: *mut libc::c_double,
    mut t: *mut libc::c_double,
    mut tout: libc::c_double,
    mut dTStep: libc::c_double,
) -> libc::c_int {
    static mut rgdDeriv: PDOUBLE = 0 as *const libc::c_double as *mut libc::c_double;
    let mut dTmp_step: libc::c_double = 0.;
    let mut i: libc::c_long = 0;
    if rgdDeriv.is_null() {
        rgdDeriv = InitdVector(neq);
        if rgdDeriv.is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"Euler\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    }
    while *t < tout {
        CalcDeriv(y, rgdDeriv, t);
        *t = *t + dTStep;
        if *t > tout {
            dTmp_step = tout - (*t - dTStep);
            *t = tout;
        } else {
            dTmp_step = dTStep;
        }
        i = 0 as libc::c_int as libc::c_long;
        while i < neq {
            *y
                .offset(
                    i as isize,
                ) = *y.offset(i as isize) + dTmp_step * *rgdDeriv.offset(i as isize);
            i += 1;
            i;
        }
        if bDelays != 0 {
            StoreDelayed(*t);
        }
        DoStep_by_Step();
    }
    CalcDeriv(y, rgdDeriv, t);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FreeVarMod(mut pData: PVOID) {
    let mut pvarmod: PVARMOD = pData as PVARMOD;
    if IsInput((*pvarmod).hvar) != 0 {
        if !((*pvarmod).uvar.pifn).is_null() {
            free((*pvarmod).uvar.pifn as *mut libc::c_void);
        }
    }
    free(pvarmod as *mut libc::c_void);
}
pub unsafe extern "C" fn ModifyOneParm(
    mut pData: PVOID,
    mut pNullInfo: PVOID,
) -> libc::c_int {
    let mut pvarmod: PVARMOD = pData as PVARMOD;
    if IsInput((*pvarmod).hvar) != 0 {
        SetInput((*pvarmod).hvar, (*pvarmod).uvar.pifn);
    } else {
        SetVar((*pvarmod).hvar, (*pvarmod).uvar.dVal);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ModifyParms(mut plistParmMods: PLIST) {
    if !plistParmMods.is_null() {} else {
        __assert_fail(
            b"plistParmMods\0" as *const u8 as *const libc::c_char,
            b"sim.c\0" as *const u8 as *const libc::c_char,
            279 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"void ModifyParms(PLIST)\0"))
                .as_ptr(),
        );
    }
    'c_5628: {
        if !plistParmMods.is_null() {} else {
            __assert_fail(
                b"plistParmMods\0" as *const u8 as *const libc::c_char,
                b"sim.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"void ModifyParms(PLIST)\0"))
                    .as_ptr(),
            );
        }
    };
    ForAllList(
        plistParmMods,
        Some(ModifyOneParm as unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int),
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn DoOneExperiment(mut pexp: PEXPERIMENT) -> libc::c_int {
    let mut dTout: libc::c_double = 0.;
    let mut dTtrans: libc::c_double = 0.;
    let mut dTup: libc::c_double = 0.;
    let mut iOut: libc::c_int = 0;
    let mut pmod: PMODELINFO = 0 as *mut tagMODELINFO;
    let mut pis: PINTSPEC = 0 as *mut tagINTSPEC;
    if pexp.is_null() {
        return 0 as libc::c_int;
    }
    pmod = (*pexp).pmodelinfo;
    pis = &mut (*pexp).is;
    if InitOutputs(pexp, &mut iOut, &mut dTout) == 0 {
        return 0 as libc::c_int;
    }
    if (*pexp).hT0 != 0 {
        (*pexp).dT0 = GetVarValue((*pexp).hT0);
    }
    UpdateInputs(&mut (*pexp).dT0, &mut dTtrans);
    if bDelays != 0 {
        InitDelays((*pexp).hT0 as libc::c_double);
    }
    if (*pexp).dT0 > dTtrans {
        printf(
            b"\nError: starting time is greater than first discontinuity,       check your inputs - Exiting.\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    if (*pexp).dT0 > dTout {
        printf(
            b"\nError: starting time is greater than first output time,       check your outputs - Exiting.\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    (*pexp).dTime = (*pexp).dT0;
    if (*pis).iAlgo == 3 as libc::c_int {
        (*pis).iDSFlag = 1 as libc::c_int as libc::c_long;
    } else {
        (*pis).iAlgo == 4 as libc::c_int;
    }
    while (*pexp).dTime < (*pexp).dTfinal {
        if (*pmod).nStates > 0 as libc::c_int as libc::c_long {
            if fabs(dTout - dTtrans)
                < 2.2204460492503131e-16f64 * 2.0f64
                    * (if fabs(dTout) >= fabs(dTtrans) {
                        fabs(dTout)
                    } else {
                        fabs(dTtrans)
                    })
            {
                dTtrans = dTout;
            }
            dTup = if dTout < dTtrans { dTout } else { dTtrans };
            if fabs(dTup - (*pexp).dTime)
                < 2.2204460492503131e-16f64 * 2.0f64
                    * (if fabs(dTup) >= fabs((*pexp).dTime) {
                        fabs(dTup)
                    } else {
                        fabs((*pexp).dTime)
                    })
            {
                (*pexp).dTime = dTup;
            }
            if (*pis).iAlgo == 3 as libc::c_int {
                *((*pis).rwork).offset(0 as libc::c_int as isize) = dTup;
                lsodes_(
                    &mut (*pmod).nStates,
                    (*pmod).pdModelVars,
                    &mut (*pexp).dTime,
                    &mut dTup,
                    &mut (*pis).itol,
                    &mut (*pis).dRtol,
                    &mut (*pis).dAtol,
                    &mut (*pis).itask,
                    &mut (*pis).iDSFlag,
                    &mut (*pis).iopt,
                    (*pis).rwork,
                    &mut (*pis).lrw,
                    (*pis).iwork,
                    &mut (*pis).liw,
                    &mut (*pis).iMf,
                );
                if (*pis).iDSFlag < 0 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int;
                }
            } else if !((*pis).iAlgo == 4 as libc::c_int) {
                if (*pis).iAlgo == 2 as libc::c_int {
                    Euler(
                        (*pmod).nStates,
                        (*pmod).pdModelVars,
                        &mut (*pexp).dTime,
                        dTup,
                        (*pis).dTStep,
                    );
                }
            }
        } else {
            (*pexp).dTime = if dTout < dTtrans { dTout } else { dTtrans };
        }
        if dTtrans <= dTout {
            if dTtrans < dTout {
                if dTtrans < (*pexp).dTfinal {
                    CorrectInputToTransition(pexp, &mut dTtrans);
                    (*pis).iDSFlag = 1 as libc::c_int as libc::c_long;
                }
            } else {
                if dTtrans < (*pexp).dTfinal {
                    CorrectInputToTransition(pexp, &mut dTtrans);
                    (*pis).iDSFlag = 1 as libc::c_int as libc::c_long;
                }
                SaveOutputs(pexp, &mut dTout);
                NextOutputTime(pexp, &mut dTout, &mut iOut);
            }
        } else {
            SaveOutputs(pexp, &mut dTout);
            NextOutputTime(pexp, &mut dTout, &mut iOut);
        }
    }
    (*pis).iAlgo == 4 as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn DoOneNormalExp(
    mut panal: PANALYSIS,
    mut pexp: PEXPERIMENT,
) -> libc::c_int {
    printf(b"experiment %d\n\0" as *const u8 as *const libc::c_char, (*pexp).iExp);
    InitModel();
    ModifyParms((*panal).expGlobal.plistParmMods);
    ModifyParms((*pexp).plistParmMods);
    if DoOneExperiment(pexp) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn DoOneMCExp(
    mut panal: PANALYSIS,
    mut pexp: PEXPERIMENT,
) -> libc::c_int {
    let mut pmc: *mut MONTECARLO = &mut (*panal).mc;
    InitModel();
    ModifyParms((*panal).expGlobal.plistParmMods);
    SetParms((*pmc).nParms, (*pmc).rghvar, (*pmc).rgdParms);
    ModifyParms((*pexp).plistParmMods);
    if DoOneExperiment(pexp) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn DoNormal(mut panal: PANALYSIS) {
    let mut nExps: libc::c_int = (*panal).expGlobal.iExp;
    let mut i: libc::c_int = 0;
    printf(
        b"\nDoing analysis - %d normal experiment%c\n\0" as *const u8
            as *const libc::c_char,
        nExps,
        if nExps > 1 as libc::c_int { 's' as i32 } else { ' ' as i32 },
    );
    i = 0 as libc::c_int;
    while i < nExps {
        if DoOneNormalExp(panal, (*panal).rgpExps[i as usize]) != 0 {
            WriteNormalOutput(panal, (*panal).rgpExps[i as usize]);
        } else {
            printf(
                b"[MCSIM ERROR] Integration failed - No output generated\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn DoMonteCarlo(mut panal: PANALYSIS) {
    let mut nExps: libc::c_int = (*panal).expGlobal.iExp;
    let mut nRuns: libc::c_long = (*panal).mc.nRuns;
    let mut mcpredout: MCPREDOUT = MCPREDOUT {
        nbrdy: 0,
        pred: 0 as *mut libc::c_double,
        passflag: 0,
    };
    let mut bOK: BOOL = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    if (*panal).rank == 0 as libc::c_int {
        printf(
            b"Doing %ld Monte Carlo simulation%c, %d experiment%c%s\n\0" as *const u8
                as *const libc::c_char,
            nRuns,
            if nRuns != 1 as libc::c_int as libc::c_long {
                's' as i32
            } else {
                ' ' as i32
            },
            nExps,
            if nExps > 1 as libc::c_int { 's' as i32 } else { ' ' as i32 },
            if nRuns != 1 as libc::c_int as libc::c_long {
                b" each\0" as *const u8 as *const libc::c_char
            } else {
                b".\0" as *const u8 as *const libc::c_char
            },
        );
        if (*panal).size > 1 as libc::c_int {
            printf(
                b"Split between %d processors\n\0" as *const u8 as *const libc::c_char,
                (*panal).size,
            );
        }
    } else {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    SetParents(&mut (*panal).mc, 0 as libc::c_int as libc::c_long);
    OpenMCFiles(panal);
    mcpredout.pred = 0 as *mut libc::c_double;
    i = (*panal).rank as libc::c_long;
    while i < nRuns {
        if i == 0 as libc::c_int as libc::c_long {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        if (*panal).bOutputIter != 0
            && (i + 1 as libc::c_int as libc::c_long)
                % (*panal).nOutputFreq as libc::c_long
                == 0 as libc::c_int as libc::c_long
        {
            if (*panal).size > 1 as libc::c_int {
                printf(
                    b"Processor %d, Iteration %ld\n\0" as *const u8
                        as *const libc::c_char,
                    (*panal).rank,
                    i + 1 as libc::c_int as libc::c_long,
                );
            } else {
                printf(
                    b"Iteration %ld\n\0" as *const u8 as *const libc::c_char,
                    i + 1 as libc::c_int as libc::c_long,
                );
            }
        }
        (*panal).mc.lRun = i;
        CalcMCParms(
            &mut (*panal).mc,
            0 as *mut libc::c_double,
            0 as libc::c_int as libc::c_long,
        );
        j = 0 as libc::c_int as libc::c_long;
        while j < nExps as libc::c_long {
            bOK = DoOneMCExp(panal, (*panal).rgpExps[j as usize]);
            if bOK == 0 {
                break;
            }
            j += 1;
            j;
        }
        if bOK != 0 {
            TransformPred(panal, &mut mcpredout);
            WriteMCOutput(panal, &mut mcpredout);
        } else {
            printf(
                b"Warning: Integration failed on iteration %ld, experiment %ld:\n         No output generated\n\0"
                    as *const u8 as *const libc::c_char,
                (*panal).mc.lRun + 1 as libc::c_int as libc::c_long,
                j + 1 as libc::c_int as libc::c_long,
            );
        }
        i += (*panal).size as libc::c_long;
    }
    CloseMCFiles(panal);
    if !(mcpredout.pred).is_null() {
        free(mcpredout.pred as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn DoSetPoints(mut panal: PANALYSIS) {
    let mut nExps: libc::c_int = (*panal).expGlobal.iExp;
    let mut nRuns: libc::c_long = (*panal).mc.nRuns;
    let mut mcpredout: MCPREDOUT = MCPREDOUT {
        nbrdy: 0,
        pred: 0 as *mut libc::c_double,
        passflag: 0,
    };
    let mut bOK: BOOL = 0 as libc::c_int;
    let mut bNotDone: BOOL = 0;
    let mut i: libc::c_int = 0;
    mcpredout.pred = 0 as *mut libc::c_double;
    OpenMCFiles(panal);
    if (*panal).rank == 0 as libc::c_int {
        printf(
            b"Doing analysis - %ld SetPoints run%c... %d experiment%c%s\n\0" as *const u8
                as *const libc::c_char,
            nRuns,
            if nRuns != 1 as libc::c_int as libc::c_long {
                's' as i32
            } else {
                ' ' as i32
            },
            nExps,
            if nExps > 1 as libc::c_int { 's' as i32 } else { ' ' as i32 },
            if nRuns != 1 as libc::c_int as libc::c_long {
                b" each\0" as *const u8 as *const libc::c_char
            } else {
                b" \0" as *const u8 as *const libc::c_char
            },
        );
        if (*panal).size > 1 as libc::c_int {
            printf(
                b"Split between %d processors\n\0" as *const u8 as *const libc::c_char,
                (*panal).size,
            );
        }
    } else {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if nRuns == 0 && (*panal).rank == 0 as libc::c_int {
        printf(
            b"0 runs specified for SetPoint(). Reading entire file.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    SetParents(&mut (*panal).mc, (*panal).mc.nSetParms);
    (*panal).mc.lRun = 0 as libc::c_int as libc::c_long;
    bNotDone = 1 as libc::c_int;
    while bNotDone != 0 {
        bNotDone = GetSPMods(panal, 0 as *mut libc::c_double);
        if bNotDone != 0
            && (*panal).mc.lRun % (*panal).size as libc::c_long
                == (*panal).rank as libc::c_long
        {
            if (*panal).bOutputIter != 0
                && ((*panal).mc.lRun + 1 as libc::c_int as libc::c_long)
                    % (*panal).nOutputFreq as libc::c_long
                    == 0 as libc::c_int as libc::c_long
            {
                if (*panal).size > 1 as libc::c_int {
                    printf(
                        b"Processor %d, Iteration %ld\n\0" as *const u8
                            as *const libc::c_char,
                        (*panal).rank,
                        (*panal).mc.lRun + 1 as libc::c_int as libc::c_long,
                    );
                } else {
                    printf(
                        b"Iteration %ld\n\0" as *const u8 as *const libc::c_char,
                        (*panal).mc.lRun + 1 as libc::c_int as libc::c_long,
                    );
                }
            }
            i = 0 as libc::c_int;
            while i < nExps {
                bOK = DoOneMCExp(panal, (*panal).rgpExps[i as usize]);
                if bOK == 0 {
                    break;
                }
                i += 1;
                i;
            }
            if bOK != 0 {
                TransformPred(panal, &mut mcpredout);
                WriteMCOutput(panal, &mut mcpredout);
            } else {
                printf(
                    b"Warning: Integration failed on iteration %ld, experiment %d:\n         No output generated\n\0"
                        as *const u8 as *const libc::c_char,
                    (*panal).mc.lRun + 1 as libc::c_int as libc::c_long,
                    i + 1 as libc::c_int,
                );
            }
        }
        (*panal).mc.lRun += 1;
        (*panal).mc.lRun;
        if nRuns != 0 {
            bNotDone = ((*panal).mc.lRun < nRuns) as libc::c_int;
        }
    }
    CloseMCFiles(panal);
    if !(mcpredout.pred).is_null() {
        free(mcpredout.pred as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn DoAnalysis(mut panal: PANALYSIS) {
    if (*panal).size == 1 as libc::c_int {
        InitRandom((*panal).rank, (*panal).dSeed, 1 as libc::c_int);
    } else {
        InitRandom(
            (*panal).rank,
            (*panal).dSeed + (*panal).rank as libc::c_double,
            1 as libc::c_int,
        );
    }
    match (*panal).iType {
        3 => {
            DoSetPoints(panal);
        }
        2 => {
            DoMonteCarlo(panal);
        }
        4 => {
            DoMarkov(panal);
        }
        5 => {
            if (*panal).rank == 0 as libc::c_int {
                DoOptimalDesign(panal);
            }
        }
        1 | _ => {
            if (*panal).rank == 0 as libc::c_int {
                DoNormal(panal);
            }
        }
    }
    if !((*panal).pfileOut).is_null() {
        fclose((*panal).pfileOut);
        printf(
            b"Wrote output file \"%s\"\n\0" as *const u8 as *const libc::c_char,
            (*panal).szOutfilename,
        );
    }
}
pub unsafe extern "C" fn FreeMemory(mut panal: PANALYSIS) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    free((*panal).modelinfo.pStateHvar as *mut libc::c_void);
    FreeList(&mut (*panal).mc.plistMCVars, None, 1 as libc::c_int);
    if !((*panal).mc.rgdParms).is_null() {
        free((*panal).mc.rgdParms as *mut libc::c_void);
        free((*panal).mc.rghvar as *mut libc::c_void);
    }
    let mut pis: PINTSPEC = &mut (**((*panal).rgpExps)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .is;
    free((*pis).iwork as *mut libc::c_void);
    free((*pis).rwork as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < (*panal).expGlobal.iExp {
        if !((*panal).rgpExps[i as usize]).is_null() {
            FreeList(
                &mut (**((*panal).rgpExps).as_mut_ptr().offset(i as isize))
                    .plistParmMods,
                Some(FreeVarMod as unsafe extern "C" fn(PVOID) -> ()),
                1 as libc::c_int,
            );
            let mut pos: POUTSPEC = &mut (**((*panal).rgpExps)
                .as_mut_ptr()
                .offset(i as isize))
                .os;
            free((*pos).pszOutputNames as *mut libc::c_void);
            free((*pos).phvar_out as *mut libc::c_void);
            free((*pos).pcOutputTimes as *mut libc::c_void);
            free((*pos).piCurrentOut as *mut libc::c_void);
            free((*pos).prgdOutputTimes as *mut libc::c_void);
            j = 0 as libc::c_int;
            while j < (*pos).nOutputs {
                free(*((*pos).prgdOutputVals).offset(j as isize) as *mut libc::c_void);
                j += 1;
                j;
            }
            free((*pos).prgdOutputVals as *mut libc::c_void);
            free((*pos).rgdDistinctTimes as *mut libc::c_void);
            ForAllList(
                (*pos).plistPrintRecs,
                Some(FreePrintRec as unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int),
                0 as *mut libc::c_void,
            );
            FreeList(&mut (*pos).plistPrintRecs, None, 0 as libc::c_int);
            free((*pos).plistPrintRecs as *mut libc::c_void);
            ForAllList(
                (*pos).plistDataRecs,
                Some(FreeDataRec as unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int),
                0 as *mut libc::c_void,
            );
            FreeList(&mut (*pos).plistDataRecs, None, 0 as libc::c_int);
            free((*pos).plistDataRecs as *mut libc::c_void);
            free((*panal).rgpExps[i as usize] as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    if (*panal).bAllocatedFileName != 0 {
        if !((*panal).szOutfilename).is_null() {
            free((*panal).szOutfilename as *mut libc::c_void);
        }
        if !((*panal).mc.szMCOutfilename).is_null() {
            free((*panal).mc.szMCOutfilename as *mut libc::c_void);
        }
        if !((*panal).gd.szGout).is_null() {
            free((*panal).gd.szGout as *mut libc::c_void);
        }
    }
    if !((*panal).mc.szSetPointsFilename).is_null() {
        free((*panal).mc.szSetPointsFilename as *mut libc::c_void);
    }
    if !((*panal).gd.szGrestart).is_null() {
        free((*panal).gd.szGrestart as *mut libc::c_void);
    }
    if !((*panal).gd.szGdata).is_null() {
        free((*panal).gd.szGdata as *mut libc::c_void);
    }
    FreeList(&mut (*panal).expGlobal.plistParmMods, None, 1 as libc::c_int);
    free(panal as *mut libc::c_void);
}
pub static mut vrgpMCVar: *mut *mut MCVAR = 0 as *const *mut MCVAR as *mut *mut MCVAR;
pub static mut viMCVar: libc::c_int = 0;
pub unsafe extern "C" fn MCVarListToArray(
    mut pv_pMCVar: PVOID,
    mut pv_Null: PVOID,
) -> libc::c_int {
    let ref mut fresh0 = *vrgpMCVar.offset(viMCVar as isize);
    *fresh0 = pv_pMCVar as *mut MCVAR;
    viMCVar += 1;
    viMCVar;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn PrepAnalysis(mut panal: PANALYSIS) {
    let mut pmc: *mut MONTECARLO = &mut (*panal).mc;
    let mut l: libc::c_int = 0;
    (*pmc)
        .nParms = (if !((*pmc).plistMCVars).is_null() {
        (*(*pmc).plistMCVars).iSize
    } else {
        0 as libc::c_int
    }) as libc::c_long;
    if (*pmc).nParms == 0 as libc::c_int as libc::c_long {
        return;
    }
    (*pmc).rgdParms = InitdVector((*pmc).nParms);
    (*pmc)
        .rgpMCVar = malloc(
        ((*pmc).nParms as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut MCVAR>() as libc::c_ulong),
    ) as *mut *mut MCVAR;
    if !(!((*pmc).rgdParms).is_null() && !((*pmc).rgpMCVar).is_null()) {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"PrepAnalysis\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    vrgpMCVar = &mut *((*pmc).rgpMCVar).offset(0 as libc::c_int as isize)
        as *mut *mut MCVAR;
    viMCVar = 0 as libc::c_int;
    ForAllList(
        (*pmc).plistMCVars,
        Some(MCVarListToArray as unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int),
        0 as *mut libc::c_void,
    );
    FreeList(&mut (*pmc).plistMCVars, None, 0 as libc::c_int);
    (*pmc)
        .rghvar = malloc(
        ((*pmc).nParms as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HVAR>() as libc::c_ulong),
    ) as *mut HVAR;
    if !((*pmc).rghvar).is_null() {
        l = 0 as libc::c_int;
        while (l as libc::c_long) < (*pmc).nParms {
            *((*pmc).rghvar)
                .offset(l as isize) = (**((*pmc).rgpMCVar).offset(l as isize)).hvar;
            l += 1;
            l;
        }
    } else {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"PrepAnalysis\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    };
}
pub unsafe extern "C" fn PromptFilenames(
    mut pszFileIn: *mut PSTR,
    mut pszFileOut: *mut PSTR,
) {
    *pszFileIn = calloc(
        1 as libc::c_int as libc::c_ulong,
        200 as libc::c_int as libc::c_ulong,
    ) as PSTR;
    *pszFileOut = calloc(
        1 as libc::c_int as libc::c_ulong,
        200 as libc::c_int as libc::c_ulong,
    ) as PSTR;
    printf(b"Input filename? \0" as *const u8 as *const libc::c_char);
    if (fgets(*pszFileIn, 200 as libc::c_int, stdin)).is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x5 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"stdin\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    } else {
        *pszFileIn = strtok(*pszFileIn, b" \t\n\0" as *const u8 as *const libc::c_char);
    }
    if (*pszFileIn).is_null() {
        return;
    }
    if *(*pszFileIn).offset(0 as libc::c_int as isize) != 0 {
        printf(b"Output filename? \0" as *const u8 as *const libc::c_char);
        if (fgets(*pszFileOut, 200 as libc::c_int, stdin)).is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x5 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"stdin\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        } else {
            *pszFileOut = strtok(
                *pszFileOut,
                b" \t\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*pszFileOut).is_null() || *(*pszFileOut).offset(0 as libc::c_int as isize) == 0 {
        free(*pszFileOut as *mut libc::c_void);
        *pszFileOut = 0 as PSTR;
    } else {
        *pszFileIn = realloc(
            *pszFileIn as *mut libc::c_void,
            (if !(*pszFileIn).is_null() {
                strlen(*pszFileIn as *const libc::c_char)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as PSTR;
        *pszFileOut = realloc(
            *pszFileOut as *mut libc::c_void,
            (if !(*pszFileOut).is_null() {
                strlen(*pszFileOut as *const libc::c_char)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as PSTR;
    };
}
static mut vszOptions: [libc::c_char; 14] = unsafe {
    *::std::mem::transmute::<&[u8; 14], &mut [libc::c_char; 14]>(b"c::h::H::i:D:\0")
};
pub unsafe extern "C" fn GetCmdLineArgs(
    mut cArg: libc::c_int,
    mut rgszArg: *const *mut libc::c_char,
    mut pszFileIn: *mut PSTR,
    mut pszFileOut: *mut PSTR,
    mut panal: PANALYSIS,
) {
    let mut c: libc::c_int = 0;
    *pszFileOut = 0 as *mut libc::c_void as PSTR;
    *pszFileIn = *pszFileOut;
    loop {
        c = _getopt(cArg, rgszArg, vszOptions.as_mut_ptr());
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            99 => {
                printf(
                    b">> MPI parallelization not active: option -c is ignored\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            68 => {
                if *optarg.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
                {
                    optarg = optarg.offset(1);
                    optarg;
                }
                if strcmp(
                    optarg,
                    b"print-hierarchy\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    printf(
                        b">> Debug option %s\n\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    (*panal).bDependents = 1 as libc::c_int;
                } else {
                    printf(
                        b">> A known debugging code must follow -D\nExiting.\n\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(-(1 as libc::c_int));
                }
            }
            72 | 104 => {
                printf(
                    b"Usage: %s [options] <input-file> [<output-file>]\n\0" as *const u8
                        as *const libc::c_char,
                    *rgszArg.offset(0 as libc::c_int as isize),
                );
                printf(b"Options:\n\0" as *const u8 as *const libc::c_char);
                printf(
                    b"  -c                   Display MCMC convergence (if MPI is used)\n\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(
                    b"  -D=print-hierarchy   Print out the hierarchy for debugging\n\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(
                    b"  -h                   Display this information\n\0" as *const u8
                        as *const libc::c_char,
                );
                printf(
                    b"  -H                   Display this information\n\0" as *const u8
                        as *const libc::c_char,
                );
                printf(
                    b"  -i=<arg>             Print out every <arg> iteration\n\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(
                    b"\nFor further help on GNU MCSim please see:\nhttp://www.gnu.org/software/mcsim.\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            105 => {
                if *optarg.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
                {
                    optarg = optarg.offset(1);
                    optarg;
                }
                (*panal)
                    .nOutputFreq = strtol(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_int;
                if (*panal).nOutputFreq > 0 as libc::c_int {
                    if (*panal).rank == 0 as libc::c_int {
                        printf(
                            b">> Print iteration frequency %d\n\n\0" as *const u8
                                as *const libc::c_char,
                            (*panal).nOutputFreq,
                        );
                    }
                    (*panal).bOutputIter = 1 as libc::c_int;
                } else {
                    printf(
                        b">> Error: An integer print step must follow -i\nExiting.\n\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(-(1 as libc::c_int));
                }
            }
            _ => {
                printf(b"Exiting.\n\n\0" as *const u8 as *const libc::c_char);
                exit(-(1 as libc::c_int));
            }
        }
    }
    let mut current_block_39: u64;
    match cArg - optind {
        2 => {
            *pszFileOut = *rgszArg.offset((optind + 1 as libc::c_int) as isize);
            current_block_39 = 8412371970001094869;
        }
        1 => {
            current_block_39 = 8412371970001094869;
        }
        0 => {
            PromptFilenames(pszFileIn, pszFileOut);
            current_block_39 = 11048769245176032998;
        }
        _ => {
            exit(-(1 as libc::c_int));
        }
    }
    match current_block_39 {
        8412371970001094869 => {
            *pszFileIn = *rgszArg.offset(optind as isize);
        }
        _ => {}
    }
    while !(*pszFileIn).is_null()
        && *(*pszFileIn).offset(0 as libc::c_int as isize) as libc::c_int != 0
        && MyStrcmp(
            *pszFileIn as *const libc::c_char,
            *pszFileOut as *const libc::c_char,
        ) == 0
    {
        printf(
            b"\n** Input and output filename must be different.\n\0" as *const u8
                as *const libc::c_char,
        );
        PromptFilenames(pszFileIn, pszFileOut);
    }
    if !(!(*pszFileIn).is_null()
        && *(*pszFileIn).offset(0 as libc::c_int as isize) as libc::c_int != 0)
    {
        printf(
            b"Error: an input file name must be specified - Exiting.\n\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
}
pub unsafe extern "C" fn AnnounceProgram(mut iRank: libc::c_int) {
    if iRank == 0 as libc::c_int {
        printf(
            b"\n________________________________________\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(b"\nMCSim v6.2.0\n\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"Copyright (c) 1993-2020 Free Software Foundation, Inc.\n\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"MCSim comes with ABSOLUTELY NO WARRANTY;\nThis is free software, and you are welcome to redistribute it\nunder certain conditions; see the GNU General Public License.\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"* Using `%s' model in file \"%s\" created by %s\n\n\0" as *const u8
                as *const libc::c_char,
            szModelDescFilename.as_mut_ptr(),
            szModelSourceFilename.as_mut_ptr(),
            szModelGenAndVersion.as_mut_ptr(),
        );
    }
}
unsafe fn main_0(
    mut nArg: libc::c_int,
    mut rgszArg: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut rank: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = 1 as libc::c_int;
    let mut szFileIn: PSTR = 0 as *mut libc::c_char;
    let mut szFileOut: PSTR = 0 as *mut libc::c_char;
    let mut ibIn: INPUTBUF = INPUTBUF {
        pfileIn: 0 as *mut FILE,
        pbufOrg: 0 as *mut libc::c_char,
        pbufCur: 0 as *mut libc::c_char,
        iLineNum: 0,
        iLNPrev: 0,
        cErrors: 0,
        pInfo: 0 as *mut libc::c_void,
    };
    let mut panal: PANALYSIS = malloc(::std::mem::size_of::<ANALYSIS>() as libc::c_ulong)
        as PANALYSIS;
    (*panal).rank = rank;
    (*panal).size = size;
    AnnounceProgram((*panal).rank);
    if panal.is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ANALYSIS specification too large\0" as *const u8 as *const libc::c_char
                as PSTR,
            0 as PSTR,
        );
    }
    InitAnalysis(panal);
    GetCmdLineArgs(nArg, rgszArg, &mut szFileIn, &mut szFileOut, panal);
    (*panal).szOutfilename = szFileOut;
    if szFileOut.is_null() {
        (*panal).bCommandLineSpec = 0 as libc::c_int;
    } else {
        (*panal).bCommandLineSpec = 1 as libc::c_int;
    };
    if InitBuffer(&mut ibIn, szFileIn) == 0 {
        ReportError(
            &mut ibIn,
            (0x1 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ReadInput\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    ibIn.pInfo = panal as PVOID;
    if ReadAnalysis(&mut ibIn) != 0 {
        PrepAnalysis(panal);
        DoAnalysis(panal);
    }
    if (*panal).rank == 0 as libc::c_int {
        printf(b"Done.\n\n\0" as *const u8 as *const libc::c_char);
    }
    if (*panal).iType == 4 as libc::c_int {
        FreeLevels(panal);
    } else {
        FreeMemory(panal);
        free(ibIn.pbufOrg as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
