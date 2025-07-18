use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn MakeStringBuffer(pBuf: PINPUTBUF, pStrBuf: PINPUTBUF, sz: PSTR);
    fn NextLex(_: PINPUTBUF, _: *mut libc::c_char, _: PINT);
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn GetFnType(szName: PSTR) -> libc::c_int;
    fn GetInputFn(pibIn: PINPUTBUF, sz: PSTR, pifn: PIFN) -> BOOL;
    fn GetKeywordCode(szKeyword: PSTR, pfContext: PINT) -> libc::c_int;
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
pub type PVOID = *mut libc::c_void;
pub type PDOUBLE = *mut libc::c_double;
pub type PFILE = *mut FILE;
pub type PBUF = PSTR;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagINPUTBUF {
    pub pfileIn: PFILE,
    pub pbufOrg: PBUF,
    pub lBufSize: libc::c_long,
    pub pbufCur: PBUF,
    pub iLineNum: libc::c_int,
    pub iLNPrev: libc::c_int,
    pub cErrors: libc::c_int,
    pub pInfo: PVOID,
    pub pTempInfo: PVOID,
}
pub type INPUTBUF = tagINPUTBUF;
pub type PINPUTBUF = *mut tagINPUTBUF;
pub type PSTRLEX = [libc::c_char; 1023];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagVM {
    pub szName: PSTR,
    pub szEqn: PSTR,
    pub hType: HANDLE,
    pub pvmNextVar: *mut tagVM,
}
pub type VMMAPSTRCT = tagVM;
pub type PVMMAPSTRCT = *mut tagVM;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagINPUTINFO {
    pub wContext: WORD,
    pub bDelays: BOOL,
    pub bforR: BOOL,
    pub bTemplateInUse: BOOL,
    pub szInputFilename: PSTR,
    pub szModGenName: PSTR,
    pub pvmGloVars: PVMMAPSTRCT,
    pub pvmDynEqns: PVMMAPSTRCT,
    pub pvmScaleEqns: PVMMAPSTRCT,
    pub pvmJacobEqns: PVMMAPSTRCT,
    pub pvmCalcOutEqns: PVMMAPSTRCT,
    pub pvmEventEqns: PVMMAPSTRCT,
    pub pvmRootEqns: PVMMAPSTRCT,
    pub pvmCpts: PVMMAPSTRCT,
    pub pvmLocalCpts: PVMMAPSTRCT,
}
pub type PINPUTINFO = *mut tagINPUTINFO;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagIFN {
    pub iType: libc::c_int,
    pub bOn: BOOL,
    pub dMag: libc::c_double,
    pub dTper: libc::c_double,
    pub dT0: libc::c_double,
    pub dTexp: libc::c_double,
    pub dDecay: libc::c_double,
    pub dVal: libc::c_double,
    pub dTStartPeriod: libc::c_double,
    pub hMag: HANDLE,
    pub hTper: HANDLE,
    pub hT0: HANDLE,
    pub hTexp: HANDLE,
    pub hDecay: HANDLE,
    pub nDoses: libc::c_int,
    pub iDoseCur: libc::c_int,
    pub rgT0s: PDOUBLE,
    pub rgTexps: PDOUBLE,
    pub rgMags: PDOUBLE,
}
pub type IFN = tagIFN;
pub type PIFN = *mut tagIFN;
pub static mut vrgszMathFuncs: [PSTR; 52] = [
    b"acos\0" as *const u8 as *const libc::c_char as PSTR,
    b"asin\0" as *const u8 as *const libc::c_char as PSTR,
    b"atan\0" as *const u8 as *const libc::c_char as PSTR,
    b"atan2\0" as *const u8 as *const libc::c_char as PSTR,
    b"ceil\0" as *const u8 as *const libc::c_char as PSTR,
    b"cos\0" as *const u8 as *const libc::c_char as PSTR,
    b"cosh\0" as *const u8 as *const libc::c_char as PSTR,
    b"exp\0" as *const u8 as *const libc::c_char as PSTR,
    b"fabs\0" as *const u8 as *const libc::c_char as PSTR,
    b"floor\0" as *const u8 as *const libc::c_char as PSTR,
    b"fmax\0" as *const u8 as *const libc::c_char as PSTR,
    b"fmin\0" as *const u8 as *const libc::c_char as PSTR,
    b"fmod\0" as *const u8 as *const libc::c_char as PSTR,
    b"log\0" as *const u8 as *const libc::c_char as PSTR,
    b"log10\0" as *const u8 as *const libc::c_char as PSTR,
    b"pow\0" as *const u8 as *const libc::c_char as PSTR,
    b"sin\0" as *const u8 as *const libc::c_char as PSTR,
    b"sinh\0" as *const u8 as *const libc::c_char as PSTR,
    b"sqrt\0" as *const u8 as *const libc::c_char as PSTR,
    b"tan\0" as *const u8 as *const libc::c_char as PSTR,
    b"tanh\0" as *const u8 as *const libc::c_char as PSTR,
    b"CDFNormal\0" as *const u8 as *const libc::c_char as PSTR,
    b"erfc\0" as *const u8 as *const libc::c_char as PSTR,
    b"lnDFNormal\0" as *const u8 as *const libc::c_char as PSTR,
    b"lnGamma\0" as *const u8 as *const libc::c_char as PSTR,
    b"piecewise\0" as *const u8 as *const libc::c_char as PSTR,
    b"and\0" as *const u8 as *const libc::c_char as PSTR,
    b"leq\0" as *const u8 as *const libc::c_char as PSTR,
    b"lt\0" as *const u8 as *const libc::c_char as PSTR,
    b"BetaRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"BinomialBetaRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"BinomialRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"CauchyRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"Chi2Random\0" as *const u8 as *const libc::c_char as PSTR,
    b"ExpRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"GammaRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"GetSeed\0" as *const u8 as *const libc::c_char as PSTR,
    b"GGammaRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"InvGGammaRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"LogNormalRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"LogUniformRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"NegativeBinomialRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"NormalRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"PiecewiseRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"PoissonRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"SetSeed\0" as *const u8 as *const libc::c_char as PSTR,
    b"StudentTRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"TruncInvGGammaRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"TruncLogNormalRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"TruncNormalRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"UniformRandom\0" as *const u8 as *const libc::c_char as PSTR,
    b"\0" as *const u8 as *const libc::c_char as PSTR,
];
pub static mut vszHasInitializer: [libc::c_char; 27] = unsafe {
    *::std::mem::transmute::<
        &[u8; 27],
        &mut [libc::c_char; 27],
    >(b"0.0; /* Redefined later */\0")
};
pub unsafe extern "C" fn IsMathFunc(mut sz: PSTR) -> BOOL {
    let mut i: libc::c_int = 0 as libc::c_int;
    while *vrgszMathFuncs[i as usize] as libc::c_int != 0
        && strcmp(
            vrgszMathFuncs[i as usize] as *const libc::c_char,
            sz as *const libc::c_char,
        ) != 0
    {
        i += 1;
        i;
    }
    return *vrgszMathFuncs[i as usize] as BOOL;
}
pub unsafe extern "C" fn IsDelayFunc(mut bDelays: *mut BOOL, mut sz: PSTR) -> BOOL {
    let mut bIsDelay: BOOL = 0;
    bIsDelay = (strcmp(
        b"CalcDelay\0" as *const u8 as *const libc::c_char,
        sz as *const libc::c_char,
    ) == 0) as libc::c_int;
    if bIsDelay != 0 && *bDelays == 0 {
        *bDelays = 1 as libc::c_int;
    }
    return bIsDelay;
}
pub unsafe extern "C" fn VerifyEqn(mut pibIn: PINPUTBUF, mut szEqn: PSTR) -> BOOL {
    let mut ibDummy: INPUTBUF = INPUTBUF {
        pfileIn: 0 as *mut FILE,
        pbufOrg: 0 as *mut libc::c_char,
        lBufSize: 0,
        pbufCur: 0 as *mut libc::c_char,
        iLineNum: 0,
        iLNPrev: 0,
        cErrors: 0,
        pInfo: 0 as *mut libc::c_void,
        pTempInfo: 0 as *mut libc::c_void,
    };
    let mut szLex: PSTRLEX = [0; 1023];
    let mut iType: libc::c_int = 0;
    let mut fContext: libc::c_int = 0;
    let mut bReturn: BOOL = 1 as libc::c_int;
    let mut bOK: BOOL = 1 as libc::c_int;
    let mut pinfo: PINPUTINFO = 0 as *mut tagINPUTINFO;
    pinfo = (*pibIn).pInfo as PINPUTINFO;
    MakeStringBuffer(pibIn, &mut ibDummy, szEqn);
    while !((&mut ibDummy as *mut INPUTBUF).is_null()
        || ((ibDummy.pbufCur).is_null() || *ibDummy.pbufCur == 0)
            && ((ibDummy.pfileIn).is_null() || feof(ibDummy.pfileIn) != 0))
    {
        NextLex(&mut ibDummy, szLex.as_mut_ptr(), &mut iType);
        match iType {
            1 => {
                iType = GetKeywordCode(szLex.as_mut_ptr(), &mut fContext);
                if iType != 0 {
                    bOK = (iType == 20 as libc::c_int
                        && fContext as libc::c_uint & (*pinfo).wContext != 0)
                        as libc::c_int;
                    if bOK == 0 {
                        ReportError(
                            pibIn,
                            (0x100 as libc::c_int + 1 as libc::c_int
                                | 0x8000 as libc::c_int) as WORD,
                            szLex.as_mut_ptr(),
                            0 as PSTR,
                        );
                    }
                } else {
                    if GetFnType(szLex.as_mut_ptr()) != 0 {
                        ReportError(
                            pibIn,
                            (0x100 as libc::c_int + 1 as libc::c_int
                                | 0x8000 as libc::c_int) as WORD,
                            szLex.as_mut_ptr(),
                            0 as PSTR,
                        );
                    }
                    bOK = (GetVarType((*pinfo).pvmGloVars, szLex.as_mut_ptr()) != 0
                        || IsMathFunc(szLex.as_mut_ptr()) != 0
                        || IsDelayFunc(&mut (*pinfo).bDelays, szLex.as_mut_ptr()) != 0
                        || ((*pinfo).wContext == 0x2 as libc::c_int as libc::c_uint
                            || (*pinfo).wContext == 0x3 as libc::c_int as libc::c_uint
                            || (*pinfo).wContext == 0x5 as libc::c_int as libc::c_uint)
                            && !(strcmp(
                                szLex.as_mut_ptr(),
                                b"t\0" as *const u8 as *const libc::c_char,
                            ) != 0
                                && strcmp(
                                    szLex.as_mut_ptr(),
                                    b"time\0" as *const u8 as *const libc::c_char,
                                ) != 0)) as libc::c_int;
                    if bOK == 0 {
                        ReportError(
                            pibIn,
                            (0x100 as libc::c_int + 6 as libc::c_int
                                | 0x8000 as libc::c_int) as WORD,
                            szLex.as_mut_ptr(),
                            0 as PSTR,
                        );
                    }
                }
            }
            32 => {
                if (szLex[0 as libc::c_int as usize] as libc::c_int == '!' as i32
                    || szLex[0 as libc::c_int as usize] as libc::c_int == '=' as i32)
                    && strlen(szLex.as_mut_ptr()) == 1 as libc::c_int as libc::c_ulong
                {
                    ReportError(
                        pibIn,
                        0x11 as libc::c_int as WORD,
                        szLex.as_mut_ptr(),
                        b".. in equation\0" as *const u8 as *const libc::c_char as PSTR,
                    );
                    bOK = 0 as libc::c_int;
                }
            }
            8 => {
                ReportError(
                    pibIn,
                    0x11 as libc::c_int as WORD,
                    szLex.as_mut_ptr(),
                    b".. in equation\0" as *const u8 as *const libc::c_char as PSTR,
                );
                bOK = 0 as libc::c_int;
            }
            2 | 4 => {}
            _ => {
                ReportError(
                    pibIn,
                    0x11 as libc::c_int as WORD,
                    szLex.as_mut_ptr(),
                    b".. in equation\0" as *const u8 as *const libc::c_char as PSTR,
                );
                bOK = 0 as libc::c_int;
            }
        }
        bReturn = (bReturn != 0 && bOK != 0) as libc::c_int;
    }
    return bReturn;
}
pub unsafe extern "C" fn AddEquation(
    mut ppvm: *mut PVMMAPSTRCT,
    mut szName: PSTR,
    mut szEqn: PSTR,
    mut hType: HANDLE,
) {
    let mut pvmNew: PVMMAPSTRCT = 0 as *mut tagVM;
    if ppvm.is_null() || szName.is_null() {
        return;
    }
    pvmNew = malloc(::std::mem::size_of::<VMMAPSTRCT>() as libc::c_ulong) as PVMMAPSTRCT;
    if !pvmNew.is_null() {
        (*pvmNew).szName = CopyString(szName);
        (*pvmNew).szEqn = CopyString(szEqn);
        (*pvmNew).hType = hType;
        (*pvmNew).pvmNextVar = *ppvm;
        *ppvm = pvmNew;
    } else {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szName,
            b"* .. creating new equation in AddEquation\0" as *const u8
                as *const libc::c_char as PSTR,
        );
    };
}
pub unsafe extern "C" fn CopyString(mut szOrg: PSTR) -> PSTR {
    let mut szBuf: PSTR = 0 as *mut libc::c_char;
    if !szOrg.is_null() {
        szBuf = malloc(
            (strlen(szOrg as *const libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as PSTR;
        if !szBuf.is_null() {
            return strcpy(szBuf, szOrg as *const libc::c_char)
        } else {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                szOrg,
                b"* .. defining equation in CopyString\0" as *const u8
                    as *const libc::c_char as PSTR,
            );
        }
    }
    return 0 as PSTR;
}
pub unsafe extern "C" fn SetEquation(mut pvm: PVMMAPSTRCT, mut szEqn: PSTR) {
    if pvm.is_null() || szEqn.is_null() {
        return;
    }
    if !((*pvm).szEqn).is_null() {
        free((*pvm).szEqn as *mut libc::c_void);
    }
    (*pvm).szEqn = CopyString(szEqn);
}
pub unsafe extern "C" fn SetVarType(
    mut pvm: PVMMAPSTRCT,
    mut szName: PSTR,
    mut hType: HANDLE,
) {
    while !pvm.is_null()
        && strcmp(szName as *const libc::c_char, (*pvm).szName as *const libc::c_char)
            != 0
    {
        pvm = (*pvm).pvmNextVar;
    }
    if !pvm.is_null() {
        (*pvm).hType = hType;
    }
}
pub unsafe extern "C" fn GetVarPTR(
    mut pvm: PVMMAPSTRCT,
    mut szName: PSTR,
) -> PVMMAPSTRCT {
    while !pvm.is_null()
        && strcmp(szName as *const libc::c_char, (*pvm).szName as *const libc::c_char)
            != 0
    {
        pvm = (*pvm).pvmNextVar;
    }
    return pvm;
}
pub unsafe extern "C" fn GetVarType(
    mut pvm: PVMMAPSTRCT,
    mut szName: PSTR,
) -> libc::c_int {
    pvm = GetVarPTR(pvm, szName);
    return (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
}
pub unsafe extern "C" fn CalculateVarHandle(
    mut pvm: PVMMAPSTRCT,
    mut sz: PSTR,
) -> HANDLE {
    let mut pvmVar: PVMMAPSTRCT = 0 as *mut tagVM;
    let mut cSameType: libc::c_int = 0 as libc::c_int;
    pvmVar = GetVarPTR(pvm, sz);
    pvm = pvmVar;
    if !pvm.is_null() {
        pvm = (*pvm).pvmNextVar;
    }
    while !pvm.is_null() && (*pvm).hType == (*pvmVar).hType {
        cSameType += 1;
        cSameType;
        pvm = (*pvm).pvmNextVar;
    }
    return if !pvmVar.is_null() {
        (*pvmVar).hType | cSameType as HANDLE
    } else {
        0 as libc::c_int as libc::c_ulong
    };
}
pub unsafe extern "C" fn DefineGlobalVar(
    mut pibIn: PINPUTBUF,
    mut pvm: PVMMAPSTRCT,
    mut szName: PSTR,
    mut szEqn: PSTR,
    mut hType: HANDLE,
) {
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    if strcmp(
        szName as *const libc::c_char,
        b"Inline\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        AddEquation(
            &mut (*pinfo).pvmGloVars,
            szName,
            szEqn,
            0xa0000 as libc::c_int as HANDLE,
        );
    } else {
        let mut current_block_12: u64;
        match hType {
            0 => {
                AddEquation(
                    &mut (*pinfo).pvmGloVars,
                    szName,
                    szEqn,
                    0x40000 as libc::c_int as HANDLE,
                );
                current_block_12 = 12039483399334584727;
            }
            131072 | 196608 | 65536 => {
                if !pvm.is_null() {} else {
                    __assert_fail(
                        b"pvm != NULL\0" as *const u8 as *const libc::c_char,
                        b"modd.c\0" as *const u8 as *const libc::c_char,
                        378 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 65],
                            &[libc::c_char; 65],
                        >(
                            b"void DefineGlobalVar(PINPUTBUF, PVMMAPSTRCT, PSTR, PSTR, HANDLE)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_4992: {
                    if !pvm.is_null() {} else {
                        __assert_fail(
                            b"pvm != NULL\0" as *const u8 as *const libc::c_char,
                            b"modd.c\0" as *const u8 as *const libc::c_char,
                            378 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<
                                &[u8; 65],
                                &[libc::c_char; 65],
                            >(
                                b"void DefineGlobalVar(PINPUTBUF, PVMMAPSTRCT, PSTR, PSTR, HANDLE)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if ((*pvm).szEqn).is_null() {
                    if hType == 0x20000 as libc::c_int as libc::c_ulong {
                        let mut pifn: PIFN = malloc(
                            ::std::mem::size_of::<IFN>() as libc::c_ulong,
                        ) as PIFN;
                        if GetInputFn(pibIn, szEqn, pifn) != 0 {
                            (*pvm).szEqn = pifn as PSTR;
                        } else {
                            (*pvm).szEqn = 0 as PSTR;
                        }
                    } else {
                        (*pvm).szEqn = vszHasInitializer.as_mut_ptr();
                        AddEquation(&mut (*pinfo).pvmGloVars, szName, szEqn, hType);
                    }
                    current_block_12 = 12039483399334584727;
                } else {
                    current_block_12 = 13613537577709841742;
                }
            }
            262144 => {
                current_block_12 = 13613537577709841742;
            }
            _ => {
                ReportError(
                    pibIn,
                    (0x100 as libc::c_int + 1 as libc::c_int) as WORD,
                    szName,
                    0 as PSTR,
                );
                current_block_12 = 12039483399334584727;
            }
        }
        match current_block_12 {
            13613537577709841742 => {
                ReportError(
                    pibIn,
                    (0x100 as libc::c_int + 3 as libc::c_int | 0x4000 as libc::c_int)
                        as WORD,
                    szName,
                    0 as PSTR,
                );
            }
            _ => {}
        }
    };
}
pub unsafe extern "C" fn DefineDynamicsEqn(
    mut pibIn: PINPUTBUF,
    mut szName: PSTR,
    mut szEqn: PSTR,
    mut hType: HANDLE,
) {
    let mut hNewType: HANDLE = if hType != 0 {
        hType
    } else {
        0x50000 as libc::c_int as libc::c_ulong
    };
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    hNewType |= 0xf000 as libc::c_int as libc::c_ulong;
    if strcmp(
        szName as *const libc::c_char,
        b"Inline\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        AddEquation(
            &mut (*pinfo).pvmDynEqns,
            szName,
            szEqn,
            0xa0000 as libc::c_int as HANDLE,
        );
    } else {
        let mut current_block_6: u64;
        match hType {
            0 => {
                AddEquation(&mut (*pinfo).pvmGloVars, szName, 0 as PSTR, hNewType);
                current_block_6 = 6212760239852099662;
            }
            327680 => {
                current_block_6 = 6212760239852099662;
            }
            786432 => {
                AddEquation(&mut (*pinfo).pvmGloVars, szName, 0 as PSTR, hNewType);
                current_block_6 = 13536709405535804910;
            }
            589824 => {
                current_block_6 = 7083505803230017759;
            }
            65536 | 196608 => {
                current_block_6 = 7083505803230017759;
            }
            131072 | 262144 => {
                ReportError(
                    pibIn,
                    (0x100 as libc::c_int + 3 as libc::c_int | 0x4000 as libc::c_int)
                        as WORD,
                    szName,
                    b"  Inputs and parameters cannot be assigned in Dynamics\n\0"
                        as *const u8 as *const libc::c_char as PSTR,
                );
                current_block_6 = 13536709405535804910;
            }
            _ => {
                current_block_6 = 13536709405535804910;
            }
        }
        match current_block_6 {
            6212760239852099662 => {
                AddEquation(&mut (*pinfo).pvmDynEqns, szName, szEqn, hNewType);
            }
            7083505803230017759 => {
                AddEquation(&mut (*pinfo).pvmDynEqns, szName, szEqn, hNewType);
            }
            _ => {}
        }
    }
    (*pibIn).iLNPrev = (*pibIn).iLineNum;
}
pub unsafe extern "C" fn DefineScaleEqn(
    mut pibIn: PINPUTBUF,
    mut szName: PSTR,
    mut szEqn: PSTR,
    mut hType: HANDLE,
) {
    let mut hNewType: HANDLE = if hType != 0 {
        hType
    } else {
        0x60000 as libc::c_int as libc::c_ulong
    };
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    if (*pibIn).iLineNum - (*pibIn).iLNPrev - 1 as libc::c_int != 0 {
        hNewType |= 0xf000 as libc::c_int as libc::c_ulong;
    }
    if strcmp(
        szName as *const libc::c_char,
        b"Inline\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        AddEquation(
            &mut (*pinfo).pvmScaleEqns,
            szName,
            szEqn,
            0xa0000 as libc::c_int as HANDLE,
        );
    } else {
        if hType == 0 {
            AddEquation(&mut (*pinfo).pvmGloVars, szName, 0 as PSTR, hNewType);
        }
        if hType & 0x60000 as libc::c_int as libc::c_ulong != 0
            || (GetVarPTR((*pinfo).pvmScaleEqns, szName)).is_null()
        {
            AddEquation(&mut (*pinfo).pvmScaleEqns, szName, szEqn, hNewType);
        } else {
            ReportError(
                pibIn,
                (0x100 as libc::c_int + 3 as libc::c_int | 0x4000 as libc::c_int)
                    as WORD,
                szName,
                b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
            );
        }
    }
    (*pibIn).iLNPrev = (*pibIn).iLineNum;
}
pub unsafe extern "C" fn DefineCalcOutEqn(
    mut pibIn: PINPUTBUF,
    mut szName: PSTR,
    mut szEqn: PSTR,
    mut hType: HANDLE,
) {
    let mut hNewType: HANDLE = if hType != 0 {
        hType
    } else {
        0x80000 as libc::c_int as libc::c_ulong
    };
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    if (*pibIn).iLineNum - (*pibIn).iLNPrev - 1 as libc::c_int != 0 {
        hNewType |= 0xf000 as libc::c_int as libc::c_ulong;
    }
    if strcmp(
        szName as *const libc::c_char,
        b"Inline\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        AddEquation(
            &mut (*pinfo).pvmCalcOutEqns,
            szName,
            szEqn,
            0xa0000 as libc::c_int as HANDLE,
        );
    } else {
        if hType == 0 {
            AddEquation(&mut (*pinfo).pvmGloVars, szName, 0 as PSTR, hNewType);
        }
        AddEquation(&mut (*pinfo).pvmCalcOutEqns, szName, szEqn, hNewType);
    }
    (*pibIn).iLNPrev = (*pibIn).iLineNum;
}
pub unsafe extern "C" fn DefineJacobEqn(
    mut pibIn: PINPUTBUF,
    mut szName: PSTR,
    mut szEqn: PSTR,
    mut hType: HANDLE,
) {
    let mut hNewType: HANDLE = if hType != 0 {
        hType
    } else {
        0x70000 as libc::c_int as libc::c_ulong
    };
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    if (*pibIn).iLineNum - (*pibIn).iLNPrev - 1 as libc::c_int != 0 {
        hNewType |= 0xf000 as libc::c_int as libc::c_ulong;
    }
    if strcmp(
        szName as *const libc::c_char,
        b"Inline\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        AddEquation(
            &mut (*pinfo).pvmJacobEqns,
            szName,
            szEqn,
            0xa0000 as libc::c_int as HANDLE,
        );
    } else {
        if hType == 0 {
            AddEquation(&mut (*pinfo).pvmGloVars, szName, 0 as PSTR, hNewType);
        }
        if hType & 0x70000 as libc::c_int as libc::c_ulong != 0
            || (GetVarPTR((*pinfo).pvmJacobEqns, szName)).is_null()
        {
            AddEquation(&mut (*pinfo).pvmJacobEqns, szName, szEqn, hNewType);
        } else {
            ReportError(
                pibIn,
                (0x100 as libc::c_int + 3 as libc::c_int | 0x4000 as libc::c_int)
                    as WORD,
                szName,
                b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
            );
        }
    }
    (*pibIn).iLNPrev = (*pibIn).iLineNum;
}
pub unsafe extern "C" fn DefineEventEqn(
    mut pibIn: PINPUTBUF,
    mut szName: PSTR,
    mut szEqn: PSTR,
    mut hType: HANDLE,
) {
    let mut hNewType: HANDLE = if hType != 0 {
        hType
    } else {
        0x70000 as libc::c_int as libc::c_ulong
    };
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    if (*pibIn).iLineNum - (*pibIn).iLNPrev - 1 as libc::c_int != 0 {
        hNewType |= 0xf000 as libc::c_int as libc::c_ulong;
    }
    if strcmp(
        szName as *const libc::c_char,
        b"Inline\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        AddEquation(
            &mut (*pinfo).pvmEventEqns,
            szName,
            szEqn,
            0xa0000 as libc::c_int as HANDLE,
        );
    } else {
        if hType == 0 {
            AddEquation(&mut (*pinfo).pvmGloVars, szName, 0 as PSTR, hNewType);
        }
        if hType & 0x70000 as libc::c_int as libc::c_ulong != 0
            || (GetVarPTR((*pinfo).pvmEventEqns, szName)).is_null()
        {
            AddEquation(&mut (*pinfo).pvmEventEqns, szName, szEqn, hNewType);
        } else {
            ReportError(
                pibIn,
                (0x100 as libc::c_int + 3 as libc::c_int | 0x4000 as libc::c_int)
                    as WORD,
                szName,
                b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
            );
        }
    }
    (*pibIn).iLNPrev = (*pibIn).iLineNum;
}
pub unsafe extern "C" fn DefineRootEqn(
    mut pibIn: PINPUTBUF,
    mut szName: PSTR,
    mut szEqn: PSTR,
    mut hType: HANDLE,
) {
    let mut hNewType: HANDLE = if hType != 0 {
        hType
    } else {
        0x70000 as libc::c_int as libc::c_ulong
    };
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    if (*pibIn).iLineNum - (*pibIn).iLNPrev - 1 as libc::c_int != 0 {
        hNewType |= 0xf000 as libc::c_int as libc::c_ulong;
    }
    if strcmp(
        szName as *const libc::c_char,
        b"Inline\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        AddEquation(
            &mut (*pinfo).pvmRootEqns,
            szName,
            szEqn,
            0xa0000 as libc::c_int as HANDLE,
        );
    } else {
        if hType == 0 {
            AddEquation(&mut (*pinfo).pvmGloVars, szName, 0 as PSTR, hNewType);
        }
        if hType & 0x70000 as libc::c_int as libc::c_ulong != 0
            || (GetVarPTR((*pinfo).pvmRootEqns, szName)).is_null()
        {
            AddEquation(&mut (*pinfo).pvmRootEqns, szName, szEqn, hNewType);
        } else {
            ReportError(
                pibIn,
                (0x100 as libc::c_int + 3 as libc::c_int | 0x4000 as libc::c_int)
                    as WORD,
                szName,
                b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
            );
        }
    }
    (*pibIn).iLNPrev = (*pibIn).iLineNum;
}
pub unsafe extern "C" fn DefineVariable(
    mut pibIn: PINPUTBUF,
    mut szName: PSTR,
    mut szEqn: PSTR,
    mut iKWCode: libc::c_int,
) {
    let mut pvm: PVMMAPSTRCT = 0 as *mut tagVM;
    let mut hGloVarType: HANDLE = 0;
    let mut pinfo: PINPUTINFO = 0 as *mut tagINPUTINFO;
    pinfo = (*pibIn).pInfo as PINPUTINFO;
    if (*pinfo).wContext != 0x4000 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"pinfo->wContext != CN_END\0" as *const u8 as *const libc::c_char,
            b"modd.c\0" as *const u8 as *const libc::c_char,
            661 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void DefineVariable(PINPUTBUF, PSTR, PSTR, int)\0"))
                .as_ptr(),
        );
    }
    'c_6973: {
        if (*pinfo).wContext != 0x4000 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"pinfo->wContext != CN_END\0" as *const u8 as *const libc::c_char,
                b"modd.c\0" as *const u8 as *const libc::c_char,
                661 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void DefineVariable(PINPUTBUF, PSTR, PSTR, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if szName.is_null() || szEqn.is_null() {
        return;
    }
    pvm = GetVarPTR((*pinfo).pvmGloVars, szName);
    hGloVarType = if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if (*pinfo).wContext == 0x2 as libc::c_int as libc::c_uint
        && (hGloVarType == 0x60000 as libc::c_int as libc::c_ulong
            || hGloVarType == 0x70000 as libc::c_int as libc::c_ulong
            || hGloVarType == 0x70300 as libc::c_int as libc::c_ulong
            || hGloVarType == 0x70600 as libc::c_int as libc::c_ulong
            || hGloVarType == 0x80000 as libc::c_int as libc::c_ulong)
        || (*pinfo).wContext == 0x3 as libc::c_int as libc::c_uint
            && (hGloVarType == 0x50000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70300 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70600 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x80000 as libc::c_int as libc::c_ulong)
        || (*pinfo).wContext == 0x4 as libc::c_int as libc::c_uint
            && (hGloVarType == 0x50000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x60000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70300 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70600 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x80000 as libc::c_int as libc::c_ulong)
        || (*pinfo).wContext == 0x6 as libc::c_int as libc::c_uint
            && (hGloVarType == 0x50000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x60000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70600 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x80000 as libc::c_int as libc::c_ulong)
        || (*pinfo).wContext == 0x7 as libc::c_int as libc::c_uint
            && (hGloVarType == 0x50000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x60000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70300 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x80000 as libc::c_int as libc::c_ulong)
        || (*pinfo).wContext == 0x5 as libc::c_int as libc::c_uint
            && (hGloVarType == 0x50000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x60000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70300 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70600 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x70000 as libc::c_int as libc::c_ulong)
    {
        pvm = 0 as PVMMAPSTRCT;
        hGloVarType = if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        };
    }
    if iKWCode != 30 as libc::c_int
        && (hGloVarType != 0x20000 as libc::c_int as libc::c_ulong
            || (*pinfo).wContext != 0x1 as libc::c_int as libc::c_uint)
        && VerifyEqn(pibIn, szEqn) == 0
    {
        return;
    }
    match (*pinfo).wContext {
        1 => {
            DefineGlobalVar(pibIn, pvm, szName, szEqn, hGloVarType);
        }
        2 => {
            if iKWCode == 20 as libc::c_int {
                DefineDynamicsEqn(
                    pibIn,
                    szName,
                    szEqn,
                    0x90000 as libc::c_int as HANDLE,
                );
            } else if iKWCode == 43 as libc::c_int {
                DefineDynamicsEqn(
                    pibIn,
                    szName,
                    szEqn,
                    0xc0000 as libc::c_int as HANDLE,
                );
            } else {
                DefineDynamicsEqn(pibIn, szName, szEqn, hGloVarType);
            }
            if hGloVarType == 0x10000 as libc::c_int as libc::c_ulong
                && iKWCode != 20 as libc::c_int
            {
                ReportError(
                    pibIn,
                    (0x100 as libc::c_int + 3 as libc::c_int | 0x4000 as libc::c_int)
                        as WORD,
                    szName,
                    b"Non-standard assignment in Dynamics section. Potential state discontinuity.\n\0"
                        as *const u8 as *const libc::c_char as PSTR,
                );
            }
        }
        4 => {
            DefineJacobEqn(pibIn, szName, szEqn, hGloVarType);
        }
        6 => {
            DefineEventEqn(pibIn, szName, szEqn, hGloVarType);
        }
        7 => {
            DefineRootEqn(pibIn, szName, szEqn, hGloVarType);
        }
        3 => {
            DefineScaleEqn(pibIn, szName, szEqn, hGloVarType);
        }
        5 => {
            if hGloVarType == 0x30000 as libc::c_int as libc::c_ulong
                || hGloVarType == 0 as libc::c_int as libc::c_ulong
                || hGloVarType == 0x80000 as libc::c_int as libc::c_ulong
                || iKWCode == 30 as libc::c_int
            {
                DefineCalcOutEqn(pibIn, szName, szEqn, hGloVarType);
            } else {
                ReportError(
                    pibIn,
                    (0x100 as libc::c_int + 1 as libc::c_int | 0x8000 as libc::c_int)
                        as WORD,
                    szName,
                    b"Only outputs and local variables can be defined in CalcOutputs{} section.\0"
                        as *const u8 as *const libc::c_char as PSTR,
                );
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn DeclareModelVar(
    mut pibIn: PINPUTBUF,
    mut szName: PSTR,
    mut iKWCode: libc::c_int,
) {
    let mut hType: HANDLE = 0;
    let mut pinfo: PINPUTINFO = 0 as *mut tagINPUTINFO;
    pinfo = (*pibIn).pInfo as PINPUTINFO;
    if iKWCode == 1 as libc::c_int || iKWCode == 2 as libc::c_int
        || iKWCode == 3 as libc::c_int || iKWCode == 42 as libc::c_int
    {} else {
        __assert_fail(
            b"iKWCode == KM_STATES || iKWCode == KM_INPUTS || iKWCode == KM_OUTPUTS || iKWCode == KM_COMPARTMENTS\0"
                as *const u8 as *const libc::c_char,
            b"modd.c\0" as *const u8 as *const libc::c_char,
            801 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"void DeclareModelVar(PINPUTBUF, PSTR, int)\0"))
                .as_ptr(),
        );
    }
    'c_4469: {
        if iKWCode == 1 as libc::c_int || iKWCode == 2 as libc::c_int
            || iKWCode == 3 as libc::c_int || iKWCode == 42 as libc::c_int
        {} else {
            __assert_fail(
                b"iKWCode == KM_STATES || iKWCode == KM_INPUTS || iKWCode == KM_OUTPUTS || iKWCode == KM_COMPARTMENTS\0"
                    as *const u8 as *const libc::c_char,
                b"modd.c\0" as *const u8 as *const libc::c_char,
                801 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"void DeclareModelVar(PINPUTBUF, PSTR, int)\0"))
                    .as_ptr(),
            );
        }
    };
    iKWCode = if iKWCode == 1 as libc::c_int {
        0x10000 as libc::c_int
    } else if iKWCode == 2 as libc::c_int {
        0x20000 as libc::c_int
    } else if iKWCode == 3 as libc::c_int {
        0x30000 as libc::c_int
    } else {
        0xb0000 as libc::c_int
    };
    hType = GetVarType((*pinfo).pvmGloVars, szName) as HANDLE;
    if hType == 0 {
        if iKWCode == 0xb0000 as libc::c_int {
            AddEquation(&mut (*pinfo).pvmCpts, szName, 0 as PSTR, iKWCode as HANDLE);
        } else {
            AddEquation(&mut (*pinfo).pvmGloVars, szName, 0 as PSTR, iKWCode as HANDLE);
        }
    } else if hType == iKWCode as libc::c_ulong {
        ReportError(
            pibIn,
            (0x100 as libc::c_int + 2 as libc::c_int | 0x4000 as libc::c_int) as WORD,
            szName,
            0 as PSTR,
        );
    } else if hType == 0x40000 as libc::c_int as libc::c_ulong {
        ReportError(
            pibIn,
            (0x100 as libc::c_int + 2 as libc::c_int | 0x4000 as libc::c_int) as WORD,
            szName,
            b"Model variable initialized before declaration\0" as *const u8
                as *const libc::c_char as PSTR,
        );
        SetVarType((*pinfo).pvmGloVars, szName, iKWCode as HANDLE);
    } else {
        ReportError(
            pibIn,
            (0x100 as libc::c_int + 2 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szName,
            0 as PSTR,
        );
    };
}
