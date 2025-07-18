use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type table_entry;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn error_at_line(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __fname: *const libc::c_char,
        __lineno: libc::c_uint,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut debug: libc::c_int;
    static mut preprocess_option: libc::c_int;
    fn lookup(_: *const libc::c_char) -> *mut Symbol;
    fn install(_: *mut libc::c_char, _: libc::c_int) -> *mut Symbol;
    fn delete_statics();
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
pub type ptrdiff_t = libc::c_long;
pub type int16_t = __int16_t;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list_entry {
    pub next: *mut linked_list_entry,
    pub prev: *mut linked_list_entry,
    pub list: *mut linked_list,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linked_list {
    pub free_data: linked_list_free_data_fp,
    pub head: *mut linked_list_entry,
    pub tail: *mut linked_list_entry,
}
pub type linked_list_free_data_fp = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type symtype = libc::c_uint;
pub const SymIdentifier: symtype = 2;
pub const SymToken: symtype = 1;
pub const SymUndefined: symtype = 0;
pub type storage = libc::c_uint;
pub const AnyStorage: storage = 4;
pub const AutoStorage: storage = 3;
pub const StaticStorage: storage = 2;
pub const ExplicitExternStorage: storage = 1;
pub const ExternStorage: storage = 0;
pub type symbol_flag = libc::c_uint;
pub const symbol_alias: symbol_flag = 5;
pub const symbol_parm: symbol_flag = 4;
pub const symbol_local: symbol_flag = 3;
pub const symbol_target: symbol_flag = 2;
pub const symbol_start: symbol_flag = 1;
pub const symbol_none: symbol_flag = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol {
    pub owner: *mut table_entry,
    pub next: *mut Symbol,
    pub entry: *mut linked_list_entry,
    pub type_0: symtype,
    pub name: *mut libc::c_char,
    pub flag: symbol_flag,
    pub alias: *mut symbol,
    pub active: libc::c_int,
    pub expand_line: libc::c_int,
    pub visible: libc::c_int,
    pub token_type: libc::c_int,
    pub source: *mut libc::c_char,
    pub def_line: libc::c_int,
    pub ref_line: *mut linked_list,
    pub level: libc::c_int,
    pub decl: *mut libc::c_char,
    pub storage: storage,
    pub arity: libc::c_int,
    pub recursive: libc::c_int,
    pub ord: size_t,
    pub caller: *mut linked_list,
    pub callee: *mut linked_list,
}
pub type Symbol = symbol;
pub type flex_int16_t = int16_t;
pub type yy_state_type = libc::c_int;
pub type flex_uint16_t = uint16_t;
pub type YY_CHAR = libc::c_uchar;
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: yy_size_t,
    pub yy_n_chars: yy_size_t,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_bs_lineno: libc::c_int,
    pub yy_bs_column: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
pub type yy_size_t = size_t;
pub const RBRACE0: C2RustUnnamed_3 = 259;
pub const LBRACE0: C2RustUnnamed_3 = 258;
pub const STRING: C2RustUnnamed_3 = 271;
pub const WORD: C2RustUnnamed_3 = 257;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YYSTYPE {
    pub str_0: *mut libc::c_char,
}
pub const IDENTIFIER: C2RustUnnamed_3 = 260;
pub const STRUCT: C2RustUnnamed_3 = 264;
pub const OP: C2RustUnnamed_3 = 266;
pub const MEMBER_OF: C2RustUnnamed_3 = 269;
pub const MODIFIER: C2RustUnnamed_3 = 265;
pub const TYPEDEF: C2RustUnnamed_3 = 263;
pub const STATIC: C2RustUnnamed_3 = 262;
pub const EXTERN: C2RustUnnamed_3 = 261;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const QUALIFIER: C2RustUnnamed_3 = 273;
pub const TYPE: C2RustUnnamed_3 = 270;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PARM_WRAPPER: C2RustUnnamed_3 = 272;
pub const ENUM: C2RustUnnamed_3 = 268;
pub const UNION: C2RustUnnamed_3 = 267;
pub const RBRACE: C2RustUnnamed_3 = 125;
pub const LBRACE: C2RustUnnamed_3 = 123;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut string_stk: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const libc::c_char as *mut libc::c_char,
    next_free: 0 as *const libc::c_char as *mut libc::c_char,
    chunk_limit: 0 as *const libc::c_char as *mut libc::c_char,
    temp: C2RustUnnamed_1 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_0 { plain: None },
    freefun: C2RustUnnamed { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
pub static mut line_num: libc::c_int = 0;
pub static mut filename: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut canonical_filename: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut yylval: YYSTYPE = YYSTYPE {
    str_0: 0 as *const libc::c_char as *mut libc::c_char,
};
pub static mut input_file_count: libc::c_uint = 0;
static mut prev_token: libc::c_int = 0;
static mut keywords: [*mut libc::c_char; 13] = [
    b"break\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"case\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"continue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"do\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"else\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"for\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"goto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"if\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"return\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sizeof\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"switch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"while\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut types: [*mut libc::c_char; 5] = [
    b"char\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"double\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"float\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"int\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"void\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut qualifiers: [*mut libc::c_char; 9] = [
    b"long\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"const\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"register\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"restrict\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"short\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"signed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"unsigned\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"volatile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"inline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub unsafe extern "C" fn init_tokens() {
    let mut i: libc::c_int = 0;
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*mut libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        sp = install(keywords[i as usize], 0x1 as libc::c_int);
        (*sp).type_0 = SymToken;
        (*sp).token_type = WORD as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*mut libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        sp = install(types[i as usize], 0x1 as libc::c_int);
        (*sp).type_0 = SymToken;
        (*sp).token_type = TYPE as libc::c_int;
        (*sp).source = 0 as *mut libc::c_char;
        (*sp).def_line = -(1 as libc::c_int);
        (*sp).ref_line = 0 as *mut linked_list;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*mut libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        sp = install(qualifiers[i as usize], 0x1 as libc::c_int);
        (*sp).type_0 = SymToken;
        (*sp).token_type = QUALIFIER as libc::c_int;
        (*sp).source = 0 as *mut libc::c_char;
        (*sp).def_line = -(1 as libc::c_int);
        (*sp).ref_line = 0 as *mut linked_list;
        i += 1;
        i;
    }
    sp = install(
        b"...\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0x1 as libc::c_int,
    );
    (*sp).type_0 = SymToken;
    (*sp).token_type = IDENTIFIER as libc::c_int;
    (*sp).source = 0 as *mut libc::c_char;
    (*sp).def_line = -(1 as libc::c_int);
    (*sp).ref_line = 0 as *mut linked_list;
}
pub unsafe extern "C" fn init_lex(mut debug_level: libc::c_int) {
    yy_flex_debug = debug_level;
    _obstack_begin(
        &mut string_stk,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    init_tokens();
}
pub unsafe extern "C" fn ident() -> libc::c_int {
    if prev_token != STRUCT as libc::c_int {
        let mut sp: *mut Symbol = lookup(yytext);
        if !sp.is_null()
            && (*sp).type_0 as libc::c_uint == SymToken as libc::c_int as libc::c_uint
        {
            yylval.str_0 = (*sp).name;
            return (*sp).token_type;
        }
    }
    let mut __o: *mut obstack = &mut string_stk;
    let mut __len: size_t = yyleng;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    memcpy((*__o).next_free as *mut libc::c_void, yytext as *const libc::c_void, __len);
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    let mut __o_0: *mut obstack = &mut string_stk;
    if ({
        let mut __o1: *const obstack = __o_0;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o_0, 1 as libc::c_int as size_t);
    }
    let fresh0 = (*__o_0).next_free;
    (*__o_0).next_free = ((*__o_0).next_free).offset(1);
    *fresh0 = 0 as libc::c_int as libc::c_char;
    yylval
        .str_0 = ({
        let mut __o1: *mut obstack = &mut string_stk as *mut obstack;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut libc::c_char {
            (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
        }
        (*__o1)
            .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        {
            (*__o1).object_base
        } else {
            0 as *mut libc::c_char
        })
            .offset(
                ((((*__o1).next_free)
                    .offset_from(
                        (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut libc::c_char
                        }),
                    ) as libc::c_long as libc::c_ulong)
                    .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                    as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
            as libc::c_long as size_t
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *mut libc::c_char;
    return IDENTIFIER as libc::c_int;
}
pub static mut pp_bin: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut pp_opts: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut opt_stack: *mut obstack = 0 as *const obstack as *mut obstack;
pub unsafe extern "C" fn set_preprocessor(mut arg: *const libc::c_char) {
    pp_bin = if !arg.is_null() { xstrdup(arg) } else { 0 as *mut libc::c_char };
}
pub unsafe extern "C" fn pp_option(mut arg: *const libc::c_char) {
    if opt_stack.is_null() {
        if pp_bin.is_null() {
            pp_bin = b"/usr/bin/cpp\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        opt_stack = xmalloc(::std::mem::size_of::<obstack>() as libc::c_ulong)
            as *mut obstack;
        _obstack_begin(
            opt_stack,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    let mut __o: *mut obstack = opt_stack;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o, 1 as libc::c_int as size_t);
    }
    let fresh1 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh1 = ' ' as i32 as libc::c_char;
    let mut __o_0: *mut obstack = opt_stack;
    let mut __len: size_t = strlen(arg);
    if ({
        let mut __o1: *const obstack = __o_0;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o_0, __len);
    }
    memcpy((*__o_0).next_free as *mut libc::c_void, arg as *const libc::c_void, __len);
    (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
}
pub unsafe extern "C" fn pp_finalize() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut __o: *mut obstack = opt_stack;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o, 1 as libc::c_int as size_t);
    }
    let fresh2 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh2 = 0 as libc::c_int as libc::c_char;
    s = ({
        let mut __o1: *mut obstack = opt_stack;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut libc::c_char {
            (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
        }
        (*__o1)
            .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        {
            (*__o1).object_base
        } else {
            0 as *mut libc::c_char
        })
            .offset(
                ((((*__o1).next_free)
                    .offset_from(
                        (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut libc::c_char
                        }),
                    ) as libc::c_long as libc::c_ulong)
                    .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                    as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
            as libc::c_long as size_t
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *mut libc::c_char;
    if pp_opts.is_null() {
        pp_opts = xstrdup(s);
    } else {
        pp_opts = xrealloc(
            pp_opts as *mut libc::c_void,
            (strlen(pp_opts))
                .wrapping_add(strlen(s))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcat(pp_opts, s);
    }
    let mut __o_0: *mut obstack = opt_stack;
    let mut __obj: *mut libc::c_void = s as *mut libc::c_void;
    if __obj > (*__o_0).chunk as *mut libc::c_void
        && __obj < (*__o_0).chunk_limit as *mut libc::c_void
    {
        (*__o_0).object_base = __obj as *mut libc::c_char;
        (*__o_0).next_free = (*__o_0).object_base;
    } else {
        _obstack_free(__o_0, __obj);
    }
    free(opt_stack as *mut libc::c_void);
    opt_stack = 0 as *mut obstack;
}
pub unsafe extern "C" fn pp_open(mut name: *const libc::c_char) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    if !opt_stack.is_null() {
        pp_finalize();
    }
    size = (strlen(pp_bin))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(strlen(name))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if !pp_opts.is_null() {
        size = (size as libc::c_ulong).wrapping_add(strlen(pp_opts)) as size_t as size_t;
    }
    s = xmalloc(size) as *mut libc::c_char;
    strcpy(s, pp_bin);
    if !pp_opts.is_null() {
        strcat(s, pp_opts);
    }
    strcat(s, b" \0" as *const u8 as *const libc::c_char);
    strcat(s, name);
    if debug != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Command line: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            s,
        );
    }
    fp = popen(s, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot execute `%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            s,
        );
    }
    free(s as *mut libc::c_void);
    return fp;
}
static mut yy_buffer_stack_top: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack_max: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack: *mut YY_BUFFER_STATE = 0 as *const YY_BUFFER_STATE
    as *mut YY_BUFFER_STATE;
pub unsafe extern "C" fn pp_close(mut fp: *mut FILE) {
    pclose(fp);
}
pub unsafe extern "C" fn yywrap() -> libc::c_int {
    if yyin.is_null() {
        return 1 as libc::c_int;
    }
    if preprocess_option != 0 {
        pp_close(yyin);
    } else {
        fclose(yyin);
    }
    yyin = 0 as *mut FILE;
    yy_delete_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
    );
    delete_statics();
    return 1 as libc::c_int;
}
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: yy_size_t = 0;
pub static mut yyleng: yy_size_t = 0;
static mut yy_c_buf_p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut yy_init: libc::c_int = 0 as libc::c_int;
static mut yy_start: libc::c_int = 0 as libc::c_int;
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
static mut hit_eof: libc::c_int = 0;
pub unsafe extern "C" fn get_token() -> libc::c_int {
    let mut tok: libc::c_int = 0;
    if hit_eof != 0 {
        tok = 0 as libc::c_int;
    } else {
        tok = yylex();
        prev_token = tok;
        if tok == 0 {
            hit_eof = 1 as libc::c_int;
        }
    }
    return tok;
}
pub unsafe extern "C" fn source(mut name: *mut libc::c_char) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open `%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
        return 1 as libc::c_int;
    }
    if preprocess_option != 0 {
        fclose(fp);
        fp = pp_open(name);
        if fp.is_null() {
            return 1 as libc::c_int;
        }
    }
    let mut __o: *mut obstack = &mut string_stk;
    let mut __len: size_t = (strlen(name))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    memcpy((*__o).next_free as *mut libc::c_void, name as *const libc::c_void, __len);
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    filename = ({
        let mut __o1: *mut obstack = &mut string_stk as *mut obstack;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut libc::c_char {
            (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
        }
        (*__o1)
            .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        {
            (*__o1).object_base
        } else {
            0 as *mut libc::c_char
        })
            .offset(
                ((((*__o1).next_free)
                    .offset_from(
                        (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut libc::c_char
                        }),
                    ) as libc::c_long as libc::c_ulong)
                    .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                    as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
            as libc::c_long as size_t
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *mut libc::c_char;
    canonical_filename = filename;
    line_num = 1 as libc::c_int;
    input_file_count = input_file_count.wrapping_add(1);
    input_file_count;
    hit_eof = 0 as libc::c_int;
    yyrestart(fp);
    return 0 as libc::c_int;
}
unsafe extern "C" fn getnum(
    mut base: libc::c_uint,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    n = 0 as libc::c_int;
    while count != 0 {
        c = input();
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            i = (c - '0' as i32) as libc::c_uint;
        } else {
            i = (({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = c;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(c);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(c as isize);
                }
                __res
            }) - 'A' as i32 + 10 as libc::c_int) as libc::c_uint;
        }
        if i > base {
            yyunput(c, yytext);
            break;
        } else {
            n = (n as libc::c_uint).wrapping_mul(base).wrapping_add(i) as libc::c_int;
            count -= 1;
            count;
        }
    }
    return n;
}
pub static mut yyin: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut yyout: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut yylineno: libc::c_int = 1 as libc::c_int;
pub unsafe extern "C" fn backslash() -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = input();
    match c {
        97 => return '\u{7}' as i32,
        98 => return '\u{8}' as i32,
        102 => return '\u{c}' as i32,
        110 => return '\n' as i32,
        114 => return '\r' as i32,
        116 => return '\t' as i32,
        120 => return getnum(16 as libc::c_int as libc::c_uint, 2 as libc::c_int),
        48 => return getnum(8 as libc::c_int as libc::c_uint, 3 as libc::c_int),
        _ => {}
    }
    return c;
}
pub unsafe extern "C" fn update_loc() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = (strchr(yytext, '#' as i32)).offset(1 as libc::c_int as isize);
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        p = p.offset(1);
        p;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'l' as i32 {
        p = p.offset(4 as libc::c_int as isize);
    }
    line_num = strtoul(p, &mut p, 10 as libc::c_int) as libc::c_int;
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        p = p.offset(1);
        p;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32 {
        let mut n: libc::c_int = 0;
        p = p.offset(1);
        p;
        n = 0 as libc::c_int;
        while *p.offset(n as isize) as libc::c_int != 0
            && *p.offset(n as isize) as libc::c_int != '"' as i32
        {
            n += 1;
            n;
        }
        let mut __o: *mut obstack = &mut string_stk;
        let mut __len: size_t = n as size_t;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        memcpy((*__o).next_free as *mut libc::c_void, p as *const libc::c_void, __len);
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        let mut __o_0: *mut obstack = &mut string_stk;
        if ({
            let mut __o1: *const obstack = __o_0;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < 1 as libc::c_int as libc::c_ulong
        {
            _obstack_newchunk(__o_0, 1 as libc::c_int as size_t);
        }
        let fresh3 = (*__o_0).next_free;
        (*__o_0).next_free = ((*__o_0).next_free).offset(1);
        *fresh3 = 0 as libc::c_int as libc::c_char;
        filename = ({
            let mut __o1: *mut obstack = &mut string_stk as *mut obstack;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            }),
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }) as *mut libc::c_char;
    }
    if debug > 1 as libc::c_int {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"New location: %s:%d\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            line_num,
        );
    }
}
static mut yy_accept: [flex_int16_t; 191] = [
    0 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
];
static mut yy_ec: [YY_CHAR; 256] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    9 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    10 as libc::c_int as YY_CHAR,
    11 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    12 as libc::c_int as YY_CHAR,
    13 as libc::c_int as YY_CHAR,
    14 as libc::c_int as YY_CHAR,
    15 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    16 as libc::c_int as YY_CHAR,
    17 as libc::c_int as YY_CHAR,
    17 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    18 as libc::c_int as YY_CHAR,
    19 as libc::c_int as YY_CHAR,
    20 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    22 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    24 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    25 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    26 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    27 as libc::c_int as YY_CHAR,
    21 as libc::c_int as YY_CHAR,
    28 as libc::c_int as YY_CHAR,
    29 as libc::c_int as YY_CHAR,
    30 as libc::c_int as YY_CHAR,
    31 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    32 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    33 as libc::c_int as YY_CHAR,
    34 as libc::c_int as YY_CHAR,
    35 as libc::c_int as YY_CHAR,
    36 as libc::c_int as YY_CHAR,
    37 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    38 as libc::c_int as YY_CHAR,
    39 as libc::c_int as YY_CHAR,
    40 as libc::c_int as YY_CHAR,
    41 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    42 as libc::c_int as YY_CHAR,
    43 as libc::c_int as YY_CHAR,
    23 as libc::c_int as YY_CHAR,
    44 as libc::c_int as YY_CHAR,
    45 as libc::c_int as YY_CHAR,
    46 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
];
static mut yy_meta: [YY_CHAR; 47] = [
    0 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    2 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    4 as libc::c_int as YY_CHAR,
    5 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    3 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    7 as libc::c_int as YY_CHAR,
    6 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    8 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
    1 as libc::c_int as YY_CHAR,
];
static mut yy_base: [flex_uint16_t; 212] = [
    0 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    45 as libc::c_int as flex_uint16_t,
    45 as libc::c_int as flex_uint16_t,
    46 as libc::c_int as flex_uint16_t,
    47 as libc::c_int as flex_uint16_t,
    54 as libc::c_int as flex_uint16_t,
    58 as libc::c_int as flex_uint16_t,
    62 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    51 as libc::c_int as flex_uint16_t,
    469 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    56 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    424 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    417 as libc::c_int as flex_uint16_t,
    400 as libc::c_int as flex_uint16_t,
    58 as libc::c_int as flex_uint16_t,
    381 as libc::c_int as flex_uint16_t,
    370 as libc::c_int as flex_uint16_t,
    67 as libc::c_int as flex_uint16_t,
    80 as libc::c_int as flex_uint16_t,
    88 as libc::c_int as flex_uint16_t,
    71 as libc::c_int as flex_uint16_t,
    93 as libc::c_int as flex_uint16_t,
    98 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    357 as libc::c_int as flex_uint16_t,
    51 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    314 as libc::c_int as flex_uint16_t,
    291 as libc::c_int as flex_uint16_t,
    52 as libc::c_int as flex_uint16_t,
    291 as libc::c_int as flex_uint16_t,
    287 as libc::c_int as flex_uint16_t,
    294 as libc::c_int as flex_uint16_t,
    79 as libc::c_int as flex_uint16_t,
    110 as libc::c_int as flex_uint16_t,
    134 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    325 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    115 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    324 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    321 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    117 as libc::c_int as flex_uint16_t,
    120 as libc::c_int as flex_uint16_t,
    319 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    318 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    311 as libc::c_int as flex_uint16_t,
    123 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    128 as libc::c_int as flex_uint16_t,
    139 as libc::c_int as flex_uint16_t,
    145 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    305 as libc::c_int as flex_uint16_t,
    153 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    314 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    156 as libc::c_int as flex_uint16_t,
    164 as libc::c_int as flex_uint16_t,
    172 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    177 as libc::c_int as flex_uint16_t,
    296 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    295 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    262 as libc::c_int as flex_uint16_t,
    260 as libc::c_int as flex_uint16_t,
    258 as libc::c_int as flex_uint16_t,
    96 as libc::c_int as flex_uint16_t,
    258 as libc::c_int as flex_uint16_t,
    254 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    125 as libc::c_int as flex_uint16_t,
    193 as libc::c_int as flex_uint16_t,
    173 as libc::c_int as flex_uint16_t,
    225 as libc::c_int as flex_uint16_t,
    188 as libc::c_int as flex_uint16_t,
    181 as libc::c_int as flex_uint16_t,
    197 as libc::c_int as flex_uint16_t,
    272 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    143 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    209 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    270 as libc::c_int as flex_uint16_t,
    196 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    199 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    259 as libc::c_int as flex_uint16_t,
    226 as libc::c_int as flex_uint16_t,
    216 as libc::c_int as flex_uint16_t,
    225 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    221 as libc::c_int as flex_uint16_t,
    237 as libc::c_int as flex_uint16_t,
    227 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    244 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    184 as libc::c_int as flex_uint16_t,
    173 as libc::c_int as flex_uint16_t,
    171 as libc::c_int as flex_uint16_t,
    157 as libc::c_int as flex_uint16_t,
    141 as libc::c_int as flex_uint16_t,
    144 as libc::c_int as flex_uint16_t,
    130 as libc::c_int as flex_uint16_t,
    208 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    254 as libc::c_int as flex_uint16_t,
    214 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    241 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    258 as libc::c_int as flex_uint16_t,
    155 as libc::c_int as flex_uint16_t,
    199 as libc::c_int as flex_uint16_t,
    230 as libc::c_int as flex_uint16_t,
    247 as libc::c_int as flex_uint16_t,
    265 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    125 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    112 as libc::c_int as flex_uint16_t,
    104 as libc::c_int as flex_uint16_t,
    95 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    262 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    119 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    112 as libc::c_int as flex_uint16_t,
    268 as libc::c_int as flex_uint16_t,
    273 as libc::c_int as flex_uint16_t,
    72 as libc::c_int as flex_uint16_t,
    74 as libc::c_int as flex_uint16_t,
    48 as libc::c_int as flex_uint16_t,
    44 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    291 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    42 as libc::c_int as flex_uint16_t,
    294 as libc::c_int as flex_uint16_t,
    288 as libc::c_int as flex_uint16_t,
    0 as libc::c_int as flex_uint16_t,
    274 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    309 as libc::c_int as flex_uint16_t,
    275 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    484 as libc::c_int as flex_uint16_t,
    334 as libc::c_int as flex_uint16_t,
    342 as libc::c_int as flex_uint16_t,
    350 as libc::c_int as flex_uint16_t,
    358 as libc::c_int as flex_uint16_t,
    366 as libc::c_int as flex_uint16_t,
    374 as libc::c_int as flex_uint16_t,
    382 as libc::c_int as flex_uint16_t,
    385 as libc::c_int as flex_uint16_t,
    393 as libc::c_int as flex_uint16_t,
    401 as libc::c_int as flex_uint16_t,
    409 as libc::c_int as flex_uint16_t,
    417 as libc::c_int as flex_uint16_t,
    425 as libc::c_int as flex_uint16_t,
    433 as libc::c_int as flex_uint16_t,
    441 as libc::c_int as flex_uint16_t,
    449 as libc::c_int as flex_uint16_t,
    452 as libc::c_int as flex_uint16_t,
    459 as libc::c_int as flex_uint16_t,
    464 as libc::c_int as flex_uint16_t,
    468 as libc::c_int as flex_uint16_t,
    475 as libc::c_int as flex_uint16_t,
];
static mut yy_def: [flex_int16_t; 212] = [
    0 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
];
static mut yy_nxt: [flex_uint16_t; 531] = [
    0 as libc::c_int as flex_uint16_t,
    12 as libc::c_int as flex_uint16_t,
    13 as libc::c_int as flex_uint16_t,
    14 as libc::c_int as flex_uint16_t,
    15 as libc::c_int as flex_uint16_t,
    16 as libc::c_int as flex_uint16_t,
    17 as libc::c_int as flex_uint16_t,
    18 as libc::c_int as flex_uint16_t,
    19 as libc::c_int as flex_uint16_t,
    20 as libc::c_int as flex_uint16_t,
    21 as libc::c_int as flex_uint16_t,
    22 as libc::c_int as flex_uint16_t,
    23 as libc::c_int as flex_uint16_t,
    24 as libc::c_int as flex_uint16_t,
    25 as libc::c_int as flex_uint16_t,
    26 as libc::c_int as flex_uint16_t,
    27 as libc::c_int as flex_uint16_t,
    27 as libc::c_int as flex_uint16_t,
    28 as libc::c_int as flex_uint16_t,
    29 as libc::c_int as flex_uint16_t,
    30 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    12 as libc::c_int as flex_uint16_t,
    32 as libc::c_int as flex_uint16_t,
    33 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    34 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    35 as libc::c_int as flex_uint16_t,
    36 as libc::c_int as flex_uint16_t,
    37 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    31 as libc::c_int as flex_uint16_t,
    12 as libc::c_int as flex_uint16_t,
    38 as libc::c_int as flex_uint16_t,
    12 as libc::c_int as flex_uint16_t,
    39 as libc::c_int as flex_uint16_t,
    44 as libc::c_int as flex_uint16_t,
    44 as libc::c_int as flex_uint16_t,
    47 as libc::c_int as flex_uint16_t,
    40 as libc::c_int as flex_uint16_t,
    48 as libc::c_int as flex_uint16_t,
    55 as libc::c_int as flex_uint16_t,
    55 as libc::c_int as flex_uint16_t,
    45 as libc::c_int as flex_uint16_t,
    45 as libc::c_int as flex_uint16_t,
    47 as libc::c_int as flex_uint16_t,
    57 as libc::c_int as flex_uint16_t,
    48 as libc::c_int as flex_uint16_t,
    51 as libc::c_int as flex_uint16_t,
    52 as libc::c_int as flex_uint16_t,
    58 as libc::c_int as flex_uint16_t,
    53 as libc::c_int as flex_uint16_t,
    51 as libc::c_int as flex_uint16_t,
    52 as libc::c_int as flex_uint16_t,
    63 as libc::c_int as flex_uint16_t,
    53 as libc::c_int as flex_uint16_t,
    87 as libc::c_int as flex_uint16_t,
    88 as libc::c_int as flex_uint16_t,
    90 as libc::c_int as flex_uint16_t,
    91 as libc::c_int as flex_uint16_t,
    49 as libc::c_int as flex_uint16_t,
    184 as libc::c_int as flex_uint16_t,
    181 as libc::c_int as flex_uint16_t,
    56 as libc::c_int as flex_uint16_t,
    56 as libc::c_int as flex_uint16_t,
    64 as libc::c_int as flex_uint16_t,
    68 as libc::c_int as flex_uint16_t,
    49 as libc::c_int as flex_uint16_t,
    69 as libc::c_int as flex_uint16_t,
    78 as libc::c_int as flex_uint16_t,
    70 as libc::c_int as flex_uint16_t,
    71 as libc::c_int as flex_uint16_t,
    71 as libc::c_int as flex_uint16_t,
    79 as libc::c_int as flex_uint16_t,
    72 as libc::c_int as flex_uint16_t,
    95 as libc::c_int as flex_uint16_t,
    180 as libc::c_int as flex_uint16_t,
    41 as libc::c_int as flex_uint16_t,
    80 as libc::c_int as flex_uint16_t,
    42 as libc::c_int as flex_uint16_t,
    73 as libc::c_int as flex_uint16_t,
    69 as libc::c_int as flex_uint16_t,
    96 as libc::c_int as flex_uint16_t,
    70 as libc::c_int as flex_uint16_t,
    71 as libc::c_int as flex_uint16_t,
    71 as libc::c_int as flex_uint16_t,
    100 as libc::c_int as flex_uint16_t,
    74 as libc::c_int as flex_uint16_t,
    75 as libc::c_int as flex_uint16_t,
    76 as libc::c_int as flex_uint16_t,
    179 as libc::c_int as flex_uint16_t,
    77 as libc::c_int as flex_uint16_t,
    77 as libc::c_int as flex_uint16_t,
    77 as libc::c_int as flex_uint16_t,
    81 as libc::c_int as flex_uint16_t,
    178 as libc::c_int as flex_uint16_t,
    82 as libc::c_int as flex_uint16_t,
    82 as libc::c_int as flex_uint16_t,
    83 as libc::c_int as flex_uint16_t,
    81 as libc::c_int as flex_uint16_t,
    102 as libc::c_int as flex_uint16_t,
    86 as libc::c_int as flex_uint16_t,
    86 as libc::c_int as flex_uint16_t,
    86 as libc::c_int as flex_uint16_t,
    103 as libc::c_int as flex_uint16_t,
    84 as libc::c_int as flex_uint16_t,
    112 as libc::c_int as flex_uint16_t,
    120 as libc::c_int as flex_uint16_t,
    122 as libc::c_int as flex_uint16_t,
    168 as libc::c_int as flex_uint16_t,
    57 as libc::c_int as flex_uint16_t,
    140 as libc::c_int as flex_uint16_t,
    101 as libc::c_int as flex_uint16_t,
    113 as libc::c_int as flex_uint16_t,
    58 as libc::c_int as flex_uint16_t,
    102 as libc::c_int as flex_uint16_t,
    166 as libc::c_int as flex_uint16_t,
    114 as libc::c_int as flex_uint16_t,
    176 as libc::c_int as flex_uint16_t,
    103 as libc::c_int as flex_uint16_t,
    125 as libc::c_int as flex_uint16_t,
    175 as libc::c_int as flex_uint16_t,
    141 as libc::c_int as flex_uint16_t,
    84 as libc::c_int as flex_uint16_t,
    105 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    126 as libc::c_int as flex_uint16_t,
    126 as libc::c_int as flex_uint16_t,
    174 as libc::c_int as flex_uint16_t,
    121 as libc::c_int as flex_uint16_t,
    121 as libc::c_int as flex_uint16_t,
    77 as libc::c_int as flex_uint16_t,
    77 as libc::c_int as flex_uint16_t,
    77 as libc::c_int as flex_uint16_t,
    112 as libc::c_int as flex_uint16_t,
    127 as libc::c_int as flex_uint16_t,
    173 as libc::c_int as flex_uint16_t,
    106 as libc::c_int as flex_uint16_t,
    106 as libc::c_int as flex_uint16_t,
    106 as libc::c_int as flex_uint16_t,
    81 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    83 as libc::c_int as flex_uint16_t,
    83 as libc::c_int as flex_uint16_t,
    83 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    81 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    86 as libc::c_int as flex_uint16_t,
    86 as libc::c_int as flex_uint16_t,
    86 as libc::c_int as flex_uint16_t,
    172 as libc::c_int as flex_uint16_t,
    168 as libc::c_int as flex_uint16_t,
    127 as libc::c_int as flex_uint16_t,
    163 as libc::c_int as flex_uint16_t,
    108 as libc::c_int as flex_uint16_t,
    129 as libc::c_int as flex_uint16_t,
    129 as libc::c_int as flex_uint16_t,
    129 as libc::c_int as flex_uint16_t,
    77 as libc::c_int as flex_uint16_t,
    77 as libc::c_int as flex_uint16_t,
    77 as libc::c_int as flex_uint16_t,
    162 as libc::c_int as flex_uint16_t,
    130 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    81 as libc::c_int as flex_uint16_t,
    133 as libc::c_int as flex_uint16_t,
    82 as libc::c_int as flex_uint16_t,
    82 as libc::c_int as flex_uint16_t,
    83 as libc::c_int as flex_uint16_t,
    161 as libc::c_int as flex_uint16_t,
    130 as libc::c_int as flex_uint16_t,
    148 as libc::c_int as flex_uint16_t,
    81 as libc::c_int as flex_uint16_t,
    133 as libc::c_int as flex_uint16_t,
    83 as libc::c_int as flex_uint16_t,
    83 as libc::c_int as flex_uint16_t,
    83 as libc::c_int as flex_uint16_t,
    81 as libc::c_int as flex_uint16_t,
    145 as libc::c_int as flex_uint16_t,
    86 as libc::c_int as flex_uint16_t,
    86 as libc::c_int as flex_uint16_t,
    86 as libc::c_int as flex_uint16_t,
    105 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    160 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    120 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    159 as libc::c_int as flex_uint16_t,
    122 as libc::c_int as flex_uint16_t,
    146 as libc::c_int as flex_uint16_t,
    146 as libc::c_int as flex_uint16_t,
    146 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    158 as libc::c_int as flex_uint16_t,
    106 as libc::c_int as flex_uint16_t,
    106 as libc::c_int as flex_uint16_t,
    106 as libc::c_int as flex_uint16_t,
    145 as libc::c_int as flex_uint16_t,
    112 as libc::c_int as flex_uint16_t,
    147 as libc::c_int as flex_uint16_t,
    154 as libc::c_int as flex_uint16_t,
    154 as libc::c_int as flex_uint16_t,
    154 as libc::c_int as flex_uint16_t,
    164 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    113 as libc::c_int as flex_uint16_t,
    157 as libc::c_int as flex_uint16_t,
    121 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    114 as libc::c_int as flex_uint16_t,
    121 as libc::c_int as flex_uint16_t,
    150 as libc::c_int as flex_uint16_t,
    108 as libc::c_int as flex_uint16_t,
    105 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    149 as libc::c_int as flex_uint16_t,
    132 as libc::c_int as flex_uint16_t,
    151 as libc::c_int as flex_uint16_t,
    151 as libc::c_int as flex_uint16_t,
    147 as libc::c_int as flex_uint16_t,
    150 as libc::c_int as flex_uint16_t,
    150 as libc::c_int as flex_uint16_t,
    129 as libc::c_int as flex_uint16_t,
    129 as libc::c_int as flex_uint16_t,
    129 as libc::c_int as flex_uint16_t,
    147 as libc::c_int as flex_uint16_t,
    106 as libc::c_int as flex_uint16_t,
    106 as libc::c_int as flex_uint16_t,
    106 as libc::c_int as flex_uint16_t,
    130 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    170 as libc::c_int as flex_uint16_t,
    170 as libc::c_int as flex_uint16_t,
    170 as libc::c_int as flex_uint16_t,
    153 as libc::c_int as flex_uint16_t,
    153 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    130 as libc::c_int as flex_uint16_t,
    154 as libc::c_int as flex_uint16_t,
    154 as libc::c_int as flex_uint16_t,
    154 as libc::c_int as flex_uint16_t,
    155 as libc::c_int as flex_uint16_t,
    155 as libc::c_int as flex_uint16_t,
    145 as libc::c_int as flex_uint16_t,
    108 as libc::c_int as flex_uint16_t,
    156 as libc::c_int as flex_uint16_t,
    156 as libc::c_int as flex_uint16_t,
    156 as libc::c_int as flex_uint16_t,
    156 as libc::c_int as flex_uint16_t,
    156 as libc::c_int as flex_uint16_t,
    156 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    166 as libc::c_int as flex_uint16_t,
    150 as libc::c_int as flex_uint16_t,
    146 as libc::c_int as flex_uint16_t,
    146 as libc::c_int as flex_uint16_t,
    146 as libc::c_int as flex_uint16_t,
    118 as libc::c_int as flex_uint16_t,
    167 as libc::c_int as flex_uint16_t,
    167 as libc::c_int as flex_uint16_t,
    110 as libc::c_int as flex_uint16_t,
    165 as libc::c_int as flex_uint16_t,
    186 as libc::c_int as flex_uint16_t,
    189 as libc::c_int as flex_uint16_t,
    147 as libc::c_int as flex_uint16_t,
    171 as libc::c_int as flex_uint16_t,
    171 as libc::c_int as flex_uint16_t,
    171 as libc::c_int as flex_uint16_t,
    170 as libc::c_int as flex_uint16_t,
    170 as libc::c_int as flex_uint16_t,
    170 as libc::c_int as flex_uint16_t,
    143 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    171 as libc::c_int as flex_uint16_t,
    171 as libc::c_int as flex_uint16_t,
    171 as libc::c_int as flex_uint16_t,
    186 as libc::c_int as flex_uint16_t,
    177 as libc::c_int as flex_uint16_t,
    182 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    142 as libc::c_int as flex_uint16_t,
    182 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    139 as libc::c_int as flex_uint16_t,
    188 as libc::c_int as flex_uint16_t,
    188 as libc::c_int as flex_uint16_t,
    138 as libc::c_int as flex_uint16_t,
    137 as libc::c_int as flex_uint16_t,
    187 as libc::c_int as flex_uint16_t,
    187 as libc::c_int as flex_uint16_t,
    187 as libc::c_int as flex_uint16_t,
    183 as libc::c_int as flex_uint16_t,
    183 as libc::c_int as flex_uint16_t,
    183 as libc::c_int as flex_uint16_t,
    183 as libc::c_int as flex_uint16_t,
    183 as libc::c_int as flex_uint16_t,
    183 as libc::c_int as flex_uint16_t,
    186 as libc::c_int as flex_uint16_t,
    188 as libc::c_int as flex_uint16_t,
    136 as libc::c_int as flex_uint16_t,
    135 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    132 as libc::c_int as flex_uint16_t,
    128 as libc::c_int as flex_uint16_t,
    107 as libc::c_int as flex_uint16_t,
    123 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    118 as libc::c_int as flex_uint16_t,
    187 as libc::c_int as flex_uint16_t,
    187 as libc::c_int as flex_uint16_t,
    187 as libc::c_int as flex_uint16_t,
    117 as libc::c_int as flex_uint16_t,
    110 as libc::c_int as flex_uint16_t,
    99 as libc::c_int as flex_uint16_t,
    98 as libc::c_int as flex_uint16_t,
    97 as libc::c_int as flex_uint16_t,
    94 as libc::c_int as flex_uint16_t,
    93 as libc::c_int as flex_uint16_t,
    188 as libc::c_int as flex_uint16_t,
    43 as libc::c_int as flex_uint16_t,
    43 as libc::c_int as flex_uint16_t,
    43 as libc::c_int as flex_uint16_t,
    43 as libc::c_int as flex_uint16_t,
    43 as libc::c_int as flex_uint16_t,
    43 as libc::c_int as flex_uint16_t,
    43 as libc::c_int as flex_uint16_t,
    43 as libc::c_int as flex_uint16_t,
    46 as libc::c_int as flex_uint16_t,
    46 as libc::c_int as flex_uint16_t,
    46 as libc::c_int as flex_uint16_t,
    46 as libc::c_int as flex_uint16_t,
    46 as libc::c_int as flex_uint16_t,
    46 as libc::c_int as flex_uint16_t,
    46 as libc::c_int as flex_uint16_t,
    46 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    50 as libc::c_int as flex_uint16_t,
    54 as libc::c_int as flex_uint16_t,
    54 as libc::c_int as flex_uint16_t,
    54 as libc::c_int as flex_uint16_t,
    54 as libc::c_int as flex_uint16_t,
    54 as libc::c_int as flex_uint16_t,
    54 as libc::c_int as flex_uint16_t,
    54 as libc::c_int as flex_uint16_t,
    54 as libc::c_int as flex_uint16_t,
    60 as libc::c_int as flex_uint16_t,
    60 as libc::c_int as flex_uint16_t,
    60 as libc::c_int as flex_uint16_t,
    60 as libc::c_int as flex_uint16_t,
    60 as libc::c_int as flex_uint16_t,
    60 as libc::c_int as flex_uint16_t,
    60 as libc::c_int as flex_uint16_t,
    60 as libc::c_int as flex_uint16_t,
    65 as libc::c_int as flex_uint16_t,
    89 as libc::c_int as flex_uint16_t,
    65 as libc::c_int as flex_uint16_t,
    65 as libc::c_int as flex_uint16_t,
    65 as libc::c_int as flex_uint16_t,
    65 as libc::c_int as flex_uint16_t,
    65 as libc::c_int as flex_uint16_t,
    65 as libc::c_int as flex_uint16_t,
    85 as libc::c_int as flex_uint16_t,
    85 as libc::c_int as flex_uint16_t,
    85 as libc::c_int as flex_uint16_t,
    85 as libc::c_int as flex_uint16_t,
    85 as libc::c_int as flex_uint16_t,
    85 as libc::c_int as flex_uint16_t,
    67 as libc::c_int as flex_uint16_t,
    85 as libc::c_int as flex_uint16_t,
    92 as libc::c_int as flex_uint16_t,
    92 as libc::c_int as flex_uint16_t,
    92 as libc::c_int as flex_uint16_t,
    104 as libc::c_int as flex_uint16_t,
    104 as libc::c_int as flex_uint16_t,
    104 as libc::c_int as flex_uint16_t,
    104 as libc::c_int as flex_uint16_t,
    104 as libc::c_int as flex_uint16_t,
    104 as libc::c_int as flex_uint16_t,
    104 as libc::c_int as flex_uint16_t,
    104 as libc::c_int as flex_uint16_t,
    109 as libc::c_int as flex_uint16_t,
    109 as libc::c_int as flex_uint16_t,
    109 as libc::c_int as flex_uint16_t,
    109 as libc::c_int as flex_uint16_t,
    66 as libc::c_int as flex_uint16_t,
    109 as libc::c_int as flex_uint16_t,
    109 as libc::c_int as flex_uint16_t,
    109 as libc::c_int as flex_uint16_t,
    111 as libc::c_int as flex_uint16_t,
    111 as libc::c_int as flex_uint16_t,
    111 as libc::c_int as flex_uint16_t,
    111 as libc::c_int as flex_uint16_t,
    111 as libc::c_int as flex_uint16_t,
    111 as libc::c_int as flex_uint16_t,
    111 as libc::c_int as flex_uint16_t,
    111 as libc::c_int as flex_uint16_t,
    115 as libc::c_int as flex_uint16_t,
    62 as libc::c_int as flex_uint16_t,
    61 as libc::c_int as flex_uint16_t,
    115 as libc::c_int as flex_uint16_t,
    115 as libc::c_int as flex_uint16_t,
    115 as libc::c_int as flex_uint16_t,
    115 as libc::c_int as flex_uint16_t,
    115 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    116 as libc::c_int as flex_uint16_t,
    119 as libc::c_int as flex_uint16_t,
    119 as libc::c_int as flex_uint16_t,
    119 as libc::c_int as flex_uint16_t,
    119 as libc::c_int as flex_uint16_t,
    119 as libc::c_int as flex_uint16_t,
    119 as libc::c_int as flex_uint16_t,
    119 as libc::c_int as flex_uint16_t,
    119 as libc::c_int as flex_uint16_t,
    124 as libc::c_int as flex_uint16_t,
    59 as libc::c_int as flex_uint16_t,
    124 as libc::c_int as flex_uint16_t,
    124 as libc::c_int as flex_uint16_t,
    124 as libc::c_int as flex_uint16_t,
    124 as libc::c_int as flex_uint16_t,
    124 as libc::c_int as flex_uint16_t,
    124 as libc::c_int as flex_uint16_t,
    131 as libc::c_int as flex_uint16_t,
    131 as libc::c_int as flex_uint16_t,
    131 as libc::c_int as flex_uint16_t,
    131 as libc::c_int as flex_uint16_t,
    131 as libc::c_int as flex_uint16_t,
    131 as libc::c_int as flex_uint16_t,
    131 as libc::c_int as flex_uint16_t,
    131 as libc::c_int as flex_uint16_t,
    134 as libc::c_int as flex_uint16_t,
    134 as libc::c_int as flex_uint16_t,
    144 as libc::c_int as flex_uint16_t,
    144 as libc::c_int as flex_uint16_t,
    144 as libc::c_int as flex_uint16_t,
    144 as libc::c_int as flex_uint16_t,
    144 as libc::c_int as flex_uint16_t,
    144 as libc::c_int as flex_uint16_t,
    144 as libc::c_int as flex_uint16_t,
    144 as libc::c_int as flex_uint16_t,
    152 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    152 as libc::c_int as flex_uint16_t,
    152 as libc::c_int as flex_uint16_t,
    169 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    169 as libc::c_int as flex_uint16_t,
    169 as libc::c_int as flex_uint16_t,
    185 as libc::c_int as flex_uint16_t,
    185 as libc::c_int as flex_uint16_t,
    185 as libc::c_int as flex_uint16_t,
    185 as libc::c_int as flex_uint16_t,
    185 as libc::c_int as flex_uint16_t,
    185 as libc::c_int as flex_uint16_t,
    185 as libc::c_int as flex_uint16_t,
    185 as libc::c_int as flex_uint16_t,
    11 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
    190 as libc::c_int as flex_uint16_t,
];
static mut yy_chk: [flex_int16_t; 531] = [
    0 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    181 as libc::c_int as flex_int16_t,
    175 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    174 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    173 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    172 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    169 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    161 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    160 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    159 as libc::c_int as flex_int16_t,
    152 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    143 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    142 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    141 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    140 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    139 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    121 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    131 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    126 as libc::c_int as flex_int16_t,
    144 as libc::c_int as flex_int16_t,
    127 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    147 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    130 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    146 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    170 as libc::c_int as flex_int16_t,
    170 as libc::c_int as flex_int16_t,
    170 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    171 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    185 as libc::c_int as flex_int16_t,
    188 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    183 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    177 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    182 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    187 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    191 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    192 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    193 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    194 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    195 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    196 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    197 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    198 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    199 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    200 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    201 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    202 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    203 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    204 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    205 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    206 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    207 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    208 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    209 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    210 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    211 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
    190 as libc::c_int as flex_int16_t,
];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut yy_flex_debug: libc::c_int = 1 as libc::c_int;
static mut yy_rule_linenum: [flex_int16_t; 81] = [
    0 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    119 as libc::c_int as flex_int16_t,
    123 as libc::c_int as flex_int16_t,
    124 as libc::c_int as flex_int16_t,
    125 as libc::c_int as flex_int16_t,
    129 as libc::c_int as flex_int16_t,
    133 as libc::c_int as flex_int16_t,
    134 as libc::c_int as flex_int16_t,
    135 as libc::c_int as flex_int16_t,
    148 as libc::c_int as flex_int16_t,
    149 as libc::c_int as flex_int16_t,
    150 as libc::c_int as flex_int16_t,
    151 as libc::c_int as flex_int16_t,
    152 as libc::c_int as flex_int16_t,
    153 as libc::c_int as flex_int16_t,
    154 as libc::c_int as flex_int16_t,
    155 as libc::c_int as flex_int16_t,
    156 as libc::c_int as flex_int16_t,
    157 as libc::c_int as flex_int16_t,
    162 as libc::c_int as flex_int16_t,
    163 as libc::c_int as flex_int16_t,
    165 as libc::c_int as flex_int16_t,
    166 as libc::c_int as flex_int16_t,
    167 as libc::c_int as flex_int16_t,
];
pub static mut yytext: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub unsafe extern "C" fn yylex() -> libc::c_int {
    let mut yy_amount_of_matched_text: libc::c_int = 0;
    let mut yy_next_state: yy_state_type = 0;
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_act: libc::c_int = 0;
    if yy_init == 0 {
        yy_init = 1 as libc::c_int;
        if yy_start == 0 {
            yy_start = 1 as libc::c_int;
        }
        if yyin.is_null() {
            yyin = stdin;
        }
        if yyout.is_null() {
            yyout = stdout;
        }
        if if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        }
            .is_null()
        {
            yyensure_buffer_stack();
            let ref mut fresh4 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            *fresh4 = yy_create_buffer(yyin, 16384 as libc::c_int);
        }
        yy_load_buffer_state();
    }
    loop {
        yy_cp = yy_c_buf_p;
        *yy_cp = yy_hold_char;
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        yy_current_state
            += (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_at_bol;
        '_yy_match: loop {
            loop {
                let mut yy_c: YY_CHAR = yy_ec[*yy_cp as libc::c_uchar as libc::c_uint
                    as usize];
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
                    + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
                    if yy_current_state >= 191 as libc::c_int {
                        yy_c = yy_meta[yy_c as libc::c_uint as usize];
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize]
                    as libc::c_uint)
                    .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                yy_cp;
                if !(yy_base[yy_current_state as usize] as libc::c_int
                    != 484 as libc::c_int)
                {
                    break;
                }
            }
            '_yy_find_action: loop {
                yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                if yy_act == 0 as libc::c_int {
                    yy_cp = yy_last_accepting_cpos;
                    yy_current_state = yy_last_accepting_state;
                    yy_act = yy_accept[yy_current_state as usize] as libc::c_int;
                }
                yytext = yy_bp;
                yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as size_t;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as libc::c_char;
                yy_c_buf_p = yy_cp;
                loop {
                    if yy_flex_debug != 0 {
                        if yy_act == 0 as libc::c_int {
                            fprintf(
                                stderr,
                                b"--scanner backing up\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else if yy_act < 81 as libc::c_int {
                            fprintf(
                                stderr,
                                b"--accepting rule at line %ld (\"%s\")\n\0" as *const u8
                                    as *const libc::c_char,
                                yy_rule_linenum[yy_act as usize] as libc::c_long,
                                yytext,
                            );
                        } else if yy_act == 81 as libc::c_int {
                            fprintf(
                                stderr,
                                b"--accepting default rule (\"%s\")\n\0" as *const u8
                                    as *const libc::c_char,
                                yytext,
                            );
                        } else if yy_act == 82 as libc::c_int {
                            fprintf(
                                stderr,
                                b"--(end of buffer or a NUL)\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            fprintf(
                                stderr,
                                b"--EOF (start condition %d)\n\0" as *const u8
                                    as *const libc::c_char,
                                (yy_start - 1 as libc::c_int) / 2 as libc::c_int,
                            );
                        }
                    }
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue '_yy_find_action;
                        }
                        1 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        2 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 1 as libc::c_int;
                            break '_yy_match;
                        }
                        3 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break '_yy_match;
                        }
                        4 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        5 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break '_yy_match;
                        }
                        6 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        7 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 0 as libc::c_int;
                            break '_yy_match;
                        }
                        8 | 9 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            update_loc();
                            break '_yy_match;
                        }
                        10 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 4 as libc::c_int;
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        11 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        12 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        13 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 0 as libc::c_int;
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        14 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break '_yy_match;
                        }
                        15 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return EXTERN as libc::c_int;
                        }
                        16 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return STATIC as libc::c_int;
                        }
                        17 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return TYPEDEF as libc::c_int;
                        }
                        18 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"struct\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return STRUCT as libc::c_int;
                        }
                        19 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"union\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return STRUCT as libc::c_int;
                        }
                        20 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"enum\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return STRUCT as libc::c_int;
                        }
                        21 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"*\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return MODIFIER as libc::c_int;
                        }
                        22 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"->\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return MEMBER_OF as libc::c_int;
                        }
                        23 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b".\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return MEMBER_OF as libc::c_int;
                        }
                        24 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"*=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        25 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"/=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        26 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"/\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        27 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"%=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        28 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"%\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        29 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"+=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        30 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"+\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        31 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"-=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        32 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"-\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        33 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"<<=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        34 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b">>=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        35 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"&=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        36 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"|=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        37 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"^=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        38 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"^\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        39 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"||\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        40 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"|\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        41 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"&&\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        42 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"&\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        43 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"==\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        44 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return '=' as i32;
                        }
                        45 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"!=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        46 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"!\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        47 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b">=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        48 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b">\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        49 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"<=\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        50 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"<\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        51 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"<<\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        52 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b">>\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        53 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"++\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        54 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yylval
                                .str_0 = b"--\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                            return OP as libc::c_int;
                        }
                        55 | 56 | 57 | 58 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return STRING as libc::c_int;
                        }
                        59 | 60 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return ident();
                        }
                        61 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            let mut __o: *mut obstack = &mut string_stk;
                            let mut __len: size_t = yyleng
                                .wrapping_add(1 as libc::c_int as libc::c_ulong);
                            if ({
                                let mut __o1: *const obstack = __o;
                                ((*__o1).chunk_limit).offset_from((*__o1).next_free)
                                    as libc::c_long as size_t
                            }) < __len
                            {
                                _obstack_newchunk(__o, __len);
                            }
                            memcpy(
                                (*__o).next_free as *mut libc::c_void,
                                yytext as *const libc::c_void,
                                __len,
                            );
                            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                            yylval
                                .str_0 = ({
                                let mut __o1: *mut obstack = &mut string_stk
                                    as *mut obstack;
                                let mut __value: *mut libc::c_void = (*__o1).object_base
                                    as *mut libc::c_void;
                                if (*__o1).next_free == __value as *mut libc::c_char {
                                    (*__o1)
                                        .set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                                }
                                (*__o1)
                                    .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                                    as libc::c_ulong)
                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut libc::c_char
                                })
                                    .offset(
                                        ((((*__o1).next_free)
                                            .offset_from(
                                                (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                                        as libc::c_ulong
                                                {
                                                    (*__o1).object_base
                                                } else {
                                                    0 as *mut libc::c_char
                                                }),
                                            ) as libc::c_long as libc::c_ulong)
                                            .wrapping_add((*__o1).alignment_mask)
                                            & !(*__o1).alignment_mask) as isize,
                                    );
                                if ((*__o1).next_free)
                                    .offset_from((*__o1).chunk as *mut libc::c_char)
                                    as libc::c_long as size_t
                                    > ((*__o1).chunk_limit)
                                        .offset_from((*__o1).chunk as *mut libc::c_char)
                                        as libc::c_long as size_t
                                {
                                    (*__o1).next_free = (*__o1).chunk_limit;
                                }
                                (*__o1).object_base = (*__o1).next_free;
                                __value
                            }) as *mut libc::c_char;
                            return WORD as libc::c_int;
                        }
                        62 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            let mut yyless_macro_arg: libc::c_int = yyleng
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int;
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_bp
                                .offset(yyless_macro_arg as isize)
                                .offset(-(0 as libc::c_int as isize));
                            yy_c_buf_p = yy_cp;
                            yytext = yy_bp;
                            yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as size_t;
                            yy_hold_char = *yy_cp;
                            *yy_cp = '\0' as i32 as libc::c_char;
                            yy_c_buf_p = yy_cp;
                            let mut __o_0: *mut obstack = &mut string_stk;
                            let mut __len_0: size_t = yyleng
                                .wrapping_add(1 as libc::c_int as libc::c_ulong);
                            if ({
                                let mut __o1: *const obstack = __o_0;
                                ((*__o1).chunk_limit).offset_from((*__o1).next_free)
                                    as libc::c_long as size_t
                            }) < __len_0
                            {
                                _obstack_newchunk(__o_0, __len_0);
                            }
                            memcpy(
                                (*__o_0).next_free as *mut libc::c_void,
                                yytext as *const libc::c_void,
                                __len_0,
                            );
                            (*__o_0)
                                .next_free = ((*__o_0).next_free).offset(__len_0 as isize);
                            yylval
                                .str_0 = ({
                                let mut __o1: *mut obstack = &mut string_stk
                                    as *mut obstack;
                                let mut __value: *mut libc::c_void = (*__o1).object_base
                                    as *mut libc::c_void;
                                if (*__o1).next_free == __value as *mut libc::c_char {
                                    (*__o1)
                                        .set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                                }
                                (*__o1)
                                    .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                                    as libc::c_ulong)
                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut libc::c_char
                                })
                                    .offset(
                                        ((((*__o1).next_free)
                                            .offset_from(
                                                (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                                        as libc::c_ulong
                                                {
                                                    (*__o1).object_base
                                                } else {
                                                    0 as *mut libc::c_char
                                                }),
                                            ) as libc::c_long as libc::c_ulong)
                                            .wrapping_add((*__o1).alignment_mask)
                                            & !(*__o1).alignment_mask) as isize,
                                    );
                                if ((*__o1).next_free)
                                    .offset_from((*__o1).chunk as *mut libc::c_char)
                                    as libc::c_long as size_t
                                    > ((*__o1).chunk_limit)
                                        .offset_from((*__o1).chunk as *mut libc::c_char)
                                        as libc::c_long as size_t
                                {
                                    (*__o1).next_free = (*__o1).chunk_limit;
                                }
                                (*__o1).object_base = (*__o1).next_free;
                                __value
                            }) as *mut libc::c_char;
                            return WORD as libc::c_int;
                        }
                        63 | 64 | 65 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            let mut __o_1: *mut obstack = &mut string_stk;
                            let mut __len_1: size_t = yyleng
                                .wrapping_add(1 as libc::c_int as libc::c_ulong);
                            if ({
                                let mut __o1: *const obstack = __o_1;
                                ((*__o1).chunk_limit).offset_from((*__o1).next_free)
                                    as libc::c_long as size_t
                            }) < __len_1
                            {
                                _obstack_newchunk(__o_1, __len_1);
                            }
                            memcpy(
                                (*__o_1).next_free as *mut libc::c_void,
                                yytext as *const libc::c_void,
                                __len_1,
                            );
                            (*__o_1)
                                .next_free = ((*__o_1).next_free).offset(__len_1 as isize);
                            yylval
                                .str_0 = ({
                                let mut __o1: *mut obstack = &mut string_stk
                                    as *mut obstack;
                                let mut __value: *mut libc::c_void = (*__o1).object_base
                                    as *mut libc::c_void;
                                if (*__o1).next_free == __value as *mut libc::c_char {
                                    (*__o1)
                                        .set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
                                }
                                (*__o1)
                                    .next_free = (if (::std::mem::size_of::<ptrdiff_t>()
                                    as libc::c_ulong)
                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut libc::c_char
                                })
                                    .offset(
                                        ((((*__o1).next_free)
                                            .offset_from(
                                                (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                                    < ::std::mem::size_of::<*mut libc::c_void>()
                                                        as libc::c_ulong
                                                {
                                                    (*__o1).object_base
                                                } else {
                                                    0 as *mut libc::c_char
                                                }),
                                            ) as libc::c_long as libc::c_ulong)
                                            .wrapping_add((*__o1).alignment_mask)
                                            & !(*__o1).alignment_mask) as isize,
                                    );
                                if ((*__o1).next_free)
                                    .offset_from((*__o1).chunk as *mut libc::c_char)
                                    as libc::c_long as size_t
                                    > ((*__o1).chunk_limit)
                                        .offset_from((*__o1).chunk as *mut libc::c_char)
                                        as libc::c_long as size_t
                                {
                                    (*__o1).next_free = (*__o1).chunk_limit;
                                }
                                (*__o1).object_base = (*__o1).next_free;
                                __value
                            }) as *mut libc::c_char;
                            return WORD as libc::c_int;
                        }
                        66 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 2 as libc::c_int;
                            break '_yy_match;
                        }
                        67 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break '_yy_match;
                        }
                        68 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            line_num;
                            error_at_line(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                filename,
                                line_num as libc::c_uint,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"unterminated string?\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            break '_yy_match;
                        }
                        69 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break '_yy_match;
                        }
                        70 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        71 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 3 as libc::c_int;
                            break '_yy_match;
                        }
                        72 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break '_yy_match;
                        }
                        73 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        74 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 2 as libc::c_int;
                            break '_yy_match;
                        }
                        75 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            yy_start = 1 as libc::c_int
                                + 2 as libc::c_int * 0 as libc::c_int;
                            let mut yyless_macro_arg_0: libc::c_int = 0 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_bp
                                .offset(yyless_macro_arg_0 as isize)
                                .offset(-(0 as libc::c_int as isize));
                            yy_c_buf_p = yy_cp;
                            yytext = yy_bp;
                            yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as size_t;
                            yy_hold_char = *yy_cp;
                            *yy_cp = '\0' as i32 as libc::c_char;
                            yy_c_buf_p = yy_cp;
                            return STRING as libc::c_int;
                        }
                        76 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            line_num += 1;
                            line_num;
                            break '_yy_match;
                        }
                        77 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            break '_yy_match;
                        }
                        78 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return LBRACE0 as libc::c_int;
                        }
                        79 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return RBRACE0 as libc::c_int;
                        }
                        80 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            return *yytext.offset(0 as libc::c_int as isize)
                                as libc::c_int;
                        }
                        81 => {
                            if yyleng > 0 as libc::c_int as libc::c_ulong {
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_at_bol = (*yytext
                                    .offset(
                                        yyleng.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == '\n' as i32) as libc::c_int;
                            }
                            fwrite(
                                yytext as *const libc::c_void,
                                yyleng,
                                1 as libc::c_int as libc::c_ulong,
                                yyout,
                            ) != 0;
                            break '_yy_match;
                        }
                        83 | 84 | 85 | 86 | 87 => return 0 as libc::c_int,
                        82 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(yytext)
                                as libc::c_long as libc::c_int - 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                .yy_buffer_status == 0 as libc::c_int
                            {
                                yy_n_chars = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_n_chars;
                                let ref mut fresh5 = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_input_file;
                                *fresh5 = yyin;
                                (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                    .yy_buffer_status = 1 as libc::c_int;
                            }
                            if yy_c_buf_p
                                <= &mut *((**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_ch_buf)
                                    .offset(yy_n_chars as isize) as *mut libc::c_char
                            {
                                yy_next_state = 0;
                                yy_c_buf_p = yytext
                                    .offset(yy_amount_of_matched_text as isize);
                                yy_current_state = yy_get_previous_state();
                                yy_next_state = yy_try_NUL_trans(yy_current_state);
                                yy_bp = yytext.offset(0 as libc::c_int as isize);
                                if yy_next_state != 0 {
                                    current_block = 114911724110381832;
                                    break;
                                } else {
                                    current_block = 15264922082917136948;
                                    break;
                                }
                            } else {
                                match yy_get_next_buffer() {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as libc::c_int;
                                        if yywrap() != 0 {
                                            yy_c_buf_p = yytext.offset(0 as libc::c_int as isize);
                                            yy_act = 82 as libc::c_int
                                                + (yy_start - 1 as libc::c_int) / 2 as libc::c_int
                                                + 1 as libc::c_int;
                                        } else {
                                            if yy_did_buffer_switch_on_eof == 0 {
                                                yyrestart(yyin);
                                            }
                                            break '_yy_match;
                                        }
                                    }
                                    0 => {
                                        yy_c_buf_p = yytext
                                            .offset(yy_amount_of_matched_text as isize);
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
                                        break '_yy_find_action;
                                    }
                                    2 => {
                                        yy_c_buf_p = &mut *((**yy_buffer_stack
                                            .offset(yy_buffer_stack_top as isize))
                                            .yy_ch_buf)
                                            .offset(yy_n_chars as isize) as *mut libc::c_char;
                                        yy_current_state = yy_get_previous_state();
                                        yy_cp = yy_c_buf_p;
                                        yy_bp = yytext.offset(0 as libc::c_int as isize);
                                        continue '_yy_find_action;
                                    }
                                    _ => {
                                        break '_yy_match;
                                    }
                                }
                            }
                        }
                        _ => {
                            yy_fatal_error(
                                b"fatal flex scanner internal error--no action found\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
                match current_block {
                    15264922082917136948 => {
                        yy_cp = yy_c_buf_p;
                    }
                    _ => {
                        yy_c_buf_p = yy_c_buf_p.offset(1);
                        yy_cp = yy_c_buf_p;
                        yy_current_state = yy_next_state;
                        break;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn yy_get_next_buffer() -> libc::c_int {
    let mut dest: *mut libc::c_char = (**yy_buffer_stack
        .offset(yy_buffer_stack_top as isize))
        .yy_ch_buf;
    let mut source_0: *mut libc::c_char = yytext;
    let mut number_to_move: yy_size_t = 0;
    let mut i: yy_size_t = 0;
    let mut ret_val: libc::c_int = 0;
    if yy_c_buf_p
        > &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset(yy_n_chars.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
            as *mut libc::c_char
    {
        yy_fatal_error(
            b"fatal flex scanner internal error--end of buffer missed\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_fill_buffer
        == 0 as libc::c_int
    {
        if yy_c_buf_p.offset_from(yytext) as libc::c_long
            - 0 as libc::c_int as libc::c_long == 1 as libc::c_int as libc::c_long
        {
            return 1 as libc::c_int
        } else {
            return 2 as libc::c_int
        }
    }
    number_to_move = (yy_c_buf_p.offset_from(yytext) as libc::c_long as yy_size_t)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    i = 0 as libc::c_int as yy_size_t;
    while i < number_to_move {
        let fresh6 = source_0;
        source_0 = source_0.offset(1);
        let fresh7 = dest;
        dest = dest.offset(1);
        *fresh7 = *fresh6;
        i = i.wrapping_add(1);
        i;
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buffer_status
        == 2 as libc::c_int
    {
        yy_n_chars = 0 as libc::c_int as yy_size_t;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    } else {
        let mut num_to_read: yy_size_t = ((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_buf_size)
            .wrapping_sub(number_to_move)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        while num_to_read <= 0 as libc::c_int as libc::c_ulong {
            let mut b: YY_BUFFER_STATE = *yy_buffer_stack
                .offset(yy_buffer_stack_top as isize);
            let mut yy_c_buf_p_offset: libc::c_int = yy_c_buf_p
                .offset_from((*b).yy_ch_buf) as libc::c_long as libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: yy_size_t = ((*b).yy_buf_size)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong);
                if new_size <= 0 as libc::c_int as libc::c_ulong {
                    (*b)
                        .yy_buf_size = ((*b).yy_buf_size as libc::c_ulong)
                        .wrapping_add(
                            ((*b).yy_buf_size)
                                .wrapping_div(8 as libc::c_int as libc::c_ulong),
                        ) as yy_size_t as yy_size_t;
                } else {
                    (*b)
                        .yy_buf_size = ((*b).yy_buf_size as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as yy_size_t
                        as yy_size_t;
                }
                (*b)
                    .yy_ch_buf = yyrealloc(
                    (*b).yy_ch_buf as *mut libc::c_void,
                    ((*b).yy_buf_size).wrapping_add(2 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
            } else {
                (*b).yy_ch_buf = 0 as *mut libc::c_char;
            }
            if ((*b).yy_ch_buf).is_null() {
                yy_fatal_error(
                    b"fatal error - scanner input buffer overflow\0" as *const u8
                        as *const libc::c_char,
                );
            }
            yy_c_buf_p = &mut *((*b).yy_ch_buf).offset(yy_c_buf_p_offset as isize)
                as *mut libc::c_char;
            num_to_read = ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buf_size)
                .wrapping_sub(number_to_move)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
        if num_to_read > 8192 as libc::c_int as libc::c_ulong {
            num_to_read = 8192 as libc::c_int as yy_size_t;
        }
        if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_is_interactive
            != 0
        {
            let mut c: libc::c_int = '*' as i32;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < num_to_read
                && {
                    c = getc(yyin);
                    c != -(1 as libc::c_int)
                } && c != '\n' as i32
            {
                *(&mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut libc::c_char)
                    .offset(n as isize) = c as libc::c_char;
                n = n.wrapping_add(1);
                n;
            }
            if c == '\n' as i32 {
                let fresh8 = n;
                n = n.wrapping_add(1);
                *(&mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut libc::c_char)
                    .offset(fresh8 as isize) = c as libc::c_char;
            }
            if c == -(1 as libc::c_int) && ferror(yyin) != 0 {
                yy_fatal_error(
                    b"input in flex scanner failed\0" as *const u8 as *const libc::c_char,
                );
            }
            yy_n_chars = n;
        } else {
            *__errno_location() = 0 as libc::c_int;
            loop {
                yy_n_chars = fread(
                    &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                        .yy_ch_buf)
                        .offset(number_to_move as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    num_to_read,
                    yyin,
                );
                if !(yy_n_chars == 0 as libc::c_int as libc::c_ulong
                    && ferror(yyin) != 0)
                {
                    break;
                }
                if *__errno_location() != 4 as libc::c_int {
                    yy_fatal_error(
                        b"input in flex scanner failed\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    *__errno_location() = 0 as libc::c_int;
                    clearerr(yyin);
                }
            }
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if yy_n_chars == 0 as libc::c_int as libc::c_ulong {
        if number_to_move == 0 as libc::c_int as libc::c_ulong {
            ret_val = 1 as libc::c_int;
            yyrestart(yyin);
        } else {
            ret_val = 2 as libc::c_int;
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                .yy_buffer_status = 2 as libc::c_int;
        }
    } else {
        ret_val = 0 as libc::c_int;
    }
    if yy_n_chars.wrapping_add(number_to_move)
        > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
    {
        let mut new_size_0: yy_size_t = yy_n_chars
            .wrapping_add(number_to_move)
            .wrapping_add(yy_n_chars >> 1 as libc::c_int);
        let ref mut fresh9 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_ch_buf;
        *fresh9 = yyrealloc(
            (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
                as *mut libc::c_void,
            new_size_0,
        ) as *mut libc::c_char;
        if ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf).is_null()
        {
            yy_fatal_error(
                b"out of dynamic memory in yy_get_next_buffer()\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    yy_n_chars = (yy_n_chars as libc::c_ulong).wrapping_add(number_to_move) as yy_size_t
        as yy_size_t;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(yy_n_chars as isize) = 0 as libc::c_int as libc::c_char;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(
            yy_n_chars.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    yytext = &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    return ret_val;
}
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_current_state = yy_start;
    yy_current_state
        += (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_at_bol;
    yy_cp = yytext.offset(0 as libc::c_int as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR = (if *yy_cp as libc::c_int != 0 {
            yy_ec[*yy_cp as libc::c_uchar as libc::c_uint as usize] as libc::c_int
        } else {
            1 as libc::c_int
        }) as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            yy_last_accepting_state = yy_current_state;
            yy_last_accepting_cpos = yy_cp;
        }
        while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
            + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
        {
            yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
            if yy_current_state >= 191 as libc::c_int {
                yy_c = yy_meta[yy_c as libc::c_uint as usize];
            }
        }
        yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_uint)
            .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1);
        yy_cp;
    }
    return yy_current_state;
}
unsafe extern "C" fn yy_try_NUL_trans(
    mut yy_current_state: yy_state_type,
) -> yy_state_type {
    let mut yy_is_jam: libc::c_int = 0;
    let mut yy_cp: *mut libc::c_char = yy_c_buf_p;
    let mut yy_c: YY_CHAR = 1 as libc::c_int as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        yy_last_accepting_state = yy_current_state;
        yy_last_accepting_cpos = yy_cp;
    }
    while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
        + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
    {
        yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
        if yy_current_state >= 191 as libc::c_int {
            yy_c = yy_meta[yy_c as libc::c_uint as usize];
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_uint)
        .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 190 as libc::c_int) as libc::c_int;
    return if yy_is_jam != 0 { 0 as libc::c_int } else { yy_current_state };
}
unsafe extern "C" fn yyunput(mut c: libc::c_int, mut yy_bp: *mut libc::c_char) {
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_cp = yy_c_buf_p;
    *yy_cp = yy_hold_char;
    if yy_cp
        < ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset(2 as libc::c_int as isize)
    {
        let mut number_to_move: yy_size_t = yy_n_chars
            .wrapping_add(2 as libc::c_int as libc::c_ulong);
        let mut dest: *mut libc::c_char = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(
                ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut libc::c_char;
        let mut source_0: *mut libc::c_char = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(number_to_move as isize) as *mut libc::c_char;
        while source_0
            > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
        {
            source_0 = source_0.offset(-1);
            dest = dest.offset(-1);
            *dest = *source_0;
        }
        yy_cp = yy_cp
            .offset(dest.offset_from(source_0) as libc::c_long as libc::c_int as isize);
        yy_bp = yy_bp
            .offset(dest.offset_from(source_0) as libc::c_long as libc::c_int as isize);
        yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_size;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
        if yy_cp
            < ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                .offset(2 as libc::c_int as isize)
        {
            yy_fatal_error(
                b"flex scanner push-back overflow\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    yy_cp = yy_cp.offset(-1);
    *yy_cp = c as libc::c_char;
    yytext = yy_bp;
    yy_hold_char = *yy_cp;
    yy_c_buf_p = yy_cp;
}
unsafe extern "C" fn input() -> libc::c_int {
    let mut c: libc::c_int = 0;
    *yy_c_buf_p = yy_hold_char;
    if *yy_c_buf_p as libc::c_int == 0 as libc::c_int {
        if yy_c_buf_p
            < &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
                .offset(yy_n_chars as isize) as *mut libc::c_char
        {
            *yy_c_buf_p = '\0' as i32 as libc::c_char;
        } else {
            let mut offset: yy_size_t = yy_c_buf_p.offset_from(yytext) as libc::c_long
                as yy_size_t;
            yy_c_buf_p = yy_c_buf_p.offset(1);
            yy_c_buf_p;
            let mut current_block_10: u64;
            match yy_get_next_buffer() {
                2 => {
                    yyrestart(yyin);
                    current_block_10 = 5943912363581501881;
                }
                1 => {
                    current_block_10 = 5943912363581501881;
                }
                0 => {
                    yy_c_buf_p = yytext.offset(offset as isize);
                    current_block_10 = 7746791466490516765;
                }
                _ => {
                    current_block_10 = 7746791466490516765;
                }
            }
            match current_block_10 {
                7746791466490516765 => {}
                _ => {
                    if yywrap() != 0 {
                        return -(1 as libc::c_int);
                    }
                    if yy_did_buffer_switch_on_eof == 0 {
                        yyrestart(yyin);
                    }
                    return input();
                }
            }
        }
    }
    c = *(yy_c_buf_p as *mut libc::c_uchar) as libc::c_int;
    *yy_c_buf_p = '\0' as i32 as libc::c_char;
    yy_c_buf_p = yy_c_buf_p.offset(1);
    yy_hold_char = *yy_c_buf_p;
    (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
        .yy_at_bol = (c == '\n' as i32) as libc::c_int;
    return c;
}
pub unsafe extern "C" fn yyrestart(mut input_file: *mut FILE) {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yyensure_buffer_stack();
        let ref mut fresh10 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh10 = yy_create_buffer(yyin, 16384 as libc::c_int);
    }
    yy_init_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
        input_file,
    );
    yy_load_buffer_state();
}
pub unsafe extern "C" fn yy_switch_to_buffer(mut new_buffer: YY_BUFFER_STATE) {
    yyensure_buffer_stack();
    if (if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }) == new_buffer
    {
        return;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh11 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh11 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    let ref mut fresh12 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh12 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
unsafe extern "C" fn yy_load_buffer_state() {
    yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars;
    yy_c_buf_p = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_pos;
    yytext = yy_c_buf_p;
    yyin = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
}
pub unsafe extern "C" fn yy_create_buffer(
    mut file: *mut FILE,
    mut size: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b = yyalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size as yy_size_t;
    (*b)
        .yy_ch_buf = yyalloc(
        ((*b).yy_buf_size).wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*b).yy_ch_buf).is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_create_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    yy_init_buffer(b, file);
    return b;
}
pub unsafe extern "C" fn yy_delete_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        let ref mut fresh13 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh13 = 0 as YY_BUFFER_STATE;
    }
    if (*b).yy_is_our_buffer != 0 {
        yyfree((*b).yy_ch_buf as *mut libc::c_void);
    }
    yyfree(b as *mut libc::c_void);
}
unsafe extern "C" fn yy_init_buffer(mut b: YY_BUFFER_STATE, mut file: *mut FILE) {
    let mut oerrno: libc::c_int = *__errno_location();
    yy_flush_buffer(b);
    (*b).yy_input_file = file;
    (*b).yy_fill_buffer = 1 as libc::c_int;
    if b
        != (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        (*b).yy_bs_lineno = 1 as libc::c_int;
        (*b).yy_bs_column = 0 as libc::c_int;
    }
    (*b)
        .yy_is_interactive = if !file.is_null() {
        (isatty(fileno(file)) > 0 as libc::c_int) as libc::c_int
    } else {
        0 as libc::c_int
    };
    *__errno_location() = oerrno;
}
pub unsafe extern "C" fn yy_flush_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() {
        return;
    }
    (*b).yy_n_chars = 0 as libc::c_int as yy_size_t;
    *((*b).yy_ch_buf)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    *((*b).yy_ch_buf)
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    (*b)
        .yy_buf_pos = &mut *((*b).yy_ch_buf).offset(0 as libc::c_int as isize)
        as *mut libc::c_char;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    if b
        == (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        })
    {
        yy_load_buffer_state();
    }
}
pub unsafe extern "C" fn yypush_buffer_state(mut new_buffer: YY_BUFFER_STATE) {
    if new_buffer.is_null() {
        return;
    }
    yyensure_buffer_stack();
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh14 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh14 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_add(1);
        yy_buffer_stack_top;
    }
    let ref mut fresh15 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh15 = new_buffer;
    yy_load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
pub unsafe extern "C" fn yypop_buffer_state() {
    if if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        return;
    }
    yy_delete_buffer(
        if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else {
            0 as YY_BUFFER_STATE
        },
    );
    let ref mut fresh16 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh16 = 0 as YY_BUFFER_STATE;
    if yy_buffer_stack_top > 0 as libc::c_int as libc::c_ulong {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_sub(1);
        yy_buffer_stack_top;
    }
    if !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_load_buffer_state();
        yy_did_buffer_switch_on_eof = 1 as libc::c_int;
    }
}
unsafe extern "C" fn yyensure_buffer_stack() {
    let mut num_to_alloc: yy_size_t = 0;
    if yy_buffer_stack.is_null() {
        num_to_alloc = 1 as libc::c_int as yy_size_t;
        yy_buffer_stack = yyalloc(
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack as *mut libc::c_void,
            0 as libc::c_int,
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc;
        yy_buffer_stack_top = 0 as libc::c_int as size_t;
        return;
    }
    if yy_buffer_stack_top
        >= yy_buffer_stack_max.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut grow_size: yy_size_t = 8 as libc::c_int as yy_size_t;
        num_to_alloc = yy_buffer_stack_max.wrapping_add(grow_size);
        yy_buffer_stack = yyrealloc(
            yy_buffer_stack as *mut libc::c_void,
            num_to_alloc
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        ) as *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(
                b"out of dynamic memory in yyensure_buffer_stack()\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memset(
            yy_buffer_stack.offset(yy_buffer_stack_max as isize) as *mut libc::c_void,
            0 as libc::c_int,
            grow_size
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc;
    }
}
pub unsafe extern "C" fn yy_scan_buffer(
    mut base: *mut libc::c_char,
    mut size: yy_size_t,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if size < 2 as libc::c_int as libc::c_ulong
        || *base.offset(size.wrapping_sub(2 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != 0 as libc::c_int
        || *base.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != 0 as libc::c_int
    {
        return 0 as YY_BUFFER_STATE;
    }
    b = yyalloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
        as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_buffer()\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*b).yy_buf_size = size.wrapping_sub(2 as libc::c_int as libc::c_ulong);
    (*b).yy_ch_buf = base;
    (*b).yy_buf_pos = (*b).yy_ch_buf;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    (*b).yy_input_file = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size;
    (*b).yy_is_interactive = 0 as libc::c_int;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_fill_buffer = 0 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    yy_switch_to_buffer(b);
    return b;
}
pub unsafe extern "C" fn yy_scan_string(
    mut yystr: *const libc::c_char,
) -> YY_BUFFER_STATE {
    return yy_scan_bytes(yystr, strlen(yystr));
}
pub unsafe extern "C" fn yy_scan_bytes(
    mut yybytes: *const libc::c_char,
    mut _yybytes_len: yy_size_t,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: yy_size_t = 0;
    n = _yybytes_len.wrapping_add(2 as libc::c_int as libc::c_ulong);
    buf = yyalloc(n) as *mut libc::c_char;
    if buf.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_bytes()\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int as yy_size_t;
    while i < _yybytes_len {
        *buf.offset(i as isize) = *yybytes.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    let ref mut fresh17 = *buf
        .offset(_yybytes_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
    *fresh17 = 0 as libc::c_int as libc::c_char;
    *buf.offset(_yybytes_len as isize) = *fresh17;
    b = yy_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(
            b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char) -> ! {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, msg);
    exit(2 as libc::c_int);
}
pub unsafe extern "C" fn yyget_lineno() -> libc::c_int {
    return yylineno;
}
pub unsafe extern "C" fn yyget_in() -> *mut FILE {
    return yyin;
}
pub unsafe extern "C" fn yyget_out() -> *mut FILE {
    return yyout;
}
pub unsafe extern "C" fn yyget_leng() -> yy_size_t {
    return yyleng;
}
pub unsafe extern "C" fn yyget_text() -> *mut libc::c_char {
    return yytext;
}
pub unsafe extern "C" fn yyset_lineno(mut _line_number: libc::c_int) {
    yylineno = _line_number;
}
pub unsafe extern "C" fn yyset_in(mut _in_str: *mut FILE) {
    yyin = _in_str;
}
pub unsafe extern "C" fn yyset_out(mut _out_str: *mut FILE) {
    yyout = _out_str;
}
pub unsafe extern "C" fn yyget_debug() -> libc::c_int {
    return yy_flex_debug;
}
pub unsafe extern "C" fn yyset_debug(mut _bdebug: libc::c_int) {
    yy_flex_debug = _bdebug;
}
unsafe extern "C" fn yy_init_globals() -> libc::c_int {
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_buffer_stack_top = 0 as libc::c_int as size_t;
    yy_buffer_stack_max = 0 as libc::c_int as size_t;
    yy_c_buf_p = 0 as *mut libc::c_char;
    yy_init = 0 as libc::c_int;
    yy_start = 0 as libc::c_int;
    yyin = 0 as *mut FILE;
    yyout = 0 as *mut FILE;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yylex_destroy() -> libc::c_int {
    while !if !yy_buffer_stack.is_null() {
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
    } else {
        0 as YY_BUFFER_STATE
    }
        .is_null()
    {
        yy_delete_buffer(
            if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else {
                0 as YY_BUFFER_STATE
            },
        );
        let ref mut fresh18 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh18 = 0 as YY_BUFFER_STATE;
        yypop_buffer_state();
    }
    yyfree(yy_buffer_stack as *mut libc::c_void);
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_init_globals();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn yyalloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size);
}
pub unsafe extern "C" fn yyrealloc(
    mut ptr: *mut libc::c_void,
    mut size: yy_size_t,
) -> *mut libc::c_void {
    return realloc(ptr as *mut libc::c_char as *mut libc::c_void, size);
}
pub unsafe extern "C" fn yyfree(mut ptr: *mut libc::c_void) {
    free(ptr as *mut libc::c_char as *mut libc::c_void);
}
