use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn MakeStringBuffer(pBuf: PINPUTBUF, pStrBuf: PINPUTBUF, sz: PSTR);
    fn EGetPunct(pibIn: PINPUTBUF, szLex: PSTR, chPunct: libc::c_char) -> libc::c_int;
    fn ENextLex(_: PINPUTBUF, _: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn GetArrayBounds(pibIn: PINPUTBUF, piLB: PLONG, piUB: PLONG);
    fn GetFuncArgs(_: PINPUTBUF, _: libc::c_int, _: PINT, _: PSTR) -> BOOL;
    fn GetOptPunct(_: PINPUTBUF, _: PSTR, _: libc::c_char) -> libc::c_int;
    fn GetPunct(pibIn: PINPUTBUF, szLex: PSTR, chPunct: libc::c_char) -> libc::c_int;
    fn NextLex(_: PINPUTBUF, _: *mut libc::c_char, _: PINT);
    fn GetVarHandle(szName: PSTR) -> HVAR;
    fn InitdVector(cVectors: libc::c_long) -> *mut libc::c_double;
    fn InitiVector(cVectors: libc::c_long) -> *mut libc::c_int;
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn GetKeywordCode(szKeyword: PSTR, pfContext: PINT) -> libc::c_int;
    fn MyStrcmp(sz1: *const libc::c_char, sz2: *const libc::c_char) -> libc::c_int;
    static mut vrgszLexTypes: [PSTR; 0];
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
pub type INPUTBUF = tagINPUTBUF;
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
pub type PIFM = *mut tagINPUTFUNCTIONMAP;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagINPUTFUNCTIONMAP {
    pub szName: PSTR,
    pub iIFNType: libc::c_int,
}
pub type IFM = tagINPUTFUNCTIONMAP;
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
pub static mut vrgifmMap: [IFM; 7] = [
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"PerDose\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"PerExp\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"PerTransit\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"NDoses\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"Spikes\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = tagINPUTFUNCTIONMAP {
            szName: b"Events\0" as *const u8 as *const libc::c_char as PSTR,
            iIFNType: 6 as libc::c_int,
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
    (*pifn).nDoses = 0 as libc::c_int;
    (*pifn).iDoseCur = 0 as libc::c_int;
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
        *phvar = GetVarHandle(szLex);
        if *phvar == 0 {
            bReturn = 0 as libc::c_int;
            ReportError(0 as PINPUTBUF, 0x106 as libc::c_int as WORD, szLex, 0 as PSTR);
        }
    } else {
        *pdValue = atof(szLex as *const libc::c_char);
        *phvar = 0 as libc::c_int as HANDLE;
    }
    return bReturn;
}
pub unsafe extern "C" fn GetInputArgs(
    mut pibIn: PINPUTBUF,
    mut pifn: PIFN,
    mut nArgs: libc::c_int,
) -> BOOL {
    let mut rgszLex: *mut PSTRLEX_ = malloc(
        (nArgs as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<PSTRLEX_>() as libc::c_ulong),
    ) as *mut PSTRLEX_;
    let mut rgiTypes: *mut libc::c_int = malloc(
        (nArgs as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut bReturn: BOOL = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nArgs {
        *rgiTypes
            .offset(
                i as isize,
            ) = 0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int;
        i += 1;
        i;
    }
    if GetFuncArgs(
        pibIn,
        nArgs,
        rgiTypes,
        (*rgszLex.offset(0 as libc::c_int as isize)).as_mut_ptr(),
    ) != 0
    {
        bReturn = 1 as libc::c_int;
        bReturn
            &= DefDepParm(
                (*rgszLex.offset(0 as libc::c_int as isize)).as_mut_ptr(),
                &mut (*pifn).dMag,
                &mut (*pifn).hMag,
            );
        bReturn
            &= DefDepParm(
                (*rgszLex.offset(1 as libc::c_int as isize)).as_mut_ptr(),
                &mut (*pifn).dTper,
                &mut (*pifn).hTper,
            );
        bReturn
            &= DefDepParm(
                (*rgszLex.offset(2 as libc::c_int as isize)).as_mut_ptr(),
                &mut (*pifn).dT0,
                &mut (*pifn).hT0,
            );
        if (*pifn).iType == 3 as libc::c_int || (*pifn).iType == 7 as libc::c_int {
            bReturn
                &= DefDepParm(
                    (*rgszLex.offset(3 as libc::c_int as isize)).as_mut_ptr(),
                    &mut (*pifn).dDecay,
                    &mut (*pifn).hDecay,
                );
        } else {
            bReturn
                &= DefDepParm(
                    (*rgszLex.offset(3 as libc::c_int as isize)).as_mut_ptr(),
                    &mut (*pifn).dTexp,
                    &mut (*pifn).hTexp,
                );
        }
        if (*pifn).iType == 7 as libc::c_int && nArgs == 5 as libc::c_int {
            bReturn
                &= DefDepParm(
                    (*rgszLex.offset(4 as libc::c_int as isize)).as_mut_ptr(),
                    &mut (*pifn).dNcpt,
                    &mut (*pifn).hNcpt,
                );
        }
        if bReturn == 0 {
            ReportError(
                pibIn,
                0x13 as libc::c_int as WORD,
                b"input-spec\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    }
    free(rgiTypes as *mut libc::c_void);
    free(rgszLex as *mut libc::c_void);
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
    let mut current_block: u64;
    let mut rgszLex: *mut PSTRLEX_ = 0 as *mut PSTRLEX_;
    let mut szTmp: PSTRLEX_ = [0; 255];
    let mut rgiTypes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut iType: libc::c_int = 0;
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut iLB: libc::c_long = 0;
    let mut iUB: libc::c_long = 0;
    let mut iDoseArg: libc::c_long = 0;
    let mut bOK: BOOL = 1 as libc::c_int;
    let mut bErr: BOOL = 0 as libc::c_int;
    let mut hvar: HVAR = 0;
    bErr = EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char);
    if !(bErr != 0) {
        bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
        if !(bErr != 0) {
            (*pifn).nDoses = atoi(szLex as *const libc::c_char);
            bErr = ((*pifn).nDoses <= 0 as libc::c_int) as libc::c_int;
            if bErr != 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"positive-integer\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            } else {
                iDoseArg = (2 as libc::c_int * (*pifn).nDoses) as libc::c_long;
                rgiTypes = InitiVector(iDoseArg);
                if rgiTypes.is_null() {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetNDoses\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                rgszLex = malloc(
                    (iDoseArg as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<PSTRLEX_>() as libc::c_ulong),
                ) as *mut PSTRLEX_;
                if rgszLex.is_null() {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetNDoses\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                (*pifn)
                    .rgT0s = InitdVector(
                    ((*pifn).nDoses + 1 as libc::c_int) as libc::c_long,
                );
                if ((*pifn).rgT0s).is_null()
                    || {
                        (*pifn).rgMags = InitdVector((*pifn).nDoses as libc::c_long);
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
                (*pifn)
                    .rghT0s = malloc(
                    (((*pifn).nDoses + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<HANDLE>() as libc::c_ulong),
                ) as *mut HANDLE;
                if ((*pifn).rghT0s).is_null()
                    || {
                        (*pifn)
                            .rghMags = malloc(
                            ((*pifn).nDoses as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<HANDLE>() as libc::c_ulong,
                                ),
                        ) as *mut HANDLE;
                        ((*pifn).rghMags).is_null()
                    }
                {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetNDoses\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                bOK = GetPunct(
                    pibIn,
                    (*rgszLex.offset(0 as libc::c_int as isize)).as_mut_ptr(),
                    ',' as i32 as libc::c_char,
                );
                if bOK == 0 {
                    *(*rgszLex.offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) = ',' as i32 as libc::c_char;
                    ReportError(
                        pibIn,
                        (0x13 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        (*rgszLex.offset(0 as libc::c_int as isize)).as_mut_ptr(),
                        0 as PSTR,
                    );
                }
                i = 0 as libc::c_int as libc::c_long;
                loop {
                    if !(i < iDoseArg / 2 as libc::c_int as libc::c_long && bOK != 0) {
                        current_block = 6450636197030046351;
                        break;
                    }
                    *rgiTypes
                        .offset(
                            i as isize,
                        ) = 0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int;
                    NextLex(pibIn, szLex, &mut iType);
                    bOK
                        &= (iType & *rgiTypes.offset(i as isize) > 0 as libc::c_int)
                            as libc::c_int;
                    if bOK == 0 {
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            *vrgszLexTypes
                                .as_mut_ptr()
                                .offset(*rgiTypes.offset(i as isize) as isize),
                            szLex,
                        );
                    }
                    iUB = -(1 as libc::c_int) as libc::c_long;
                    iLB = iUB;
                    if GetPunct(pibIn, szTmp.as_mut_ptr(), '[' as i32 as libc::c_char)
                        != 0
                    {
                        GetArrayBounds(pibIn, &mut iLB, &mut iUB);
                    }
                    if iUB == -(1 as libc::c_int) as libc::c_long {
                        strcpy(
                            (*rgszLex.offset(i as isize)).as_mut_ptr(),
                            szLex as *const libc::c_char,
                        );
                        i += 1;
                        i;
                    } else {
                        if 2 as libc::c_int as libc::c_long * (iUB - iLB) != iDoseArg {
                            ReportError(
                                pibIn,
                                (0x117 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                b"GetNDoses\0" as *const u8 as *const libc::c_char as PSTR,
                                0 as PSTR,
                            );
                        }
                        j = iLB;
                        while j < iUB {
                            sprintf(
                                szTmp.as_mut_ptr(),
                                b"%s_%ld\0" as *const u8 as *const libc::c_char,
                                szLex,
                                j,
                            );
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
                                strcpy(
                                    (*rgszLex.offset((i + j - iLB) as isize)).as_mut_ptr(),
                                    szTmp.as_mut_ptr(),
                                );
                            }
                            j += 1;
                            j;
                        }
                        bErr = (GetPunct(
                            pibIn,
                            szTmp.as_mut_ptr(),
                            ',' as i32 as libc::c_char,
                        ) == 0) as libc::c_int;
                        if bErr != 0 {
                            current_block = 8715050895493170594;
                            break;
                        } else {
                            current_block = 6450636197030046351;
                            break;
                        }
                    }
                }
                match current_block {
                    8715050895493170594 => {}
                    _ => {
                        i = iDoseArg / 2 as libc::c_int as libc::c_long;
                        loop {
                            if !(i < iDoseArg && bOK != 0) {
                                current_block = 12829669402821218572;
                                break;
                            }
                            *rgiTypes
                                .offset(
                                    i as isize,
                                ) = 0x2 as libc::c_int | 0x4 as libc::c_int
                                | 0x1 as libc::c_int;
                            NextLex(pibIn, szLex, &mut iType);
                            bOK
                                &= (iType & *rgiTypes.offset(i as isize) > 0 as libc::c_int)
                                    as libc::c_int;
                            if bOK == 0 {
                                ReportError(
                                    pibIn,
                                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                    *vrgszLexTypes
                                        .as_mut_ptr()
                                        .offset(*rgiTypes.offset(i as isize) as isize),
                                    szLex,
                                );
                            }
                            iUB = -(1 as libc::c_int) as libc::c_long;
                            iLB = iUB;
                            if GetPunct(
                                pibIn,
                                szTmp.as_mut_ptr(),
                                '[' as i32 as libc::c_char,
                            ) != 0
                            {
                                GetArrayBounds(pibIn, &mut iLB, &mut iUB);
                            }
                            if iUB == -(1 as libc::c_int) as libc::c_long {
                                strcpy(
                                    (*rgszLex.offset(i as isize)).as_mut_ptr(),
                                    szLex as *const libc::c_char,
                                );
                                i += 1;
                                i;
                            } else {
                                if 2 as libc::c_int as libc::c_long * (iUB - iLB)
                                    != iDoseArg
                                {
                                    ReportError(
                                        pibIn,
                                        (0x117 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                        b"GetNDoses\0" as *const u8 as *const libc::c_char as PSTR,
                                        0 as PSTR,
                                    );
                                }
                                j = iLB;
                                while j < iUB {
                                    sprintf(
                                        szTmp.as_mut_ptr(),
                                        b"%s_%ld\0" as *const u8 as *const libc::c_char,
                                        szLex,
                                        j,
                                    );
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
                                        strcpy(
                                            (*rgszLex.offset((i + j - iLB) as isize)).as_mut_ptr(),
                                            szTmp.as_mut_ptr(),
                                        );
                                    }
                                    j += 1;
                                    j;
                                }
                                bErr = EGetPunct(
                                    pibIn,
                                    szTmp.as_mut_ptr(),
                                    ')' as i32 as libc::c_char,
                                );
                                if bErr != 0 {
                                    current_block = 8715050895493170594;
                                    break;
                                } else {
                                    current_block = 12829669402821218572;
                                    break;
                                }
                            }
                        }
                        match current_block {
                            8715050895493170594 => {}
                            _ => {
                                bErr = (szTmp[0 as libc::c_int as usize] as libc::c_int
                                    != ')' as i32) as libc::c_int;
                                if !(bErr != 0) {
                                    bOK = 1 as libc::c_int;
                                    i = 0 as libc::c_int as libc::c_long;
                                    while i < (*pifn).nDoses as libc::c_long {
                                        bOK
                                            &= DefDepParm(
                                                (*rgszLex.offset(i as isize)).as_mut_ptr(),
                                                ((*pifn).rgMags).offset(i as isize),
                                                ((*pifn).rghMags).offset(i as isize),
                                            );
                                        i += 1;
                                        i;
                                    }
                                    i = 0 as libc::c_int as libc::c_long;
                                    while i < (*pifn).nDoses as libc::c_long {
                                        bOK
                                            &= DefDepParm(
                                                (*rgszLex
                                                    .offset((i + (*pifn).nDoses as libc::c_long) as isize))
                                                    .as_mut_ptr(),
                                                ((*pifn).rgT0s).offset(i as isize),
                                                ((*pifn).rghT0s).offset(i as isize),
                                            );
                                        i += 1;
                                        i;
                                    }
                                    i = (*pifn).nDoses as libc::c_long;
                                    *((*pifn).rgT0s)
                                        .offset(i as isize) = 1.7976931348623157e+308f64;
                                    *((*pifn).rghT0s)
                                        .offset(i as isize) = 0 as libc::c_int as HANDLE;
                                    if bOK == 0 {
                                        ReportError(
                                            pibIn,
                                            (0x13 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                            b"input-spec\0" as *const u8 as *const libc::c_char as PSTR,
                                            0 as PSTR,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if bErr != 0 {
        printf(
            b"Syntax: NDoses (nInputs, <n Magnitudes>, <n T0's>)\n\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return (bErr == 0) as libc::c_int;
}
pub unsafe extern "C" fn GetEvents(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pifn: PIFN,
) -> BOOL {
    let mut rgszLex: *mut PSTRLEX_ = 0 as *mut PSTRLEX_;
    let mut iType: libc::c_int = 0;
    let mut rgiTypes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut iDoseArg: libc::c_int = 0;
    let mut bOK: BOOL = 1 as libc::c_int;
    let mut bErr: BOOL = 0 as libc::c_int;
    bErr = EGetPunct(pibIn, szLex, '(' as i32 as libc::c_char);
    if !(bErr != 0) {
        NextLex(pibIn, szLex, &mut iType);
        bOK &= (iType & 0x1 as libc::c_int > 0 as libc::c_int) as libc::c_int;
        if bOK == 0 {
            ReportError(
                pibIn,
                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                *vrgszLexTypes.as_mut_ptr().offset(0x1 as libc::c_int as isize),
                szLex,
            );
        }
        (*pifn).target_state = GetVarHandle(szLex);
        if (*pifn).target_state == 0 {
            ReportError(
                0 as PINPUTBUF,
                (0x106 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                szLex,
                0 as PSTR,
            );
        }
        bOK = GetPunct(pibIn, szLex, ',' as i32 as libc::c_char);
        if bOK == 0 {
            ReportError(
                pibIn,
                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b",\0" as *const u8 as *const libc::c_char as PSTR,
                szLex,
            );
        }
        bErr = ENextLex(pibIn, szLex, 0x2 as libc::c_int);
        if !(bErr != 0) {
            (*pifn).nDoses = atoi(szLex as *const libc::c_char);
            bErr = ((*pifn).nDoses <= 0 as libc::c_int) as libc::c_int;
            if bErr != 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"positive-integer\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            } else {
                iDoseArg = 3 as libc::c_int * (*pifn).nDoses;
                rgiTypes = InitiVector(iDoseArg as libc::c_long);
                if rgiTypes.is_null() {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetEvents\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                rgszLex = malloc(
                    (iDoseArg as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<PSTRLEX_>() as libc::c_ulong),
                ) as *mut PSTRLEX_;
                if rgszLex.is_null() {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetEvents\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                i = 0 as libc::c_int;
                while i < iDoseArg && bOK != 0 {
                    *rgiTypes
                        .offset(
                            i as isize,
                        ) = 0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int;
                    bOK = GetOptPunct(
                        pibIn,
                        (*rgszLex.offset(i as isize)).as_mut_ptr(),
                        ',' as i32 as libc::c_char,
                    );
                    if bOK == 0 {
                        *(*rgszLex.offset(i as isize))
                            .as_mut_ptr()
                            .offset(
                                1 as libc::c_int as isize,
                            ) = ',' as i32 as libc::c_char;
                        ReportError(
                            pibIn,
                            (0x13 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            (*rgszLex.offset(i as isize)).as_mut_ptr(),
                            0 as PSTR,
                        );
                    }
                    NextLex(
                        pibIn,
                        (*rgszLex.offset(i as isize)).as_mut_ptr(),
                        &mut iType,
                    );
                    bOK
                        &= (iType & *rgiTypes.offset(i as isize) > 0 as libc::c_int)
                            as libc::c_int;
                    if bOK == 0 {
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            *vrgszLexTypes
                                .as_mut_ptr()
                                .offset(*rgiTypes.offset(i as isize) as isize),
                            (*rgszLex.offset(i as isize)).as_mut_ptr(),
                        );
                    }
                    i += 1;
                    i;
                }
                bErr = EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char);
                if !(bErr != 0) {
                    (*pifn).rgT0s = InitdVector((*pifn).nDoses as libc::c_long);
                    if ((*pifn).rgT0s).is_null()
                        || {
                            (*pifn).rgOper = InitiVector((*pifn).nDoses as libc::c_long);
                            ((*pifn).rgOper).is_null()
                        }
                        || {
                            (*pifn).rgMags = InitdVector((*pifn).nDoses as libc::c_long);
                            ((*pifn).rgMags).is_null()
                        }
                    {
                        ReportError(
                            pibIn,
                            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"GetEvents\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                    (*pifn)
                        .rghT0s = malloc(
                        ((*pifn).nDoses as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<HANDLE>() as libc::c_ulong,
                            ),
                    ) as *mut HANDLE;
                    if ((*pifn).rghT0s).is_null()
                        || {
                            (*pifn)
                                .rghMags = malloc(
                                ((*pifn).nDoses as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<HANDLE>() as libc::c_ulong,
                                    ),
                            ) as *mut HANDLE;
                            ((*pifn).rghMags).is_null()
                        }
                    {
                        ReportError(
                            pibIn,
                            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"GetEvents\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                    bOK = 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < (*pifn).nDoses {
                        bOK
                            &= DefDepParm(
                                (*rgszLex.offset(i as isize)).as_mut_ptr(),
                                ((*pifn).rgT0s).offset(i as isize),
                                ((*pifn).rghT0s).offset(i as isize),
                            );
                        i += 1;
                        i;
                    }
                    i = 0 as libc::c_int;
                    while i < (*pifn).nDoses {
                        *((*pifn).rgOper)
                            .offset(
                                i as isize,
                            ) = GetKeywordCode(
                            (*rgszLex.offset(((*pifn).nDoses + i) as isize))
                                .as_mut_ptr(),
                            0 as PINT,
                        );
                        if *((*pifn).rgOper).offset(i as isize) == 0 {
                            ReportError(
                                pibIn,
                                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                b"Replace, Add or Multiply operation\0" as *const u8
                                    as *const libc::c_char as PSTR,
                                (*rgszLex.offset(((*pifn).nDoses + i) as isize))
                                    .as_mut_ptr(),
                            );
                        }
                        i += 1;
                        i;
                    }
                    i = 0 as libc::c_int;
                    while i < (*pifn).nDoses {
                        bOK
                            &= DefDepParm(
                                (*rgszLex
                                    .offset((2 as libc::c_int * (*pifn).nDoses + i) as isize))
                                    .as_mut_ptr(),
                                ((*pifn).rgMags).offset(i as isize),
                                ((*pifn).rghMags).offset(i as isize),
                            );
                        i += 1;
                        i;
                    }
                    if bOK == 0 {
                        ReportError(
                            pibIn,
                            (0x13 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"input-spec\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                }
            }
        }
    }
    if bErr != 0 {
        printf(
            b"Syntax: Events (State, nEvents, <n Times>, <n Operations>, <n Values>)\n\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return (bErr == 0) as libc::c_int;
}
pub unsafe extern "C" fn GetSpikes(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut pifn: PIFN,
) -> BOOL {
    let mut rgszLex: *mut PSTRLEX_ = 0 as *mut PSTRLEX_;
    let mut rgiTypes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut iType: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut iDoseArg: libc::c_int = 0;
    let mut bOK: BOOL = 1 as libc::c_int;
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
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"positive-integer\0" as *const u8 as *const libc::c_char as PSTR,
                    szLex,
                );
            } else {
                iDoseArg = 2 as libc::c_int * (*pifn).nDoses;
                rgiTypes = InitiVector(iDoseArg as libc::c_long);
                if rgiTypes.is_null() {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetSpikes\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                rgszLex = malloc(
                    (iDoseArg as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<PSTRLEX_>() as libc::c_ulong),
                ) as *mut PSTRLEX_;
                if rgszLex.is_null() {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetSpikes\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                (*pifn).rgT0s = InitdVector((*pifn).nDoses as libc::c_long);
                if ((*pifn).rgT0s).is_null()
                    || {
                        (*pifn).rgMags = InitdVector((*pifn).nDoses as libc::c_long);
                        ((*pifn).rgMags).is_null()
                    }
                {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetSpikes\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                (*pifn)
                    .rghT0s = malloc(
                    ((*pifn).nDoses as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<HANDLE>() as libc::c_ulong),
                ) as *mut HANDLE;
                if ((*pifn).rghT0s).is_null()
                    || {
                        (*pifn)
                            .rghMags = malloc(
                            ((*pifn).nDoses as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<HANDLE>() as libc::c_ulong,
                                ),
                        ) as *mut HANDLE;
                        ((*pifn).rghMags).is_null()
                    }
                {
                    ReportError(
                        pibIn,
                        (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"GetSpikes\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                i = 0 as libc::c_int;
                while i < iDoseArg && bOK != 0 {
                    *rgiTypes
                        .offset(
                            i as isize,
                        ) = 0x2 as libc::c_int | 0x4 as libc::c_int | 0x1 as libc::c_int;
                    bOK = GetOptPunct(
                        pibIn,
                        (*rgszLex.offset(i as isize)).as_mut_ptr(),
                        ',' as i32 as libc::c_char,
                    );
                    if bOK == 0 {
                        *(*rgszLex.offset(i as isize))
                            .as_mut_ptr()
                            .offset(
                                1 as libc::c_int as isize,
                            ) = ',' as i32 as libc::c_char;
                        ReportError(
                            pibIn,
                            (0x13 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            (*rgszLex.offset(i as isize)).as_mut_ptr(),
                            0 as PSTR,
                        );
                        break;
                    } else {
                        NextLex(
                            pibIn,
                            (*rgszLex.offset(i as isize)).as_mut_ptr(),
                            &mut iType,
                        );
                        bOK
                            &= (iType & *rgiTypes.offset(i as isize) > 0 as libc::c_int)
                                as libc::c_int;
                        if bOK == 0 {
                            ReportError(
                                pibIn,
                                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                                *vrgszLexTypes
                                    .as_mut_ptr()
                                    .offset(*rgiTypes.offset(i as isize) as isize),
                                (*rgszLex.offset(i as isize)).as_mut_ptr(),
                            );
                        }
                        i += 1;
                        i;
                    }
                }
                bErr = EGetPunct(pibIn, szLex, ')' as i32 as libc::c_char);
                if !(bErr != 0) {
                    bOK = 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < (*pifn).nDoses {
                        bOK
                            &= DefDepParm(
                                (*rgszLex.offset(i as isize)).as_mut_ptr(),
                                ((*pifn).rgMags).offset(i as isize),
                                ((*pifn).rghMags).offset(i as isize),
                            );
                        i += 1;
                        i;
                    }
                    i = 0 as libc::c_int;
                    while i < (*pifn).nDoses {
                        bOK
                            &= DefDepParm(
                                (*rgszLex.offset((i + (*pifn).nDoses) as isize))
                                    .as_mut_ptr(),
                                ((*pifn).rgT0s).offset(i as isize),
                                ((*pifn).rghT0s).offset(i as isize),
                            );
                        i += 1;
                        i;
                    }
                    if bOK == 0 {
                        ReportError(
                            pibIn,
                            (0x13 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"input-spec\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                }
            }
        }
    }
    if bErr != 0 {
        printf(
            b"Syntax: Spikes (nInputs, <n Magnitudes>, <n Times>)\n\n\0" as *const u8
                as *const libc::c_char,
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
        pbufCur: 0 as *mut libc::c_char,
        iLineNum: 0,
        iLNPrev: 0,
        cErrors: 0,
        pInfo: 0 as *mut libc::c_void,
    };
    let mut pibDum: PINPUTBUF = &mut ibDummy;
    let mut szLex: PSTRLEX_ = [0; 255];
    let mut iType: libc::c_int = 0;
    let mut bReturn: BOOL = 0 as libc::c_int;
    if pibIn.is_null() || pifn.is_null() {
        return 0 as libc::c_int;
    }
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
                    4 => {
                        bReturn = GetNDoses(pibDum, szLex.as_mut_ptr(), pifn);
                    }
                    5 => {
                        bReturn = GetSpikes(pibDum, szLex.as_mut_ptr(), pifn);
                    }
                    6 => {
                        bReturn = GetEvents(pibDum, szLex.as_mut_ptr(), pifn);
                    }
                    2 | 3 => {
                        bReturn = GetInputArgs(pibDum, pifn, 4 as libc::c_int);
                    }
                    7 => {
                        bReturn = GetInputArgs(pibDum, pifn, 5 as libc::c_int);
                    }
                    _ => {
                        (*pifn).iType = 0 as libc::c_int;
                        ReportError(
                            pibIn,
                            0x14 as libc::c_int as WORD,
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
                0x14 as libc::c_int as WORD,
                b"input-spec\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    }
    return bReturn;
}
