use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    static mut mi: MapperInf;
    fn Declare_Initializer(initializer_fct: *mut libc::c_char);
    fn Decl_Code(c: *mut CodeInf);
    fn Decl_Long(l: *mut LongInf);
    fn Call_C(
        fct_name_0: *mut libc::c_char,
        fc_0: Bool,
        nb_args_0: libc::c_int,
        nb_args_in_words_0: libc::c_int,
        arg_0: *mut ArgInf,
    );
    fn Switch_Ret(nb_swt_0: libc::c_int, swt_0: *mut SwtInf);
    fn Label_Printf(label: *mut libc::c_char, _: ...);
    fn Code_Start(c: *mut CodeInf);
    fn Code_Stop(c: *mut CodeInf);
    fn Label(label: *mut libc::c_char);
    fn Reload_E_In_Register();
    fn Pl_Jump(label: *mut libc::c_char);
    fn Prep_CP();
    fn Here_CP();
    fn Pl_Call(label: *mut libc::c_char);
    fn Pl_Fail();
    fn Pl_Ret();
    fn Jump(label: *mut libc::c_char);
    fn Move_From_Reg_X(index: libc::c_int);
    fn Move_From_Reg_Y(index: libc::c_int);
    fn Move_To_Reg_X(index: libc::c_int);
    fn Move_To_Reg_Y(index: libc::c_int);
    fn Jump_Ret();
    fn Fail_Ret();
    fn Move_Ret_To_Mem_L(name: *mut libc::c_char, index: libc::c_int);
    fn Move_Ret_To_Reg_X(index: libc::c_int);
    fn Move_Ret_To_Reg_Y(index: libc::c_int);
    fn Move_Ret_To_Foreign_L(index: libc::c_int);
    fn Move_Ret_To_Foreign_D(index: libc::c_int);
    fn C_Ret();
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
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
pub type Bool = libc::c_int;
pub type ArgTyp = libc::c_uint;
pub const STRING: ArgTyp = 264;
pub const FD_ARRAY: ArgTyp = 263;
pub const FL_ARRAY: ArgTyp = 262;
pub const Y_REG: ArgTyp = 261;
pub const X_REG: ArgTyp = 260;
pub const MEM: ArgTyp = 259;
pub const FLOAT: ArgTyp = 258;
pub const INTEGER: ArgTyp = 257;
pub const IDENTIFIER: ArgTyp = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArgInf {
    pub type_0: ArgTyp,
    pub adr_of: Bool,
    pub str_val: *mut libc::c_char,
    pub int_val: PlLong,
    pub dbl_val: libc::c_double,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwtInf {
    pub int_val: PlLong,
    pub label: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MapperInf {
    pub can_produce_pic_code: Bool,
    pub needs_pre_pass: Bool,
    pub comment_prefix: *mut libc::c_char,
    pub local_symb_prefix: *mut libc::c_char,
    pub string_symb_prefix: *mut libc::c_char,
    pub double_symb_prefix: *mut libc::c_char,
    pub strings_need_null: Bool,
    pub call_c_reverse_args: Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CodeInf {
    pub name: *mut libc::c_char,
    pub prolog: Bool,
    pub initializer: Bool,
    pub global: Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LongInf {
    pub name: *mut libc::c_char,
    pub global: Bool,
    pub vtype: VType,
    pub value: PlLong,
}
pub type VType = libc::c_uint;
pub const ARRAY_SIZE: VType = 2;
pub const INITIAL_VALUE: VType = 1;
pub const NONE: VType = 0;
pub const LONG: C2RustUnnamed_0 = 16;
pub const C_RET: C2RustUnnamed_0 = 15;
pub const SWITCH_RET: C2RustUnnamed_0 = 13;
pub const MOVE_RET: C2RustUnnamed_0 = 12;
pub const FAIL_RET: C2RustUnnamed_0 = 11;
pub const JUMP_RET: C2RustUnnamed_0 = 10;
pub const CALL_C: C2RustUnnamed_0 = 9;
pub const MOVE: C2RustUnnamed_0 = 8;
pub const JUMP: C2RustUnnamed_0 = 7;
pub const PL_RET: C2RustUnnamed_0 = 6;
pub const PL_FAIL: C2RustUnnamed_0 = 5;
pub const PL_CALL: C2RustUnnamed_0 = 4;
pub const HERE_CP: C2RustUnnamed_0 = 3;
pub const PREP_CP: C2RustUnnamed_0 = 2;
pub const PL_JUMP: C2RustUnnamed_0 = 1;
pub const C_CODE: C2RustUnnamed_0 = 14;
pub const PL_CODE: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub static mut inst: [*mut libc::c_char; 18] = [
    b"pl_code\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pl_jump\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"prep_cp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"here_cp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pl_call\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pl_fail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pl_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"jump\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"move\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"call_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"jump_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"fail_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"move_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"switch_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"c_code\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"c_ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"long\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
pub static mut reload_e: Bool = 0;
pub static mut fct_name: [libc::c_char; 32768] = [0; 32768];
pub static mut fc: Bool = 0;
pub static mut nb_args: libc::c_int = 0;
pub static mut nb_args_in_words: libc::c_int = 0;
pub static mut arg: [ArgInf; 128] = [ArgInf {
    type_0: 0 as ArgTyp,
    adr_of: 0,
    str_val: 0 as *const libc::c_char as *mut libc::c_char,
    int_val: 0,
    dbl_val: 0.,
    index: 0,
}; 128];
pub static mut nb_swt: libc::c_int = 0;
pub static mut swt: [SwtInf; 65536] = [SwtInf {
    int_val: 0,
    label: 0 as *const libc::c_char as *mut libc::c_char,
}; 65536];
pub static mut jumper: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
pub static mut keep_source_lines: libc::c_int = 0;
pub static mut file_in: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut cur_line_no: libc::c_int = 0;
pub static mut cur_line_str: [libc::c_char; 65536] = [0; 65536];
pub static mut cur_line_p: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut beg_last_token: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut str_val: [libc::c_char; 32768] = [0; 32768];
pub static mut int_val: PlLong = 0;
pub static mut dbl_val: libc::c_double = 0.;
pub unsafe extern "C" fn Parse_Ma_File(
    mut file_name_in: *mut libc::c_char,
    mut comment: libc::c_int,
) -> libc::c_int {
    let mut ret_val: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nb_passes: libc::c_int = mi.needs_pre_pass + 1 as libc::c_int;
    if file_name_in.is_null() {
        file_name_in = b"stdin\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        file_in = stdin;
    } else {
        file_in = fopen(file_name_in, b"rt\0" as *const u8 as *const libc::c_char);
        if file_in.is_null() {
            fprintf(
                stderr,
                b"cannot open input file %s\n\0" as *const u8 as *const libc::c_char,
                file_name_in,
            );
            return 0 as libc::c_int;
        }
    }
    i = 1 as libc::c_int;
    while i <= nb_passes {
        if i == 2 as libc::c_int
            && fseek(file_in, 0 as libc::c_int as libc::c_long, 0 as libc::c_int)
                == -(1 as libc::c_int)
        {
            fprintf(
                stderr,
                b"cannot reposition file %s (needed for 2 passes)\n\0" as *const u8
                    as *const libc::c_char,
                file_name_in,
            );
            return 0 as libc::c_int;
        }
        keep_source_lines = comment;
        ret_val = _setjmp(jumper.as_mut_ptr());
        if ret_val == 0 as libc::c_int {
            Parser(i, nb_passes);
        }
        if ret_val != 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    if file_in != stdin {
        fclose(file_in);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Parser(mut pass_no: libc::c_int, mut nb_passes: libc::c_int) {
    let mut inside_code: Bool = 0 as libc::c_int;
    let mut initializer_defined: Bool = 0 as libc::c_int;
    let mut cur_code: CodeInf = CodeInf {
        name: 0 as *mut libc::c_char,
        prolog: 0,
        initializer: 0,
        global: 0,
    };
    let mut l: LongInf = LongInf {
        name: 0 as *mut libc::c_char,
        global: 0,
        vtype: NONE,
        value: 0,
    };
    let mut in_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if pass_no < nb_passes {
        keep_source_lines = 0 as libc::c_int;
    }
    cur_line_p = cur_line_str.as_mut_ptr();
    cur_line_str[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    cur_line_no = 0 as libc::c_int;
    loop {
        k = Scanner();
        if k == 0 as libc::c_int {
            break;
        }
        if k != IDENTIFIER as libc::c_int {
            Syntax_Error(
                b"miniasm declaration or instruction expected\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        in_0 = inst.as_mut_ptr();
        while !(*in_0).is_null() {
            if strcmp(str_val.as_mut_ptr(), *in_0) == 0 as libc::c_int {
                break;
            }
            in_0 = in_0.offset(1);
            in_0;
        }
        k = in_0.offset_from(inst.as_mut_ptr()) as libc::c_long as libc::c_int;
        if pass_no < nb_passes && k != PL_CODE as libc::c_int
            && k != C_CODE as libc::c_int && k != LONG as libc::c_int
            && !(*in_0).is_null()
            || pass_no > 1 as libc::c_int && k == LONG as libc::c_int
        {
            *cur_line_p = '\0' as i32 as libc::c_char;
        } else {
            match k {
                0 => {
                    if !(pass_no < nb_passes) && inside_code != 0 {
                        Code_Stop(&mut cur_code);
                        inside_code = 0 as libc::c_int;
                    }
                    cur_code.prolog = 1 as libc::c_int;
                    cur_code.global = Read_If_Global(0 as libc::c_int);
                    cur_code.initializer = 0 as libc::c_int;
                    Read_Token(IDENTIFIER as libc::c_int);
                    cur_code.name = strdup(str_val.as_mut_ptr());
                    if pass_no < nb_passes {
                        Decl_Code(&mut cur_code);
                    } else {
                        Code_Start(&mut cur_code);
                        reload_e = 1 as libc::c_int;
                        inside_code = 1 as libc::c_int;
                    }
                }
                14 => {
                    if !(pass_no < nb_passes) && inside_code != 0 {
                        Code_Stop(&mut cur_code);
                        inside_code = 0 as libc::c_int;
                    }
                    cur_code.prolog = 0 as libc::c_int;
                    cur_code
                        .global = Read_If_Global(
                        (initializer_defined == 0) as libc::c_int,
                    );
                    cur_code.initializer = 0 as libc::c_int;
                    Read_Token(IDENTIFIER as libc::c_int);
                    cur_code.name = strdup(str_val.as_mut_ptr());
                    if cur_code.global == 2 as libc::c_int {
                        initializer_defined = 1 as libc::c_int;
                        cur_code.global = 0 as libc::c_int;
                        cur_code.initializer = 1 as libc::c_int;
                        if !(pass_no < nb_passes) {
                            Declare_Initializer(cur_code.name);
                        }
                    }
                    if pass_no < nb_passes {
                        Decl_Code(&mut cur_code);
                    } else {
                        Code_Start(&mut cur_code);
                        inside_code = 1 as libc::c_int;
                    }
                }
                1 => {
                    Read_Token(IDENTIFIER as libc::c_int);
                    Pl_Jump(str_val.as_mut_ptr());
                    reload_e = 1 as libc::c_int;
                }
                2 => {
                    Prep_CP();
                }
                3 => {
                    Here_CP();
                }
                4 => {
                    Read_Token(IDENTIFIER as libc::c_int);
                    Pl_Call(str_val.as_mut_ptr());
                    reload_e = 1 as libc::c_int;
                }
                5 => {
                    Pl_Fail();
                    reload_e = 1 as libc::c_int;
                }
                6 => {
                    Pl_Ret();
                    reload_e = 1 as libc::c_int;
                }
                7 => {
                    Read_Token(IDENTIFIER as libc::c_int);
                    Jump(str_val.as_mut_ptr());
                    reload_e = 1 as libc::c_int;
                }
                8 => {
                    k = Read_Token(X_REG as libc::c_int);
                    i = Read_Index();
                    if k == X_REG as libc::c_int {
                        Move_From_Reg_X(i);
                    } else {
                        Move_From_Reg_Y(i);
                    }
                    Read_Token(',' as i32);
                    k = Read_Token(X_REG as libc::c_int);
                    i = Read_Index();
                    if k == X_REG as libc::c_int {
                        Move_To_Reg_X(i);
                    } else {
                        Move_To_Reg_Y(i);
                    }
                }
                9 => {
                    Read_Function();
                    Call_C(
                        fct_name.as_mut_ptr(),
                        fc,
                        nb_args,
                        nb_args_in_words,
                        arg.as_mut_ptr(),
                    );
                }
                10 => {
                    Jump_Ret();
                    reload_e = 1 as libc::c_int;
                }
                11 => {
                    Fail_Ret();
                }
                12 => {
                    k = Scanner();
                    match k {
                        256 => {
                            Move_Ret_To_Mem_L(
                                str_val.as_mut_ptr(),
                                Read_Optional_Index(),
                            );
                        }
                        260 => {
                            Move_Ret_To_Reg_X(Read_Index());
                        }
                        261 => {
                            Move_Ret_To_Reg_Y(Read_Index());
                        }
                        262 => {
                            Move_Ret_To_Foreign_L(Read_Index());
                        }
                        263 => {
                            Move_Ret_To_Foreign_D(Read_Index());
                        }
                        _ => {
                            Syntax_Error(
                                b"identifier, X(...), Y(...), FL(...) or FD(...) expected\0"
                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                    }
                }
                13 => {
                    Read_Switch();
                    Switch_Ret(nb_swt, swt.as_mut_ptr());
                }
                15 => {
                    C_Ret();
                }
                16 => {
                    if !(pass_no < nb_passes) && inside_code != 0 {
                        Code_Stop(&mut cur_code);
                        inside_code = 0 as libc::c_int;
                    }
                    l.global = Read_If_Global(0 as libc::c_int);
                    Read_Token(IDENTIFIER as libc::c_int);
                    l.name = strdup(str_val.as_mut_ptr());
                    i = Read_Optional_Index();
                    if i > 0 as libc::c_int {
                        l.vtype = ARRAY_SIZE;
                        l.value = i as PlLong;
                    } else {
                        while *(*__ctype_b_loc())
                            .offset(*cur_line_p as libc::c_int as isize) as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            cur_line_p = cur_line_p.offset(1);
                            cur_line_p;
                        }
                        if *cur_line_p as libc::c_int != '=' as i32 {
                            l.vtype = NONE;
                            l.value = 1 as libc::c_int as PlLong;
                        } else {
                            cur_line_p = cur_line_p.offset(1);
                            cur_line_p;
                            Read_Token(INTEGER as libc::c_int);
                            l.vtype = INITIAL_VALUE;
                            l.value = int_val;
                        }
                    }
                    Decl_Long(&mut l);
                }
                _ => {
                    if (*in_0).is_null() {
                        let mut c_lab: CodeInf = CodeInf {
                            name: 0 as *mut libc::c_char,
                            prolog: 0,
                            initializer: 0,
                            global: 0,
                        };
                        c_lab.name = strdup(str_val.as_mut_ptr());
                        c_lab.prolog = 0 as libc::c_int;
                        c_lab.initializer = 0 as libc::c_int;
                        c_lab.global = 0 as libc::c_int;
                        Read_Token(':' as i32);
                        if pass_no < nb_passes {
                            Decl_Code(&mut c_lab);
                        } else {
                            Label(str_val.as_mut_ptr());
                        }
                    }
                }
            }
        }
    }
    if !(pass_no < nb_passes) && inside_code != 0 {
        Code_Stop(&mut cur_code);
        inside_code = 0 as libc::c_int;
    }
}
unsafe extern "C" fn Read_If_Global(mut initializer: Bool) -> libc::c_int {
    if !(Scanner() != IDENTIFIER as libc::c_int) {
        if strcmp(str_val.as_mut_ptr(), b"local\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if strcmp(str_val.as_mut_ptr(), b"global\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if initializer != 0
            && strcmp(
                str_val.as_mut_ptr(),
                b"initializer\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            return 2 as libc::c_int;
        }
    }
    if initializer == 0 {
        Syntax_Error(
            b"local / global expected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else {
        Syntax_Error(
            b"local / global / initializer expected\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Read_Function() {
    let mut k: libc::c_int = 0;
    fc = 0 as libc::c_int;
    Read_Token(IDENTIFIER as libc::c_int);
    if strcmp(str_val.as_mut_ptr(), b"fast\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        fc = 1 as libc::c_int;
        Read_Token(IDENTIFIER as libc::c_int);
    }
    strcpy(fct_name.as_mut_ptr(), str_val.as_mut_ptr());
    nb_args = 0 as libc::c_int;
    nb_args_in_words = 0 as libc::c_int;
    Read_Token('(' as i32);
    k = Scanner();
    if k == ')' as i32 {
        return;
    }
    loop {
        arg[nb_args as usize].type_0 = k as ArgTyp;
        arg[nb_args as usize].adr_of = 0 as libc::c_int;
        loop {
            match k {
                38 => {
                    k = Scanner();
                    if k != IDENTIFIER as libc::c_int && k != X_REG as libc::c_int
                        && k != Y_REG as libc::c_int && k != FL_ARRAY as libc::c_int
                        && k != FD_ARRAY as libc::c_int
                    {
                        Syntax_Error(
                            b"identifier, X(...), Y(...), FL(...) or FD(...) expected\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    arg[nb_args as usize].type_0 = k as ArgTyp;
                    arg[nb_args as usize].adr_of = 1 as libc::c_int;
                }
                264 => {
                    arg[nb_args as usize].str_val = strdup(str_val.as_mut_ptr());
                    break;
                }
                257 => {
                    arg[nb_args as usize].int_val = int_val;
                    break;
                }
                258 => {
                    arg[nb_args as usize].str_val = strdup(str_val.as_mut_ptr());
                    arg[nb_args as usize].dbl_val = dbl_val;
                    break;
                }
                256 => {
                    arg[nb_args as usize].type_0 = MEM;
                    arg[nb_args as usize].str_val = strdup(str_val.as_mut_ptr());
                    arg[nb_args as usize].index = Read_Optional_Index();
                    break;
                }
                263 | 262 | 260 | 261 => {
                    arg[nb_args as usize].index = Read_Index();
                    break;
                }
                _ => {
                    break;
                }
            }
        }
        k = Scanner();
        nb_args += 1;
        nb_args;
        nb_args_in_words += 1;
        nb_args_in_words;
        if k == ')' as i32 {
            break;
        }
        if k != ',' as i32 {
            Syntax_Error(
                b") or , expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        k = Scanner();
    };
}
unsafe extern "C" fn Read_Switch() {
    let mut k: libc::c_int = 0;
    Read_Token('(' as i32);
    nb_swt = 0 as libc::c_int;
    loop {
        if Scanner() != INTEGER as libc::c_int {
            Syntax_Error(
                b"integer expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        Read_Token('=' as i32);
        Read_Token(IDENTIFIER as libc::c_int);
        swt[nb_swt as usize].int_val = int_val;
        swt[nb_swt as usize].label = strdup(str_val.as_mut_ptr());
        nb_swt += 1;
        nb_swt;
        k = Scanner();
        if k == ')' as i32 {
            break;
        }
        if k != ',' as i32 {
            Syntax_Error(
                b") or , expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn Read_Index() -> libc::c_int {
    let mut k: libc::c_int = Scanner();
    if k == '(' as i32 {
        k = ')' as i32;
    } else if k == '[' as i32 {
        k = ']' as i32;
    } else {
        Syntax_Error(
            b"( index ) or [ index ] expected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    Read_Token(INTEGER as libc::c_int);
    Read_Token(k);
    return int_val as libc::c_int;
}
unsafe extern "C" fn Read_Optional_Index() -> libc::c_int {
    return if *cur_line_p as libc::c_int == '(' as i32
        || *cur_line_p as libc::c_int == '[' as i32
    {
        Read_Index()
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn Read_Token(mut what: libc::c_int) -> libc::c_int {
    let mut str: [libc::c_char; 80] = [0; 80];
    let mut k: libc::c_int = 0;
    k = Scanner();
    if k == what || what == X_REG as libc::c_int && k == Y_REG as libc::c_int
        || what == FL_ARRAY as libc::c_int && k == FD_ARRAY as libc::c_int
    {
        return k;
    }
    match what {
        256 => {
            Syntax_Error(
                b"identifier expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        264 => {
            Syntax_Error(
                b"string expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        257 => {
            Syntax_Error(
                b"integer expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        258 => {
            Syntax_Error(
                b"float expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        260 => {
            Syntax_Error(
                b"X(...) or Y(...) expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        262 => {
            Syntax_Error(
                b"FL(...) or FD(...) expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        _ => {
            sprintf(
                str.as_mut_ptr(),
                b"%c expected\0" as *const u8 as *const libc::c_char,
                what,
            );
            Syntax_Error(str.as_mut_ptr());
        }
    }
    return k;
}
unsafe extern "C" fn Scanner() -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: PlLong = 0;
    let mut d: libc::c_double = 0.;
    loop {
        while *(*__ctype_b_loc()).offset(*cur_line_p as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            cur_line_p = cur_line_p.offset(1);
            cur_line_p;
        }
        if *cur_line_p as libc::c_int != '\0' as i32
            && *cur_line_p as libc::c_int != ';' as i32
        {
            break;
        }
        !(fgets(
            cur_line_str.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 65536]>() as libc::c_ulong
                as libc::c_int,
            file_in,
        ))
            .is_null();
        if feof(file_in) != 0 {
            return 0 as libc::c_int;
        }
        cur_line_no += 1;
        cur_line_no;
        cur_line_p = cur_line_str.as_mut_ptr();
        if keep_source_lines != 0 {
            while *(*__ctype_b_loc()).offset(*cur_line_p as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                cur_line_p = cur_line_p.offset(1);
                cur_line_p;
            }
            if *cur_line_p != 0 {
                p = cur_line_p
                    .offset(strlen(cur_line_p) as isize)
                    .offset(-(1 as libc::c_int as isize));
                if *p as libc::c_int == '\n' as i32 {
                    *p = '\0' as i32 as libc::c_char;
                }
                Label_Printf(
                    b"\t%s %6d: %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    mi.comment_prefix,
                    cur_line_no,
                    cur_line_p,
                );
            }
        }
    }
    beg_last_token = cur_line_p;
    if *cur_line_p as libc::c_int == '"' as i32 {
        p = str_val.as_mut_ptr();
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '"' as i32 as libc::c_char;
        cur_line_p = cur_line_p.offset(1);
        cur_line_p;
        while *cur_line_p as libc::c_int != '"' as i32 {
            let fresh1 = cur_line_p;
            cur_line_p = cur_line_p.offset(1);
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = *fresh1;
            if *fresh2 as libc::c_int == '\\' as i32 {
                let fresh3 = cur_line_p;
                cur_line_p = cur_line_p.offset(1);
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = *fresh3;
            }
        }
        cur_line_p = cur_line_p.offset(1);
        cur_line_p;
        if mi.strings_need_null != 0 {
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = '\\' as i32 as libc::c_char;
            let fresh6 = p;
            p = p.offset(1);
            *fresh6 = '0' as i32 as libc::c_char;
        }
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = '"' as i32 as libc::c_char;
        *p = '\0' as i32 as libc::c_char;
        return STRING as libc::c_int;
    }
    if *(*__ctype_b_loc()).offset(*cur_line_p as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
        || *cur_line_p as libc::c_int == '_' as i32
        || *cur_line_p as libc::c_int == '.' as i32
    {
        p = str_val.as_mut_ptr();
        if *cur_line_p as libc::c_int == '.' as i32 {
            strcpy(p, mi.local_symb_prefix);
            p = p.offset(strlen(p) as isize);
            cur_line_p = cur_line_p.offset(1);
            cur_line_p;
        }
        while *(*__ctype_b_loc()).offset(*cur_line_p as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            != 0 || *cur_line_p as libc::c_int == '_' as i32
        {
            let fresh8 = cur_line_p;
            cur_line_p = cur_line_p.offset(1);
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = *fresh8;
        }
        *p = '\0' as i32 as libc::c_char;
        if str_val[0 as libc::c_int as usize] as libc::c_int == 'X' as i32
            && str_val[1 as libc::c_int as usize] as libc::c_int == '\0' as i32
            && *cur_line_p as libc::c_int == '(' as i32
        {
            return X_REG as libc::c_int;
        }
        if str_val[0 as libc::c_int as usize] as libc::c_int == 'Y' as i32
            && str_val[1 as libc::c_int as usize] as libc::c_int == '\0' as i32
            && *cur_line_p as libc::c_int == '(' as i32
        {
            if reload_e != 0 {
                Reload_E_In_Register();
                reload_e = 0 as libc::c_int;
            }
            return Y_REG as libc::c_int;
        }
        if strcmp(str_val.as_mut_ptr(), b"FL\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int && *cur_line_p as libc::c_int == '(' as i32
        {
            return FL_ARRAY as libc::c_int;
        }
        if strcmp(str_val.as_mut_ptr(), b"FD\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int && *cur_line_p as libc::c_int == '(' as i32
        {
            return FD_ARRAY as libc::c_int;
        }
        return IDENTIFIER as libc::c_int;
    }
    i = strtol(cur_line_p, &mut p, 0 as libc::c_int);
    if p == cur_line_p {
        let fresh10 = cur_line_p;
        cur_line_p = cur_line_p.offset(1);
        return *fresh10 as libc::c_int;
    }
    d = strtod(cur_line_p, &mut p1);
    if p1 == p {
        int_val = i;
        cur_line_p = p;
        return INTEGER as libc::c_int;
    }
    strncpy(
        str_val.as_mut_ptr(),
        cur_line_p,
        p1.offset_from(cur_line_p) as libc::c_long as libc::c_ulong,
    );
    str_val[p1.offset_from(cur_line_p) as libc::c_long
        as usize] = '\0' as i32 as libc::c_char;
    cur_line_p = p1;
    dbl_val = d;
    return FLOAT as libc::c_int;
}
pub unsafe extern "C" fn Syntax_Error(mut s: *mut libc::c_char) {
    let mut p: *mut libc::c_char = cur_line_str
        .as_mut_ptr()
        .offset(strlen(cur_line_str.as_mut_ptr()) as isize)
        .offset(-(1 as libc::c_int as isize));
    if *p as libc::c_int == '\n' as i32 {
        *p = '\0' as i32 as libc::c_char;
    }
    fprintf(
        stderr,
        b"line %d: %s\n\0" as *const u8 as *const libc::c_char,
        cur_line_no,
        s,
    );
    fprintf(
        stderr,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        cur_line_str.as_mut_ptr(),
    );
    p = cur_line_str.as_mut_ptr();
    while p < beg_last_token {
        if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            *p = ' ' as i32 as libc::c_char;
        }
        p = p.offset(1);
        p;
    }
    *p = '\0' as i32 as libc::c_char;
    fprintf(
        stderr,
        b"%s^ here\n\0" as *const u8 as *const libc::c_char,
        cur_line_str.as_mut_ptr(),
    );
    longjmp(jumper.as_mut_ptr(), 1 as libc::c_int);
}
