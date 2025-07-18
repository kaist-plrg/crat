use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type table_entry;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
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
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut verbose: libc::c_int;
    static mut use_indentation: libc::c_int;
    static mut strict_ansi: libc::c_int;
    static mut debug: libc::c_int;
    static mut omit_arguments_option: libc::c_int;
    static mut omit_symbol_names_option: libc::c_int;
    fn globals_only() -> libc::c_int;
    fn lookup(_: *const libc::c_char) -> *mut Symbol;
    fn install(_: *mut libc::c_char, _: libc::c_int) -> *mut Symbol;
    fn install_ident(name: *mut libc::c_char, storage: storage) -> *mut Symbol;
    fn init_ident(sp: *mut Symbol, storage: storage);
    fn ident_change_storage(sp: *mut Symbol, storage: storage);
    fn delete_autos(level_0: libc::c_int);
    fn delete_parms(level_0: libc::c_int);
    fn move_parms(level_0: libc::c_int);
    fn linked_list_create(fun: linked_list_free_data_fp) -> *mut linked_list;
    fn linked_list_append(plist: *mut *mut linked_list, data: *mut libc::c_void);
    fn data_in_list(data: *mut libc::c_void, list: *mut linked_list) -> libc::c_int;
    fn get_token() -> libc::c_int;
    static mut yylval: YYSTYPE;
    static mut filename: *mut libc::c_char;
    static mut line_num: libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ref {
    pub line: libc::c_int,
    pub source: *mut libc::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TOKSTK {
    pub type_0: libc::c_int,
    pub token: *mut libc::c_char,
    pub line: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ident {
    pub name: *mut libc::c_char,
    pub type_end: libc::c_int,
    pub parmcnt: libc::c_int,
    pub line: libc::c_int,
    pub storage: storage,
}
pub const STRING: C2RustUnnamed_2 = 271;
pub const TYPEDEF: C2RustUnnamed_2 = 263;
pub const STATIC: C2RustUnnamed_2 = 262;
pub const EXTERN: C2RustUnnamed_2 = 261;
pub const RBRACE: C2RustUnnamed_2 = 125;
pub const RBRACE0: C2RustUnnamed_2 = 259;
pub const LBRACE: C2RustUnnamed_2 = 123;
pub const LBRACE0: C2RustUnnamed_2 = 258;
pub const OP: C2RustUnnamed_2 = 266;
pub const QUALIFIER: C2RustUnnamed_2 = 273;
pub const PARM_WRAPPER: C2RustUnnamed_2 = 272;
pub const STRUCT: C2RustUnnamed_2 = 264;
pub const MODIFIER: C2RustUnnamed_2 = 265;
pub const WORD: C2RustUnnamed_2 = 257;
pub const TYPE: C2RustUnnamed_2 = 270;
pub const IDENTIFIER: C2RustUnnamed_2 = 260;
pub const _ISprint: C2RustUnnamed_3 = 16384;
pub const MEMBER_OF: C2RustUnnamed_2 = 269;
pub const ENUM: C2RustUnnamed_2 = 268;
pub const UNION: C2RustUnnamed_2 = 267;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YYSTYPE {
    pub str_0: *mut libc::c_char,
}
pub type Stackpos = [libc::c_int; 1];
pub type C2RustUnnamed_2 = libc::c_uint;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_3 = 8;
pub const _ISpunct: C2RustUnnamed_3 = 4;
pub const _IScntrl: C2RustUnnamed_3 = 2;
pub const _ISblank: C2RustUnnamed_3 = 1;
pub const _ISgraph: C2RustUnnamed_3 = 32768;
pub const _ISspace: C2RustUnnamed_3 = 8192;
pub const _ISxdigit: C2RustUnnamed_3 = 4096;
pub const _ISdigit: C2RustUnnamed_3 = 2048;
pub const _ISalpha: C2RustUnnamed_3 = 1024;
pub const _ISlower: C2RustUnnamed_3 = 512;
pub const _ISupper: C2RustUnnamed_3 = 256;
static mut level: libc::c_int = 0;
static mut caller: *mut Symbol = 0 as *const Symbol as *mut Symbol;
static mut text_stk: obstack = obstack {
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
static mut parm_level: libc::c_int = 0;
static mut tok: TOKSTK = TOKSTK {
    type_0: 0,
    token: 0 as *const libc::c_char as *mut libc::c_char,
    line: 0,
};
static mut token_stack: *mut TOKSTK = 0 as *const TOKSTK as *mut TOKSTK;
static mut tos: libc::c_int = 0;
static mut curs: libc::c_int = 0;
pub static mut token_stack_length: libc::c_int = 64 as libc::c_int;
pub static mut token_stack_increase: libc::c_int = 32 as libc::c_int;
static mut need_space: libc::c_int = 0;
unsafe extern "C" fn print_token(mut tokptr: *mut TOKSTK) {
    match (*tokptr).type_0 {
        260 | 270 | 257 | 265 | 264 | 272 | 273 | 266 => {
            fprintf(
                stderr,
                b"`%s'\0" as *const u8 as *const libc::c_char,
                (*tokptr).token,
            );
        }
        258 | 123 => {
            fprintf(stderr, b"`{'\0" as *const u8 as *const libc::c_char);
        }
        259 | 125 => {
            fprintf(stderr, b"`}'\0" as *const u8 as *const libc::c_char);
        }
        261 => {
            fprintf(stderr, b"`extern'\0" as *const u8 as *const libc::c_char);
        }
        262 => {
            fprintf(stderr, b"`static'\0" as *const u8 as *const libc::c_char);
        }
        263 => {
            fprintf(stderr, b"`typedef'\0" as *const u8 as *const libc::c_char);
        }
        271 => {
            fprintf(
                stderr,
                b"\"%s\"\0" as *const u8 as *const libc::c_char,
                (*tokptr).token,
            );
        }
        _ => {
            fprintf(
                stderr,
                b"`%c'\0" as *const u8 as *const libc::c_char,
                (*tokptr).type_0,
            );
        }
    };
}
unsafe extern "C" fn token_type_str(mut t: libc::c_int) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 80] = [0; 80];
    match t {
        0 => return b"EOF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        257 => return b"WORD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        258 => return b"'{'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        259 => return b"'}'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        260 => {
            return b"IDENTIFIER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        261 => {
            return b"EXTERN\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        262 => {
            return b"STATIC\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        263 => {
            return b"TYPEDEF\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        264 => {
            return b"STRUCT\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        265 => {
            return b"MODIFIER\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        266 => return b"OP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        267 => return b"UNION\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        268 => return b"ENUM\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        123 => return b"' {'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        125 => return b"' }'\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        269 => {
            return b"MEMBER_OF\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        270 => return b"TYPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        271 => {
            return b"STRING\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        272 => {
            return b"PARM_WRAPPER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        273 => {
            return b"QUALIFIER\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        _ => {}
    }
    if *(*__ctype_b_loc()).offset(t as isize) as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
            b"'%c'(%d)\0" as *const u8 as *const libc::c_char,
            t,
            t,
        );
    } else {
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            t,
        );
    }
    return buf.as_mut_ptr();
}
unsafe extern "C" fn dbgtok(mut t: *mut TOKSTK, mut delim: libc::c_int) {
    if delim != 0 {
        fputc(delim, stderr);
    }
    fprintf(
        stderr,
        b"{ %s \0" as *const u8 as *const libc::c_char,
        token_type_str((*t).type_0),
    );
    if (*t).type_0 != 0 {
        fprintf(
            stderr,
            b", %s, %d \0" as *const u8 as *const libc::c_char,
            if !((*t).token).is_null() {
                (*t).token as *const libc::c_char
            } else {
                b"NULL\0" as *const u8 as *const libc::c_char
            },
            (*t).line,
        );
    }
    fputc('}' as i32, stderr);
}
unsafe extern "C" fn debugtoken(
    mut t: *mut TOKSTK,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    if debug > 1 as libc::c_int {
        let mut ap: ::std::ffi::VaListImpl;
        let mut i: libc::c_int = 0;
        if !fmt.is_null() {
            ap = args.clone();
            vfprintf(stderr, fmt, ap.as_va_list());
            fprintf(stderr, b": \0" as *const u8 as *const libc::c_char);
        }
        if !t.is_null() {
            dbgtok(t, 0 as libc::c_int);
            fprintf(stderr, b"; \0" as *const u8 as *const libc::c_char);
        }
        fprintf(stderr, b"%d: {\0" as *const u8 as *const libc::c_char, curs);
        i = curs;
        while i < tos {
            dbgtok(
                token_stack.offset(i as isize),
                if i == curs { 0 as libc::c_int } else { ',' as i32 },
            );
            i += 1;
            i;
        }
        fprintf(stderr, b"}\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn file_error(mut msg: *mut libc::c_char, mut tokptr: *mut TOKSTK) {
    fprintf(
        stderr,
        b"%s:%d: %s\0" as *const u8 as *const libc::c_char,
        filename,
        tok.line,
        msg,
    );
    if !tokptr.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b" near \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        print_token(tokptr);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn mark(mut pos: *mut libc::c_int) {
    *pos.offset(0 as libc::c_int as isize) = curs;
    if debug > 1 as libc::c_int {
        fprintf(
            stderr,
            b"marking stack at %d\n\0" as *const u8 as *const libc::c_char,
            curs,
        );
    }
}
pub unsafe extern "C" fn restore(mut pos: *mut libc::c_int) {
    curs = *pos.offset(0 as libc::c_int as isize);
    if curs != 0 {
        tok = *token_stack.offset((curs - 1 as libc::c_int) as isize);
        debugtoken(
            &mut tok as *mut TOKSTK,
            b"restored stack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
pub unsafe extern "C" fn tokdel(mut beg: libc::c_int, mut end: libc::c_int) {
    if end >= beg {
        if end < tos {
            memmove(
                token_stack.offset(beg as isize) as *mut libc::c_void,
                token_stack.offset(end as isize).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                ((tos - end - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<TOKSTK>() as libc::c_ulong),
            );
        }
        tos -= end - beg + 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn tokins(
    mut pos: libc::c_int,
    mut type_0: libc::c_int,
    mut line: libc::c_int,
    mut token: *mut libc::c_char,
) {
    tos += 1;
    if tos == token_stack_length {
        token_stack_length += token_stack_increase;
        token_stack = xrealloc(
            token_stack as *mut libc::c_void,
            (token_stack_length as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<TOKSTK>() as libc::c_ulong),
        ) as *mut TOKSTK;
    }
    memmove(
        token_stack.offset(pos as isize).offset(1 as libc::c_int as isize)
            as *mut libc::c_void,
        token_stack.offset(pos as isize) as *const libc::c_void,
        ((tos - pos - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TOKSTK>() as libc::c_ulong),
    );
    (*token_stack.offset(pos as isize)).type_0 = type_0;
    let ref mut fresh0 = (*token_stack.offset(pos as isize)).token;
    *fresh0 = token;
    (*token_stack.offset(pos as isize)).line = line;
    debugtoken(
        &mut *token_stack.offset(pos as isize) as *mut TOKSTK,
        b"insert at %d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pos,
    );
}
pub unsafe extern "C" fn tokpush(
    mut type_0: libc::c_int,
    mut line: libc::c_int,
    mut token: *mut libc::c_char,
) {
    (*token_stack.offset(tos as isize)).type_0 = type_0;
    let ref mut fresh1 = (*token_stack.offset(tos as isize)).token;
    *fresh1 = token;
    (*token_stack.offset(tos as isize)).line = line;
    tos += 1;
    if tos == token_stack_length {
        token_stack_length += token_stack_increase;
        token_stack = xrealloc(
            token_stack as *mut libc::c_void,
            (token_stack_length as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<TOKSTK>() as libc::c_ulong),
        ) as *mut TOKSTK;
    }
}
pub unsafe extern "C" fn cleanup_stack() {
    let mut delta: libc::c_int = tos - curs;
    if delta > 0 as libc::c_int {
        memmove(
            token_stack as *mut libc::c_void,
            token_stack.offset(curs as isize) as *const libc::c_void,
            (delta as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<TOKSTK>() as libc::c_ulong),
        );
    } else if delta < 0 as libc::c_int {
        delta = 0 as libc::c_int;
    }
    tos = delta;
    curs = 0 as libc::c_int;
}
pub unsafe extern "C" fn clearstack() {
    curs = 0 as libc::c_int;
    tos = curs;
}
pub unsafe extern "C" fn nexttoken() -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    if curs == tos {
        type_0 = get_token();
        tokpush(type_0, line_num, yylval.str_0);
        yylval.str_0 = 0 as *mut libc::c_char;
    }
    tok = *token_stack.offset(curs as isize);
    curs += 1;
    curs;
    debugtoken(
        &mut tok as *mut TOKSTK,
        b"next token\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return tok.type_0;
}
pub unsafe extern "C" fn putback() -> libc::c_int {
    if curs == 0 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"INTERNAL ERROR: cannot return token to stream\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        abort();
    }
    curs -= 1;
    curs;
    if curs > 0 as libc::c_int {
        tok = *token_stack.offset((curs - 1 as libc::c_int) as isize);
    } else {
        tok.type_0 = 0 as libc::c_int;
    }
    debugtoken(
        &mut tok as *mut TOKSTK,
        b"putback\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return tok.type_0;
}
pub unsafe extern "C" fn init_parse() {
    _obstack_begin(
        &mut text_stk,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    token_stack = xmalloc(
        (token_stack_length as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TOKSTK>() as libc::c_ulong),
    ) as *mut TOKSTK;
    clearstack();
}
pub unsafe extern "C" fn save_token(mut tokptr: *mut TOKSTK) {
    let mut len: libc::c_int = 0;
    match (*tokptr).type_0 {
        260 | 270 | 264 | 272 | 257 | 273 => {
            if need_space != 0 {
                let mut __o: *mut obstack = &mut text_stk;
                if ({
                    let mut __o1: *const obstack = __o;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < 1 as libc::c_int as libc::c_ulong
                {
                    _obstack_newchunk(__o, 1 as libc::c_int as size_t);
                }
                let fresh2 = (*__o).next_free;
                (*__o).next_free = ((*__o).next_free).offset(1);
                *fresh2 = ' ' as i32 as libc::c_char;
            }
            len = strlen((*tokptr).token) as libc::c_int;
            let mut __o_0: *mut obstack = &mut text_stk;
            let mut __len: size_t = len as size_t;
            if ({
                let mut __o1: *const obstack = __o_0;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len
            {
                _obstack_newchunk(__o_0, __len);
            }
            memcpy(
                (*__o_0).next_free as *mut libc::c_void,
                (*tokptr).token as *const libc::c_void,
                __len,
            );
            (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
            need_space = 1 as libc::c_int;
        }
        265 => {
            if need_space != 0 {
                let mut __o_1: *mut obstack = &mut text_stk;
                if ({
                    let mut __o1: *const obstack = __o_1;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < 1 as libc::c_int as libc::c_ulong
                {
                    _obstack_newchunk(__o_1, 1 as libc::c_int as size_t);
                }
                let fresh3 = (*__o_1).next_free;
                (*__o_1).next_free = ((*__o_1).next_free).offset(1);
                *fresh3 = ' ' as i32 as libc::c_char;
            }
            if *((*tokptr).token).offset(0 as libc::c_int as isize) as libc::c_int
                == '*' as i32
            {
                need_space = 0 as libc::c_int;
            } else {
                need_space = 1 as libc::c_int;
            }
            len = strlen((*tokptr).token) as libc::c_int;
            let mut __o_2: *mut obstack = &mut text_stk;
            let mut __len_0: size_t = len as size_t;
            if ({
                let mut __o1: *const obstack = __o_2;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len_0
            {
                _obstack_newchunk(__o_2, __len_0);
            }
            memcpy(
                (*__o_2).next_free as *mut libc::c_void,
                (*tokptr).token as *const libc::c_void,
                __len_0,
            );
            (*__o_2).next_free = ((*__o_2).next_free).offset(__len_0 as isize);
        }
        261 | 262 => {}
        44 => {
            let mut __o_3: *mut obstack = &mut text_stk;
            if ({
                let mut __o1: *const obstack = __o_3;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_3, 1 as libc::c_int as size_t);
            }
            let fresh4 = (*__o_3).next_free;
            (*__o_3).next_free = ((*__o_3).next_free).offset(1);
            *fresh4 = ',' as i32 as libc::c_char;
            need_space = 1 as libc::c_int;
        }
        40 => {
            if need_space != 0 {
                let mut __o_4: *mut obstack = &mut text_stk;
                if ({
                    let mut __o1: *const obstack = __o_4;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < 1 as libc::c_int as libc::c_ulong
                {
                    _obstack_newchunk(__o_4, 1 as libc::c_int as size_t);
                }
                let fresh5 = (*__o_4).next_free;
                (*__o_4).next_free = ((*__o_4).next_free).offset(1);
                *fresh5 = ' ' as i32 as libc::c_char;
            }
            let mut __o_5: *mut obstack = &mut text_stk;
            if ({
                let mut __o1: *const obstack = __o_5;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_5, 1 as libc::c_int as size_t);
            }
            let fresh6 = (*__o_5).next_free;
            (*__o_5).next_free = ((*__o_5).next_free).offset(1);
            *fresh6 = (*tokptr).type_0 as libc::c_char;
            need_space = 0 as libc::c_int;
        }
        41 => {
            let mut __o_6: *mut obstack = &mut text_stk;
            if ({
                let mut __o1: *const obstack = __o_6;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_6, 1 as libc::c_int as size_t);
            }
            let fresh7 = (*__o_6).next_free;
            (*__o_6).next_free = ((*__o_6).next_free).offset(1);
            *fresh7 = (*tokptr).type_0 as libc::c_char;
            need_space = 1 as libc::c_int;
        }
        91 | 93 => {
            let mut __o_7: *mut obstack = &mut text_stk;
            if ({
                let mut __o1: *const obstack = __o_7;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_7, 1 as libc::c_int as size_t);
            }
            let fresh8 = (*__o_7).next_free;
            (*__o_7).next_free = ((*__o_7).next_free).offset(1);
            *fresh8 = (*tokptr).type_0 as libc::c_char;
            need_space = 0 as libc::c_int;
        }
        123 | 258 => {
            if need_space != 0 {
                let mut __o_8: *mut obstack = &mut text_stk;
                if ({
                    let mut __o1: *const obstack = __o_8;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < 1 as libc::c_int as libc::c_ulong
                {
                    _obstack_newchunk(__o_8, 1 as libc::c_int as size_t);
                }
                let fresh9 = (*__o_8).next_free;
                (*__o_8).next_free = ((*__o_8).next_free).offset(1);
                *fresh9 = ' ' as i32 as libc::c_char;
            }
            let mut __o_9: *mut obstack = &mut text_stk;
            if ({
                let mut __o1: *const obstack = __o_9;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_9, 1 as libc::c_int as size_t);
            }
            let fresh10 = (*__o_9).next_free;
            (*__o_9).next_free = ((*__o_9).next_free).offset(1);
            *fresh10 = '{' as i32 as libc::c_char;
            need_space = 1 as libc::c_int;
        }
        125 | 259 => {
            if need_space != 0 {
                let mut __o_10: *mut obstack = &mut text_stk;
                if ({
                    let mut __o1: *const obstack = __o_10;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                        as size_t
                }) < 1 as libc::c_int as libc::c_ulong
                {
                    _obstack_newchunk(__o_10, 1 as libc::c_int as size_t);
                }
                let fresh11 = (*__o_10).next_free;
                (*__o_10).next_free = ((*__o_10).next_free).offset(1);
                *fresh11 = ' ' as i32 as libc::c_char;
            }
            let mut __o_11: *mut obstack = &mut text_stk;
            if ({
                let mut __o1: *const obstack = __o_11;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_11, 1 as libc::c_int as size_t);
            }
            let fresh12 = (*__o_11).next_free;
            (*__o_11).next_free = ((*__o_11).next_free).offset(1);
            *fresh12 = '}' as i32 as libc::c_char;
            need_space = 1 as libc::c_int;
        }
        266 => {
            let mut __o_12: *mut obstack = &mut text_stk;
            if ({
                let mut __o1: *const obstack = __o_12;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o_12, 1 as libc::c_int as size_t);
            }
            let fresh13 = (*__o_12).next_free;
            (*__o_12).next_free = ((*__o_12).next_free).offset(1);
            *fresh13 = ' ' as i32 as libc::c_char;
            let mut __o_13: *mut obstack = &mut text_stk;
            let mut __len_1: size_t = strlen((*tokptr).token);
            if ({
                let mut __o1: *const obstack = __o_13;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < __len_1
            {
                _obstack_newchunk(__o_13, __len_1);
            }
            memcpy(
                (*__o_13).next_free as *mut libc::c_void,
                (*tokptr).token as *const libc::c_void,
                __len_1,
            );
            (*__o_13).next_free = ((*__o_13).next_free).offset(__len_1 as isize);
            need_space = 1 as libc::c_int;
        }
        _ => {
            if verbose != 0 {
                file_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unrecognized definition\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    tokptr,
                );
            }
        }
    };
}
static mut start_pos: Stackpos = [0; 1];
static mut save_end: libc::c_int = 0;
pub unsafe extern "C" fn save_stack() {
    mark(start_pos.as_mut_ptr());
    save_end = curs - 1 as libc::c_int;
}
pub unsafe extern "C" fn undo_save_stack() {
    save_end = -(1 as libc::c_int);
}
pub unsafe extern "C" fn save_stack_is_empty() -> libc::c_int {
    return (save_end <= 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn finish_save_stack(
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut level_0: libc::c_int = 0 as libc::c_int;
    let mut found_ident: libc::c_int = (omit_symbol_names_option == 0) as libc::c_int;
    need_space = 0 as libc::c_int;
    let mut current_block_13: u64;
    i = 0 as libc::c_int;
    while i < save_end {
        match (*token_stack.offset(i as isize)).type_0 {
            40 => {
                if omit_arguments_option != 0 {
                    if level_0 == 0 as libc::c_int {
                        save_token(token_stack.offset(i as isize));
                    }
                    level_0 += 1;
                    level_0;
                }
                current_block_13 = 13056961889198038528;
            }
            41 => {
                if omit_arguments_option != 0 {
                    level_0 -= 1;
                    level_0;
                }
                current_block_13 = 13056961889198038528;
            }
            260 => {
                if found_ident == 0
                    && strcmp(name, (*token_stack.offset(i as isize)).token)
                        == 0 as libc::c_int
                {
                    need_space = 1 as libc::c_int;
                    found_ident = 1 as libc::c_int;
                    current_block_13 = 4988723283678924448;
                } else {
                    current_block_13 = 13056961889198038528;
                }
            }
            _ => {
                current_block_13 = 13056961889198038528;
            }
        }
        match current_block_13 {
            13056961889198038528 => {
                if level_0 == 0 as libc::c_int {
                    save_token(token_stack.offset(i as isize));
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    let mut __o: *mut obstack = &mut text_stk;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o, 1 as libc::c_int as size_t);
    }
    let fresh14 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh14 = 0 as libc::c_int as libc::c_char;
    return ({
        let mut __o1: *mut obstack = &mut text_stk as *mut obstack;
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
}
pub unsafe extern "C" fn skip_to(mut c: libc::c_int) {
    while nexttoken() != 0 {
        if tok.type_0 == c {
            break;
        }
    }
}
pub unsafe extern "C" fn skip_balanced(
    mut open_tok: libc::c_int,
    mut close_tok: libc::c_int,
    mut level_0: libc::c_int,
) -> libc::c_int {
    if level_0 == 0 as libc::c_int {
        if nexttoken() != open_tok {
            return 1 as libc::c_int;
        }
        level_0 += 1;
        level_0;
    }
    while nexttoken() != 0 {
        if tok.type_0 == LBRACE0 as libc::c_int && open_tok == '{' as i32 {
            tok.type_0 = '{' as i32;
        } else if tok.type_0 == RBRACE0 as libc::c_int && close_tok == '}' as i32 {
            tok.type_0 = '}' as i32;
        }
        if tok.type_0 == open_tok {
            level_0 += 1;
            level_0;
        } else if tok.type_0 == close_tok {
            level_0 -= 1;
            if level_0 == 0 as libc::c_int {
                nexttoken();
                return 0 as libc::c_int;
            }
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut identifier: Ident = Ident {
        name: 0 as *mut libc::c_char,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: ExternStorage,
    };
    level = 0 as libc::c_int;
    caller = 0 as *mut Symbol;
    clearstack();
    let mut current_block_11: u64;
    while nexttoken() != 0 {
        identifier.storage = ExternStorage;
        match tok.type_0 {
            0 => return 0 as libc::c_int,
            273 => {
                continue;
            }
            263 => {
                parse_typedef();
                current_block_11 = 5399440093318478209;
            }
            261 => {
                identifier.storage = ExplicitExternStorage;
                parse_declaration(&mut identifier, 0 as libc::c_int);
                current_block_11 = 5399440093318478209;
            }
            262 => {
                identifier.storage = StaticStorage;
                nexttoken();
                current_block_11 = 15963427805601190657;
            }
            _ => {
                current_block_11 = 15963427805601190657;
            }
        }
        match current_block_11 {
            15963427805601190657 => {
                parse_declaration(&mut identifier, 0 as libc::c_int);
            }
            _ => {}
        }
        cleanup_stack();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_function() -> libc::c_int {
    let mut sp: Stackpos = [0; 1];
    let mut res: libc::c_int = 0 as libc::c_int;
    mark(sp.as_mut_ptr());
    loop {
        match tok.type_0 {
            273 | 270 | 260 | 265 | 262 | 261 | 264 | 267 | 268 => {
                nexttoken();
            }
            272 => {
                if skip_balanced('(' as i32, ')' as i32, 0 as libc::c_int)
                    == -(1 as libc::c_int)
                {
                    file_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected end of file in declaration\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        0 as *mut TOKSTK,
                    );
                }
            }
            40 => {
                res = (nexttoken() != MODIFIER as libc::c_int) as libc::c_int;
                break;
            }
            _ => {
                break;
            }
        }
    }
    restore(sp.as_mut_ptr());
    return res;
}
pub unsafe extern "C" fn parse_declaration(
    mut ident: *mut Ident,
    mut parm: libc::c_int,
) {
    if is_function() != 0 {
        parse_function_declaration(ident, parm);
    } else {
        parse_variable_declaration(ident, parm);
    }
    delete_parms(parm_level);
}
pub unsafe extern "C" fn expression() {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: libc::c_int = 0;
    let mut parens_lev: libc::c_int = 0;
    parens_lev = 0 as libc::c_int;
    loop {
        match tok.type_0 {
            59 => return,
            272 => {
                if skip_balanced('(' as i32, ')' as i32, 0 as libc::c_int)
                    == -(1 as libc::c_int)
                {
                    file_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected end of file in expression\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        0 as *mut TOKSTK,
                    );
                    return;
                }
                putback();
            }
            123 | 258 | 125 | 259 => {
                putback();
                return;
            }
            44 => {
                if parens_lev == 0 as libc::c_int {
                    return;
                }
            }
            0 => {
                if verbose != 0 {
                    file_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected end of file in expression\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        0 as *mut TOKSTK,
                    );
                }
                return;
            }
            260 => {
                name = tok.token;
                line = tok.line;
                nexttoken();
                if tok.type_0 == '(' as i32 {
                    call(name, line);
                    parens_lev += 1;
                    parens_lev;
                } else {
                    reference(name, line);
                    while parens_lev > 0 as libc::c_int && tok.type_0 == ')' as i32 {
                        parens_lev -= 1;
                        parens_lev;
                        nexttoken();
                    }
                    if tok.type_0 == MEMBER_OF as libc::c_int {
                        nexttoken();
                    } else {
                        putback();
                    }
                }
            }
            40 => {
                if nexttoken() == TYPE as libc::c_int
                    || tok.type_0 == STRUCT as libc::c_int
                {
                    if skip_balanced('(' as i32, ')' as i32, 1 as libc::c_int)
                        == -(1 as libc::c_int)
                    {
                        file_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unexpected end of file in expression\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            0 as *mut TOKSTK,
                        );
                        return;
                    }
                    if tok.type_0 == LBRACE as libc::c_int
                        || tok.type_0 == LBRACE0 as libc::c_int
                    {
                        if skip_balanced('{' as i32, '}' as i32, 1 as libc::c_int)
                            == -(1 as libc::c_int)
                        {
                            file_error(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"unexpected end of file in expression\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                0 as *mut TOKSTK,
                            );
                            return;
                        }
                    }
                } else {
                    parens_lev += 1;
                    parens_lev;
                }
                putback();
            }
            41 => {
                parens_lev -= 1;
                parens_lev;
            }
            269 => {
                nexttoken();
            }
            _ => {}
        }
        nexttoken();
    };
}
pub unsafe extern "C" fn parse_function_declaration(
    mut ident: *mut Ident,
    mut parm: libc::c_int,
) {
    let mut error_recovery: libc::c_int = 0 as libc::c_int;
    (*ident).type_end = -(1 as libc::c_int);
    parse_knr_dcl(ident);
    loop {
        match tok.type_0 {
            41 => {
                if parm != 0 {
                    break;
                }
            }
            59 | 44 => {
                break;
            }
            258 | 123 => {
                if !((*ident).name).is_null() {
                    caller = lookup((*ident).name);
                    if !caller.is_null()
                        && ((*caller).storage as libc::c_uint
                            == AutoStorage as libc::c_int as libc::c_uint
                            || (*caller).flag as libc::c_uint
                                == symbol_target as libc::c_int as libc::c_uint)
                    {
                        caller = 0 as *mut Symbol;
                    }
                    func_body();
                }
                break;
            }
            0 => {
                if verbose != 0 {
                    file_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected end of file in declaration\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        0 as *mut TOKSTK,
                    );
                }
                break;
            }
            _ => {}
        }
        if error_recovery != 0 {
            nexttoken();
        } else {
            if verbose != 0 {
                file_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"expected `;'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    &mut tok,
                );
            }
            error_recovery = 1 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn fake_struct(mut ident: *mut Ident) -> libc::c_int {
    (*ident).type_end = -(1 as libc::c_int);
    if tok.type_0 == STRUCT as libc::c_int {
        if nexttoken() == IDENTIFIER as libc::c_int {
            (*ident).type_end = curs;
        }
        putback();
        skip_struct();
        if tok.type_0 == IDENTIFIER as libc::c_int
            || tok.type_0 == MODIFIER as libc::c_int
            || tok.type_0 == QUALIFIER as libc::c_int
        {
            putback();
        } else if tok.type_0 == '(' as i32 {
            return 0 as libc::c_int
        } else if tok.type_0 != ';' as i32 {
            file_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"missing `;' after struct declaration\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                &mut tok,
            );
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn parse_variable_declaration(
    mut ident: *mut Ident,
    mut parm: libc::c_int,
) {
    let mut current_block: u64;
    let mut sp: Stackpos = [0; 1];
    mark(sp.as_mut_ptr());
    (*ident).type_end = -(1 as libc::c_int);
    if tok.type_0 == STRUCT as libc::c_int || tok.type_0 == UNION as libc::c_int {
        if nexttoken() == IDENTIFIER as libc::c_int {
            (*ident).type_end = curs;
        }
        putback();
        skip_struct();
        while tok.type_0 == MODIFIER as libc::c_int
            || tok.type_0 == QUALIFIER as libc::c_int
        {
            nexttoken();
        }
        if tok.type_0 == IDENTIFIER as libc::c_int {
            if (*ident).type_end == -(1 as libc::c_int) {
                let mut pos: libc::c_int = curs - 1 as libc::c_int;
                restore(sp.as_mut_ptr());
                tokdel(curs, pos - 1 as libc::c_int);
                tokins(
                    curs,
                    IDENTIFIER as libc::c_int,
                    tok.line,
                    b"{ ... }\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                debugtoken(
                    &mut tok as *mut TOKSTK,
                    b"modified stack\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        } else {
            if tok.type_0 == ';' as i32 {
                return;
            }
            restore(sp.as_mut_ptr());
        }
    }
    '_again: loop {
        parse_dcl(ident, 0 as libc::c_int);
        loop {
            match tok.type_0 {
                41 => {
                    if parm != 0 {
                        current_block = 7245201122033322888;
                        break '_again;
                    } else {
                        current_block = 14048827063796126698;
                        break '_again;
                    }
                }
                59 => {
                    current_block = 7245201122033322888;
                    break '_again;
                }
                44 => {
                    if parm != 0 {
                        current_block = 7245201122033322888;
                        break '_again;
                    }
                    tos = (*ident).type_end;
                    restore(sp.as_mut_ptr());
                    break;
                }
                61 => {
                    nexttoken();
                    if tok.type_0 == LBRACE as libc::c_int
                        || tok.type_0 == LBRACE0 as libc::c_int
                    {
                        initializer_list();
                    } else {
                        expression();
                    }
                }
                258 | 123 => {
                    func_body();
                    current_block = 7245201122033322888;
                    break '_again;
                }
                0 => {
                    if verbose != 0 {
                        file_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unexpected end of file in declaration\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            0 as *mut TOKSTK,
                        );
                    }
                    current_block = 7245201122033322888;
                    break '_again;
                }
                _ => {
                    current_block = 14048827063796126698;
                    break '_again;
                }
            }
        }
    }
    match current_block {
        14048827063796126698 => {
            if verbose != 0 {
                file_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"expected `;'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    &mut tok,
                );
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn initializer_list() {
    let mut lev: libc::c_int = 0 as libc::c_int;
    loop {
        match tok.type_0 {
            123 | 258 => {
                lev += 1;
                lev;
            }
            125 | 259 => {
                lev -= 1;
                if lev <= 0 as libc::c_int {
                    nexttoken();
                    return;
                }
            }
            0 => {
                file_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected end of file in initializer list\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as *mut TOKSTK,
                );
                return;
            }
            44 => {}
            _ => {
                expression();
            }
        }
        nexttoken();
    };
}
pub unsafe extern "C" fn parse_knr_dcl(mut ident: *mut Ident) {
    (*ident).type_end = -(1 as libc::c_int);
    parse_dcl(ident, (strict_ansi == 0) as libc::c_int);
}
pub unsafe extern "C" fn skip_struct() {
    if nexttoken() == IDENTIFIER as libc::c_int {
        nexttoken();
    } else if tok.type_0 == ';' as i32 {
        return
    }
    if tok.type_0 == LBRACE as libc::c_int || tok.type_0 == LBRACE0 as libc::c_int {
        if skip_balanced('{' as i32, '}' as i32, 1 as libc::c_int) == -(1 as libc::c_int)
        {
            file_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unexpected end of file in struct\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *mut TOKSTK,
            );
            return;
        }
    }
    while tok.type_0 == PARM_WRAPPER as libc::c_int {
        if skip_balanced('(' as i32, ')' as i32, 0 as libc::c_int) == -(1 as libc::c_int)
        {
            file_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unexpected end of file in struct\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *mut TOKSTK,
            );
            return;
        }
    }
}
pub unsafe extern "C" fn parse_typedef() {
    let mut ident: Ident = Ident {
        name: 0 as *mut libc::c_char,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: ExternStorage,
    };
    ident.name = 0 as *mut libc::c_char;
    ident.type_end = -(1 as libc::c_int);
    ident.parmcnt = -(1 as libc::c_int);
    ident.line = -(1 as libc::c_int);
    ident.storage = AnyStorage;
    nexttoken();
    if fake_struct(&mut ident) == 0 {
        putback();
    }
    dcl(&mut ident);
    if !(ident.name).is_null() {
        declare_type(&mut ident);
    }
}
pub unsafe extern "C" fn parse_dcl(mut ident: *mut Ident, mut maybe_knr: libc::c_int) {
    (*ident).parmcnt = -(1 as libc::c_int);
    (*ident).name = 0 as *mut libc::c_char;
    putback();
    dcl(ident);
    save_stack();
    if !((*ident).name).is_null() {
        declare(ident, maybe_knr);
    } else {
        undo_save_stack();
    };
}
pub unsafe extern "C" fn dcl(mut idptr: *mut Ident) -> libc::c_int {
    while nexttoken() != 0 as libc::c_int && tok.type_0 != '(' as i32 {
        if tok.type_0 == MODIFIER as libc::c_int {
            if !idptr.is_null() && (*idptr).type_end == -(1 as libc::c_int) {
                (*idptr).type_end = curs - 1 as libc::c_int;
            }
        } else if tok.type_0 == PARM_WRAPPER as libc::c_int {
            if skip_balanced('(' as i32, ')' as i32, 0 as libc::c_int)
                == -(1 as libc::c_int)
            {
                file_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected end of file in function declaration\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as *mut TOKSTK,
                );
                return 1 as libc::c_int;
            }
            putback();
        } else if tok.type_0 == IDENTIFIER as libc::c_int {
            let mut type_0: libc::c_int = 0;
            while tok.type_0 == IDENTIFIER as libc::c_int {
                nexttoken();
            }
            type_0 = tok.type_0;
            putback();
            if !(type_0 == TYPE as libc::c_int || type_0 == MODIFIER as libc::c_int
                || type_0 == QUALIFIER as libc::c_int)
            {
                break;
            }
        } else if tok.type_0 == ')' as i32 || tok.type_0 == ';' as i32 {
            return 1 as libc::c_int
        }
    }
    if !idptr.is_null() && (*idptr).type_end == -(1 as libc::c_int) {
        (*idptr).type_end = curs - 1 as libc::c_int;
    }
    return dirdcl(idptr);
}
pub unsafe extern "C" fn dirdcl(mut idptr: *mut Ident) -> libc::c_int {
    let mut wrapper: libc::c_int = 0 as libc::c_int;
    let mut parm_ptr: *mut libc::c_int = 0 as *mut libc::c_int;
    if tok.type_0 == '(' as i32 {
        dcl(idptr);
        if tok.type_0 != ')' as i32 && verbose != 0 {
            file_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"expected `)'\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                &mut tok,
            );
            return 1 as libc::c_int;
        }
    } else if tok.type_0 == IDENTIFIER as libc::c_int {
        if !idptr.is_null() {
            (*idptr).name = tok.token;
            (*idptr).line = tok.line;
            parm_ptr = &mut (*idptr).parmcnt;
        }
    }
    if nexttoken() == PARM_WRAPPER as libc::c_int {
        wrapper = 1 as libc::c_int;
        nexttoken();
    } else {
        putback();
    }
    while nexttoken() == '[' as i32 || tok.type_0 == '(' as i32 {
        if tok.type_0 == '[' as i32 {
            skip_to(']' as i32);
        } else {
            maybe_parm_list(parm_ptr);
            if tok.type_0 != ')' as i32 && verbose != 0 {
                file_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"expected `)'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    &mut tok,
                );
                return 1 as libc::c_int;
            }
        }
    }
    if wrapper != 0 {
        nexttoken();
    }
    while tok.type_0 == PARM_WRAPPER as libc::c_int {
        if skip_balanced('(' as i32, ')' as i32, 0 as libc::c_int) == -(1 as libc::c_int)
        {
            file_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"unexpected end of file in function declaration\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *mut TOKSTK,
            );
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn parmdcl(mut idptr: *mut Ident) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    while nexttoken() != 0 as libc::c_int && tok.type_0 != '(' as i32 {
        if tok.type_0 == MODIFIER as libc::c_int {
            if !idptr.is_null() && (*idptr).type_end == -(1 as libc::c_int) {
                (*idptr).type_end = curs - 1 as libc::c_int;
            }
        } else if tok.type_0 == IDENTIFIER as libc::c_int {
            while tok.type_0 == IDENTIFIER as libc::c_int {
                nexttoken();
            }
            type_0 = tok.type_0;
            putback();
            if type_0 != MODIFIER as libc::c_int {
                break;
            }
        } else if tok.type_0 == ')' as i32 || tok.type_0 == ',' as i32 {
            return 0 as libc::c_int
        }
    }
    if !idptr.is_null() && (*idptr).type_end == -(1 as libc::c_int) {
        (*idptr).type_end = curs - 1 as libc::c_int;
    }
    return dirdcl(idptr);
}
pub unsafe extern "C" fn maybe_parm_list(mut parm_cnt_return: *mut libc::c_int) {
    let mut parmcnt: libc::c_int = 0 as libc::c_int;
    let mut ident: Ident = Ident {
        name: 0 as *mut libc::c_char,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: ExternStorage,
    };
    let mut level_0: libc::c_int = 0;
    parm_level += 1;
    parm_level;
    while nexttoken() != 0 {
        match tok.type_0 {
            41 => {
                if !parm_cnt_return.is_null() {
                    *parm_cnt_return = parmcnt;
                }
                parm_level -= 1;
                parm_level;
                return;
            }
            44 => {}
            273 | 260 | 265 | 264 | 267 | 270 => {
                parmcnt += 1;
                parmcnt;
                ident.storage = AutoStorage;
                parse_declaration(&mut ident, 1 as libc::c_int);
                putback();
            }
            _ => {
                if verbose != 0 {
                    file_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected token in parameter list\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        &mut tok,
                    );
                }
                level_0 = 0 as libc::c_int;
                loop {
                    if tok.type_0 == '(' as i32 {
                        level_0 += 1;
                        level_0;
                    } else if tok.type_0 == ')' as i32 {
                        let fresh15 = level_0;
                        level_0 = level_0 - 1;
                        if fresh15 == 0 as libc::c_int {
                            break;
                        }
                    }
                    if !(nexttoken() != 0) {
                        break;
                    }
                }
                putback();
            }
        }
    }
    if verbose != 0 {
        file_error(
            dcgettext(
                0 as *const libc::c_char,
                b"unexpected end of file in parameter list\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            0 as *mut TOKSTK,
        );
    }
}
pub unsafe extern "C" fn func_body() {
    let mut ident: Ident = Ident {
        name: 0 as *mut libc::c_char,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: ExternStorage,
    };
    level += 1;
    level;
    move_parms(level);
    while level != 0 {
        cleanup_stack();
        nexttoken();
        match tok.type_0 {
            262 => {
                ident.storage = StaticStorage;
                nexttoken();
                parse_variable_declaration(&mut ident, 0 as libc::c_int);
                continue;
            }
            270 | 264 => {
                ident.storage = AutoStorage;
                parse_variable_declaration(&mut ident, 0 as libc::c_int);
                continue;
            }
            261 => {
                ident.storage = ExplicitExternStorage;
                parse_declaration(&mut ident, 0 as libc::c_int);
                continue;
            }
            258 | 123 => {
                level += 1;
                level;
                continue;
            }
            259 => {
                if use_indentation != 0 {
                    if verbose != 0 && level != 1 as libc::c_int {
                        file_error(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"forced function body close\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            0 as *mut TOKSTK,
                        );
                    }
                    while level != 0 {
                        delete_autos(level);
                        level -= 1;
                        level;
                    }
                    continue;
                }
            }
            125 => {}
            0 => {
                if verbose != 0 {
                    file_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"unexpected end of file in function body\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        0 as *mut TOKSTK,
                    );
                }
                break;
            }
            _ => {
                expression();
                continue;
            }
        }
        delete_autos(level);
        level -= 1;
        level;
    }
    caller = 0 as *mut Symbol;
}
pub unsafe extern "C" fn get_knr_args(mut ident: *mut Ident) -> libc::c_int {
    let mut parmcnt: libc::c_int = 0;
    let mut stop: libc::c_int = 0;
    let mut sp: Stackpos = [0; 1];
    let mut new_sp: Stackpos = [0; 1];
    let mut id: Ident = Ident {
        name: 0 as *mut libc::c_char,
        type_end: 0,
        parmcnt: 0,
        line: 0,
        storage: ExternStorage,
    };
    match tok.type_0 {
        273 | 260 | 270 | 264 => {
            mark(sp.as_mut_ptr());
            parmcnt = 0 as libc::c_int;
            stop = 0 as libc::c_int;
            while stop == 0 && parmcnt < (*ident).parmcnt {
                id.type_end = -(1 as libc::c_int);
                let mut current_block_19: u64;
                match tok.type_0 {
                    123 | 258 => {
                        putback();
                        stop = 1 as libc::c_int;
                        current_block_19 = 10652014663920648156;
                    }
                    270 | 273 | 260 | 264 => {
                        putback();
                        mark(new_sp.as_mut_ptr());
                        if dcl(&mut id) == 0 as libc::c_int {
                            parmcnt += 1;
                            parmcnt;
                            if tok.type_0 == ',' as i32 {
                                loop {
                                    tos = id.type_end;
                                    restore(new_sp.as_mut_ptr());
                                    dcl(&mut id);
                                    if !(tok.type_0 == ',' as i32) {
                                        break;
                                    }
                                }
                            } else if tok.type_0 != ';' as i32 {
                                putback();
                            }
                            current_block_19 = 10652014663920648156;
                        } else {
                            current_block_19 = 9633169930869042418;
                        }
                    }
                    _ => {
                        current_block_19 = 9633169930869042418;
                    }
                }
                match current_block_19 {
                    10652014663920648156 => {}
                    _ => {
                        restore(sp.as_mut_ptr());
                        return 1 as libc::c_int;
                    }
                }
                nexttoken();
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn declare(mut ident: *mut Ident, mut maybe_knr: libc::c_int) {
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    if (*ident).storage as libc::c_uint == AutoStorage as libc::c_int as libc::c_uint {
        undo_save_stack();
        sp = install_ident((*ident).name, (*ident).storage);
        if parm_level != 0 {
            (*sp).level = parm_level;
            (*sp).flag = symbol_parm;
        } else {
            (*sp).level = level;
        }
        (*sp).arity = -(1 as libc::c_int);
        return;
    }
    if (*ident).parmcnt >= 0 as libc::c_int
        && (maybe_knr == 0 || get_knr_args(ident) == 0 as libc::c_int)
        && !(tok.type_0 == LBRACE as libc::c_int || tok.type_0 == LBRACE0 as libc::c_int
            || tok.type_0 == TYPE as libc::c_int
            || tok.type_0 == PARM_WRAPPER as libc::c_int)
        || (*ident).parmcnt < 0 as libc::c_int
            && (*ident).storage as libc::c_uint
                == ExplicitExternStorage as libc::c_int as libc::c_uint
        || save_stack_is_empty() != 0
    {
        undo_save_stack();
        return;
    }
    sp = get_symbol((*ident).name);
    if !((*sp).source).is_null() {
        if (*ident).storage as libc::c_uint
            == StaticStorage as libc::c_int as libc::c_uint
            && ((*sp).storage as libc::c_uint
                != StaticStorage as libc::c_int as libc::c_uint
                || level > 0 as libc::c_int)
        {
            sp = install_ident((*ident).name, (*ident).storage);
        } else {
            if (*sp).arity >= 0 as libc::c_int {
                error_at_line(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    filename,
                    (*ident).line as libc::c_uint,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s/%d redefined\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*ident).name,
                    (*sp).arity,
                );
            } else {
                error_at_line(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    filename,
                    (*ident).line as libc::c_uint,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s redefined\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*ident).name,
                );
            }
            error_at_line(
                0 as libc::c_int,
                0 as libc::c_int,
                (*sp).source,
                (*sp).def_line as libc::c_uint,
                dcgettext(
                    0 as *const libc::c_char,
                    b"this is the place of previous definition\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    (*sp).type_0 = SymIdentifier;
    (*sp).arity = (*ident).parmcnt;
    ident_change_storage(
        sp,
        (if (*ident).storage as libc::c_uint
            == ExplicitExternStorage as libc::c_int as libc::c_uint
        {
            ExternStorage as libc::c_int as libc::c_uint
        } else {
            (*ident).storage as libc::c_uint
        }) as storage,
    );
    (*sp).decl = finish_save_stack((*ident).name);
    (*sp).source = filename;
    (*sp).def_line = (*ident).line;
    (*sp).level = level;
    if debug != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s/%d defined to %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            line_num,
            (*ident).name,
            (*ident).parmcnt,
            (*sp).decl,
        );
    }
}
pub unsafe extern "C" fn declare_type(mut ident: *mut Ident) {
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    undo_save_stack();
    sp = lookup((*ident).name);
    while !sp.is_null() {
        if (*sp).type_0 as libc::c_uint == SymToken as libc::c_int as libc::c_uint
            && (*sp).token_type == TYPE as libc::c_int
        {
            break;
        }
        sp = (*sp).next;
    }
    if sp.is_null() {
        sp = install((*ident).name, 0x4 as libc::c_int);
    }
    (*sp).type_0 = SymToken;
    (*sp).token_type = TYPE as libc::c_int;
    (*sp).source = filename;
    (*sp).def_line = (*ident).line;
    (*sp).ref_line = 0 as *mut linked_list;
    if debug != 0 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: type %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            line_num,
            (*ident).name,
        );
    }
}
pub unsafe extern "C" fn get_symbol(mut name: *mut libc::c_char) -> *mut Symbol {
    let mut sp: *mut Symbol = lookup(name);
    if !sp.is_null() {
        while !sp.is_null() {
            if (*sp).flag as libc::c_uint == symbol_start as libc::c_int as libc::c_uint
                || (*sp).flag as libc::c_uint
                    == symbol_target as libc::c_int as libc::c_uint
                || (*sp).type_0 as libc::c_uint
                    == SymIdentifier as libc::c_int as libc::c_uint
            {
                break;
            }
            sp = (*sp).next;
        }
        if !sp.is_null() {
            if ((*sp).flag as libc::c_uint == symbol_start as libc::c_int as libc::c_uint
                || (*sp).flag as libc::c_uint
                    == symbol_target as libc::c_int as libc::c_uint)
                && (*sp).type_0 as libc::c_uint
                    == SymUndefined as libc::c_int as libc::c_uint
            {
                init_ident(sp, ExternStorage);
            }
            return sp;
        }
    }
    return install_ident(name, ExternStorage);
}
pub unsafe extern "C" fn add_reference(
    mut name: *mut libc::c_char,
    mut line: libc::c_int,
) -> *mut Symbol {
    let mut sp: *mut Symbol = get_symbol(name);
    let mut refptr: *mut Ref = 0 as *mut Ref;
    if (*sp).storage as libc::c_uint == AutoStorage as libc::c_int as libc::c_uint
        || (*sp).storage as libc::c_uint == StaticStorage as libc::c_int as libc::c_uint
            && globals_only() != 0
    {
        return 0 as *mut Symbol;
    }
    refptr = xmalloc(::std::mem::size_of::<Ref>() as libc::c_ulong) as *mut Ref;
    (*refptr).source = filename;
    (*refptr).line = line;
    if ((*sp).ref_line).is_null() {
        (*sp)
            .ref_line = linked_list_create(
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    linked_list_append(&mut (*sp).ref_line, refptr as *mut libc::c_void);
    return sp;
}
pub unsafe extern "C" fn call(mut name: *mut libc::c_char, mut line: libc::c_int) {
    let mut sp: *mut Symbol = 0 as *mut Symbol;
    sp = add_reference(name, line);
    if sp.is_null() {
        return;
    }
    if (*sp).arity < 0 as libc::c_int {
        (*sp).arity = 0 as libc::c_int;
    }
    if !caller.is_null() {
        if data_in_list(caller as *mut libc::c_void, (*sp).caller) == 0 {
            linked_list_append(&mut (*sp).caller, caller as *mut libc::c_void);
        }
        if data_in_list(sp as *mut libc::c_void, (*caller).callee) == 0 {
            linked_list_append(&mut (*caller).callee, sp as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn reference(mut name: *mut libc::c_char, mut line: libc::c_int) {
    let mut sp: *mut Symbol = add_reference(name, line);
    if sp.is_null() {
        return;
    }
    if !caller.is_null() {
        if data_in_list(caller as *mut libc::c_void, (*sp).caller) == 0 {
            linked_list_append(&mut (*sp).caller, caller as *mut libc::c_void);
        }
        if data_in_list(sp as *mut libc::c_void, (*caller).callee) == 0 {
            linked_list_append(&mut (*caller).callee, sp as *mut libc::c_void);
        }
    }
}
pub unsafe extern "C" fn reset_static_caller() {
    if !caller.is_null()
        && ((*caller).storage as libc::c_uint
            == StaticStorage as libc::c_int as libc::c_uint
            || (*caller).flag as libc::c_uint
                == symbol_local as libc::c_int as libc::c_uint)
    {
        caller = 0 as *mut Symbol;
    }
}
