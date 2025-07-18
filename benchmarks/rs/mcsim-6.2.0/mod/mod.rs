use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    static mut opterr: libc::c_int;
    fn _getopt(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn ReportError(_: PINPUTBUF, _: WORD, _: PSTR, _: PSTR);
    fn ReadModel(pinfo: PINPUTINFO, ptempinfo: PINPUTINFO, szFileIn: PSTR);
    fn WriteModel(pinfo: PINPUTINFO, szFileOut: PSTR);
    fn Write_R_Model(pinfo: PINPUTINFO, szFileOut: PSTR);
    fn MyStrcmp(sz1: *const libc::c_char, sz2: *const libc::c_char) -> libc::c_int;
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
pub type INPUTINFO = tagINPUTINFO;
pub type PINPUTINFO = *mut tagINPUTINFO;
static mut vszOptions: [libc::c_char; 6] = unsafe {
    *::std::mem::transmute::<&[u8; 6], &mut [libc::c_char; 6]>(b"hHDRG\0")
};
static mut vszFilenameDefault: [libc::c_char; 8] = unsafe {
    *::std::mem::transmute::<&[u8; 8], &mut [libc::c_char; 8]>(b"model.c\0")
};
pub static mut szFileWithExt: [libc::c_char; 256] = [0; 256];
pub unsafe extern "C" fn AnnounceProgram() {
    printf(
        b"\n________________________________________\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\nMod v6.2.0 - Model Generator for MCSim\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"MCSim and associated software comes with ABSOLUTELY NO WARRANTY;\nThis is free software, and you are welcome to redistribute it\nunder certain conditions; see the GNU General Public License.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn PromptFilenames(
    mut pszFileIn: *mut PSTR,
    mut pszFileOut: *mut PSTR,
) {
    *pszFileIn = calloc(
        1 as libc::c_int as libc::c_ulong,
        256 as libc::c_int as libc::c_ulong,
    ) as PSTR;
    if (*pszFileIn).is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"PromptFilenames\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    *pszFileOut = calloc(
        1 as libc::c_int as libc::c_ulong,
        256 as libc::c_int as libc::c_ulong,
    ) as PSTR;
    if (*pszFileOut).is_null() {
        ReportError(
            0 as PINPUTBUF,
            (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
            b"PromptFilenames\0" as *const u8 as *const libc::c_char as PSTR,
            0 as PSTR,
        );
    }
    printf(b"Input filename? \0" as *const u8 as *const libc::c_char);
    fgets(*pszFileIn, 256 as libc::c_int, stdin);
    *pszFileIn = strtok(*pszFileIn, b" \t\n\0" as *const u8 as *const libc::c_char);
    if (*pszFileIn).is_null() {
        return;
    }
    if *(*pszFileIn).offset(0 as libc::c_int as isize) != 0 {
        printf(b"Output filename? \0" as *const u8 as *const libc::c_char);
        fgets(*pszFileOut, 256 as libc::c_int, stdin);
        *pszFileOut = strtok(
            *pszFileOut,
            b" \t\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pszFileOut).is_null() || *(*pszFileOut).offset(0 as libc::c_int as isize) == 0 {
        free(*pszFileOut as *mut libc::c_void);
        *pszFileOut = 0 as PSTR;
    } else {
        *pszFileIn = realloc(
            *pszFileIn as *mut libc::c_void,
            (if !(*pszFileIn).is_null() {
                strlen(*pszFileIn as *const libc::c_char)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as PSTR;
        if (*pszFileIn).is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"PromptFilenames\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
        *pszFileOut = realloc(
            *pszFileOut as *mut libc::c_void,
            (if !(*pszFileOut).is_null() {
                strlen(*pszFileOut as *const libc::c_char)
            } else {
                0 as libc::c_int as libc::c_ulong
            })
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as PSTR;
        if (*pszFileOut).is_null() {
            ReportError(
                0 as PINPUTBUF,
                (0x4 as libc::c_int | 0x8000 as libc::c_int) as WORD,
                b"PromptFilenames\0" as *const u8 as *const libc::c_char as PSTR,
                0 as PSTR,
            );
        }
    };
}
pub unsafe extern "C" fn ShowHelp() {
    printf(b"Help:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Usage: mod [options] [input-file [output-file]]\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"Options:\n\0" as *const u8 as *const libc::c_char);
    printf(b"  -h  Display this information\n\0" as *const u8 as *const libc::c_char);
    printf(b"  -H  Display this information\n\0" as *const u8 as *const libc::c_char);
    printf(b"  -D  Debug mode\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  -R  Generate an R deSolve compatible C file\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Creates file 'output-file' (or 'model.c', by default)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"according to the input-file specifications.\n\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn GetCmdLineArgs(
    mut nArg: libc::c_int,
    mut rgszArg: *const *mut libc::c_char,
    mut pszFileIn: *mut PSTR,
    mut pszFileOut: *mut PSTR,
    mut pinfo: PINPUTINFO,
) {
    let mut c: libc::c_int = 0;
    opterr = 1 as libc::c_int;
    *pszFileOut = 0 as *mut libc::c_void as PSTR;
    *pszFileIn = *pszFileOut;
    loop {
        c = _getopt(nArg, rgszArg, vszOptions.as_mut_ptr());
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            68 => {
                printf(
                    b">> Debug mode using option '%s': Not implemented, ignored.\n\n\0"
                        as *const u8 as *const libc::c_char,
                    optarg,
                );
            }
            72 | 104 => {
                ShowHelp();
                exit(0 as libc::c_int);
            }
            82 => {
                printf(
                    b">> Generating code for linking with R deSolve package.\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
                (*pinfo).bforR = 1 as libc::c_int;
            }
            63 | _ => {
                ShowHelp();
                exit(0 as libc::c_int);
            }
        }
    }
    let mut current_block_17: u64;
    match nArg - optind {
        2 => {
            *pszFileOut = *rgszArg.offset((optind + 1 as libc::c_int) as isize);
            current_block_17 = 9331050529421454957;
        }
        1 => {
            current_block_17 = 9331050529421454957;
        }
        0 => {
            PromptFilenames(pszFileIn, pszFileOut);
            current_block_17 = 224731115979188411;
        }
        _ => {
            printf(
                b"mod: too many parameters on command line\n\0" as *const u8
                    as *const libc::c_char,
            );
            ShowHelp();
            exit(-(1 as libc::c_int));
        }
    }
    match current_block_17 {
        9331050529421454957 => {
            *pszFileIn = *rgszArg.offset(optind as isize);
        }
        _ => {}
    }
    while !(*pszFileIn).is_null()
        && *(*pszFileIn).offset(0 as libc::c_int as isize) as libc::c_int != 0
        && MyStrcmp(
            *pszFileIn as *const libc::c_char,
            *pszFileOut as *const libc::c_char,
        ) == 0
    {
        printf(
            b"\n** Input and output filename must be different.\n\0" as *const u8
                as *const libc::c_char,
        );
        PromptFilenames(pszFileIn, pszFileOut);
    }
    if !(!(*pszFileIn).is_null()
        && *(*pszFileIn).offset(0 as libc::c_int as isize) as libc::c_int != 0)
    {
        printf(
            b"Error: an input file name must be specified - Exiting\n\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    (*pinfo).szInputFilename = *pszFileIn;
    if (*pszFileOut).is_null() {
        *pszFileOut = vszFilenameDefault.as_mut_ptr();
    }
}
pub unsafe extern "C" fn InitInfo(mut pinfo: PINPUTINFO, mut szModGenName: PSTR) {
    (*pinfo).wContext = 0x1 as libc::c_int as WORD;
    (*pinfo).bDelays = 0 as libc::c_int;
    (*pinfo).bforR = 0 as libc::c_int;
    (*pinfo).bTemplateInUse = 0 as libc::c_int;
    (*pinfo).szModGenName = szModGenName;
    (*pinfo).pvmGloVars = 0 as PVMMAPSTRCT;
    (*pinfo).pvmDynEqns = 0 as PVMMAPSTRCT;
    (*pinfo).pvmScaleEqns = 0 as PVMMAPSTRCT;
    (*pinfo).pvmJacobEqns = 0 as PVMMAPSTRCT;
    (*pinfo).pvmCalcOutEqns = 0 as PVMMAPSTRCT;
    (*pinfo).pvmEventEqns = 0 as PVMMAPSTRCT;
    (*pinfo).pvmRootEqns = 0 as PVMMAPSTRCT;
    (*pinfo).pvmCpts = 0 as PVMMAPSTRCT;
    (*pinfo).pvmLocalCpts = 0 as PVMMAPSTRCT;
}
pub unsafe extern "C" fn Cleanup(mut pinfo: PINPUTINFO) {
    let mut next: PVMMAPSTRCT = 0 as *mut tagVM;
    while !((*pinfo).pvmGloVars).is_null() {
        next = (*(*pinfo).pvmGloVars).pvmNextVar;
        free((*(*pinfo).pvmGloVars).szName as *mut libc::c_void);
        free((*(*pinfo).pvmGloVars).szEqn as *mut libc::c_void);
        free((*pinfo).pvmGloVars as *mut libc::c_void);
        (*pinfo).pvmGloVars = next;
    }
    while !((*pinfo).pvmDynEqns).is_null() {
        next = (*(*pinfo).pvmDynEqns).pvmNextVar;
        free((*(*pinfo).pvmDynEqns).szName as *mut libc::c_void);
        free((*(*pinfo).pvmDynEqns).szEqn as *mut libc::c_void);
        free((*pinfo).pvmDynEqns as *mut libc::c_void);
        (*pinfo).pvmDynEqns = next;
    }
    while !((*pinfo).pvmScaleEqns).is_null() {
        next = (*(*pinfo).pvmScaleEqns).pvmNextVar;
        free((*(*pinfo).pvmScaleEqns).szName as *mut libc::c_void);
        free((*(*pinfo).pvmScaleEqns).szEqn as *mut libc::c_void);
        free((*pinfo).pvmScaleEqns as *mut libc::c_void);
        (*pinfo).pvmScaleEqns = next;
    }
    while !((*pinfo).pvmJacobEqns).is_null() {
        next = (*(*pinfo).pvmJacobEqns).pvmNextVar;
        free((*(*pinfo).pvmJacobEqns).szName as *mut libc::c_void);
        free((*(*pinfo).pvmJacobEqns).szEqn as *mut libc::c_void);
        free((*pinfo).pvmJacobEqns as *mut libc::c_void);
        (*pinfo).pvmJacobEqns = next;
    }
    while !((*pinfo).pvmCalcOutEqns).is_null() {
        next = (*(*pinfo).pvmCalcOutEqns).pvmNextVar;
        free((*(*pinfo).pvmCalcOutEqns).szName as *mut libc::c_void);
        free((*(*pinfo).pvmCalcOutEqns).szEqn as *mut libc::c_void);
        free((*pinfo).pvmCalcOutEqns as *mut libc::c_void);
        (*pinfo).pvmCalcOutEqns = next;
    }
    while !((*pinfo).pvmEventEqns).is_null() {
        next = (*(*pinfo).pvmEventEqns).pvmNextVar;
        free((*(*pinfo).pvmEventEqns).szName as *mut libc::c_void);
        free((*(*pinfo).pvmEventEqns).szEqn as *mut libc::c_void);
        free((*pinfo).pvmEventEqns as *mut libc::c_void);
        (*pinfo).pvmEventEqns = next;
    }
    while !((*pinfo).pvmRootEqns).is_null() {
        next = (*(*pinfo).pvmRootEqns).pvmNextVar;
        free((*(*pinfo).pvmRootEqns).szName as *mut libc::c_void);
        free((*(*pinfo).pvmRootEqns).szEqn as *mut libc::c_void);
        free((*pinfo).pvmRootEqns as *mut libc::c_void);
        (*pinfo).pvmRootEqns = next;
    }
    while !((*pinfo).pvmCpts).is_null() {
        next = (*(*pinfo).pvmCpts).pvmNextVar;
        free((*(*pinfo).pvmCpts).szName as *mut libc::c_void);
        free((*(*pinfo).pvmCpts).szEqn as *mut libc::c_void);
        free((*pinfo).pvmCpts as *mut libc::c_void);
        (*pinfo).pvmCpts = next;
    }
    while !((*pinfo).pvmLocalCpts).is_null() {
        next = (*(*pinfo).pvmJacobEqns).pvmNextVar;
        free((*(*pinfo).pvmJacobEqns).szName as *mut libc::c_void);
        free((*(*pinfo).pvmJacobEqns).szEqn as *mut libc::c_void);
        free((*pinfo).pvmJacobEqns as *mut libc::c_void);
        (*pinfo).pvmJacobEqns = next;
    }
}
unsafe fn main_0(mut nArg: libc::c_int, mut rgszArg: *mut PSTR) -> libc::c_int {
    let mut info: INPUTINFO = INPUTINFO {
        wContext: 0,
        bDelays: 0,
        bforR: 0,
        bTemplateInUse: 0,
        szInputFilename: 0 as *mut libc::c_char,
        szModGenName: 0 as *mut libc::c_char,
        pvmGloVars: 0 as *mut tagVM,
        pvmDynEqns: 0 as *mut tagVM,
        pvmScaleEqns: 0 as *mut tagVM,
        pvmJacobEqns: 0 as *mut tagVM,
        pvmCalcOutEqns: 0 as *mut tagVM,
        pvmEventEqns: 0 as *mut tagVM,
        pvmRootEqns: 0 as *mut tagVM,
        pvmCpts: 0 as *mut tagVM,
        pvmLocalCpts: 0 as *mut tagVM,
    };
    let mut tempinfo: INPUTINFO = INPUTINFO {
        wContext: 0,
        bDelays: 0,
        bforR: 0,
        bTemplateInUse: 0,
        szInputFilename: 0 as *mut libc::c_char,
        szModGenName: 0 as *mut libc::c_char,
        pvmGloVars: 0 as *mut tagVM,
        pvmDynEqns: 0 as *mut tagVM,
        pvmScaleEqns: 0 as *mut tagVM,
        pvmJacobEqns: 0 as *mut tagVM,
        pvmCalcOutEqns: 0 as *mut tagVM,
        pvmEventEqns: 0 as *mut tagVM,
        pvmRootEqns: 0 as *mut tagVM,
        pvmCpts: 0 as *mut tagVM,
        pvmLocalCpts: 0 as *mut tagVM,
    };
    let mut szFileIn: PSTR = 0 as *mut libc::c_char;
    let mut szFileOut: PSTR = 0 as *mut libc::c_char;
    AnnounceProgram();
    InitInfo(&mut info, *rgszArg.offset(0 as libc::c_int as isize));
    InitInfo(&mut tempinfo, *rgszArg.offset(0 as libc::c_int as isize));
    GetCmdLineArgs(
        nArg,
        rgszArg as *const *mut libc::c_char,
        &mut szFileIn,
        &mut szFileOut,
        &mut info,
    );
    ReadModel(&mut info, &mut tempinfo, szFileIn);
    if info.bforR == 1 as libc::c_int {
        Write_R_Model(&mut info, szFileOut);
    } else {
        WriteModel(&mut info, szFileOut);
    }
    Cleanup(&mut info);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as libc::c_int, args.as_mut_ptr() as *mut PSTR)
                as i32,
        )
    }
}
