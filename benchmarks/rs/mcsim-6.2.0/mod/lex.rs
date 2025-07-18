use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn rewind(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub type PSTR = *mut libc::c_char;
pub type PINT = *mut libc::c_int;
pub type PLONG = *mut libc::c_long;
pub type PVOID = *mut libc::c_void;
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
pub type PINPUTBUF = *mut tagINPUTBUF;
pub type PSTRLEX = [libc::c_char; 1023];
pub const End: States = 7;
pub type States = libc::c_uint;
pub const Digits3: States = 6;
pub const ExpSign: States = 5;
pub const Exp: States = 4;
pub const Digits2: States = 3;
pub const Point: States = 2;
pub const Digits1: States = 1;
pub const Start: States = 0;
pub const End_0: States_0 = 2;
pub type States_0 = libc::c_uint;
pub const Digits1_0: States_0 = 1;
pub const Start_0: States_0 = 0;
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
pub static mut vrgszLexTypes: [PSTR; 18] = [
    b"null-type??\0" as *const u8 as *const libc::c_char as PSTR,
    b"identifier\0" as *const u8 as *const libc::c_char as PSTR,
    b"integer\0" as *const u8 as *const libc::c_char as PSTR,
    b"integer-or-id\0" as *const u8 as *const libc::c_char as PSTR,
    b"floating-point\0" as *const u8 as *const libc::c_char as PSTR,
    b"float-or-id\0" as *const u8 as *const libc::c_char as PSTR,
    b"number\0" as *const u8 as *const libc::c_char as PSTR,
    b"number-or-id\0" as *const u8 as *const libc::c_char as PSTR,
    b"punctuation\0" as *const u8 as *const libc::c_char as PSTR,
    b"invalid lexical type\0" as *const u8 as *const libc::c_char as PSTR,
    b"invalid lexical type\0" as *const u8 as *const libc::c_char as PSTR,
    b"invalid lexical type\0" as *const u8 as *const libc::c_char as PSTR,
    b"invalid lexical type\0" as *const u8 as *const libc::c_char as PSTR,
    b"invalid lexical type\0" as *const u8 as *const libc::c_char as PSTR,
    b"invalid lexical type\0" as *const u8 as *const libc::c_char as PSTR,
    b"invalid lexical type\0" as *const u8 as *const libc::c_char as PSTR,
    b"quoted-string\0" as *const u8 as *const libc::c_char as PSTR,
    b"\0" as *const u8 as *const libc::c_char as PSTR,
];
pub unsafe extern "C" fn CountChars(mut pFileIn: PFILE) -> libc::c_long {
    let mut nChars: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut c: libc::c_char = 0;
    loop {
        c = getc(pFileIn) as libc::c_char;
        if !(c as libc::c_int != -(1 as libc::c_int)) {
            break;
        }
        nChars += 1;
        nChars;
    }
    rewind(pFileIn);
    return nChars;
}
pub unsafe extern "C" fn ENextLex(
    mut pibIn: PINPUTBUF,
    mut szLex: *mut libc::c_char,
    mut iType: libc::c_int,
) -> libc::c_int {
    let mut iLex: libc::c_int = 0;
    let mut iErr: libc::c_int = 0;
    NextLex(pibIn, szLex, &mut iLex);
    iErr = (iType & iLex == 0) as libc::c_int;
    if iErr != 0 {
        ReportError(
            pibIn,
            0x15 as libc::c_int as WORD,
            vrgszLexTypes[iType as usize],
            szLex,
        );
    }
    return iErr;
}
pub unsafe extern "C" fn EvalAtom(
    mut pibIn: PINPUTBUF,
    mut index: libc::c_long,
    mut szExp: *mut PSTR,
    mut szToken: PSTR,
    mut piType: PINT,
) -> libc::c_long {
    let mut result: libc::c_long = 0 as libc::c_int as libc::c_long;
    match *piType {
        1 => {
            result = index;
            GetToken(szExp, szToken, piType);
        }
        2 => {
            result = atol(szToken as *const libc::c_char);
            GetToken(szExp, szToken, piType);
        }
        _ => {
            ReportError(
                pibIn,
                (0x11 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                *szExp,
                b"(While parsing bracketed expression)\0" as *const u8
                    as *const libc::c_char as PSTR,
            );
        }
    }
    return result;
}
pub unsafe extern "C" fn EvalParen(
    mut pibIn: PINPUTBUF,
    mut index: libc::c_long,
    mut szExp: *mut PSTR,
    mut szToken: PSTR,
    mut piType: PINT,
) -> libc::c_long {
    let mut result: libc::c_long = 0;
    if *szToken as libc::c_int == '(' as i32 {
        GetToken(szExp, szToken, piType);
        result = EvalSum(pibIn, index, szExp, szToken, piType);
        if *szToken as libc::c_int != ')' as i32 {
            ReportError(
                pibIn,
                (0x11 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                *szExp,
                b"(While parsing bracketed expression)\0" as *const u8
                    as *const libc::c_char as PSTR,
            );
        }
        GetToken(szExp, szToken, piType);
    } else {
        result = EvalAtom(pibIn, index, szExp, szToken, piType);
    }
    return result;
}
pub unsafe extern "C" fn EvalProd(
    mut pibIn: PINPUTBUF,
    mut index: libc::c_long,
    mut szExp: *mut PSTR,
    mut szToken: PSTR,
    mut piType: PINT,
) -> libc::c_long {
    let mut cOperator: libc::c_char = 0;
    let mut result: libc::c_long = 0;
    let mut dTmp: libc::c_long = 0;
    result = EvalUnary(pibIn, index, szExp, szToken, piType);
    loop {
        cOperator = *szToken;
        if !(cOperator as libc::c_int == '*' as i32
            || cOperator as libc::c_int == '/' as i32)
        {
            break;
        }
        GetToken(szExp, szToken, piType);
        dTmp = EvalUnary(pibIn, index, szExp, szToken, piType);
        match cOperator as libc::c_int {
            42 => {
                result = result * dTmp;
                return result;
            }
            47 => {
                result = result / dTmp;
                return result;
            }
            _ => {
                ReportError(
                    pibIn,
                    (0x11 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    *szExp,
                    b"(While parsing bracketed expression)\0" as *const u8
                        as *const libc::c_char as PSTR,
                );
            }
        }
    }
    return result;
}
pub unsafe extern "C" fn EvalSum(
    mut pibIn: PINPUTBUF,
    mut index: libc::c_long,
    mut szExp: *mut PSTR,
    mut szToken: PSTR,
    mut piType: PINT,
) -> libc::c_long {
    let mut cOperator: libc::c_char = 0;
    let mut result: libc::c_long = 0;
    let mut dTmp: libc::c_long = 0;
    result = EvalProd(pibIn, index, szExp, szToken, piType);
    loop {
        cOperator = *szToken;
        if !(cOperator as libc::c_int == '+' as i32
            || cOperator as libc::c_int == '-' as i32)
        {
            break;
        }
        GetToken(szExp, szToken, piType);
        dTmp = EvalProd(pibIn, index, szExp, szToken, piType);
        match cOperator as libc::c_int {
            45 => {
                result = result - dTmp;
            }
            43 => {
                result = result + dTmp;
            }
            _ => {
                ReportError(
                    pibIn,
                    (0x11 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    *szExp,
                    b"(While parsing bracketed expression)\0" as *const u8
                        as *const libc::c_char as PSTR,
                );
            }
        }
    }
    return result;
}
pub unsafe extern "C" fn EvaluateExpression(
    mut pibIn: PINPUTBUF,
    mut index: libc::c_long,
    mut szExpress: PSTR,
) -> libc::c_long {
    let mut szToken: PSTRLEX = [0; 1023];
    let mut iType: libc::c_int = 0;
    GetToken(&mut szExpress, szToken.as_mut_ptr(), &mut iType);
    if *szToken.as_mut_ptr() == 0 {
        ReportError(
            pibIn,
            (0x11 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szExpress,
            b"(While parsing bracketed expression)\0" as *const u8 as *const libc::c_char
                as PSTR,
        );
        return 0 as libc::c_int as libc::c_long;
    } else {
        return EvalSum(pibIn, index, &mut szExpress, szToken.as_mut_ptr(), &mut iType)
    };
}
pub unsafe extern "C" fn EvalUnary(
    mut pibIn: PINPUTBUF,
    mut index: libc::c_long,
    mut szExp: *mut PSTR,
    mut szToken: PSTR,
    mut piType: PINT,
) -> libc::c_long {
    let mut cOperator: libc::c_char = 0;
    let mut result: libc::c_long = 0;
    cOperator = 0 as libc::c_int as libc::c_char;
    if *piType == 0x20 as libc::c_int
        && (*szToken as libc::c_int == '+' as i32
            || *szToken as libc::c_int == '-' as i32)
    {
        cOperator = *szToken;
        GetToken(szExp, szToken, piType);
    }
    result = EvalParen(pibIn, index, szExp, szToken, piType);
    if cOperator as libc::c_int == '-' as i32 {
        result = -result;
    }
    return result;
}
pub unsafe extern "C" fn FillBuffer(
    mut pibIn: PINPUTBUF,
    mut lBuffer_size: libc::c_long,
) -> libc::c_int {
    let mut iReturn: libc::c_int = 0 as libc::c_int;
    let mut iOffset: libc::c_int = 0;
    if !pibIn.is_null() && !((*pibIn).pfileIn).is_null() && !((*pibIn).pbufOrg).is_null()
    {
        iOffset = fread(
            (*pibIn).pbufOrg as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            lBuffer_size as libc::c_ulong,
            (*pibIn).pfileIn,
        ) as libc::c_int;
        if iOffset != 0 {
            iReturn = iOffset;
            (*pibIn).pbufCur = (*pibIn).pbufOrg;
        } else if feof((*pibIn).pfileIn) != 0 {
            iReturn = -(1 as libc::c_int);
        } else {
            ReportError(
                pibIn,
                0x8000 as libc::c_int as WORD,
                0 as PSTR,
                b"Unexpected end of file.\0" as *const u8 as *const libc::c_char as PSTR,
            );
        }
    }
    return iReturn;
}
pub unsafe extern "C" fn GetToken(
    mut szExp: *mut PSTR,
    mut szToken: PSTR,
    mut piType: PINT,
) {
    let mut cTmp: PSTR = 0 as *mut libc::c_char;
    *piType = 0 as libc::c_int;
    cTmp = szToken;
    *cTmp = '\0' as i32 as libc::c_char;
    if (*szExp).is_null() {
        return;
    }
    while *(*__ctype_b_loc()).offset(**szExp as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *szExp = (*szExp).offset(1);
        *szExp;
    }
    if !(strchr(b"+-*/()\0" as *const u8 as *const libc::c_char, **szExp as libc::c_int))
        .is_null()
    {
        *piType = 0x20 as libc::c_int;
        *cTmp = **szExp;
        cTmp = cTmp.offset(1);
        cTmp;
        *szExp = (*szExp).offset(1);
        *szExp;
    } else if **szExp as libc::c_int == 'i' as i32 {
        *piType = 0x1 as libc::c_int;
        while !(!(strchr(
            b"+-*/()\0" as *const u8 as *const libc::c_char,
            **szExp as libc::c_int,
        ))
            .is_null() || **szExp as libc::c_int == '\0' as i32)
        {
            *cTmp = **szExp;
            cTmp = cTmp.offset(1);
            cTmp;
            *szExp = (*szExp).offset(1);
            *szExp;
        }
    } else if *(*__ctype_b_loc()).offset(**szExp as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *piType = 0x2 as libc::c_int;
        while !(!(strchr(
            b"+-*/()\0" as *const u8 as *const libc::c_char,
            **szExp as libc::c_int,
        ))
            .is_null() || **szExp as libc::c_int == '\0' as i32)
        {
            *cTmp = **szExp;
            cTmp = cTmp.offset(1);
            cTmp;
            *szExp = (*szExp).offset(1);
            *szExp;
        }
    }
    *cTmp = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn InitBuffer(
    mut pibIn: PINPUTBUF,
    mut lSize: libc::c_long,
    mut szFileIn: PSTR,
) -> BOOL {
    let mut bReturn: BOOL = 0 as libc::c_int;
    if pibIn.is_null() {
        return 0 as libc::c_int;
    }
    if lSize < 0 as libc::c_int as libc::c_long {
        (*pibIn)
            .pfileIn = fopen(
            szFileIn as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if !((*pibIn).pfileIn).is_null() {
            (*pibIn).lBufSize = CountChars((*pibIn).pfileIn);
            fclose((*pibIn).pfileIn);
        } else {
            ReportError(
                pibIn,
                (0x2 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                szFileIn,
                0 as PSTR,
            );
        }
    } else {
        (*pibIn).lBufSize = lSize;
    }
    (*pibIn).iLineNum = 1 as libc::c_int;
    (*pibIn).iLNPrev = 0 as libc::c_int;
    (*pibIn).cErrors = 0 as libc::c_int;
    (*pibIn).pInfo = 0 as *mut libc::c_void;
    (*pibIn).pTempInfo = 0 as *mut libc::c_void;
    (*pibIn).pbufCur = 0 as PBUF;
    (*pibIn)
        .pfileIn = fopen(
        szFileIn as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !((*pibIn).pfileIn).is_null() {
        (*pibIn).pbufOrg = malloc((*pibIn).lBufSize as libc::c_ulong) as PBUF;
        if !((*pibIn).pbufOrg).is_null() {
            bReturn = FillBuffer(pibIn, (*pibIn).lBufSize);
        } else {
            ReportError(
                pibIn,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"InitBuffer\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    } else {
        ReportError(
            pibIn,
            (0x2 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szFileIn,
            0 as PSTR,
        );
    }
    if lSize < 0 as libc::c_int as libc::c_long {
        fclose((*pibIn).pfileIn);
    }
    return bReturn;
}
pub unsafe extern "C" fn MakeStringBuffer(
    mut pBuf: PINPUTBUF,
    mut pbufStr: PINPUTBUF,
    mut sz: PSTR,
) {
    (*pbufStr).pfileIn = 0 as PFILE;
    (*pbufStr).pbufOrg = sz;
    (*pbufStr).pbufCur = (*pbufStr).pbufOrg;
    (*pbufStr).iLineNum = 0 as libc::c_int;
    (*pbufStr).iLNPrev = 0 as libc::c_int;
    (*pbufStr)
        .pInfo = if !pBuf.is_null() { (*pBuf).pInfo } else { 0 as *mut libc::c_void };
    if !pBuf.is_null() {
        (*pbufStr).iLineNum = (*pBuf).iLineNum;
        (*pbufStr).iLNPrev = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn FlushBuffer(mut pibIn: PINPUTBUF) {
    let mut pbuf: PBUF = (*pibIn).pbufOrg;
    while *pbuf != 0 {
        let fresh0 = pbuf;
        pbuf = pbuf.offset(1);
        printf(b"%c\0" as *const u8 as *const libc::c_char, *fresh0 as libc::c_int);
    }
    printf(b"\x10\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn GetArrayBounds(
    mut pibIn: PINPUTBUF,
    mut piLB: PLONG,
    mut piUB: PLONG,
) {
    let mut szTmp: PSTRLEX = [0; 1023];
    if ENextLex(pibIn, szTmp.as_mut_ptr(), 0x2 as libc::c_int) != 0 {
        ReportError(
            pibIn,
            (0x1 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            0 as PSTR,
            0 as PSTR,
        );
    } else {
        *piLB = atol(szTmp.as_mut_ptr());
        if *piLB < 0 as libc::c_int as libc::c_long {
            ReportError(
                pibIn,
                (0x100 as libc::c_int + 10 as libc::c_int | 0x8000 as libc::c_int)
                    as WORD,
                szTmp.as_mut_ptr(),
                0 as PSTR,
            );
        }
        if NextChar(pibIn) as libc::c_int == '-' as i32 {
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            (*pibIn).pbufCur;
            if ENextLex(pibIn, szTmp.as_mut_ptr(), 0x2 as libc::c_int) != 0 {
                ReportError(
                    pibIn,
                    (0x1 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    0 as PSTR,
                    0 as PSTR,
                );
            } else {
                *piUB = atol(szTmp.as_mut_ptr()) + 1 as libc::c_int as libc::c_long;
                if *piUB <= *piLB {
                    ReportError(
                        pibIn,
                        (0 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"\0" as *const u8 as *const libc::c_char as PSTR,
                        b"Upper bound must be higher than lower bound\0" as *const u8
                            as *const libc::c_char as PSTR,
                    );
                }
            }
            if GetPunct(pibIn, szTmp.as_mut_ptr(), ']' as i32 as libc::c_char) == 0 {
                ReportError(
                    pibIn,
                    (0x15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"]\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
        } else if GetPunct(pibIn, szTmp.as_mut_ptr(), ']' as i32 as libc::c_char) == 0 {
            ReportError(
                pibIn,
                (0x15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"]\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        } else {
            *piUB = *piLB + 1 as libc::c_int as libc::c_long;
        }
    };
}
pub unsafe extern "C" fn GetaString(mut pibIn: PINPUTBUF, mut szLex: PSTR) {
    let mut i: libc::c_int = 0 as libc::c_int;
    if pibIn.is_null() || szLex.is_null() {
        return;
    }
    if if !((*pibIn).pbufCur).is_null() {
        (*(*pibIn).pbufCur as libc::c_int == '"' as i32) as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        loop {
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            let fresh1 = i;
            i = i + 1;
            *szLex.offset(fresh1 as isize) = *(*pibIn).pbufCur;
            if !(*(*pibIn).pbufCur as libc::c_int != 0
                && *(*pibIn).pbufCur as libc::c_int != '"' as i32
                && i < 0x3ff as libc::c_int - 1 as libc::c_int)
            {
                break;
            }
        }
    }
    if i == 0x3ff as libc::c_int - 1 as libc::c_int {
        printf(
            b"\n***Error: max string length MAX_LEX exceeded in: %s\n\0" as *const u8
                as *const libc::c_char,
            szLex,
        );
        printf(b"Exiting...\n\n\0" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    }
    if *(*pibIn).pbufCur as libc::c_int == '"' as i32 {
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        (*pibIn).pbufCur;
        i -= 1;
        i;
    }
    *szLex.offset(i as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn GetFuncArgs(
    mut pibIn: PINPUTBUF,
    mut nArgs: libc::c_int,
    mut rgiArgTypes: *mut libc::c_int,
    mut szArgs: PSTR,
    mut rgiLowerB: *mut libc::c_long,
    mut rgiUpperB: *mut libc::c_long,
) -> BOOL {
    let mut bOK: BOOL = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut iType: libc::c_int = 0;
    let mut szPunct: PSTRLEX = [0; 1023];
    bOK = GetPunct(pibIn, szPunct.as_mut_ptr(), '(' as i32 as libc::c_char);
    if bOK == 0 {
        szPunct[1 as libc::c_int as usize] = '(' as i32 as libc::c_char;
        ReportError(pibIn, 0x14 as libc::c_int as WORD, szPunct.as_mut_ptr(), 0 as PSTR);
    }
    i = 0 as libc::c_int;
    while i < nArgs && bOK != 0 {
        if i != 0 {
            bOK = GetOptPunct(pibIn, szArgs, ',' as i32 as libc::c_char);
            if bOK == 0 {
                *szArgs.offset(1 as libc::c_int as isize) = ',' as i32 as libc::c_char;
                ReportError(pibIn, 0x14 as libc::c_int as WORD, szArgs, 0 as PSTR);
                break;
            }
        }
        NextLex(pibIn, szArgs, &mut iType);
        bOK
            &= (iType & *rgiArgTypes.offset(i as isize) > 0 as libc::c_int)
                as libc::c_int;
        if bOK == 0 {
            ReportError(
                pibIn,
                0x15 as libc::c_int as WORD,
                vrgszLexTypes[*rgiArgTypes.offset(i as isize) as usize],
                szArgs,
            );
        }
        let ref mut fresh2 = *rgiUpperB.offset(i as isize);
        *fresh2 = -(1 as libc::c_int) as libc::c_long;
        *rgiLowerB.offset(i as isize) = *fresh2;
        if GetPunct(pibIn, szPunct.as_mut_ptr(), '[' as i32 as libc::c_char) != 0 {
            GetArrayBounds(
                pibIn,
                &mut *rgiLowerB.offset(i as isize),
                &mut *rgiUpperB.offset(i as isize),
            );
        }
        i += 1;
        i;
        szArgs = szArgs.offset(0x3ff as libc::c_int as isize);
    }
    bOK = (szPunct[0 as libc::c_int as usize] as libc::c_int == ')' as i32
        || GetPunct(pibIn, szPunct.as_mut_ptr(), ')' as i32 as libc::c_char) != 0)
        as libc::c_int;
    if bOK == 0 {
        szPunct[1 as libc::c_int as usize] = ')' as i32 as libc::c_char;
        ReportError(pibIn, 0x14 as libc::c_int as WORD, szPunct.as_mut_ptr(), 0 as PSTR);
    }
    return bOK;
}
pub unsafe extern "C" fn GetIdentifier(mut pibIn: PINPUTBUF, mut szLex: PSTR) {
    let mut i: libc::c_int = 0 as libc::c_int;
    if pibIn.is_null() || szLex.is_null() {
        return;
    }
    if *(*__ctype_b_loc()).offset(*(*pibIn).pbufCur as libc::c_int as isize)
        as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
        || *(*pibIn).pbufCur as libc::c_int == '_' as i32
    {
        loop {
            let fresh3 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            let fresh4 = i;
            i = i + 1;
            *szLex.offset(fresh4 as isize) = *fresh3;
            if !(*(*pibIn).pbufCur as libc::c_int != 0
                && (*(*__ctype_b_loc()).offset(*(*pibIn).pbufCur as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *(*pibIn).pbufCur as libc::c_int == '_' as i32)
                && i < 0x3ff as libc::c_int - 1 as libc::c_int)
            {
                break;
            }
        }
    }
    *szLex.offset(i as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn GetInteger(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut piLexType: PINT,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut bHasSign: BOOL = 0 as libc::c_int;
    let mut eState: States_0 = Start_0;
    if pibIn.is_null() || szLex.is_null() || piLexType.is_null() {
        return;
    }
    eState = Start_0;
    *piLexType = 0 as libc::c_int;
    loop {
        c = *(*pibIn).pbufCur;
        if !(c as libc::c_int != 0 && i < 0x3ff as libc::c_int - 1 as libc::c_int
            && eState as libc::c_uint != End_0 as libc::c_int as libc::c_uint)
        {
            break;
        }
        match eState as libc::c_uint {
            0 => {
                if bHasSign == 0
                    && (c as libc::c_int == '+' as i32 || c as libc::c_int == '-' as i32)
                {
                    bHasSign = 1 as libc::c_int;
                } else if *(*__ctype_b_loc())
                    .offset(*(*pibIn).pbufCur as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    *piLexType = 0x2 as libc::c_int;
                    eState = Digits1_0;
                } else {
                    eState = End_0;
                }
            }
            1 => {
                if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    eState = End_0;
                }
            }
            2 | _ => {}
        }
        if eState as libc::c_uint != End_0 as libc::c_int as libc::c_uint {
            let fresh5 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            let fresh6 = i;
            i = i + 1;
            *szLex.offset(fresh6 as isize) = *fresh5;
        }
    }
    *szLex.offset(i as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn GetNumber(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut piLexType: PINT,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut bHasSign: BOOL = 0 as libc::c_int;
    let mut bLeadingDigits: BOOL = 0 as libc::c_int;
    let mut eState: States = Start;
    if pibIn.is_null() || szLex.is_null() || piLexType.is_null() {
        return;
    }
    eState = Start;
    *piLexType = 0 as libc::c_int;
    loop {
        c = *(*pibIn).pbufCur;
        if !(c as libc::c_int != 0 && i < 0x3ff as libc::c_int - 1 as libc::c_int
            && eState as libc::c_uint != End as libc::c_int as libc::c_uint)
        {
            break;
        }
        let mut current_block_40: u64;
        match eState as libc::c_uint {
            0 => {
                if c as libc::c_int == '.' as i32 {
                    eState = Point;
                } else if bHasSign == 0
                    && (c as libc::c_int == '+' as i32 || c as libc::c_int == '-' as i32)
                {
                    bHasSign = 1 as libc::c_int;
                } else if *(*__ctype_b_loc())
                    .offset(*(*pibIn).pbufCur as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    *piLexType = 0x2 as libc::c_int;
                    bLeadingDigits = *piLexType;
                    eState = Digits1;
                } else {
                    eState = End;
                }
                current_block_40 = 2480299350034459858;
            }
            1 => {
                if c as libc::c_int == '.' as i32 {
                    eState = Point;
                } else if c as libc::c_int == 'e' as i32
                    || c as libc::c_int == 'E' as i32
                {
                    eState = Exp;
                } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    eState = End;
                }
                current_block_40 = 2480299350034459858;
            }
            2 => {
                *piLexType = 0x4 as libc::c_int;
                if bLeadingDigits != 0
                    && (c as libc::c_int == 'e' as i32 || c as libc::c_int == 'E' as i32)
                {
                    eState = Exp;
                } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    eState = Digits2;
                } else {
                    if bLeadingDigits == 0 {
                        *piLexType = 0 as libc::c_int;
                    }
                    eState = End;
                }
                current_block_40 = 2480299350034459858;
            }
            3 => {
                if c as libc::c_int == 'e' as i32 || c as libc::c_int == 'E' as i32 {
                    eState = Exp;
                } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    eState = End;
                }
                current_block_40 = 2480299350034459858;
            }
            4 => {
                *piLexType = 0x4 as libc::c_int;
                if c as libc::c_int == '+' as i32 || c as libc::c_int == '-' as i32 {
                    eState = ExpSign;
                    current_block_40 = 2480299350034459858;
                } else {
                    current_block_40 = 2064595280536116385;
                }
            }
            5 => {
                current_block_40 = 2064595280536116385;
            }
            6 => {
                if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    eState = End;
                }
                current_block_40 = 2480299350034459858;
            }
            7 | _ => {
                current_block_40 = 2480299350034459858;
            }
        }
        match current_block_40 {
            2064595280536116385 => {
                if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    eState = Digits3;
                } else {
                    *piLexType = 0 as libc::c_int;
                    eState = End;
                }
            }
            _ => {}
        }
        if eState as libc::c_uint != End as libc::c_int as libc::c_uint {
            let fresh7 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            let fresh8 = i;
            i = i + 1;
            *szLex.offset(fresh8 as isize) = *fresh7;
        }
    }
    *szLex.offset(i as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn NextLex(
    mut pibIn: PINPUTBUF,
    mut szLex: *mut libc::c_char,
    mut piLexType: PINT,
) {
    static mut vszEqnPunct: [libc::c_char; 14] = unsafe {
        *::std::mem::transmute::<&[u8; 14], &mut [libc::c_char; 14]>(b"+-/*()><?:,!=\0")
    };
    let mut c: libc::c_char = 0;
    let mut fDone: BOOL = 0 as libc::c_int;
    *piLexType = 0 as libc::c_int;
    if pibIn.is_null() || szLex.is_null() || piLexType.is_null()
        || ((*pibIn).pbufCur).is_null() || *(*pibIn).pbufCur == 0
    {
        return;
    }
    while fDone == 0 {
        fDone = 1 as libc::c_int;
        SkipWhitespace(pibIn);
        if !(pibIn.is_null()
            || (((*pibIn).pbufCur).is_null() || *(*pibIn).pbufCur == 0)
                && (((*pibIn).pfileIn).is_null() || feof((*pibIn).pfileIn) != 0))
        {
            c = *(*pibIn).pbufCur;
            if c as libc::c_int == '#' as i32 {
                fDone = 0 as libc::c_int;
                SkipComment(pibIn);
            } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                != 0 || c as libc::c_int == '_' as i32
            {
                *piLexType = 0x1 as libc::c_int;
                GetIdentifier(pibIn, szLex);
            } else if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0 || c as libc::c_int == '.' as i32
                || (c as libc::c_int == '+' as i32 || c as libc::c_int == '-' as i32)
            {
                GetNumber(pibIn, szLex, piLexType);
                if (c as libc::c_int == '+' as i32 || c as libc::c_int == '-' as i32)
                    && *piLexType == 0
                {
                    *szLex.offset(0 as libc::c_int as isize) = c;
                    *szLex
                        .offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                    *piLexType = 0x20 as libc::c_int;
                }
            } else if c as libc::c_int == '"' as i32 {
                *piLexType = 0x10 as libc::c_int;
                GetaString(pibIn, szLex);
            } else if !(strchr(vszEqnPunct.as_mut_ptr(), c as libc::c_int)).is_null() {
                *piLexType = 0x20 as libc::c_int;
                let fresh9 = (*pibIn).pbufCur;
                (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                *szLex.offset(0 as libc::c_int as isize) = *fresh9;
                c = *(*pibIn).pbufCur;
                if c as libc::c_int != '=' as i32 {
                    *szLex
                        .offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                } else if *szLex.offset(0 as libc::c_int as isize) as libc::c_int
                    == '!' as i32
                    || *szLex.offset(0 as libc::c_int as isize) as libc::c_int
                        == '<' as i32
                    || *szLex.offset(0 as libc::c_int as isize) as libc::c_int
                        == '>' as i32
                    || *szLex.offset(0 as libc::c_int as isize) as libc::c_int
                        == '=' as i32
                {
                    let fresh10 = (*pibIn).pbufCur;
                    (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                    *szLex.offset(1 as libc::c_int as isize) = *fresh10;
                    *szLex
                        .offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                }
            } else {
                *piLexType = 0x8 as libc::c_int;
                let fresh11 = (*pibIn).pbufCur;
                (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                *szLex.offset(0 as libc::c_int as isize) = *fresh11;
                *szLex.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
        }
    }
}
pub unsafe extern "C" fn PreventLexSplit(
    mut pibIn: PINPUTBUF,
    mut iOffset: libc::c_int,
) {
    let mut lDelta: libc::c_long = 0;
    let mut pbufEOB: PBUF = ((*pibIn).pbufOrg).offset(iOffset as isize);
    let mut pbufEOBOld: PBUF = 0 as *mut libc::c_char;
    if !(pibIn.is_null()
        || (((*pibIn).pbufCur).is_null() || *(*pibIn).pbufCur == 0)
            && (((*pibIn).pfileIn).is_null() || feof((*pibIn).pfileIn) != 0))
        || iOffset == 0x1000 as libc::c_int
    {
        pbufEOBOld = pbufEOB;
        loop {
            pbufEOB = pbufEOB.offset(-1);
            if !(*pbufEOB as libc::c_int != '\n' as i32) {
                break;
            }
        }
        *pbufEOB = '\0' as i32 as libc::c_char;
        lDelta = pbufEOB.offset_from(pbufEOBOld) as libc::c_long;
        if lDelta != 0 {
            fseek((*pibIn).pfileIn, lDelta, 1 as libc::c_int);
        }
    } else {
        *pbufEOB = '\0' as i32 as libc::c_char;
    };
}
pub unsafe extern "C" fn SkipComment(mut pibIn: PINPUTBUF) {
    if pibIn.is_null() {
        return;
    }
    if *(*pibIn).pbufCur == 0 {
        FillBuffer(pibIn, 0x1000 as libc::c_int as libc::c_long);
    }
    loop {
        let fresh12 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh12 as libc::c_int != '\n' as i32) {
            break;
        }
        if !(*(*pibIn).pbufCur == 0) {
            continue;
        }
        if FillBuffer(pibIn, 0x1000 as libc::c_int as libc::c_long)
            == -(1 as libc::c_int)
        {
            break;
        }
    }
    (*pibIn).iLineNum += 1;
    (*pibIn).iLineNum;
    if *(*pibIn).pbufCur == 0 {
        FillBuffer(pibIn, 0x1000 as libc::c_int as libc::c_long);
    }
}
pub unsafe extern "C" fn NextChar(mut pibIn: PINPUTBUF) -> libc::c_char {
    if pibIn.is_null()
        || *(*pibIn).pbufCur == 0
            && FillBuffer(pibIn, 0x1000 as libc::c_int as libc::c_long)
                == -(1 as libc::c_int)
    {
        return 0 as libc::c_int as libc::c_char
    } else {
        return *(*pibIn).pbufCur
    };
}
pub unsafe extern "C" fn GetOptPunct(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut chPunct: libc::c_char,
) -> libc::c_int {
    let mut iReturn: libc::c_int = 0;
    let mut iType: libc::c_int = 0;
    iReturn = SkipWhitespace(pibIn);
    if NextChar(pibIn) as libc::c_int == chPunct as libc::c_int {
        iReturn = 1 as libc::c_int;
        NextLex(pibIn, szLex, &mut iType);
    }
    return iReturn;
}
pub unsafe extern "C" fn GetPunct(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut chPunct: libc::c_char,
) -> libc::c_int {
    let mut iType: libc::c_int = 0;
    NextLex(pibIn, szLex, &mut iType);
    return ((iType == 0x8 as libc::c_int || iType == 0x20 as libc::c_int)
        && *szLex.offset(0 as libc::c_int as isize) as libc::c_int
            == chPunct as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn EGetPunct(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut chPunct: libc::c_char,
) -> libc::c_int {
    let mut iReturn: libc::c_int = 0;
    iReturn = (GetPunct(pibIn, szLex, chPunct) == 0) as libc::c_int;
    if iReturn != 0 {
        *szLex.offset(1 as libc::c_int as isize) = chPunct;
        ReportError(pibIn, 0x14 as libc::c_int as WORD, szLex, 0 as PSTR);
    }
    return iReturn;
}
pub unsafe extern "C" fn EatStatement(mut pib: PINPUTBUF) {
    let mut c: libc::c_char = 0;
    if pib.is_null() {
        return;
    }
    loop {
        c = NextChar(pib);
        if !(c as libc::c_int != 0 && c as libc::c_int != ';' as i32) {
            break;
        }
        if c as libc::c_int == '\n' as i32 {
            (*pib).iLineNum += 1;
            (*pib).iLineNum;
        }
        (*pib).pbufCur = ((*pib).pbufCur).offset(1);
        (*pib).pbufCur;
    }
    if c != 0 {
        (*pib).pbufCur = ((*pib).pbufCur).offset(1);
        (*pib).pbufCur;
    }
}
pub unsafe extern "C" fn GetStatement(
    mut pibIn: PINPUTBUF,
    mut szStmt: PSTR,
    mut iKWCode: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut fDone: libc::c_int = 0 as libc::c_int;
    let mut iParCount: libc::c_int = 0 as libc::c_int;
    let mut bParOpen: BOOL = 0 as libc::c_int;
    let mut bEscaped: BOOL = 0 as libc::c_int;
    if pibIn.is_null() || szStmt.is_null() {
        return;
    }
    SkipWhitespace(pibIn);
    if !(pibIn.is_null()
        || (((*pibIn).pbufCur).is_null() || *(*pibIn).pbufCur == 0)
            && (((*pibIn).pfileIn).is_null() || feof((*pibIn).pfileIn) != 0))
    {
        while fDone == 0 {
            if *(*pibIn).pbufCur != 0 {
                if *(*pibIn).pbufCur as libc::c_int == '\\' as i32 {
                    let fresh13 = (*pibIn).pbufCur;
                    (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                    *fresh13;
                    if *(*pibIn).pbufCur as libc::c_int != '#' as i32 {
                        let mut szTmp: [libc::c_char; 3] = [0; 3];
                        sprintf(
                            szTmp.as_mut_ptr(),
                            b"\\%c\0" as *const u8 as *const libc::c_char,
                            *(*pibIn).pbufCur as libc::c_int,
                        );
                        ReportError(
                            pibIn,
                            (0x12 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            szTmp.as_mut_ptr(),
                            0 as PSTR,
                        );
                    }
                    bEscaped = 1 as libc::c_int;
                }
                fDone = (NextChar(pibIn) as libc::c_int == ';' as i32) as libc::c_int;
                if iKWCode == 30 as libc::c_int {
                    fDone = (fDone != 0 && bParOpen == 0) as libc::c_int;
                }
                if fDone == 0 {
                    if *(*pibIn).pbufCur as libc::c_int == '#' as i32 && bEscaped == 0 {
                        SkipComment(pibIn);
                    } else {
                        if bEscaped != 0 {
                            bEscaped = 0 as libc::c_int;
                        }
                        if i < 0x13ff as libc::c_int - 2 as libc::c_int {
                            let fresh14 = (*pibIn).pbufCur;
                            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                            let fresh15 = i;
                            i = i + 1;
                            let ref mut fresh16 = *szStmt.offset(fresh15 as isize);
                            *fresh16 = *fresh14;
                            if *fresh16 as libc::c_int == '\n' as i32 {
                                (*pibIn).iLineNum += 1;
                                (*pibIn).iLineNum;
                            }
                            if *szStmt.offset((i - 1 as libc::c_int) as isize)
                                as libc::c_int == '(' as i32
                            {
                                iParCount += 1;
                                iParCount;
                                bParOpen = 1 as libc::c_int;
                            }
                            if *szStmt.offset((i - 1 as libc::c_int) as isize)
                                as libc::c_int == ')' as i32
                            {
                                iParCount -= 1;
                                iParCount;
                            }
                            if iParCount == 0 as libc::c_int && bParOpen != 0 {
                                bParOpen = 0 as libc::c_int;
                            }
                        } else if bParOpen != 0 {
                            ReportError(
                                pibIn,
                                (0x100 as libc::c_int + 12 as libc::c_int
                                    | 0x8000 as libc::c_int) as WORD,
                                0 as PSTR,
                                0 as PSTR,
                            );
                        } else {
                            ReportError(
                                pibIn,
                                (0x100 as libc::c_int + 4 as libc::c_int
                                    | 0x8000 as libc::c_int) as WORD,
                                0 as PSTR,
                                0 as PSTR,
                            );
                        }
                    }
                } else if bParOpen != 0 {
                    ReportError(
                        pibIn,
                        (0x100 as libc::c_int + 12 as libc::c_int
                            | 0x8000 as libc::c_int) as WORD,
                        0 as PSTR,
                        0 as PSTR,
                    );
                }
            } else {
                ReportError(
                    pibIn,
                    (0x100 as libc::c_int + 12 as libc::c_int | 0x8000 as libc::c_int)
                        as WORD,
                    0 as PSTR,
                    0 as PSTR,
                );
            }
        }
        while *(*__ctype_b_loc())
            .offset(
                *szStmt.offset((i - 1 as libc::c_int) as isize) as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            i = i - 1 as libc::c_int;
        }
        *szStmt.offset(i as isize) = '\0' as i32 as libc::c_char;
    }
    if i == 0 {
        ReportError(
            pibIn,
            (0x15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"rvalue to assignment\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
}
pub unsafe extern "C" fn NextListItem(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut bIdTypes: libc::c_int,
    mut fItemNum: libc::c_int,
    mut cListTerm: libc::c_char,
) -> libc::c_int {
    let mut iType: libc::c_int = 0;
    let mut iReturn: libc::c_int = 0 as libc::c_int;
    if fItemNum == 0 || GetOptPunct(pibIn, szLex, ',' as i32 as libc::c_char) != 0 {
        if NextChar(pibIn) as libc::c_int != cListTerm as libc::c_int {
            NextLex(pibIn, szLex, &mut iType);
            if iType & bIdTypes != 0 {
                iReturn = 1 as libc::c_int;
            } else {
                iReturn = -(1 as libc::c_int);
            }
        }
    }
    return iReturn;
}
pub unsafe extern "C" fn SkipWhitespace(mut pibIn: PINPUTBUF) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut fSkipped: libc::c_int = 0 as libc::c_int;
    if pibIn.is_null() {
        return 0 as libc::c_int;
    }
    if *(*pibIn).pbufCur == 0 && !((*pibIn).pfileIn).is_null() {
        FillBuffer(pibIn, 0x1000 as libc::c_int as libc::c_long);
    }
    loop {
        c = *(*pibIn).pbufCur;
        if !(*(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            || c as libc::c_int == '#' as i32)
        {
            break;
        }
        fSkipped = 1 as libc::c_int;
        if c as libc::c_int == '#' as i32 {
            SkipComment(pibIn);
        } else {
            if c as libc::c_int == '\n' as i32 {
                (*pibIn).iLineNum += 1;
                (*pibIn).iLineNum;
            }
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            if !(*(*pibIn).pbufCur == 0 && !((*pibIn).pfileIn).is_null()) {
                continue;
            }
            if FillBuffer(pibIn, 0x1000 as libc::c_int as libc::c_long)
                == -(1 as libc::c_int)
            {
                break;
            }
        }
    }
    return fSkipped;
}
pub unsafe extern "C" fn UnrollEquation(
    mut pibIn: PINPUTBUF,
    mut index: libc::c_long,
    mut szEqn: PSTR,
    mut szEqnU: PSTR,
) {
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut m: libc::c_int = 0;
    let mut bExpress: BOOL = 0 as libc::c_int;
    let mut szExpression: PSTRLEX = [0; 1023];
    while *szEqn.offset(j as isize) as libc::c_int != '\0' as i32
        && k < 0x13ff as libc::c_int - 1 as libc::c_int
    {
        if bExpress != 0 {
            m = 0 as libc::c_int;
            while *szEqn.offset(j as isize) as libc::c_int != '\0' as i32
                && *szEqn.offset(j as isize) as libc::c_int != ']' as i32
                && m < 0x13ff as libc::c_int - 1 as libc::c_int
            {
                szExpression[m as usize] = *szEqn.offset(j as isize);
                j += 1;
                j;
                m += 1;
                m;
            }
            if *szEqn.offset(j as isize) as libc::c_int == ']' as i32 {
                j += 1;
                j;
                bExpress = 0 as libc::c_int;
            }
            if *szEqn.offset(j as isize) as libc::c_int != '\0' as i32
                && m == 0x13ff as libc::c_int - 1 as libc::c_int
            {
                ReportError(
                    pibIn,
                    (0x100 as libc::c_int + 4 as libc::c_int | 0x8000 as libc::c_int)
                        as WORD,
                    0 as PSTR,
                    b"(Occured while unrolling a loop)\0" as *const u8
                        as *const libc::c_char as PSTR,
                );
            }
            szExpression[m as usize] = '\0' as i32 as libc::c_char;
            sprintf(
                szExpression.as_mut_ptr(),
                b"%ld\0" as *const u8 as *const libc::c_char,
                EvaluateExpression(pibIn, index, szExpression.as_mut_ptr()),
            );
            m = 0 as libc::c_int;
            while szExpression[m as usize] as libc::c_int != '\0' as i32
                && m < 0x13ff as libc::c_int - 1 as libc::c_int
            {
                *szEqnU.offset(k as isize) = szExpression[m as usize];
                k += 1;
                k;
                m += 1;
                m;
            }
        } else {
            let mut current_block_28: u64;
            match *szEqn.offset(j as isize) as libc::c_int {
                91 => {
                    *szEqnU.offset(k as isize) = '_' as i32 as libc::c_char;
                    j += 1;
                    j;
                    k += 1;
                    k;
                    bExpress = 1 as libc::c_int;
                    current_block_28 = 16203760046146113240;
                }
                93 => {
                    ReportError(
                        pibIn,
                        (0x11 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"]\0" as *const u8 as *const libc::c_char as PSTR,
                        b"(Could be nested brackets)\0" as *const u8
                            as *const libc::c_char as PSTR,
                    );
                    current_block_28 = 1624255800480114126;
                }
                _ => {
                    current_block_28 = 1624255800480114126;
                }
            }
            match current_block_28 {
                1624255800480114126 => {
                    *szEqnU.offset(k as isize) = *szEqn.offset(j as isize);
                    j += 1;
                    j;
                    k += 1;
                    k;
                }
                _ => {}
            }
        }
    }
    if *szEqn.offset(j as isize) as libc::c_int != '\0' as i32
        && k == 0x13ff as libc::c_int - 1 as libc::c_int
    {
        ReportError(
            pibIn,
            (0x100 as libc::c_int + 4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            0 as PSTR,
            b"(Occured in UnrollEquation)\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
    *szEqnU.offset(k as isize) = '\0' as i32 as libc::c_char;
}
