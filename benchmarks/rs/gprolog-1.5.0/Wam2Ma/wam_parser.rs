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
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn Source_Line(line_no: libc::c_int, cmt: *mut libc::c_char);
    fn F_file_name(arg_0: *mut ArgVal);
    fn F_predicate(arg_0: *mut ArgVal);
    fn F_directive(arg_0: *mut ArgVal);
    fn F_ensure_linked(arg_0: *mut ArgVal);
    fn F_get_variable(arg_0: *mut ArgVal);
    fn F_get_value(arg_0: *mut ArgVal);
    fn F_get_atom(arg_0: *mut ArgVal);
    fn F_get_integer(arg_0: *mut ArgVal);
    fn F_get_float(arg_0: *mut ArgVal);
    fn F_get_nil(arg_0: *mut ArgVal);
    fn F_get_list(arg_0: *mut ArgVal);
    fn F_get_structure(arg_0: *mut ArgVal);
    fn F_put_variable(arg_0: *mut ArgVal);
    fn F_put_void(arg_0: *mut ArgVal);
    fn F_put_value(arg_0: *mut ArgVal);
    fn F_put_unsafe_value(arg_0: *mut ArgVal);
    fn F_put_atom(arg_0: *mut ArgVal);
    fn F_put_integer(arg_0: *mut ArgVal);
    fn F_put_float(arg_0: *mut ArgVal);
    fn F_put_nil(arg_0: *mut ArgVal);
    fn F_put_list(arg_0: *mut ArgVal);
    fn F_put_structure(arg_0: *mut ArgVal);
    fn F_put_meta_term(arg_0: *mut ArgVal);
    fn F_math_load_value(arg_0: *mut ArgVal);
    fn F_math_fast_load_value(arg_0: *mut ArgVal);
    fn F_unify_variable(arg_0: *mut ArgVal);
    fn F_unify_void(arg_0: *mut ArgVal);
    fn F_unify_value(arg_0: *mut ArgVal);
    fn F_unify_local_value(arg_0: *mut ArgVal);
    fn F_unify_atom(arg_0: *mut ArgVal);
    fn F_unify_integer(arg_0: *mut ArgVal);
    fn F_unify_nil(arg_0: *mut ArgVal);
    fn F_unify_list(arg_0: *mut ArgVal);
    fn F_unify_structure(arg_0: *mut ArgVal);
    fn F_allocate(arg_0: *mut ArgVal);
    fn F_deallocate(arg_0: *mut ArgVal);
    fn F_call(arg_0: *mut ArgVal);
    fn F_execute(arg_0: *mut ArgVal);
    fn F_proceed(arg_0: *mut ArgVal);
    fn F_fail(arg_0: *mut ArgVal);
    fn F_label(arg_0: *mut ArgVal);
    fn F_switch_on_term(arg_0: *mut ArgVal);
    fn F_switch_on_atom(arg_0: *mut ArgVal);
    fn F_switch_on_integer(arg_0: *mut ArgVal);
    fn F_switch_on_structure(arg_0: *mut ArgVal);
    fn F_try_me_else(arg_0: *mut ArgVal);
    fn F_retry_me_else(arg_0: *mut ArgVal);
    fn F_trust_me_else_fail(arg_0: *mut ArgVal);
    fn F_try(arg_0: *mut ArgVal);
    fn F_retry(arg_0: *mut ArgVal);
    fn F_trust(arg_0: *mut ArgVal);
    fn F_pragma_arity(arg_0: *mut ArgVal);
    fn F_get_current_choice(arg_0: *mut ArgVal);
    fn F_cut(arg_0: *mut ArgVal);
    fn F_soft_cut(arg_0: *mut ArgVal);
    fn F_foreign_call_c(arg_0: *mut ArgVal);
    fn F_call_c(arg_0: *mut ArgVal);
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
pub type ArgTyp = libc::c_uint;
pub const LIST_INST: ArgTyp = 264;
pub const ANY: ArgTyp = 263;
pub const LABEL: ArgTyp = 262;
pub const MP_N: ArgTyp = 261;
pub const F_N: ArgTyp = 260;
pub const X_Y: ArgTyp = 259;
pub const FLOAT: ArgTyp = 258;
pub const INTEGER: ArgTyp = 257;
pub const ATOM: ArgTyp = 256;
pub type ArgVal = libc::c_double;
pub type PlLong = intptr_t;
pub type intptr_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseInf {
    pub keyword: *mut libc::c_char,
    pub fct: Option::<unsafe extern "C" fn() -> ()>,
    pub nb_args: libc::c_int,
    pub arg_type: [ArgTyp; 10],
}
pub type Bool = libc::c_int;
pub static mut decl: [ParseInf; 5] = unsafe {
    [
        {
            let mut init = ParseInf {
                keyword: b"file_name\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_file_name as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    ATOM,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"directive\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_directive as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 3 as libc::c_int,
                arg_type: [
                    INTEGER,
                    ATOM,
                    LIST_INST,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"predicate\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_predicate as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 7 as libc::c_int,
                arg_type: [
                    MP_N,
                    INTEGER,
                    ATOM,
                    ATOM,
                    ATOM,
                    ATOM,
                    LIST_INST,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"ensure_linked\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_ensure_linked as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    ((MP_N as libc::c_int) << 16 as libc::c_int | 0 as libc::c_int)
                        as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: 0 as *const libc::c_char as *mut libc::c_char,
                fct: None,
                nb_args: 0 as libc::c_int,
                arg_type: [
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
    ]
};
pub static mut inst: [ParseInf; 54] = unsafe {
    [
        {
            let mut init = ParseInf {
                keyword: b"get_variable\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_get_variable as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    X_Y,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"get_value\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_get_value as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    X_Y,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"get_atom\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_get_atom as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    ATOM,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"get_integer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_get_integer as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    INTEGER,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"get_float\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_get_float as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    FLOAT,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"get_nil\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_get_nil as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"get_list\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_get_list as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"get_structure\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_get_structure as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    F_N,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_variable\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_variable as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    X_Y,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_void\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_void as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_value\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_value as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    X_Y,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_unsafe_value\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_unsafe_value as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    X_Y,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_atom\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_atom as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    ATOM,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_integer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_integer as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    INTEGER,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_float\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_float as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    FLOAT,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_nil\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_nil as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_list\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_list as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_structure\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_structure as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    F_N,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"put_meta_term\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_put_meta_term as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    ATOM,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"math_load_value\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_math_load_value as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    X_Y,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"math_fast_load_value\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(
                    Some(
                        F_math_fast_load_value as unsafe extern "C" fn(*mut ArgVal) -> (),
                    ),
                ),
                nb_args: 2 as libc::c_int,
                arg_type: [
                    X_Y,
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"unify_variable\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_unify_variable as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    X_Y,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"unify_void\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_unify_void as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"unify_value\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_unify_value as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    X_Y,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"unify_local_value\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_unify_local_value as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    X_Y,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"unify_atom\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_unify_atom as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    ATOM,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"unify_integer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_unify_integer as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"unify_nil\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_unify_nil as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 0 as libc::c_int,
                arg_type: [
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"unify_list\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_unify_list as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 0 as libc::c_int,
                arg_type: [
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"unify_structure\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_unify_structure as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    F_N,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"allocate\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_allocate as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"deallocate\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_deallocate as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 0 as libc::c_int,
                arg_type: [
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"call\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_call as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    MP_N,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"execute\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_execute as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    MP_N,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"proceed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_proceed as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 0 as libc::c_int,
                arg_type: [
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"fail\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_fail as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 0 as libc::c_int,
                arg_type: [
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"label\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_label as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"switch_on_term\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_switch_on_term as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 5 as libc::c_int,
                arg_type: [
                    LABEL,
                    LABEL,
                    LABEL,
                    LABEL,
                    LABEL,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"switch_on_atom\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_switch_on_atom as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    ((ATOM as libc::c_int) << 16 as libc::c_int | INTEGER as libc::c_int)
                        as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"switch_on_integer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_switch_on_integer as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    ((INTEGER as libc::c_int) << 16 as libc::c_int
                        | INTEGER as libc::c_int) as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"switch_on_structure\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(
                    Some(
                        F_switch_on_structure as unsafe extern "C" fn(*mut ArgVal) -> (),
                    ),
                ),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    ((F_N as libc::c_int) << 16 as libc::c_int | INTEGER as libc::c_int)
                        as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"try_me_else\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_try_me_else as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"retry_me_else\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_retry_me_else as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"trust_me_else_fail\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_trust_me_else_fail as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 0 as libc::c_int,
                arg_type: [
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"try\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_try as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"retry\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_retry as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"trust\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_trust as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"pragma_arity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_pragma_arity as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    INTEGER,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"get_current_choice\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_get_current_choice as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    X_Y,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"cut\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_cut as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    X_Y,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"soft_cut\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_soft_cut as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 1 as libc::c_int,
                arg_type: [
                    X_Y,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"call_c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_call_c as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 3 as libc::c_int,
                arg_type: [
                    ATOM,
                    ((ANY as libc::c_int) << 16 as libc::c_int | 0 as libc::c_int)
                        as ArgTyp,
                    ((ANY as libc::c_int) << 16 as libc::c_int | 0 as libc::c_int)
                        as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: b"foreign_call_c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut ArgVal) -> ()>,
                    Option::<unsafe extern "C" fn() -> ()>,
                >(Some(F_foreign_call_c as unsafe extern "C" fn(*mut ArgVal) -> ())),
                nb_args: 5 as libc::c_int,
                arg_type: [
                    ATOM,
                    ATOM,
                    F_N,
                    INTEGER,
                    ((ATOM as libc::c_int) << 16 as libc::c_int | ATOM as libc::c_int)
                        as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
        {
            let mut init = ParseInf {
                keyword: 0 as *const libc::c_char as *mut libc::c_char,
                fct: None,
                nb_args: 0 as libc::c_int,
                arg_type: [
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                    0 as ArgTyp,
                ],
            };
            init
        },
    ]
};
pub static mut arg: [ArgVal; 65536] = [0.; 65536];
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
pub unsafe extern "C" fn Parse_Wam_File(
    mut file_name_in: *mut libc::c_char,
    mut comment: libc::c_int,
) -> libc::c_int {
    let mut ret_val: libc::c_int = 0;
    keep_source_lines = comment;
    if file_name_in.is_null() {
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
    cur_line_p = cur_line_str.as_mut_ptr();
    cur_line_str[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    cur_line_no = 0 as libc::c_int;
    ret_val = _setjmp(jumper.as_mut_ptr());
    if ret_val == 0 as libc::c_int {
        Parser();
    }
    if file_in != stdin {
        fclose(file_in);
    }
    return (ret_val == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn Parser() {
    while !(Parse_And_Treat_Decl_Or_Inst(decl.as_mut_ptr()) == 0) {
        Read_Token('.' as i32);
    }
}
unsafe extern "C" fn Parse_And_Treat_Decl_Or_Inst(
    mut what: *mut ParseInf,
) -> libc::c_int {
    let mut in_0: *mut ParseInf = what;
    let mut top: *mut ArgVal = arg.as_mut_ptr();
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut fct_called: Bool = 0 as libc::c_int;
    k = Scanner(0 as libc::c_int);
    if k == 0 as libc::c_int && what == decl.as_mut_ptr() {
        return 0 as libc::c_int;
    }
    if k != ATOM as libc::c_int {
        Syntax_Error(
            (if what == decl.as_mut_ptr() {
                b"wam declaration expected\0" as *const u8 as *const libc::c_char
            } else {
                b"wam instruction expected\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
        );
    }
    in_0 = what;
    while !((*in_0).keyword).is_null()
        && strcmp(str_val.as_mut_ptr(), (*in_0).keyword) != 0 as libc::c_int
    {
        in_0 = in_0.offset(1);
        in_0;
    }
    if ((*in_0).keyword).is_null() {
        Syntax_Error(
            (if what == decl.as_mut_ptr() {
                b"unknown wam declaration\0" as *const u8 as *const libc::c_char
            } else {
                b"unknown wam instruction\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
        );
    }
    if (*in_0).nb_args != 0 {
        Read_Token('(' as i32);
        i = 0 as libc::c_int;
        while i < (*in_0).nb_args {
            if i > 0 as libc::c_int {
                Read_Token(',' as i32);
            }
            if (*in_0).arg_type[i as usize] as libc::c_uint
                == LIST_INST as libc::c_int as libc::c_uint
            {
                ::std::mem::transmute::<
                    _,
                    fn(_),
                >((Some(((*in_0).fct).unwrap())).unwrap())(arg.as_mut_ptr());
                fct_called = 1 as libc::c_int;
                Read_Token('[' as i32);
                loop {
                    Parse_And_Treat_Decl_Or_Inst(inst.as_mut_ptr());
                    k = Scanner(0 as libc::c_int);
                    if k == ']' as i32 {
                        break;
                    }
                    if k != ',' as i32 {
                        Syntax_Error(
                            b"] or , expected\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                }
            } else {
                Read_Argument((*in_0).arg_type[i as usize], &mut top);
            }
            i += 1;
            i;
        }
        Read_Token(')' as i32);
    }
    if fct_called == 0 {
        ::std::mem::transmute::<
            _,
            fn(_),
        >((Some(((*in_0).fct).unwrap())).unwrap())(arg.as_mut_ptr());
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Read_Argument(mut arg_type: ArgTyp, mut top: *mut *mut ArgVal) {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut top1: *mut ArgVal = 0 as *mut ArgVal;
    let mut t1: ArgTyp = 0 as ArgTyp;
    let mut t2: ArgTyp = 0 as ArgTyp;
    's_256: {
        let mut current_block_59: u64;
        match arg_type as libc::c_uint {
            256 => {
                Read_Token(ATOM as libc::c_int);
                let ref mut fresh0 = *(*top as *mut *mut libc::c_char);
                *fresh0 = strdup(str_val.as_mut_ptr());
                *top = (*top).offset(1);
                *top;
                return;
            }
            257 => {
                Read_Token(INTEGER as libc::c_int);
                current_block_59 = 1773932184051511839;
            }
            258 => {
                Read_Token(FLOAT as libc::c_int);
                current_block_59 = 6031847643693701641;
            }
            259 => {
                if Scanner(0 as libc::c_int) != ATOM as libc::c_int
                    || *str_val.as_mut_ptr() as libc::c_int != 'x' as i32
                        && *str_val.as_mut_ptr() as libc::c_int != 'y' as i32
                    || str_val[1 as libc::c_int as usize] as libc::c_int != '\0' as i32
                {
                    Syntax_Error(
                        b"x(...) or y(...) expected\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                current_block_59 = 5634248450439885685;
            }
            260 => {
                Read_Argument(ATOM, top);
                Read_Token('/' as i32);
                Read_Argument(INTEGER, top);
                return;
            }
            261 => {
                Read_Token(ATOM as libc::c_int);
                k = Scanner(0 as libc::c_int);
                if k == ':' as i32 {
                    let ref mut fresh1 = *(*top as *mut *mut libc::c_char);
                    *fresh1 = strdup(str_val.as_mut_ptr());
                    *top = (*top).offset(1);
                    *top;
                    Read_Token(ATOM as libc::c_int);
                    let ref mut fresh2 = *(*top as *mut *mut libc::c_char);
                    *fresh2 = strdup(str_val.as_mut_ptr());
                    *top = (*top).offset(1);
                    *top;
                    Read_Token('/' as i32);
                } else if k == '/' as i32 {
                    let ref mut fresh3 = *(*top as *mut *mut libc::c_char);
                    *fresh3 = 0 as *mut libc::c_char;
                    *top = (*top).offset(1);
                    *top;
                    let ref mut fresh4 = *(*top as *mut *mut libc::c_char);
                    *fresh4 = strdup(str_val.as_mut_ptr());
                    *top = (*top).offset(1);
                    *top;
                } else {
                    Syntax_Error(
                        b"/ or : expected\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                Read_Argument(INTEGER, top);
                return;
            }
            262 => {
                k = Scanner(0 as libc::c_int);
                if k != INTEGER as libc::c_int {
                    if k != ATOM as libc::c_int
                        || strcmp(
                            str_val.as_mut_ptr(),
                            b"fail\0" as *const u8 as *const libc::c_char,
                        ) != 0 as libc::c_int
                    {
                        Syntax_Error(
                            b"label or fail expected\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    } else {
                        int_val = -(1 as libc::c_int) as PlLong;
                    }
                }
                *(*top as *mut PlLong) = int_val;
                *top = (*top).offset(1);
                *top;
                return;
            }
            263 => {
                t1 = Scanner(1 as libc::c_int) as ArgTyp;
                top1 = *top;
                *(*top as *mut PlLong) = t1 as PlLong;
                *top = (*top).offset(1);
                *top;
                if t1 as libc::c_uint == INTEGER as libc::c_int as libc::c_uint {
                    current_block_59 = 1773932184051511839;
                } else if t1 as libc::c_uint == FLOAT as libc::c_int as libc::c_uint {
                    current_block_59 = 6031847643693701641;
                } else {
                    if t1 as libc::c_uint != ATOM as libc::c_int as libc::c_uint {
                        Syntax_Error(
                            b"x(...), y(...), atom, integer or float expected\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    if (*str_val.as_mut_ptr() as libc::c_int == 'x' as i32
                        || *str_val.as_mut_ptr() as libc::c_int == 'y' as i32)
                        && str_val[1 as libc::c_int as usize] as libc::c_int
                            == '\0' as i32
                        && Peek_Char(0 as libc::c_int) as libc::c_int == '(' as i32
                    {
                        *(top1 as *mut PlLong) = X_Y as libc::c_int as PlLong;
                        top1 = top1.offset(1);
                        top1;
                    } else {
                        let ref mut fresh5 = *(*top as *mut *mut libc::c_char);
                        *fresh5 = strdup(str_val.as_mut_ptr());
                        *top = (*top).offset(1);
                        *top;
                        if Peek_Char(1 as libc::c_int) as libc::c_int == '/' as i32 {
                            Read_Token('/' as i32);
                            Read_Argument(INTEGER, top);
                            *(top1 as *mut PlLong) = F_N as libc::c_int as PlLong;
                            top1 = top1.offset(1);
                            top1;
                        }
                        return;
                    }
                    current_block_59 = 5634248450439885685;
                }
            }
            264 => {
                fprintf(
                    stderr,
                    b"BAD Read_Argument(LIST_INST) !!!\n\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            _ => {
                break 's_256;
            }
        }
        match current_block_59 {
            1773932184051511839 => {
                *(*top as *mut PlLong) = int_val;
                *top = (*top).offset(1);
                *top;
                return;
            }
            6031847643693701641 => {
                *(*top as *mut libc::c_double) = dbl_val;
                *top = (*top).offset(1);
                *top;
                return;
            }
            _ => {
                Read_Token('(' as i32);
                Read_Token(INTEGER as libc::c_int);
                Read_Token(')' as i32);
                if *str_val.as_mut_ptr() as libc::c_int == 'x' as i32 {
                    *(*top as *mut PlLong) = int_val;
                    *top = (*top).offset(1);
                    *top;
                } else {
                    *(*top
                        as *mut PlLong) = 5000 as libc::c_int as libc::c_long + int_val;
                    *top = (*top).offset(1);
                    *top;
                }
                return;
            }
        }
    }
    t1 = (arg_type as libc::c_uint >> 16 as libc::c_int) as ArgTyp;
    t2 = (arg_type as libc::c_uint
        & (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        as ArgTyp;
    top1 = *top;
    *(*top as *mut PlLong) = 0 as libc::c_int as PlLong;
    *top = (*top).offset(1);
    *top;
    n = 0 as libc::c_int;
    k = Scanner(1 as libc::c_int);
    if k == ATOM as libc::c_int
        && strcmp(str_val.as_mut_ptr(), b"[]\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        return;
    }
    if k != '[' as i32 {
        Syntax_Error(
            b"[] or [ expected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    loop {
        n += 1;
        n;
        if t2 as libc::c_uint == 0 as libc::c_int as libc::c_uint {
            Read_Argument(t1, top);
        } else {
            Read_Token('(' as i32);
            Read_Argument(t1, top);
            Read_Token(',' as i32);
            Read_Argument(t2, top);
            Read_Token(')' as i32);
        }
        k = Scanner(0 as libc::c_int);
        if k == ']' as i32 {
            break;
        }
        if k != ',' as i32 {
            Syntax_Error(
                b"] or , expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    *(top1 as *mut PlLong) = n as PlLong;
    top1 = top1.offset(1);
    top1;
}
unsafe extern "C" fn Read_Token(mut what: libc::c_int) {
    let mut str: [libc::c_char; 80] = [0; 80];
    let mut k: libc::c_int = 0;
    k = Scanner((what == ATOM as libc::c_int) as libc::c_int);
    if k == what {
        return;
    }
    if what >= 256 as libc::c_int && k == '(' as i32 {
        Read_Token(what);
        Read_Token(')' as i32);
        return;
    }
    match what {
        256 => {
            Syntax_Error(
                b"atom expected\0" as *const u8 as *const libc::c_char
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
        _ => {
            sprintf(
                str.as_mut_ptr(),
                b"%c expected\0" as *const u8 as *const libc::c_char,
                what,
            );
            Syntax_Error(str.as_mut_ptr());
        }
    };
}
unsafe extern "C" fn Scanner(mut complex_atom: libc::c_int) -> libc::c_int {
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
            && *cur_line_p as libc::c_int != '%' as i32
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
                Source_Line(cur_line_no, cur_line_p);
            }
        }
    }
    beg_last_token = cur_line_p;
    if *cur_line_p as libc::c_int == '\'' as i32 {
        p = str_val.as_mut_ptr();
        cur_line_p = cur_line_p.offset(1);
        cur_line_p;
        while *cur_line_p as libc::c_int != '\'' as i32
            || *cur_line_p.offset(1 as libc::c_int as isize) as libc::c_int
                == '\'' as i32
        {
            if *cur_line_p as libc::c_int == '\'' as i32 {
                let fresh6 = p;
                p = p.offset(1);
                *fresh6 = '\'' as i32 as libc::c_char;
                cur_line_p = cur_line_p.offset(2 as libc::c_int as isize);
            } else if *cur_line_p as libc::c_int == '"' as i32 {
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = '\\' as i32 as libc::c_char;
                let fresh8 = p;
                p = p.offset(1);
                *fresh8 = '"' as i32 as libc::c_char;
                cur_line_p = cur_line_p.offset(1);
                cur_line_p;
            } else {
                let fresh9 = cur_line_p;
                cur_line_p = cur_line_p.offset(1);
                let fresh10 = p;
                p = p.offset(1);
                *fresh10 = *fresh9;
                if *fresh10 as libc::c_int == '\\' as i32 {
                    if *cur_line_p as libc::c_int == '\\' as i32
                        || !(strchr(
                            b"abfnrtv\0" as *const u8 as *const libc::c_char,
                            *cur_line_p as libc::c_int,
                        ))
                            .is_null()
                    {
                        let fresh11 = cur_line_p;
                        cur_line_p = cur_line_p.offset(1);
                        let fresh12 = p;
                        p = p.offset(1);
                        *fresh12 = *fresh11;
                    } else {
                        if *cur_line_p as libc::c_int == 'x' as i32 {
                            cur_line_p = cur_line_p.offset(1);
                            cur_line_p;
                            i = 16 as libc::c_int as PlLong;
                        } else {
                            i = 8 as libc::c_int as PlLong;
                        }
                        i = strtol(cur_line_p, &mut p1, i as libc::c_int);
                        cur_line_p = p1.offset(1 as libc::c_int as isize);
                        sprintf(p, b"%03lo\0" as *const u8 as *const libc::c_char, i);
                        p = p.offset(3 as libc::c_int as isize);
                    }
                }
            }
        }
        cur_line_p = cur_line_p.offset(1);
        cur_line_p;
        *p = '\0' as i32 as libc::c_char;
        return ATOM as libc::c_int;
    }
    if *(*__ctype_b_loc()).offset(*cur_line_p as libc::c_int as isize) as libc::c_int
        & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
        || *cur_line_p as libc::c_int == '_' as i32
    {
        p = str_val.as_mut_ptr();
        while *(*__ctype_b_loc()).offset(*cur_line_p as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            != 0 || *cur_line_p as libc::c_int == '_' as i32
        {
            let fresh13 = cur_line_p;
            cur_line_p = cur_line_p.offset(1);
            let fresh14 = p;
            p = p.offset(1);
            *fresh14 = *fresh13;
        }
        *p = '\0' as i32 as libc::c_char;
        return ATOM as libc::c_int;
    }
    if complex_atom != 0 {
        if *cur_line_p.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32
            && *cur_line_p.offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
            || *cur_line_p.offset(0 as libc::c_int as isize) as libc::c_int == '{' as i32
                && *cur_line_p.offset(1 as libc::c_int as isize) as libc::c_int
                    == '}' as i32
        {
            let fresh15 = cur_line_p;
            cur_line_p = cur_line_p.offset(1);
            str_val[0 as libc::c_int as usize] = *fresh15;
            let fresh16 = cur_line_p;
            cur_line_p = cur_line_p.offset(1);
            str_val[1 as libc::c_int as usize] = *fresh16;
            str_val[2 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            return ATOM as libc::c_int;
        }
        if !(strchr(
            b"#$&*+-./:<=>?@\\^~\0" as *const u8 as *const libc::c_char,
            *cur_line_p as libc::c_int,
        ))
            .is_null()
        {
            p = str_val.as_mut_ptr();
            loop {
                if *cur_line_p as libc::c_int == '"' as i32
                    || *cur_line_p as libc::c_int == '\\' as i32
                {
                    let fresh17 = p;
                    p = p.offset(1);
                    *fresh17 = '\\' as i32 as libc::c_char;
                }
                let fresh18 = cur_line_p;
                cur_line_p = cur_line_p.offset(1);
                let fresh19 = p;
                p = p.offset(1);
                *fresh19 = *fresh18;
                if (strchr(
                    b"#$&*+-./:<=>?@\\^~\0" as *const u8 as *const libc::c_char,
                    *cur_line_p as libc::c_int,
                ))
                    .is_null()
                {
                    break;
                }
            }
            *p = '\0' as i32 as libc::c_char;
            return ATOM as libc::c_int;
        }
        if !(strchr(
            b"!;,\0" as *const u8 as *const libc::c_char,
            *cur_line_p as libc::c_int,
        ))
            .is_null()
        {
            let fresh20 = cur_line_p;
            cur_line_p = cur_line_p.offset(1);
            str_val[0 as libc::c_int as usize] = *fresh20;
            str_val[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            return ATOM as libc::c_int;
        }
    }
    i = strtol(cur_line_p, &mut p, 0 as libc::c_int);
    if p == cur_line_p {
        let fresh21 = cur_line_p;
        cur_line_p = cur_line_p.offset(1);
        return *fresh21 as libc::c_int;
    }
    d = strtod(cur_line_p, &mut p1);
    if p1 == p {
        int_val = i;
        cur_line_p = p;
        return INTEGER as libc::c_int;
    }
    dbl_val = d;
    cur_line_p = p1;
    return FLOAT as libc::c_int;
}
unsafe extern "C" fn Peek_Char(mut skip_spaces: libc::c_int) -> libc::c_char {
    let mut p: *mut libc::c_char = cur_line_p;
    if skip_spaces != 0 {
        while *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            p = p.offset(1);
            p;
        }
    }
    return *p;
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
