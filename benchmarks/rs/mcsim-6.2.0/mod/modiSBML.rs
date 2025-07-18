use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn rewind(__stream: *mut FILE);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn FillBuffer(pibIn: PINPUTBUF, lBuffer_size: libc::c_long) -> libc::c_int;
    fn GetaString(pibIn: PINPUTBUF, szLex: PSTR);
    fn GetIdentifier(pibIn: PINPUTBUF, szLex: PSTR);
    fn GetNumber(pibIn: PINPUTBUF, szLex: PSTR, piLexType: PINT);
    fn InitBuffer(
        pibIn: PINPUTBUF,
        lBuffer_size: libc::c_long,
        szFullPathname: PSTR,
    ) -> BOOL;
    fn MakeStringBuffer(pBuf: PINPUTBUF, pStrBuf: PINPUTBUF, sz: PSTR);
    fn NextLex(_: PINPUTBUF, _: *mut libc::c_char, _: PINT);
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn SkipComment(_: PINPUTBUF);
    fn SkipWhitespace(pibIn: PINPUTBUF) -> libc::c_int;
    fn ProcessWord(pibIn: PINPUTBUF, szLex: PSTR, szEqn: PSTR);
    fn AddEquation(ppvm: *mut PVMMAPSTRCT, szName: PSTR, szEqn: PSTR, hType: HANDLE);
    fn DeclareModelVar(pibIn: PINPUTBUF, szName: PSTR, iKWCode: libc::c_int);
    fn DefineGlobalVar(
        pibIn: PINPUTBUF,
        pvm: PVMMAPSTRCT,
        szName: PSTR,
        szEqn: PSTR,
        hType: HANDLE,
    );
    fn DefineVariable(pibIn: PINPUTBUF, szName: PSTR, szEqn: PSTR, iKWCode: libc::c_int);
    fn GetVarPTR(pvm: PVMMAPSTRCT, szName: PSTR) -> PVMMAPSTRCT;
    fn GetVarType(pvm: PVMMAPSTRCT, szName: PSTR) -> libc::c_int;
    fn IsMathFunc(sz: PSTR) -> BOOL;
    fn ForAllVar(
        pfile: PFILE,
        pvm: PVMMAPSTRCT,
        pfiFunc: PFI_CALLBACK,
        hType: HANDLE,
        pinfo: PVOID,
    ) -> libc::c_int;
    fn ReversePointers(ppvm: *mut PVMMAPSTRCT);
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
pub type VARTYPES = libc::c_uint;
pub const parameter: VARTYPES = 2;
pub const reactant: VARTYPES = 1;
pub const product: VARTYPES = 0;
pub type OPSIGNS = libc::c_uint;
pub const minus: OPSIGNS = 1;
pub const plus: OPSIGNS = 0;
pub type FORSV = tagFORSV;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagFORSV {
    pub pibIn: PINPUTBUF,
    pub szName: PSTR,
    pub szVal: PSTR,
    pub pTarget: PVMMAPSTRCT,
}
pub type PFORSV = *mut tagFORSV;
pub type PFI_CALLBACK = Option::<
    unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
