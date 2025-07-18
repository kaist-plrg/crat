use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn MakeStringBuffer(pBuf: PINPUTBUF, pStrBuf: PINPUTBUF, sz: PSTR);
    fn GetFuncArgs(
        pibIn: PINPUTBUF,
        nArgs: libc::c_int,
        rgiArgTypes: *mut libc::c_int,
        szArgs: PSTR,
        rgiLowerB: *mut libc::c_long,
        rgiUpperB: *mut libc::c_long,
    ) -> BOOL;
    fn NextLex(_: PINPUTBUF, _: *mut libc::c_char, _: PINT);
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn GetKeywordCode(szKeyword: PSTR, pfContext: PINT) -> libc::c_int;
    fn GetVarPTR(pvm: PVMMAPSTRCT, szName: PSTR) -> PVMMAPSTRCT;
    static mut vszHasInitializer: [libc::c_char; 0];
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
pub type BOOL = libc::c_int;
pub type WORD = libc::c_uint;
pub type DWORD = libc::c_ulong;
pub type HANDLE = DWORD;
pub type PSTR = *mut libc::c_char;
pub type PINT = *mut libc::c_int;
pub type PWORD = *mut WORD;
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
pub type PFI_CALLBACK = Option::<
    unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
>;
static mut vszModelFilename: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut vszModGenName: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut vszModelArrayName: [libc::c_char; 13] = unsafe {
    *::std::mem::transmute::<&[u8; 13], &mut [libc::c_char; 13]>(b"vrgModelVars\0")
};
static mut vszInputArrayName: [libc::c_char; 10] = unsafe {
    *::std::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(b"vrgInputs\0")
};
pub static mut vpvmGloVarList: PVMMAPSTRCT = 0 as *const tagVM as *mut tagVM;
pub static mut vszIFNTypes: [*mut libc::c_char; 6] = [
    b"IFN_NULL /* ?? */\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"IFN_CONSTANT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"IFN_PERDOSE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"IFN_PERRATE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"IFN_PEREXP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"IFN_NDOSES\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut vnStates: libc::c_int = 0;
pub static mut vnOutputs: libc::c_int = 0;
pub static mut vnInputs: libc::c_int = 0;
pub static mut vnParms: libc::c_int = 0;
pub static mut vnModelVars: libc::c_int = 0;
pub static mut bForR: BOOL = 0 as libc::c_int;
pub static mut bForInits: BOOL = 0 as libc::c_int;
pub static mut bDelay: BOOL = 1 as libc::c_int;
pub unsafe extern "C" fn ForAllVar(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pfiFunc: PFI_CALLBACK,
    mut hType: HANDLE,
    mut pinfo: PVOID,
) -> libc::c_int {
    let mut iTotal: libc::c_int = 0 as libc::c_int;
    while !pvm.is_null() {
        if hType == 0 as libc::c_int as libc::c_ulong
            || (if !pvm.is_null() {
                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) == hType
        {
            if pfiFunc.is_some() {
                iTotal += (Some(pfiFunc.unwrap())).unwrap()(pfile, pvm, pinfo);
            } else {
                iTotal += 1;
                iTotal;
            }
        }
        pvm = (*pvm).pvmNextVar;
    }
    return iTotal;
}
pub unsafe extern "C" fn CountOneDecl(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    if (*pvm).szEqn != vszHasInitializer.as_mut_ptr() {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn WritebDelays(mut pfile: PFILE, mut bDelays: BOOL) {
    fprintf(
        pfile,
        b"\nBOOL bDelays = %d;\n\0" as *const u8 as *const libc::c_char,
        bDelays,
    );
}
pub unsafe extern "C" fn WriteOneName(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    if (*pvm).szEqn != vszHasInitializer.as_mut_ptr() {
        if bForR != 0 {
            if (if !pvm.is_null() {
                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) == 0x30000 as libc::c_int as libc::c_ulong
            {
                fprintf(
                    pfile,
                    b"    \"%s\0" as *const u8 as *const libc::c_char,
                    (*pvm).szName,
                );
            } else {
                fprintf(
                    pfile,
                    b"     %s\0" as *const u8 as *const libc::c_char,
                    (*pvm).szName,
                );
            }
            if (if !pvm.is_null() {
                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) != 0x20000 as libc::c_int as libc::c_ulong
            {
                if (if !pvm.is_null() {
                    (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) == 0x30000 as libc::c_int as libc::c_ulong
                {
                    fprintf(pfile, b"\",\n\0" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(
                        pfile,
                        b" = %s,\n\0" as *const u8 as *const libc::c_char,
                        if !((*pvm).szEqn).is_null() {
                            (*pvm).szEqn as *const libc::c_char
                        } else {
                            b"0.0\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
            } else {
                fprintf(
                    pfile,
                    b" (forcing function)\n\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            fprintf(
                pfile,
                b"     %s\0" as *const u8 as *const libc::c_char,
                (*pvm).szName,
            );
            if (if !pvm.is_null() {
                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) != 0x20000 as libc::c_int as libc::c_ulong
            {
                fprintf(
                    pfile,
                    b" %s %s;\n\0" as *const u8 as *const libc::c_char,
                    if !((*pvm).szEqn).is_null() {
                        b"=\0" as *const u8 as *const libc::c_char
                    } else {
                        b"->\0" as *const u8 as *const libc::c_char
                    },
                    if !((*pvm).szEqn).is_null() {
                        (*pvm).szEqn as *const libc::c_char
                    } else {
                        b"0.0\0" as *const u8 as *const libc::c_char
                    },
                );
            } else {
                fprintf(
                    pfile,
                    b" (is a function)\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn WriteHeader(
    mut pfile: PFILE,
    mut szName: PSTR,
    mut pvmGlo: PVMMAPSTRCT,
) {
    let mut ttTime: time_t = 0;
    time(&mut ttTime);
    if fprintf(pfile, b"/* %s\n\0" as *const u8 as *const libc::c_char, szName)
        < 0 as libc::c_int
    {
        ReportError(
            0 as PINPUTBUF,
            (0x3 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szName,
            b"...in WriteHeader ()\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
    fprintf(
        pfile,
        b"   ___________________________________________________\n\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"   Model File:  %s\n\n\0" as *const u8 as *const libc::c_char,
        vszModelFilename,
    );
    fprintf(
        pfile,
        b"   Date:  %s\n\0" as *const u8 as *const libc::c_char,
        ctime(&mut ttTime),
    );
    fprintf(
        pfile,
        b"   Created by:  \"%s %s\"\n\0" as *const u8 as *const libc::c_char,
        vszModGenName,
        b"v6.2.0\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"    -- a model preprocessor by Don Maszle\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"   ___________________________________________________\n\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"   Copyright (c) 1993-2020 Free Software Foundation, Inc.\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"\n   Model calculations for compartmental model:\n\n\0" as *const u8
            as *const libc::c_char,
    );
    if vnStates == 1 as libc::c_int {
        fprintf(pfile, b"   1 State:\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            pfile,
            b"   %d States:\n\0" as *const u8 as *const libc::c_char,
            vnStates,
        );
    }
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneName
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x10000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    if vnOutputs == 1 as libc::c_int {
        fprintf(pfile, b"\n   1 Output:\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            pfile,
            b"\n   %d Outputs:\n\0" as *const u8 as *const libc::c_char,
            vnOutputs,
        );
    }
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneName
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x30000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    if vnInputs == 1 as libc::c_int {
        fprintf(pfile, b"\n   1 Input:\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            pfile,
            b"\n   %d Inputs:\n\0" as *const u8 as *const libc::c_char,
            vnInputs,
        );
    }
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneName
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x20000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    if vnParms == 1 as libc::c_int {
        fprintf(pfile, b"\n   1 Parameter:\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(
            pfile,
            b"\n   %d Parameters:\n\0" as *const u8 as *const libc::c_char,
            vnParms,
        );
    }
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneName
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b"*/\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn WriteIncludes(mut pfile: PFILE) {
    fprintf(pfile, b"\n#include <stdlib.h>\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"#include <stdio.h>\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"#include <math.h>\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"#include <string.h>\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"#include <float.h>\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"#include \"modelu.h\"\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"#include \"random.h\"\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"#include \"yourcode.h\"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn WriteOneDecl(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    if (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) != 0x20000 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"TYPE(pvm) != ID_INPUT\0" as *const u8 as *const libc::c_char,
            b"modo.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"int WriteOneDecl(PFILE, PVMMAPSTRCT, PVOID)\0"))
                .as_ptr(),
        );
    }
    'c_5415: {
        if (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) != 0x20000 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"TYPE(pvm) != ID_INPUT\0" as *const u8 as *const libc::c_char,
                b"modo.c\0" as *const u8 as *const libc::c_char,
                238 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"int WriteOneDecl(PFILE, PVMMAPSTRCT, PVOID)\0"))
                    .as_ptr(),
            );
        }
    };
    if (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) != 0x30000 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"TYPE(pvm) != ID_OUTPUT\0" as *const u8 as *const libc::c_char,
            b"modo.c\0" as *const u8 as *const libc::c_char,
            239 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"int WriteOneDecl(PFILE, PVMMAPSTRCT, PVOID)\0"))
                .as_ptr(),
        );
    }
    'c_5352: {
        if (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) != 0x30000 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"TYPE(pvm) != ID_OUTPUT\0" as *const u8 as *const libc::c_char,
                b"modo.c\0" as *const u8 as *const libc::c_char,
                239 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"int WriteOneDecl(PFILE, PVMMAPSTRCT, PVOID)\0"))
                    .as_ptr(),
            );
        }
    };
    if (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) != 0x10000 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"TYPE(pvm) != ID_STATE\0" as *const u8 as *const libc::c_char,
            b"modo.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"int WriteOneDecl(PFILE, PVMMAPSTRCT, PVOID)\0"))
                .as_ptr(),
        );
    }
    'c_5289: {
        if (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) != 0x10000 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"TYPE(pvm) != ID_STATE\0" as *const u8 as *const libc::c_char,
                b"modo.c\0" as *const u8 as *const libc::c_char,
                240 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"int WriteOneDecl(PFILE, PVMMAPSTRCT, PVOID)\0"))
                    .as_ptr(),
            );
        }
    };
    if (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) > 0x40000 as libc::c_int as libc::c_ulong
    {
        fprintf(pfile, b"  /* local */ \0" as *const u8 as *const libc::c_char);
    }
    fprintf(pfile, b"double %s;\n\0" as *const u8 as *const libc::c_char, (*pvm).szName);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn WriteOneIndexDefine(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    if (*pvm).szEqn != vszHasInitializer.as_mut_ptr() {
        fprintf(pfile, b"#define \0" as *const u8 as *const libc::c_char);
        fprintf(pfile, b"ID_%s\0" as *const u8 as *const libc::c_char, (*pvm).szName);
        if if !pvm.is_null() {
            (*pvm).hType & 0x7fff as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        } != 0
        {
            fprintf(
                pfile,
                b" 0x%05lx\n\0" as *const u8 as *const libc::c_char,
                if !pvm.is_null() {
                    (*pvm).hType & 0x7fff as libc::c_int as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                },
            );
        } else {
            fprintf(pfile, b" 0x00000\n\0" as *const u8 as *const libc::c_char);
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn WriteDecls(mut pfile: PFILE, mut pvmGlo: PVMMAPSTRCT) {
    fprintf(
        pfile,
        b"\n\n/*----- Indices to Global Variables */\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"\n/* Model variables: States and other outputs */\n\0" as *const u8
            as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneIndexDefine
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x10000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneIndexDefine
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x30000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b"\n/* Inputs */\n\0" as *const u8 as *const libc::c_char);
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneIndexDefine
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x20000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b"\n/* Parameters */\n\0" as *const u8 as *const libc::c_char);
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneIndexDefine
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(
        pfile,
        b"\n\n/*----- Global Variables */\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"\n/* For export. Keep track of who we are. */\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"char szModelDescFilename[] = \"%s\";\n\0" as *const u8 as *const libc::c_char,
        vszModelFilename,
    );
    fprintf(
        pfile,
        b"char szModelSourceFilename[] = __FILE__;\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"char szModelGenAndVersion[] = \"%s %s\";\n\0" as *const u8
            as *const libc::c_char,
        vszModGenName,
        b"v6.2.0\0" as *const u8 as *const libc::c_char,
    );
    fprintf(pfile, b"\n/* Externs */\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"extern BOOL vbModelReinitd;\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(pfile, b"\n/* Model Dimensions */\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"int vnStates = %d;\n\0" as *const u8 as *const libc::c_char,
        vnStates,
    );
    fprintf(
        pfile,
        b"int vnOutputs = %d;\n\0" as *const u8 as *const libc::c_char,
        vnOutputs,
    );
    fprintf(
        pfile,
        b"int vnModelVars = %d;\n\0" as *const u8 as *const libc::c_char,
        vnModelVars,
    );
    fprintf(
        pfile,
        b"int vnInputs = %d;\n\0" as *const u8 as *const libc::c_char,
        vnInputs,
    );
    fprintf(
        pfile,
        b"int vnParms = %d;\n\0" as *const u8 as *const libc::c_char,
        vnParms,
    );
    fprintf(pfile, b"\n/* States and Outputs*/\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"double %s[%d];\n\0" as *const u8 as *const libc::c_char,
        vszModelArrayName.as_mut_ptr(),
        vnModelVars,
    );
    fprintf(pfile, b"\n/* Inputs */\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"IFN %s[%d];\n\0" as *const u8 as *const libc::c_char,
        vszInputArrayName.as_mut_ptr(),
        if vnInputs > 0 as libc::c_int { vnInputs } else { 1 as libc::c_int },
    );
    fprintf(pfile, b"\n/* Parameters */\n\0" as *const u8 as *const libc::c_char);
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn GetName(
    mut pvm: PVMMAPSTRCT,
    mut szModelVarName: PSTR,
    mut szDerivName: PSTR,
    mut hType: HANDLE,
) -> PSTR {
    static mut vszVarName: PSTRLEX = [0; 1023];
    let mut hTypeToUse: HANDLE = if hType != 0 {
        hType
    } else if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    match hTypeToUse {
        131072 => {
            if bForR != 0 {
                sprintf(
                    vszVarName.as_mut_ptr(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*pvm).szName,
                );
            } else {
                sprintf(
                    vszVarName.as_mut_ptr(),
                    b"vrgInputs[ID_%s]\0" as *const u8 as *const libc::c_char,
                    (*pvm).szName,
                );
            }
        }
        65536 => {
            if bForR != 0 {
                if bForInits != 0 {
                    sprintf(
                        vszVarName.as_mut_ptr(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        (*pvm).szName,
                    );
                } else {
                    sprintf(
                        vszVarName.as_mut_ptr(),
                        b"y[ID_%s]\0" as *const u8 as *const libc::c_char,
                        (*pvm).szName,
                    );
                }
            } else if !szModelVarName.is_null() {
                sprintf(
                    vszVarName.as_mut_ptr(),
                    b"%s[ID_%s]\0" as *const u8 as *const libc::c_char,
                    szModelVarName,
                    (*pvm).szName,
                );
            } else {
                sprintf(
                    vszVarName.as_mut_ptr(),
                    b"vrgModelVars[ID_%s]\0" as *const u8 as *const libc::c_char,
                    (*pvm).szName,
                );
            }
        }
        196608 => {
            if bForR != 0 {
                sprintf(
                    vszVarName.as_mut_ptr(),
                    b"yout[ID_%s]\0" as *const u8 as *const libc::c_char,
                    (*pvm).szName,
                );
            } else if !szModelVarName.is_null() {
                sprintf(
                    vszVarName.as_mut_ptr(),
                    b"%s[ID_%s]\0" as *const u8 as *const libc::c_char,
                    szModelVarName,
                    (*pvm).szName,
                );
            } else {
                sprintf(
                    vszVarName.as_mut_ptr(),
                    b"vrgModelVars[ID_%s]\0" as *const u8 as *const libc::c_char,
                    (*pvm).szName,
                );
            }
        }
        589824 => {
            if !szDerivName.is_null() {} else {
                __assert_fail(
                    b"szDerivName\0" as *const u8 as *const libc::c_char,
                    b"modo.c\0" as *const u8 as *const libc::c_char,
                    375 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<
                        &[u8; 46],
                        &[libc::c_char; 46],
                    >(b"PSTR GetName(PVMMAPSTRCT, PSTR, PSTR, HANDLE)\0"))
                        .as_ptr(),
                );
            }
            'c_3504: {
                if !szDerivName.is_null() {} else {
                    __assert_fail(
                        b"szDerivName\0" as *const u8 as *const libc::c_char,
                        b"modo.c\0" as *const u8 as *const libc::c_char,
                        375 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<
                            &[u8; 46],
                            &[libc::c_char; 46],
                        >(b"PSTR GetName(PVMMAPSTRCT, PSTR, PSTR, HANDLE)\0"))
                            .as_ptr(),
                    );
                }
            };
            if bForR != 0 {
                sprintf(
                    vszVarName.as_mut_ptr(),
                    b"ydot[ID_%s]\0" as *const u8 as *const libc::c_char,
                    (*pvm).szName,
                );
            } else {
                sprintf(
                    vszVarName.as_mut_ptr(),
                    b"%s[ID_%s]\0" as *const u8 as *const libc::c_char,
                    szDerivName,
                    (*pvm).szName,
                );
            }
        }
        _ => {
            sprintf(
                vszVarName.as_mut_ptr(),
                b"%s\0" as *const u8 as *const libc::c_char,
                (*pvm).szName,
            );
        }
    }
    return vszVarName.as_mut_ptr();
}
pub unsafe extern "C" fn WriteOneVMEntry(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut iType: libc::c_int = (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
    if pvm.is_null() {
        fprintf(
            pfile,
            b"  {\"\", NULL, 0} /* End flag */\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if iType != 0 && iType != 0x50000 as libc::c_int && iType != 0x60000 as libc::c_int
        && iType != 0x70000 as libc::c_int
    {} else {
        __assert_fail(
            b"iType && iType != ID_LOCALDYN && iType != ID_LOCALSCALE && iType != ID_LOCALJACOB\0"
                as *const u8 as *const libc::c_char,
            b"modo.c\0" as *const u8 as *const libc::c_char,
            409 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"int WriteOneVMEntry(PFILE, PVMMAPSTRCT, PVOID)\0"))
                .as_ptr(),
        );
    }
    'c_7484: {
        if iType != 0 && iType != 0x50000 as libc::c_int
            && iType != 0x60000 as libc::c_int && iType != 0x70000 as libc::c_int
        {} else {
            __assert_fail(
                b"iType && iType != ID_LOCALDYN && iType != ID_LOCALSCALE && iType != ID_LOCALJACOB\0"
                    as *const u8 as *const libc::c_char,
                b"modo.c\0" as *const u8 as *const libc::c_char,
                409 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 47],
                    &[libc::c_char; 47],
                >(b"int WriteOneVMEntry(PFILE, PVMMAPSTRCT, PVOID)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*pvm).szEqn != vszHasInitializer.as_mut_ptr() {
        fprintf(
            pfile,
            b"  {\"%s\", (PVOID) &%s\0" as *const u8 as *const libc::c_char,
            (*pvm).szName,
            GetName(
                pvm,
                vszModelArrayName.as_mut_ptr(),
                0 as PSTR,
                0 as libc::c_int as HANDLE,
            ),
        );
        fprintf(
            pfile,
            b", ID_%s | ID_%s},\n\0" as *const u8 as *const libc::c_char,
            if iType == 0x40000 as libc::c_int {
                b"PARM\0" as *const u8 as *const libc::c_char
            } else if iType == 0x20000 as libc::c_int {
                b"INPUT\0" as *const u8 as *const libc::c_char
            } else if iType == 0x30000 as libc::c_int {
                b"OUTPUT\0" as *const u8 as *const libc::c_char
            } else {
                b"STATE\0" as *const u8 as *const libc::c_char
            },
            (*pvm).szName,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn WriteVarMap(mut pfile: PFILE, mut pvmGlo: PVMMAPSTRCT) {
    fprintf(
        pfile,
        b"\n\n/*----- Global Variable Map */\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(pfile, b"VMMAPSTRCT vrgvmGlo[] = {\n\0" as *const u8 as *const libc::c_char);
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneVMEntry
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x10000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneVMEntry
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x30000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneVMEntry
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x20000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneVMEntry
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    WriteOneVMEntry(pfile, 0 as PVMMAPSTRCT, 0 as *mut libc::c_void);
    fprintf(pfile, b"};  /* vrgpvmGlo[] */\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn WriteOneInit(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut ifnNull: IFN = {
        let mut init = tagIFN {
            iType: 1 as libc::c_int,
            bOn: 0,
            dMag: 0.,
            dTper: 0.,
            dT0: 0.,
            dTexp: 0.,
            dDecay: 0.,
            dVal: 0.,
            dTStartPeriod: 0.,
            hMag: 0,
            hTper: 0,
            hT0: 0,
            hTexp: 0,
            hDecay: 0,
            nDoses: 0,
            iDoseCur: 0,
            rgT0s: 0 as *mut libc::c_double,
            rgTexps: 0 as *mut libc::c_double,
            rgMags: 0 as *mut libc::c_double,
        };
        init
    };
    let mut szVarName: PSTR = GetName(
        pvm,
        0 as PSTR,
        0 as PSTR,
        0 as libc::c_int as HANDLE,
    );
    if (*pvm).szEqn != vszHasInitializer.as_mut_ptr()
        && (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) <= 0x40000 as libc::c_int as libc::c_ulong
        || (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) == 0xa0000 as libc::c_int as libc::c_ulong
    {
        if (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) == 0x20000 as libc::c_int as libc::c_ulong
        {
            let mut pifn: PIFN = (*pvm).szEqn as PIFN;
            if pifn.is_null() {
                pifn = &mut ifnNull;
            }
            fprintf(
                pfile,
                b"  %s.iType = %s;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                vszIFNTypes[(*pifn).iType as usize],
            );
            fprintf(
                pfile,
                b"  %s.dTStartPeriod = 0;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
            );
            fprintf(
                pfile,
                b"  %s.bOn = FALSE;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
            );
            fprintf(
                pfile,
                b"  %s.dMag = %f;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).dMag,
            );
            fprintf(
                pfile,
                b"  %s.dT0 = %f;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).dT0,
            );
            fprintf(
                pfile,
                b"  %s.dTexp = %f;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).dTexp,
            );
            fprintf(
                pfile,
                b"  %s.dDecay = %f;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).dDecay,
            );
            fprintf(
                pfile,
                b"  %s.dTper = %f;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).dTper,
            );
            fprintf(
                pfile,
                b"  %s.hMag = %#lx;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).hMag,
            );
            fprintf(
                pfile,
                b"  %s.hT0 = %#lx;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).hT0,
            );
            fprintf(
                pfile,
                b"  %s.hTexp = %#lx;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).hTexp,
            );
            fprintf(
                pfile,
                b"  %s.hDecay = %#lx;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).hDecay,
            );
            fprintf(
                pfile,
                b"  %s.hTper = %#lx;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pifn).hTper,
            );
            fprintf(
                pfile,
                b"  %s.dVal = 0.0;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
            );
            fprintf(
                pfile,
                b"  %s.nDoses = 0;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
            );
        } else if (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) == 0xa0000 as libc::c_int as libc::c_ulong
        {
            fprintf(
                pfile,
                b"\n%s\n\0" as *const u8 as *const libc::c_char,
                (*pvm).szEqn,
            );
        } else {
            fprintf(
                pfile,
                b"  %s = %s;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                if !((*pvm).szEqn).is_null() {
                    (*pvm).szEqn as *const libc::c_char
                } else {
                    b"0.0\0" as *const u8 as *const libc::c_char
                },
            );
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn WriteInitModel(mut pfile: PFILE, mut pvmGlo: PVMMAPSTRCT) {
    fprintf(pfile, b"\n\n/*----- InitModel\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"   Should be called to initialize model variables at\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"   the beginning of experiment before reading\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"   variants from the simulation spec file.\n*/\n\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"void InitModel(void)\n{\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"  /* Initialize things in the order that they appear in\n     model definition file so that dependencies are\n     handled correctly. */\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneInit
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(
        pfile,
        b"\n  vbModelReinitd = TRUE;\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(pfile, b"} /* InitModel */\n\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn TranslateID(
    mut pibDum: PINPUTBUF,
    mut pfile: PFILE,
    mut szLex: PSTR,
    mut iEqType: libc::c_int,
) {
    let mut iKWCode: libc::c_int = 0;
    let mut fContext: libc::c_int = 0;
    let mut iLowerB: libc::c_long = 0;
    let mut iUpperB: libc::c_long = 0;
    iKWCode = GetKeywordCode(szLex, &mut fContext);
    match iKWCode {
        20 => {
            let mut iArg: libc::c_int = 0x1 as libc::c_int;
            let mut pvm: PVMMAPSTRCT = 0 as PVMMAPSTRCT;
            if GetFuncArgs(
                pibDum,
                1 as libc::c_int,
                &mut iArg,
                szLex,
                &mut iLowerB,
                &mut iUpperB,
            ) != 0
                && {
                    pvm = GetVarPTR(vpvmGloVarList, szLex);
                    !pvm.is_null()
                }
                && (if !pvm.is_null() {
                    (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) == 0x10000 as libc::c_int as libc::c_ulong
            {
                fprintf(
                    pfile,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    GetName(
                        pvm,
                        0 as PSTR,
                        b"rgDerivs\0" as *const u8 as *const libc::c_char as PSTR,
                        0x90000 as libc::c_int as HANDLE,
                    ),
                );
            } else {
                ReportError(
                    pibDum,
                    (0x100 as libc::c_int + 5 as libc::c_int | 0x8000 as libc::c_int)
                        as WORD,
                    if !pvm.is_null() { szLex } else { 0 as PSTR },
                    0 as PSTR,
                );
            }
        }
        0 => {
            let mut pvm_0: PVMMAPSTRCT = GetVarPTR(vpvmGloVarList, szLex);
            if pvm_0.is_null() {
                if (iEqType == 4 as libc::c_int || iEqType == 5 as libc::c_int
                    || iEqType == 7 as libc::c_int)
                    && !(strcmp(
                        szLex as *const libc::c_char,
                        b"t\0" as *const u8 as *const libc::c_char,
                    ) != 0
                        && strcmp(
                            szLex as *const libc::c_char,
                            b"time\0" as *const u8 as *const libc::c_char,
                        ) != 0)
                {
                    fprintf(pfile, b"(*pdTime)\0" as *const u8 as *const libc::c_char);
                } else {
                    fprintf(pfile, b"%s\0" as *const u8 as *const libc::c_char, szLex);
                }
            } else {
                if iEqType == 5 as libc::c_int {
                    fprintf(
                        pfile,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        GetName(pvm_0, 0 as PSTR, 0 as PSTR, 0 as libc::c_int as HANDLE),
                    );
                } else {
                    fprintf(
                        pfile,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        GetName(
                            pvm_0,
                            b"rgModelVars\0" as *const u8 as *const libc::c_char as PSTR,
                            0 as PSTR,
                            0 as libc::c_int as HANDLE,
                        ),
                    );
                }
                if (if !pvm_0.is_null() {
                    (*pvm_0).hType & 0xf0000 as libc::c_int as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) == 0x20000 as libc::c_int as libc::c_ulong && bForR == 0
                {
                    fprintf(pfile, b".dVal\0" as *const u8 as *const libc::c_char);
                }
            }
        }
        _ => {
            ReportError(
                pibDum,
                (0x100 as libc::c_int + 1 as libc::c_int | 0x8000 as libc::c_int)
                    as WORD,
                szLex,
                0 as PSTR,
            );
        }
    };
}
pub unsafe extern "C" fn TranslateEquation(
    mut pfile: PFILE,
    mut szEqn: PSTR,
    mut iEqType: libc::c_long,
) {
    let mut ibDum: INPUTBUF = INPUTBUF {
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
    let mut pibDum: PINPUTBUF = &mut ibDum;
    let mut szLex: PSTRLEX = [0; 1023];
    let mut pvm: PVMMAPSTRCT = 0 as PVMMAPSTRCT;
    let mut iType: libc::c_int = 0;
    let mut bDelayCall: BOOL = 0 as libc::c_int;
    MakeStringBuffer(0 as PINPUTBUF, pibDum, szEqn);
    NextLex(pibDum, szLex.as_mut_ptr(), &mut iType);
    if iType == 0 {
        fprintf(pfile, b"0.0;  /* NULL EQN!?? */\0" as *const u8 as *const libc::c_char);
        return;
    }
    loop {
        if iType == 0x1 as libc::c_int {
            if bDelayCall != 0 {
                pvm = GetVarPTR(vpvmGloVarList, szLex.as_mut_ptr());
                if bForR != 0
                    && (!pvm.is_null()
                        && (if !pvm.is_null() {
                            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
                        } else {
                            0 as libc::c_int as libc::c_ulong
                        }) == 0x10000 as libc::c_int as libc::c_ulong)
                    || bForR == 0
                        && (!pvm.is_null()
                            && (if !pvm.is_null() {
                                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
                            } else {
                                0 as libc::c_int as libc::c_ulong
                            }) == 0x10000 as libc::c_int as libc::c_ulong
                            || (if !pvm.is_null() {
                                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
                            } else {
                                0 as libc::c_int as libc::c_ulong
                            }) == 0x30000 as libc::c_int as libc::c_ulong)
                {
                    fprintf(
                        pfile,
                        b"ID_%s\0" as *const u8 as *const libc::c_char,
                        szLex.as_mut_ptr(),
                    );
                    fprintf(pfile, b", (*pdTime)\0" as *const u8 as *const libc::c_char);
                    bDelayCall = 0 as libc::c_int;
                } else {
                    ReportError(
                        pibDum,
                        (0x15 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                        (if bForR != 0 {
                            b"state\0" as *const u8 as *const libc::c_char
                        } else {
                            b"state or output\0" as *const u8 as *const libc::c_char
                        }) as PSTR,
                        0 as PSTR,
                    );
                }
            } else {
                TranslateID(pibDum, pfile, szLex.as_mut_ptr(), iEqType as libc::c_int);
            }
        } else if (iType == 0x20 as libc::c_int || iType == 0x8 as libc::c_int)
            && *szLex.as_mut_ptr() as libc::c_int == '#' as i32
        {
            while *(*pibDum).pbufCur as libc::c_int != 0
                && *(*pibDum).pbufCur as libc::c_int != '\n' as i32
            {
                (*pibDum).pbufCur = ((*pibDum).pbufCur).offset(1);
                (*pibDum).pbufCur;
            }
            fprintf(pfile, b"\n\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(
                pfile,
                b"%s\0" as *const u8 as *const libc::c_char,
                szLex.as_mut_ptr(),
            );
        }
        if bDelayCall == 0 {
            bDelayCall = (strcmp(
                b"CalcDelay\0" as *const u8 as *const libc::c_char,
                szLex.as_mut_ptr(),
            ) == 0) as libc::c_int;
            bDelay = (bDelay != 0 || bDelayCall != 0) as libc::c_int;
        }
        fprintf(pfile, b" \0" as *const u8 as *const libc::c_char);
        NextLex(pibDum, szLex.as_mut_ptr(), &mut iType);
        if !(iType != 0) {
            break;
        }
    }
    if bForR != 0 && bForInits != 0 {
        fprintf(pfile, b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(pfile, b";\n\0" as *const u8 as *const libc::c_char);
    };
}
pub unsafe extern "C" fn WriteOneEquation(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut iType: libc::c_long = pInfo as libc::c_long;
    if (*pvm).hType & 0xf000 as libc::c_int as libc::c_ulong != 0 {
        fprintf(pfile, b"\n\0" as *const u8 as *const libc::c_char);
    }
    match iType {
        7 | 4 | 6 | 8 | 9 => {
            if (if !pvm.is_null() {
                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) != 0xa0000 as libc::c_int as libc::c_ulong
            {
                fprintf(
                    pfile,
                    b"  %s = \0" as *const u8 as *const libc::c_char,
                    GetName(
                        pvm,
                        b"rgModelVars\0" as *const u8 as *const libc::c_char as PSTR,
                        b"rgDerivs\0" as *const u8 as *const libc::c_char as PSTR,
                        0 as libc::c_int as HANDLE,
                    ),
                );
            }
        }
        5 | _ => {
            if (if !pvm.is_null() {
                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) == 0x20000 as libc::c_int as libc::c_ulong
            {
                printf(
                    b"Error: input '%s' used in Scale context.\n\0" as *const u8
                        as *const libc::c_char,
                    (*pvm).szName,
                );
                exit(0 as libc::c_int);
            }
            if (if !pvm.is_null() {
                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) != 0xa0000 as libc::c_int as libc::c_ulong
            {
                if bForR != 0 && bForInits != 0
                    && (if !pvm.is_null() {
                        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }) == 0x10000 as libc::c_int as libc::c_ulong
                {
                    fprintf(
                        pfile,
                        b"    Y[\"%s\"] <- \0" as *const u8 as *const libc::c_char,
                        GetName(pvm, 0 as PSTR, 0 as PSTR, 0 as libc::c_int as HANDLE),
                    );
                } else {
                    fprintf(
                        pfile,
                        b"  %s = \0" as *const u8 as *const libc::c_char,
                        GetName(pvm, 0 as PSTR, 0 as PSTR, 0 as libc::c_int as HANDLE),
                    );
                }
            }
        }
    }
    if (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) == 0xa0000 as libc::c_int as libc::c_ulong
    {
        fprintf(pfile, b"\n%s\n\0" as *const u8 as *const libc::c_char, (*pvm).szEqn);
    } else {
        TranslateEquation(pfile, (*pvm).szEqn, iType);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn WriteCalcDeriv(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmDyn: PVMMAPSTRCT,
) {
    if pvmDyn.is_null() {
        printf(b"No Dynamics{} equations.\n\n\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        pfile,
        b"/*----- Dynamics section */\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"void CalcDeriv (double  rgModelVars[], \0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"double  rgDerivs[], PDOUBLE pdTime)\n{\n\0" as *const u8 as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x50000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(
        pfile,
        b"\n  CalcInputs (pdTime); /* Get new input vals */\n\n\0" as *const u8
            as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmDyn,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        4 as libc::c_int as PVOID,
    );
    fprintf(pfile, b"\n} /* CalcDeriv */\n\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn WriteScale(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmScale: PVMMAPSTRCT,
) {
    if pvmScale.is_null() {
        printf(
            b"No Scale{} equations. Null function defined.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    fprintf(
        pfile,
        b"/*----- Model scaling */\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"void ScaleModel (PDOUBLE pdTime)\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(pfile, b"{\n\0" as *const u8 as *const libc::c_char);
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x60000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmScale,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        5 as libc::c_int as PVOID,
    );
    fprintf(pfile, b"\n} /* ScaleModel */\n\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn WriteCalcJacob(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmJacob: PVMMAPSTRCT,
) {
    fprintf(
        pfile,
        b"/*----- Jacobian calculations */\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"void CalcJacob (PDOUBLE pdTime, double rgModelVars[],\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"                long column, double rgdJac[])\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"{\n\0" as *const u8 as *const libc::c_char);
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x70000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmJacob,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        6 as libc::c_int as PVOID,
    );
    fprintf(pfile, b"\n} /* CalcJacob */\n\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn WriteCalcOutputs(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmCalcOut: PVMMAPSTRCT,
) {
    if pvmCalcOut.is_null() {
        printf(
            b"No CalcOutputs{} equations. Null function defined.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    fprintf(
        pfile,
        b"/*----- Outputs calculations */\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"void CalcOutputs (double  rgModelVars[], \0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"double  rgDerivs[], PDOUBLE pdTime)\n{\n\0" as *const u8 as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x80000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmCalcOut,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        7 as libc::c_int as PVOID,
    );
    fprintf(
        pfile,
        b"\n}  /* CalcOutputs */\n\n\n\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn IndexOneVar(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    if (*pvm).szEqn != vszHasInitializer.as_mut_ptr() {
        let ref mut fresh0 = *(pInfo as PINT);
        let fresh1 = *fresh0;
        *fresh0 = *fresh0 + 1;
        (*pvm).hType |= fresh1 as libc::c_ulong;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn IndexVariables(mut pvmGlo: PVMMAPSTRCT) {
    let mut iIndex: libc::c_int = 0;
    let mut iMax: libc::c_int = 0xf000 as libc::c_int;
    vnStates = ForAllVar(
        0 as PFILE,
        pvmGlo,
        Some(
            CountOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x10000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    vnOutputs = ForAllVar(
        0 as PFILE,
        pvmGlo,
        Some(
            CountOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x30000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    vnInputs = ForAllVar(
        0 as PFILE,
        pvmGlo,
        Some(
            CountOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x20000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    vnParms = ForAllVar(
        0 as PFILE,
        pvmGlo,
        Some(
            CountOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    vnModelVars = vnStates + vnOutputs;
    if vnStates > 0xf000 as libc::c_int {
        ReportError(
            0 as PINPUTBUF,
            (0x100 as libc::c_int + 9 as libc::c_int) as WORD,
            b"state\0" as *const u8 as *const libc::c_char as PSTR,
            &mut iMax as *mut libc::c_int as PSTR,
        );
    }
    if vnOutputs > 0xf000 as libc::c_int {
        ReportError(
            0 as PINPUTBUF,
            (0x100 as libc::c_int + 9 as libc::c_int) as WORD,
            b"input\0" as *const u8 as *const libc::c_char as PSTR,
            &mut iMax as *mut libc::c_int as PSTR,
        );
    }
    if vnInputs > 0xf000 as libc::c_int {
        ReportError(
            0 as PINPUTBUF,
            (0x100 as libc::c_int + 9 as libc::c_int) as WORD,
            b"output\0" as *const u8 as *const libc::c_char as PSTR,
            &mut iMax as *mut libc::c_int as PSTR,
        );
    }
    iMax = 0xf000 as libc::c_int - vnModelVars;
    if vnParms > iMax {
        ReportError(
            0 as PINPUTBUF,
            (0x100 as libc::c_int + 9 as libc::c_int) as WORD,
            b"parameter\0" as *const u8 as *const libc::c_char as PSTR,
            &mut iMax as *mut libc::c_int as PSTR,
        );
    }
    if vnStates > 0xf000 as libc::c_int || vnInputs > 0xf000 as libc::c_int
        || vnOutputs > 0xf000 as libc::c_int || vnParms > iMax
    {
        ReportError(0 as PINPUTBUF, 0x8000 as libc::c_int as WORD, 0 as PSTR, 0 as PSTR);
    }
    iIndex = 0 as libc::c_int;
    ForAllVar(
        0 as PFILE,
        pvmGlo,
        Some(
            IndexOneVar as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x10000 as libc::c_int as HANDLE,
        &mut iIndex as *mut libc::c_int as PVOID,
    );
    ForAllVar(
        0 as PFILE,
        pvmGlo,
        Some(
            IndexOneVar as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x30000 as libc::c_int as HANDLE,
        &mut iIndex as *mut libc::c_int as PVOID,
    );
    iIndex = 0 as libc::c_int;
    ForAllVar(
        0 as PFILE,
        pvmGlo,
        Some(
            IndexOneVar as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x20000 as libc::c_int as HANDLE,
        &mut iIndex as *mut libc::c_int as PVOID,
    );
    iIndex = vnStates + vnOutputs + vnInputs;
    ForAllVar(
        0 as PFILE,
        pvmGlo,
        Some(
            IndexOneVar as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        &mut iIndex as *mut libc::c_int as PVOID,
    );
}
pub unsafe extern "C" fn AdjustOneVar(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut pifn: PIFN = (*pvm).szEqn as PIFN;
    let mut wOffset: WORD = *(pInfo as PWORD);
    if pifn.is_null() {
        return 1 as libc::c_int;
    }
    if (*pifn).hMag != 0 {
        (*pifn)
            .hMag = ((*pifn).hMag as libc::c_ulong)
            .wrapping_add(wOffset as libc::c_ulong) as HANDLE as HANDLE;
    }
    if (*pifn).hTper != 0 {
        (*pifn)
            .hTper = ((*pifn).hTper as libc::c_ulong)
            .wrapping_add(wOffset as libc::c_ulong) as HANDLE as HANDLE;
    }
    if (*pifn).hT0 != 0 {
        (*pifn)
            .hT0 = ((*pifn).hT0 as libc::c_ulong).wrapping_add(wOffset as libc::c_ulong)
            as HANDLE as HANDLE;
    }
    if (*pifn).hTexp != 0 {
        (*pifn)
            .hTexp = ((*pifn).hTexp as libc::c_ulong)
            .wrapping_add(wOffset as libc::c_ulong) as HANDLE as HANDLE;
    }
    if (*pifn).hDecay != 0 {
        (*pifn)
            .hDecay = ((*pifn).hDecay as libc::c_ulong)
            .wrapping_add(wOffset as libc::c_ulong) as HANDLE as HANDLE;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn AdjustVarHandles(mut pvmGlo: PVMMAPSTRCT) {
    let mut wOffset: WORD = (vnInputs as WORD)
        .wrapping_add(vnStates as libc::c_uint)
        .wrapping_add(vnOutputs as libc::c_uint);
    ForAllVar(
        0 as PFILE,
        pvmGlo,
        Some(
            AdjustOneVar
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x20000 as libc::c_int as HANDLE,
        &mut wOffset as *mut WORD as PVOID,
    );
}
pub unsafe extern "C" fn ReversePointers(mut ppvm: *mut PVMMAPSTRCT) {
    let mut pvmPrev: PVMMAPSTRCT = 0 as *mut tagVM;
    let mut pvmNext: PVMMAPSTRCT = 0 as *mut tagVM;
    if ppvm.is_null() || (*ppvm).is_null() || ((**ppvm).pvmNextVar).is_null() {
        return;
    }
    pvmPrev = 0 as PVMMAPSTRCT;
    loop {
        pvmNext = (**ppvm).pvmNextVar;
        if pvmNext.is_null() {
            break;
        }
        (**ppvm).pvmNextVar = pvmPrev;
        pvmPrev = *ppvm;
        *ppvm = pvmNext;
    }
    (**ppvm).pvmNextVar = pvmPrev;
}
pub unsafe extern "C" fn AssertExistsEqn(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut iReturn: libc::c_int = 0 as libc::c_int;
    let mut pvmDyn: PVMMAPSTRCT = pInfo as PVMMAPSTRCT;
    if (*pvm).szEqn != vszHasInitializer.as_mut_ptr() {
        if !pvmDyn.is_null() {
            iReturn = (GetVarPTR(pvmDyn, (*pvm).szName)
                != 0 as *mut libc::c_void as PVMMAPSTRCT) as libc::c_int;
            if iReturn == 0 {
                ReportError(
                    0 as PINPUTBUF,
                    (0x100 as libc::c_int + 7 as libc::c_int) as WORD,
                    (*pvm).szName,
                    0 as PSTR,
                );
            }
        } else {
            iReturn = ((*pvm).szEqn != 0 as *mut libc::c_void as PSTR) as libc::c_int;
            if iReturn == 0 {
                ReportError(
                    0 as PINPUTBUF,
                    (0x100 as libc::c_int + 8 as libc::c_int) as WORD,
                    (*pvm).szName,
                    0 as PSTR,
                );
            }
        }
    }
    return if iReturn != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub unsafe extern "C" fn VerifyEqns(mut pvmGlo: PVMMAPSTRCT, mut pvmDyn: PVMMAPSTRCT) {
    let mut bStatesOK: BOOL = 0;
    bStatesOK = (vnStates
        == ForAllVar(
            0 as PFILE,
            pvmGlo,
            Some(
                AssertExistsEqn
                    as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
            ),
            0x10000 as libc::c_int as HANDLE,
            pvmDyn as PVOID,
        )) as libc::c_int;
    if bStatesOK == 0 {
        ReportError(
            0 as PINPUTBUF,
            0x8000 as libc::c_int as WORD,
            0 as PSTR,
            b"State equations missing.\n\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
}
pub unsafe extern "C" fn AssertExistsOutputEqn(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut iReturn: libc::c_int = 0 as libc::c_int;
    let mut pinfo: PINPUTINFO = pInfo as PINPUTINFO;
    let mut pvmDyn: PVMMAPSTRCT = (*pinfo).pvmDynEqns;
    let mut pvmOut: PVMMAPSTRCT = (*pinfo).pvmCalcOutEqns;
    if (*pvm).szEqn != vszHasInitializer.as_mut_ptr() {
        if (GetVarPTR(pvmDyn, (*pvm).szName)).is_null()
            && (GetVarPTR(pvmOut, (*pvm).szName)).is_null()
        {
            ReportError(
                0 as PINPUTBUF,
                (0x100 as libc::c_int + 13 as libc::c_int) as WORD,
                (*pvm).szName,
                0 as PSTR,
            );
            iReturn = 0 as libc::c_int;
        } else {
            iReturn = 1 as libc::c_int;
        }
    }
    return if iReturn != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub unsafe extern "C" fn VerifyOutputEqns(mut pInfo: PINPUTINFO) {
    let mut bOutputsOK: BOOL = 0;
    bOutputsOK = (vnOutputs
        == ForAllVar(
            0 as PFILE,
            (*pInfo).pvmGloVars,
            Some(
                AssertExistsOutputEqn
                    as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
            ),
            0x30000 as libc::c_int as HANDLE,
            pInfo as PVOID,
        )) as libc::c_int;
    if bOutputsOK == 0 {
        ReportError(
            0 as PINPUTBUF,
            0x8000 as libc::c_int as WORD,
            0 as PSTR,
            b"Output equations missing.\n\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
}
pub unsafe extern "C" fn WriteModel(mut pinfo: PINPUTINFO, mut szFileOut: PSTR) {
    let mut pfile: PFILE = 0 as *mut FILE;
    if ((*pinfo).pvmGloVars).is_null()
        || ((*pinfo).pvmDynEqns).is_null() && ((*pinfo).pvmCalcOutEqns).is_null()
    {
        printf(
            b"Error: No Dynamics, no outputs or no global variables defined\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    ReversePointers(&mut (*pinfo).pvmGloVars);
    ReversePointers(&mut (*pinfo).pvmDynEqns);
    ReversePointers(&mut (*pinfo).pvmScaleEqns);
    ReversePointers(&mut (*pinfo).pvmCalcOutEqns);
    ReversePointers(&mut (*pinfo).pvmJacobEqns);
    vpvmGloVarList = (*pinfo).pvmGloVars;
    IndexVariables((*pinfo).pvmGloVars);
    AdjustVarHandles((*pinfo).pvmGloVars);
    VerifyEqns((*pinfo).pvmGloVars, (*pinfo).pvmDynEqns);
    VerifyOutputEqns(pinfo);
    pfile = fopen(
        szFileOut as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if !pfile.is_null() {
        vszModelFilename = (*pinfo).szInputFilename;
        vszModGenName = (*pinfo).szModGenName;
        WriteHeader(pfile, szFileOut, (*pinfo).pvmGloVars);
        WriteIncludes(pfile);
        WriteDecls(pfile, (*pinfo).pvmGloVars);
        WritebDelays(pfile, (*pinfo).bDelays);
        WriteVarMap(pfile, (*pinfo).pvmGloVars);
        WriteInitModel(pfile, (*pinfo).pvmGloVars);
        WriteCalcDeriv(pfile, (*pinfo).pvmGloVars, (*pinfo).pvmDynEqns);
        WriteScale(pfile, (*pinfo).pvmGloVars, (*pinfo).pvmScaleEqns);
        WriteCalcJacob(pfile, (*pinfo).pvmGloVars, (*pinfo).pvmJacobEqns);
        WriteCalcOutputs(pfile, (*pinfo).pvmGloVars, (*pinfo).pvmCalcOutEqns);
        fclose(pfile);
        printf(
            b"\n* Created model file '%s'.\n\n\0" as *const u8 as *const libc::c_char,
            szFileOut,
        );
    } else {
        ReportError(
            0 as PINPUTBUF,
            (0x3 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szFileOut,
            b"...in WriteModel ()\0" as *const u8 as *const libc::c_char as PSTR,
        );
    };
}
pub unsafe extern "C" fn WriteOne_R_SODefine(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    static mut iStates: libc::c_long = 0 as libc::c_int as libc::c_long;
    static mut iOutputs: libc::c_long = 0 as libc::c_int as libc::c_long;
    if (*pvm).szEqn != vszHasInitializer.as_mut_ptr() {
        fprintf(pfile, b"#define \0" as *const u8 as *const libc::c_char);
        fprintf(pfile, b"ID_%s\0" as *const u8 as *const libc::c_char, (*pvm).szName);
        if (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) == 0x10000 as libc::c_int as libc::c_ulong
        {
            fprintf(pfile, b" 0x%05lx\n\0" as *const u8 as *const libc::c_char, iStates);
            iStates = iStates + 1 as libc::c_int as libc::c_long;
        } else {
            fprintf(
                pfile,
                b" 0x%05lx\n\0" as *const u8 as *const libc::c_char,
                iOutputs,
            );
            iOutputs = iOutputs + 1 as libc::c_int as libc::c_long;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Write_R_Scale(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmScale: PVMMAPSTRCT,
) {
    fprintf(
        pfile,
        b"void getParms (double *inParms, double *out, int *nout) {\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"/*----- Model scaling */\n\n\0" as *const u8 as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x60000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b"  int i;\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"  for (i = 0; i < *nout; i++) {\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"    parms[i] = inParms[i];\n  }\n\n\0" as *const u8 as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmScale,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        5 as libc::c_int as PVOID,
    );
    fprintf(
        pfile,
        b"\n  for (i = 0; i < *nout; i++) {\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"    out[i] = parms[i];\n  }\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(pfile, b"  }\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn Write_R_State_Scale(
    mut pfile: PFILE,
    mut pvmScale: PVMMAPSTRCT,
) {
    ForAllVar(
        pfile,
        pvmScale,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x10000 as libc::c_int as HANDLE,
        5 as libc::c_int as PVOID,
    );
    ForAllVar(
        pfile,
        pvmScale,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0xa0000 as libc::c_int as HANDLE,
        5 as libc::c_int as PVOID,
    );
}
pub unsafe extern "C" fn Write_R_CalcDeriv(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmDyn: PVMMAPSTRCT,
    mut pvmCalcOut: PVMMAPSTRCT,
) {
    if pvmDyn.is_null() {
        printf(b"No Dynamics{} equations.\n\n\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        pfile,
        b"/*----- Dynamics section */\n\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"void derivs (int *neq, double *pdTime, double *y, \0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"double *ydot, double *yout, int *ip)\n{\n\0" as *const u8
            as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x50000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x80000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmDyn,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        4 as libc::c_int as PVOID,
    );
    ForAllVar(
        pfile,
        pvmCalcOut,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        7 as libc::c_int as PVOID,
    );
    fprintf(pfile, b"\n} /* derivs */\n\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn Write_R_InitModel(mut pfile: PFILE, mut pvmGlo: PVMMAPSTRCT) {
    fprintf(pfile, b"/*----- Initializers */\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"void initmod (void (* odeparms)(int *, double *))\n{\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"  int N=%d;\n\0" as *const u8 as *const libc::c_char, vnParms);
    fprintf(pfile, b"  odeparms(&N, parms);\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"}\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"void initforc (void (* odeforcs)(int *, double *))\n{\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"  int N=%d;\n\0" as *const u8 as *const libc::c_char, vnInputs);
    fprintf(pfile, b"  odeforcs(&N, forc);\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"}\n\n\n\0" as *const u8 as *const libc::c_char);
    if bDelay != 0 {
        fprintf(
            pfile,
            b"/* Calling R code will ensure that input y has same\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            pfile,
            b"   dimension as yini */\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            pfile,
            b"void initState (double *y)\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(pfile, b"{\n\0" as *const u8 as *const libc::c_char);
        fprintf(pfile, b"  int i;\n\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            pfile,
            b"  for (i = 0; i < sizeof(yini) / sizeof(yini[0]); i++)\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(pfile, b"  {\n\0" as *const u8 as *const libc::c_char);
        fprintf(pfile, b"    yini[i] = y[i];\n\0" as *const u8 as *const libc::c_char);
        fprintf(pfile, b"  }\n}\n\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn Write_R_CalcJacob(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmJacob: PVMMAPSTRCT,
) {
    fprintf(
        pfile,
        b"/*----- Jacobian calculations: */\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"void jac (int *neq, double *t, double *y, int *ml, \0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"int *mu, \0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"double *pd, int *nrowpd, double *yout, int *ip)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"{\n\0" as *const u8 as *const libc::c_char);
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x70000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmJacob,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        6 as libc::c_int as PVOID,
    );
    fprintf(pfile, b"\n} /* jac */\n\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn Write_R_Events(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmEvents: PVMMAPSTRCT,
) {
    fprintf(
        pfile,
        b"/*----- Events calculations: */\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"void event (int *n, double *t, double *y)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"{\n\0" as *const u8 as *const libc::c_char);
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x70300 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmEvents,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        8 as libc::c_int as PVOID,
    );
    fprintf(pfile, b"\n} /* event */\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn Write_R_Roots(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmRoots: PVMMAPSTRCT,
) {
    fprintf(
        pfile,
        b"/*----- Roots calculations: */\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"void root (int *neq, double *t, double *y, \0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"int *ng, double *gout, double *out, int *ip)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"{\n\0" as *const u8 as *const libc::c_char);
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOneDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x70600 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    ForAllVar(
        pfile,
        pvmRoots,
        Some(
            WriteOneEquation
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0 as libc::c_int as HANDLE,
        9 as libc::c_int as PVOID,
    );
    fprintf(pfile, b"\n} /* root */\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn WriteOne_R_PIDefine(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    static mut iParms: libc::c_long = 0 as libc::c_int as libc::c_long;
    static mut iForcs: libc::c_long = 0 as libc::c_int as libc::c_long;
    if (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) != 0x30000 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"TYPE(pvm) != ID_OUTPUT\0" as *const u8 as *const libc::c_char,
            b"modo.c\0" as *const u8 as *const libc::c_char,
            1324 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"int WriteOne_R_PIDefine(PFILE, PVMMAPSTRCT, PVOID)\0"))
                .as_ptr(),
        );
    }
    'c_8176: {
        if (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) != 0x30000 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"TYPE(pvm) != ID_OUTPUT\0" as *const u8 as *const libc::c_char,
                b"modo.c\0" as *const u8 as *const libc::c_char,
                1324 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"int WriteOne_R_PIDefine(PFILE, PVMMAPSTRCT, PVOID)\0"))
                    .as_ptr(),
            );
        }
    };
    if (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) != 0x10000 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"TYPE(pvm) != ID_STATE\0" as *const u8 as *const libc::c_char,
            b"modo.c\0" as *const u8 as *const libc::c_char,
            1325 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"int WriteOne_R_PIDefine(PFILE, PVMMAPSTRCT, PVOID)\0"))
                .as_ptr(),
        );
    }
    'c_8114: {
        if (if !pvm.is_null() {
            (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
        } else {
            0 as libc::c_int as libc::c_ulong
        }) != 0x10000 as libc::c_int as libc::c_ulong
        {} else {
            __assert_fail(
                b"TYPE(pvm) != ID_STATE\0" as *const u8 as *const libc::c_char,
                b"modo.c\0" as *const u8 as *const libc::c_char,
                1325 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"int WriteOne_R_PIDefine(PFILE, PVMMAPSTRCT, PVOID)\0"))
                    .as_ptr(),
            );
        }
    };
    if (if !pvm.is_null() {
        (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    }) == 0x40000 as libc::c_int as libc::c_ulong
    {
        fprintf(
            pfile,
            b"#define %s parms[%ld]\n\0" as *const u8 as *const libc::c_char,
            (*pvm).szName,
            iParms,
        );
        iParms = iParms + 1 as libc::c_int as libc::c_long;
    } else {
        fprintf(
            pfile,
            b"#define %s forc[%ld]\n\0" as *const u8 as *const libc::c_char,
            (*pvm).szName,
            iForcs,
        );
        iForcs = iForcs + 1 as libc::c_int as libc::c_long;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn ForAllVarwSep(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pfiFunc: PFI_CALLBACK,
    mut hType: HANDLE,
    mut pinfo: PVOID,
) -> libc::c_int {
    let mut iTotal: libc::c_int = 0 as libc::c_int;
    let mut End: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    let mut iCount: libc::c_int = 0 as libc::c_int;
    while !pvm.is_null() {
        if hType == 0 as libc::c_int as libc::c_ulong
            || (if !pvm.is_null() {
                (*pvm).hType & 0xf0000 as libc::c_int as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) == hType
        {
            if (*pvm).szEqn != vszHasInitializer.as_mut_ptr() {
                if pfiFunc.is_some() {
                    if iCount > 0 as libc::c_int {
                        End = 0 as libc::c_int as libc::c_long;
                    }
                    iTotal
                        += (Some(pfiFunc.unwrap())).unwrap()(pfile, pvm, End as PVOID);
                    iCount += 1;
                    iCount;
                } else {
                    iTotal += 1;
                    iTotal;
                }
            }
        }
        pvm = (*pvm).pvmNextVar;
    }
    End = 1 as libc::c_int as libc::c_long;
    (Some(pfiFunc.unwrap())).unwrap()(pfile, pvm, End as PVOID);
    return iTotal;
}
pub unsafe extern "C" fn Is_numeric(mut str: PSTR) -> libc::c_int {
    let mut val: libc::c_double = 0.;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !str.is_null() {
        val = strtod(str, &mut ptr);
        if strlen(ptr) > 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int
        } else {
            return 1 as libc::c_int
        }
    } else {
        return 2 as libc::c_int
    };
}
pub unsafe extern "C" fn WriteOne_R_PSDecl(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut szVarName: PSTR = 0 as *mut libc::c_char;
    let mut szZero: PSTR = b"0.0\0" as *const u8 as *const libc::c_char as PSTR;
    let mut End: libc::c_long = pInfo as libc::c_long;
    let mut RHS: PSTR = 0 as *mut libc::c_char;
    let mut iOut: libc::c_int = 0;
    if End < 1 as libc::c_int as libc::c_long {
        szVarName = GetName(pvm, 0 as PSTR, 0 as PSTR, 0 as libc::c_int as HANDLE);
        iOut = Is_numeric((*pvm).szEqn);
        match iOut {
            0 => {
                RHS = szZero;
            }
            1 => {
                RHS = (*pvm).szEqn;
            }
            2 => {
                RHS = szZero;
            }
            _ => {}
        }
    }
    match End {
        -1 => {
            fprintf(
                pfile,
                b"    %s = %s\0" as *const u8 as *const libc::c_char,
                szVarName,
                RHS,
            );
        }
        0 => {
            fprintf(
                pfile,
                b",\n    %s = %s\0" as *const u8 as *const libc::c_char,
                szVarName,
                RHS,
            );
        }
        1 => {
            fprintf(pfile, b"\n\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn WriteOne_R_ParmInit(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut szVarName: PSTR = 0 as *mut libc::c_char;
    let mut iOut: libc::c_int = 0;
    if (pInfo as libc::c_long) < 1 as libc::c_int as libc::c_long {
        szVarName = GetName(pvm, 0 as PSTR, 0 as PSTR, 0 as libc::c_int as HANDLE);
        iOut = Is_numeric((*pvm).szEqn);
        if iOut == 0 as libc::c_int {
            fprintf(
                pfile,
                b"    %s = %s;\n\0" as *const u8 as *const libc::c_char,
                szVarName,
                (*pvm).szEqn,
            );
        }
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn WriteOne_R_StateInit(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut szVarName: PSTR = 0 as *mut libc::c_char;
    let mut End: libc::c_long = pInfo as libc::c_long;
    if End < 1 as libc::c_int as libc::c_long {
        szVarName = GetName(pvm, 0 as PSTR, 0 as PSTR, 0 as libc::c_int as HANDLE);
    }
    match End {
        -1 => {
            fprintf(
                pfile,
                b"    %s = %s\0" as *const u8 as *const libc::c_char,
                szVarName,
                if !((*pvm).szEqn).is_null() {
                    (*pvm).szEqn as *const libc::c_char
                } else {
                    b"0.0\0" as *const u8 as *const libc::c_char
                },
            );
        }
        0 => {
            fprintf(
                pfile,
                b",\n    %s = %s\0" as *const u8 as *const libc::c_char,
                szVarName,
                if !((*pvm).szEqn).is_null() {
                    (*pvm).szEqn as *const libc::c_char
                } else {
                    b"0.0\0" as *const u8 as *const libc::c_char
                },
            );
        }
        1 => {
            fprintf(pfile, b"\n\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn WriteOneOutputName(
    mut pfile: PFILE,
    mut pvm: PVMMAPSTRCT,
    mut pInfo: PVOID,
) -> libc::c_int {
    let mut END: libc::c_long = pInfo as libc::c_long;
    match END {
        -1 => {
            fprintf(
                pfile,
                b"    \"%s\"\0" as *const u8 as *const libc::c_char,
                (*pvm).szName,
            );
        }
        0 => {
            fprintf(
                pfile,
                b",\n    \"%s\"\0" as *const u8 as *const libc::c_char,
                (*pvm).szName,
            );
        }
        1 => {
            fprintf(pfile, b"\n\0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Write_R_InitPOS(
    mut pfile: PFILE,
    mut pvmGlo: PVMMAPSTRCT,
    mut pvmScale: PVMMAPSTRCT,
) {
    fprintf(
        pfile,
        b"initParms <- function(newParms = NULL) {\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"  parms <- c(\n\0" as *const u8 as *const libc::c_char);
    ForAllVarwSep(
        pfile,
        pvmGlo,
        Some(
            WriteOne_R_PSDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b"  )\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"  if (!is.null(newParms)) {\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"    if (!all(names(newParms) %%in%% c(names(parms)))) {\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"      stop(\"illegal parameter name\")\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(pfile, b"    }\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"    parms[names(newParms)] <- newParms\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(pfile, b"  }\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"  parms <- within(as.list(parms), {\n\0" as *const u8 as *const libc::c_char,
    );
    ForAllVarwSep(
        pfile,
        pvmGlo,
        Some(
            WriteOne_R_ParmInit
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b"  })\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"  out <- .C(\"getParms\",  as.double(parms),\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"            out=double(length(parms)),\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"            as.integer(length(parms)))$out\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"  names(out) <- names(parms)\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(pfile, b"  out\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"}\n\n\0" as *const u8 as *const libc::c_char);
    fprintf(pfile, b"Outputs <- c(\n\0" as *const u8 as *const libc::c_char);
    ForAllVarwSep(
        pfile,
        pvmGlo,
        Some(
            WriteOneOutputName
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x30000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b")\n\n\0" as *const u8 as *const libc::c_char);
    bForInits = 1 as libc::c_int;
    fprintf(
        pfile,
        b"initStates <- function(parms, newStates = NULL) {\n  Y <- c(\n\0" as *const u8
            as *const libc::c_char,
    );
    ForAllVarwSep(
        pfile,
        pvmGlo,
        Some(
            WriteOne_R_PSDecl
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x10000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b"  )\n\n\0" as *const u8 as *const libc::c_char);
    if ForAllVar(
        pfile,
        pvmScale,
        None,
        0x10000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    ) != 0
        || ForAllVar(
            pfile,
            pvmScale,
            None,
            0xa0000 as libc::c_int as HANDLE,
            0 as *mut libc::c_void,
        ) != 0
    {
        fprintf(
            pfile,
            b"  Y <- within(c(as.list(parms),as.list(Y)), {\0" as *const u8
                as *const libc::c_char,
        );
        Write_R_State_Scale(pfile, pvmScale);
        fprintf(pfile, b"\n  })$Y\n\n\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        pfile,
        b"  if (!is.null(newStates)) {\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"    if (!all(names(newStates) %%in%% c(names(Y)))) {\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"      stop(\"illegal state variable name in newStates\")\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(pfile, b"    }\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"    Y[names(newStates)] <- newStates\n  }\n\n\0" as *const u8
            as *const libc::c_char,
    );
    if bDelay != 0 {
        fprintf(
            pfile,
            b".C(\"initState\", as.double(Y));\n\0" as *const u8 as *const libc::c_char,
        );
    }
    fprintf(pfile, b"Y\n}\n\0" as *const u8 as *const libc::c_char);
    bForInits = 0 as libc::c_int;
}
pub unsafe extern "C" fn Write_R_Decls(mut pfile: PFILE, mut pvmGlo: PVMMAPSTRCT) {
    fprintf(
        pfile,
        b"\n/* Model variables: States */\n\0" as *const u8 as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOne_R_SODefine
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x10000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(
        pfile,
        b"\n/* Model variables: Outputs */\n\0" as *const u8 as *const libc::c_char,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOne_R_SODefine
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x30000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b"\n/* Parameters */\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        pfile,
        b"static double parms[%d];\n\n\0" as *const u8 as *const libc::c_char,
        vnParms,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOne_R_PIDefine
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x40000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(
        pfile,
        b"\n/* Forcing (Input) functions */\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        pfile,
        b"static double forc[%d];\n\n\0" as *const u8 as *const libc::c_char,
        vnInputs,
    );
    ForAllVar(
        pfile,
        pvmGlo,
        Some(
            WriteOne_R_PIDefine
                as unsafe extern "C" fn(PFILE, PVMMAPSTRCT, PVOID) -> libc::c_int,
        ),
        0x20000 as libc::c_int as HANDLE,
        0 as *mut libc::c_void,
    );
    fprintf(pfile, b"\n\0" as *const u8 as *const libc::c_char);
    if bDelay != 0 {
        fprintf(
            pfile,
            b"/* Function definitions for delay differential equations */\n\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(pfile, b"int Nout=1;\n\0" as *const u8 as *const libc::c_char);
        fprintf(pfile, b"int nr[1]={0};\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            pfile,
            b"double ytau[1] = {0.0};\n\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            pfile,
            b"static double yini[%d] = {\0" as *const u8 as *const libc::c_char,
            vnStates,
        );
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while i <= vnStates {
            if i == vnStates {
                fprintf(pfile, b"0.0\0" as *const u8 as *const libc::c_char);
            } else {
                fprintf(pfile, b"0.0, \0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            i;
        }
        fprintf(
            pfile,
            b"}; /*Array of initial state variables*/\n\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            pfile,
            b"void lagvalue(double T, int *nr, int N, double *ytau) {\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            pfile,
            b"  static void(*fun)(double, int*, int, double*) = NULL;\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(pfile, b"  if (fun == NULL)\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            pfile,
            b"    fun = (void(*)(double, int*, int, double*))R_GetCCallable(\"deSolve\", \"lagvalue\");\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            pfile,
            b"  return fun(T, nr, N, ytau);\n}\n\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            pfile,
            b"double CalcDelay(int hvar, double dTime, double delay) {\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            pfile,
            b"  double T = dTime-delay;\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(pfile, b"  if (dTime > delay){\n\0" as *const u8 as *const libc::c_char);
        fprintf(pfile, b"    nr[0] = hvar;\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            pfile,
            b"    lagvalue( T, nr, Nout, ytau );\n}\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(pfile, b"  else{\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            pfile,
            b"    ytau[0] = yini[hvar];\n}\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            pfile,
            b"  return(ytau[0]);\n}\n\n\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn Write_R_Includes(mut pfile: PFILE) {
    fprintf(pfile, b"#include <R.h>\n\0" as *const u8 as *const libc::c_char);
    if bDelay != 0 {
        fprintf(
            pfile,
            b"#include <Rinternals.h>\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(pfile, b"#include <Rdefines.h>\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            pfile,
            b"#include <R_ext/Rdynload.h>\n\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn Write_R_Model(mut pinfo: PINPUTINFO, mut szFileOut: PSTR) {
    static mut vszModified_Title: PSTRLEX = [0; 1023];
    let mut pfile: PFILE = 0 as *mut FILE;
    let mut Rfile: PSTR = 0 as *mut libc::c_char;
    let mut Rappend: PSTR = b"_inits.R\0" as *const u8 as *const libc::c_char as PSTR;
    let mut nRout: size_t = 0;
    let mut nbase: size_t = 0;
    let mut lastdot: *mut libc::c_char = 0 as *mut libc::c_char;
    bForR = 1 as libc::c_int;
    if ((*pinfo).pvmGloVars).is_null()
        || ((*pinfo).pvmDynEqns).is_null() && ((*pinfo).pvmCalcOutEqns).is_null()
    {
        printf(
            b"Error: No Dynamics, outputs or global variables defined\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    ReversePointers(&mut (*pinfo).pvmGloVars);
    ReversePointers(&mut (*pinfo).pvmDynEqns);
    ReversePointers(&mut (*pinfo).pvmScaleEqns);
    ReversePointers(&mut (*pinfo).pvmCalcOutEqns);
    ReversePointers(&mut (*pinfo).pvmJacobEqns);
    ReversePointers(&mut (*pinfo).pvmEventEqns);
    ReversePointers(&mut (*pinfo).pvmRootEqns);
    vpvmGloVarList = (*pinfo).pvmGloVars;
    IndexVariables((*pinfo).pvmGloVars);
    AdjustVarHandles((*pinfo).pvmGloVars);
    VerifyEqns((*pinfo).pvmGloVars, (*pinfo).pvmDynEqns);
    VerifyOutputEqns(pinfo);
    pfile = fopen(
        szFileOut as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if !pfile.is_null() {
        vszModelFilename = (*pinfo).szInputFilename;
        vszModGenName = (*pinfo).szModGenName;
        sprintf(
            vszModified_Title.as_mut_ptr(),
            b"%s %s\0" as *const u8 as *const libc::c_char,
            szFileOut,
            b"for R deSolve package\0" as *const u8 as *const libc::c_char,
        );
        WriteHeader(pfile, vszModified_Title.as_mut_ptr(), (*pinfo).pvmGloVars);
        Write_R_Includes(pfile);
        Write_R_Decls(pfile, (*pinfo).pvmGloVars);
        Write_R_InitModel(pfile, (*pinfo).pvmGloVars);
        Write_R_Scale(pfile, (*pinfo).pvmGloVars, (*pinfo).pvmScaleEqns);
        Write_R_CalcDeriv(
            pfile,
            (*pinfo).pvmGloVars,
            (*pinfo).pvmDynEqns,
            (*pinfo).pvmCalcOutEqns,
        );
        Write_R_CalcJacob(pfile, (*pinfo).pvmGloVars, (*pinfo).pvmJacobEqns);
        Write_R_Events(pfile, (*pinfo).pvmGloVars, (*pinfo).pvmEventEqns);
        Write_R_Roots(pfile, (*pinfo).pvmGloVars, (*pinfo).pvmRootEqns);
        fclose(pfile);
        printf(
            b"\n* Created C model file '%s'.\n\n\0" as *const u8 as *const libc::c_char,
            szFileOut,
        );
    } else {
        ReportError(
            0 as PINPUTBUF,
            (0x3 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            szFileOut,
            b"in Write_R_Model ()\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
    lastdot = strrchr(szFileOut as *const libc::c_char, '.' as i32);
    if !lastdot.is_null() {
        *lastdot = '\0' as i32 as libc::c_char;
    }
    nbase = strlen(szFileOut as *const libc::c_char);
    nRout = nbase
        .wrapping_add(strlen(Rappend as *const libc::c_char))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    Rfile = malloc(nRout) as PSTR;
    Rfile = strncpy(Rfile, szFileOut as *const libc::c_char, nbase);
    *Rfile.offset(nbase as isize) = '\0' as i32 as libc::c_char;
    Rfile = strcat(Rfile, Rappend as *const libc::c_char);
    pfile = fopen(
        Rfile as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if !pfile.is_null() {
        Write_R_InitPOS(pfile, (*pinfo).pvmGloVars, (*pinfo).pvmScaleEqns);
        fclose(pfile);
        printf(
            b"\n* Created R parameter initialization file '%s'.\n\n\0" as *const u8
                as *const libc::c_char,
            Rfile,
        );
    } else {
        ReportError(
            0 as PINPUTBUF,
            (0x3 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            Rfile,
            b"in Write_R_Model ()\0" as *const u8 as *const libc::c_char as PSTR,
        );
    }
    free(Rfile as *mut libc::c_void);
}
