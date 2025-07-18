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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn InitlVector(cVectors: libc::c_long) -> *mut libc::c_long;
    fn InitdVector(cVectors: libc::c_long) -> *mut libc::c_double;
    fn QueueListItem(plist: PLIST, pData: PVOID);
    fn InitList() -> PLIST;
    fn FreeList(pplist: *mut PLIST, pfvFreeData: PFV_FREELISTCALLBACK, bAndData: BOOL);
    fn ForAllList(
        plist: PLIST,
        pfiForAllData: PFI_FORLISTCALLBACK,
        pInfo: PVOID,
    ) -> libc::c_int;
    fn IsParm(hVar: HVAR) -> BOOL;
    fn IsInput(hVar: HVAR) -> BOOL;
    fn GetVarHandle(szName: PSTR) -> HVAR;
    fn GetInputFn(pibIn: PINPUTBUF, sz: PSTR, pifn: PIFN) -> BOOL;
    fn DefDepParm(szLex: PSTR, pdValue: PDOUBLE, phvar: *mut HANDLE) -> BOOL;
    fn UnrollEquation(pibIn: PINPUTBUF, index: libc::c_long, szEqn: PSTR, szEqnU: PSTR);
    fn SkipWhitespace(pibIn: PINPUTBUF) -> libc::c_int;
    fn SkipComment(_: PINPUTBUF);
    fn NextListItem(
        _: PINPUTBUF,
        _: PSTR,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_char,
    ) -> libc::c_int;
    fn NextLex(_: PINPUTBUF, _: *mut libc::c_char, _: PINT);
    fn NextChar(pibIn: PINPUTBUF) -> libc::c_char;
    fn GetStatement(pibIn: PINPUTBUF, szStmt: PSTR);
    fn GetPunct(pibIn: PINPUTBUF, szLex: PSTR, chPunct: libc::c_char) -> libc::c_int;
    fn GetOptPunct(_: PINPUTBUF, _: PSTR, _: libc::c_char) -> libc::c_int;
    fn GetFuncArgs(_: PINPUTBUF, _: libc::c_int, _: PINT, _: PSTR) -> BOOL;
    fn GetArrayBounds(pibIn: PINPUTBUF, piLB: PLONG, piUB: PLONG);
    fn FillBuffer(pibIn: PINPUTBUF) -> libc::c_int;
    fn ENextLex(_: PINPUTBUF, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn EGetPunct(pibIn: PINPUTBUF, szLex: PSTR, chPunct: libc::c_char) -> libc::c_int;
    fn EatStatement(pib: PINPUTBUF);
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn ReportRunTimeError(_: PANALYSIS, _: WORD, _: ...);
    fn FreeVarMod(pData: PVOID);
    fn MyStrcmp(sz1: *const libc::c_char, sz2: *const libc::c_char) -> libc::c_int;
    fn PrepareOutSpec(pexp: PEXPERIMENT) -> BOOL;
    fn InitSetPoints(pMC: PMONTECARLO) -> BOOL;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type PSTRLEX_ = [libc::c_char; 255];
pub type PSTREQN_ = [libc::c_char; 1023];
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
pub type IFN = tagIFN;
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
pub type VARMODIFICATION = tagVARMODIFICATION;
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
pub type PRINTREC = tagPRINTREC;
pub type PPRINTREC = *mut tagPRINTREC;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagDATAREC {
    pub szDataName: PSTR,
    pub hvar: HVAR,
    pub cData: libc::c_long,
    pub pdData: PDOUBLE,
}
pub type DATAREC = tagDATAREC;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const backward: C2RustUnnamed_0 = 1;
pub const forward: C2RustUnnamed_0 = 0;
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
pub type LEVEL = tagLEVEL;
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
pub type PKM = *mut tagKM;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagKM {
    pub szKeyword: PSTR,
    pub iKWCode: libc::c_int,
    pub fContext: WORD,
}
pub type KM = tagKM;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
pub static mut vrgszlexArgs: [PSTRLEX_; 8] = [[0; 255]; 8];
pub static mut vrgkmKeywordMap: [KM; 69] = [
    {
        let mut init = tagKM {
            szKeyword: b"Level\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 16 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Experiment\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 10 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Simulation\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 10 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"OutputFile\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 14 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"MCMC\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 15 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"OptimalDesign\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 17 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"MonteCarlo\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 11 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Distrib\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 12 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Likelihood\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 12 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Density\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 12 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"MCVary\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 12 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"InvTemperature\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 4 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Perks\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 4 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"SetPoints\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 13 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Integrate\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 1 as libc::c_int,
            fContext: (0x1 as libc::c_int | 0x2 as libc::c_int) as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Simulate\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 2 as libc::c_int,
            fContext: (0x1 as libc::c_int | 0x2 as libc::c_int) as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"StartTime\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 3 as libc::c_int,
            fContext: (0x1 as libc::c_int | 0x2 as libc::c_int) as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Print\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 5 as libc::c_int,
            fContext: 0x2 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Prediction\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 5 as libc::c_int,
            fContext: 0x2 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"PrintStep\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 6 as libc::c_int,
            fContext: 0x2 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Data\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 7 as libc::c_int,
            fContext: 0x2 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"SimType\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 8 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"End\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 100 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"END\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 100 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"DefaultSim\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 9 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"No\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 201 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Yes\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 200 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Beta\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 212 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Binomial\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 218 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"BinomialBeta\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 228 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Cauchy\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 233 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Chi2\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 217 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Exponential\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 220 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Gamma\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 221 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"GenLogNormal\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 231 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"HalfCauchy\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 234 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"HalfNormal\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 229 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"InvGamma\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 223 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"LogNormal\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 214 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"LogNormal_v\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 225 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"LogUniform\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 211 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"NegativeBinomial\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 238 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Normal\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 213 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Normal_cv\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 235 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Normal_v\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 224 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Piecewise\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 219 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Poisson\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 222 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"StudentT\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 232 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"TruncInvGamma\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 230 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"TruncLogNormal\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 216 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"TruncLogNormal_v\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 227 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"TruncNormal\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 215 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"TruncNormal_cv\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 236 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"TruncNormal_v\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 226 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Uniform\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 210 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"UserSpecifiedLL\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 237 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Prediction\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 300 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Data\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 7 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Euler\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 602 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Lsodes\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 600 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"LSODES\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 600 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Cvodes\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 601 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"CVODES\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 601 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Forward\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 700 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Backward\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 701 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Replace\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 710 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Add\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 711 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Multiply\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 712 as libc::c_int,
            fContext: 0x100 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 0 as libc::c_int,
            fContext: 0xffff as libc::c_int as WORD,
        };
        init
    },
];
pub unsafe extern "C" fn GetKeywordCode(
    mut szKeyword: PSTR,
    mut pfContext: PINT,
) -> libc::c_int {
    let mut pkm: PKM = &mut *vrgkmKeywordMap
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut KM;
    while *(*pkm).szKeyword as libc::c_int != 0
        && MyStrcmp(
            szKeyword as *const libc::c_char,
            (*pkm).szKeyword as *const libc::c_char,
        ) != 0
    {
        pkm = pkm.offset(1);
        pkm;
    }
    if !pfContext.is_null() {
        *pfContext = (*pkm).fContext as libc::c_int;
    }
    return (*pkm).iKWCode;
}
pub unsafe extern "C" fn GetKeywordCode_in_context(
    mut szKeyword: PSTR,
    mut fContext: WORD,
) -> libc::c_int {
    let mut pkm: PKM = &mut *vrgkmKeywordMap
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut KM;
    while *(*pkm).szKeyword as libc::c_int != 0
        && !((*pkm).fContext == fContext
            && MyStrcmp(
                szKeyword as *const libc::c_char,
                (*pkm).szKeyword as *const libc::c_char,
            ) == 0)
    {
        pkm = pkm.offset(1);
        pkm;
    }
    return (*pkm).iKWCode;
}
pub unsafe extern "C" fn GetKeyword(mut iKWCode: libc::c_int) -> PSTR {
    let mut pkm: PKM = &mut *vrgkmKeywordMap
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut KM;
    while *(*pkm).szKeyword as libc::c_int != 0 && iKWCode != (*pkm).iKWCode {
        pkm = pkm.offset(1);
        pkm;
    }
    return (*pkm).szKeyword;
}
pub unsafe extern "C" fn YesNoFromLex(mut szLex: PSTR) -> BOOL {
    let mut ikwcode: libc::c_int = GetKeywordCode(szLex, 0 as PINT);
    let mut bReturn: BOOL = 0;
    bReturn = if *(*__ctype_b_loc())
        .offset(*szLex.offset(0 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        atoi(szLex as *const libc::c_char)
    } else if ikwcode == 200 as libc::c_int {
        1 as libc::c_int
    } else if ikwcode == 201 as libc::c_int {
        0 as libc::c_int
    } else {
        0 as libc::c_int
    };
    return bReturn;
}
pub unsafe extern "C" fn ImFromLex(mut szLex: PSTR) -> libc::c_long {
    let mut ikwcode: libc::c_int = GetKeywordCode(szLex, 0 as PINT);
    let mut lReturn: libc::c_long = 0;
    lReturn = (if *(*__ctype_b_loc())
        .offset(*szLex.offset(0 as libc::c_int as isize) as libc::c_int as isize)
        as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        atoi(szLex as *const libc::c_char)
    } else if ikwcode == 600 as libc::c_int {
        3 as libc::c_int
    } else if ikwcode == 601 as libc::c_int {
        4 as libc::c_int
    } else if ikwcode == 602 as libc::c_int {
        2 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_long;
    return lReturn;
}
pub unsafe extern "C" fn McvFromLex(mut szLex: PSTR) -> libc::c_int {
    let mut ikwcode: libc::c_int = GetKeywordCode(szLex, 0 as PINT);
    let mut iReturn: libc::c_int = 0;
    iReturn = if ikwcode == 210 as libc::c_int {
        0 as libc::c_int
    } else if ikwcode == 211 as libc::c_int {
        1 as libc::c_int
    } else if ikwcode == 212 as libc::c_int {
        2 as libc::c_int
    } else if ikwcode == 213 as libc::c_int {
        3 as libc::c_int
    } else if ikwcode == 229 as libc::c_int {
        19 as libc::c_int
    } else if ikwcode == 214 as libc::c_int {
        4 as libc::c_int
    } else if ikwcode == 215 as libc::c_int {
        5 as libc::c_int
    } else if ikwcode == 216 as libc::c_int {
        6 as libc::c_int
    } else if ikwcode == 217 as libc::c_int {
        7 as libc::c_int
    } else if ikwcode == 218 as libc::c_int {
        8 as libc::c_int
    } else if ikwcode == 219 as libc::c_int {
        9 as libc::c_int
    } else if ikwcode == 220 as libc::c_int {
        10 as libc::c_int
    } else if ikwcode == 221 as libc::c_int {
        11 as libc::c_int
    } else if ikwcode == 222 as libc::c_int {
        12 as libc::c_int
    } else if ikwcode == 223 as libc::c_int {
        13 as libc::c_int
    } else if ikwcode == 230 as libc::c_int {
        20 as libc::c_int
    } else if ikwcode == 224 as libc::c_int {
        14 as libc::c_int
    } else if ikwcode == 235 as libc::c_int {
        25 as libc::c_int
    } else if ikwcode == 225 as libc::c_int {
        15 as libc::c_int
    } else if ikwcode == 226 as libc::c_int {
        16 as libc::c_int
    } else if ikwcode == 236 as libc::c_int {
        26 as libc::c_int
    } else if ikwcode == 227 as libc::c_int {
        17 as libc::c_int
    } else if ikwcode == 228 as libc::c_int {
        18 as libc::c_int
    } else if ikwcode == 231 as libc::c_int {
        21 as libc::c_int
    } else if ikwcode == 232 as libc::c_int {
        22 as libc::c_int
    } else if ikwcode == 233 as libc::c_int {
        23 as libc::c_int
    } else if ikwcode == 234 as libc::c_int {
        24 as libc::c_int
    } else if ikwcode == 237 as libc::c_int {
        27 as libc::c_int
    } else if ikwcode == 238 as libc::c_int {
        28 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    return iReturn;
}
pub unsafe extern "C" fn GetTerminator(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
) -> libc::c_int {
    let mut iErr: libc::c_int = 0;
    iErr = (GetPunct(pibIn, szLex, ';' as i32 as libc::c_char) == 0) as libc::c_int;
    if iErr != 0 {
        *szLex.offset(1 as libc::c_int as isize) = ';' as i32 as libc::c_char;
        ReportError(pibIn, 0x13 as libc::c_int as WORD, szLex, 0 as PSTR);
    }
    return iErr;
}
pub unsafe extern "C" fn GetSimType(mut pibIn: PINPUTBUF) -> BOOL {
    static mut vrgiAtArgTypes: [libc::c_int; 1] = [0x1 as libc::c_int];
    let mut panal: PANALYSIS = (*pibIn).pInfo as PANALYSIS;
    let mut iAT: libc::c_int = 1 as libc::c_int;
    let mut iKwCode: libc::c_int = 0 as libc::c_int;
    let mut bErr: BOOL = (GetFuncArgs(
        pibIn,
        1 as libc::c_int,
        vrgiAtArgTypes.as_mut_ptr(),
        (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
    ) == 0) as libc::c_int;
    if bErr == 0 {
        iKwCode = GetKeywordCode(
            (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
            0 as PINT,
        );
        match iKwCode {
            11 => {
                iAT = 2 as libc::c_int;
            }
            13 => {
                iAT = 3 as libc::c_int;
            }
            15 => {
                iAT = 4 as libc::c_int;
            }
            17 => {
                iAT = 5 as libc::c_int;
            }
            9 => {
                iAT = 1 as libc::c_int;
            }
            _ => {
                ReportError(
                    pibIn,
                    (0x205 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"Unknown SimType \0" as *const u8 as *const libc::c_char as PSTR,
                    (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
                );
            }
        }
    } else {
        printf(
            b"Syntax: %s (Normal | MonteCarlo | SetPoints | MCMC)\n  -- if not specified, the first spec section will be used.\n\n\0"
                as *const u8 as *const libc::c_char,
            GetKeyword(8 as libc::c_int),
        );
    }
    if bErr == 0 {
        (*panal).iType = iAT;
    }
    return bErr;
}
pub unsafe extern "C" fn GetPerks(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pgd: PGIBBSDATA,
) -> BOOL {
    let mut iType: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bOK: BOOL = 1 as libc::c_int;
    let mut bErr: BOOL = 0 as libc::c_int;
    bErr = EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char);
    if !(bErr != 0) {
        bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
        if !(bErr != 0) {
            (*pgd).nPerks = atoi(szLex as *const libc::c_char);
            bErr = ((*pgd).nPerks <= 0 as libc::c_int) as libc::c_int;
            if bErr != 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"positive-integer\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            } else {
                (*pgd).endT = (*pgd).nPerks - 1 as libc::c_int;
                (*pgd).rgdPerks = InitdVector((*pgd).nPerks as libc::c_long);
                if ((*pgd).rgdPerks).is_null() {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetPerks\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                (*pgd).rgdlnPi = InitdVector((*pgd).nPerks as libc::c_long);
                if ((*pgd).rgdlnPi).is_null()
                    || {
                        (*pgd).rglCount = InitlVector((*pgd).nPerks as libc::c_long);
                        ((*pgd).rglCount).is_null()
                    }
                {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetPerks\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                i = 0 as libc::c_int;
                while i < (*pgd).nPerks && bOK != 0 {
                    bOK = GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
                    if bOK == 0 {
                        let fresh0 = (*pibIn).pbufCur;
                        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                        *szLex.offset(0 as libc::c_int as isize) = *fresh0;
                        *szLex
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b",\0" as *const u8 as *const libc::c_char as PSTR,
                            szLex,
                        );
                    }
                    NextLex(pibIn, szLex, &mut iType);
                    bOK
                        &= (iType & (0x2 as libc::c_int | 0x4 as libc::c_int)
                            > 0 as libc::c_int) as libc::c_int;
                    if bOK == 0 {
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"number\0" as *const u8 as *const libc::c_char as PSTR,
                            szLex,
                        );
                    }
                    *((*pgd).rgdPerks)
                        .offset(i as isize) = atof(szLex as *const libc::c_char);
                    *((*pgd).rgdlnPi)
                        .offset(i as isize) = 0 as libc::c_int as libc::c_double;
                    *((*pgd).rglCount)
                        .offset(i as isize) = 0 as libc::c_int as libc::c_long;
                    if *((*pgd).rgdPerks).offset(i as isize)
                        < 0 as libc::c_int as libc::c_double
                    {
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"positive inverse temperature\0" as *const u8
                                as *const libc::c_char as PSTR,
                            szLex,
                        );
                    }
                    if i > 0 as libc::c_int
                        && *((*pgd).rgdPerks).offset(i as isize)
                            <= *((*pgd).rgdPerks).offset((i - 1 as libc::c_int) as isize)
                    {
                        ReportError(
                            pibIn,
                            (0x205 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"Inverse temperatures out of order\0" as *const u8
                                as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                    i += 1;
                    i;
                }
                bErr = EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char);
            }
        }
    }
    if bErr != 0 {
        printf(
            b"Syntax: Inverse temperatures (nPerks, <n increasing inverse temperature values >= 0>)\n\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return (bErr == 0) as libc::c_int;
}
pub unsafe extern "C" fn GetLsodesOptions(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pis: PINTSPEC,
) -> BOOL {
    let mut bErr: BOOL = 0 as libc::c_int;
    if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
        ReportError(
            pibIn,
            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b",\0" as *const u8 as *const libc::c_char as PSTR,
            szLex,
        );
    }
    bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
    if !(bErr != 0) {
        (*pis).dRtol = atof(szLex as *const libc::c_char);
        if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
            ReportError(
                pibIn,
                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b",\0" as *const u8 as *const libc::c_char as PSTR,
                szLex,
            );
        }
        bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
        if !(bErr != 0) {
            (*pis).dAtol = atof(szLex as *const libc::c_char);
            if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b",\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            }
            bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
            if !(bErr != 0) {
                (*pis).iMf = atoi(szLex as *const libc::c_char) as libc::c_long;
                match (*pis).iMf {
                    0 => {
                        (*pis).iMf = 10 as libc::c_int as libc::c_long;
                    }
                    1 => {
                        (*pis).iMf = 222 as libc::c_int as libc::c_long;
                    }
                    2 => {
                        (*pis).iMf = 121 as libc::c_int as libc::c_long;
                    }
                    _ => {
                        printf(
                            b"Error: method flag must be 0, 1 or 2 for Lsodes - \0"
                                as *const u8 as *const libc::c_char,
                        );
                        printf(b"Exiting\n\n\0" as *const u8 as *const libc::c_char);
                        exit(0 as libc::c_int);
                    }
                }
                (*pis).iDSFlag = 1 as libc::c_int as libc::c_long;
            }
        }
    }
    if bErr != 0 {
        printf(
            b"Lsodes options are: relative tolerance, absolute tolerance, method.\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    return bErr;
}
pub unsafe extern "C" fn GetCvodesOptions(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pis: PINTSPEC,
) -> BOOL {
    let mut bErr: BOOL = 0 as libc::c_int;
    if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
        ReportError(
            pibIn,
            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b",\0" as *const u8 as *const libc::c_char as PSTR,
            szLex,
        );
    }
    bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
    if !(bErr != 0) {
        (*pis).dRtol = atof(szLex as *const libc::c_char);
        if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
            ReportError(
                pibIn,
                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b",\0" as *const u8 as *const libc::c_char as PSTR,
                szLex,
            );
        }
        bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
        if !(bErr != 0) {
            (*pis).dAtol = atof(szLex as *const libc::c_char);
            if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b",\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            }
            bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
            if !(bErr != 0) {
                (*pis).maxsteps = atoi(szLex as *const libc::c_char);
                if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b",\0" as *const u8 as *const libc::c_char as PSTR,
                        szLex,
                    );
                }
                bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
                if !(bErr != 0) {
                    (*pis).maxnef = atoi(szLex as *const libc::c_char);
                    if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b",\0" as *const u8 as *const libc::c_char as PSTR,
                            szLex,
                        );
                    }
                    bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
                    if !(bErr != 0) {
                        (*pis).maxcor = atoi(szLex as *const libc::c_char);
                        if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
                            ReportError(
                                pibIn,
                                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                b",\0" as *const u8 as *const libc::c_char as PSTR,
                                szLex,
                            );
                        }
                        bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
                        if !(bErr != 0) {
                            (*pis).maxncf = atoi(szLex as *const libc::c_char);
                            if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
                                ReportError(
                                    pibIn,
                                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                    b",\0" as *const u8 as *const libc::c_char as PSTR,
                                    szLex,
                                );
                            }
                            bErr = ENextLex(
                                pibIn,
                                szLex,
                                0x2 as libc::c_int | 0x4 as libc::c_int,
                            );
                            if !(bErr != 0) {
                                (*pis).nlscoef = atof(szLex as *const libc::c_char);
                            }
                        }
                    }
                }
            }
        }
    }
    if bErr != 0 {
        printf(
            b"Cvodes options are: relative tolerance, absolute tolerance, maxsteps, maxnef, maxcor, maxncf, nlscoef.\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    return bErr;
}
pub unsafe extern "C" fn GetEulerOptions(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pis: PINTSPEC,
) -> BOOL {
    let mut bErr: BOOL = 0 as libc::c_int;
    if GetPunct(pibIn, szLex, ',' as i32 as libc::c_char) == 0 {
        ReportError(
            pibIn,
            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b",\0" as *const u8 as *const libc::c_char as PSTR,
            szLex,
        );
    }
    bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
    if !(bErr != 0) {
        (*pis).dTStep = atof(szLex as *const libc::c_char);
        if (*pis).dTStep <= 0 as libc::c_int as libc::c_double {
            printf(
                b"Error: Time step specified is null or negative - Exiting\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    if bErr != 0 {
        printf(
            b"Euler has one option: time-step.\n\n\0" as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    return bErr;
}
pub unsafe extern "C" fn GetIntegrate(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pis: PINTSPEC,
) -> BOOL {
    let mut current_block: u64;
    let mut bErr: BOOL = 0 as libc::c_int;
    bErr = EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char);
    if !(bErr != 0) {
        bErr = ENextLex(pibIn, szLex, 0x1 as libc::c_int);
        if !(bErr != 0) {
            (*pis).iAlgo = ImFromLex(szLex) as libc::c_int;
            match (*pis).iAlgo {
                3 => {
                    GetLsodesOptions(pibIn, szLex, pis);
                    current_block = 7651349459974463963;
                }
                4 => {
                    printf(
                        b"Warning: Cvodes libraries are not available -\n         Switching to Lsodes with default options\n\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    (*pis).iAlgo = 3 as libc::c_int;
                    current_block = 2203719417613228758;
                }
                2 => {
                    GetEulerOptions(pibIn, szLex, pis);
                    current_block = 7651349459974463963;
                }
                _ => {
                    printf(
                        b"Error: Unknown integration method: %s - Exiting\n\n\0"
                            as *const u8 as *const libc::c_char,
                        (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
                    );
                    bErr = 1 as libc::c_int;
                    current_block = 2203719417613228758;
                }
            }
            match current_block {
                2203719417613228758 => {}
                _ => {
                    bErr = EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char);
                }
            }
        }
    }
    if bErr != 0 {
        printf(
            b"Syntax: %s([Lsodes | Cvodes | Euler], [OPTIONS]);\n\n\0" as *const u8
                as *const libc::c_char,
            GetKeyword(1 as libc::c_int),
        );
        exit(0 as libc::c_int);
    }
    return (bErr == 0) as libc::c_int;
}
pub unsafe extern "C" fn OneDToArray(mut pData: PVOID, mut pInfo: PVOID) -> libc::c_int {
    let mut ppdArrayVal: *mut PDOUBLE = pInfo as *mut PDOUBLE;
    let fresh1 = *ppdArrayVal;
    *ppdArrayVal = (*ppdArrayVal).offset(1);
    *fresh1 = *(pData as PDOUBLE);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn DListToArray(
    mut plist: PLIST,
    mut pcDouble: PLONG,
    mut ppDouble: *mut PDOUBLE,
) {
    let mut pdTmp: PDOUBLE = 0 as *mut libc::c_double;
    *pcDouble = (if !plist.is_null() { (*plist).iSize } else { 0 as libc::c_int })
        as libc::c_long;
    *ppDouble = InitdVector(*pcDouble);
    pdTmp = *ppDouble;
    if pdTmp.is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"DListToArray\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    ForAllList(
        plist,
        Some(OneDToArray as unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int),
        &mut pdTmp as *mut PDOUBLE as PVOID,
    );
}
pub unsafe extern "C" fn GetListOfTimes(
    mut pibIn: PINPUTBUF,
    mut nRecs: libc::c_int,
    mut ppr: *mut PPRINTREC,
    mut szLex: PSTR,
) -> BOOL {
    let mut plistTimes: PLIST = InitList();
    let mut pdTmp: PDOUBLE = 0 as *mut libc::c_double;
    let mut iNLI: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut bErr: BOOL = 0;
    loop {
        pdTmp = InitdVector(1 as libc::c_int as libc::c_long);
        if pdTmp.is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"GetListOfTimes\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        *pdTmp = atof(szLex as *const libc::c_char);
        QueueListItem(plistTimes, pdTmp as PVOID);
        iNLI = NextListItem(
            pibIn,
            szLex,
            0x2 as libc::c_int | 0x4 as libc::c_int,
            1 as libc::c_int,
            ')' as i32 as libc::c_char,
        );
        if !(iNLI > 0 as libc::c_int) {
            break;
        }
    }
    if iNLI == 0 {
        bErr = (EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char) != 0
            || (if !plistTimes.is_null() {
                (*plistTimes).iSize
            } else {
                0 as libc::c_int
            }) == 0) as libc::c_int;
    } else {
        bErr = 1 as libc::c_int;
        ReportError(
            pibIn,
            0x14 as libc::c_int as WORD,
            b"number\0" as *const u8 as *const libc::c_char as PSTR,
            szLex,
        );
    }
    if bErr == 0 {
        i = 0 as libc::c_int;
        while i < nRecs {
            DListToArray(
                plistTimes,
                &mut (**ppr.offset(i as isize)).cTimes,
                &mut (**ppr.offset(i as isize)).pdTimes,
            );
            i += 1;
            i;
        }
    }
    FreeList(&mut plistTimes, None, 1 as libc::c_int);
    i = 1 as libc::c_int;
    while (i as libc::c_long) < (**ppr.offset(0 as libc::c_int as isize)).cTimes
        && bErr == 0
    {
        bErr = (*((**ppr.offset(0 as libc::c_int as isize)).pdTimes).offset(i as isize)
            <= *((**ppr.offset(0 as libc::c_int as isize)).pdTimes)
                .offset(i as isize)
                .offset(-(1 as libc::c_int as isize))) as libc::c_int;
        if bErr != 0 {
            j = 0 as libc::c_int;
            while j < nRecs {
                free((**ppr.offset(j as isize)).pdTimes as *mut libc::c_void);
                j += 1;
                j;
            }
            ReportError(
                pibIn,
                (0x205 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"Times out of order\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        i += 1;
        i;
    }
    return bErr;
}
pub unsafe extern "C" fn GetListOfData(
    mut pibIn: PINPUTBUF,
    mut pda: PDATAREC,
    mut szLex: PSTR,
) -> BOOL {
    let mut plistData: PLIST = InitList();
    let mut pdTmp: PDOUBLE = 0 as *mut libc::c_double;
    let mut iNLI: libc::c_int = 0;
    let mut bErr: BOOL = 0;
    loop {
        iNLI = NextListItem(
            pibIn,
            szLex,
            0x2 as libc::c_int | 0x4 as libc::c_int,
            1 as libc::c_int,
            ')' as i32 as libc::c_char,
        );
        if !(iNLI > 0 as libc::c_int) {
            break;
        }
        pdTmp = InitdVector(1 as libc::c_int as libc::c_long);
        if pdTmp.is_null() {
            ReportError(
                pibIn,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"GetListOfData\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        *pdTmp = atof(szLex as *const libc::c_char);
        QueueListItem(plistData, pdTmp as PVOID);
    }
    if iNLI == 0 {
        bErr = (EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char) != 0
            || (if !plistData.is_null() { (*plistData).iSize } else { 0 as libc::c_int })
                == 0) as libc::c_int;
    } else {
        bErr = 1 as libc::c_int;
        ReportError(
            pibIn,
            0x14 as libc::c_int as WORD,
            b"number\0" as *const u8 as *const libc::c_char as PSTR,
            szLex,
        );
    }
    if bErr == 0 {
        DListToArray(plistData, &mut (*pda).cData, &mut (*pda).pdData);
    }
    FreeList(&mut plistData, None, 1 as libc::c_int);
    return bErr;
}
pub static mut bGavePrintUsage: BOOL = 0 as libc::c_int;
pub unsafe extern "C" fn GetPrint(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pos: POUTSPEC,
) -> BOOL {
    let mut pprintrec: [PPRINTREC; 200] = [0 as *mut tagPRINTREC; 200];
    let mut bErr: BOOL = 0 as libc::c_int;
    let mut hvar: HVAR = 0;
    let mut nVars: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut iLex: libc::c_int = 0;
    let mut i: libc::c_long = 0;
    let mut iLB: libc::c_long = 0;
    let mut iUB: libc::c_long = 0;
    let mut szTmp: PSTRLEX_ = [0; 255];
    bErr = EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char);
    if bErr == 0 {
        loop {
            NextLex(pibIn, szLex, &mut iLex);
            if iLex != 0x1 as libc::c_int {
                break;
            }
            iUB = -(1 as libc::c_int) as libc::c_long;
            iLB = iUB;
            if GetPunct(pibIn, szTmp.as_mut_ptr(), '[' as i32 as libc::c_char) != 0 {
                GetArrayBounds(pibIn, &mut iLB, &mut iUB);
            }
            if iUB == -(1 as libc::c_int) as libc::c_long {
                if nVars == 200 as libc::c_int {
                    ReportError(
                        pibIn,
                        (0x117 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetPrint\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                hvar = GetVarHandle(szLex);
                bErr = (hvar == 0) as libc::c_int;
                if bErr != 0 {
                    ReportError(
                        pibIn,
                        (0x106 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        szLex,
                        0 as PSTR,
                    );
                } else {
                    pprintrec[nVars
                        as usize] = malloc(
                        ::std::mem::size_of::<PRINTREC>() as libc::c_ulong,
                    ) as PPRINTREC;
                    if (pprintrec[nVars as usize]).is_null()
                        || {
                            (*pprintrec[nVars as usize])
                                .szOutputName = malloc(
                                (if !szLex.is_null() {
                                    strlen(szLex as *const libc::c_char)
                                } else {
                                    0 as libc::c_int as libc::c_ulong
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as PSTR;
                            ((*pprintrec[nVars as usize]).szOutputName).is_null()
                        }
                    {
                        ReportError(
                            pibIn,
                            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"GetPrint\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                    if !((*pprintrec[nVars as usize]).szOutputName).is_null()
                        && !szLex.is_null()
                    {
                        strcpy(
                            (*pprintrec[nVars as usize]).szOutputName,
                            szLex as *const libc::c_char,
                        );
                    } else {};
                    (*pprintrec[nVars as usize]).hvar = hvar;
                    if !(pprintrec[nVars as usize]).is_null() {} else {
                        __assert_fail(
                            b"pprintrec[nVars]\0" as *const u8 as *const libc::c_char,
                            b"simi.c\0" as *const u8 as *const libc::c_char,
                            862 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 41],
                                &[libc::c_char; 41],
                            >(b"BOOL GetPrint(PINPUTBUF, PSTR, POUTSPEC)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_13018: {
                        if !(pprintrec[nVars as usize]).is_null() {} else {
                            __assert_fail(
                                b"pprintrec[nVars]\0" as *const u8 as *const libc::c_char,
                                b"simi.c\0" as *const u8 as *const libc::c_char,
                                862 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 41],
                                    &[libc::c_char; 41],
                                >(b"BOOL GetPrint(PINPUTBUF, PSTR, POUTSPEC)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    nVars += 1;
                    nVars;
                }
                GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
            } else {
                i = iLB;
                while i < iUB {
                    sprintf(
                        szTmp.as_mut_ptr(),
                        b"%s_%ld\0" as *const u8 as *const libc::c_char,
                        szLex,
                        i,
                    );
                    if nVars == 200 as libc::c_int {
                        ReportError(
                            pibIn,
                            (0x117 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"GetPrint\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                    hvar = GetVarHandle(szTmp.as_mut_ptr());
                    bErr = (hvar == 0) as libc::c_int;
                    if bErr != 0 {
                        ReportError(
                            pibIn,
                            (0x106 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            szTmp.as_mut_ptr(),
                            0 as PSTR,
                        );
                    } else {
                        pprintrec[nVars
                            as usize] = malloc(
                            ::std::mem::size_of::<PRINTREC>() as libc::c_ulong,
                        ) as PPRINTREC;
                        if (pprintrec[nVars as usize]).is_null()
                            || {
                                (*pprintrec[nVars as usize])
                                    .szOutputName = malloc(
                                    (strlen(szTmp.as_mut_ptr()))
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as PSTR;
                                ((*pprintrec[nVars as usize]).szOutputName).is_null()
                            }
                        {
                            ReportError(
                                pibIn,
                                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                b"GetPrint\0" as *const u8 as *const libc::c_char as PSTR,
                                0 as PSTR,
                            );
                        }
                        strcpy(
                            (*pprintrec[nVars as usize]).szOutputName,
                            szTmp.as_mut_ptr(),
                        );
                        (*pprintrec[nVars as usize]).hvar = hvar;
                        if !(pprintrec[nVars as usize]).is_null() {} else {
                            __assert_fail(
                                b"pprintrec[nVars]\0" as *const u8 as *const libc::c_char,
                                b"simi.c\0" as *const u8 as *const libc::c_char,
                                885 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 41],
                                    &[libc::c_char; 41],
                                >(b"BOOL GetPrint(PINPUTBUF, PSTR, POUTSPEC)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_12803: {
                            if !(pprintrec[nVars as usize]).is_null() {} else {
                                __assert_fail(
                                    b"pprintrec[nVars]\0" as *const u8 as *const libc::c_char,
                                    b"simi.c\0" as *const u8 as *const libc::c_char,
                                    885 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 41],
                                        &[libc::c_char; 41],
                                    >(b"BOOL GetPrint(PINPUTBUF, PSTR, POUTSPEC)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                        nVars += 1;
                        nVars;
                    }
                    if i == iUB - 1 as libc::c_int as libc::c_long {
                        GetOptPunct(
                            pibIn,
                            szTmp.as_mut_ptr(),
                            ',' as i32 as libc::c_char,
                        );
                    }
                    i += 1;
                    i;
                }
            }
        }
        if nVars < 1 as libc::c_int {
            ReportError(
                pibIn,
                0x14 as libc::c_int as WORD,
                b"identifier\0" as *const u8 as *const libc::c_char as PSTR,
                szLex,
            );
        }
        bErr = GetListOfTimes(pibIn, nVars, pprintrec.as_mut_ptr(), szLex);
        if bErr != 0 {
            n = 0 as libc::c_int;
            while n < nVars {
                free((*pprintrec[n as usize]).szOutputName as *mut libc::c_void);
                free(pprintrec[n as usize] as *mut libc::c_void);
                n += 1;
                n;
            }
        } else {
            n = 0 as libc::c_int;
            while n < nVars {
                QueueListItem((*pos).plistPrintRecs, pprintrec[n as usize] as PVOID);
                n += 1;
                n;
            }
        }
    }
    if bErr == 0 {
        bErr = GetTerminator(pibIn, szLex);
    } else if bGavePrintUsage == 0 {
        printf(
            b"Syntax: %s (<Identifiers>, Time1, Time2, ...)\n\n\0" as *const u8
                as *const libc::c_char,
            GetKeyword(5 as libc::c_int),
        );
        bGavePrintUsage = 1 as libc::c_int;
    }
    return bErr;
}
pub static mut bGavePrintStepUsage: BOOL = 0 as libc::c_int;
pub unsafe extern "C" fn GetPrintStep(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pos: POUTSPEC,
) -> BOOL {
    let mut pprintrec: [PPRINTREC; 200] = [0 as *mut tagPRINTREC; 200];
    let mut bErr: BOOL = 0 as libc::c_int;
    let mut bOK: BOOL = 1 as libc::c_int;
    let mut hvar: HVAR = 0 as libc::c_int as HVAR;
    let mut nVars: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut iLex: libc::c_int = 0;
    let mut i: libc::c_long = 0;
    let mut iLB: libc::c_long = 0;
    let mut iUB: libc::c_long = 0;
    let mut dStart: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dEnd: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dStep: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut dTmp: libc::c_double = 0.;
    let mut szTmp: PSTRLEX_ = [0; 255];
    bErr = EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char);
    if !(bErr != 0) {
        loop {
            NextLex(pibIn, szLex, &mut iLex);
            if iLex != 0x1 as libc::c_int {
                break;
            }
            iUB = -(1 as libc::c_int) as libc::c_long;
            iLB = iUB;
            if GetPunct(pibIn, szTmp.as_mut_ptr(), '[' as i32 as libc::c_char) != 0 {
                GetArrayBounds(pibIn, &mut iLB, &mut iUB);
            }
            if iUB == -(1 as libc::c_int) as libc::c_long {
                if nVars == 200 as libc::c_int {
                    ReportError(
                        pibIn,
                        (0x117 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetPrint\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                hvar = GetVarHandle(szLex);
                bErr = (hvar == 0) as libc::c_int;
                if bErr != 0 {
                    ReportError(
                        pibIn,
                        (0x106 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        szLex,
                        0 as PSTR,
                    );
                } else {
                    pprintrec[nVars
                        as usize] = malloc(
                        ::std::mem::size_of::<PRINTREC>() as libc::c_ulong,
                    ) as PPRINTREC;
                    if (pprintrec[nVars as usize]).is_null()
                        || {
                            (*pprintrec[nVars as usize])
                                .szOutputName = malloc(
                                (if !szLex.is_null() {
                                    strlen(szLex as *const libc::c_char)
                                } else {
                                    0 as libc::c_int as libc::c_ulong
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            ) as PSTR;
                            ((*pprintrec[nVars as usize]).szOutputName).is_null()
                        }
                    {
                        ReportError(
                            pibIn,
                            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"GetPrint\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                    if !((*pprintrec[nVars as usize]).szOutputName).is_null()
                        && !szLex.is_null()
                    {
                        strcpy(
                            (*pprintrec[nVars as usize]).szOutputName,
                            szLex as *const libc::c_char,
                        );
                    } else {};
                    (*pprintrec[nVars as usize]).hvar = hvar;
                    if !(pprintrec[nVars as usize]).is_null() {} else {
                        __assert_fail(
                            b"pprintrec[nVars]\0" as *const u8 as *const libc::c_char,
                            b"simi.c\0" as *const u8 as *const libc::c_char,
                            977 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 45],
                                &[libc::c_char; 45],
                            >(b"BOOL GetPrintStep(PINPUTBUF, PSTR, POUTSPEC)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_14027: {
                        if !(pprintrec[nVars as usize]).is_null() {} else {
                            __assert_fail(
                                b"pprintrec[nVars]\0" as *const u8 as *const libc::c_char,
                                b"simi.c\0" as *const u8 as *const libc::c_char,
                                977 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 45],
                                    &[libc::c_char; 45],
                                >(b"BOOL GetPrintStep(PINPUTBUF, PSTR, POUTSPEC)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    nVars += 1;
                    nVars;
                }
                GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
            } else {
                i = iLB;
                while i < iUB {
                    sprintf(
                        szTmp.as_mut_ptr(),
                        b"%s_%ld\0" as *const u8 as *const libc::c_char,
                        szLex,
                        i,
                    );
                    if nVars == 200 as libc::c_int {
                        ReportError(
                            pibIn,
                            (0x117 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"GetPrint\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                    hvar = GetVarHandle(szTmp.as_mut_ptr());
                    bErr = (hvar == 0) as libc::c_int;
                    if bErr != 0 {
                        ReportError(
                            pibIn,
                            (0x106 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            szTmp.as_mut_ptr(),
                            0 as PSTR,
                        );
                    } else {
                        pprintrec[nVars
                            as usize] = malloc(
                            ::std::mem::size_of::<PRINTREC>() as libc::c_ulong,
                        ) as PPRINTREC;
                        if (pprintrec[nVars as usize]).is_null()
                            || {
                                (*pprintrec[nVars as usize])
                                    .szOutputName = malloc(
                                    (strlen(szTmp.as_mut_ptr()))
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as PSTR;
                                ((*pprintrec[nVars as usize]).szOutputName).is_null()
                            }
                        {
                            ReportError(
                                pibIn,
                                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                b"GetPrint\0" as *const u8 as *const libc::c_char as PSTR,
                                0 as PSTR,
                            );
                        }
                        strcpy(
                            (*pprintrec[nVars as usize]).szOutputName,
                            szTmp.as_mut_ptr(),
                        );
                        (*pprintrec[nVars as usize]).hvar = hvar;
                        if !(pprintrec[nVars as usize]).is_null() {} else {
                            __assert_fail(
                                b"pprintrec[nVars]\0" as *const u8 as *const libc::c_char,
                                b"simi.c\0" as *const u8 as *const libc::c_char,
                                1000 as libc::c_int as libc::c_uint,
                                (*::std::mem::transmute::<
                                    &[u8; 45],
                                    &[libc::c_char; 45],
                                >(b"BOOL GetPrintStep(PINPUTBUF, PSTR, POUTSPEC)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_13815: {
                            if !(pprintrec[nVars as usize]).is_null() {} else {
                                __assert_fail(
                                    b"pprintrec[nVars]\0" as *const u8 as *const libc::c_char,
                                    b"simi.c\0" as *const u8 as *const libc::c_char,
                                    1000 as libc::c_int as libc::c_uint,
                                    (*::std::mem::transmute::<
                                        &[u8; 45],
                                        &[libc::c_char; 45],
                                    >(b"BOOL GetPrintStep(PINPUTBUF, PSTR, POUTSPEC)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                        nVars += 1;
                        nVars;
                    }
                    if i == iUB - 1 as libc::c_int as libc::c_long {
                        GetOptPunct(
                            pibIn,
                            szTmp.as_mut_ptr(),
                            ',' as i32 as libc::c_char,
                        );
                    }
                    i += 1;
                    i;
                }
            }
        }
        if nVars < 1 as libc::c_int {
            ReportError(
                pibIn,
                0x14 as libc::c_int as WORD,
                b"identifier\0" as *const u8 as *const libc::c_char as PSTR,
                szLex,
            );
            bErr = 1 as libc::c_int;
        } else {
            dStart = atof(szLex as *const libc::c_char);
            bOK = GetPunct(pibIn, szLex, ',' as i32 as libc::c_char);
            if bOK == 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b",\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            }
            bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
            if !(bErr != 0) {
                dEnd = atof(szLex as *const libc::c_char);
                bOK = GetPunct(pibIn, szLex, ',' as i32 as libc::c_char);
                if bOK == 0 {
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b",\0" as *const u8 as *const libc::c_char as PSTR,
                        szLex,
                    );
                }
                bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
                if !(bErr != 0) {
                    dStep = atof(szLex as *const libc::c_char);
                    bOK = GetPunct(pibIn, szLex, ')' as i32 as libc::c_char);
                    if bOK == 0 {
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b")\0" as *const u8 as *const libc::c_char as PSTR,
                            szLex,
                        );
                    }
                    bErr = (dEnd <= dStart) as libc::c_int;
                    if bErr != 0 {
                        ReportError(
                            pibIn,
                            0x205 as libc::c_int as WORD,
                            b"End_time must be > Start_time\0" as *const u8
                                as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    } else {
                        bErr = (dStep > dEnd - dStart) as libc::c_int;
                        if bErr != 0 {
                            ReportError(
                                pibIn,
                                0x205 as libc::c_int as WORD,
                                b"Time_step too large\0" as *const u8 as *const libc::c_char
                                    as PSTR,
                                0 as PSTR,
                            );
                        } else {
                            dTmp = 1 as libc::c_int as libc::c_double
                                + ceil((dEnd - dStart) / dStep);
                            n = 0 as libc::c_int;
                            while n < nVars {
                                if dTmp
                                    < 9223372036854775807 as libc::c_long as libc::c_double
                                {
                                    (*pprintrec[n as usize]).cTimes = dTmp as libc::c_long;
                                } else {
                                    (*pprintrec[n as usize])
                                        .cTimes = 9223372036854775807 as libc::c_long;
                                }
                                n += 1;
                                n;
                            }
                            n = 0 as libc::c_int;
                            while n < nVars {
                                (*pprintrec[n as usize])
                                    .pdTimes = InitdVector((*pprintrec[n as usize]).cTimes);
                                if ((*pprintrec[n as usize]).pdTimes).is_null() {
                                    ReportError(
                                        pibIn,
                                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                        b"GetPrintStep\0" as *const u8 as *const libc::c_char
                                            as PSTR,
                                        0 as PSTR,
                                    );
                                }
                                i = 0 as libc::c_int as libc::c_long;
                                while i
                                    < (*pprintrec[n as usize]).cTimes
                                        - 1 as libc::c_int as libc::c_long
                                {
                                    *((*pprintrec[n as usize]).pdTimes)
                                        .offset(i as isize) = dStart + i as libc::c_double * dStep;
                                    i += 1;
                                    i;
                                }
                                *((*pprintrec[n as usize]).pdTimes)
                                    .offset(
                                        ((*pprintrec[n as usize]).cTimes
                                            - 1 as libc::c_int as libc::c_long) as isize,
                                    ) = dEnd;
                                QueueListItem(
                                    (*pos).plistPrintRecs,
                                    pprintrec[n as usize] as PVOID,
                                );
                                n += 1;
                                n;
                            }
                        }
                    }
                }
            }
        }
    }
    if bErr != 0 {
        if bGavePrintStepUsage == 0 {
            printf(
                b"Syntax: %s (<Identifiers>, Start_time, End_time, Time_step)\n\n\0"
                    as *const u8 as *const libc::c_char,
                GetKeyword(6 as libc::c_int),
            );
            bGavePrintStepUsage = 1 as libc::c_int;
        }
    }
    return bErr;
}
pub static mut bGaveDataUsage: BOOL = 0 as libc::c_int;
pub unsafe extern "C" fn GetData(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pos: POUTSPEC,
) -> BOOL {
    let mut pdatarec: PDATAREC = 0 as *mut tagDATAREC;
    let mut bErr: BOOL = 0 as libc::c_int;
    let mut hvar: HVAR = 0;
    bErr = EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char);
    if bErr == 0 {
        bErr = ENextLex(pibIn, szLex, 0x1 as libc::c_int);
        if bErr == 0 {
            hvar = GetVarHandle(szLex);
            bErr = (hvar == 0) as libc::c_int;
            if bErr != 0 {
                ReportError(pibIn, 0x106 as libc::c_int as WORD, szLex, 0 as PSTR);
            } else {
                pdatarec = malloc(::std::mem::size_of::<DATAREC>() as libc::c_ulong)
                    as PDATAREC;
                if pdatarec.is_null() {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetData\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                (*pdatarec)
                    .szDataName = malloc(
                    (if !szLex.is_null() {
                        strlen(szLex as *const libc::c_char)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    })
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as PSTR;
                if ((*pdatarec).szDataName).is_null() {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetData\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                if !((*pdatarec).szDataName).is_null() && !szLex.is_null() {
                    strcpy((*pdatarec).szDataName, szLex as *const libc::c_char);
                } else {};
                if !pdatarec.is_null() {} else {
                    __assert_fail(
                        b"pdatarec\0" as *const u8 as *const libc::c_char,
                        b"simi.c\0" as *const u8 as *const libc::c_char,
                        1110 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 40],
                            &[libc::c_char; 40],
                        >(b"BOOL GetData(PINPUTBUF, PSTR, POUTSPEC)\0"))
                            .as_ptr(),
                    );
                }
                'c_6603: {
                    if !pdatarec.is_null() {} else {
                        __assert_fail(
                            b"pdatarec\0" as *const u8 as *const libc::c_char,
                            b"simi.c\0" as *const u8 as *const libc::c_char,
                            1110 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 40],
                                &[libc::c_char; 40],
                            >(b"BOOL GetData(PINPUTBUF, PSTR, POUTSPEC)\0"))
                                .as_ptr(),
                        );
                    }
                };
                (*pdatarec).hvar = hvar;
                bErr = GetListOfData(pibIn, pdatarec, szLex);
                if bErr != 0 {
                    free((*pdatarec).szDataName as *mut libc::c_void);
                    free(pdatarec as *mut libc::c_void);
                } else {
                    QueueListItem((*pos).plistDataRecs, pdatarec as PVOID);
                }
            }
        }
    }
    if bErr == 0 {
        bErr = GetTerminator(pibIn, szLex);
    } else if bGaveDataUsage == 0 {
        printf(
            b"Syntax: %s (identifier, Time1, Time2, ...)\n\n\0" as *const u8
                as *const libc::c_char,
            GetKeyword(7 as libc::c_int),
        );
        bGaveDataUsage = 1 as libc::c_int;
    }
    return bErr;
}
pub unsafe extern "C" fn GetStringArg(
    mut pibIn: PINPUTBUF,
    mut pszArg: *mut PSTR,
    mut szLex: PSTR,
    mut bDelim: BOOL,
) -> BOOL {
    let mut bErr: BOOL = 0;
    if !szLex.is_null() {} else {
        __assert_fail(
            b"szLex\0" as *const u8 as *const libc::c_char,
            b"simi.c\0" as *const u8 as *const libc::c_char,
            1155 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 49],
                &[libc::c_char; 49],
            >(b"BOOL GetStringArg(PINPUTBUF, PSTR *, PSTR, BOOL)\0"))
                .as_ptr(),
        );
    }
    'c_12131: {
        if !szLex.is_null() {} else {
            __assert_fail(
                b"szLex\0" as *const u8 as *const libc::c_char,
                b"simi.c\0" as *const u8 as *const libc::c_char,
                1155 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 49],
                    &[libc::c_char; 49],
                >(b"BOOL GetStringArg(PINPUTBUF, PSTR *, PSTR, BOOL)\0"))
                    .as_ptr(),
            );
        }
    };
    if bDelim != 0 {
        GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
    }
    bErr = ENextLex(pibIn, szLex, 0x10 as libc::c_int);
    if bErr == 0 {
        if *szLex.offset(0 as libc::c_int as isize) != 0 {
            *pszArg = malloc(
                (if !szLex.is_null() {
                    strlen(szLex as *const libc::c_char)
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as PSTR;
            if (*pszArg).is_null() {
                ReportError(
                    pibIn,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"GetStringArg\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
            if !(*pszArg).is_null() && !szLex.is_null() {
                strcpy(*pszArg, szLex as *const libc::c_char);
            } else {};
        } else {
            *pszArg = 0 as PSTR;
        }
    }
    return bErr;
}
pub unsafe extern "C" fn GetOutputFile(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut panal: PANALYSIS,
) -> BOOL {
    let mut bErr: BOOL = 0 as libc::c_int;
    bErr = (EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char) != 0
        || GetStringArg(pibIn, &mut (*panal).szOutfilename, szLex, 0 as libc::c_int)
            != 0) as libc::c_int;
    if bErr == 0 {
        (*panal).bAllocatedFileName = 1 as libc::c_int;
        bErr = EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char);
    }
    if bErr == 0 {
        bErr = GetTerminator(pibIn, szLex);
    } else {
        printf(
            b"Syntax: %s (szOutputFilename)\n\n\0" as *const u8 as *const libc::c_char,
            GetKeyword(14 as libc::c_int),
        );
    }
    return bErr;
}
pub static mut bGaveSimulateUsage: BOOL = 0 as libc::c_int;
pub unsafe extern "C" fn GetSimulate() -> BOOL {
    if bGaveSimulateUsage == 0 {
        printf(
            b"Warning: %s statements are obsolete and ignored.\n\n\0" as *const u8
                as *const libc::c_char,
            GetKeyword(2 as libc::c_int),
        );
        bGaveSimulateUsage = 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub static mut bGaveSrtTUsage: BOOL = 0 as libc::c_int;
pub unsafe extern "C" fn GetStartTime(
    mut pibIn: PINPUTBUF,
    mut pexp: PEXPERIMENT,
) -> BOOL {
    static mut vrgiSimArgTypes: [libc::c_int; 1] = [
        0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int,
    ];
    let mut bErr: BOOL = (GetFuncArgs(
        pibIn,
        1 as libc::c_int,
        vrgiSimArgTypes.as_mut_ptr(),
        (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
    ) == 0) as libc::c_int;
    if bErr == 0 {
        if DefDepParm(
            (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
            &mut (*pexp).dT0,
            &mut (*pexp).hT0,
        ) == 0
        {
            ReportError(
                pibIn,
                0x13 as libc::c_int as WORD,
                b"StartTime spec\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    } else if bGaveSrtTUsage == 0 {
        printf(
            b"Syntax: %s (InitialTime)\n\n\0" as *const u8 as *const libc::c_char,
            GetKeyword(3 as libc::c_int),
        );
        bGaveSrtTUsage = 1 as libc::c_int;
    }
    return bErr;
}
pub unsafe extern "C" fn GetMCMCSpec(
    mut pibIn: PINPUTBUF,
    mut pexp: PEXPERIMENT,
) -> BOOL {
    static mut vrgiGibbsArgTypes: [libc::c_int; 8] = [
        0x10 as libc::c_int,
        0x10 as libc::c_int,
        0x10 as libc::c_int,
        0x2 as libc::c_int,
        0x2 as libc::c_int,
        0x2 as libc::c_int,
        0x2 as libc::c_int,
        0x2 as libc::c_int | 0x4 as libc::c_int,
    ];
    let mut panal: PANALYSIS = (*pibIn).pInfo as PANALYSIS;
    let mut bErr: BOOL = (GetFuncArgs(
        pibIn,
        8 as libc::c_int,
        vrgiGibbsArgTypes.as_mut_ptr(),
        (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
    ) == 0) as libc::c_int;
    static mut vszGibbsOutDefault: [libc::c_char; 17] = unsafe {
        *::std::mem::transmute::<
            &[u8; 17],
            &mut [libc::c_char; 17],
        >(b"MCMC.default.out\0")
    };
    if bErr == 0 {
        if *(vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr() != 0 {
            (*panal)
                .gd
                .szGout = malloc(
                (if !(vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr().is_null() {
                    strlen((vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr())
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as PSTR;
            if ((*panal).gd.szGout).is_null() {
                ReportError(
                    pibIn,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"GetMCMCSpec\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
            if !((*panal).gd.szGout).is_null()
                && !(vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr().is_null()
            {
                strcpy(
                    (*panal).gd.szGout,
                    (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
                );
            } else {};
            (*panal).bAllocatedFileName = 1 as libc::c_int;
        } else {
            (*panal).gd.szGout = vszGibbsOutDefault.as_mut_ptr();
        }
        if *(vrgszlexArgs[1 as libc::c_int as usize]).as_mut_ptr() != 0 {
            (*panal)
                .gd
                .szGrestart = malloc(
                (if !(vrgszlexArgs[1 as libc::c_int as usize]).as_mut_ptr().is_null() {
                    strlen((vrgszlexArgs[1 as libc::c_int as usize]).as_mut_ptr())
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as PSTR;
            if ((*panal).gd.szGrestart).is_null() {
                ReportError(
                    pibIn,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"GetMCMCSpec\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
            if !((*panal).gd.szGrestart).is_null()
                && !(vrgszlexArgs[1 as libc::c_int as usize]).as_mut_ptr().is_null()
            {
                strcpy(
                    (*panal).gd.szGrestart,
                    (vrgszlexArgs[1 as libc::c_int as usize]).as_mut_ptr(),
                );
            } else {};
        }
        if !((*panal).gd.szGrestart).is_null()
            && strcmp(
                (*panal).gd.szGout as *const libc::c_char,
                (*panal).gd.szGrestart as *const libc::c_char,
            ) == 0
        {
            ReportError(
                pibIn,
                (0x210 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"GetMCMCSpec\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        if *(vrgszlexArgs[2 as libc::c_int as usize]).as_mut_ptr() != 0 {
            (*panal)
                .gd
                .szGdata = malloc(
                (if !(vrgszlexArgs[2 as libc::c_int as usize]).as_mut_ptr().is_null() {
                    strlen((vrgszlexArgs[2 as libc::c_int as usize]).as_mut_ptr())
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as PSTR;
            if ((*panal).gd.szGdata).is_null() {
                ReportError(
                    pibIn,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"GetMCMCSpec\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
            if !((*panal).gd.szGdata).is_null()
                && !(vrgszlexArgs[2 as libc::c_int as usize]).as_mut_ptr().is_null()
            {
                strcpy(
                    (*panal).gd.szGdata,
                    (vrgszlexArgs[2 as libc::c_int as usize]).as_mut_ptr(),
                );
            } else {};
        }
        (*panal)
            .gd
            .nMaxIter = atol((vrgszlexArgs[3 as libc::c_int as usize]).as_mut_ptr());
        (*panal)
            .gd
            .nSimTypeFlag = atol((vrgszlexArgs[4 as libc::c_int as usize]).as_mut_ptr());
        (*panal)
            .gd
            .nPrintFreq = atol((vrgszlexArgs[5 as libc::c_int as usize]).as_mut_ptr());
        (*panal)
            .gd
            .nPrintIter = atol((vrgszlexArgs[6 as libc::c_int as usize]).as_mut_ptr());
        (*panal).dSeed = atof((vrgszlexArgs[7 as libc::c_int as usize]).as_mut_ptr());
        if (*panal).gd.nSimTypeFlag == 1 as libc::c_int as libc::c_long
            && ((*panal).gd.szGrestart).is_null()
            || (*panal).gd.nSimTypeFlag == 2 as libc::c_int as libc::c_long
                && ((*panal).gd.szGrestart).is_null()
        {
            printf(
                b"Error: if simTypeFlag is one or two a restart file must be given - Exiting\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    } else {
        printf(
            b"Syntax:\n%s (szOut, szRestart, szData, nMaxIters, simTypeFlag, nPrintFreq,\n      nIterToPrint, dSeed)\nExiting.\n\n\0"
                as *const u8 as *const libc::c_char,
            GetKeyword(15 as libc::c_int),
        );
        exit(0 as libc::c_int);
    }
    if bErr == 0 {
        (*panal).iType = 4 as libc::c_int;
    }
    return (bErr == 0) as libc::c_int;
}
pub unsafe extern "C" fn GetOptDSpec(
    mut pibIn: PINPUTBUF,
    mut panal: PANALYSIS,
    mut szLex: PSTR,
) -> BOOL {
    let mut current_block: u64;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut hvar: HVAR = 0;
    let mut iErr: libc::c_int = 0 as libc::c_int;
    let mut iNLI: libc::c_int = 0;
    let mut ikwcode: libc::c_int = 0;
    iErr = (EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char) != 0
        || GetStringArg(pibIn, &mut (*panal).gd.szGout, szLex, 0 as libc::c_int) != 0
        || GetStringArg(pibIn, &mut (*panal).gd.szGrestart, szLex, 1 as libc::c_int)
            != 0) as libc::c_int;
    if !(iErr != 0) {
        (*panal).bAllocatedFileName = 1 as libc::c_int;
        if ((*panal).gd.szGrestart).is_null() {
            ReportError(
                pibIn,
                (0x205 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"Missing restart file\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
        iErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
        if !(iErr != 0) {
            (*panal).mc.nRuns = atol(szLex as *const libc::c_char);
            GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
            iErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
            if !(iErr != 0) {
                (*panal).dSeed = atof(szLex as *const libc::c_char);
                GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
                iErr = ENextLex(pibIn, szLex, 0x1 as libc::c_int);
                if !(iErr != 0) {
                    ikwcode = GetKeywordCode(szLex, 0 as PINT);
                    if ikwcode == 700 as libc::c_int {
                        (*panal).mc.style = forward as libc::c_int;
                        current_block = 15652330335145281839;
                    } else if ikwcode == 701 as libc::c_int {
                        (*panal).mc.style = backward as libc::c_int;
                        current_block = 15652330335145281839;
                    } else {
                        iErr = 1 as libc::c_int;
                        current_block = 1538046216550696469;
                    }
                    match current_block {
                        1538046216550696469 => {}
                        _ => {
                            loop {
                                iNLI = NextListItem(
                                    pibIn,
                                    szLex,
                                    0x1 as libc::c_int,
                                    1 as libc::c_int,
                                    ')' as i32 as libc::c_char,
                                );
                                if !(iNLI > 0 as libc::c_int) {
                                    break;
                                }
                                hvar = GetVarHandle(szLex);
                                iErr = (hvar == 0 || IsInput(hvar) != 0) as libc::c_int;
                                if iErr != 0 {
                                    break;
                                }
                                pMCVar = malloc(
                                    ::std::mem::size_of::<MCVAR>() as libc::c_ulong,
                                ) as PMCVAR;
                                if pMCVar.is_null() {
                                    ReportError(
                                        pibIn,
                                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                        b"GetOptDSpec\0" as *const u8 as *const libc::c_char
                                            as PSTR,
                                        0 as PSTR,
                                    );
                                }
                                (*pMCVar).hvar = hvar;
                                (*pMCVar).iType = -(1 as libc::c_int);
                                (*pMCVar).dParm[3 as libc::c_int as usize] = 0.0f64;
                                (*pMCVar)
                                    .dParm[2 as libc::c_int
                                    as usize] = (*pMCVar).dParm[3 as libc::c_int as usize];
                                QueueListItem((*panal).mc.plistMCVars, pMCVar as PVOID);
                            }
                            (*panal)
                                .mc
                                .nSetParms = (if !((*panal).mc.plistMCVars).is_null() {
                                (*(*panal).mc.plistMCVars).iSize
                            } else {
                                0 as libc::c_int
                            }) as libc::c_long;
                            if (*panal).mc.nSetParms == 0 as libc::c_int as libc::c_long
                            {
                                iErr = 1 as libc::c_int;
                                printf(
                                    b"\nError: you must specify a list of parameters to read.\n\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            } else if iNLI == 0 {
                                iErr = EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char);
                            } else {
                                iErr = 1 as libc::c_int;
                                ReportError(
                                    pibIn,
                                    0x14 as libc::c_int as WORD,
                                    b"identifier\0" as *const u8 as *const libc::c_char as PSTR,
                                    szLex,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    if iErr != 0 {
        printf(
            b"Syntax:\n%s (\"Output_File\", \"Param_Sample_File\", nSamples, random_seed, <Forward or Backward>, <param-id-list...>)\n\n\0"
                as *const u8 as *const libc::c_char,
            GetKeyword(17 as libc::c_int),
        );
        printf(b"Exiting...\n\0" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    } else {
        (*panal).iType = 5 as libc::c_int;
    }
    return iErr;
}
pub static mut bGaveMCVaryUsage: BOOL = 0 as libc::c_int;
pub unsafe extern "C" fn GetDistribSpec(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut panal: PANALYSIS,
) -> libc::c_int {
    let mut current_block: u64;
    let mut plist: PLIST = 0 as *mut tagLIST;
    let mut pMCVar: PMCVAR = 0 as PMCVAR;
    let mut hvar: HVAR = 0;
    let mut n: libc::c_int = 0;
    let mut iErr: libc::c_int = 0 as libc::c_int;
    let mut szDummy: PSTRLEX_ = [0; 255];
    if (*panal).iType != 0
        && !((*panal).iType == 2 as libc::c_int || (*panal).iType == 3 as libc::c_int
            || (*panal).iType == 5 as libc::c_int || (*panal).iType == 4 as libc::c_int)
    {
        EatStatement(pibIn);
    } else {
        iErr = (EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char) != 0
            || ENextLex(pibIn, szLex, 0x1 as libc::c_int) != 0) as libc::c_int;
        if !(iErr != 0) {
            if GetKeywordCode(szLex, 0 as PINT) == 7 as libc::c_int {
                if EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char) != 0 {
                    exit(0 as libc::c_int);
                }
                ENextLex(pibIn, szLex, 0x1 as libc::c_int);
                hvar = GetVarHandle(szLex);
                if hvar == 0 || IsParm(hvar) != 0 {
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"input, output or state variable\0" as *const u8
                            as *const libc::c_char as PSTR,
                        szLex,
                    );
                }
                if EGetPunct(pibIn, szDummy.as_mut_ptr(), ')' as i32 as libc::c_char)
                    != 0
                {
                    exit(0 as libc::c_int);
                }
                current_block = 10599921512955367680;
            } else {
                hvar = GetVarHandle(szLex);
                iErr = (hvar == 0 || IsInput(hvar) != 0) as libc::c_int;
                if iErr != 0 {
                    ReportError(
                        pibIn,
                        0x14 as libc::c_int as WORD,
                        b"state, output or parameter\0" as *const u8
                            as *const libc::c_char as PSTR,
                        szLex,
                    );
                    current_block = 18201902862271706575;
                } else {
                    current_block = 10599921512955367680;
                }
            }
            match current_block {
                18201902862271706575 => {}
                _ => {
                    if (*panal).iCurrentDepth == 0 as libc::c_int {
                        plist = (*panal).mc.plistMCVars;
                    } else if IsParm(hvar) == 0 {
                        plist = (*(*panal)
                            .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int)
                            as usize])
                            .plistLikes;
                    } else {
                        plist = (*(*panal)
                            .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int)
                            as usize])
                            .plistMCVars;
                    }
                    pMCVar = malloc(::std::mem::size_of::<MCVAR>() as libc::c_ulong)
                        as PMCVAR;
                    if pMCVar.is_null() {
                        ReportError(
                            pibIn,
                            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"GetDistribSpec\0" as *const u8 as *const libc::c_char
                                as PSTR,
                            0 as PSTR,
                        );
                    }
                    (*pMCVar)
                        .pszName = malloc(
                        (strlen(szLex as *const libc::c_char))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as PSTR;
                    if ((*pMCVar).pszName).is_null() {
                        ReportError(
                            pibIn,
                            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"GetDistribSpec\0" as *const u8 as *const libc::c_char
                                as PSTR,
                            0 as PSTR,
                        );
                    }
                    strcpy((*pMCVar).pszName, szLex as *const libc::c_char);
                    (*pMCVar).hvar = hvar;
                    (*pMCVar).pdVal = &mut (*pMCVar).dVal;
                    (*pMCVar).iDepth = (*panal).iCurrentDepth - 1 as libc::c_int;
                    (*pMCVar).plistDependents = InitList();
                    (*pMCVar).bIsFixed = 0 as libc::c_int;
                    (*pMCVar).bExptIsDep = (*pMCVar).bIsFixed;
                    (*pMCVar).lCount = 0 as libc::c_int as libc::c_long;
                    (*pMCVar).lJumps = (*pMCVar).lCount;
                    (*pMCVar).dKernelSD = 1.0f64;
                    (*pMCVar).bGibbs = 0 as libc::c_int;
                    n = 0 as libc::c_int;
                    while n < 4 as libc::c_int {
                        (*pMCVar).hParm[n as usize] = 0 as libc::c_int as HVAR;
                        (*pMCVar).pMCVParent[n as usize] = 0 as *mut tagMCVAR;
                        (*pMCVar)
                            .pdParm[n
                            as usize] = &mut *((*pMCVar).dParm)
                            .as_mut_ptr()
                            .offset(n as isize) as *mut libc::c_double;
                        (*pMCVar).iParmType[n as usize] = 0 as libc::c_int;
                        n += 1;
                        n;
                    }
                    GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
                    iErr |= ENextLex(pibIn, szLex, 0x1 as libc::c_int);
                    (*pMCVar).iType = McvFromLex(szLex);
                    iErr |= ((*pMCVar).iType < 0 as libc::c_int) as libc::c_int;
                    if iErr != 0 {
                        ReportError(
                            pibIn,
                            0x14 as libc::c_int as WORD,
                            b"distribution-type\0" as *const u8 as *const libc::c_char
                                as PSTR,
                            szLex,
                        );
                    } else {
                        match (*pMCVar).iType {
                            0 | 1 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    iErr = GetDistribParam(
                                        pibIn,
                                        szLex,
                                        plist,
                                        1 as libc::c_int,
                                        pMCVar,
                                    );
                                    if iErr != 0 {
                                        current_block = 18201902862271706575;
                                    } else {
                                        (*pMCVar)
                                            .dParm[2 as libc::c_int
                                            as usize] = -1.7976931348623157e+308f64 * 0.5f64;
                                        (*pMCVar)
                                            .dParm[3 as libc::c_int
                                            as usize] = 1.7976931348623157e+308f64 * 0.5f64;
                                        current_block = 1425453989644512380;
                                    }
                                }
                            }
                            3 => {
                                current_block = 473344465769554418;
                            }
                            4 => {
                                current_block = 473344465769554418;
                            }
                            25 => {
                                current_block = 9229921293278419701;
                            }
                            14 => {
                                current_block = 7059540211943029960;
                            }
                            15 => {
                                current_block = 5458555550210122808;
                            }
                            19 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    1 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    (*pMCVar).dParm[0 as libc::c_int as usize] = 0.0f64;
                                    (*pMCVar).dParm[2 as libc::c_int as usize] = 0.0f64;
                                    (*pMCVar)
                                        .dParm[3 as libc::c_int
                                        as usize] = 1.7976931348623157e+308f64;
                                    current_block = 1425453989644512380;
                                }
                            }
                            2 => {
                                current_block = 426108667778819661;
                            }
                            5 => {
                                current_block = 426108667778819661;
                            }
                            6 => {
                                current_block = 10265717308518604954;
                            }
                            26 => {
                                current_block = 16137297766726163412;
                            }
                            16 => {
                                current_block = 8283066247764068699;
                            }
                            17 => {
                                current_block = 14843621488218680859;
                            }
                            7 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    (*pMCVar).dParm[2 as libc::c_int as usize] = 0.0f64;
                                    (*pMCVar)
                                        .dParm[3 as libc::c_int
                                        as usize] = 1.7976931348623157e+308f64;
                                    current_block = 1425453989644512380;
                                }
                            }
                            8 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    iErr = GetDistribParam(
                                        pibIn,
                                        szLex,
                                        plist,
                                        1 as libc::c_int,
                                        pMCVar,
                                    );
                                    if iErr != 0 {
                                        current_block = 18201902862271706575;
                                    } else {
                                        (*pMCVar).dParm[2 as libc::c_int as usize] = 0.0f64;
                                        if (*pMCVar).iParmType[1 as libc::c_int as usize]
                                            != 0 as libc::c_int
                                        {
                                            (*pMCVar)
                                                .dParm[3 as libc::c_int
                                                as usize] = 1.7976931348623157e+308f64;
                                        } else {
                                            (*pMCVar)
                                                .dParm[3 as libc::c_int
                                                as usize] = (*pMCVar).dParm[1 as libc::c_int as usize];
                                        }
                                        current_block = 1425453989644512380;
                                    }
                                }
                            }
                            28 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    iErr = GetDistribParam(
                                        pibIn,
                                        szLex,
                                        plist,
                                        1 as libc::c_int,
                                        pMCVar,
                                    );
                                    if iErr != 0 {
                                        current_block = 18201902862271706575;
                                    } else {
                                        (*pMCVar).dParm[2 as libc::c_int as usize] = 0.0f64;
                                        (*pMCVar)
                                            .dParm[3 as libc::c_int
                                            as usize] = 1.7976931348623157e+308f64;
                                        current_block = 1425453989644512380;
                                    }
                                }
                            }
                            9 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    2 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    iErr = GetDistribParam(
                                        pibIn,
                                        szLex,
                                        plist,
                                        0 as libc::c_int,
                                        pMCVar,
                                    );
                                    if iErr != 0 {
                                        current_block = 18201902862271706575;
                                    } else {
                                        iErr = GetDistribParam(
                                            pibIn,
                                            szLex,
                                            plist,
                                            1 as libc::c_int,
                                            pMCVar,
                                        );
                                        if iErr != 0 {
                                            current_block = 18201902862271706575;
                                        } else {
                                            iErr = GetDistribParam(
                                                pibIn,
                                                szLex,
                                                plist,
                                                3 as libc::c_int,
                                                pMCVar,
                                            );
                                            if iErr != 0 {
                                                current_block = 18201902862271706575;
                                            } else {
                                                current_block = 1425453989644512380;
                                            }
                                        }
                                    }
                                }
                            }
                            10 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    (*pMCVar).dParm[2 as libc::c_int as usize] = 0.0f64;
                                    (*pMCVar)
                                        .dParm[3 as libc::c_int
                                        as usize] = 1.7976931348623157e+308f64;
                                    current_block = 1425453989644512380;
                                }
                            }
                            11 | 13 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    iErr = GetDistribParam(
                                        pibIn,
                                        szLex,
                                        plist,
                                        1 as libc::c_int,
                                        pMCVar,
                                    );
                                    if iErr != 0 {
                                        current_block = 18201902862271706575;
                                    } else {
                                        (*pMCVar).dParm[2 as libc::c_int as usize] = 0.0f64;
                                        (*pMCVar)
                                            .dParm[3 as libc::c_int
                                            as usize] = 1.7976931348623157e+308f64;
                                        current_block = 1425453989644512380;
                                    }
                                }
                            }
                            20 => {
                                printf(
                                    b"Warning: The truncated inverse gamma density cannot be\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                printf(
                                    b"         used in MCMC simulations if the GNU Scientific\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                printf(
                                    b"         Library is not installed.\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    iErr = GetDistribParam(
                                        pibIn,
                                        szLex,
                                        plist,
                                        1 as libc::c_int,
                                        pMCVar,
                                    );
                                    if iErr != 0 {
                                        current_block = 18201902862271706575;
                                    } else {
                                        iErr = GetDistribParam(
                                            pibIn,
                                            szLex,
                                            plist,
                                            2 as libc::c_int,
                                            pMCVar,
                                        );
                                        if iErr != 0 {
                                            current_block = 18201902862271706575;
                                        } else {
                                            iErr = GetDistribParam(
                                                pibIn,
                                                szLex,
                                                plist,
                                                3 as libc::c_int,
                                                pMCVar,
                                            );
                                            if iErr != 0 {
                                                current_block = 18201902862271706575;
                                            } else {
                                                current_block = 1425453989644512380;
                                            }
                                        }
                                    }
                                }
                            }
                            12 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    (*pMCVar).dParm[2 as libc::c_int as usize] = 0.0f64;
                                    (*pMCVar)
                                        .dParm[3 as libc::c_int
                                        as usize] = 1.7976931348623157e+308f64;
                                    current_block = 1425453989644512380;
                                }
                            }
                            18 => {
                                current_block = 18146812617755870114;
                            }
                            21 => {
                                current_block = 18146812617755870114;
                            }
                            22 => {
                                current_block = 15799049419295925500;
                            }
                            23 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    (*pMCVar)
                                        .dParm[1 as libc::c_int
                                        as usize] = 1.7976931348623157e+308f64;
                                    (*pMCVar)
                                        .dParm[2 as libc::c_int
                                        as usize] = -1.7976931348623157e+308f64 * 0.5f64;
                                    (*pMCVar)
                                        .dParm[3 as libc::c_int
                                        as usize] = 1.7976931348623157e+308f64 * 0.5f64;
                                    current_block = 1425453989644512380;
                                }
                            }
                            24 => {
                                iErr = GetDistribParam(
                                    pibIn,
                                    szLex,
                                    plist,
                                    0 as libc::c_int,
                                    pMCVar,
                                );
                                if iErr != 0 {
                                    current_block = 18201902862271706575;
                                } else {
                                    (*pMCVar)
                                        .dParm[1 as libc::c_int
                                        as usize] = 1.7976931348623157e+308f64 * 0.5f64;
                                    (*pMCVar)
                                        .dParm[2 as libc::c_int
                                        as usize] = 0 as libc::c_int as libc::c_double;
                                    (*pMCVar)
                                        .dParm[3 as libc::c_int
                                        as usize] = 1.7976931348623157e+308f64;
                                    current_block = 1425453989644512380;
                                }
                            }
                            27 => {
                                if ((*panal)
                                    .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int)
                                    as usize])
                                    .is_null()
                                    || plist
                                        != (*(*panal)
                                            .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int)
                                            as usize])
                                            .plistLikes
                                {
                                    printf(
                                        b"UserSpecifiefLL can only be used in Likelihood().\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    iErr = 1 as libc::c_int;
                                    current_block = 18201902862271706575;
                                } else {
                                    iErr = GetDistribParam(
                                        pibIn,
                                        szLex,
                                        plist,
                                        0 as libc::c_int,
                                        pMCVar,
                                    );
                                    if iErr != 0 {
                                        current_block = 18201902862271706575;
                                    } else {
                                        (*pMCVar)
                                            .dParm[1 as libc::c_int
                                            as usize] = 1.7976931348623157e+308f64 * 0.5f64;
                                        (*pMCVar)
                                            .dParm[2 as libc::c_int
                                            as usize] = 0 as libc::c_int as libc::c_double;
                                        (*pMCVar)
                                            .dParm[3 as libc::c_int
                                            as usize] = 1.7976931348623157e+308f64;
                                        current_block = 1425453989644512380;
                                    }
                                }
                            }
                            _ => {
                                ReportRunTimeError(
                                    panal,
                                    (0x305 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                    b"GetDistribSpec\0" as *const u8 as *const libc::c_char,
                                );
                                current_block = 1425453989644512380;
                            }
                        }
                        match current_block {
                            18201902862271706575 => {}
                            _ => {
                                match current_block {
                                    473344465769554418 => {
                                        current_block = 9229921293278419701;
                                    }
                                    426108667778819661 => {
                                        current_block = 10265717308518604954;
                                    }
                                    18146812617755870114 => {
                                        current_block = 15799049419295925500;
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    15799049419295925500 => {
                                        iErr = GetDistribParam(
                                            pibIn,
                                            szLex,
                                            plist,
                                            0 as libc::c_int,
                                            pMCVar,
                                        );
                                        if iErr != 0 {
                                            current_block = 18201902862271706575;
                                        } else {
                                            iErr = GetDistribParam(
                                                pibIn,
                                                szLex,
                                                plist,
                                                1 as libc::c_int,
                                                pMCVar,
                                            );
                                            if iErr != 0 {
                                                current_block = 18201902862271706575;
                                            } else {
                                                iErr = GetDistribParam(
                                                    pibIn,
                                                    szLex,
                                                    plist,
                                                    2 as libc::c_int,
                                                    pMCVar,
                                                );
                                                if iErr != 0 {
                                                    current_block = 18201902862271706575;
                                                } else {
                                                    (*pMCVar)
                                                        .dParm[3 as libc::c_int
                                                        as usize] = 1.7976931348623157e+308f64;
                                                    current_block = 1425453989644512380;
                                                }
                                            }
                                        }
                                    }
                                    9229921293278419701 => {
                                        current_block = 7059540211943029960;
                                    }
                                    10265717308518604954 => {
                                        current_block = 16137297766726163412;
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    18201902862271706575 => {}
                                    _ => {
                                        match current_block {
                                            7059540211943029960 => {
                                                current_block = 5458555550210122808;
                                            }
                                            16137297766726163412 => {
                                                current_block = 8283066247764068699;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            5458555550210122808 => {
                                                iErr = GetDistribParam(
                                                    pibIn,
                                                    szLex,
                                                    plist,
                                                    0 as libc::c_int,
                                                    pMCVar,
                                                );
                                                if iErr != 0 {
                                                    current_block = 18201902862271706575;
                                                } else {
                                                    iErr = GetDistribParam(
                                                        pibIn,
                                                        szLex,
                                                        plist,
                                                        1 as libc::c_int,
                                                        pMCVar,
                                                    );
                                                    if iErr != 0 {
                                                        current_block = 18201902862271706575;
                                                    } else {
                                                        if (*pMCVar).iType == 3 as libc::c_int
                                                            || (*pMCVar).iType == 25 as libc::c_int
                                                            || (*pMCVar).iType == 14 as libc::c_int
                                                        {
                                                            (*pMCVar)
                                                                .dParm[2 as libc::c_int
                                                                as usize] = -1.7976931348623157e+308f64 * 0.5f64;
                                                            (*pMCVar)
                                                                .dParm[3 as libc::c_int
                                                                as usize] = 1.7976931348623157e+308f64 * 0.5f64;
                                                        } else {
                                                            (*pMCVar).dParm[2 as libc::c_int as usize] = 0.0f64;
                                                            (*pMCVar)
                                                                .dParm[3 as libc::c_int
                                                                as usize] = 1.7976931348623157e+308f64;
                                                        }
                                                        current_block = 1425453989644512380;
                                                    }
                                                }
                                            }
                                            8283066247764068699 => {
                                                current_block = 14843621488218680859;
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            18201902862271706575 => {}
                                            _ => {
                                                match current_block {
                                                    14843621488218680859 => {
                                                        iErr = GetDistribParam(
                                                            pibIn,
                                                            szLex,
                                                            plist,
                                                            0 as libc::c_int,
                                                            pMCVar,
                                                        );
                                                        if iErr != 0 {
                                                            current_block = 18201902862271706575;
                                                        } else {
                                                            iErr = GetDistribParam(
                                                                pibIn,
                                                                szLex,
                                                                plist,
                                                                1 as libc::c_int,
                                                                pMCVar,
                                                            );
                                                            if iErr != 0 {
                                                                current_block = 18201902862271706575;
                                                            } else {
                                                                (*pMCVar).dParm[2 as libc::c_int as usize] = 0.0f64;
                                                                (*pMCVar).dParm[3 as libc::c_int as usize] = 1.0f64;
                                                                if (*pMCVar).iType == 5 as libc::c_int
                                                                    || (*pMCVar).iType == 26 as libc::c_int
                                                                    || (*pMCVar).iType == 16 as libc::c_int
                                                                {
                                                                    (*pMCVar)
                                                                        .dParm[2 as libc::c_int
                                                                        as usize] = -1.7976931348623157e+308f64 * 0.5f64;
                                                                    (*pMCVar)
                                                                        .dParm[3 as libc::c_int
                                                                        as usize] = 1.7976931348623157e+308f64 * 0.5f64;
                                                                } else if (*pMCVar).iType == 6 as libc::c_int
                                                                    || (*pMCVar).iType == 17 as libc::c_int
                                                                {
                                                                    (*pMCVar)
                                                                        .dParm[3 as libc::c_int
                                                                        as usize] = 1.7976931348623157e+308f64;
                                                                }
                                                                SkipWhitespace(pibIn);
                                                                if (*pMCVar).iType == 2 as libc::c_int
                                                                    && NextChar(pibIn) as libc::c_int == ')' as i32
                                                                {
                                                                    current_block = 1425453989644512380;
                                                                } else {
                                                                    iErr = GetDistribParam(
                                                                        pibIn,
                                                                        szLex,
                                                                        plist,
                                                                        2 as libc::c_int,
                                                                        pMCVar,
                                                                    );
                                                                    if iErr != 0 {
                                                                        current_block = 18201902862271706575;
                                                                    } else {
                                                                        iErr = GetDistribParam(
                                                                            pibIn,
                                                                            szLex,
                                                                            plist,
                                                                            3 as libc::c_int,
                                                                            pMCVar,
                                                                        );
                                                                        if iErr != 0 {
                                                                            current_block = 18201902862271706575;
                                                                        } else {
                                                                            current_block = 1425453989644512380;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    18201902862271706575 => {}
                                                    _ => {
                                                        EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char);
                                                        if (*pMCVar).iParmType[2 as libc::c_int as usize]
                                                            == 0 as libc::c_int
                                                            && (*pMCVar).iParmType[3 as libc::c_int as usize]
                                                                == 0 as libc::c_int
                                                            && (*pMCVar).dParm[3 as libc::c_int as usize]
                                                                < (*pMCVar).dParm[2 as libc::c_int as usize]
                                                        {
                                                            let mut dTmp: libc::c_double = (*pMCVar)
                                                                .dParm[3 as libc::c_int as usize];
                                                            (*pMCVar)
                                                                .dParm[3 as libc::c_int
                                                                as usize] = (*pMCVar).dParm[2 as libc::c_int as usize];
                                                            (*pMCVar).dParm[2 as libc::c_int as usize] = dTmp;
                                                            ReportError(
                                                                pibIn,
                                                                (0x209 as libc::c_int | 0x4000 as libc::c_int) as WORD,
                                                                0 as PSTR,
                                                                0 as PSTR,
                                                            );
                                                        }
                                                        if iErr == 0 {
                                                            QueueListItem(plist, pMCVar as PVOID);
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
            }
        }
        if iErr != 0 {
            if !pMCVar.is_null() {
                free(pMCVar as *mut libc::c_void);
            }
            if bGaveMCVaryUsage == 0 {
                printf(
                    b"\nSyntax: Check the syntax of %s.\n\0" as *const u8
                        as *const libc::c_char,
                    GetKeyword(12 as libc::c_int),
                );
                bGaveMCVaryUsage = 1 as libc::c_int;
            }
            ReportError(
                pibIn,
                (0x15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                0 as PSTR,
                0 as PSTR,
            );
        }
    }
    return iErr;
}
pub unsafe extern "C" fn CheckDistribParam(
    mut plist: PLIST,
    mut hvar1: HVAR,
    mut hvar2: HVAR,
) -> BOOL {
    let mut n: libc::c_int = 0;
    let mut p: PLISTELEM = (*plist).pleHead;
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    if plist.is_null() {
        return 1 as libc::c_int;
    }
    n = 0 as libc::c_int;
    while n < (*plist).iSize {
        pMCVar = (*p).pData as PMCVAR;
        if hvar2 == (*pMCVar).hvar {
            if (*pMCVar).iParmType[0 as libc::c_int as usize] == 1 as libc::c_int
                && hvar1 == (*pMCVar).hParm[0 as libc::c_int as usize]
            {
                return 0 as libc::c_int;
            }
            if (*pMCVar).iParmType[1 as libc::c_int as usize] == 1 as libc::c_int
                && hvar1 == (*pMCVar).hParm[1 as libc::c_int as usize]
            {
                return 0 as libc::c_int;
            }
            if (*pMCVar).iParmType[2 as libc::c_int as usize] == 1 as libc::c_int
                && hvar1 == (*pMCVar).hParm[2 as libc::c_int as usize]
            {
                return 0 as libc::c_int;
            }
            if (*pMCVar).iParmType[3 as libc::c_int as usize] == 1 as libc::c_int
                && hvar1 == (*pMCVar).hParm[3 as libc::c_int as usize]
            {
                return 0 as libc::c_int;
            }
        }
        p = (*p).pleNext;
        n += 1;
        n;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn GetDistribParam(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut plist: PLIST,
    mut n: libc::c_int,
    mut pMCVar: PMCVAR,
) -> libc::c_int {
    let mut panal: PANALYSIS = (*pibIn).pInfo as PANALYSIS;
    let mut iLex: libc::c_int = 0;
    let mut iCode: libc::c_int = 0;
    let mut hvar: HVAR = 0;
    GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
    if n != 3 as libc::c_int {
        NextLex(pibIn, szLex, &mut iLex);
    } else {
        SkipWhitespace(pibIn);
        iLex = 0 as libc::c_int;
        if NextChar(pibIn) as libc::c_int != ')' as i32 {
            NextLex(pibIn, szLex, &mut iLex);
        }
    }
    if iLex == 0x1 as libc::c_int {
        iCode = GetKeywordCode_in_context(szLex, 0x100 as libc::c_int as WORD);
        if iCode == 300 as libc::c_int || iCode == 7 as libc::c_int {
            if IsParm((*pMCVar).hvar) != 0 {
                ReportError(
                    pibIn,
                    (0x101 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    szLex,
                    0 as PSTR,
                );
            }
            if EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char) != 0 {
                return 1 as libc::c_int;
            }
            NextLex(pibIn, szLex, &mut iLex);
            hvar = GetVarHandle(szLex);
            if hvar == 0 || IsParm(hvar) != 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"input, output or state variable\0" as *const u8
                        as *const libc::c_char as PSTR,
                    szLex,
                );
            }
            if EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char) != 0 {
                return 1 as libc::c_int;
            }
        } else {
            hvar = GetVarHandle(szLex);
            if hvar == 0 || IsParm(hvar) == 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"parameter\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            }
        }
        if !((*panal).iType == 5 as libc::c_int)
            && ((*panal).iCurrentDepth == 0 as libc::c_int && hvar == (*pMCVar).hvar
                || CheckDistribParam(plist, (*pMCVar).hvar, hvar) == 0)
        {
            ReportError(
                pibIn,
                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"valid parameter\0" as *const u8 as *const libc::c_char as PSTR,
                szLex,
            );
        }
        if iCode == 300 as libc::c_int {
            (*pMCVar).iParmType[n as usize] = 2 as libc::c_int;
        } else if iCode == 7 as libc::c_int {
            (*pMCVar).iParmType[n as usize] = 3 as libc::c_int;
        } else {
            (*pMCVar).iParmType[n as usize] = 1 as libc::c_int;
        }
        (*pMCVar).hParm[n as usize] = hvar;
    } else if iLex == 0x4 as libc::c_int || iLex == 0x2 as libc::c_int {
        (*pMCVar).iParmType[n as usize] = 0 as libc::c_int;
        (*pMCVar).dParm[n as usize] = atof(szLex as *const libc::c_char);
    } else if n == 3 as libc::c_int {
        (*pMCVar).iParmType[n as usize] = 0 as libc::c_int;
        (*pMCVar).dParm[n as usize] = 1.7976931348623157e+308f64;
    } else {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn GetSetPointsSpec(
    mut pibIn: PINPUTBUF,
    mut panal: PANALYSIS,
    mut szLex: PSTR,
) -> libc::c_int {
    let mut pMCVar: PMCVAR = 0 as *mut tagMCVAR;
    let mut szTmp: PSTRLEX_ = [0; 255];
    let mut hvar: HVAR = 0;
    let mut iErr: libc::c_int = 0 as libc::c_int;
    let mut iNLI: libc::c_int = 0;
    let mut j: libc::c_long = 0;
    let mut iLB: libc::c_long = 0;
    let mut iUB: libc::c_long = 0;
    if (if !((*panal).mc.plistMCVars).is_null() {
        (*(*panal).mc.plistMCVars).iSize
    } else {
        0 as libc::c_int
    }) > 0 as libc::c_int
    {
        printf(
            b"Error: Distrib() statements can only appear after the SetPoints()specification, not before - Exiting\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    iErr = (EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char) != 0
        || GetStringArg(pibIn, &mut (*panal).mc.szMCOutfilename, szLex, 0 as libc::c_int)
            != 0
        || GetStringArg(
            pibIn,
            &mut (*panal).mc.szSetPointsFilename,
            szLex,
            1 as libc::c_int,
        ) != 0) as libc::c_int;
    if !(iErr != 0) {
        (*panal).bAllocatedFileName = 1 as libc::c_int;
        if ((*panal).mc.szSetPointsFilename).is_null() {
            ReportError(
                pibIn,
                (0x205 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"Missing setpoints file\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        if MyStrcmp(
            (*panal).mc.szMCOutfilename as *const libc::c_char,
            (*panal).mc.szSetPointsFilename as *const libc::c_char,
        ) == 0
        {
            ReportError(
                pibIn,
                (0x205 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"Same name for 2 files\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
        iErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
        if !(iErr != 0) {
            (*panal).mc.nRuns = atol(szLex as *const libc::c_char);
            loop {
                iNLI = NextListItem(
                    pibIn,
                    szLex,
                    0x1 as libc::c_int,
                    1 as libc::c_int,
                    ')' as i32 as libc::c_char,
                );
                if !(iNLI > 0 as libc::c_int) {
                    break;
                }
                iUB = -(1 as libc::c_int) as libc::c_long;
                iLB = iUB;
                if GetPunct(pibIn, szTmp.as_mut_ptr(), '[' as i32 as libc::c_char) != 0 {
                    GetArrayBounds(pibIn, &mut iLB, &mut iUB);
                }
                if iUB == -(1 as libc::c_int) as libc::c_long {
                    hvar = GetVarHandle(szLex);
                    iErr = (hvar == 0 || IsInput(hvar) != 0) as libc::c_int;
                    if iErr != 0 {
                        break;
                    }
                    pMCVar = malloc(::std::mem::size_of::<MCVAR>() as libc::c_ulong)
                        as PMCVAR;
                    if pMCVar.is_null() {
                        ReportError(
                            pibIn,
                            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"GetSetPointsSpec\0" as *const u8 as *const libc::c_char
                                as PSTR,
                            0 as PSTR,
                        );
                    }
                    (*pMCVar).hvar = hvar;
                    (*pMCVar).iType = -(1 as libc::c_int);
                    (*pMCVar).dParm[3 as libc::c_int as usize] = 0.0f64;
                    (*pMCVar)
                        .dParm[2 as libc::c_int
                        as usize] = (*pMCVar).dParm[3 as libc::c_int as usize];
                    QueueListItem((*panal).mc.plistMCVars, pMCVar as PVOID);
                } else {
                    j = iLB;
                    while j < iUB {
                        sprintf(
                            szTmp.as_mut_ptr(),
                            b"%s_%ld\0" as *const u8 as *const libc::c_char,
                            szLex,
                            j,
                        );
                        hvar = GetVarHandle(szTmp.as_mut_ptr());
                        iErr = (hvar == 0 || IsInput(hvar) != 0) as libc::c_int;
                        if iErr != 0 {
                            break;
                        }
                        pMCVar = malloc(::std::mem::size_of::<MCVAR>() as libc::c_ulong)
                            as PMCVAR;
                        if pMCVar.is_null() {
                            ReportError(
                                pibIn,
                                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                b"GetSetPointsSpec\0" as *const u8 as *const libc::c_char
                                    as PSTR,
                                0 as PSTR,
                            );
                        }
                        (*pMCVar).hvar = hvar;
                        (*pMCVar).iType = -(1 as libc::c_int);
                        (*pMCVar).dParm[3 as libc::c_int as usize] = 0.0f64;
                        (*pMCVar)
                            .dParm[2 as libc::c_int
                            as usize] = (*pMCVar).dParm[3 as libc::c_int as usize];
                        QueueListItem((*panal).mc.plistMCVars, pMCVar as PVOID);
                        j += 1;
                        j;
                    }
                }
            }
            (*panal)
                .mc
                .nSetParms = (if !((*panal).mc.plistMCVars).is_null() {
                (*(*panal).mc.plistMCVars).iSize
            } else {
                0 as libc::c_int
            }) as libc::c_long;
            if (*panal).mc.nSetParms == 0 as libc::c_int as libc::c_long {
                iErr = 1 as libc::c_int;
                printf(
                    b"\nError: you must specify a list of parameters to read.\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
            } else if iNLI == 0 {
                iErr = (szTmp[0 as libc::c_int as usize] as libc::c_int != ')' as i32
                    && EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char) != 0
                    || InitSetPoints(&mut (*panal).mc) != 0) as libc::c_int;
            } else {
                iErr = 1 as libc::c_int;
                ReportError(
                    pibIn,
                    0x14 as libc::c_int as WORD,
                    b"identifier\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            }
        }
    }
    if iErr != 0 {
        printf(
            b"Syntax:\n%s (\"OutputFile\", \"SetPtsFile\", nRuns, <param-id-list...>)\n\n\0"
                as *const u8 as *const libc::c_char,
            GetKeyword(13 as libc::c_int),
        );
        printf(b"Exiting...\n\0" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    } else {
        (*panal).iType = 3 as libc::c_int;
    }
    return iErr;
}
pub unsafe extern "C" fn GetMonteCarloSpec(
    mut pibIn: PINPUTBUF,
    mut panal: PANALYSIS,
    mut szLex: PSTR,
) -> libc::c_int {
    static mut vrgiMCArgTypes: [libc::c_int; 3] = [
        0x10 as libc::c_int,
        0x2 as libc::c_int,
        0x2 as libc::c_int | 0x4 as libc::c_int,
    ];
    let mut iErr: libc::c_int = 0 as libc::c_int;
    iErr = (GetFuncArgs(
        pibIn,
        3 as libc::c_int,
        vrgiMCArgTypes.as_mut_ptr(),
        (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
    ) == 0) as libc::c_int;
    if iErr == 0 {
        if *(vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr() != 0 {
            (*panal)
                .mc
                .szMCOutfilename = malloc(
                (if !(vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr().is_null() {
                    strlen((vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr())
                } else {
                    0 as libc::c_int as libc::c_ulong
                })
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as PSTR;
            if ((*panal).mc.szMCOutfilename).is_null() {
                ReportError(
                    pibIn,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"GetMonteCarloSpec\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
            if !((*panal).mc.szMCOutfilename).is_null()
                && !(vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr().is_null()
            {
                strcpy(
                    (*panal).mc.szMCOutfilename,
                    (vrgszlexArgs[0 as libc::c_int as usize]).as_mut_ptr(),
                );
            } else {};
            (*panal).bAllocatedFileName = 1 as libc::c_int;
        }
        (*panal).mc.nRuns = atol((vrgszlexArgs[1 as libc::c_int as usize]).as_mut_ptr());
        (*panal).dSeed = atof((vrgszlexArgs[2 as libc::c_int as usize]).as_mut_ptr());
    } else {
        printf(
            b"Syntax: %s (szOutfilename, nRuns, dSeed)\n\n\0" as *const u8
                as *const libc::c_char,
            GetKeyword(11 as libc::c_int),
        );
    }
    if iErr == 0 {
        (*panal).iType = 2 as libc::c_int;
    }
    return iErr;
}
pub unsafe extern "C" fn GetParmMod(
    mut pibIn: PINPUTBUF,
    mut szLex: *mut libc::c_char,
) -> BOOL {
    let mut hvar: HVAR = GetVarHandle(szLex);
    let mut panal: PANALYSIS = (*pibIn).pInfo as PANALYSIS;
    let mut pexp: PEXPERIMENT = (*panal).pexpCurrent;
    let mut szPunct: PSTRLEX_ = [0; 255];
    let mut iErr: libc::c_int = 0;
    let mut pvarmod: PVARMOD = 0 as *mut tagVARMODIFICATION;
    iErr = (hvar == 0) as libc::c_int;
    if iErr != 0 {
        ReportError(
            pibIn,
            0x14 as libc::c_int as WORD,
            b"model-variable\0" as *const u8 as *const libc::c_char as PSTR,
            szLex,
        );
    } else {
        pvarmod = malloc(::std::mem::size_of::<VARMODIFICATION>() as libc::c_ulong)
            as PVARMOD;
        if pvarmod.is_null() {
            ReportError(
                pibIn,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"GetParmMod\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        (*pvarmod).hvar = hvar;
        if GetOptPunct(pibIn, szPunct.as_mut_ptr(), '=' as i32 as libc::c_char) == 0 {
            szPunct[1 as libc::c_int as usize] = '=' as i32 as libc::c_char;
            iErr = szPunct[1 as libc::c_int as usize] as libc::c_int;
            ReportError(
                pibIn,
                0x13 as libc::c_int as WORD,
                szPunct.as_mut_ptr(),
                0 as PSTR,
            );
        } else if IsInput(hvar) != 0 {
            (*pvarmod)
                .uvar
                .pifn = malloc(::std::mem::size_of::<IFN>() as libc::c_ulong) as PIFN;
            if ((*pvarmod).uvar.pifn).is_null() {
                ReportError(
                    pibIn,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"GetParmMod\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
            iErr = (((*pvarmod).uvar.pifn).is_null()
                || GetInputFn(pibIn, 0 as PSTR, (*pvarmod).uvar.pifn) == 0)
                as libc::c_int;
            if iErr != 0 {
                free((*pvarmod).uvar.pifn as *mut libc::c_void);
                (*pvarmod).uvar.pifn = 0 as PIFN;
            }
        } else {
            iErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
            if iErr == 0 {
                (*pvarmod).uvar.dVal = atof(szLex as *const libc::c_char);
            }
        }
        if iErr == 0 {
            if (*panal).iCurrentDepth == 0 as libc::c_int
                || (*panal).wContext == 0x2 as libc::c_int as libc::c_uint
            {
                QueueListItem((*pexp).plistParmMods, pvarmod as PVOID);
            } else {
                QueueListItem(
                    (*(*panal)
                        .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int)
                        as usize])
                        .plistVars,
                    pvarmod as PVOID,
                );
            }
            iErr = GetTerminator(pibIn, szLex);
        } else {
            free(pvarmod as *mut libc::c_void);
        }
    }
    return iErr;
}
pub unsafe extern "C" fn GetParmMod2(
    mut pibIn: PINPUTBUF,
    mut szLex: *mut libc::c_char,
    mut szEqn: *mut libc::c_char,
) -> BOOL {
    let mut iErr: libc::c_int = 0;
    let mut pvarmod: PVARMOD = 0 as *mut tagVARMODIFICATION;
    let mut hvar: HVAR = GetVarHandle(szLex);
    let mut panal: PANALYSIS = (*pibIn).pInfo as PANALYSIS;
    let mut pexp: PEXPERIMENT = (*panal).pexpCurrent;
    iErr = (hvar == 0) as libc::c_int;
    if iErr != 0 {
        ReportError(
            pibIn,
            0x14 as libc::c_int as WORD,
            b"model-variable\0" as *const u8 as *const libc::c_char as PSTR,
            szLex,
        );
    } else {
        pvarmod = malloc(::std::mem::size_of::<VARMODIFICATION>() as libc::c_ulong)
            as PVARMOD;
        if pvarmod.is_null() {
            ReportError(
                pibIn,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"GetParmMod\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        (*pvarmod).hvar = hvar;
        if IsInput(hvar) != 0 {
            (*pvarmod)
                .uvar
                .pifn = malloc(::std::mem::size_of::<IFN>() as libc::c_ulong) as PIFN;
            if ((*pvarmod).uvar.pifn).is_null() {
                ReportError(
                    pibIn,
                    (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"GetParmMod\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
            iErr = (((*pvarmod).uvar.pifn).is_null()
                || GetInputFn(pibIn, 0 as PSTR, (*pvarmod).uvar.pifn) == 0)
                as libc::c_int;
            if iErr != 0 {
                free((*pvarmod).uvar.pifn as *mut libc::c_void);
                (*pvarmod).uvar.pifn = 0 as PIFN;
            }
        } else {
            (*pvarmod).uvar.dVal = atof(szEqn as *const libc::c_char);
        }
        if iErr == 0 {
            if (*panal).iCurrentDepth == 0 as libc::c_int
                || (*panal).wContext == 0x2 as libc::c_int as libc::c_uint
            {
                QueueListItem((*pexp).plistParmMods, pvarmod as PVOID);
            } else {
                QueueListItem(
                    (*(*panal)
                        .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int)
                        as usize])
                        .plistVars,
                    pvarmod as PVOID,
                );
            }
        } else {
            free(pvarmod as *mut libc::c_void);
        }
    }
    return iErr;
}
pub unsafe extern "C" fn NewExperiment(mut pibIn: PINPUTBUF) {
    let mut panal: PANALYSIS = (*pibIn).pInfo as PANALYSIS;
    let mut plevel: PLEVEL = 0 as *mut tagLEVEL;
    let mut n: libc::c_int = 0;
    if (*panal).iCurrentDepth < 0 as libc::c_int {
        ReportError(
            pibIn,
            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"Level statement\0" as *const u8 as *const libc::c_char as PSTR,
            b"Simulation\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
    if (*panal).iCurrentDepth == 0 as libc::c_int {
        (*panal).expGlobal.iExp += 1;
        (*panal).expGlobal.iExp;
        (*panal)
            .rgpExps[((*panal).expGlobal.iExp - 1 as libc::c_int)
            as usize] = malloc(::std::mem::size_of::<EXPERIMENT>() as libc::c_ulong)
            as PEXPERIMENT;
        (*panal)
            .pexpCurrent = (*panal)
            .rgpExps[((*panal).expGlobal.iExp - 1 as libc::c_int) as usize];
        if ((*panal).pexpCurrent).is_null() {
            ReportError(
                pibIn,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"NewExperiment()\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        if (*panal).rank == 0 as libc::c_int {
            printf(
                b"Reading experiment %d.\n\0" as *const u8 as *const libc::c_char,
                (*panal).expGlobal.iExp,
            );
        }
    } else {
        plevel = (*panal).pLevels[((*panal).iInstances - 1 as libc::c_int) as usize];
        n = 0 as libc::c_int;
        while n < (*panal).iCurrentDepth - 1 as libc::c_int {
            plevel = (*plevel)
                .pLevels[((*plevel).iInstances - 1 as libc::c_int) as usize];
            n += 1;
            n;
        }
        if (*plevel).iInstances == 20000 as libc::c_int - 1 as libc::c_int {
            ReportError(
                pibIn,
                (0x111 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"NewExperiment\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        let fresh2 = (*(*panal)
            .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int) as usize])
            .iInstances;
        (*(*panal).pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int) as usize])
            .iInstances = (*(*panal)
            .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int) as usize])
            .iInstances + 1;
        n = fresh2;
        (*plevel)
            .pLevels[n
            as usize] = malloc(::std::mem::size_of::<LEVEL>() as libc::c_ulong)
            as PLEVEL;
        plevel = (*plevel).pLevels[n as usize];
        if plevel.is_null() {
            ReportError(
                pibIn,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"NewExperiment\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        (*plevel).iInstances = 0 as libc::c_int;
        (*plevel).iSequence = n + 1 as libc::c_int;
        (*plevel).iDepth = (*panal).iCurrentDepth;
        let fresh3 = (*panal).iCurrentDepth;
        (*panal).iCurrentDepth = (*panal).iCurrentDepth + 1;
        (*panal).pCurrentLevel[fresh3 as usize] = plevel;
        if (*panal).iDepth < (*panal).iCurrentDepth {
            (*panal).iDepth = (*panal).iCurrentDepth;
        }
        (*plevel).nLikes = 0 as libc::c_int as libc::c_long;
        (*plevel).nFixedVars = (*plevel).nLikes;
        (*plevel).nMCVars = (*plevel).nFixedVars;
        (*plevel).plistVars = InitList();
        (*plevel).plistMCVars = InitList();
        (*plevel).plistLikes = InitList();
        (*plevel)
            .pexpt = malloc(::std::mem::size_of::<EXPERIMENT>() as libc::c_ulong)
            as PEXPERIMENT;
        if ((*plevel).pexpt).is_null() {
            ReportError(
                pibIn,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"NewExperiment\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        (*panal).pexpCurrent = (*plevel).pexpt;
        (*panal).iExpts += 1;
        (*panal).expGlobal.iExp = (*panal).iExpts;
        (*(*panal).pexpCurrent).iExp = (*panal).expGlobal.iExp;
        (*panal).wContext = 0x2 as libc::c_int as WORD;
        if (*panal).rank == 0 as libc::c_int {
            printf(
                b"Simulation %d - depth %d, instance %d\n\0" as *const u8
                    as *const libc::c_char,
                (*panal).iExpts,
                (*panal).iCurrentDepth,
                (*(*panal)
                    .pCurrentLevel[((*panal).iCurrentDepth - 2 as libc::c_int) as usize])
                    .iInstances,
            );
        }
    }
    memcpy(
        (*panal).pexpCurrent as *mut libc::c_void,
        &mut (*panal).expGlobal as *mut EXPERIMENT as *const libc::c_void,
        ::std::mem::size_of::<EXPERIMENT>() as libc::c_ulong,
    );
    (*panal).wContext = 0x2 as libc::c_int as WORD;
    (*(*panal).pexpCurrent).plistParmMods = InitList();
    (*(*panal).pexpCurrent).os.plistPrintRecs = InitList();
    (*(*panal).pexpCurrent).os.plistDataRecs = InitList();
}
pub unsafe extern "C" fn EndExperiment(
    mut pibIn: PINPUTBUF,
    mut panal: PANALYSIS,
) -> BOOL {
    let mut bReturn: BOOL = 0;
    bReturn = ((*pibIn).cErrors == 0) as libc::c_int;
    if bReturn == 0 {
        ReportError(
            pibIn,
            (0x201 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            &mut (*(*panal).pexpCurrent).iExp as *mut libc::c_int as PSTR,
            0 as PSTR,
        );
        if !pibIn.is_null() {
            (*pibIn).cErrors = 0 as libc::c_int;
        } else {};
        (*panal).expGlobal.iExp -= 1;
        (*panal).rgpExps[(*panal).expGlobal.iExp as usize] = 0 as PEXPERIMENT;
        free((*panal).pexpCurrent as *mut libc::c_void);
    } else {
        PrepareOutSpec((*panal).pexpCurrent);
    }
    (*panal).pexpCurrent = &mut (*panal).expGlobal;
    (*panal).wContext = 0x1 as libc::c_int as WORD;
    if (*panal).iType == 4 as libc::c_int
        && {
            let fresh4 = (*panal).iCurrentDepth;
            (*panal).iCurrentDepth = (*panal).iCurrentDepth - 1;
            fresh4 == 0 as libc::c_int
        }
    {
        return 0 as libc::c_int;
    }
    return bReturn;
}
pub unsafe extern "C" fn SetLevel(mut pibIn: PINPUTBUF) -> libc::c_int {
    let mut szPunct: PSTRLEX_ = [0; 255];
    let mut panal: PANALYSIS = (*pibIn).pInfo as PANALYSIS;
    let mut plevel: PLEVEL = 0 as *mut tagLEVEL;
    let mut n: libc::c_int = 0;
    if (*panal).iType != 4 as libc::c_int {
        ReportError(
            pibIn,
            (0x116 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"SetLevel\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    if (*panal).iCurrentDepth == 10 as libc::c_int {
        ReportError(
            pibIn,
            (0x110 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"SetLevel\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    if (*panal).wContext == 0x2 as libc::c_int as libc::c_uint {
        ReportError(
            pibIn,
            (0x113 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"SetLevel\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    if EGetPunct(pibIn, szPunct.as_mut_ptr(), '{' as i32 as libc::c_char) != 0 {
        return 1 as libc::c_int;
    }
    if (*panal).iCurrentDepth == 0 as libc::c_int {
        let fresh5 = (*panal).iInstances;
        (*panal).iInstances = (*panal).iInstances + 1;
        (*panal)
            .pLevels[fresh5
            as usize] = malloc(::std::mem::size_of::<LEVEL>() as libc::c_ulong)
            as PLEVEL;
        plevel = (*panal).pLevels[fresh5 as usize];
        if plevel.is_null() {
            ReportError(
                pibIn,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"SetLevel\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        if (*panal).iInstances > 1 as libc::c_int {
            printf(
                b"Error: only one top level is allowed - Exiting.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
        (*plevel).iSequence = (*panal).iInstances;
        if (*panal).rank == 0 as libc::c_int {
            printf(
                b"New level - depth 1, instance %d\n\0" as *const u8
                    as *const libc::c_char,
                (*panal).iInstances,
            );
        }
    } else {
        plevel = (*panal).pLevels[((*panal).iInstances - 1 as libc::c_int) as usize];
        n = 0 as libc::c_int;
        while n < (*panal).iCurrentDepth - 1 as libc::c_int {
            plevel = (*plevel)
                .pLevels[((*plevel).iInstances - 1 as libc::c_int) as usize];
            n += 1;
            n;
        }
        if (*plevel).iInstances == 20000 as libc::c_int - 1 as libc::c_int {
            ReportError(
                pibIn,
                (0x111 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"SetLevel\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        let fresh6 = (*(*panal)
            .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int) as usize])
            .iInstances;
        (*(*panal).pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int) as usize])
            .iInstances = (*(*panal)
            .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int) as usize])
            .iInstances + 1;
        n = fresh6;
        (*plevel)
            .pLevels[n
            as usize] = malloc(::std::mem::size_of::<LEVEL>() as libc::c_ulong)
            as PLEVEL;
        plevel = (*plevel).pLevels[n as usize];
        if plevel.is_null() {
            ReportError(
                pibIn,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"SetLevel\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        (*plevel).iSequence = n + 1 as libc::c_int;
        if (*panal).rank == 0 as libc::c_int {
            printf(
                b"New level - depth %d, instance %d\n\0" as *const u8
                    as *const libc::c_char,
                (*panal).iCurrentDepth + 1 as libc::c_int,
                (*(*panal)
                    .pCurrentLevel[((*panal).iCurrentDepth - 1 as libc::c_int) as usize])
                    .iInstances,
            );
        }
    }
    (*plevel).iInstances = 0 as libc::c_int;
    (*plevel).iDepth = (*panal).iCurrentDepth;
    let fresh7 = (*panal).iCurrentDepth;
    (*panal).iCurrentDepth = (*panal).iCurrentDepth + 1;
    (*panal).pCurrentLevel[fresh7 as usize] = plevel;
    if (*panal).iDepth < (*panal).iCurrentDepth {
        (*panal).iDepth = (*panal).iCurrentDepth;
    }
    (*plevel).nLikes = 0 as libc::c_int as libc::c_long;
    (*plevel).nFixedVars = (*plevel).nLikes;
    (*plevel).nMCVars = (*plevel).nFixedVars;
    (*plevel).plistVars = InitList();
    (*plevel).plistMCVars = InitList();
    (*plevel).plistLikes = InitList();
    (*plevel).pexpt = 0 as PEXPERIMENT;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn EndLevel(mut panal: PANALYSIS) -> BOOL {
    let fresh8 = (*panal).iCurrentDepth;
    (*panal).iCurrentDepth = (*panal).iCurrentDepth - 1;
    if fresh8 == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn FreeLevels(mut panal: PANALYSIS) {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < (*panal).iInstances {
        if !((*panal).pLevels[n as usize]).is_null() {
            FreeOneLevel((*panal).pLevels[n as usize]);
        }
        n += 1;
        n;
    }
    if (*panal).bAllocatedFileName != 0 {
        free((*panal).gd.szGout as *mut libc::c_void);
    }
    FreeList(&mut (*panal).mc.plistMCVars, None, 1 as libc::c_int);
    FreeList(&mut (*panal).expGlobal.plistParmMods, None, 1 as libc::c_int);
    free((*panal).expGlobal.is.iwork as *mut libc::c_void);
    free((*panal).expGlobal.is.rwork as *mut libc::c_void);
    free((*panal).modelinfo.pStateHvar as *mut libc::c_void);
    free(panal as *mut libc::c_void);
}
pub unsafe extern "C" fn FreeMCVar(
    mut pData: PVOID,
    mut pUserInfo: PVOID,
) -> libc::c_int {
    let mut pMCVar: PMCVAR = pData as PMCVAR;
    FreeList(&mut (*pMCVar).plistDependents, None, 1 as libc::c_int);
    free((*pMCVar).pszName as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FreeDataRec(
    mut pData: PVOID,
    mut pUserInfo: PVOID,
) -> libc::c_int {
    let mut pDataRecord: PDATAREC = pData as PDATAREC;
    free((*pDataRecord).szDataName as *mut libc::c_void);
    free(pDataRecord as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FreePrintRec(
    mut pData: PVOID,
    mut pUserInfo: PVOID,
) -> libc::c_int {
    let mut pPrintRecord: PPRINTREC = pData as PPRINTREC;
    free((*pPrintRecord).pdTimes as *mut libc::c_void);
    free((*pPrintRecord).szOutputName as *mut libc::c_void);
    free(pPrintRecord as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn FreeOneLevel(mut plevel: PLEVEL) {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < (*plevel).iInstances {
        if !((*plevel).pLevels[n as usize]).is_null() {
            FreeOneLevel((*plevel).pLevels[n as usize]);
        }
        n += 1;
        n;
    }
    FreeList(&mut (*plevel).plistVars, None, 1 as libc::c_int);
    ForAllList(
        (*plevel).plistMCVars,
        Some(FreeMCVar as unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int),
        0 as *mut libc::c_void,
    );
    FreeList(&mut (*plevel).plistMCVars, None, 1 as libc::c_int);
    ForAllList(
        (*plevel).plistLikes,
        Some(FreeMCVar as unsafe extern "C" fn(PVOID, PVOID) -> libc::c_int),
        0 as *mut libc::c_void,
    );
    FreeList(&mut (*plevel).plistLikes, None, 1 as libc::c_int);
    if !((*plevel).pexpt).is_null() {
        FreeList(
            &mut (*(*plevel).pexpt).plistParmMods,
            Some(FreeVarMod as unsafe extern "C" fn(PVOID) -> ()),
            0 as libc::c_int,
        );
        let mut pos: POUTSPEC = &mut (*(*plevel).pexpt).os;
        free((*pos).pszOutputNames as *mut libc::c_void);
        free((*pos).phvar_out as *mut libc::c_void);
        free((*pos).pcOutputTimes as *mut libc::c_void);
        free((*pos).piCurrentOut as *mut libc::c_void);
        free((*pos).prgdOutputTimes as *mut libc::c_void);
        n = 0 as libc::c_int;
        while n < (*pos).nOutputs {
            free(*((*pos).prgdOutputVals).offset(n as isize) as *mut libc::c_void);
            n += 1;
            n;
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
        free((*pos).pcData as *mut libc::c_void);
        free((*pos).phvar_dat as *mut libc::c_void);
        free((*pos).pszDataNames as *mut libc::c_void);
        n = 0 as libc::c_int;
        while n < (*pos).nData {
            free(*((*pos).prgdDataVals).offset(n as isize) as *mut libc::c_void);
            n += 1;
            n;
        }
        free((*pos).prgdDataVals as *mut libc::c_void);
        free((*plevel).pexpt as *mut libc::c_void);
    }
    if (*plevel).nFixedVars > 0 as libc::c_int as libc::c_long {
        free((*plevel).rgpFixedVars as *mut libc::c_void);
    }
    if (*plevel).nMCVars > 0 as libc::c_int as libc::c_long {
        free((*plevel).rgpMCVars as *mut libc::c_void);
    }
    free((*plevel).rgpLikes as *mut libc::c_void);
    free(plevel as *mut libc::c_void);
}
pub unsafe extern "C" fn ProcessWord(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut szEqn: PSTR,
) {
    let mut iErr: libc::c_int = 0 as libc::c_int;
    let mut iKWCode: libc::c_int = 0;
    let mut fContext: libc::c_int = 0;
    let mut i: libc::c_long = 0;
    let mut iLB: libc::c_long = 0;
    let mut iUB: libc::c_long = 0;
    let mut szEqnU: PSTREQN_ = [0; 1023];
    let mut szTmp: PSTRLEX_ = [0; 255];
    let mut panal: PANALYSIS = 0 as *mut tagANALYSIS;
    if pibIn.is_null() || szLex.is_null()
        || *szLex.offset(0 as libc::c_int as isize) == 0 || szEqn.is_null()
    {
        return;
    }
    panal = (*pibIn).pInfo as PANALYSIS;
    iKWCode = GetKeywordCode(szLex, &mut fContext);
    if (*panal).wContext != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"panal->wContext != CN_END\0" as *const u8 as *const libc::c_char,
            b"simi.c\0" as *const u8 as *const libc::c_char,
            2686 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void ProcessWord(PINPUTBUF, PSTR, PSTR)\0"))
                .as_ptr(),
        );
    }
    'c_17343: {
        if (*panal).wContext != 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"panal->wContext != CN_END\0" as *const u8 as *const libc::c_char,
                b"simi.c\0" as *const u8 as *const libc::c_char,
                2686 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void ProcessWord(PINPUTBUF, PSTR, PSTR)\0"))
                    .as_ptr(),
            );
        }
    };
    iErr = (iKWCode != 0 && fContext as libc::c_uint & (*panal).wContext == 0)
        as libc::c_int;
    if iErr != 0 {
        ReportError(pibIn, 0x101 as libc::c_int as WORD, szLex, 0 as PSTR);
    } else {
        match iKWCode {
            5 => {
                iErr = GetPrint(pibIn, szLex, &mut (*(*panal).pexpCurrent).os);
            }
            6 => {
                iErr = GetPrintStep(pibIn, szLex, &mut (*(*panal).pexpCurrent).os);
            }
            10 => {
                iErr = EGetPunct(pibIn, szTmp.as_mut_ptr(), '{' as i32 as libc::c_char);
                if iErr == 0 {
                    NewExperiment(pibIn);
                }
            }
            16 => {
                iErr = SetLevel(pibIn);
            }
            12 => {
                iErr = GetDistribSpec(pibIn, szLex, panal);
            }
            14 => {
                if !((*panal).szOutfilename).is_null() {
                    EatStatement(pibIn);
                } else {
                    iErr = GetOutputFile(pibIn, szLex, panal);
                }
            }
            7 => {
                iErr = GetData(pibIn, szLex, &mut (*(*panal).pexpCurrent).os);
            }
            1 => {
                iErr = GetIntegrate(pibIn, szLex, &mut (*(*panal).pexpCurrent).is);
            }
            15 => {
                iErr = GetMCMCSpec(pibIn, (*panal).pexpCurrent);
            }
            17 => {
                iErr = GetOptDSpec(pibIn, panal, szLex);
            }
            11 => {
                iErr = GetMonteCarloSpec(pibIn, panal, szLex);
            }
            13 => {
                iErr = GetSetPointsSpec(pibIn, panal, szLex);
            }
            2 => {
                iErr = GetSimulate();
            }
            3 => {
                iErr = GetStartTime(pibIn, (*panal).pexpCurrent);
            }
            8 => {
                iErr = GetSimType(pibIn);
            }
            4 => {
                iErr = GetPerks(pibIn, szLex, &mut (*panal).gd);
            }
            100 => {
                (*panal).wContext = 0 as libc::c_int as WORD;
            }
            _ => {
                iUB = -(1 as libc::c_int) as libc::c_long;
                iLB = iUB;
                if GetPunct(pibIn, szTmp.as_mut_ptr(), '[' as i32 as libc::c_char) != 0 {
                    GetArrayBounds(pibIn, &mut iLB, &mut iUB);
                } else if strcmp(
                    szTmp.as_mut_ptr(),
                    b"=\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(-1);
                    (*pibIn).pbufCur;
                }
                if iUB == -(1 as libc::c_int) as libc::c_long {
                    iErr = GetParmMod(pibIn, szLex);
                } else if GetPunct(pibIn, szTmp.as_mut_ptr(), '=' as i32 as libc::c_char)
                    != 0
                {
                    GetStatement(pibIn, szEqn);
                    i = iLB;
                    while i < iUB {
                        sprintf(
                            szTmp.as_mut_ptr(),
                            b"%s_%ld\0" as *const u8 as *const libc::c_char,
                            szLex,
                            i,
                        );
                        UnrollEquation(pibIn, i, szEqn, szEqnU.as_mut_ptr());
                        iErr = GetParmMod2(
                            pibIn,
                            szTmp.as_mut_ptr(),
                            szEqnU.as_mut_ptr(),
                        );
                        if iErr != 0 {
                            break;
                        }
                        i += 1;
                        i;
                    }
                } else {
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"= or [\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
            }
        }
    }
    if iErr != 0 {
        EatStatement(pibIn);
    }
}
pub unsafe extern "C" fn ReadAnalysis(mut pibIn: PINPUTBUF) -> BOOL {
    let mut szLex: PSTRLEX_ = [0; 255];
    let mut szEqn: PSTREQN_ = [0; 1023];
    let mut iLexType: libc::c_int = 0;
    let mut bReturn: BOOL = 1 as libc::c_int;
    let mut panal: PANALYSIS = 0 as *mut tagANALYSIS;
    if pibIn.is_null() {
        return 0 as libc::c_int;
    }
    panal = (*pibIn).pInfo as PANALYSIS;
    (*panal).iInstances = 0 as libc::c_int;
    (*panal).iCurrentDepth = (*panal).iInstances;
    (*panal).iDepth = (*panal).iCurrentDepth;
    (*panal).mc.plistMCVars = InitList();
    loop {
        NextLex(pibIn, szLex.as_mut_ptr(), &mut iLexType);
        let mut current_block_16: u64;
        match iLexType {
            0 => {
                if (*panal).wContext != 0x1 as libc::c_int as libc::c_uint {
                    ReportError(
                        pibIn,
                        0x4000 as libc::c_int as WORD,
                        0 as PSTR,
                        b"Unexpected end of file\0" as *const u8 as *const libc::c_char
                            as PSTR,
                    );
                }
                if (*panal).wContext == 0x2 as libc::c_int as libc::c_uint {
                    bReturn &= EndExperiment(pibIn, panal);
                }
                (*panal).wContext = 0 as libc::c_int as WORD;
                current_block_16 = 15768484401365413375;
            }
            1 => {
                ProcessWord(pibIn, szLex.as_mut_ptr(), szEqn.as_mut_ptr());
                current_block_16 = 15768484401365413375;
            }
            8 => {
                if szLex[0 as libc::c_int as usize] as libc::c_int == ';' as i32 {
                    current_block_16 = 15768484401365413375;
                } else if szLex[0 as libc::c_int as usize] as libc::c_int == '}' as i32 {
                    if (*panal).wContext & 0x2 as libc::c_int as libc::c_uint != 0 {
                        bReturn &= EndExperiment(pibIn, panal);
                    } else {
                        bReturn &= EndLevel(panal);
                    }
                    current_block_16 = 15768484401365413375;
                } else if szLex[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
                    SkipComment(pibIn);
                    current_block_16 = 15768484401365413375;
                } else {
                    current_block_16 = 17565523809617478571;
                }
            }
            2 | 4 => {
                ReportError(
                    pibIn,
                    0x12 as libc::c_int as WORD,
                    szLex.as_mut_ptr(),
                    b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
                );
                current_block_16 = 15768484401365413375;
            }
            _ => {
                current_block_16 = 17565523809617478571;
            }
        }
        match current_block_16 {
            17565523809617478571 => {
                ReportError(
                    pibIn,
                    0x11 as libc::c_int as WORD,
                    szLex.as_mut_ptr(),
                    b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
                );
            }
            _ => {}
        }
        if !((*panal).wContext != 0 as libc::c_int as libc::c_uint
            && (*(*pibIn).pbufCur as libc::c_int != 0
                || FillBuffer(pibIn) != -(1 as libc::c_int)))
        {
            break;
        }
    }
    if (*panal).iCurrentDepth != 0 as libc::c_int {
        ReportError(
            pibIn,
            (0x112 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ReadAnalysis\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    return bReturn;
}
