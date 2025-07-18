use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn FreeLevels(panal: PANALYSIS);
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
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
pub unsafe extern "C" fn ReportError(
    mut pibIn: PINPUTBUF,
    mut wCode: WORD,
    mut szMsg: PSTR,
    mut szAltMsg: PSTR,
) {
    let mut cNull: libc::c_char = '\0' as i32 as libc::c_char;
    let mut bFatal: BOOL = (wCode & 0x8000 as libc::c_int as libc::c_uint) as BOOL;
    let mut bWarning: BOOL = (wCode & 0x4000 as libc::c_int as libc::c_uint) as BOOL;
    wCode &= !(0x8000 as libc::c_int | 0x4000 as libc::c_int) as libc::c_uint;
    if szMsg.is_null() {
        szMsg = &mut cNull;
    }
    if wCode != 0 {
        if bWarning != 0 {
            printf(b"Warning: \0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"Error: \0" as *const u8 as *const libc::c_char);
            bFatal
                |= (!pibIn.is_null()
                    && {
                        let fresh0 = (*pibIn).cErrors;
                        (*pibIn).cErrors = (*pibIn).cErrors + 1;
                        fresh0 > 0 as libc::c_int
                    }) as libc::c_int;
        }
    }
    if !pibIn.is_null() {
        if !((*pibIn).pfileIn).is_null() || (*pibIn).iLNPrev != 0 {
            printf(
                b"line %d: \0" as *const u8 as *const libc::c_char,
                (*pibIn).iLineNum,
            );
        } else if wCode != 0x2 as libc::c_int as libc::c_uint {
            let mut szTmp: PSTRLEX_ = [0; 255];
            szTmp[(255 as libc::c_int - 1 as libc::c_int)
                as usize] = '\0' as i32 as libc::c_char;
            printf(
                b"'%s'...\n  \0" as *const u8 as *const libc::c_char,
                strncpy(
                    szTmp.as_mut_ptr(),
                    (*pibIn).pbufOrg as *const libc::c_char,
                    (255 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                ),
            );
        }
    }
    let mut current_block_48: u64;
    match wCode {
        0 => {
            current_block_48 = 6528285054092551010;
        }
        1 => {
            current_block_48 = 5516530427166507445;
        }
        2 => {
            printf(
                b"File not found \"%s\".\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_48 = 6528285054092551010;
        }
        3 => {
            printf(
                b"Cannot open file \"%s\".\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_48 = 6528285054092551010;
        }
        4 => {
            printf(
                b"Out of memory in %s().\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_48 = 6528285054092551010;
        }
        5 => {
            printf(
                b"%s file cannot be read\n\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_48 = 6528285054092551010;
        }
        17 => {
            printf(
                b"Unexpected character '%c' in input file.\0" as *const u8
                    as *const libc::c_char,
                *szMsg as libc::c_int,
            );
            current_block_48 = 6528285054092551010;
        }
        18 => {
            printf(
                b"Unexpected number %s in input file.\0" as *const u8
                    as *const libc::c_char,
                szMsg,
            );
            current_block_48 = 6528285054092551010;
        }
        19 => {
            printf(
                b"Expected '%c' before '%c'.\0" as *const u8 as *const libc::c_char,
                *szMsg.offset(1 as libc::c_int as isize) as libc::c_int,
                *szMsg.offset(0 as libc::c_int as isize) as libc::c_int,
            );
            current_block_48 = 6528285054092551010;
        }
        20 => {
            printf(b"Expected <%s>\0" as *const u8 as *const libc::c_char, szMsg);
            if !szAltMsg.is_null() {
                printf(b" before '%s'\0" as *const u8 as *const libc::c_char, szAltMsg);
            }
            current_block_48 = 6528285054092551010;
        }
        21 => {
            printf(b"Syntax error %s\0" as *const u8 as *const libc::c_char, szMsg);
            current_block_48 = 6528285054092551010;
        }
        257 => {
            printf(
                b"'%s' used in invalid context.\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_48 = 6528285054092551010;
        }
        272 => {
            printf(b"Too many levels\0" as *const u8 as *const libc::c_char);
            current_block_48 = 6528285054092551010;
        }
        273 => {
            printf(
                b"Too many instances at level %s\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_48 = 6528285054092551010;
        }
        274 => {
            printf(b"Unclosed level statement\0" as *const u8 as *const libc::c_char);
            current_block_48 = 6528285054092551010;
        }
        275 => {
            printf(
                b"Level statement enclosed in Simulation (Experiment) statement\0"
                    as *const u8 as *const libc::c_char,
            );
            current_block_48 = 6528285054092551010;
        }
        279 => {
            printf(
                b"Too many variables in 'Print(...)' statement\0" as *const u8
                    as *const libc::c_char,
            );
            current_block_48 = 6528285054092551010;
        }
        260 => {
            printf(
                b"Equation is too long.  Possibly missing terminator.\0" as *const u8
                    as *const libc::c_char,
            );
            current_block_48 = 6528285054092551010;
        }
        262 => {
            printf(
                b"Undefined identifier '%s'.\0" as *const u8 as *const libc::c_char,
                szMsg,
            );
            current_block_48 = 6528285054092551010;
        }
        278 => {
            printf(
                b"The level statement is permitted only in MCMC simulations.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            current_block_48 = 6528285054092551010;
        }
        513 => {
            printf(
                b"Bad definition of experiment %d\n\0" as *const u8
                    as *const libc::c_char,
                *(szMsg as PINT),
            );
            current_block_48 = 6528285054092551010;
        }
        514 => {
            printf(
                b"Simulation (Experiment) %d has no outputs specified\n\0" as *const u8
                    as *const libc::c_char,
                *(szMsg as PINT),
            );
            current_block_48 = 6528285054092551010;
        }
        290 => {
            printf(b"Positive number expected.\0" as *const u8 as *const libc::c_char);
            current_block_48 = 6528285054092551010;
        }
        517 => {
            printf(b"in specification: %s\0" as *const u8 as *const libc::c_char, szMsg);
            current_block_48 = 6528285054092551010;
        }
        520 => {
            printf(
                b"Insufficient points in file \"%s\"\n\0" as *const u8
                    as *const libc::c_char,
                szMsg,
            );
            current_block_48 = 6528285054092551010;
        }
        521 => {
            printf(b"Max is less than min\n\0" as *const u8 as *const libc::c_char);
            current_block_48 = 6528285054092551010;
        }
        528 => {
            printf(
                b"Output and restart files have the same name\n\0" as *const u8
                    as *const libc::c_char,
            );
            current_block_48 = 6528285054092551010;
        }
        _ => {
            printf(
                b"Unknown error code %x: %s\0" as *const u8 as *const libc::c_char,
                wCode,
                szMsg,
            );
            current_block_48 = 5516530427166507445;
        }
    }
    match current_block_48 {
        5516530427166507445 => {
            printf(b"Initialization error.\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if !szAltMsg.is_null() && wCode != 0x14 as libc::c_int as libc::c_uint {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, szAltMsg);
    }
    if bFatal != 0 {
        if !pibIn.is_null() && !((*pibIn).pInfo).is_null() {
            FreeLevels((*pibIn).pInfo as PANALYSIS);
        }
        printf(b"\nFatal errors.  Exiting.\n\n\0" as *const u8 as *const libc::c_char);
        exit(wCode as libc::c_int);
    }
}
pub unsafe extern "C" fn ReportRunTimeError(
    mut panal: PANALYSIS,
    mut wCode: WORD,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut szMsg1: PSTR = 0 as *mut libc::c_char;
    let mut szMsg2: PSTR = 0 as *mut libc::c_char;
    let mut szMsg3: PSTR = 0 as *mut libc::c_char;
    let mut bFatal: BOOL = (wCode & 0x8000 as libc::c_int as libc::c_uint) as BOOL;
    let mut bWarning: BOOL = (wCode & 0x4000 as libc::c_int as libc::c_uint) as BOOL;
    wCode &= !(0x8000 as libc::c_int | 0x4000 as libc::c_int) as libc::c_uint;
    if wCode != 0 {
        if bWarning != 0 {
            printf(b"Warning: \0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"Fatal error: \0" as *const u8 as *const libc::c_char);
        };
    }
    ap = args.clone();
    match wCode {
        0 => {}
        4 => {
            szMsg1 = ap.arg::<PSTR>();
            printf(
                b"Out of memory in %s().\0" as *const u8 as *const libc::c_char,
                szMsg1,
            );
        }
        3 => {
            szMsg1 = ap.arg::<PSTR>();
            szMsg2 = ap.arg::<PSTR>();
            printf(
                b"Cannot open file \"%s\" in %s().\0" as *const u8
                    as *const libc::c_char,
                szMsg1,
                szMsg2,
            );
        }
        769 => {
            szMsg1 = ap.arg::<PSTR>();
            szMsg2 = ap.arg::<PSTR>();
            szMsg3 = ap.arg::<PSTR>();
            printf(
                b"SD of normal variate %s = %s in %s().\0" as *const u8
                    as *const libc::c_char,
                szMsg1,
                szMsg2,
                szMsg3,
            );
        }
        770 => {
            szMsg1 = ap.arg::<PSTR>();
            szMsg2 = ap.arg::<PSTR>();
            szMsg3 = ap.arg::<PSTR>();
            printf(
                b"SD of lognormal variate %s = %s in %s().\0" as *const u8
                    as *const libc::c_char,
                szMsg1,
                szMsg2,
                szMsg3,
            );
        }
        771 => {
            szMsg1 = ap.arg::<PSTR>();
            szMsg2 = ap.arg::<PSTR>();
            szMsg3 = ap.arg::<PSTR>();
            printf(
                b"Mean of lognormal variate %s = %s in %s().\0" as *const u8
                    as *const libc::c_char,
                szMsg1,
                szMsg2,
                szMsg3,
            );
        }
        772 => {
            szMsg1 = ap.arg::<PSTR>();
            szMsg2 = ap.arg::<PSTR>();
            printf(
                b"Max and min of uniform variate %s are equal or inverted in %s().\0"
                    as *const u8 as *const libc::c_char,
                szMsg1,
                szMsg2,
            );
        }
        773 => {
            szMsg1 = ap.arg::<PSTR>();
            printf(
                b"Unknown distribution in %s().\0" as *const u8 as *const libc::c_char,
                szMsg1,
            );
        }
        775 => {
            printf(
                b"Bad value in output; model is not computable.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        289 => {
            szMsg1 = ap.arg::<PSTR>();
            szMsg2 = ap.arg::<PSTR>();
            printf(
                b"Variable %s appears in two or more '%s' statements.\n\0" as *const u8
                    as *const libc::c_char,
                szMsg1,
                szMsg2,
            );
        }
        _ => {
            printf(
                b"Unknown error code %x\0" as *const u8 as *const libc::c_char,
                wCode,
            );
        }
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if bFatal != 0 {
        if !panal.is_null() {
            FreeLevels(panal);
        }
        printf(b"\nFatal errors.  Exiting.\n\n\0" as *const u8 as *const libc::c_char);
        exit(wCode as libc::c_int);
    }
}
