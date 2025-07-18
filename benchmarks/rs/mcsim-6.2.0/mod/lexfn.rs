use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn NextLex(_: PINPUTBUF, _: *mut libc::c_char, _: PINT);
    fn EGetPunct(pibIn: PINPUTBUF, szLex: PSTR, chPunct: libc::c_char) -> libc::c_int;
    fn GetFuncArgs(
        pibIn: PINPUTBUF,
        nArgs: libc::c_int,
        rgiArgTypes: *mut libc::c_int,
        szArgs: PSTR,
        rgiLowerB: *mut libc::c_long,
        rgiUpperB: *mut libc::c_long,
    ) -> BOOL;
    fn ENextLex(_: PINPUTBUF, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn MakeStringBuffer(pBuf: PINPUTBUF, pStrBuf: PINPUTBUF, sz: PSTR);
    fn GetOptPunct(_: PINPUTBUF, _: PSTR, _: libc::c_char) -> libc::c_int;
    fn MyStrcmp(sz1: *const libc::c_char, sz2: *const libc::c_char) -> libc::c_int;
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn CalculateVarHandle(pvm: PVMMAPSTRCT, sz: PSTR) -> HANDLE;
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
pub type PIFN = *mut tagIFN;
pub type PIFM = *mut tagINPUTFUNCTIONMAP;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagINPUTFUNCTIONMAP {
    pub szName: PSTR,
    pub iIFNType: libc::c_int,
}
pub type IFM = tagINPUTFUNCTIONMAP;
pub type PVMMAPSTRCT = *mut tagVM;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagVM {
    pub szName: PSTR,
    pub szEqn: PSTR,
    pub hType: HANDLE,
    pub pvmNextVar: *mut tagVM,
}
pub type PINPUTINFO = *mut tagINPUTINFO;
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut vpvmGlo: PVMMAPSTRCT = 0 as *const tagVM as *mut tagVM;
pub static mut vrgifmMap: [IFM; 5] = [
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"PerDose\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"PerRate\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"PerExp\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"NDoses\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 0 as libc::c_int,
        };
        init
    },
];
pub unsafe extern "C" fn GetFnType(mut szName: PSTR) -> libc::c_int {
    let mut pifm: PIFM = &mut *vrgifmMap.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut IFM;
    while *(*pifm).szName as libc::c_int != 0
        && MyStrcmp(szName as *const libc::c_char, (*pifm).szName as *const libc::c_char)
            != 0
    {
        pifm = pifm.offset(1);
        pifm;
    }
    return (*pifm).iIFNType;
}
pub unsafe extern "C" fn InitIFN(mut pifn: PIFN) {
    (*pifn).dTStartPeriod = 0.0f64;
    (*pifn).bOn = 0 as libc::c_int;
    (*pifn).dMag = 0.0f64;
    (*pifn).dTper = 0.0f64;
    (*pifn).dT0 = 0.0f64;
    (*pifn).dTexp = 0.0f64;
    (*pifn).dDecay = 0.0f64;
    (*pifn).dVal = 0.0f64;
    (*pifn).nDoses = 0 as libc::c_int;
    (*pifn).hMag = 0 as libc::c_int as HANDLE;
    (*pifn).hTper = 0 as libc::c_int as HANDLE;
    (*pifn).hT0 = 0 as libc::c_int as HANDLE;
    (*pifn).hTexp = 0 as libc::c_int as HANDLE;
    (*pifn).hDecay = 0 as libc::c_int as HANDLE;
    (*pifn).iDoseCur = 0 as libc::c_int;
    (*pifn).nDoses = (*pifn).iDoseCur;
    (*pifn).rgMags = 0 as PDOUBLE;
    (*pifn).rgTexps = (*pifn).rgMags;
    (*pifn).rgT0s = (*pifn).rgTexps;
}
pub unsafe extern "C" fn DefDepParm(
    mut szLex: PSTR,
    mut pdValue: PDOUBLE,
    mut phvar: *mut HANDLE,
) -> BOOL {
    let mut bReturn: BOOL = 1 as libc::c_int;
    if if !szLex.is_null() {
        (*(*__ctype_b_loc()).offset(*szLex as libc::c_int as isize) as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *szLex as libc::c_int == '_' as i32) as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        *phvar = CalculateVarHandle(vpvmGlo, szLex);
        if *phvar == 0 {
            bReturn = 0 as libc::c_int;
            ReportError(
                0 as PINPUTBUF,
                (0x100 as libc::c_int + 6 as libc::c_int) as WORD,
                szLex,
                0 as PSTR,
            );
        }
    } else {
        *pdValue = atof(szLex as *const libc::c_char);
    }
    return bReturn;
}
pub unsafe extern "C" fn GetInputArgs(mut pibIn: PINPUTBUF, mut pifn: PIFN) -> BOOL {
    let mut rgszLex: [PSTRLEX; 4] = [[0; 1023]; 4];
    let mut rgiTypes: [libc::c_int; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    let mut rgiLowerB: [libc::c_long; 4] = [0; 4];
    let mut rgiUpperB: [libc::c_long; 4] = [0; 4];
    let mut bReturn: BOOL = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        rgiTypes[i
            as usize] = 0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int;
        i += 1;
        i;
    }
    if GetFuncArgs(
        pibIn,
        4 as libc::c_int,
        rgiTypes.as_mut_ptr(),
        (rgszLex[0 as libc::c_int as usize]).as_mut_ptr(),
        rgiLowerB.as_mut_ptr(),
        rgiUpperB.as_mut_ptr(),
    ) != 0
    {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            if rgiLowerB[i as usize] != -(1 as libc::c_int) as libc::c_long
                || rgiUpperB[i as usize] != -(1 as libc::c_int) as libc::c_long
            {
                ReportError(
                    pibIn,
                    (0x100 as libc::c_int + 1 as libc::c_int | 0x8000 as libc::c_int)
                        as WORD,
                    b"array bounds\0" as *const u8 as *const libc::c_char as PSTR,
                    b"Arrays cannot be used as input function parameters\0" as *const u8
                        as *const libc::c_char as PSTR,
                );
            }
            i += 1;
            i;
        }
        bReturn = 1 as libc::c_int;
        bReturn
            &= DefDepParm(
                (rgszLex[0 as libc::c_int as usize]).as_mut_ptr(),
                &mut (*pifn).dMag,
                &mut (*pifn).hMag,
            );
        bReturn
            &= DefDepParm(
                (rgszLex[1 as libc::c_int as usize]).as_mut_ptr(),
                &mut (*pifn).dTper,
                &mut (*pifn).hTper,
            );
        bReturn
            &= DefDepParm(
                (rgszLex[2 as libc::c_int as usize]).as_mut_ptr(),
                &mut (*pifn).dT0,
                &mut (*pifn).hT0,
            );
        if (*pifn).iType == 4 as libc::c_int {
            bReturn
                &= DefDepParm(
                    (rgszLex[3 as libc::c_int as usize]).as_mut_ptr(),
                    &mut (*pifn).dDecay,
                    &mut (*pifn).hDecay,
                );
        } else {
            bReturn
                &= DefDepParm(
                    (rgszLex[3 as libc::c_int as usize]).as_mut_ptr(),
                    &mut (*pifn).dTexp,
                    &mut (*pifn).hTexp,
                );
        }
        if bReturn == 0 {
            ReportError(
                pibIn,
                0x14 as libc::c_int as WORD,
                b"input-spec\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    }
    return bReturn;
}
pub unsafe extern "C" fn GetNNumbers(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut nNumbers: libc::c_int,
    mut rgd: PDOUBLE,
) -> BOOL {
    let mut bErr: BOOL = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nNumbers && bErr == 0 {
        if i != 0 {
            GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
        }
        bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int | 0x4 as libc::c_int);
        if bErr == 0 {
            *rgd.offset(i as isize) = atof(szLex as *const libc::c_char);
        }
        i += 1;
        i;
    }
    return bErr;
}
pub unsafe extern "C" fn GetNDoses(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pifn: PIFN,
) -> BOOL {
    let mut bErr: BOOL = 0 as libc::c_int;
    bErr = EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char);
    if !(bErr != 0) {
        bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
        if !(bErr != 0) {
            (*pifn).nDoses = atoi(szLex as *const libc::c_char);
            bErr = ((*pifn).nDoses <= 0 as libc::c_int) as libc::c_int;
            if bErr != 0 {
                ReportError(
                    pibIn,
                    0x15 as libc::c_int as WORD,
                    b"positive-integer\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            } else {
                (*pifn)
                    .rgT0s = malloc(
                    ((*pifn).nDoses as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                        ),
                ) as PDOUBLE;
                if ((*pifn).rgT0s).is_null()
                    || {
                        (*pifn)
                            .rgTexps = malloc(
                            ((*pifn).nDoses as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                ),
                        ) as PDOUBLE;
                        ((*pifn).rgTexps).is_null()
                    }
                    || {
                        (*pifn)
                            .rgMags = malloc(
                            ((*pifn).nDoses as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                                ),
                        ) as PDOUBLE;
                        ((*pifn).rgMags).is_null()
                    }
                {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetNDoses\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
                bErr = GetNNumbers(pibIn, szLex, (*pifn).nDoses, (*pifn).rgMags);
                GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
                if bErr == 0 {
                    bErr = GetNNumbers(pibIn, szLex, (*pifn).nDoses, (*pifn).rgT0s);
                }
                GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char);
                if bErr == 0 {
                    bErr = GetNNumbers(pibIn, szLex, (*pifn).nDoses, (*pifn).rgTexps);
                }
                if bErr == 0 {
                    bErr = EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char);
                }
            }
        }
    }
    if bErr != 0 {
        fprintf(
            stderr,
            b"Syntax: GetNDoses (nDoses, <n Magnitudes>, <n T0's>, <n Texposure's>)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return (bErr == 0) as libc::c_int;
}
pub unsafe extern "C" fn GetInputFn(
    mut pibIn: PINPUTBUF,
    mut sz: PSTR,
    mut pifn: PIFN,
) -> BOOL {
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
    let mut pibDum: PINPUTBUF = &mut ibDummy;
    let mut szLex: PSTRLEX = [0; 1023];
    let mut iType: libc::c_int = 0;
    let mut bReturn: BOOL = 0 as libc::c_int;
    if pibIn.is_null() || pifn.is_null() {
        return 0 as libc::c_int;
    }
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    vpvmGlo = (*pinfo).pvmGloVars;
    if !sz.is_null() {
        MakeStringBuffer(pibIn, pibDum, sz);
    } else {
        pibDum = pibIn;
    }
    NextLex(pibDum, szLex.as_mut_ptr(), &mut iType);
    match iType {
        4 | 2 | 1 => {
            InitIFN(pifn);
            if iType == 0x1 as libc::c_int {
                (*pifn).iType = GetFnType(szLex.as_mut_ptr());
                match (*pifn).iType {
                    5 => {
                        bReturn = GetNDoses(pibDum, szLex.as_mut_ptr(), pifn);
                    }
                    2 | 3 | 4 => {
                        bReturn = GetInputArgs(pibDum, pifn);
                    }
                    _ => {
                        (*pifn).iType = 0 as libc::c_int;
                        ReportError(
                            pibIn,
                            0x15 as libc::c_int as WORD,
                            b"input-spec\0" as *const u8 as *const libc::c_char as PSTR,
                            szLex.as_mut_ptr(),
                        );
                    }
                }
            } else {
                (*pifn).iType = 1 as libc::c_int;
                (*pifn).dVal = atof(szLex.as_mut_ptr());
                (*pifn).dMag = (*pifn).dVal;
                (*pifn).bOn = 1 as libc::c_int;
                bReturn = 1 as libc::c_int;
            }
        }
        0 | 8 | _ => {
            ReportError(
                pibIn,
                0x15 as libc::c_int as WORD,
                b"input-spec\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    }
    return bReturn;
}
