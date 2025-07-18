use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    pub pbufCur: PBUF,
    pub iLineNum: libc::c_int,
    pub iLNPrev: libc::c_int,
    pub cErrors: libc::c_int,
    pub pInfo: PVOID,
}
pub type PINPUTBUF = *mut tagINPUTBUF;
pub type PSTRLEX_ = [libc::c_char; 255];
pub const End: States = 7;
pub type States = libc::c_uint;
pub const Digits3: States = 6;
pub const ExpSign: States = 5;
pub const Exp: States = 4;
pub const Digits2: States = 3;
pub const Point: States = 2;
pub const Digits1: States = 1;
pub const Start: States = 0;
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
        || iOffset == 0x10000 as libc::c_int
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
pub unsafe extern "C" fn FillBuffer(mut pibIn: PINPUTBUF) -> libc::c_int {
    let mut iReturn: libc::c_int = 0 as libc::c_int;
    let mut iOffset: libc::c_int = 0;
    if !pibIn.is_null() && !((*pibIn).pfileIn).is_null() && !((*pibIn).pbufOrg).is_null()
    {
        iOffset = fread(
            (*pibIn).pbufOrg as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            0x10000 as libc::c_int as libc::c_ulong,
            (*pibIn).pfileIn,
        ) as libc::c_int;
        if iOffset != 0 {
            iReturn = iOffset;
            PreventLexSplit(pibIn, iOffset);
            (*pibIn).pbufCur = (*pibIn).pbufOrg;
        } else if feof((*pibIn).pfileIn) != 0 {
            iReturn = -(1 as libc::c_int);
        } else {
            ReportError(
                pibIn,
                0x8000 as libc::c_int as WORD,
                b"Unexpected end of file.\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    }
    return iReturn;
}
pub unsafe extern "C" fn InitBuffer(
    mut pibIn: PINPUTBUF,
    mut szFullPathname: PSTR,
) -> BOOL {
    let mut bReturn: BOOL = 0 as libc::c_int;
    if pibIn.is_null() {
        return 0 as libc::c_int;
    }
    (*pibIn).iLineNum = 1 as libc::c_int;
    (*pibIn).iLNPrev = 0 as libc::c_int;
    (*pibIn).cErrors = 0 as libc::c_int;
    (*pibIn).pInfo = 0 as *mut libc::c_void;
    (*pibIn).pbufCur = 0 as PBUF;
    (*pibIn)
        .pfileIn = fopen(
        szFullPathname as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !((*pibIn).pfileIn).is_null() {
        (*pibIn).pbufOrg = malloc(0x10000 as libc::c_int as libc::c_ulong) as PBUF;
        if !((*pibIn).pbufOrg).is_null() {
            bReturn = FillBuffer(pibIn);
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
            szFullPathname,
            0 as PSTR,
        );
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
pub unsafe extern "C" fn SkipWhitespace(mut pibIn: PINPUTBUF) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut fSkipped: libc::c_int = 0 as libc::c_int;
    if pibIn.is_null() {
        return 0 as libc::c_int;
    }
    if *(*pibIn).pbufCur == 0 && !((*pibIn).pfileIn).is_null() {
        FillBuffer(pibIn);
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
            if FillBuffer(pibIn) == -(1 as libc::c_int) {
                break;
            }
        }
    }
    return fSkipped;
}
pub unsafe extern "C" fn GetArrayBounds(
    mut pibIn: PINPUTBUF,
    mut piLB: PLONG,
    mut piUB: PLONG,
) {
    let mut szTmp: PSTRLEX_ = [0; 255];
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
                (0x122 as libc::c_int | 0x8000 as libc::c_int) as WORD,
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
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"]\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
        } else if GetPunct(pibIn, szTmp.as_mut_ptr(), ']' as i32 as libc::c_char) == 0 {
            ReportError(
                pibIn,
                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
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
                && i < 255 as libc::c_int - 1 as libc::c_int)
            {
                break;
            }
        }
    }
    if *(*pibIn).pbufCur as libc::c_int == '"' as i32 {
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        (*pibIn).pbufCur;
        i -= 1;
        i;
    }
    *szLex.offset(i as isize) = '\0' as i32 as libc::c_char;
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
            let fresh2 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            let fresh3 = i;
            i = i + 1;
            *szLex.offset(fresh3 as isize) = *fresh2;
            if !(*(*pibIn).pbufCur as libc::c_int != 0
                && (*(*__ctype_b_loc()).offset(*(*pibIn).pbufCur as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || *(*pibIn).pbufCur as libc::c_int == '_' as i32)
                && i < 255 as libc::c_int - 1 as libc::c_int)
            {
                break;
            }
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
        if !(c as libc::c_int != 0 && i < 255 as libc::c_int - 1 as libc::c_int
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
                    current_block_40 = 14748314904637597825;
                }
            }
            5 => {
                current_block_40 = 14748314904637597825;
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
            14748314904637597825 => {
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
            let fresh4 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            let fresh5 = i;
            i = i + 1;
            *szLex.offset(fresh5 as isize) = *fresh4;
        }
    }
    *szLex.offset(i as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn NextLex(
    mut pibIn: PINPUTBUF,
    mut szLex: *mut libc::c_char,
    mut piLexType: PINT,
) {
    let mut c: libc::c_char = 0;
    let mut fDone: BOOL = 0 as libc::c_int;
    *piLexType = 0 as libc::c_int;
    if pibIn.is_null() || szLex.is_null() || piLexType.is_null()
        || ((*pibIn).pbufCur).is_null()
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
                    *piLexType = 0x8 as libc::c_int;
                }
            } else if c as libc::c_int == '"' as i32 {
                *piLexType = 0x10 as libc::c_int;
                GetaString(pibIn, szLex);
            } else {
                *piLexType = 0x8 as libc::c_int;
                let fresh6 = (*pibIn).pbufCur;
                (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                *szLex.offset(0 as libc::c_int as isize) = *fresh6;
                *szLex.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
        }
    }
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
            0x14 as libc::c_int as WORD,
            vrgszLexTypes[iType as usize],
            szLex,
        );
    }
    return iErr;
}
pub unsafe extern "C" fn SkipComment(mut pibIn: PINPUTBUF) {
    if pibIn.is_null() {
        return;
    }
    if *(*pibIn).pbufCur == 0 {
        FillBuffer(pibIn);
    }
    loop {
        let fresh7 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh7 as libc::c_int != '\n' as i32) {
            break;
        }
        if !(*(*pibIn).pbufCur == 0) {
            continue;
        }
        if FillBuffer(pibIn) == -(1 as libc::c_int) {
            break;
        }
    }
    (*pibIn).iLineNum += 1;
    (*pibIn).iLineNum;
    if *(*pibIn).pbufCur == 0 {
        FillBuffer(pibIn);
    }
}
pub unsafe extern "C" fn NextChar(mut pibIn: PINPUTBUF) -> libc::c_char {
    if pibIn.is_null()
        || *(*pibIn).pbufCur == 0 && FillBuffer(pibIn) == -(1 as libc::c_int)
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
    return (iType == 0x8 as libc::c_int
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
        ReportError(pibIn, 0x13 as libc::c_int as WORD, szLex, 0 as PSTR);
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
pub unsafe extern "C" fn GetStatement(mut pibIn: PINPUTBUF, mut szStmt: PSTR) {
    let mut i: libc::c_int = 0;
    let mut fDone: libc::c_int = 0 as libc::c_int;
    if pibIn.is_null() || szStmt.is_null() {
        return;
    }
    SkipWhitespace(pibIn);
    i = 0 as libc::c_int;
    if !(pibIn.is_null()
        || (((*pibIn).pbufCur).is_null() || *(*pibIn).pbufCur == 0)
            && (((*pibIn).pfileIn).is_null() || feof((*pibIn).pfileIn) != 0))
    {
        while fDone == 0 {
            if *(*pibIn).pbufCur != 0 {
                fDone = (NextChar(pibIn) as libc::c_int == ';' as i32) as libc::c_int;
                if fDone == 0 {
                    if i < 0x3ff as libc::c_int - 2 as libc::c_int {
                        let fresh8 = (*pibIn).pbufCur;
                        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                        let fresh9 = i;
                        i = i + 1;
                        let ref mut fresh10 = *szStmt.offset(fresh9 as isize);
                        *fresh10 = *fresh8;
                        if *fresh10 as libc::c_int == '\n' as i32 {
                            (*pibIn).iLineNum += 1;
                            (*pibIn).iLineNum;
                        }
                    } else {
                        ReportError(
                            pibIn,
                            (0x104 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            0 as PSTR,
                            0 as PSTR,
                        );
                    }
                }
            } else {
                fDone = (FillBuffer(pibIn) == -(1 as libc::c_int)) as libc::c_int;
            }
        }
        *szStmt.offset(i as isize) = '\0' as i32 as libc::c_char;
    }
    if i == 0 {
        ReportError(
            pibIn,
            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
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
pub unsafe extern "C" fn GetFuncArgs(
    mut pibIn: PINPUTBUF,
    mut nArgs: libc::c_int,
    mut rgiArgTypes: *mut libc::c_int,
    mut szArgs: PSTR,
) -> BOOL {
    let mut bOK: BOOL = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut iType: libc::c_int = 0;
    let mut szPunct: PSTRLEX_ = [0; 255];
    bOK = GetPunct(pibIn, szPunct.as_mut_ptr(), '(' as i32 as libc::c_char);
    if bOK == 0 {
        szPunct[1 as libc::c_int as usize] = '(' as i32 as libc::c_char;
        ReportError(pibIn, 0x13 as libc::c_int as WORD, szPunct.as_mut_ptr(), 0 as PSTR);
    }
    i = 0 as libc::c_int;
    while i < nArgs && bOK != 0 {
        if i != 0 {
            bOK = GetOptPunct(pibIn, szArgs, ',' as i32 as libc::c_char);
            if bOK == 0 {
                *szArgs.offset(1 as libc::c_int as isize) = ',' as i32 as libc::c_char;
                ReportError(pibIn, 0x13 as libc::c_int as WORD, szArgs, 0 as PSTR);
                return bOK;
            }
        }
        NextLex(pibIn, szArgs, &mut iType);
        bOK
            &= (iType & *rgiArgTypes.offset(i as isize) > 0 as libc::c_int)
                as libc::c_int;
        if bOK == 0 {
            ReportError(
                pibIn,
                0x14 as libc::c_int as WORD,
                vrgszLexTypes[*rgiArgTypes.offset(i as isize) as usize],
                szArgs,
            );
            return bOK;
        }
        i += 1;
        i;
        szArgs = szArgs.offset(255 as libc::c_int as isize);
    }
    bOK = GetPunct(pibIn, szPunct.as_mut_ptr(), ')' as i32 as libc::c_char);
    if bOK == 0 {
        szPunct[1 as libc::c_int as usize] = ')' as i32 as libc::c_char;
        ReportError(pibIn, 0x13 as libc::c_int as WORD, szPunct.as_mut_ptr(), 0 as PSTR);
    }
    return bOK;
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
    let mut szExpression: PSTRLEX_ = [0; 255];
    while *szEqn.offset(j as isize) as libc::c_int != '\0' as i32
        && k < 0x3ff as libc::c_int - 1 as libc::c_int
    {
        if bExpress != 0 {
            m = 0 as libc::c_int;
            while *szEqn.offset(j as isize) as libc::c_int != '\0' as i32
                && *szEqn.offset(j as isize) as libc::c_int != ']' as i32
                && m < 0x3ff as libc::c_int - 1 as libc::c_int
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
                && m == 0x3ff as libc::c_int - 1 as libc::c_int
            {
                ReportError(
                    pibIn,
                    (0x104 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    0 as PSTR,
                    b"(Occured while unrolling a loop)\0" as *const u8
                        as *const libc::c_char as PSTR,
                );
            }
            szExpression[m as usize] = '\0' as i32 as libc::c_char;
            m = 0 as libc::c_int;
            while szExpression[m as usize] as libc::c_int != '\0' as i32
                && m < 0x3ff as libc::c_int - 1 as libc::c_int
            {
                *szEqnU.offset(k as isize) = szExpression[m as usize];
                k += 1;
                k;
                m += 1;
                m;
            }
        } else {
            let mut current_block_27: u64;
            match *szEqn.offset(j as isize) as libc::c_int {
                91 => {
                    *szEqnU.offset(k as isize) = '_' as i32 as libc::c_char;
                    j += 1;
                    j;
                    k += 1;
                    k;
                    bExpress = 1 as libc::c_int;
                    current_block_27 = 2719512138335094285;
                }
                93 => {
                    ReportError(
                        pibIn,
                        (0x11 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"]\0" as *const u8 as *const libc::c_char as PSTR,
                        b"(Could be nested brackets)\0" as *const u8
                            as *const libc::c_char as PSTR,
                    );
                    current_block_27 = 14960392761403554468;
                }
                _ => {
                    current_block_27 = 14960392761403554468;
                }
            }
            match current_block_27 {
                14960392761403554468 => {
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
        && k == 0x3ff as libc::c_int - 1 as libc::c_int
    {
        ReportError(
            pibIn,
            (0x104 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            0 as PSTR,
            b"(Occured in UnrollEquation)\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
    *szEqnU.offset(k as isize) = '\0' as i32 as libc::c_char;
}
