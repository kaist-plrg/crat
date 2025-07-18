use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn InitdVector(cVectors: libc::c_long) -> *mut libc::c_double;
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagMCPREDOUT {
    pub nbrdy: libc::c_long,
    pub pred: *mut libc::c_double,
    pub passflag: libc::c_int,
}
pub type PMCPREDOUT = *mut tagMCPREDOUT;
pub unsafe extern "C" fn DoStep_by_Step() {}
pub unsafe extern "C" fn TransformPred(
    mut panal: PANALYSIS,
    mut pMCPredOut: PMCPREDOUT,
) {
    OutspecToLinearArray(panal, pMCPredOut);
}
pub unsafe extern "C" fn OutspecToLinearArray(
    mut panal: PANALYSIS,
    mut pMCPredOut: PMCPREDOUT,
) {
    let mut pos: POUTSPEC = 0 as *mut tagOUTSPEC;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    (*pMCPredOut).nbrdy = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int as libc::c_long;
    while i < (*panal).expGlobal.iExp as libc::c_long {
        j = 0 as libc::c_int as libc::c_long;
        pos = &mut (**((*panal).rgpExps).as_mut_ptr().offset(i as isize)).os;
        while j < (*pos).nOutputs as libc::c_long {
            k = 0 as libc::c_int as libc::c_long;
            while k < *((*pos).pcOutputTimes).offset(j as isize) as libc::c_long {
                (*pMCPredOut).nbrdy += 1;
                (*pMCPredOut).nbrdy;
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if ((*pMCPredOut).pred).is_null() {
        (*pMCPredOut).pred = InitdVector((*pMCPredOut).nbrdy);
        if ((*pMCPredOut).pred).is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"OutspecToLinearArray\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    }
    (*pMCPredOut).nbrdy = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int as libc::c_long;
    while i < (*panal).expGlobal.iExp as libc::c_long {
        j = 0 as libc::c_int as libc::c_long;
        pos = &mut (**((*panal).rgpExps).as_mut_ptr().offset(i as isize)).os;
        while j < (*pos).nOutputs as libc::c_long {
            k = 0 as libc::c_int as libc::c_long;
            while k < *((*pos).pcOutputTimes).offset(j as isize) as libc::c_long {
                let fresh0 = (*pMCPredOut).nbrdy;
                (*pMCPredOut).nbrdy = (*pMCPredOut).nbrdy + 1;
                *((*pMCPredOut).pred)
                    .offset(
                        fresh0 as isize,
                    ) = *(*((*pos).prgdOutputVals).offset(j as isize))
                    .offset(k as isize);
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
pub unsafe extern "C" fn Definite_Integral(
    mut Function: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
    mut dFrom: libc::c_double,
    mut dTo: libc::c_double,
) -> libc::c_double {
    let mut dTotal_Area: libc::c_double = 0.;
    let mut dDelta_Area: libc::c_double = 0.;
    let mut pdArea: [libc::c_double; 27] = [0.; 27];
    let mut pdTemp: [libc::c_double; 27] = [0.; 27];
    let mut i: libc::c_int = 0;
    if dFrom >= dTo {
        if dFrom == dTo {
            return 0 as libc::c_int as libc::c_double
        } else {
            printf(
                b"\nError: inverted integration bounds in Definite_Integral - Exiting\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        }
    }
    pdTemp[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < 25 as libc::c_int {
        pdArea[i as usize] = Trapezes(Function, dFrom, dTo, i + 1 as libc::c_int);
        if i >= 4 as libc::c_int {
            Interpolate_Poly(
                &mut *pdTemp.as_mut_ptr().offset((i - 4 as libc::c_int) as isize),
                &mut *pdArea.as_mut_ptr().offset((i - 4 as libc::c_int) as isize),
                4 as libc::c_int + 1 as libc::c_int,
                0.0f64,
                &mut dTotal_Area,
                &mut dDelta_Area,
            );
            if dTotal_Area == 0 as libc::c_int as libc::c_double {
                if fabs(dDelta_Area) < 1.0e-6f64 {
                    return dTotal_Area;
                }
            } else if fabs(dDelta_Area) < 1.0e-6f64 * fabs(dTotal_Area) {
                return dTotal_Area
            }
        }
        pdArea[(i + 1 as libc::c_int) as usize] = pdArea[i as usize];
        pdTemp[(i + 1 as libc::c_int) as usize] = 0.25f64 * pdTemp[i as usize];
        i += 1;
        i;
    }
    printf(
        b"\nError: Too many steps in routine Definite_Integral - Exiting\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}
pub unsafe extern "C" fn Interpolate_Poly(
    mut rgdX: *mut libc::c_double,
    mut rgdY: *mut libc::c_double,
    mut n: libc::c_int,
    mut x: libc::c_double,
    mut pdY: *mut libc::c_double,
    mut pdDY: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nIndex: libc::c_int = 1 as libc::c_int;
    let mut dDenom: libc::c_double = 0.;
    let mut dDiff: libc::c_double = 0.;
    let mut dTemp1: libc::c_double = 0.;
    let mut dTemp2: libc::c_double = 0.;
    static mut pdTerm1: PDOUBLE = 0 as *const libc::c_double as PDOUBLE;
    static mut pdTerm2: PDOUBLE = 0 as *const libc::c_double as PDOUBLE;
    if pdTerm1.is_null() {
        pdTerm1 = InitdVector((n + 1 as libc::c_int) as libc::c_long);
        if pdTerm1.is_null()
            || {
                pdTerm2 = InitdVector((n + 1 as libc::c_int) as libc::c_long);
                pdTerm2.is_null()
            }
        {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"Interpolate_Poly\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    }
    dDiff = fabs(x - *rgdX.offset(0 as libc::c_int as isize));
    *pdTerm1.offset(0 as libc::c_int as isize) = *rgdY.offset(0 as libc::c_int as isize);
    *pdTerm2.offset(0 as libc::c_int as isize) = *rgdY.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < n {
        dTemp1 = fabs(x - *rgdX.offset(i as isize));
        if dTemp1 < dDiff {
            nIndex = i;
            dDiff = dTemp1;
        }
        *pdTerm1.offset(i as isize) = *rgdY.offset(i as isize);
        *pdTerm2.offset(i as isize) = *rgdY.offset(i as isize);
        i += 1;
        i;
    }
    let fresh1 = nIndex;
    nIndex = nIndex - 1;
    *pdY = *rgdY.offset(fresh1 as isize);
    j = 1 as libc::c_int;
    while j < n {
        i = 0 as libc::c_int;
        while i < n - j {
            dTemp1 = *rgdX.offset(i as isize) - x;
            dTemp2 = *rgdX.offset((i + j) as isize) - x;
            dDenom = dTemp1 - dTemp2;
            if dDenom == 0 as libc::c_int as libc::c_double {
                printf(
                    b"\nError: null denominator in Interpolate_Poly - Exiting\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            dDenom = (*pdTerm1.offset((i + 1 as libc::c_int) as isize)
                - *pdTerm2.offset(i as isize)) / dDenom;
            *pdTerm2.offset(i as isize) = dTemp2 * dDenom;
            *pdTerm1.offset(i as isize) = dTemp1 * dDenom;
            i += 1;
            i;
        }
        *pdDY = if 2 as libc::c_int * (nIndex + 1 as libc::c_int) < n - j {
            *pdTerm1.offset((nIndex + 1 as libc::c_int) as isize)
        } else {
            let fresh2 = nIndex;
            nIndex = nIndex - 1;
            *pdTerm2.offset(fresh2 as isize)
        };
        *pdY = *pdY + *pdDY;
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn Trapezes(
    mut Function: Option::<unsafe extern "C" fn(libc::c_double) -> libc::c_double>,
    mut dFrom: libc::c_double,
    mut dTo: libc::c_double,
    mut nSteps: libc::c_int,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut dSum: libc::c_double = 0.;
    let mut dDelta: libc::c_double = 0.;
    static mut dStoredArea: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if nSteps == 1 as libc::c_int {
        dStoredArea = 0.5f64 * (dTo - dFrom)
            * ((Some(Function.unwrap())).unwrap()(dFrom)
                + (Some(Function.unwrap())).unwrap()(dTo));
        return dStoredArea;
    } else {
        i = 1 as libc::c_int;
        j = 0 as libc::c_int;
        while j < nSteps - 2 as libc::c_int {
            i = i << 1 as libc::c_int;
            j += 1;
            j;
        }
        dDelta = (dTo - dFrom) / i as libc::c_double;
        x = dFrom + 0.5f64 * dDelta;
        dSum = 0 as libc::c_int as libc::c_double;
        while x < dTo {
            dSum = dSum + (Some(Function.unwrap())).unwrap()(x);
            x = x + dDelta;
        }
        dStoredArea = 0.5f64 * (dStoredArea + dDelta * dSum);
        return dStoredArea;
    };
}