>;
pub static mut vrgSBMLKW: [KM; 30] = [
    {
        let mut init = tagKM {
            szKeyword: b"sbml\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 10 as libc::c_int,
            fContext: 1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"model\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 1 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"listOfFunctionDefinitions\0" as *const u8 as *const libc::c_char
                as PSTR,
            iKWCode: 17 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"functionDefinition\0" as *const u8 as *const libc::c_char
                as PSTR,
            iKWCode: 18 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"listOfCompartments\0" as *const u8 as *const libc::c_char
                as PSTR,
            iKWCode: 2 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"compartment\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 3 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"listOfSpecies\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 4 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"specie\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 6 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"species\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 6 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"listOfParameters\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 7 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"parameter\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 8 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"listOfRules\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 15 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"rateRule\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 16 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"listOfReactions\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 9 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"reaction\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 20 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"listOfReactants\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 30 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"listOfProducts\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 40 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"specieReference\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 50 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"speciesReference\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 50 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"stoichiometry\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 51 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"kineticLaw\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 60 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"math\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 61 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"apply\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 62 as libc::c_int,
            fContext: 0x1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"plus\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 64 as libc::c_int,
            fContext: 1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"minus\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 65 as libc::c_int,
            fContext: 1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"times\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 66 as libc::c_int,
            fContext: 1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"divide\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 67 as libc::c_int,
            fContext: 1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"power\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 68 as libc::c_int,
            fContext: 1 as libc::c_int as WORD,
        };
        init
    },
    {
        let mut init = tagKM {
            szKeyword: b"ci\0" as *const u8 as *const libc::c_char as PSTR,
            iKWCode: 63 as libc::c_int,
            fContext: 1 as libc::c_int as WORD,
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
pub unsafe extern "C" fn AugmentEquation(
    mut pvm: PVMMAPSTRCT,
    mut szEqn: PSTR,
    mut szStoi: PSTR,
    mut sign: OPSIGNS,
) {
    let mut szBuf: PSTR = 0 as *mut libc::c_char;
    let mut szSymbol: PSTRLEX = [0; 1023];
    if pvm.is_null() || szEqn.is_null() || szStoi.is_null() {
        return;
    }
    if sign as libc::c_uint == plus as libc::c_int as libc::c_uint {
        sprintf(szSymbol.as_mut_ptr(), b" + \0" as *const u8 as *const libc::c_char);
    }
    if sign as libc::c_uint == minus as libc::c_int as libc::c_uint {
        sprintf(szSymbol.as_mut_ptr(), b" - \0" as *const u8 as *const libc::c_char);
    }
    szBuf = malloc(
        (strlen((*pvm).szEqn as *const libc::c_char))
            .wrapping_add(strlen(szSymbol.as_mut_ptr()))
            .wrapping_add(strlen(szStoi as *const libc::c_char))
            .wrapping_add(strlen(szEqn as *const libc::c_char))
            .wrapping_add(5 as libc::c_int as libc::c_ulong),
    ) as PSTR;
    if !szBuf.is_null() {
        if strcmp(
            szStoi as *const libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            sprintf(
                szBuf,
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                (*pvm).szEqn,
                szSymbol.as_mut_ptr(),
                szEqn,
            );
        } else {
            sprintf(
                szBuf,
                b"%s%s %s * %s\0" as *const u8 as *const libc::c_char,
                (*pvm).szEqn,
                szSymbol.as_mut_ptr(),
                szStoi,
                szEqn,
            );
        }
    } else {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szEqn,
            b"* .. defining equation in AugmentEquation\0" as *const u8
                as *const libc::c_char as PSTR,
        );
    }
    if !((*pvm).szEqn).is_null() {
        free((*pvm).szEqn as *mut libc::c_void);
    }
    (*pvm).szEqn = szBuf;
}
pub unsafe extern "C" fn ConstructEqn(
    mut pibIn: PINPUTBUF,
    mut szRName: PSTR,
    mut eType: VARTYPES,
) {
    let mut iLexType: libc::c_int = 0;
    let mut hType: HANDLE = 0;
    let mut szSName: PSTRLEX = [0; 1023];
    let mut szStoichio: PSTRLEX = [0; 1023];
    let mut pvm: PVMMAPSTRCT = 0 as *mut tagVM;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    loop {
        let fresh0 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh0 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetIdentifier(pibIn, szSName.as_mut_ptr());
    *(*pibIn)
        .pbufCur = (*(*pibIn).pbufCur as libc::c_int + 1 as libc::c_int) as libc::c_char;
    while *(*pibIn).pbufCur as libc::c_int != '"' as i32
        && *(*pibIn).pbufCur as libc::c_int != '>' as i32
    {
        let fresh1 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        *fresh1;
    }
    if *(*pibIn).pbufCur as libc::c_int == '"' as i32 {
        let fresh2 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        *fresh2;
        GetNumber(pibIn, szStoichio.as_mut_ptr(), &mut iLexType);
    } else {
        sprintf(szStoichio.as_mut_ptr(), b"1\0" as *const u8 as *const libc::c_char);
    }
    printf(
        b"%s stoichio: %s\n\0" as *const u8 as *const libc::c_char,
        szSName.as_mut_ptr(),
        szStoichio.as_mut_ptr(),
    );
    if (GetVarPTR((*pinfo).pvmGloVars, szSName.as_mut_ptr())).is_null() {
        sprintf(
            szSName.as_mut_ptr(),
            b"%s_%s\0" as *const u8 as *const libc::c_char,
            szSName.as_mut_ptr(),
            (*(*pinfo).pvmLocalCpts).szName,
        );
    }
    hType = GetVarType((*pinfo).pvmGloVars, szSName.as_mut_ptr()) as HANDLE;
    if hType == 0x10000 as libc::c_int as libc::c_ulong {
        pvm = GetVarPTR((*pinfo).pvmDynEqns, szSName.as_mut_ptr());
        if pvm.is_null() {
            DefineVariable(
                pibIn,
                szSName.as_mut_ptr(),
                b"\0" as *const u8 as *const libc::c_char as PSTR,
                20 as libc::c_int,
            );
            pvm = GetVarPTR((*pinfo).pvmDynEqns, szSName.as_mut_ptr());
        }
        AugmentEquation(
            pvm,
            szRName,
            szStoichio.as_mut_ptr(),
            (if eType as libc::c_uint == reactant as libc::c_int as libc::c_uint {
                minus as libc::c_int
            } else {
                plus as libc::c_int
            }) as OPSIGNS,
        );
    } else if hType != 0x40000 as libc::c_int as libc::c_ulong {
        ReportError(
            0 as PINPUTBUF,
            (0x100 as libc::c_int + 5 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szSName.as_mut_ptr(),
            0 as PSTR,
        );
    }
}
pub unsafe extern "C" fn CountLines(mut pFileIn: PFILE) -> libc::c_long {
    let mut nLines: libc::c_int = 0 as libc::c_int;
    let mut szDummy: [libc::c_char; 2] = [0; 2];
    fscanf(pFileIn, b"%*[^\n]\0" as *const u8 as *const libc::c_char);
    getc(pFileIn);
    while feof(pFileIn) == 0 {
        if fscanf(
            pFileIn,
            b"%1s\0" as *const u8 as *const libc::c_char,
            szDummy.as_mut_ptr(),
        ) > 0 as libc::c_int
        {
            nLines += 1;
            nLines;
        }
        fscanf(pFileIn, b"%*[^\n]\0" as *const u8 as *const libc::c_char);
        getc(pFileIn);
    }
    rewind(pFileIn);
    return nLines as libc::c_long;
}
pub unsafe extern "C" fn GetSBMLKeywordCode(mut szKeyword: PSTR) -> libc::c_int {
    let mut pkm: PKM = &mut *vrgSBMLKW.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut KM;
    while *(*pkm).szKeyword as libc::c_int != 0
        && strcmp(
            szKeyword as *const libc::c_char,
            (*pkm).szKeyword as *const libc::c_char,
        ) != 0
    {
        pkm = pkm.offset(1);
        pkm;
    }
    return (*pkm).iKWCode;
}
pub unsafe extern "C" fn GetSBMLLex(
    mut pibIn: PINPUTBUF,
    mut iEnding: libc::c_int,
    mut iKWCode: libc::c_int,
) -> libc::c_int {
    let mut szLex: PSTRLEX = [0; 1023];
    let mut c: libc::c_char = 0;
    let mut bDone: BOOL = 0 as libc::c_int;
    let mut bFound: BOOL = 0 as libc::c_int;
    if pibIn.is_null() {
        return 0 as libc::c_int;
    }
    if ((*pibIn).pbufCur).is_null() {
        return 0 as libc::c_int;
    }
    while bDone == 0 {
        c = *(*pibIn).pbufCur;
        if c as libc::c_int == '<' as i32 {
            let fresh3 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            szLex[0 as libc::c_int as usize] = *fresh3;
            c = *(*pibIn).pbufCur;
            if c as libc::c_int == '!' as i32 || c as libc::c_int == '?' as i32
                || c as libc::c_int == '/' as i32
            {
                if c as libc::c_int == '/' as i32 {
                    let fresh4 = (*pibIn).pbufCur;
                    (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                    szLex[0 as libc::c_int as usize] = *fresh4;
                    GetIdentifier(pibIn, szLex.as_mut_ptr());
                    bDone = (GetSBMLKeywordCode(szLex.as_mut_ptr()) == iEnding)
                        as libc::c_int;
                } else {
                    loop {
                        let fresh5 = (*pibIn).pbufCur;
                        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
                        if !(*fresh5 as libc::c_int != '>' as i32) {
                            break;
                        }
                    }
                }
            } else {
                GetIdentifier(pibIn, szLex.as_mut_ptr());
                bFound = (iKWCode == GetSBMLKeywordCode(szLex.as_mut_ptr()))
                    as libc::c_int;
                bDone = bFound;
            }
        } else if c as libc::c_int == 0 as libc::c_int {
            bDone = 1 as libc::c_int;
        } else {
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1 as libc::c_int as isize);
        }
    }
    return bFound;
}
pub unsafe extern "C" fn SetVar(
    mut pibIn: PINPUTBUF,
    mut szName: PSTR,
    mut szVal: PSTR,
    mut hType: HANDLE,
) {
    let mut pvm: PVMMAPSTRCT = 0 as *mut tagVM;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    let mut iKWCode: libc::c_int = 0;
    if (GetVarPTR((*pinfo).pvmGloVars, szName)).is_null() {
        iKWCode = if hType == 0x10000 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else if hType == 0x20000 as libc::c_int as libc::c_ulong {
            2 as libc::c_int
        } else if hType == 0x30000 as libc::c_int as libc::c_ulong {
            3 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if hType == 0x40000 as libc::c_int as libc::c_ulong
            || hType == (0x50000 as libc::c_int | 0xf000 as libc::c_int) as libc::c_ulong
            || hType == (0x80000 as libc::c_int | 0xf000 as libc::c_int) as libc::c_ulong
            || hType == (0x60000 as libc::c_int | 0xf000 as libc::c_int) as libc::c_ulong
        {
            AddEquation(&mut (*pinfo).pvmGloVars, szName, szVal, hType);
            if hType == 0x40000 as libc::c_int as libc::c_ulong {
                printf(
                    b"param.   %s = %s\n\0" as *const u8 as *const libc::c_char,
                    szName,
                    szVal,
                );
            }
        } else {
            DeclareModelVar(pibIn, szName, iKWCode);
            pvm = GetVarPTR((*pinfo).pvmGloVars, szName);
            DefineGlobalVar(pibIn, pvm, szName, szVal, hType);
            if hType == 0x10000 as libc::c_int as libc::c_ulong {
                printf(
                    b"species  %s = %s\n\0" as *const u8 as *const libc::c_char,
                    szName,
                    szVal,
                );
            }
            if hType == 0x20000 as libc::c_int as libc::c_ulong {
                printf(
                    b"input    %s = %s\n\0" as *const u8 as *const libc::c_char,
                    szName,
                    szVal,
                );
            }
            if hType == 0x30000 as libc::c_int as libc::c_ulong {
                printf(
                    b"output   %s = %s\n\0" as *const u8 as *const libc::c_char,
                    szName,
                    szVal,
                );
            }
        }
    }
}
pub unsafe extern "C" fn Create1Var(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut pV: PFORSV = pInfo as PFORSV;
    let mut szTmp: PSTRLEX = [0; 1023];
    if *((*pvm).szName).offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32 {
        sprintf(
            szTmp.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (*pV).szName,
            (*pvm).szName,
        );
        if (*pvm).hType
            == (0x50000 as libc::c_int | 0xf000 as libc::c_int) as libc::c_ulong
        {
            SetVar((*pV).pibIn, szTmp.as_mut_ptr(), (*pvm).szEqn, (*pvm).hType);
        } else {
            SetVar((*pV).pibIn, szTmp.as_mut_ptr(), (*pV).szVal, (*pvm).hType);
        }
    } else {
        SetVar((*pV).pibIn, (*pvm).szName, (*pvm).szEqn, (*pvm).hType);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Transcribe1AlgEqn(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut pV: PFORSV = pInfo as PFORSV;
    let mut szTmpName: PSTRLEX = *::std::mem::transmute::<
        &[u8; 1023],
        &mut PSTRLEX,
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut szTmpEq: PSTREQN = *::std::mem::transmute::<
        &[u8; 5119],
        &mut PSTREQN,
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
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
    if *((*pvm).szName).offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32 {
        sprintf(
            szTmpName.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (*pV).szName,
            (*pvm).szName,
        );
    } else {
        sprintf(
            szTmpName.as_mut_ptr(),
            b"%s\0" as *const u8 as *const libc::c_char,
            (*pvm).szName,
        );
    }
    MakeStringBuffer(0 as PINPUTBUF, &mut ibDummy, (*pvm).szEqn);
    while !((&mut ibDummy as *mut INPUTBUF).is_null()
        || ((ibDummy.pbufCur).is_null() || *ibDummy.pbufCur == 0)
            && ((ibDummy.pfileIn).is_null() || feof(ibDummy.pfileIn) != 0))
    {
        NextLex(&mut ibDummy, szLex.as_mut_ptr(), &mut iType);
        if iType == 0x1 as libc::c_int && IsMathFunc(szLex.as_mut_ptr()) == 0
            && szLex[0 as libc::c_int as usize] as libc::c_int == '_' as i32
        {
            sprintf(
                szTmpEq.as_mut_ptr(),
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                szTmpEq.as_mut_ptr(),
                (*pV).szName,
                szLex.as_mut_ptr(),
            );
        } else {
            sprintf(
                szTmpEq.as_mut_ptr(),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                szTmpEq.as_mut_ptr(),
                szLex.as_mut_ptr(),
            );
        }
    }
    if (GetVarPTR((*pV).pTarget, szTmpName.as_mut_ptr())).is_null() {
        if (*pvm).hType < 0x90000 as libc::c_int as libc::c_ulong {
            DefineVariable(
                (*pV).pibIn,
                szTmpName.as_mut_ptr(),
                szTmpEq.as_mut_ptr(),
                0 as libc::c_int,
            );
            printf(
                b"local v. %s = %s\n\0" as *const u8 as *const libc::c_char,
                szTmpName.as_mut_ptr(),
                szTmpEq.as_mut_ptr(),
            );
        } else if (*pvm).hType == 0xa0000 as libc::c_int as libc::c_ulong {
            DefineVariable(
                (*pV).pibIn,
                szTmpName.as_mut_ptr(),
                szTmpEq.as_mut_ptr(),
                30 as libc::c_int,
            );
            printf(
                b"inline   %s\n\0" as *const u8 as *const libc::c_char,
                szTmpEq.as_mut_ptr(),
            );
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Transcribe1DiffEqn(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut pV: PFORSV = pInfo as PFORSV;
    let mut szTmpName: PSTRLEX = *::std::mem::transmute::<
        &[u8; 1023],
        &mut PSTRLEX,
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut szTmpEq: PSTREQN = *::std::mem::transmute::<
        &[u8; 5119],
        &mut PSTREQN,
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
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
    if (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        != 0x90000 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if *((*pvm).szName).offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32 {
        sprintf(
            szTmpName.as_mut_ptr(),
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (*pV).szName,
            (*pvm).szName,
        );
    } else {
        sprintf(
            szTmpName.as_mut_ptr(),
            b"%s\0" as *const u8 as *const libc::c_char,
            (*pvm).szName,
        );
    }
    MakeStringBuffer(0 as PINPUTBUF, &mut ibDummy, (*pvm).szEqn);
    while !((&mut ibDummy as *mut INPUTBUF).is_null()
        || ((ibDummy.pbufCur).is_null() || *ibDummy.pbufCur == 0)
            && ((ibDummy.pfileIn).is_null() || feof(ibDummy.pfileIn) != 0))
    {
        NextLex(&mut ibDummy, szLex.as_mut_ptr(), &mut iType);
        if iType == 0x1 as libc::c_int && IsMathFunc(szLex.as_mut_ptr()) == 0
            && szLex[0 as libc::c_int as usize] as libc::c_int == '_' as i32
        {
            sprintf(
                szTmpEq.as_mut_ptr(),
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                szTmpEq.as_mut_ptr(),
                (*pV).szName,
                szLex.as_mut_ptr(),
            );
        } else {
            sprintf(
                szTmpEq.as_mut_ptr(),
                b"%s%s\0" as *const u8 as *const libc::c_char,
                szTmpEq.as_mut_ptr(),
                szLex.as_mut_ptr(),
            );
        }
    }
    if (GetVarPTR((*pV).pTarget, szTmpName.as_mut_ptr())).is_null() {
        DefineVariable(
            (*pV).pibIn,
            szTmpName.as_mut_ptr(),
            szTmpEq.as_mut_ptr(),
            20 as libc::c_int,
        );
        printf(
            b"template ODE term for %s = %s\n\0" as *const u8 as *const libc::c_char,
            szTmpName.as_mut_ptr(),
            szTmpEq.as_mut_ptr(),
        );
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ReadCpt(mut pibIn: PINPUTBUF, mut bTell: BOOL) {
    let mut szName: PSTRLEX = [0; 1023];
    let mut szEqn: PSTREQN = [0; 5119];
    let mut iLexType: libc::c_int = 0;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    (*pinfo).wContext = 0x1 as libc::c_int as WORD;
    loop {
        let fresh6 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh6 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetIdentifier(pibIn, szName.as_mut_ptr());
    if strcmp(szName.as_mut_ptr(), b"compartment\0" as *const u8 as *const libc::c_char)
        == 0
    {
        return;
    }
    if (GetVarPTR((*pinfo).pvmLocalCpts, szName.as_mut_ptr())).is_null() {
        *(*pibIn)
            .pbufCur = (*(*pibIn).pbufCur as libc::c_int + 1 as libc::c_int)
            as libc::c_char;
        loop {
            let fresh7 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            if !(*fresh7 as libc::c_int != '"' as i32) {
                break;
            }
        }
        GetNumber(pibIn, szEqn.as_mut_ptr(), &mut iLexType);
        if iLexType == 0 {
            sprintf(szEqn.as_mut_ptr(), b"0.0\0" as *const u8 as *const libc::c_char);
        }
        AddEquation(
            &mut (*pinfo).pvmLocalCpts,
            szName.as_mut_ptr(),
            szEqn.as_mut_ptr(),
            0xb0000 as libc::c_int as HANDLE,
        );
        if bTell != 0 {
            printf(
                b"compart. %s = %s\n\0" as *const u8 as *const libc::c_char,
                szName.as_mut_ptr(),
                szEqn.as_mut_ptr(),
            );
        }
    }
    loop {
        let fresh8 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh8 as libc::c_int != '>' as i32) {
            break;
        }
    };
}
pub unsafe extern "C" fn ReadCpts(mut pibIn: PINPUTBUF, mut bTell: BOOL) {
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    (*pinfo).pvmLocalCpts = 0 as PVMMAPSTRCT;
    while GetSBMLLex(pibIn, 2 as libc::c_int, 3 as libc::c_int) != 0 {
        ReadCpt(pibIn, bTell);
    }
}
pub unsafe extern "C" fn ReadFunction(mut pibIn: PINPUTBUF) {
    let mut szRName: PSTRLEX = [0; 1023];
    let mut szEqn: PSTREQN = [0; 5119];
    let mut bInited: libc::c_int = 0 as libc::c_int;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    (*pinfo).wContext = 0x2 as libc::c_int as WORD;
    loop {
        let fresh9 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh9 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetIdentifier(pibIn, szRName.as_mut_ptr());
    loop {
        let fresh10 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh10 as libc::c_int != '>' as i32) {
            break;
        }
    }
    GetSBMLLex(pibIn, 10 as libc::c_int, 62 as libc::c_int);
    ReadApply(pibIn, &mut bInited, szEqn.as_mut_ptr());
    printf(
        b"rate for %s = %s\n\0" as *const u8 as *const libc::c_char,
        szRName.as_mut_ptr(),
        szEqn.as_mut_ptr(),
    );
    DefineVariable(pibIn, szRName.as_mut_ptr(), szEqn.as_mut_ptr(), 20 as libc::c_int);
    loop {
        let fresh11 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh11 as libc::c_int != '>' as i32) {
            break;
        }
    };
}
pub unsafe extern "C" fn ReadFunctions(
    mut pibIn: PINPUTBUF,
    mut iSBML_level: libc::c_int,
) {
    if iSBML_level == 1 as libc::c_int {
        printf(
            b"mod: ignoring function definitions in level 1...\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        while GetSBMLLex(pibIn, 17 as libc::c_int, 18 as libc::c_int) != 0 {
            ReadFunction(pibIn);
        }
    };
}
pub unsafe extern "C" fn ReadDifferentials(mut pibIn: PINPUTBUF) {
    let mut szRName: PSTRLEX = [0; 1023];
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    (*pinfo).wContext = 0x2 as libc::c_int as WORD;
    while GetSBMLLex(pibIn, 10 as libc::c_int, 20 as libc::c_int) != 0 {
        loop {
            let fresh12 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            if !(*fresh12 as libc::c_int != '"' as i32) {
                break;
            }
        }
        GetIdentifier(pibIn, szRName.as_mut_ptr());
        loop {
            let fresh13 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            if !(*fresh13 as libc::c_int != '>' as i32) {
                break;
            }
        }
        GetSBMLLex(pibIn, 20 as libc::c_int, 30 as libc::c_int);
        while GetSBMLLex(pibIn, 30 as libc::c_int, 50 as libc::c_int) != 0 {
            ConstructEqn(pibIn, szRName.as_mut_ptr(), reactant);
        }
        GetSBMLLex(pibIn, 20 as libc::c_int, 40 as libc::c_int);
        while GetSBMLLex(pibIn, 40 as libc::c_int, 50 as libc::c_int) != 0 {
            ConstructEqn(pibIn, szRName.as_mut_ptr(), product);
        }
    }
}
pub unsafe extern "C" fn ReadParameter(mut pibIn: PINPUTBUF) {
    let mut szName: PSTRLEX = [0; 1023];
    let mut szEqn: PSTREQN = [0; 5119];
    let mut iLexType: libc::c_int = 0;
    let mut pvm: PVMMAPSTRCT = 0 as *mut tagVM;
    let mut hType: HANDLE = 0;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    (*pinfo).wContext = 0x1 as libc::c_int as WORD;
    loop {
        let fresh14 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh14 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetIdentifier(pibIn, szName.as_mut_ptr());
    hType = GetVarType((*pinfo).pvmGloVars, szName.as_mut_ptr()) as HANDLE;
    if hType == 0 {
        *(*pibIn)
            .pbufCur = (*(*pibIn).pbufCur as libc::c_int + 1 as libc::c_int)
            as libc::c_char;
        loop {
            let fresh15 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            if !(*fresh15 as libc::c_int != '"' as i32) {
                break;
            }
        }
        GetNumber(pibIn, szEqn.as_mut_ptr(), &mut iLexType);
        if iLexType == 0 {
            sprintf(szEqn.as_mut_ptr(), b"0.0\0" as *const u8 as *const libc::c_char);
        }
        DefineGlobalVar(pibIn, pvm, szName.as_mut_ptr(), szEqn.as_mut_ptr(), hType);
        printf(
            b"param.   %s = %s\n\0" as *const u8 as *const libc::c_char,
            szName.as_mut_ptr(),
            szEqn.as_mut_ptr(),
        );
    } else {
        printf(
            b"***Error: redeclaration of parameter %s\n\0" as *const u8
                as *const libc::c_char,
            szName.as_mut_ptr(),
        );
        printf(b"Exiting...\n\n\0" as *const u8 as *const libc::c_char);
        exit(0 as libc::c_int);
    }
    loop {
        let fresh16 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh16 as libc::c_int != '>' as i32) {
            break;
        }
    };
}
pub unsafe extern "C" fn ReadParameters(mut pibIn: PINPUTBUF) {
    while GetSBMLLex(pibIn, 7 as libc::c_int, 8 as libc::c_int) != 0 {
        ReadParameter(pibIn);
    }
}
pub unsafe extern "C" fn ReadReaction_L1(mut pibIn: PINPUTBUF) {
    let mut szRName: PSTRLEX = [0; 1023];
    let mut szEqn: PSTREQN = [0; 5119];
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    (*pinfo).wContext = 0x2 as libc::c_int as WORD;
    loop {
        let fresh17 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh17 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetIdentifier(pibIn, szRName.as_mut_ptr());
    loop {
        let fresh18 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh18 as libc::c_int != '>' as i32) {
            break;
        }
    }
    GetSBMLLex(pibIn, 10 as libc::c_int, 60 as libc::c_int);
    loop {
        let fresh19 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh19 as libc::c_int != '"' as i32) {
            break;
        }
    }
    (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(-(1 as libc::c_int as isize));
    GetaString(pibIn, szEqn.as_mut_ptr());
    printf(
        b"reaction %s = %s\n\0" as *const u8 as *const libc::c_char,
        szRName.as_mut_ptr(),
        szEqn.as_mut_ptr(),
    );
    DefineVariable(pibIn, szRName.as_mut_ptr(), szEqn.as_mut_ptr(), 0 as libc::c_int);
    loop {
        let fresh20 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh20 as libc::c_int != '>' as i32) {
            break;
        }
    };
}
pub unsafe extern "C" fn TranscribeOpSymbol(mut szOp: PSTR) {
    match GetSBMLKeywordCode(szOp) {
        64 => {
            sprintf(
                szOp,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"+\0" as *const u8 as *const libc::c_char,
            );
        }
        65 => {
            sprintf(
                szOp,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"-\0" as *const u8 as *const libc::c_char,
            );
        }
        66 => {
            sprintf(
                szOp,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"*\0" as *const u8 as *const libc::c_char,
            );
        }
        67 => {
            sprintf(
                szOp,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"/\0" as *const u8 as *const libc::c_char,
            );
        }
        68 => {
            sprintf(
                szOp,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"pow\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            printf(
                b"***Error: unknown mathXML operation '%s' - exiting...\n\n\0"
                    as *const u8 as *const libc::c_char,
                szOp,
            );
            exit(0 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn ReadApply(
    mut pibIn: PINPUTBUF,
    mut bInited: PINT,
    mut szEqn: PSTR,
) {
    let mut szOp: PSTRLEX = [0; 1023];
    let mut szLex: PSTRLEX = [0; 1023];
    let mut iKw: libc::c_int = 0;
    let mut ithTerm: libc::c_int = 0 as libc::c_int;
    let mut bDone: BOOL = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    if *bInited != 0 {
        sprintf(szEqn, b"%s(\0" as *const u8 as *const libc::c_char, szEqn);
    } else {
        sprintf(szEqn, b"(\0" as *const u8 as *const libc::c_char);
        *bInited = 1 as libc::c_int;
    }
    loop {
        let fresh21 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh21 as libc::c_int != '<' as i32) {
            break;
        }
    }
    c = *(*pibIn).pbufCur;
    if c as libc::c_int == '!' as i32 {
        loop {
            let fresh22 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            if !(*fresh22 as libc::c_int != '<' as i32) {
                break;
            }
        }
    }
    GetIdentifier(pibIn, szOp.as_mut_ptr());
    TranscribeOpSymbol(szOp.as_mut_ptr());
    loop {
        loop {
            let fresh23 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            if !(*fresh23 as libc::c_int != '<' as i32) {
                break;
            }
        }
        c = *(*pibIn).pbufCur;
        if c as libc::c_int == '/' as i32 {
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            (*pibIn).pbufCur;
            GetIdentifier(pibIn, szLex.as_mut_ptr());
            iKw = GetSBMLKeywordCode(szLex.as_mut_ptr());
            if iKw == 62 as libc::c_int || iKw == 61 as libc::c_int {
                sprintf(szEqn, b"%s)\0" as *const u8 as *const libc::c_char, szEqn);
                return;
            }
        } else {
            GetIdentifier(pibIn, szLex.as_mut_ptr());
        }
        if strcmp(szLex.as_mut_ptr(), b"apply\0" as *const u8 as *const libc::c_char)
            == 0
        {
            ithTerm += 1;
            ithTerm;
            if strcmp(szOp.as_mut_ptr(), b"pow\0" as *const u8 as *const libc::c_char)
                == 0
            {
                if ithTerm > 1 as libc::c_int {
                    sprintf(szEqn, b"%s)\0" as *const u8 as *const libc::c_char, szEqn);
                } else {
                    sprintf(
                        szEqn,
                        b"%s%s(,\0" as *const u8 as *const libc::c_char,
                        szEqn,
                        szOp.as_mut_ptr(),
                    );
                }
            } else if ithTerm > 1 as libc::c_int {
                sprintf(
                    szEqn,
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    szEqn,
                    szOp.as_mut_ptr(),
                );
            }
            ReadApply(pibIn, bInited, szEqn);
        } else {
            loop {
                (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1 as libc::c_int as isize);
                SkipWhitespace(pibIn);
                GetIdentifier(pibIn, szLex.as_mut_ptr());
                if (*pinfo).bTemplateInUse != 0
                    && (GetVarPTR((*pinfo).pvmGloVars, szLex.as_mut_ptr())).is_null()
                {
                    sprintf(
                        szLex.as_mut_ptr(),
                        b"%s_%s\0" as *const u8 as *const libc::c_char,
                        szLex.as_mut_ptr(),
                        (*(*pinfo).pvmLocalCpts).szName,
                    );
                }
                ithTerm += 1;
                ithTerm;
                if strcmp(
                    szOp.as_mut_ptr(),
                    b"pow\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if ithTerm > 1 as libc::c_int {
                        sprintf(
                            szEqn,
                            b"%s%s)\0" as *const u8 as *const libc::c_char,
                            szEqn,
                            szLex.as_mut_ptr(),
                        );
                    } else {
                        sprintf(
                            szEqn,
                            b"%s%s(%s,\0" as *const u8 as *const libc::c_char,
                            szEqn,
                            szOp.as_mut_ptr(),
                            szLex.as_mut_ptr(),
                        );
                    }
                } else if ithTerm > 1 as libc::c_int {
                    sprintf(
                        szEqn,
                        b"%s%s%s\0" as *const u8 as *const libc::c_char,
                        szEqn,
                        szOp.as_mut_ptr(),
                        szLex.as_mut_ptr(),
                    );
                } else {
                    sprintf(
                        szEqn,
                        b"%s%s\0" as *const u8 as *const libc::c_char,
                        szEqn,
                        szLex.as_mut_ptr(),
                    );
                }
                if !(GetSBMLLex(pibIn, 62 as libc::c_int, 63 as libc::c_int) != 0) {
                    break;
                }
            }
            sprintf(szEqn, b"%s)\0" as *const u8 as *const libc::c_char, szEqn);
            return;
        }
        if !(bDone == 0) {
            break;
        }
    };
}
pub unsafe extern "C" fn ReadReaction_L2(mut pibIn: PINPUTBUF) {
    let mut szRName: PSTRLEX = [0; 1023];
    let mut szEqn: PSTREQN = [0; 5119];
    let mut bInited: libc::c_int = 0 as libc::c_int;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    (*pinfo).wContext = 0x2 as libc::c_int as WORD;
    loop {
        let fresh24 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh24 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetIdentifier(pibIn, szRName.as_mut_ptr());
    loop {
        let fresh25 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh25 as libc::c_int != '>' as i32) {
            break;
        }
    }
    GetSBMLLex(pibIn, 10 as libc::c_int, 62 as libc::c_int);
    ReadApply(pibIn, &mut bInited, szEqn.as_mut_ptr());
    printf(
        b"reaction %s = %s\n\0" as *const u8 as *const libc::c_char,
        szRName.as_mut_ptr(),
        szEqn.as_mut_ptr(),
    );
    DefineVariable(pibIn, szRName.as_mut_ptr(), szEqn.as_mut_ptr(), 0 as libc::c_int);
    loop {
        let fresh26 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh26 as libc::c_int != '>' as i32) {
            break;
        }
    };
}
pub unsafe extern "C" fn ReadReactions(
    mut pibIn: PINPUTBUF,
    mut iSBML_level: libc::c_int,
) {
    while GetSBMLLex(pibIn, 9 as libc::c_int, 20 as libc::c_int) != 0 {
        if iSBML_level == 1 as libc::c_int {
            ReadReaction_L1(pibIn);
        } else {
            ReadReaction_L2(pibIn);
        }
    }
}
pub unsafe extern "C" fn ReadRule(mut pibIn: PINPUTBUF) {
    let mut szRName: PSTRLEX = [0; 1023];
    let mut szEqn: PSTREQN = [0; 5119];
    let mut bInited: libc::c_int = 0 as libc::c_int;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    (*pinfo).wContext = 0x2 as libc::c_int as WORD;
    loop {
        let fresh27 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh27 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetIdentifier(pibIn, szRName.as_mut_ptr());
    loop {
        let fresh28 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh28 as libc::c_int != '>' as i32) {
            break;
        }
    }
    GetSBMLLex(pibIn, 10 as libc::c_int, 62 as libc::c_int);
    ReadApply(pibIn, &mut bInited, szEqn.as_mut_ptr());
    printf(
        b"rate for %s = %s\n\0" as *const u8 as *const libc::c_char,
        szRName.as_mut_ptr(),
        szEqn.as_mut_ptr(),
    );
    DefineVariable(pibIn, szRName.as_mut_ptr(), szEqn.as_mut_ptr(), 20 as libc::c_int);
    loop {
        let fresh29 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh29 as libc::c_int != '>' as i32) {
            break;
        }
    };
}
pub unsafe extern "C" fn ReadRules(mut pibIn: PINPUTBUF, mut iSBML_level: libc::c_int) {
    if iSBML_level == 1 as libc::c_int {
        printf(
            b"mod: ignoring rate rules definitions in level 1...\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        while GetSBMLLex(pibIn, 15 as libc::c_int, 16 as libc::c_int) != 0 {
            ReadRule(pibIn);
        }
    };
}
pub unsafe extern "C" fn ReadSBMLLevel(mut pibIn: PINPUTBUF) -> libc::c_int {
    let mut szEqn: PSTREQN = [0; 5119];
    let mut iLexType: libc::c_int = 0;
    loop {
        let fresh30 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh30 as libc::c_int != '"' as i32) {
            break;
        }
    }
    loop {
        let fresh31 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh31 as libc::c_int != '"' as i32) {
            break;
        }
    }
    loop {
        let fresh32 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh32 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetNumber(pibIn, szEqn.as_mut_ptr(), &mut iLexType);
    if iLexType == 0 {
        printf(
            b"***Error: cannot read the sbml level - exiting...\n\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    loop {
        let fresh33 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh33 as libc::c_int != '>' as i32) {
            break;
        }
    }
    match szEqn[0 as libc::c_int as usize] as libc::c_int {
        49 => {
            printf(b"sbml level 1\n\0" as *const u8 as *const libc::c_char);
            return 1 as libc::c_int;
        }
        50 => {
            printf(b"sbml level 2\n\0" as *const u8 as *const libc::c_char);
            return 2 as libc::c_int;
        }
        _ => {
            printf(
                b"***Error: unknown sbml level %s - exiting...\n\n\0" as *const u8
                    as *const libc::c_char,
                szEqn.as_mut_ptr(),
            );
            exit(0 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn Read1Species(mut pibIn: PINPUTBUF, mut bProcessPK_ODEs: BOOL) {
    let mut szName: PSTRLEX = [0; 1023];
    let mut szBoundary: PSTRLEX = [0; 1023];
    let mut szCpt: PSTRLEX = [0; 1023];
    let mut szEqn: PSTREQN = [0; 5119];
    let mut iLexType: libc::c_int = 0;
    let mut bBoundary: BOOL = 0;
    let mut sVar: FORSV = FORSV {
        pibIn: 0 as *mut tagINPUTBUF,
        szName: 0 as *mut libc::c_char,
        szVal: 0 as *mut libc::c_char,
        pTarget: 0 as *mut tagVM,
    };
    let mut hType: HANDLE = 0;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    let mut ptempinfo: PINPUTINFO = (*pibIn).pTempInfo as PINPUTINFO;
    let mut pvm: PVMMAPSTRCT = 0 as *mut tagVM;
    (*pinfo).wContext = 0x1 as libc::c_int as WORD;
    loop {
        let fresh34 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh34 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetIdentifier(pibIn, szName.as_mut_ptr());
    (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1 as libc::c_int as isize);
    loop {
        let fresh35 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh35 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetIdentifier(pibIn, szBoundary.as_mut_ptr());
    (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1 as libc::c_int as isize);
    bBoundary = (strcmp(
        szBoundary.as_mut_ptr(),
        b"true\0" as *const u8 as *const libc::c_char,
    ) == 0) as libc::c_int;
    loop {
        let fresh36 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh36 as libc::c_int != '"' as i32) {
            break;
        }
    }
    GetNumber(pibIn, szEqn.as_mut_ptr(), &mut iLexType);
    if (*pinfo).bTemplateInUse != 0 {
        sprintf(szEqn.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1 as libc::c_int as isize);
        loop {
            let fresh37 = (*pibIn).pbufCur;
            (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
            if !(*fresh37 as libc::c_int != '"' as i32) {
                break;
            }
        }
        GetIdentifier(pibIn, szCpt.as_mut_ptr());
        if strcmp(
            szCpt.as_mut_ptr(),
            b"compartment\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            if (GetVarPTR((*ptempinfo).pvmCpts, szCpt.as_mut_ptr())).is_null() {
                printf(
                    b"***Error: template did not defined\0" as *const u8
                        as *const libc::c_char,
                );
                printf(
                    b" compartment '%s' - exiting...\n\n\0" as *const u8
                        as *const libc::c_char,
                    szCpt.as_mut_ptr(),
                );
                exit(0 as libc::c_int);
            } else {
                sprintf(
                    szName.as_mut_ptr(),
                    b"%s_%s\0" as *const u8 as *const libc::c_char,
                    szName.as_mut_ptr(),
                    szCpt.as_mut_ptr(),
                );
            }
            if bBoundary != 0 {
                hType = GetVarType((*pinfo).pvmGloVars, szName.as_mut_ptr()) as HANDLE;
                if hType == 0 {
                    DefineGlobalVar(
                        pibIn,
                        pvm,
                        szName.as_mut_ptr(),
                        szEqn.as_mut_ptr(),
                        hType,
                    );
                    printf(
                        b"param.   %s = %s  (was boundary species)\n\0" as *const u8
                            as *const libc::c_char,
                        szName.as_mut_ptr(),
                        szEqn.as_mut_ptr(),
                    );
                }
            } else {
                SetVar(
                    pibIn,
                    szName.as_mut_ptr(),
                    szEqn.as_mut_ptr(),
                    0x10000 as libc::c_int as HANDLE,
                );
            }
        } else {
            if bBoundary != 0 {
                printf(
                    b"***Error: Species %s is set to boundary;\n\0" as *const u8
                        as *const libc::c_char,
                    szName.as_mut_ptr(),
                );
                printf(
                    b"          It has to be inside a meaningful compartment -\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(b"exiting.\n\n\0" as *const u8 as *const libc::c_char);
                exit(0 as libc::c_int);
            }
            sVar.pibIn = pibIn;
            sVar.szName = szName.as_mut_ptr();
            sVar.szVal = szEqn.as_mut_ptr();
            ForAllVar(
                0 as PFILE,
                (*ptempinfo).pvmGloVars,
                Some(
                    Create1Var
                        as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
                ),
                0 as libc::c_int as HANDLE,
                &mut sVar as *mut FORSV as PVOID,
            );
            (*pinfo).wContext = 0x2 as libc::c_int as WORD;
            sVar.pTarget = (*pinfo).pvmDynEqns;
            ForAllVar(
                0 as PFILE,
                (*ptempinfo).pvmGloVars,
                Some(
                    Create1Var
                        as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
                ),
                0x50000 as libc::c_int as HANDLE,
                &mut sVar as *mut FORSV as PVOID,
            );
            ForAllVar(
                0 as PFILE,
                (*ptempinfo).pvmDynEqns,
                Some(
                    Transcribe1AlgEqn
                        as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
                ),
                0 as libc::c_int as HANDLE,
                &mut sVar as *mut FORSV as PVOID,
            );
            if bProcessPK_ODEs != 0 {
                ForAllVar(
                    0 as PFILE,
                    (*ptempinfo).pvmDynEqns,
                    Some(
                        Transcribe1DiffEqn
                            as unsafe extern "C" fn(
                                PFILE,
                                PVMMAPSTRCT,
                                PVOID,
                            ) -> libc::c_int,
                    ),
                    0 as libc::c_int as HANDLE,
                    &mut sVar as *mut FORSV as PVOID,
                );
            }
            (*pinfo).wContext = 0x3 as libc::c_int as WORD;
            sVar.pTarget = (*pinfo).pvmScaleEqns;
            ForAllVar(
                0 as PFILE,
                (*ptempinfo).pvmGloVars,
                Some(
                    Create1Var
                        as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
                ),
                0x60000 as libc::c_int as HANDLE,
                &mut sVar as *mut FORSV as PVOID,
            );
            ForAllVar(
                0 as PFILE,
                (*ptempinfo).pvmScaleEqns,
                Some(
                    Transcribe1AlgEqn
                        as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
                ),
                0 as libc::c_int as HANDLE,
                &mut sVar as *mut FORSV as PVOID,
            );
            (*pinfo).wContext = 0x5 as libc::c_int as WORD;
            sVar.pTarget = (*pinfo).pvmCalcOutEqns;
            ForAllVar(
                0 as PFILE,
                (*ptempinfo).pvmGloVars,
                Some(
                    Create1Var
                        as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
                ),
                0x80000 as libc::c_int as HANDLE,
                &mut sVar as *mut FORSV as PVOID,
            );
            ForAllVar(
                0 as PFILE,
                (*ptempinfo).pvmCalcOutEqns,
                Some(
                    Transcribe1AlgEqn
                        as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
                ),
                0 as libc::c_int as HANDLE,
                &mut sVar as *mut FORSV as PVOID,
            );
        }
    } else {
        if iLexType == 0 {
            sprintf(szEqn.as_mut_ptr(), b"0.0\0" as *const u8 as *const libc::c_char);
        }
        if bBoundary != 0 {
            hType = GetVarType((*pinfo).pvmGloVars, szName.as_mut_ptr()) as HANDLE;
            if hType == 0 {
                DefineGlobalVar(
                    pibIn,
                    pvm,
                    szName.as_mut_ptr(),
                    szEqn.as_mut_ptr(),
                    hType,
                );
                printf(
                    b"param.   %s = %s  (was boundary species)\n\0" as *const u8
                        as *const libc::c_char,
                    szName.as_mut_ptr(),
                    szEqn.as_mut_ptr(),
                );
            }
        } else {
            SetVar(
                pibIn,
                szName.as_mut_ptr(),
                szEqn.as_mut_ptr(),
                0x10000 as libc::c_int as HANDLE,
            );
        }
    }
    loop {
        let fresh38 = (*pibIn).pbufCur;
        (*pibIn).pbufCur = ((*pibIn).pbufCur).offset(1);
        if !(*fresh38 as libc::c_int != '>' as i32) {
            break;
        }
    };
}
pub unsafe extern "C" fn ReadSpecies(
    mut pibIn: PINPUTBUF,
    mut iSBML_level: libc::c_int,
    mut bProcessPK_ODEs: BOOL,
) {
    while GetSBMLLex(pibIn, 4 as libc::c_int, 6 as libc::c_int) != 0 {
        Read1Species(pibIn, bProcessPK_ODEs);
    }
}
pub unsafe extern "C" fn ReadFileNames(
    mut pibIn: PINPUTBUF,
    mut nFiles: PLONG,
    mut pszNames: *mut *mut PSTR,
) {
    let mut i: libc::c_long = 0;
    let mut iLexType: libc::c_int = 0;
    let mut iErr: libc::c_int = 0 as libc::c_int;
    let mut szLex: [libc::c_char; 256] = [0; 256];
    let mut szPunct: PSTRLEX = [0; 1023];
    let mut pbufStore: PSTR = 0 as *mut libc::c_char;
    pbufStore = (*pibIn).pbufCur;
    loop {
        GetaString(pibIn, szLex.as_mut_ptr());
        *nFiles = *nFiles + 1 as libc::c_int as libc::c_long;
        NextLex(pibIn, szPunct.as_mut_ptr(), &mut iLexType);
        SkipWhitespace(pibIn);
        if iLexType & 0x1 as libc::c_int == 0 {
            if szPunct[0 as libc::c_int as usize] as libc::c_int != ',' as i32
                && szPunct[0 as libc::c_int as usize] as libc::c_int != '}' as i32
            {
                szPunct[1 as libc::c_int as usize] = '}' as i32 as libc::c_char;
                iErr = szPunct[1 as libc::c_int as usize] as libc::c_int;
            }
        }
        if !(szPunct[0 as libc::c_int as usize] as libc::c_int != '}' as i32
            && iErr == 0)
        {
            break;
        }
    }
    *pszNames = malloc(
        (*nFiles as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<PSTR>() as libc::c_ulong),
    ) as *mut PSTR;
    if (*pszNames).is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ReadJModels\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    (*pibIn).pbufCur = pbufStore;
    i = 0 as libc::c_int as libc::c_long;
    while i < *nFiles {
        GetaString(pibIn, szLex.as_mut_ptr());
        NextLex(pibIn, szPunct.as_mut_ptr(), &mut iLexType);
        SkipWhitespace(pibIn);
        let ref mut fresh39 = *(*pszNames).offset(i as isize);
        *fresh39 = malloc(
            (strlen(szLex.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as PSTR;
        if (*fresh39).is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"ReadFileNames\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        } else {
            strcpy(*(*pszNames).offset(i as isize), szLex.as_mut_ptr());
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn ReadSBMLModels(mut pibIn: PINPUTBUF) {
    let mut i: libc::c_long = 0;
    let mut nFiles: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut pszFileNames: *mut PSTR = 0 as *mut PSTR;
    let mut ibInLocal: INPUTBUF = INPUTBUF {
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
    let mut iSBML_level: libc::c_int = 0;
    let mut pinfo: PINPUTINFO = (*pibIn).pInfo as PINPUTINFO;
    ReadFileNames(pibIn, &mut nFiles, &mut pszFileNames);
    i = 0 as libc::c_int as libc::c_long;
    while i < nFiles {
        printf(
            b"\nreading model %s\n\0" as *const u8 as *const libc::c_char,
            *pszFileNames.offset(i as isize),
        );
        if InitBuffer(
            &mut ibInLocal,
            -(1 as libc::c_int) as libc::c_long,
            *pszFileNames.offset(i as isize),
        ) == 0
        {
            ReportError(
                &mut ibInLocal,
                (0x1 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"ReadJModels\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        ibInLocal.pInfo = (*pibIn).pInfo;
        ibInLocal.pTempInfo = (*pibIn).pTempInfo;
        if GetSBMLLex(&mut ibInLocal, 10 as libc::c_int, 10 as libc::c_int) != 0 {
            iSBML_level = ReadSBMLLevel(&mut ibInLocal);
        }
        if (*pinfo).bTemplateInUse != 0 && iSBML_level < 2 as libc::c_int {
            printf(
                b"***Error: use of a PK template requires \0" as *const u8
                    as *const libc::c_char,
            );
            printf(b"SBML level 2 - exiting.\n\n\0" as *const u8 as *const libc::c_char);
            exit(0 as libc::c_int);
        }
        if (*pinfo).bTemplateInUse != 0 {
            ibInLocal.pbufCur = ibInLocal.pbufOrg;
            if GetSBMLLex(&mut ibInLocal, 10 as libc::c_int, 2 as libc::c_int) != 0 {
                ReadCpts(&mut ibInLocal, 1 as libc::c_int);
            }
        } else {
            printf(
                b"no PK template given: ignoring SBML compartments\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        ibInLocal.pbufCur = ibInLocal.pbufOrg;
        if GetSBMLLex(&mut ibInLocal, 10 as libc::c_int, 17 as libc::c_int) != 0 {
            ReadFunctions(&mut ibInLocal, iSBML_level);
        }
        ibInLocal.pbufCur = ibInLocal.pbufOrg;
        while GetSBMLLex(&mut ibInLocal, 10 as libc::c_int, 7 as libc::c_int) != 0 {
            ReadParameters(&mut ibInLocal);
        }
        ibInLocal.pbufCur = ibInLocal.pbufOrg;
        if GetSBMLLex(&mut ibInLocal, 10 as libc::c_int, 4 as libc::c_int) != 0 {
            ReadSpecies(&mut ibInLocal, iSBML_level, 0 as libc::c_int);
        }
        ibInLocal.pbufCur = ibInLocal.pbufOrg;
        if GetSBMLLex(&mut ibInLocal, 10 as libc::c_int, 15 as libc::c_int) != 0 {
            ReadRules(&mut ibInLocal, iSBML_level);
        }
        ibInLocal.pbufCur = ibInLocal.pbufOrg;
        if GetSBMLLex(&mut ibInLocal, 10 as libc::c_int, 9 as libc::c_int) != 0 {
            ReadReactions(&mut ibInLocal, iSBML_level);
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < nFiles {
        if InitBuffer(
            &mut ibInLocal,
            -(1 as libc::c_int) as libc::c_long,
            *pszFileNames.offset(i as isize),
        ) == 0
        {
            ReportError(
                &mut ibInLocal,
                (0x1 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"ReadJModels\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        ibInLocal.pInfo = (*pibIn).pInfo;
        ibInLocal.pTempInfo = (*pibIn).pTempInfo;
        if (*pinfo).bTemplateInUse != 0 {
            if GetSBMLLex(&mut ibInLocal, 10 as libc::c_int, 2 as libc::c_int) != 0 {
                ReadCpts(&mut ibInLocal, 0 as libc::c_int);
            }
        }
        printf(
            b"\nmod: reading differentials in model %s\n\0" as *const u8
                as *const libc::c_char,
            *pszFileNames.offset(i as isize),
        );
        ibInLocal.pbufCur = ibInLocal.pbufOrg;
        if GetSBMLLex(&mut ibInLocal, 10 as libc::c_int, 4 as libc::c_int) != 0 {
            ReadSpecies(&mut ibInLocal, iSBML_level, 1 as libc::c_int);
        }
        ReadDifferentials(&mut ibInLocal);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as libc::c_long;
    while i < nFiles {
        free(*pszFileNames.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(pszFileNames as *mut libc::c_void);
    (*pinfo).wContext = 0x4000 as libc::c_int as WORD;
}
pub unsafe extern "C" fn ReadPKTemplate(mut pibIn: PINPUTBUF) {
    let mut ibInLocal: INPUTBUF = INPUTBUF {
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
    let mut nFiles: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut pszFileNames: *mut PSTR = 0 as *mut PSTR;
    let mut pinfo: PINPUTINFO = 0 as *mut tagINPUTINFO;
    pinfo = (*pibIn).pTempInfo as PINPUTINFO;
    (*pinfo).wContext = 0x1 as libc::c_int as WORD;
    ReadFileNames(pibIn, &mut nFiles, &mut pszFileNames);
    if nFiles > 1 as libc::c_int as libc::c_long {
        printf(
            b"mod: cannot use more that one template - using only the 1st\n\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(
        b"%s\n\0" as *const u8 as *const libc::c_char,
        *pszFileNames.offset(0 as libc::c_int as isize),
    );
    if InitBuffer(
        &mut ibInLocal,
        0x1000 as libc::c_int as libc::c_long,
        *pszFileNames.offset(0 as libc::c_int as isize),
    ) == 0
    {
        ReportError(
            &mut ibInLocal,
            (0x1 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"ReadModel\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    ibInLocal.pInfo = pinfo as PVOID;
    loop {
        NextLex(&mut ibInLocal, szLex.as_mut_ptr(), &mut iLexType);
        let mut current_block_15: u64;
        match iLexType {
            0 => {
                (*pinfo).wContext = 0x4000 as libc::c_int as WORD;
                current_block_15 = 2719512138335094285;
            }
            1 => {
                ProcessWord(&mut ibInLocal, szLex.as_mut_ptr(), szEqn.as_mut_ptr());
                current_block_15 = 2719512138335094285;
            }
            8 | 32 => {
                if szLex[0 as libc::c_int as usize] as libc::c_int == ';' as i32 {
                    current_block_15 = 2719512138335094285;
                } else if szLex[0 as libc::c_int as usize] as libc::c_int == '}' as i32
                    && (*pinfo).wContext
                        & (0x2 as libc::c_int | 0x4 as libc::c_int | 0x3 as libc::c_int)
                            as libc::c_uint != 0
                {
                    (*pinfo).wContext = 0x1 as libc::c_int as WORD;
                    current_block_15 = 2719512138335094285;
                } else if szLex[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
                    SkipComment(&mut ibInLocal);
                    current_block_15 = 2719512138335094285;
                } else {
                    current_block_15 = 7767059276039156567;
                }
            }
            2 | 4 => {
                ReportError(
                    &mut ibInLocal,
                    0x13 as libc::c_int as WORD,
                    szLex.as_mut_ptr(),
                    b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
                );
                current_block_15 = 2719512138335094285;
            }
            _ => {
                current_block_15 = 7767059276039156567;
            }
        }
        match current_block_15 {
            7767059276039156567 => {
                ReportError(
                    &mut ibInLocal,
                    0x11 as libc::c_int as WORD,
                    szLex.as_mut_ptr(),
                    b"* Ignoring\0" as *const u8 as *const libc::c_char as PSTR,
                );
            }
            _ => {}
        }
        if !((*pinfo).wContext != 0x4000 as libc::c_int as libc::c_uint
            && (*ibInLocal.pbufCur as libc::c_int != 0
                || FillBuffer(&mut ibInLocal, 0x1000 as libc::c_int as libc::c_long)
                    != -(1 as libc::c_int)))
        {
            break;
        }
    }
    fclose(ibInLocal.pfileIn);
    ReversePointers(&mut (*pinfo).pvmGloVars);
    ReversePointers(&mut (*pinfo).pvmDynEqns);
    ReversePointers(&mut (*pinfo).pvmScaleEqns);
    ReversePointers(&mut (*pinfo).pvmCalcOutEqns);
    ReversePointers(&mut (*pinfo).pvmJacobEqns);
    pinfo = (*pibIn).pInfo as PINPUTINFO;
    (*pinfo).wContext = 0x200 as libc::c_int as WORD;
    (*pinfo).bTemplateInUse = 1 as libc::c_int;
}
