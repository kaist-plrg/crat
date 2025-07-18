use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn CalcDeriv(
        rgModelVars: *mut libc::c_double,
        rgDerivs: *mut libc::c_double,
        pdTime: PDOUBLE,
    );
    fn CalcOutputs(
        rgModelVars: *mut libc::c_double,
        rgDerivs: *mut libc::c_double,
        pdTime: PDOUBLE,
    );
    fn GetNModelVars() -> libc::c_int;
    fn GetVarValue(hVar: HVAR) -> libc::c_double;
    fn GetVarName(hVar: HVAR) -> *mut libc::c_char;
    fn IsInput(hVar: HVAR) -> BOOL;
    fn IsModelVar(hVar: HVAR) -> BOOL;
    fn ModelIndex(hvar: HVAR) -> libc::c_int;
    fn InitdVector(cVectors: libc::c_long) -> *mut libc::c_double;
    fn WriteArray(pfile: *mut FILE, cElems: libc::c_long, rg: *mut libc::c_double);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagMCPREDOUT {
    pub nbrdy: libc::c_long,
    pub pred: *mut libc::c_double,
    pub passflag: libc::c_int,
}
pub type PMCPREDOUT = *mut tagMCPREDOUT;
static mut vszDefOutFilename: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"sim.out\0")
};
static mut vszDefMCOutFilename: [libc::c_char; 10] = unsafe {
    *::std::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(b"simmc.out\0")
};
pub unsafe extern "C" fn SaveOutputs(mut pexp: PEXPERIMENT, mut pdTout: PDOUBLE) {
    static mut rgdInterpStates: PDOUBLE = 0 as *const libc::c_double
        as *mut libc::c_double;
    static mut rgdInterpDeriv: PDOUBLE = 0 as *const libc::c_double
        as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut pmod: PMODELINFO = (*pexp).pmodelinfo;
    let mut pos: POUTSPEC = &mut (*pexp).os;
    extern "C" {
        static mut vrgInputs: [IFN; 0];
    }
    if rgdInterpStates.is_null() || rgdInterpDeriv.is_null() {
        rgdInterpStates = InitdVector(GetNModelVars() as libc::c_long);
        if rgdInterpStates.is_null()
            || {
                rgdInterpDeriv = InitdVector(GetNModelVars() as libc::c_long);
                rgdInterpDeriv.is_null()
            }
        {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"SaveOutputs\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    }
    memcpy(
        rgdInterpStates as *mut libc::c_void,
        (*pmod).pdModelVars as *const libc::c_void,
        ((*pmod).nModelVars as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    CalcDeriv(rgdInterpStates, rgdInterpDeriv, pdTout);
    CalcOutputs(rgdInterpStates, rgdInterpDeriv, pdTout);
    i = 0 as libc::c_int;
    while i < (*pos).nOutputs {
        if *((*pos).piCurrentOut).offset(i as isize)
            < *((*pos).pcOutputTimes).offset(i as isize)
            && *pdTout
                == *(*((*pos).prgdOutputTimes).offset(i as isize))
                    .offset(*((*pos).piCurrentOut).offset(i as isize) as isize)
        {
            let mut dTmp: libc::c_double = 0 as libc::c_int as libc::c_double;
            if IsModelVar(*((*pos).phvar_out).offset(i as isize)) != 0 {
                dTmp = *rgdInterpStates
                    .offset(ModelIndex(*((*pos).phvar_out).offset(i as isize)) as isize);
            } else {
                index = (*((*pos).phvar_out).offset(i as isize)
                    & 0xffff as libc::c_int as libc::c_ulong) as libc::c_int;
                if IsInput(*((*pos).phvar_out).offset(i as isize)) != 0
                    && (*vrgInputs.as_mut_ptr().offset(index as isize)).iType
                        == 5 as libc::c_int
                {
                    j = (*vrgInputs.as_mut_ptr().offset(index as isize)).iDoseCur;
                    if *((*vrgInputs.as_mut_ptr().offset(index as isize)).rgT0s)
                        .offset(j as isize) == (*pexp).dTime
                        && j < (*vrgInputs.as_mut_ptr().offset(index as isize)).nDoses
                    {
                        dTmp = *((*vrgInputs.as_mut_ptr().offset(index as isize)).rgMags)
                            .offset(j as isize);
                    } else {
                        dTmp = 0 as libc::c_int as libc::c_double;
                    }
                } else {
                    dTmp = GetVarValue(*((*pos).phvar_out).offset(i as isize));
                }
            }
            if fabs(dTmp) < 1e-100f64 {
                dTmp = 0.0f64;
            }
            let ref mut fresh0 = *((*pos).piCurrentOut).offset(i as isize);
            let fresh1 = *fresh0;
            *fresh0 = *fresh0 + 1;
            *(*((*pos).prgdOutputVals).offset(i as isize))
                .offset(fresh1 as isize) = dTmp;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn NextOutputTime(
    mut pexp: PEXPERIMENT,
    mut pdTout: PDOUBLE,
    mut piOut: PINT,
) {
    if (*pexp).dTime < (*pexp).dTfinal {
        *piOut += 1;
        if *piOut < (*pexp).os.cDistinctTimes {
            *pdTout = *((*pexp).os.rgdDistinctTimes).offset(*piOut as isize);
        } else {
            *pdTout = (*pexp).dTfinal;
        }
    }
}
pub unsafe extern "C" fn WriteOneMod(mut pData: PVOID, mut pInfo: PVOID) -> libc::c_int {
    let mut pmcvar: PMCVAR = pData as PMCVAR;
    let mut pfile: PFILE = pInfo as PFILE;
    if IsInput((*pmcvar).hvar) == 0 {
        fprintf(pfile, b"%g\t\0" as *const u8 as *const libc::c_char, (*pmcvar).dVal);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn WriteMCHeader(mut pfileOut: PFILE, mut panal: PANALYSIS) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut pmc: PMONTECARLO = &mut (*panal).mc;
    let mut pos: *mut OUTSPEC = 0 as *mut OUTSPEC;
    fprintf(pfileOut, b"Iter\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_long;
    while i < (*pmc).nParms {
        fprintf(
            pfileOut,
            b"\t%s\0" as *const u8 as *const libc::c_char,
            GetVarName((**((*pmc).rgpMCVar).offset(i as isize)).hvar),
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < (*panal).expGlobal.iExp as libc::c_long {
        pos = &mut (**((*panal).rgpExps).as_mut_ptr().offset(i as isize)).os;
        j = 0 as libc::c_int as libc::c_long;
        while j < (*pos).nOutputs as libc::c_long {
            k = 0 as libc::c_int as libc::c_long;
            while k < *((*pos).pcOutputTimes).offset(j as isize) as libc::c_long {
                fprintf(
                    pfileOut,
                    b"\t%s_%ld.%ld\0" as *const u8 as *const libc::c_char,
                    *((*pos).pszOutputNames).offset(j as isize),
                    i + 1 as libc::c_int as libc::c_long,
                    k + 1 as libc::c_int as libc::c_long,
                );
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    fprintf(pfileOut, b"\n\0" as *const u8 as *const libc::c_char);
    fflush(pfileOut);
}
pub unsafe extern "C" fn OpenMCFiles(mut panal: PANALYSIS) {
    let mut pmc: PMONTECARLO = &mut (*panal).mc;
    if (*panal).bCommandLineSpec != 0 {
        free((*pmc).szMCOutfilename as *mut libc::c_void);
        (*panal).bAllocatedFileName = 0 as libc::c_int;
        (*pmc).szMCOutfilename = (*panal).szOutfilename;
    } else if ((*pmc).szMCOutfilename).is_null() {
        (*pmc).szMCOutfilename = vszDefMCOutFilename.as_mut_ptr();
    }
    if (*panal).size > 1 as libc::c_int {
        let mut with_rank: *mut libc::c_char = malloc(
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(
                    (6 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            strlen((*pmc).szMCOutfilename as *const libc::c_char),
                        ),
                ),
        ) as *mut libc::c_char;
        sprintf(
            with_rank,
            b"%04d_%s\0" as *const u8 as *const libc::c_char,
            (*panal).rank,
            (*pmc).szMCOutfilename,
        );
        (*pmc).szMCOutfilename = with_rank;
    }
    if ((*pmc).pfileMCOut).is_null()
        && {
            (*pmc)
                .pfileMCOut = fopen(
                (*pmc).szMCOutfilename as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            ((*pmc).pfileMCOut).is_null()
        }
    {
        ReportError(
            0 as PINPUTBUF,
            (0x8000 as libc::c_int | 0x3 as libc::c_int) as WORD,
            (*pmc).szMCOutfilename,
            b"OpenMCFiles()\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
    WriteMCHeader((*pmc).pfileMCOut, panal);
}
pub unsafe extern "C" fn CloseMCFiles(mut panal: PANALYSIS) {
    fclose((*panal).mc.pfileMCOut);
    printf(
        b"\nWrote results to \"%s\"\n\0" as *const u8 as *const libc::c_char,
        (*panal).mc.szMCOutfilename,
    );
}
pub unsafe extern "C" fn WriteMCOutput(
    mut panal: PANALYSIS,
    mut pmcpredout: PMCPREDOUT,
) {
    let mut pfileMC: PFILE = 0 as *mut FILE;
    let mut pmc: PMONTECARLO = &mut (*panal).mc;
    pfileMC = (*pmc).pfileMCOut;
    fprintf(pfileMC, b"%ld\t\0" as *const u8 as *const libc::c_char, (*panal).mc.lRun);
    WriteArray(pfileMC, (*panal).mc.nParms, (*panal).mc.rgdParms);
    fprintf(pfileMC, b"\t\0" as *const u8 as *const libc::c_char);
    WriteArray(pfileMC, (*pmcpredout).nbrdy, (*pmcpredout).pred);
    fprintf(pfileMC, b"\n\0" as *const u8 as *const libc::c_char);
    fflush(pfileMC);
}
pub unsafe extern "C" fn WriteNormalOutput(mut panal: PANALYSIS, mut pexp: PEXPERIMENT) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut pfile: PFILE = 0 as *mut FILE;
    let mut pos: POUTSPEC = 0 as *mut tagOUTSPEC;
    if panal.is_null() {
        return;
    }
    pos = &mut (*pexp).os;
    if ((*panal).szOutfilename).is_null() {
        (*panal).szOutfilename = vszDefOutFilename.as_mut_ptr();
    }
    if (*panal).size > 1 as libc::c_int {
        let mut with_rank: *mut libc::c_char = malloc(
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(
                    (5 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            strlen((*panal).szOutfilename as *const libc::c_char),
                        ),
                ),
        ) as *mut libc::c_char;
        sprintf(
            with_rank,
            b"%4d%s\0" as *const u8 as *const libc::c_char,
            (*panal).rank,
            (*panal).szOutfilename,
        );
        (*panal).szOutfilename = with_rank;
    }
    if ((*panal).pfileOut).is_null() {
        (*panal)
            .pfileOut = fopen(
            (*panal).szOutfilename as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if ((*panal).pfileOut).is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x3 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                (*panal).szOutfilename,
                0 as PSTR,
            );
        }
    }
    pfile = (*panal).pfileOut;
    fprintf(
        pfile,
        b"Results of Simulation %d\n\n\0" as *const u8 as *const libc::c_char,
        (*pexp).iExp,
    );
    fprintf(pfile, b"Time\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_long;
    while i < (*pos).nOutputs as libc::c_long {
        fprintf(
            pfile,
            b"\t%s\0" as *const u8 as *const libc::c_char,
            *((*pos).pszOutputNames).offset(i as isize),
        );
        i += 1;
        i;
    }
    fprintf(pfile, b"\n\0" as *const u8 as *const libc::c_char);
    j = 0 as libc::c_int as libc::c_long;
    while j < (*pos).nOutputs as libc::c_long {
        *((*pos).piCurrentOut).offset(j as isize) = 0 as libc::c_int;
        j += 1;
        j;
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < (*pos).cDistinctTimes as libc::c_long {
        fprintf(
            pfile,
            b"%g\0" as *const u8 as *const libc::c_char,
            *((*pos).rgdDistinctTimes).offset(i as isize),
        );
        j = 0 as libc::c_int as libc::c_long;
        while j < (*pos).nOutputs as libc::c_long {
            if *((*pos).piCurrentOut).offset(j as isize)
                < *((*pos).pcOutputTimes).offset(j as isize)
                && *((*pos).rgdDistinctTimes).offset(i as isize)
                    == *(*((*pos).prgdOutputTimes).offset(j as isize))
                        .offset(*((*pos).piCurrentOut).offset(j as isize) as isize)
            {
                let ref mut fresh2 = *((*pos).piCurrentOut).offset(j as isize);
                let fresh3 = *fresh2;
                *fresh2 = *fresh2 + 1;
                fprintf(
                    pfile,
                    b"\t%g\0" as *const u8 as *const libc::c_char,
                    *(*((*pos).prgdOutputVals).offset(j as isize))
                        .offset(fresh3 as isize),
                );
            } else {
                fprintf(pfile, b"\t\0" as *const u8 as *const libc::c_char);
            }
            j += 1;
            j;
        }
        fprintf(pfile, b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
        i;
    }
    fprintf(pfile, b"\n\n\0" as *const u8 as *const libc::c_char);
}
