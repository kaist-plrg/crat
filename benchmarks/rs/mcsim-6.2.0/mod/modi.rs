use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn GetPunct(pibIn: PINPUTBUF, szLex: PSTR, chPunct: libc::c_char) -> libc::c_int;
    fn EatStatement(pib: PINPUTBUF);
    fn GetArrayBounds(pibIn: PINPUTBUF, piLB: PLONG, piUB: PLONG);
    fn GetFuncArgs(
        pibIn: PINPUTBUF,
        nArgs: libc::c_int,
        rgiArgTypes: *mut libc::c_int,
        szArgs: PSTR,
        rgiLowerB: *mut libc::c_long,
        rgiUpperB: *mut libc::c_long,
    ) -> BOOL;
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn GetStatement(pibIn: PINPUTBUF, szStmt: PSTR, iKWCode: libc::c_int);
    fn InitBuffer(
        pibIn: PINPUTBUF,
        lBuffer_size: libc::c_long,
        szFullPathname: PSTR,
    ) -> BOOL;
    fn NextLex(_: PINPUTBUF, _: *mut libc::c_char, _: PINT);
    fn SkipComment(_: PINPUTBUF);
    fn UnrollEquation(pibIn: PINPUTBUF, index: libc::c_long, szEqn: PSTR, szEqnU: PSTR);
    fn DeclareModelVar(pibIn: PINPUTBUF, szName: PSTR, iKWCode: libc::c_int);
    fn DefineVariable(pibIn: PINPUTBUF, szName: PSTR, szEqn: PSTR, iKWCode: libc::c_int);
    fn GetVarType(pvm: PVMMAPSTRCT, szName: PSTR) -> libc::c_int;
    fn ReadSBMLModels(pibIn: PINPUTBUF);
    fn ReadPKTemplate(pibIn: PINPUTBUF);
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
pub type INPUTBUF = tagINPUTBUF;
pub type PINPUTBUF = *mut tagINPUTBUF;
pub type PSTRLEX = [libc::c_char; 1023];
pub type PSTREQN = [libc::c_char; 5119];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagVM {
    pub szName: PSTR,
    pub szEqn: PSTR,
    pub hType: HANDLE,
    pub pvmNextVar: *mut tagVM,
}
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
pub struct tagKM {
    pub szKeyword: PSTR,
    pub iKWCode: libc::c_int,
    pub fContext: WORD,
}
pub type KM = tagKM;
pub type PKM = *mut tagKM;
pub static mut vrgkmKeywordMap: [KM; 19] = [
    {
        let mut init = tagKM {
            szKeyword: b"States\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 1 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Inputs\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 2 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Forcings\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 2 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Outputs\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 3 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Compartments\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 42 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Dynamics\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 4 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Initialize\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 5 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Scale\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 5 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Jacobian\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 6 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Jacob\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 6 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Events\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 8 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Roots\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 9 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"CalcOutputs\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 7 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"dt\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 20 as libc::c_int,
            fContext: (0x2 as libc::c_int | 0x100 as libc::c_int) as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"Inline\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 30 as libc::c_int,
            fContext: 0xffff as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"SBMLModels\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 40 as libc::c_int,
            fContext: (0x1 as libc::c_int | 0x200 as libc::c_int) as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"PKTemplate\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 41 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"End\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 100 as libc::c_int,
            fContext: (0x1 as libc::c_int | 0x200 as libc::c_int) as WORD,
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
pub unsafe extern "C" fn GetKeyword(mut iCode: libc::c_int) -> PSTR {
    let mut pkm: PKM = &mut *vrgkmKeywordMap
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut KM;
    while *(*pkm).szKeyword as libc::c_int != 0 && (*pkm).iKWCode != iCode {
        pkm = pkm.offset(1);
        pkm;
    }
    return (*pkm).szKeyword;
}
pub unsafe extern "C" fn GetKeywordCode(
    mut szKeyword: PSTR,
    mut pfContext: PINT,
) -> libc::c_int {
    let mut pkm: PKM = &mut *vrgkmKeywordMap
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut KM;
    while *(*pkm).szKeyword as libc::c_int != 0
        && strcmp(
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
pub unsafe extern "C" fn GetVarList(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut iKWCode: libc::c_int,
) {
    let mut iLexType: libc::c_int = 0;
    let mut iErr: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_long = 0;
    let mut iLB: libc::c_long = 0;
    let mut iUB: libc::c_long = 0;
    let mut szPunct: PSTRLEX = [0; 1023];
    let mut szTmp: PSTRLEX = [0; 1023];
    loop {
        NextLex(pibIn, szLex, &mut iLexType);
        if iLexType & 0x1 as libc::c_int != 0 {
            if GetPunct(pibIn, szPunct.as_mut_ptr(), '[' as i32 as libc::c_char) != 0 {
                GetArrayBounds(pibIn, &mut iLB, &mut iUB);
                i = iLB;
                while i < iUB {
                    sprintf(
                        szTmp.as_mut_ptr(),
                        b"%s_%ld\0" as *const u8 as *const libc::c_char,
                        szLex,
                        i,
                    );
                    DeclareModelVar(pibIn, szTmp.as_mut_ptr(), iKWCode);
                    i += 1;
                    i;
                }
            } else {
                let fresh0 = (*pibIn).pbufCur;
                (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(-1);
                *fresh0;
                DeclareModelVar(pibIn, szLex, iKWCode);
            }
        } else if *szLex.offset(0 as libc::c_int as isize) as libc::c_int != ',' as i32
            && *szLex.offset(0 as libc::c_int as isize) as libc::c_int != '}' as i32
        {
            szPunct[1 as libc::c_int as usize] = '}' as i32 as libc::c_char;
            iErr = szPunct[1 as libc::c_int as usize] as libc::c_int;
            ReportError(
                pibIn,
                (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                szPunct.as_mut_ptr(),
                b"* List must be comma-delimited and end with }.\0" as *const u8
                    as *const libc::c_char as PSTR,
            );
        }
        if !(*szLex.offset(0 as libc::c_int as isize) as libc::c_int != '}' as i32
            && iErr == 0)
        {
            break;
        }
    };
}
pub unsafe extern "C" fn ProcessDTStatement(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut szEqn: PSTR,
    mut iKWCode: libc::c_int,
) {
    let mut szPunct: PSTRLEX = [0; 1023];
    let mut szTmp: PSTRLEX = [0; 1023];
    let mut szEqnU: PSTREQN = [0; 5119];
    let mut pinfo: PINPUTINFO = 0 as *mut tagINPUTINFO;
    let mut iArgType: libc::c_int = 0x1 as libc::c_int;
    let mut i: libc::c_long = 0;
    let mut iLB: libc::c_long = 0;
    let mut iUB: libc::c_long = 0;
    pinfo = (*pibIn).pInfo as PINPUTINFO;
    if GetFuncArgs(pibIn, 1 as libc::c_int, &mut iArgType, szLex, &mut iLB, &mut iUB)
        == 0
    {
        ReportError(
            pibIn,
            (0x100 as libc::c_int + 5 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szLex,
            0 as PSTR,
        );
    }
    if GetPunct(pibIn, szPunct.as_mut_ptr(), '=' as i32 as libc::c_char) == 0 {
        ReportError(
            pibIn,
            0x15 as libc::c_int as WORD,
            b"=\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    if iUB == -(1 as libc::c_int) as libc::c_long {
        if GetVarType((*pinfo).pvmGloVars, szLex) != 0x10000 as libc::c_int {
            ReportError(
                pibIn,
                (0x100 as libc::c_int + 5 as libc::c_int | 0x8000 as libc::c_int)
                    as WORD,
                szLex,
                0 as PSTR,
            );
        }
        GetStatement(pibIn, szEqn, iKWCode);
        UnrollEquation(
            pibIn,
            0 as libc::c_int as libc::c_long,
            szEqn,
            szEqnU.as_mut_ptr(),
        );
        DefineVariable(pibIn, szLex, szEqnU.as_mut_ptr(), iKWCode);
    } else {
        GetStatement(pibIn, szEqn, iKWCode);
        i = iLB;
        while i < iUB {
            sprintf(
                szTmp.as_mut_ptr(),
                b"%s_%ld\0" as *const u8 as *const libc::c_char,
                szLex,
                i,
            );
            if GetVarType((*pinfo).pvmGloVars, szTmp.as_mut_ptr())
                != 0x10000 as libc::c_int
            {
                sprintf(
                    szTmp.as_mut_ptr(),
                    b"%s[%ld]\0" as *const u8 as *const libc::c_char,
                    szLex,
                    i,
                );
                ReportError(
                    pibIn,
                    (0x100 as libc::c_int + 5 as libc::c_int | 0x8000 as libc::c_int)
                        as WORD,
                    szTmp.as_mut_ptr(),
                    0 as PSTR,
                );
            }
            UnrollEquation(pibIn, i, szEqn, szEqnU.as_mut_ptr());
            DefineVariable(pibIn, szTmp.as_mut_ptr(), szEqnU.as_mut_ptr(), iKWCode);
            i += 1;
            i;
        }
    }
    if GetPunct(pibIn, szLex, ';' as i32 as libc::c_char) == 0 {
        ReportError(
            pibIn,
            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b";\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
}
pub unsafe extern "C" fn ProcessIdentifier(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut szEqn: PSTR,
    mut iKWCode: libc::c_int,
) {
    let mut szPunct: PSTRLEX = [0; 1023];
    let mut szTmp: PSTRLEX = [0; 1023];
    let mut szEqnU: PSTREQN = [0; 5119];
    let mut pinfo: PINPUTINFO = 0 as *mut tagINPUTINFO;
    let mut i: libc::c_long = 0;
    let mut iLB: libc::c_long = 0;
    let mut iUB: libc::c_long = 0;
    pinfo = (*pibIn).pInfo as PINPUTINFO;
    i = strlen(szLex as *const libc::c_char) as libc::c_long;
    if i > 80 as libc::c_int as libc::c_long {
        ReportError(
            pibIn,
            (0x100 as libc::c_int + 11 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szLex,
            0 as PSTR,
        );
    }
    if GetPunct(pibIn, szPunct.as_mut_ptr(), '[' as i32 as libc::c_char) == 0 {
        if szPunct[0 as libc::c_int as usize] as libc::c_int == '=' as i32 {
            GetStatement(pibIn, szEqn, iKWCode);
            UnrollEquation(
                pibIn,
                0 as libc::c_int as libc::c_long,
                szEqn,
                szEqnU.as_mut_ptr(),
            );
            DefineVariable(pibIn, szLex, szEqnU.as_mut_ptr(), iKWCode);
            if GetPunct(pibIn, szLex, ';' as i32 as libc::c_char) == 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b";\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
        } else if szPunct[0 as libc::c_int as usize] as libc::c_int == ';' as i32 {
            if (*pinfo).wContext == 0x1 as libc::c_int as libc::c_uint {
                DefineVariable(
                    pibIn,
                    szLex,
                    b"0\0\0" as *const u8 as *const libc::c_char as PSTR,
                    iKWCode,
                );
            } else {
                ReportError(
                    pibIn,
                    (0x15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"= or [\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
        } else {
            ReportError(
                pibIn,
                (0x15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"=, [ or ;\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    } else {
        GetArrayBounds(pibIn, &mut iLB, &mut iUB);
        if GetPunct(pibIn, szPunct.as_mut_ptr(), '=' as i32 as libc::c_char) != 0 {
            GetStatement(pibIn, szEqn, iKWCode);
            i = iLB;
            while i < iUB {
                sprintf(
                    szTmp.as_mut_ptr(),
                    b"%s_%ld\0" as *const u8 as *const libc::c_char,
                    szLex,
                    i,
                );
                UnrollEquation(pibIn, i, szEqn, szEqnU.as_mut_ptr());
                DefineVariable(pibIn, szTmp.as_mut_ptr(), szEqnU.as_mut_ptr(), iKWCode);
                i += 1;
                i;
            }
            if GetPunct(pibIn, szLex, ';' as i32 as libc::c_char) == 0 {
                ReportError(
                    pibIn,
                    (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b";\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
        } else if szPunct[0 as libc::c_int as usize] as libc::c_int == ';' as i32 {
            if (*pinfo).wContext == 0x1 as libc::c_int as libc::c_uint {
                i = iLB;
                while i < iUB {
                    sprintf(
                        szTmp.as_mut_ptr(),
                        b"%s_%ld\0" as *const u8 as *const libc::c_char,
                        szLex,
                        i,
                    );
                    DefineVariable(
                        pibIn,
                        szTmp.as_mut_ptr(),
                        b"0\0\0" as *const u8 as *const libc::c_char as PSTR,
                        iKWCode,
                    );
                    i += 1;
                    i;
                }
            } else {
                ReportError(
                    pibIn,
                    (0x15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                    b"= or [\0" as *const u8 as *const libc::c_char as PSTR,
                    0 as PSTR,
                );
            }
        } else {
            ReportError(
                pibIn,
                (0x15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"= or ;\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    };
}
pub unsafe extern "C" fn ProcessInlineStatement(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut szEqn: PSTR,
    mut iKWCode: libc::c_int,
) {
    GetStatement(pibIn, szEqn, iKWCode);
    szEqn = szEqn.offset(1 as libc::c_int as isize);
    *szEqn
        .offset(
            (strlen(szEqn as *const libc::c_char))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\0' as i32 as libc::c_char;
    DefineVariable(pibIn, szLex, szEqn, iKWCode);
    if GetPunct(pibIn, szLex, ';' as i32 as libc::c_char) == 0 {
        ReportError(
            pibIn,
            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b";\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
}
pub unsafe extern "C" fn ProcessWord(
    mut pibIn: PINPUTBUF,
    mut szLex: PSTR,
    mut szEqn: PSTR,
) {
    let mut iErr: libc::c_int = 0 as libc::c_int;
    let mut iKWCode: libc::c_int = 0;
    let mut fContext: libc::c_int = 0;
    let mut szPunct: PSTRLEX = [0; 1023];
    let mut pinfo: PINPUTINFO = 0 as *mut tagINPUTINFO;
    static mut bCalcOutputsDefined: BOOL = 0 as libc::c_int;
    static mut bDynamicsDefined: BOOL = 0 as libc::c_int;
    static mut bInitializeDefined: BOOL = 0 as libc::c_int;
    static mut bJacobianDefined: BOOL = 0 as libc::c_int;
    if pibIn.is_null() || szLex.is_null()
        || *szLex.offset(0 as libc::c_int as isize) == 0 || szEqn.is_null()
    {
        return;
    }
    pinfo = (*pibIn).pInfo as PINPUTINFO;
    iKWCode = GetKeywordCode(szLex, &mut fContext);
    if (*pinfo).wContext != 0x4000 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"pinfo->wContext != CN_END\0" as *const u8 as *const libc::c_char,
            b"modi.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void ProcessWord(PINPUTBUF, PSTR, PSTR)\0"))
                .as_ptr(),
        );
    }
    'c_4935: {
        if (*pinfo).wContext != 0x4000 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"pinfo->wContext != CN_END\0" as *const u8 as *const libc::c_char,
                b"modi.c\0" as *const u8 as *const libc::c_char,
                317 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void ProcessWord(PINPUTBUF, PSTR, PSTR)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*pinfo).wContext == 0x4000 as libc::c_int as libc::c_uint
        || iKWCode != 0 && fContext as libc::c_uint & (*pinfo).wContext == 0
    {
        ReportError(
            pibIn,
            (0x100 as libc::c_int + 1 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szLex,
            0 as PSTR,
        );
    } else {
        match iKWCode {
            100 => {
                (*pinfo).wContext = 0x4000 as libc::c_int as WORD;
            }
            1 | 2 | 3 | 42 => {
                if GetPunct(pibIn, szPunct.as_mut_ptr(), '=' as i32 as libc::c_char) != 0
                {
                    if GetPunct(pibIn, szPunct.as_mut_ptr(), '{' as i32 as libc::c_char)
                        != 0
                    {
                        GetVarList(pibIn, szLex, iKWCode);
                    } else {
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"{\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                } else {
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"=\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
            }
            7 => {
                if bCalcOutputsDefined != 0 {
                    ReportError(
                        pibIn,
                        (0x100 as libc::c_int + 14 as libc::c_int
                            | 0x8000 as libc::c_int) as WORD,
                        b"CalcOutputs\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                bCalcOutputsDefined = 1 as libc::c_int;
                if GetPunct(pibIn, szPunct.as_mut_ptr(), '{' as i32 as libc::c_char) == 0
                {
                    szPunct[1 as libc::c_int as usize] = '{' as i32 as libc::c_char;
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        szPunct.as_mut_ptr(),
                        b"* Section must be delimited by curly braces.\0" as *const u8
                            as *const libc::c_char as PSTR,
                    );
                } else {
                    (*pinfo)
                        .wContext = (if iKWCode == 7 as libc::c_int {
                        0x5 as libc::c_int
                    } else if iKWCode == 6 as libc::c_int {
                        0x4 as libc::c_int
                    } else if iKWCode == 5 as libc::c_int {
                        0x3 as libc::c_int
                    } else if iKWCode == 4 as libc::c_int {
                        0x2 as libc::c_int
                    } else if iKWCode == 8 as libc::c_int {
                        0x6 as libc::c_int
                    } else if iKWCode == 9 as libc::c_int {
                        0x7 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as WORD;
                }
            }
            6 => {
                if bJacobianDefined != 0 {
                    ReportError(
                        pibIn,
                        (0x100 as libc::c_int + 14 as libc::c_int
                            | 0x8000 as libc::c_int) as WORD,
                        b"Jacobian\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                bJacobianDefined = 1 as libc::c_int;
                if GetPunct(pibIn, szPunct.as_mut_ptr(), '{' as i32 as libc::c_char) == 0
                {
                    szPunct[1 as libc::c_int as usize] = '{' as i32 as libc::c_char;
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        szPunct.as_mut_ptr(),
                        b"* Section must be delimited by curly braces.\0" as *const u8
                            as *const libc::c_char as PSTR,
                    );
                } else {
                    (*pinfo)
                        .wContext = (if iKWCode == 7 as libc::c_int {
                        0x5 as libc::c_int
                    } else if iKWCode == 6 as libc::c_int {
                        0x4 as libc::c_int
                    } else if iKWCode == 5 as libc::c_int {
                        0x3 as libc::c_int
                    } else if iKWCode == 4 as libc::c_int {
                        0x2 as libc::c_int
                    } else if iKWCode == 8 as libc::c_int {
                        0x6 as libc::c_int
                    } else if iKWCode == 9 as libc::c_int {
                        0x7 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as WORD;
                }
            }
            5 => {
                if bInitializeDefined != 0 {
                    ReportError(
                        pibIn,
                        (0x100 as libc::c_int + 14 as libc::c_int
                            | 0x8000 as libc::c_int) as WORD,
                        b"Initialize\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                bInitializeDefined = 1 as libc::c_int;
                if GetPunct(pibIn, szPunct.as_mut_ptr(), '{' as i32 as libc::c_char) == 0
                {
                    szPunct[1 as libc::c_int as usize] = '{' as i32 as libc::c_char;
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        szPunct.as_mut_ptr(),
                        b"* Section must be delimited by curly braces.\0" as *const u8
                            as *const libc::c_char as PSTR,
                    );
                } else {
                    (*pinfo)
                        .wContext = (if iKWCode == 7 as libc::c_int {
                        0x5 as libc::c_int
                    } else if iKWCode == 6 as libc::c_int {
                        0x4 as libc::c_int
                    } else if iKWCode == 5 as libc::c_int {
                        0x3 as libc::c_int
                    } else if iKWCode == 4 as libc::c_int {
                        0x2 as libc::c_int
                    } else if iKWCode == 8 as libc::c_int {
                        0x6 as libc::c_int
                    } else if iKWCode == 9 as libc::c_int {
                        0x7 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as WORD;
                }
            }
            8 | 9 => {
                if GetPunct(pibIn, szPunct.as_mut_ptr(), '{' as i32 as libc::c_char) == 0
                {
                    szPunct[1 as libc::c_int as usize] = '{' as i32 as libc::c_char;
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        szPunct.as_mut_ptr(),
                        b"* Section must be delimited by curly braces.\0" as *const u8
                            as *const libc::c_char as PSTR,
                    );
                } else {
                    (*pinfo)
                        .wContext = (if iKWCode == 7 as libc::c_int {
                        0x5 as libc::c_int
                    } else if iKWCode == 6 as libc::c_int {
                        0x4 as libc::c_int
                    } else if iKWCode == 5 as libc::c_int {
                        0x3 as libc::c_int
                    } else if iKWCode == 4 as libc::c_int {
                        0x2 as libc::c_int
                    } else if iKWCode == 8 as libc::c_int {
                        0x6 as libc::c_int
                    } else if iKWCode == 9 as libc::c_int {
                        0x7 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as WORD;
                }
            }
            4 => {
                if bDynamicsDefined != 0 {
                    ReportError(
                        pibIn,
                        (0x100 as libc::c_int + 14 as libc::c_int
                            | 0x8000 as libc::c_int) as WORD,
                        b"Dynamics\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
                bDynamicsDefined = 1 as libc::c_int;
                if GetPunct(pibIn, szPunct.as_mut_ptr(), '{' as i32 as libc::c_char) == 0
                {
                    szPunct[1 as libc::c_int as usize] = '{' as i32 as libc::c_char;
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        szPunct.as_mut_ptr(),
                        b"* Section must be delimited by curly braces.\0" as *const u8
                            as *const libc::c_char as PSTR,
                    );
                } else {
                    (*pinfo)
                        .wContext = (if iKWCode == 7 as libc::c_int {
                        0x5 as libc::c_int
                    } else if iKWCode == 6 as libc::c_int {
                        0x4 as libc::c_int
                    } else if iKWCode == 5 as libc::c_int {
                        0x3 as libc::c_int
                    } else if iKWCode == 4 as libc::c_int {
                        0x2 as libc::c_int
                    } else if iKWCode == 8 as libc::c_int {
                        0x6 as libc::c_int
                    } else if iKWCode == 9 as libc::c_int {
                        0x7 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as WORD;
                }
            }
            20 => {
                ProcessDTStatement(pibIn, szLex, szEqn, iKWCode);
            }
            30 => {
                ProcessInlineStatement(pibIn, szLex, szEqn, iKWCode);
            }
            40 => {
                if GetPunct(pibIn, szPunct.as_mut_ptr(), '=' as i32 as libc::c_char) != 0
                {
                    if GetPunct(pibIn, szPunct.as_mut_ptr(), '{' as i32 as libc::c_char)
                        != 0
                    {
                        ReadSBMLModels(pibIn);
                    } else {
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"{\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                } else {
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"=\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
            }
            41 => {
                if GetPunct(pibIn, szPunct.as_mut_ptr(), '=' as i32 as libc::c_char) != 0
                {
                    if GetPunct(pibIn, szPunct.as_mut_ptr(), '{' as i32 as libc::c_char)
                        != 0
                    {
                        printf(
                            b"\nreading pharmacokinetic template \0" as *const u8
                                as *const libc::c_char,
                        );
                        ReadPKTemplate(pibIn);
                    } else {
                        ReportError(
                            pibIn,
                            (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                            b"{\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                        );
                    }
                } else {
                    ReportError(
                        pibIn,
                        (0x14 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        b"=\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as PSTR,
                    );
                }
            }
            _ => {
                ProcessIdentifier(pibIn, szLex, szEqn, iKWCode);
            }
        }
        if iErr != 0 {
            EatStatement(pibIn);
        }
    };
}
pub unsafe extern "C" fn FindEnd(mut pBuf: PBUF, mut N: libc::c_long) -> libc::c_int {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    c = pBuf;
    end = pBuf.offset(N as isize);
    while c < end {
        if *c as libc::c_int == '\n' as i32 {
            c = c.offset(1);
            c;
            while c < end
                && *(*__ctype_b_loc()).offset(*c as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                c = c.offset(1);
                c;
            }
            if c < end {
                if c.offset(2 as libc::c_int as isize) < end
                    && *c as libc::c_int == 'E' as i32
                    && *c.offset(1 as libc::c_int as isize) as libc::c_int == 'n' as i32
                    && *c.offset(2 as libc::c_int as isize) as libc::c_int == 'd' as i32
                {
                    return 1 as libc::c_int;
                }
            }
        }
        c = c.offset(1);
        c;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ReadModel(
    mut pinfo: PINPUTINFO,
    mut ptempinfo: PINPUTINFO,
    mut szFileIn: PSTR,
) {
    let mut ibIn: INPUTBUF = INPUTBUF {
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
    let mut szEqn: PSTREQN = [0; 5119];
    let mut iLexType: libc::c_int = 0;
    if InitBuffer(&mut ibIn, -(1 as libc::c_int) as libc::c_long, szFileIn) == 0 {
        ReportError(
            &mut ibIn,
            (0x1 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ReadModel\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    ibIn.pInfo = pinfo as PVOID;
    ibIn.pTempInfo = ptempinfo as PVOID;
    if FindEnd(ibIn.pbufOrg, ibIn.lBufSize) == 0 as libc::c_int {
        ReportError(
            0 as PINPUTBUF,
            (0x100 as libc::c_int + 15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szFileIn,
            0 as PSTR,
        );
    }
    loop {
        NextLex(&mut ibIn, szLex.as_mut_ptr(), &mut iLexType);
        let mut current_block_12: u64;
        match iLexType {
            0 => {
                (*pinfo).wContext = 0x4000 as libc::c_int as WORD;
                current_block_12 = 5783071609795492627;
            }
            1 => {
                ProcessWord(&mut ibIn, szLex.as_mut_ptr(), szEqn.as_mut_ptr());
                current_block_12 = 5783071609795492627;
            }
            8 | 32 => {
                if szLex[0 as libc::c_int as usize] as libc::c_int == ';' as i32 {
                    current_block_12 = 5783071609795492627;
                } else if szLex[0 as libc::c_int as usize] as libc::c_int == '}' as i32
                    && (*pinfo).wContext
                        & (0x2 as libc::c_int | 0x4 as libc::c_int | 0x3 as libc::c_int)
                            as libc::c_uint != 0
                {
                    (*pinfo).wContext = 0x1 as libc::c_int as WORD;
                    current_block_12 = 5783071609795492627;
                } else if szLex[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
                    SkipComment(&mut ibIn);
                    current_block_12 = 5783071609795492627;
                } else {
                    current_block_12 = 16040908003852494439;
                }
            }
            2 | 4 => {
                ReportError(
                    &mut ibIn,
                    0x13 as libc::c_int as WORD,
                    szLex.as_mut_ptr(),
                    b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
                );
                current_block_12 = 5783071609795492627;
            }
            _ => {
                current_block_12 = 16040908003852494439;
            }
        }
        match current_block_12 {
            16040908003852494439 => {
                ReportError(
                    &mut ibIn,
                    0x11 as libc::c_int as WORD,
                    szLex.as_mut_ptr(),
                    b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
                );
            }
            _ => {}
        }
        if !((*pinfo).wContext != 0x4000 as libc::c_int as libc::c_uint) {
            break;
        }
    }
    (*pinfo).wContext = 0x4000 as libc::c_int as WORD;
    if !(ibIn.pbufOrg).is_null() {
        free(ibIn.pbufOrg as *mut libc::c_void);
    }
}
