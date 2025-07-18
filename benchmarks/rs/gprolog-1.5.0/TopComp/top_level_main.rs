use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut pl_os_argc: libc::c_int;
    static mut pl_os_argv: *mut *mut libc::c_char;
    fn Pl_Start_Prolog(argc: libc::c_int, argv: *mut *mut libc::c_char) -> libc::c_int;
    fn Pl_Stop_Prolog();
    fn Pl_Reset_Prolog();
    fn Pl_Call_Prolog(codep: CodePtr) -> libc::c_int;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    fn Pl_Mk_Proper_List(n: libc::c_int, arg: *mut WamWord) -> WamWord;
    fn Pl_Blt_G_Assign(x: WamWord, y: WamWord);
    fn X0_top_level__a0();
    fn X1_24657865635F636D645F6C696E655F676F616C__a1();
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
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamWordP = *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
unsafe extern "C" fn Mk_Copying_Message(
    mut sub_part: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut buff: [libc::c_char; 256] = [0; 256];
    if !sub_part.is_null() {
        sprintf(
            buff.as_mut_ptr(),
            b"%s (%s) %s\n\0" as *const u8 as *const libc::c_char,
            sub_part,
            b"GNU Prolog\0" as *const u8 as *const libc::c_char,
            b"1.5.0\0" as *const u8 as *const libc::c_char,
        );
    } else {
        sprintf(
            buff.as_mut_ptr(),
            b"%s %s\n\0" as *const u8 as *const libc::c_char,
            b"GNU Prolog\0" as *const u8 as *const libc::c_char,
            b"1.5.0\0" as *const u8 as *const libc::c_char,
        );
    }
    strcat(
        buff.as_mut_ptr(),
        b"Copyright (C) 1999-2025 Daniel Diaz\n\nGNU Prolog comes with ABSOLUTELY NO WARRANTY.\nThis is free software; see the source or the file\nnamed COPYING for copying conditions.\n\0"
            as *const u8 as *const libc::c_char,
    );
    return buff.as_mut_ptr();
}
unsafe extern "C" fn Display_Copying(mut sub_part: *mut libc::c_char) {
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const libc::c_char,
        Mk_Copying_Message(sub_part),
    );
}
unsafe extern "C" fn Main_Wrapper(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut new_argc: libc::c_int = 0 as libc::c_int;
    let mut new_argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut entry_goal: *mut WamWord = 0 as *mut WamWord;
    let mut nb_entry_goal: libc::c_int = 0 as libc::c_int;
    let mut consult_file: *mut WamWord = 0 as *mut WamWord;
    let mut nb_consult_file: libc::c_int = 0 as libc::c_int;
    let mut query_goal: *mut WamWord = 0 as *mut WamWord;
    let mut nb_query_goal: libc::c_int = 0 as libc::c_int;
    let mut word: WamWord = 0;
    Pl_Start_Prolog(argc, argv);
    new_argv = Pl_Malloc_Check(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong) as libc::c_uint,
        b"top_level_main.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        109 as libc::c_int,
    ) as *mut *mut libc::c_char;
    let fresh0 = new_argc;
    new_argc = new_argc + 1;
    let ref mut fresh1 = *new_argv.offset(fresh0 as isize);
    *fresh1 = *argv.offset(0 as libc::c_int as isize);
    consult_file = Pl_Malloc_Check(
        (::std::mem::size_of::<WamWord>() as libc::c_ulong)
            .wrapping_mul(argc as libc::c_ulong) as libc::c_uint,
        b"top_level_main.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        112 as libc::c_int,
    ) as *mut WamWord;
    entry_goal = Pl_Malloc_Check(
        (::std::mem::size_of::<WamWord>() as libc::c_ulong)
            .wrapping_mul(argc as libc::c_ulong) as libc::c_uint,
        b"top_level_main.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        113 as libc::c_int,
    ) as *mut WamWord;
    query_goal = Pl_Malloc_Check(
        (::std::mem::size_of::<WamWord>() as libc::c_ulong)
            .wrapping_mul(argc as libc::c_ulong) as libc::c_uint,
        b"top_level_main.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        114 as libc::c_int,
    ) as *mut WamWord;
    let mut current_block_30: u64;
    i = 1 as libc::c_int;
    while i < argc {
        if **argv.offset(i as isize) as libc::c_int == '-' as i32
            && *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
        {
            if strcmp(
                *argv.offset(i as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                i += 1;
                i;
                break;
            } else {
                if strncmp(
                    *argv.offset(i as isize),
                    b"--version\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
                {
                    Display_Copying(
                        b"Prolog top-Level\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                if strncmp(
                    *argv.offset(i as isize),
                    b"--init-goal\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
                {
                    i += 1;
                    if i >= argc {
                        Pl_Fatal_Error(
                            b"Goal missing after --init-goal option\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    *pl_reg_bank
                        .offset(
                            0 as libc::c_int as isize,
                        ) = (((Pl_Create_Atom(*argv.offset(i as isize)) as PlLong)
                        << 3 as libc::c_int) as libc::c_ulong)
                        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
                    Pl_Call_Prolog(
                        Some(
                            ::std::mem::transmute::<
                                unsafe extern "C" fn() -> (),
                                unsafe extern "C" fn() -> (),
                            >(X1_24657865635F636D645F6C696E655F676F616C__a1),
                        ),
                    );
                    Pl_Reset_Prolog();
                    current_block_30 = 14523784380283086299;
                } else if strncmp(
                    *argv.offset(i as isize),
                    b"--consult-file\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
                {
                    i += 1;
                    if i >= argc {
                        Pl_Fatal_Error(
                            b"File missing after --consult-file option\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    let fresh2 = nb_consult_file;
                    nb_consult_file = nb_consult_file + 1;
                    *consult_file
                        .offset(
                            fresh2 as isize,
                        ) = (((Pl_Create_Atom(*argv.offset(i as isize)) as PlLong)
                        << 3 as libc::c_int) as libc::c_ulong)
                        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
                    current_block_30 = 14523784380283086299;
                } else if strncmp(
                    *argv.offset(i as isize),
                    b"--entry-goal\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
                {
                    i += 1;
                    if i >= argc {
                        Pl_Fatal_Error(
                            b"Goal missing after --entry-goal option\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    let fresh3 = nb_entry_goal;
                    nb_entry_goal = nb_entry_goal + 1;
                    *entry_goal
                        .offset(
                            fresh3 as isize,
                        ) = (((Pl_Create_Atom(*argv.offset(i as isize)) as PlLong)
                        << 3 as libc::c_int) as libc::c_ulong)
                        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
                    current_block_30 = 14523784380283086299;
                } else if strncmp(
                    *argv.offset(i as isize),
                    b"--query-goal\0" as *const u8 as *const libc::c_char,
                    strlen(*argv.offset(i as isize)),
                ) == 0 as libc::c_int
                {
                    i += 1;
                    if i >= argc {
                        Pl_Fatal_Error(
                            b"Goal missing after --query-goal option\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    let fresh4 = nb_query_goal;
                    nb_query_goal = nb_query_goal + 1;
                    *query_goal
                        .offset(
                            fresh4 as isize,
                        ) = (((Pl_Create_Atom(*argv.offset(i as isize)) as PlLong)
                        << 3 as libc::c_int) as libc::c_ulong)
                        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
                    current_block_30 = 14523784380283086299;
                } else {
                    if strncmp(
                        *argv.offset(i as isize),
                        b"-h\0" as *const u8 as *const libc::c_char,
                        strlen(*argv.offset(i as isize)),
                    ) == 0 as libc::c_int
                        || strncmp(
                            *argv.offset(i as isize),
                            b"--help\0" as *const u8 as *const libc::c_char,
                            strlen(*argv.offset(i as isize)),
                        ) == 0 as libc::c_int
                    {
                        Display_Help();
                        exit(0 as libc::c_int);
                    }
                    current_block_30 = 8704759739624374314;
                }
            }
        } else {
            current_block_30 = 8704759739624374314;
        }
        match current_block_30 {
            8704759739624374314 => {
                let fresh5 = new_argc;
                new_argc = new_argc + 1;
                let ref mut fresh6 = *new_argv.offset(fresh5 as isize);
                *fresh6 = *argv.offset(i as isize);
            }
            _ => {}
        }
        i += 1;
        i;
    }
    while i < argc {
        let fresh7 = i;
        i = i + 1;
        let fresh8 = new_argc;
        new_argc = new_argc + 1;
        let ref mut fresh9 = *new_argv.offset(fresh8 as isize);
        *fresh9 = *argv.offset(fresh7 as isize);
    }
    let ref mut fresh10 = *new_argv.offset(new_argc as isize);
    *fresh10 = 0 as *mut libc::c_char;
    pl_os_argc = new_argc;
    pl_os_argv = new_argv;
    if nb_consult_file != 0 {
        word = Pl_Mk_Proper_List(nb_consult_file, consult_file);
        Pl_Blt_G_Assign(
            (((Pl_Create_Atom(
                b"$cmd_line_consult_file\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord,
            word,
        );
    }
    free(consult_file as *mut libc::c_void);
    if nb_entry_goal != 0 {
        word = Pl_Mk_Proper_List(nb_entry_goal, entry_goal);
        Pl_Blt_G_Assign(
            (((Pl_Create_Atom(
                b"$cmd_line_entry_goal\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord,
            word,
        );
    }
    free(entry_goal as *mut libc::c_void);
    if nb_query_goal != 0 {
        word = Pl_Mk_Proper_List(nb_query_goal, query_goal);
        Pl_Blt_G_Assign(
            (((Pl_Create_Atom(
                b"$cmd_line_query_goal\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord,
            word,
        );
    }
    free(query_goal as *mut libc::c_void);
    Pl_Reset_Prolog();
    Pl_Call_Prolog(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X0_top_level__a0),
        ),
    );
    Pl_Stop_Prolog();
    return 0 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    return Main_Wrapper(argc, argv);
}
unsafe extern "C" fn Display_Help() {
    fprintf(
        stderr,
        b"Usage: %s [OPTION]... \n\0" as *const u8 as *const libc::c_char,
        b"gprolog\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --consult-file FILE         consult FILE inside the the top-level\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --init-goal    GOAL         execute GOAL before entering the top-level\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --entry-goal   GOAL         execute GOAL inside the top-level\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --query-goal   GOAL         execute GOAL as a query for the top-level\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  -h, --help                  print this help and exit\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --version                   print version number and exit\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"  --                          do not parse the rest of the command-line\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"Report bugs to bug-prolog@gnu.org.\0" as *const u8 as *const libc::c_char,
    );
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
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
