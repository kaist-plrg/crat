use ::libc;
use ::c2rust_bitfields;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn setlinebuf(__stream: *mut FILE);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: libc::c_int) -> libc::c_int;
    fn siglongjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn Pl_Find_Linked_Objects();
    fn Pl_Fd_Init_Solver();
    fn Pl_Fd_Reset_Solver();
    fn Pl_Dummy_Ptr(p: *mut libc::c_void) -> *mut libc::c_void;
    fn Pl_Init_Oper();
    fn Pl_Init_Pred();
    fn Pl_Init_Atom();
    fn Pl_Allocate_Stacks();
    static mut pl_max_atom: PlULong;
    static mut pl_fd_init_solver: Option::<unsafe extern "C" fn() -> ()>;
    fn Pl_Init_Machine();
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn Pl_Call_Compiled(codep: CodePtr);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type sigjmp_buf = [__jmp_buf_tag; 1];
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamCont = CodePtr;
pub type WamWordP = *mut WamWord;
pub type TypTag = libc::c_uint;
pub const ADDRESS: TypTag = 2;
pub const SHORT_UNS: TypTag = 1;
pub const LONG_INT: TypTag = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfTag {
    pub name: *mut libc::c_char,
    pub type_0: TypTag,
    pub value: libc::c_int,
    pub tag_mask: PlLong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfStack {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub env_var_name: *mut libc::c_char,
    pub p_def_size: *mut PlLong,
    pub default_size: libc::c_int,
    pub size: libc::c_int,
    pub stack: *mut WamWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AtomInf {
    pub name: *mut libc::c_char,
    pub hash: libc::c_uint,
    pub prop: AtomProp,
    pub info: *mut libc::c_void,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AtomProp {
    #[bitfield(name = "length", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "op_mask", ty = "libc::c_uint", bits = "16..=19")]
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "20..=21")]
    #[bitfield(name = "needs_quote", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "needs_scan", ty = "libc::c_uint", bits = "23..=23")]
    pub length_op_mask_type_0_needs_quote_needs_scan: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PredInf {
    pub f_n: PlLong,
    pub pl_file: libc::c_int,
    pub pl_line: libc::c_int,
    pub prop: libc::c_int,
    pub codep: *mut PlLong,
    pub dyn_0: *mut PlLong,
}
unsafe extern "C" fn Set_Locale() {
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    setlocale(1 as libc::c_int, b"C\0" as *const u8 as *const libc::c_char);
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut save_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_buff_signal_reg: [WamWord; 5] = [0; 5];
pub static mut pl_reg_tbl: [*mut libc::c_char; 11] = [
    b"TR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"B\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"H\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"HB1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"E\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"STAMP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"BCI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"LSSA\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub static mut pl_tag_tbl: [InfTag; 7] = [
    {
        let mut init = InfTag {
            name: b"REF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: ADDRESS,
            value: 0 as libc::c_int,
            tag_mask: 0 as libc::c_int as PlLong,
        };
        init
    },
    {
        let mut init = InfTag {
            name: b"LST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: ADDRESS,
            value: 1 as libc::c_int,
            tag_mask: 1 as libc::c_int as PlLong,
        };
        init
    },
    {
        let mut init = InfTag {
            name: b"STC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: ADDRESS,
            value: 2 as libc::c_int,
            tag_mask: 2 as libc::c_int as PlLong,
        };
        init
    },
    {
        let mut init = InfTag {
            name: b"ATM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: SHORT_UNS,
            value: 3 as libc::c_int,
            tag_mask: 3 as libc::c_int as PlLong,
        };
        init
    },
    {
        let mut init = InfTag {
            name: b"FLT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: ADDRESS,
            value: 4 as libc::c_int,
            tag_mask: 4 as libc::c_int as PlLong,
        };
        init
    },
    {
        let mut init = InfTag {
            name: b"FDV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: ADDRESS,
            value: 5 as libc::c_int,
            tag_mask: 5 as libc::c_int as PlLong,
        };
        init
    },
    {
        let mut init = InfTag {
            name: b"INT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0: LONG_INT,
            value: 7 as libc::c_int,
            tag_mask: 7 as libc::c_int as PlLong,
        };
        init
    },
];
pub static mut pl_def_trail_size: PlLong = 0;
pub static mut pl_def_cstr_size: PlLong = 0;
pub static mut pl_def_global_size: PlLong = 0;
pub static mut pl_def_local_size: PlLong = 0;
pub static mut pl_fixed_sizes: PlLong = 0;
pub static mut pl_stk_tbl: [InfStack; 4] = unsafe {
    [
        {
            let mut init = InfStack {
                name: b"trail\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                desc: b"Trail Stack (undo)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                env_var_name: b"TRAILSZ\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p_def_size: &pl_def_trail_size as *const PlLong as *mut PlLong,
                default_size: 2097152 as libc::c_int,
                size: 0 as libc::c_int,
                stack: 0 as *const WamWord as *mut WamWord,
            };
            init
        },
        {
            let mut init = InfStack {
                name: b"cstr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                desc: b"Cstr Stack (constraints)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                env_var_name: b"CSTRSZ\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p_def_size: &pl_def_cstr_size as *const PlLong as *mut PlLong,
                default_size: 2097152 as libc::c_int,
                size: 0 as libc::c_int,
                stack: 0 as *const WamWord as *mut WamWord,
            };
            init
        },
        {
            let mut init = InfStack {
                name: b"global\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                desc: b"Global Stack (heap)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                env_var_name: b"GLOBALSZ\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p_def_size: &pl_def_global_size as *const PlLong as *mut PlLong,
                default_size: 4194304 as libc::c_int,
                size: 0 as libc::c_int,
                stack: 0 as *const WamWord as *mut WamWord,
            };
            init
        },
        {
            let mut init = InfStack {
                name: b"local\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                desc: b"Local Stack (control)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                env_var_name: b"LOCALSZ\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p_def_size: &pl_def_local_size as *const PlLong as *mut PlLong,
                default_size: 2097152 as libc::c_int,
                size: 0 as libc::c_int,
                stack: 0 as *const WamWord as *mut WamWord,
            };
            init
        },
    ]
};
pub static mut pl_os_argc: libc::c_int = 0;
pub static mut pl_os_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut pl_home: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut pl_devel_mode: libc::c_int = 0;
pub static mut pl_glob_buff: [libc::c_char; 10240] = [0; 10240];
pub static mut pl_base_fl: *mut PlLong = 0 as *const PlLong as *mut PlLong;
pub static mut pl_base_fd: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
pub static mut pl_le_mode: libc::c_int = 0;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_le_hook_start: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_put_char: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_char0: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_emit_beep: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_ins_mode: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_screen_size: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_kbd_is_not_empty: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_backd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_forwd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ_str: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_erase: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_set_line_buffering: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_line_buffering: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_flush: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_confirm_box: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_message_box: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_exit_process: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_initialize: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
unsafe extern "C" fn Get_Prolog_Path(
    mut argv0: *mut libc::c_char,
    mut devel_mode: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    static mut prolog_path_cache: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut devel_mode_cache: libc::c_int = 0 as libc::c_int;
    static mut resolved: [libc::c_char; 4096] = [0; 4096];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !prolog_path_cache.is_null() {
        *devel_mode = devel_mode_cache;
        return prolog_path_cache;
    }
    p = getenv(b"PL_PATH\0" as *const u8 as *const libc::c_char);
    if !p.is_null() {
        strcpy(resolved.as_mut_ptr(), p);
        p = Is_A_Valid_Root(resolved.as_mut_ptr(), devel_mode);
        if !p.is_null() {
            current_block = 13730816555730645546;
        } else {
            current_block = 10879442775620481940;
        }
    } else {
        current_block = 10879442775620481940;
    }
    match current_block {
        10879442775620481940 => {
            if !(!argv0.is_null()
                && {
                    p = Get_Prolog_Path_From_Exec(argv0, devel_mode);
                    !p.is_null()
                })
            {
                p = Search_Path(
                    b"gplc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if p.is_null() {
                    return 0 as *mut libc::c_char;
                }
                p = Get_Prolog_Path_From_Exec(p, devel_mode);
                if p.is_null() {
                    return 0 as *mut libc::c_char;
                }
            }
        }
        _ => {}
    }
    devel_mode_cache = *devel_mode;
    prolog_path_cache = strdup(p);
    return p;
}
unsafe extern "C" fn Get_Prolog_Path_From_Exec(
    mut str: *mut libc::c_char,
    mut devel_mode: *mut libc::c_int,
) -> *mut libc::c_char {
    static mut resolved: [libc::c_char; 4096] = [0; 4096];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if (realpath(str, resolved.as_mut_ptr())).is_null() {
        return 0 as *mut libc::c_char;
    }
    p = resolved
        .as_mut_ptr()
        .offset(strlen(resolved.as_mut_ptr()) as isize)
        .offset(-(1 as libc::c_int as isize));
    while p > resolved.as_mut_ptr()
        && !(*p as libc::c_int == '/' as i32 || *p as libc::c_int == '/' as i32)
    {
        p = p.offset(-1);
        p;
    }
    if p == resolved.as_mut_ptr() {
        return 0 as *mut libc::c_char;
    }
    while p > resolved.as_mut_ptr()
        && (*p as libc::c_int == '/' as i32 || *p as libc::c_int == '/' as i32)
    {
        p = p.offset(-1);
        p;
    }
    if p == resolved.as_mut_ptr() {
        return 0 as *mut libc::c_char;
    }
    while p > resolved.as_mut_ptr()
        && !(*p as libc::c_int == '/' as i32 || *p as libc::c_int == '/' as i32)
    {
        p = p.offset(-1);
        p;
    }
    if p == resolved.as_mut_ptr() {
        return 0 as *mut libc::c_char;
    }
    *p.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    return Is_A_Valid_Root(resolved.as_mut_ptr(), devel_mode);
}
unsafe extern "C" fn Is_A_Valid_Root(
    mut str: *mut libc::c_char,
    mut devel_mode: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = str.offset(strlen(str) as isize).offset(-(1 as libc::c_int as isize));
    while p >= str
        && (*p as libc::c_int == '/' as i32 || *p as libc::c_int == '/' as i32)
    {
        p = p.offset(-1);
        p;
    }
    if !(p < str) {
        p = p.offset(1);
        p;
        strcpy(p, b"/bin/gplc\0" as *const u8 as *const libc::c_char);
        if access(str, 1 as libc::c_int) == 0 as libc::c_int {
            *p = '\0' as i32 as libc::c_char;
            *devel_mode = 0 as libc::c_int;
            current_block = 9337025675697291090;
        } else {
            strcpy(p, b"/TopComp\0" as *const u8 as *const libc::c_char);
            if access(str, 0 as libc::c_int) == 0 as libc::c_int {
                *p = '\0' as i32 as libc::c_char;
                *devel_mode = 1 as libc::c_int;
                current_block = 9337025675697291090;
            } else {
                current_block = 1171095550847537483;
            }
        }
        match current_block {
            1171095550847537483 => {}
            _ => return str,
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn Search_Path(mut file: *mut libc::c_char) -> *mut libc::c_char {
    let mut path: *mut libc::c_char = getenv(
        b"PATH\0" as *const u8 as *const libc::c_char,
    );
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    static mut buff: [libc::c_char; 4096] = [0; 4096];
    if path.is_null() {
        return 0 as *mut libc::c_char;
    }
    p = path;
    loop {
        p = strchr(path, ':' as i32);
        if !p.is_null() {
            l = p.offset_from(path) as libc::c_long as libc::c_int;
            strncpy(buff.as_mut_ptr(), path, l as libc::c_ulong);
        } else {
            strcpy(buff.as_mut_ptr(), path);
            l = strlen(buff.as_mut_ptr()) as libc::c_int;
        }
        let fresh0 = l;
        l = l + 1;
        buff[fresh0 as usize] = '/' as i32 as libc::c_char;
        strcpy(buff.as_mut_ptr().offset(l as isize), file);
        if access(buff.as_mut_ptr(), 1 as libc::c_int) == 0 as libc::c_int {
            return buff.as_mut_ptr();
        }
        if p.is_null() {
            break;
        }
        path = p.offset(1 as libc::c_int as isize);
    }
    return 0 as *mut libc::c_char;
}
pub static mut pl_init_stream_supp: Option::<unsafe extern "C" fn() -> ()> = None;
static mut init_buff_regs: [WamWord; 4] = [0; 4];
static mut heap_actual_start: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut nb_user_directives: libc::c_int = 0 as libc::c_int;
static mut p_jumper: *mut sigjmp_buf = 0 as *const sigjmp_buf as *mut sigjmp_buf;
static mut p_buff_save: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut cont_jmp: CodePtr = None;
pub unsafe extern "C" fn Pl_Start_Prolog(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy_of_pl_init_stream_supp: Option::<unsafe extern "C" fn() -> ()> = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn() -> ()>,
    >(
        Pl_Dummy_Ptr(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                *mut libc::c_void,
            >(pl_init_stream_supp),
        ),
    );
    Set_Locale();
    pl_os_argc = argc;
    pl_os_argv = argv;
    pl_home = Get_Prolog_Path(
        *argv.offset(0 as libc::c_int as isize),
        &mut pl_devel_mode,
    );
    Pl_Init_Machine();
    setlinebuf(stdout);
    setlinebuf(stderr);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if pl_fd_init_solver.is_none()
            && strcmp(
                pl_stk_tbl[i as usize].name,
                b"cstr\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            pl_stk_tbl[i as usize].size = 0 as libc::c_int;
        } else {
            pl_stk_tbl[i as usize]
                .size = ((1024 as libc::c_int as libc::c_long
                * *pl_stk_tbl[i as usize].p_def_size) as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                as libc::c_int;
            if pl_stk_tbl[i as usize].size == 0 as libc::c_int {
                pl_stk_tbl[i as usize].size = pl_stk_tbl[i as usize].default_size;
            }
            if pl_fixed_sizes == 0
                && *pl_stk_tbl[i as usize].env_var_name as libc::c_int != 0
            {
                p = getenv(pl_stk_tbl[i as usize].env_var_name);
                if !p.is_null() && *p as libc::c_int != 0 {
                    sscanf(
                        p,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        &mut x as *mut libc::c_int,
                    );
                    pl_stk_tbl[i as usize]
                        .size = ((1024 as libc::c_int * x) as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as libc::c_int;
                }
            }
        }
        i += 1;
        i;
    }
    pl_max_atom = pl_def_max_atom as PlULong;
    if pl_max_atom == 0 as libc::c_int as libc::c_ulong {
        pl_max_atom = 32768 as libc::c_int as PlULong;
    }
    if pl_fixed_sizes == 0 {
        p = getenv(b"MAX_ATOM\0" as *const u8 as *const libc::c_char);
        if !p.is_null() && *p as libc::c_int != 0 {
            sscanf(
                p,
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut x as *mut libc::c_int,
            );
            pl_max_atom = x as PlULong;
        }
    }
    Pl_Allocate_Stacks();
    init_buff_regs[0 as libc::c_int as usize] = pl_reg_bank as WamWord;
    init_buff_regs[1 as libc::c_int as usize] = TR as WamWord;
    init_buff_regs[2 as libc::c_int as usize] = B as WamWord;
    init_buff_regs[3 as libc::c_int as usize] = H as WamWord;
    pl_reg_bank = pl_stk_tbl[2 as libc::c_int as usize].stack;
    save_reg_bank = pl_reg_bank;
    pl_stk_tbl[2 as libc::c_int as usize]
        .stack = (pl_stk_tbl[2 as libc::c_int as usize].stack)
        .offset((256 as libc::c_int + 8 as libc::c_int) as isize);
    pl_stk_tbl[2 as libc::c_int as usize].size -= 256 as libc::c_int + 8 as libc::c_int;
    heap_actual_start = pl_stk_tbl[2 as libc::c_int as usize].stack;
    Pl_Init_Atom();
    Pl_Init_Pred();
    Pl_Init_Oper();
    pl_le_mode = 0 as libc::c_int;
    if pl_le_initialize.is_some() {
        pl_le_mode = ::std::mem::transmute::<
            _,
            fn() -> libc::c_int,
        >((Some(pl_le_initialize.unwrap())).unwrap())();
    }
    if copy_of_pl_init_stream_supp.is_some() {
        ::std::mem::transmute::<
            _,
            fn(),
        >((Some(copy_of_pl_init_stream_supp.unwrap())).unwrap())();
    }
    Pl_Reset_Prolog();
    Pl_Fd_Init_Solver();
    Pl_Find_Linked_Objects();
    return nb_user_directives;
}
pub unsafe extern "C" fn Pl_Stop_Prolog() {
    pl_reg_bank = init_buff_regs[0 as libc::c_int as usize] as WamWordP;
    TR = init_buff_regs[1 as libc::c_int as usize] as WamWordP;
    B = init_buff_regs[2 as libc::c_int as usize] as WamWordP;
    H = init_buff_regs[3 as libc::c_int as usize] as WamWordP;
}
pub unsafe extern "C" fn Pl_Reset_Prolog() {
    let ref mut fresh1 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 7 as libc::c_int) as isize);
    *fresh1 = pl_stk_tbl[3 as libc::c_int as usize].stack;
    B = *fresh1;
    let ref mut fresh2 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh2 = B;
    H = heap_actual_start;
    TR = pl_stk_tbl[0 as libc::c_int as usize].stack;
    let ref mut fresh3 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh3 = None;
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 5 as libc::c_int) as isize,
        ) = 0 as libc::c_int as WamWord;
    let ref mut fresh4 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh4 = pl_stk_tbl[1 as libc::c_int as usize].stack;
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = 0 as libc::c_int as WamWord;
    Pl_Create_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(Some(Call_Prolog_Fail as unsafe extern "C" fn() -> ())),
        0 as libc::c_int,
    );
    Pl_Fd_Reset_Solver();
}
pub unsafe extern "C" fn Pl_Reset_Prolog_In_Signal() {
    if pl_buff_signal_reg[4 as libc::c_int as usize] != 0 {
        pl_reg_bank = pl_buff_signal_reg[0 as libc::c_int as usize] as WamWordP;
        TR = pl_buff_signal_reg[1 as libc::c_int as usize] as WamWordP;
        B = pl_buff_signal_reg[2 as libc::c_int as usize] as WamWordP;
        H = pl_buff_signal_reg[3 as libc::c_int as usize] as WamWordP;
        pl_buff_signal_reg[4 as libc::c_int as usize] = 0 as libc::c_int as WamWord;
    }
    pl_reg_bank = save_reg_bank;
}
pub unsafe extern "C" fn Pl_Set_Heap_Actual_Start(
    mut new_heap_actual_start: *mut WamWord,
) {
    heap_actual_start = new_heap_actual_start;
}
pub unsafe extern "C" fn Pl_Execute_Directive(
    mut pl_file: libc::c_int,
    mut pl_line: libc::c_int,
    mut is_system: Bool,
    mut proc_0: CodePtr,
) {
    Pl_Reset_Prolog();
    if is_system == 0 {
        nb_user_directives += 1;
        nb_user_directives;
    }
    if Pl_Call_Prolog(proc_0) == 0 {
        fprintf(
            stderr,
            b"warning: %s:%d: %s directive failed\n\0" as *const u8
                as *const libc::c_char,
            (*pl_atom_tbl.offset(pl_file as isize)).name,
            pl_line,
            if is_system != 0 {
                b"system\0" as *const u8 as *const libc::c_char
            } else {
                b"user\0" as *const u8 as *const libc::c_char
            },
        );
    }
    Pl_Reset_Prolog();
}
pub unsafe extern "C" fn Pl_Try_Execute_Top_Level() -> Bool {
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    Pl_Reset_Prolog();
    pred = Pl_Lookup_Pred(
        Pl_Create_Atom(
            b"top_level\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        0 as libc::c_int,
    );
    if !pred.is_null() {
        Pl_Call_Prolog(::std::mem::transmute::<*mut PlLong, CodePtr>((*pred).codep));
        return 1 as libc::c_int;
    }
    Pl_Reset_Prolog();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Call_Prolog(mut codep: CodePtr) -> libc::c_int {
    let mut query_b: *mut WamWord = B;
    let mut save_CP: WamCont = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    let mut save_ALTB: WamCont = *(&mut *query_b.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    let mut ret: libc::c_int = 0;
    let ref mut fresh5 = *(&mut *query_b.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh5 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        CodePtr,
    >(Some(Call_Prolog_Fail as unsafe extern "C" fn() -> ()));
    let ref mut fresh6 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh6 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        WamCont,
    >(Some(Call_Prolog_Success as unsafe extern "C" fn() -> ()));
    ret = Call_Next(codep);
    let ref mut fresh7 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh7 = save_CP;
    let ref mut fresh8 = *(&mut *query_b.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh8 = save_ALTB;
    return ret;
}
pub unsafe extern "C" fn Pl_Call_Prolog_Next_Sol(
    mut query_b: *mut WamWord,
) -> libc::c_int {
    let mut save_CP: WamCont = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    let mut save_ALTB: WamCont = *(&mut *query_b.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    let mut ret: libc::c_int = 0;
    let ref mut fresh9 = *(&mut *query_b.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh9 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        CodePtr,
    >(Some(Call_Prolog_Fail as unsafe extern "C" fn() -> ()));
    let ref mut fresh10 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh10 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        WamCont,
    >(Some(Call_Prolog_Success as unsafe extern "C" fn() -> ()));
    ret = Call_Next(
        *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord as *mut CodePtr),
    );
    let ref mut fresh11 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh11 = save_CP;
    let ref mut fresh12 = *(&mut *query_b.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh12 = save_ALTB;
    return ret;
}
pub unsafe extern "C" fn Pl_Keep_Rest_For_Prolog(mut query_b: *mut WamWord) {
    let mut b: *mut WamWord = 0 as *mut WamWord;
    let mut e: *mut WamWord = 0 as *mut WamWord;
    let mut query_e: *mut WamWord = 0 as *mut WamWord;
    b = B;
    while b > query_b {
        if *(&mut *b.offset(-(2 as libc::c_int) as isize) as *mut WamWord
            as *mut WamCont)
            == ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                WamCont,
            >(Some(Call_Prolog_Success as unsafe extern "C" fn() -> ()))
        {
            let ref mut fresh13 = *(&mut *b.offset(-(2 as libc::c_int) as isize)
                as *mut WamWord as *mut WamCont);
            *fresh13 = *(pl_reg_bank as *mut WamCont)
                .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
        }
        b = *(&mut *b.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    }
    query_e = *(&mut *query_b.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    e = *(&mut *B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    while e > query_e {
        if *(&mut *e.offset(-(1 as libc::c_int) as isize) as *mut WamWord
            as *mut WamCont)
            == ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                WamCont,
            >(Some(Call_Prolog_Success as unsafe extern "C" fn() -> ()))
        {
            let ref mut fresh14 = *(&mut *e.offset(-(1 as libc::c_int) as isize)
                as *mut WamWord as *mut WamCont);
            *fresh14 = *(pl_reg_bank as *mut WamCont)
                .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
        }
        e = *(&mut *e.offset(-(3 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    }
}
unsafe extern "C" fn Call_Next(mut codep: CodePtr) -> libc::c_int {
    let mut jmp_val: libc::c_int = 0;
    let mut old_jumper: *mut sigjmp_buf = p_jumper;
    let mut new_jumper: sigjmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    let mut old_buff_save: *mut WamWord = p_buff_save;
    let mut buff_save_machine_regs: [WamWord; 5] = [0; 5];
    p_jumper = &mut new_jumper;
    p_buff_save = buff_save_machine_regs.as_mut_ptr();
    buff_save_machine_regs[0 as libc::c_int as usize] = pl_reg_bank as WamWord;
    buff_save_machine_regs[1 as libc::c_int as usize] = TR as WamWord;
    buff_save_machine_regs[2 as libc::c_int as usize] = B as WamWord;
    buff_save_machine_regs[3 as libc::c_int as usize] = H as WamWord;
    jmp_val = __sigsetjmp((*p_jumper).as_mut_ptr(), 1 as libc::c_int);
    pl_reg_bank = buff_save_machine_regs[0 as libc::c_int as usize] as WamWordP;
    TR = buff_save_machine_regs[1 as libc::c_int as usize] as WamWordP;
    B = buff_save_machine_regs[2 as libc::c_int as usize] as WamWordP;
    H = buff_save_machine_regs[3 as libc::c_int as usize] as WamWordP;
    if jmp_val == 0 as libc::c_int {
        Pl_Call_Compiled(codep);
    }
    if jmp_val == 3 as libc::c_int {
        Pl_Call_Compiled(cont_jmp);
    }
    p_jumper = old_jumper;
    p_buff_save = old_buff_save;
    if jmp_val < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return jmp_val;
}
unsafe extern "C" fn Call_Prolog_Fail() {
    asm!("subq $8,%rsp", options(preserves_flags, att_syntax));
    *p_buff_save.offset(0 as libc::c_int as isize) = pl_reg_bank as WamWord;
    *p_buff_save.offset(1 as libc::c_int as isize) = TR as WamWord;
    *p_buff_save.offset(2 as libc::c_int as isize) = B as WamWord;
    *p_buff_save.offset(3 as libc::c_int as isize) = H as WamWord;
    siglongjmp((*p_jumper).as_mut_ptr(), -(1 as libc::c_int));
}
unsafe extern "C" fn Call_Prolog_Success() {
    asm!("subq $8,%rsp", options(preserves_flags, att_syntax));
    *p_buff_save.offset(0 as libc::c_int as isize) = pl_reg_bank as WamWord;
    *p_buff_save.offset(1 as libc::c_int as isize) = TR as WamWord;
    *p_buff_save.offset(2 as libc::c_int as isize) = B as WamWord;
    *p_buff_save.offset(3 as libc::c_int as isize) = H as WamWord;
    siglongjmp((*p_jumper).as_mut_ptr(), 1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Exit_With_Exception() {
    *p_buff_save.offset(0 as libc::c_int as isize) = pl_reg_bank as WamWord;
    *p_buff_save.offset(1 as libc::c_int as isize) = TR as WamWord;
    *p_buff_save.offset(2 as libc::c_int as isize) = B as WamWord;
    *p_buff_save.offset(3 as libc::c_int as isize) = H as WamWord;
    siglongjmp((*p_jumper).as_mut_ptr(), 2 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Execute_A_Continuation(mut codep: CodePtr) {
    *p_buff_save.offset(0 as libc::c_int as isize) = pl_reg_bank as WamWord;
    *p_buff_save.offset(1 as libc::c_int as isize) = TR as WamWord;
    *p_buff_save.offset(2 as libc::c_int as isize) = B as WamWord;
    *p_buff_save.offset(3 as libc::c_int as isize) = H as WamWord;
    cont_jmp = codep;
    siglongjmp((*p_jumper).as_mut_ptr(), 3 as libc::c_int);
}
