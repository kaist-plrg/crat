use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stringhash_st;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn re_compile_pattern(
        pattern: *const libc::c_char,
        length: size_t,
        buffer: *mut re_pattern_buffer,
    ) -> *const libc::c_char;
    fn re_compile_fastmap(buffer: *mut re_pattern_buffer) -> libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn xcalloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn xstrdup(_: *mut libc::c_char) -> *mut libc::c_char;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut libc::c_char,
        keylen: libc::c_int,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_get(
        hash: StringHashPtr,
        key: *const libc::c_char,
        keylen: libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    static mut program: *mut libc::c_char;
    static mut yyin: *mut FILE;
    static mut linenum: libc::c_uint;
    static mut yyin_name: *mut libc::c_char;
    static mut warning_level: WarningLevel;
    static mut path: *mut libc::c_char;
    static mut verbose: libc::c_uint;
    static mut ns_prims: StringHashPtr;
    static mut ns_vars: StringHashPtr;
    static mut ns_subs: StringHashPtr;
    static mut ns_states: StringHashPtr;
    static mut global_stmts: *mut List;
    static mut nvoid: *mut Node;
    static mut current_linenum: libc::c_uint;
    static mut current_match: *mut re_registers;
    static mut current_match_buf: *mut libc::c_char;
    fn yyparse() -> libc::c_int;
    fn yyerror(msg: *mut libc::c_char);
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut libc::c_uchar,
    pub allocated: libc::c_ulong,
    pub used: libc::c_ulong,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_char,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: libc::c_uint,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type StringHashPtr = *mut stringhash_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_item_st {
    pub next: *mut list_item_st,
    pub data: *mut libc::c_void,
}
pub type ListItem = list_item_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_st {
    pub head: *mut ListItem,
    pub tail: *mut ListItem,
}
pub type List = list_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_st {
    pub name: *mut libc::c_char,
    pub super_name: *mut libc::c_char,
    pub super_0: *mut state_st,
    pub rules: *mut List,
}
pub type State = state_st;
pub type NodeType = libc::c_uint;
pub const nARRAY: NodeType = 6;
pub const nSYMBOL: NodeType = 5;
pub const nREAL: NodeType = 4;
pub const nINTEGER: NodeType = 3;
pub const nREGEXP: NodeType = 2;
pub const nSTRING: NodeType = 1;
pub const nVOID: NodeType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_st {
    pub type_0: NodeType,
    pub refcount: libc::c_uint,
    pub linenum: libc::c_uint,
    pub filename: *mut libc::c_char,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub str_0: C2RustUnnamed_3,
    pub re: C2RustUnnamed_2,
    pub integer: libc::c_int,
    pub real: libc::c_double,
    pub sym: *mut libc::c_char,
    pub array: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub array: *mut *mut node_st,
    pub len: libc::c_uint,
    pub allocated: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub data: *mut libc::c_char,
    pub len: libc::c_uint,
    pub flags: libc::c_uint,
    pub compiled: regex_t,
    pub matches: re_registers,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub data: *mut libc::c_char,
    pub len: libc::c_uint,
}
pub type Node = node_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cons_st {
    pub car: *mut libc::c_void,
    pub cdr: *mut libc::c_void,
}
pub type Cons = cons_st;
pub type ExprType = libc::c_uint;
pub const eLE: ExprType = 30;
pub const eGE: ExprType = 29;
pub const eNE: ExprType = 28;
pub const eEQ: ExprType = 27;
pub const eGT: ExprType = 26;
pub const eLT: ExprType = 25;
pub const eMINUS: ExprType = 24;
pub const ePLUS: ExprType = 23;
pub const eDIV: ExprType = 22;
pub const eMULT: ExprType = 21;
pub const eQUESTCOLON: ExprType = 20;
pub const eARRAYREF: ExprType = 19;
pub const eARRAYASSIGN: ExprType = 18;
pub const ePREFIXSUB: ExprType = 17;
pub const ePREFIXADD: ExprType = 16;
pub const ePOSTFIXSUB: ExprType = 15;
pub const ePOSTFIXADD: ExprType = 14;
pub const eDIVASSIGN: ExprType = 13;
pub const eMULASSIGN: ExprType = 12;
pub const eSUBASSIGN: ExprType = 11;
pub const eADDASSIGN: ExprType = 10;
pub const eASSIGN: ExprType = 9;
pub const eFCALL: ExprType = 8;
pub const eOR: ExprType = 7;
pub const eAND: ExprType = 6;
pub const eNOT: ExprType = 5;
pub const eSYMBOL: ExprType = 4;
pub const eREAL: ExprType = 3;
pub const eINTEGER: ExprType = 2;
pub const eREGEXP: ExprType = 1;
pub const eSTRING: ExprType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expr_st {
    pub type_0: ExprType,
    pub linenum: libc::c_uint,
    pub filename: *mut libc::c_char,
    pub u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub node: *mut Node,
    pub not: *mut expr_st,
    pub fcall: C2RustUnnamed_10,
    pub assign: C2RustUnnamed_9,
    pub arrayassign: C2RustUnnamed_8,
    pub arrayref: C2RustUnnamed_7,
    pub questcolon: C2RustUnnamed_6,
    pub op: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub left: *mut expr_st,
    pub right: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub cond: *mut expr_st,
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
    pub expr3: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub sym: *mut Node,
    pub expr: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub name: *mut Node,
    pub args: *mut List,
}
pub type Expr = expr_st;
pub type StmtType = libc::c_uint;
pub const sFOR: StmtType = 6;
pub const sWHILE: StmtType = 5;
pub const sEXPR: StmtType = 4;
pub const sIF: StmtType = 3;
pub const sBLOCK: StmtType = 2;
pub const sDEFSUB: StmtType = 1;
pub const sRETURN: StmtType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stmt_st {
    pub type_0: StmtType,
    pub linenum: libc::c_uint,
    pub filename: *mut libc::c_char,
    pub u: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub expr: *mut Expr,
    pub defsub: C2RustUnnamed_15,
    pub stmt_if: C2RustUnnamed_14,
    pub stmt_while: C2RustUnnamed_13,
    pub stmt_for: C2RustUnnamed_12,
    pub block: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub init: *mut Expr,
    pub cond: *mut Expr,
    pub incr: *mut Expr,
    pub body: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub expr: *mut Expr,
    pub body: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub expr: *mut Expr,
    pub then_stmt: *mut stmt_st,
    pub else_stmt: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub name: *mut Node,
    pub closure: *mut Cons,
}
pub type Stmt = stmt_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct environment_st {
    pub next: *mut environment_st,
    pub name: *mut libc::c_char,
    pub val: *mut Node,
}
pub type Environment = environment_st;
pub type Primitive = Option::<
    unsafe extern "C" fn(
        *mut libc::c_char,
        *mut List,
        *mut Environment,
        *mut libc::c_char,
        libc::c_uint,
    ) -> *mut Node,
>;
pub type WarningLevel = libc::c_uint;
pub const WARN_ALL: WarningLevel = 100;
pub const WARN_LIGHT: WarningLevel = 10;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut case_insensitive_translate: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn list() -> *mut List {
    return xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<List>() as libc::c_ulong,
    ) as *mut List;
}
pub unsafe extern "C" fn list_prepend(
    mut list_0: *mut List,
    mut data: *mut libc::c_void,
) {
    let mut item: *mut ListItem = 0 as *mut ListItem;
    item = xmalloc(::std::mem::size_of::<ListItem>() as libc::c_ulong) as *mut ListItem;
    (*item).data = data;
    (*item).next = (*list_0).head;
    (*list_0).head = item;
    if ((*list_0).tail).is_null() {
        (*list_0).tail = item;
    }
}
pub unsafe extern "C" fn list_append(
    mut list_0: *mut List,
    mut data: *mut libc::c_void,
) {
    let mut item: *mut ListItem = 0 as *mut ListItem;
    item = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<ListItem>() as libc::c_ulong,
    ) as *mut ListItem;
    (*item).data = data;
    if !((*list_0).tail).is_null() {
        (*(*list_0).tail).next = item;
    } else {
        (*list_0).head = item;
    }
    (*list_0).tail = item;
}
pub unsafe extern "C" fn node_alloc(mut type_0: NodeType) -> *mut Node {
    let mut n: *mut Node = 0 as *mut Node;
    n = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<Node>() as libc::c_ulong,
    ) as *mut Node;
    (*n).type_0 = type_0;
    (*n).refcount = 1 as libc::c_int as libc::c_uint;
    (*n).linenum = linenum;
    (*n).filename = yyin_name;
    if type_0 as libc::c_uint == nREGEXP as libc::c_int as libc::c_uint {
        (*n)
            .u
            .re
            .compiled
            .fastmap = xmalloc(256 as libc::c_int as size_t) as *mut libc::c_char;
    }
    return n;
}
pub unsafe extern "C" fn node_copy(mut n: *mut Node) -> *mut Node {
    let mut n2: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    n2 = node_alloc((*n).type_0 as libc::c_uint);
    (*n2).linenum = (*n).linenum;
    (*n2).filename = (*n).filename;
    match (*n).type_0 as libc::c_uint {
        1 => {
            (*n2).u.str_0.len = (*n).u.str_0.len;
            (*n2)
                .u
                .str_0
                .data = xmalloc(
                ((*n2).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint)
                    as size_t,
            ) as *mut libc::c_char;
            memcpy(
                (*n2).u.str_0.data as *mut libc::c_void,
                (*n).u.str_0.data as *const libc::c_void,
                (*n).u.str_0.len as libc::c_ulong,
            );
        }
        2 => {
            (*n2).u.re.data = xstrdup((*n).u.re.data);
            (*n2).u.re.len = (*n).u.re.len;
        }
        3 => {
            (*n2).u.integer = (*n).u.integer;
        }
        4 => {
            (*n2).u.real = (*n).u.real;
        }
        5 => {
            (*n2).u.sym = xstrdup((*n).u.sym);
        }
        6 => {
            (*n2).u.array.len = (*n).u.array.len;
            (*n2)
                .u
                .array
                .allocated = ((*n2).u.array.len)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
            (*n2)
                .u
                .array
                .array = xcalloc(
                (*n2).u.array.allocated as size_t,
                ::std::mem::size_of::<*mut Node>() as libc::c_ulong,
            ) as *mut *mut Node;
            i = 0 as libc::c_int;
            while (i as libc::c_uint) < (*n).u.array.len {
                let ref mut fresh0 = *((*n2).u.array.array).offset(i as isize);
                *fresh0 = node_copy(*((*n).u.array.array).offset(i as isize));
                i += 1;
                i;
            }
        }
        0 | _ => {}
    }
    return n2;
}
pub unsafe extern "C" fn node_reference(mut node: *mut Node) {
    (*node).refcount = ((*node).refcount).wrapping_add(1);
    (*node).refcount;
}
pub unsafe extern "C" fn node_free(mut node: *mut Node) {
    let mut i: libc::c_uint = 0;
    if node.is_null() {
        return;
    }
    (*node).refcount = ((*node).refcount).wrapping_sub(1);
    if (*node).refcount > 0 as libc::c_int as libc::c_uint {
        return;
    }
    match (*node).type_0 as libc::c_uint {
        0 => return,
        1 => {
            xfree((*node).u.str_0.data as *mut libc::c_void);
        }
        2 => {
            free((*node).u.re.data as *mut libc::c_void);
            xfree((*node).u.re.compiled.fastmap as *mut libc::c_void);
        }
        6 => {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*node).u.array.len {
                node_free(*((*node).u.array.array).offset(i as isize));
                i = i.wrapping_add(1);
                i;
            }
            xfree((*node).u.array.array as *mut libc::c_void);
        }
        3 | 4 | 5 | _ => {}
    }
    xfree(node as *mut libc::c_void);
}
pub unsafe extern "C" fn enter_system_variable(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    let mut n: *mut Node = 0 as *mut Node;
    let mut old_val: *mut Node = 0 as *mut Node;
    n = node_alloc(nSTRING as libc::c_int as libc::c_uint);
    (*n).u.str_0.len = strlen(value) as libc::c_uint;
    (*n).u.str_0.data = xstrdup(value);
    if strhash_put(
        ns_vars,
        name,
        strlen(name) as libc::c_int,
        n as *mut libc::c_void,
        &mut old_val as *mut *mut Node as *mut *mut libc::c_void,
    ) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: out of memory\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program,
        );
        exit(1 as libc::c_int);
    }
    node_free(old_val);
}
pub unsafe extern "C" fn compile_regexp(mut re: *mut Node) {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    if case_insensitive_translate.is_null() {
        let mut i: libc::c_int = 0;
        case_insensitive_translate = xmalloc(256 as libc::c_int as size_t)
            as *mut libc::c_char;
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            if *(*__ctype_b_loc()).offset(i as isize) as libc::c_int
                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                *case_insensitive_translate
                    .offset(
                        i as isize,
                    ) = ({
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = i;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(i);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc()).offset(i as isize);
                    }
                    __res
                }) as libc::c_char;
            } else {
                *case_insensitive_translate.offset(i as isize) = i as libc::c_char;
            }
            i += 1;
            i;
        }
    }
    if (*re).u.re.flags & 1 as libc::c_int as libc::c_uint != 0 {
        (*re).u.re.compiled.translate = case_insensitive_translate;
    }
    msg = re_compile_pattern(
        (*re).u.re.data,
        (*re).u.re.len as size_t,
        &mut (*re).u.re.compiled,
    );
    if !msg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: couldn't compile regular expression \"%s\": %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*re).filename,
            (*re).linenum,
            (*re).u.re.data,
            msg,
        );
        exit(1 as libc::c_int);
    }
    re_compile_fastmap(&mut (*re).u.re.compiled);
}
pub unsafe extern "C" fn mk_stmt(
    mut type_0: StmtType,
    mut arg1: *mut libc::c_void,
    mut arg2: *mut libc::c_void,
    mut arg3: *mut libc::c_void,
    mut arg4: *mut libc::c_void,
) -> *mut Stmt {
    let mut stmt: *mut Stmt = 0 as *mut Stmt;
    stmt = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<Stmt>() as libc::c_ulong,
    ) as *mut Stmt;
    (*stmt).type_0 = type_0;
    (*stmt).linenum = linenum;
    (*stmt).filename = yyin_name;
    match type_0 as libc::c_uint {
        4 | 0 => {
            (*stmt).u.expr = arg1 as *mut Expr;
        }
        1 => {
            (*stmt).u.defsub.name = arg1 as *mut Node;
            (*stmt).u.defsub.closure = arg2 as *mut Cons;
        }
        2 => {
            (*stmt).u.block = arg1 as *mut List;
        }
        3 => {
            (*stmt).u.stmt_if.expr = arg1 as *mut Expr;
            (*stmt).u.stmt_if.then_stmt = arg2 as *mut stmt_st;
            (*stmt).u.stmt_if.else_stmt = arg3 as *mut stmt_st;
        }
        5 => {
            (*stmt).u.stmt_while.expr = arg1 as *mut Expr;
            (*stmt).u.stmt_while.body = arg2 as *mut stmt_st;
        }
        6 => {
            (*stmt).u.stmt_for.init = arg1 as *mut Expr;
            (*stmt).u.stmt_for.cond = arg2 as *mut Expr;
            (*stmt).u.stmt_for.incr = arg3 as *mut Expr;
            (*stmt).u.stmt_for.body = arg4 as *mut stmt_st;
        }
        _ => {}
    }
    return stmt;
}
pub unsafe extern "C" fn mk_expr(
    mut type_0: ExprType,
    mut arg1: *mut libc::c_void,
    mut arg2: *mut libc::c_void,
    mut arg3: *mut libc::c_void,
) -> *mut Expr {
    let mut expr: *mut Expr = 0 as *mut Expr;
    expr = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<Expr>() as libc::c_ulong,
    ) as *mut Expr;
    (*expr).type_0 = type_0;
    (*expr).linenum = linenum;
    (*expr).filename = yyin_name;
    match type_0 as libc::c_uint {
        0 | 1 | 2 | 3 | 4 => {
            (*expr).u.node = arg1 as *mut Node;
        }
        5 => {
            (*expr).u.not = arg1 as *mut expr_st;
        }
        8 => {
            (*expr).u.fcall.name = arg1 as *mut Node;
            (*expr).u.fcall.args = arg2 as *mut List;
        }
        9 | 10 | 11 | 12 | 13 => {
            (*expr).u.assign.sym = arg1 as *mut Node;
            (*expr).u.assign.expr = arg2 as *mut expr_st;
        }
        14 | 15 | 16 | 17 => {
            (*expr).u.node = arg1 as *mut Node;
        }
        18 => {
            (*expr).u.arrayassign.expr1 = arg1 as *mut expr_st;
            (*expr).u.arrayassign.expr2 = arg2 as *mut expr_st;
            (*expr).u.arrayassign.expr3 = arg3 as *mut expr_st;
        }
        19 => {
            (*expr).u.arrayref.expr1 = arg1 as *mut expr_st;
            (*expr).u.arrayref.expr2 = arg2 as *mut expr_st;
        }
        20 => {
            (*expr).u.questcolon.cond = arg1 as *mut expr_st;
            (*expr).u.questcolon.expr1 = arg2 as *mut expr_st;
            (*expr).u.questcolon.expr2 = arg3 as *mut expr_st;
        }
        21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 6 | 7 => {
            (*expr).u.op.left = arg1 as *mut expr_st;
            (*expr).u.op.right = arg2 as *mut expr_st;
        }
        _ => {}
    }
    return expr;
}
pub unsafe extern "C" fn cons(
    mut car: *mut libc::c_void,
    mut cdr: *mut libc::c_void,
) -> *mut Cons {
    let mut c: *mut Cons = 0 as *mut Cons;
    c = xmalloc(::std::mem::size_of::<Cons>() as libc::c_ulong) as *mut Cons;
    (*c).car = car;
    (*c).cdr = cdr;
    return c;
}
pub unsafe extern "C" fn define_state(
    mut sym: *mut Node,
    mut super_0: *mut Node,
    mut rules: *mut List,
) {
    let mut old_state: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut msg: [libc::c_char; 512] = [0; 512];
    let mut state: *mut State = 0 as *mut State;
    state = xcalloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<State>() as libc::c_ulong,
    ) as *mut State;
    (*state).name = xstrdup((*sym).u.sym);
    (*state).rules = rules;
    if !super_0.is_null() {
        (*state).super_name = xstrdup((*super_0).u.sym);
    }
    if strhash_put(
        ns_states,
        (*sym).u.sym,
        strlen((*sym).u.sym) as libc::c_int,
        state as *mut libc::c_void,
        &mut old_state,
    ) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: ouf of memory\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program,
        );
        exit(1 as libc::c_int);
    }
    if !old_state.is_null() {
        sprintf(
            msg.as_mut_ptr(),
            dcgettext(
                0 as *const libc::c_char,
                b"warning: redefining state `%s'\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*sym).u.sym,
        );
        yyerror(msg.as_mut_ptr());
    }
}
unsafe extern "C" fn define_sub(
    mut sym: *mut Node,
    mut args_body: *mut Cons,
    mut filename: *mut libc::c_char,
    mut linenum_0: libc::c_uint,
) {
    let mut old_data: *mut libc::c_void = 0 as *mut libc::c_void;
    if strhash_put(
        ns_subs,
        (*sym).u.sym,
        strlen((*sym).u.sym) as libc::c_int,
        args_body as *mut libc::c_void,
        &mut old_data,
    ) == 0
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: ouf of memory\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            program,
        );
        exit(1 as libc::c_int);
    }
    if !old_data.is_null()
        && warning_level as libc::c_uint >= WARN_ALL as libc::c_int as libc::c_uint
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: warning: redefining subroutine `%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum_0,
            (*sym).u.sym,
        );
    }
}
unsafe extern "C" fn lookup_var(
    mut env: *mut Environment,
    mut ns: StringHashPtr,
    mut sym: *mut Node,
    mut filename: *mut libc::c_char,
    mut linenum_0: libc::c_uint,
) -> *mut Node {
    let mut n: *mut Node = 0 as *mut Node;
    let mut e: *mut Environment = 0 as *mut Environment;
    if *((*sym).u.sym).offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32
        && *((*sym).u.sym).offset(1 as libc::c_int as isize) as libc::c_int != 0
        && *((*sym).u.sym).offset(2 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
    {
        if *((*sym).u.sym).offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
            && *((*sym).u.sym).offset(1 as libc::c_int as isize) as libc::c_int
                <= '9' as i32
        {
            let mut i: libc::c_int = 0;
            let mut len: libc::c_int = 0;
            i = *((*sym).u.sym).offset(1 as libc::c_int as isize) as libc::c_int
                - '0' as i32;
            n = node_alloc(nSTRING as libc::c_int as libc::c_uint);
            if current_match.is_null()
                || *((*current_match).start).offset(i as isize) < 0 as libc::c_int
                || current_match_buf.is_null()
            {
                (*n)
                    .u
                    .str_0
                    .data = xmalloc(1 as libc::c_int as size_t) as *mut libc::c_char;
                (*n).u.str_0.len = 0 as libc::c_int as libc::c_uint;
            } else {
                len = *((*current_match).end).offset(i as isize)
                    - *((*current_match).start).offset(i as isize);
                (*n)
                    .u
                    .str_0
                    .data = xmalloc((len + 1 as libc::c_int) as size_t)
                    as *mut libc::c_char;
                memcpy(
                    (*n).u.str_0.data as *mut libc::c_void,
                    current_match_buf
                        .offset(*((*current_match).start).offset(i as isize) as isize)
                        as *const libc::c_void,
                    len as libc::c_ulong,
                );
                (*n).u.str_0.len = len as libc::c_uint;
            }
            (*n).refcount = 0 as libc::c_int as libc::c_uint;
            return n;
        }
        if *((*sym).u.sym).offset(1 as libc::c_int as isize) as libc::c_int == '`' as i32
            || *((*sym).u.sym).offset(1 as libc::c_int as isize) as libc::c_int
                == 'B' as i32
        {
            n = node_alloc(nSTRING as libc::c_int as libc::c_uint);
            if current_match.is_null()
                || *((*current_match).start).offset(0 as libc::c_int as isize)
                    < 0 as libc::c_int || current_match_buf.is_null()
            {
                (*n)
                    .u
                    .str_0
                    .data = xmalloc(1 as libc::c_int as size_t) as *mut libc::c_char;
                (*n).u.str_0.len = 0 as libc::c_int as libc::c_uint;
            } else {
                (*n)
                    .u
                    .str_0
                    .len = *((*current_match).start).offset(0 as libc::c_int as isize)
                    as libc::c_uint;
                (*n)
                    .u
                    .str_0
                    .data = xmalloc(
                    ((*n).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint)
                        as size_t,
                ) as *mut libc::c_char;
                memcpy(
                    (*n).u.str_0.data as *mut libc::c_void,
                    current_match_buf as *const libc::c_void,
                    (*n).u.str_0.len as libc::c_ulong,
                );
            }
            (*n).refcount = 0 as libc::c_int as libc::c_uint;
            return n;
        }
        if *((*sym).u.sym).offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
        {
            n = node_alloc(nINTEGER as libc::c_int as libc::c_uint);
            (*n).u.integer = current_linenum as libc::c_int;
            (*n).refcount = 0 as libc::c_int as libc::c_uint;
            return n;
        }
    }
    e = env;
    while !e.is_null() {
        if strcmp((*e).name, (*sym).u.sym) == 0 as libc::c_int {
            return (*e).val;
        }
        e = (*e).next;
    }
    if strhash_get(
        ns,
        (*sym).u.sym,
        strlen((*sym).u.sym) as libc::c_int,
        &mut n as *mut *mut Node as *mut *mut libc::c_void,
    ) != 0
    {
        return n;
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%s:%d: error: undefined variable `%s'\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        filename,
        linenum_0,
        (*sym).u.sym,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn set_var(
    mut env: *mut Environment,
    mut ns: StringHashPtr,
    mut sym: *mut Node,
    mut val: *mut Node,
    mut filename: *mut libc::c_char,
    mut linenum_0: libc::c_uint,
) {
    let mut n: *mut Node = 0 as *mut Node;
    let mut e: *mut Environment = 0 as *mut Environment;
    e = env;
    while !e.is_null() {
        if strcmp((*e).name, (*sym).u.sym) == 0 as libc::c_int {
            node_free((*e).val);
            (*e).val = val;
            return;
        }
        e = (*e).next;
    }
    if strhash_put(
        ns,
        (*sym).u.sym,
        strlen((*sym).u.sym) as libc::c_int,
        val as *mut libc::c_void,
        &mut n as *mut *mut Node as *mut *mut libc::c_void,
    ) != 0
    {
        node_free(n);
        return;
    }
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%s:%d: error: couldn't set variable `%s'\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        filename,
        linenum_0,
        (*sym).u.sym,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn calculate_binary(
    mut l: *mut Node,
    mut r: *mut Node,
    mut type_0: ExprType,
    mut filename: *mut libc::c_char,
    mut linenum_0: libc::c_uint,
) -> *mut Node {
    let mut n: *mut Node = 0 as *mut Node;
    match type_0 as libc::c_uint {
        21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 => {
            if (*l).type_0 as libc::c_uint == (*r).type_0 as libc::c_uint
                && (*l).type_0 as libc::c_uint == nINTEGER as libc::c_int as libc::c_uint
            {
                n = node_alloc(nINTEGER as libc::c_int as libc::c_uint);
                match type_0 as libc::c_uint {
                    21 => {
                        (*n).u.integer = (*l).u.integer * (*r).u.integer;
                    }
                    22 => {
                        (*n).u.integer = (*l).u.integer / (*r).u.integer;
                    }
                    23 => {
                        (*n).u.integer = (*l).u.integer + (*r).u.integer;
                    }
                    24 => {
                        (*n).u.integer = (*l).u.integer - (*r).u.integer;
                    }
                    25 => {
                        (*n)
                            .u
                            .integer = ((*l).u.integer < (*r).u.integer) as libc::c_int;
                    }
                    26 => {
                        (*n)
                            .u
                            .integer = ((*l).u.integer > (*r).u.integer) as libc::c_int;
                    }
                    27 => {
                        (*n)
                            .u
                            .integer = ((*l).u.integer == (*r).u.integer) as libc::c_int;
                    }
                    28 => {
                        (*n)
                            .u
                            .integer = ((*l).u.integer != (*r).u.integer) as libc::c_int;
                    }
                    29 => {
                        (*n)
                            .u
                            .integer = ((*l).u.integer >= (*r).u.integer) as libc::c_int;
                    }
                    30 => {
                        (*n)
                            .u
                            .integer = ((*l).u.integer <= (*r).u.integer) as libc::c_int;
                    }
                    _ => {}
                }
            } else if ((*l).type_0 as libc::c_uint
                == nINTEGER as libc::c_int as libc::c_uint
                || (*l).type_0 as libc::c_uint == nREAL as libc::c_int as libc::c_uint)
                && ((*r).type_0 as libc::c_uint
                    == nINTEGER as libc::c_int as libc::c_uint
                    || (*r).type_0 as libc::c_uint
                        == nREAL as libc::c_int as libc::c_uint)
            {
                let mut dl: libc::c_double = 0.;
                let mut dr: libc::c_double = 0.;
                if (*l).type_0 as libc::c_uint == nINTEGER as libc::c_int as libc::c_uint
                {
                    dl = (*l).u.integer as libc::c_double;
                } else {
                    dl = (*l).u.real;
                }
                if (*r).type_0 as libc::c_uint == nINTEGER as libc::c_int as libc::c_uint
                {
                    dr = (*r).u.integer as libc::c_double;
                } else {
                    dr = (*r).u.real;
                }
                n = node_alloc(nREAL as libc::c_int as libc::c_uint);
                match type_0 as libc::c_uint {
                    21 => {
                        (*n).u.real = dl * dr;
                    }
                    22 => {
                        (*n).u.real = dl / dr;
                    }
                    23 => {
                        (*n).u.real = dl + dr;
                    }
                    24 => {
                        (*n).u.real = dl - dr;
                    }
                    25 => {
                        (*n).type_0 = nINTEGER;
                        (*n).u.integer = (dl < dr) as libc::c_int;
                    }
                    26 => {
                        (*n).type_0 = nINTEGER;
                        (*n).u.integer = (dl > dr) as libc::c_int;
                    }
                    27 => {
                        (*n).type_0 = nINTEGER;
                        (*n).u.integer = (dl == dr) as libc::c_int;
                    }
                    28 => {
                        (*n).type_0 = nINTEGER;
                        (*n).u.integer = (dl != dr) as libc::c_int;
                    }
                    29 => {
                        (*n).type_0 = nINTEGER;
                        (*n).u.integer = (dl >= dr) as libc::c_int;
                    }
                    30 => {
                        (*n).type_0 = nINTEGER;
                        (*n).u.integer = (dl <= dr) as libc::c_int;
                    }
                    _ => {}
                }
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: error: expression between illegal types\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename,
                    linenum_0,
                );
                exit(1 as libc::c_int);
            }
        }
        _ => {
            abort();
        }
    }
    return n;
}
pub unsafe extern "C" fn eval_expr(
    mut expr: *mut Expr,
    mut env: *mut Environment,
) -> *mut Node {
    let mut n: *mut Node = nvoid;
    let mut n2: *mut Node = 0 as *mut Node;
    let mut l: *mut Node = 0 as *mut Node;
    let mut r: *mut Node = 0 as *mut Node;
    let mut c: *mut Cons = 0 as *mut Cons;
    let mut prim: Primitive = None;
    let mut return_seen: libc::c_int = 0;
    let mut ei: *mut Environment = 0 as *mut Environment;
    let mut ei2: *mut Environment = 0 as *mut Environment;
    let mut i: libc::c_int = 0;
    let mut sn: Node = Node {
        type_0: nVOID,
        refcount: 0,
        linenum: 0,
        filename: 0 as *mut libc::c_char,
        u: C2RustUnnamed_0 {
            str_0: C2RustUnnamed_3 {
                data: 0 as *mut libc::c_char,
                len: 0,
            },
        },
    };
    if expr.is_null() {
        return nvoid;
    }
    match (*expr).type_0 as libc::c_uint {
        0 | 1 | 2 | 3 => {
            node_reference((*expr).u.node);
            return (*expr).u.node;
        }
        4 => {
            n = lookup_var(
                env,
                ns_vars,
                (*expr).u.node,
                (*expr).filename,
                (*expr).linenum,
            );
            node_reference(n);
            return n;
        }
        5 => {
            n = eval_expr((*expr).u.not, env);
            i = !((*n).type_0 as libc::c_uint != nINTEGER as libc::c_int as libc::c_uint
                || (*n).u.integer != 0 as libc::c_int) as libc::c_int;
            node_free(n);
            n = node_alloc(nINTEGER as libc::c_int as libc::c_uint);
            (*n).u.integer = i;
            return n;
        }
        8 => {
            n = (*expr).u.fcall.name;
            if strhash_get(
                ns_subs,
                (*n).u.sym,
                strlen((*n).u.sym) as libc::c_int,
                &mut c as *mut *mut Cons as *mut *mut libc::c_void,
            ) != 0
            {
                let mut nenv: *mut Environment = 0 as *mut Environment;
                let mut i_0: *mut ListItem = 0 as *mut ListItem;
                let mut e: *mut ListItem = 0 as *mut ListItem;
                let mut stmts: *mut List = 0 as *mut List;
                let mut lst: *mut List = 0 as *mut List;
                let mut args_locals: *mut Cons = 0 as *mut Cons;
                args_locals = (*c).car as *mut Cons;
                stmts = (*c).cdr as *mut List;
                lst = (*args_locals).car as *mut List;
                i_0 = (*lst).head;
                e = (*(*expr).u.fcall.args).head;
                while !i_0.is_null() && !e.is_null() {
                    let mut sym: *mut Node = 0 as *mut Node;
                    sym = (*i_0).data as *mut Node;
                    n = eval_expr((*e).data as *mut Expr, env);
                    ei = xcalloc(
                        1 as libc::c_int as size_t,
                        ::std::mem::size_of::<Environment>() as libc::c_ulong,
                    ) as *mut Environment;
                    (*ei).name = (*sym).u.sym;
                    (*ei).val = n;
                    (*ei).next = nenv;
                    nenv = ei;
                    i_0 = (*i_0).next;
                    e = (*e).next;
                }
                if !i_0.is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%d: error: too few arguments for subroutine\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*expr).filename,
                        (*expr).linenum,
                    );
                    exit(1 as libc::c_int);
                }
                if !e.is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%d: error: too many arguments for subroutine\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*expr).filename,
                        (*expr).linenum,
                    );
                    exit(1 as libc::c_int);
                }
                lst = (*args_locals).cdr as *mut List;
                i_0 = (*lst).head;
                while !i_0.is_null() {
                    let mut c_0: *mut Cons = 0 as *mut Cons;
                    let mut sym_0: *mut Node = 0 as *mut Node;
                    let mut init: *mut Expr = 0 as *mut Expr;
                    c_0 = (*i_0).data as *mut Cons;
                    sym_0 = (*c_0).car as *mut Node;
                    init = (*c_0).cdr as *mut Expr;
                    ei = xcalloc(
                        1 as libc::c_int as size_t,
                        ::std::mem::size_of::<Environment>() as libc::c_ulong,
                    ) as *mut Environment;
                    (*ei).name = (*sym_0).u.sym;
                    if !init.is_null() {
                        (*ei).val = eval_expr(init, nenv);
                    } else {
                        (*ei).val = nvoid;
                    }
                    (*ei).next = nenv;
                    nenv = ei;
                    i_0 = (*i_0).next;
                }
                return_seen = 0 as libc::c_int;
                n = eval_statement_list((*c).cdr as *mut List, nenv, &mut return_seen);
                ei = nenv;
                while !ei.is_null() {
                    ei2 = (*ei).next;
                    node_free((*ei).val);
                    xfree(ei as *mut libc::c_void);
                    ei = ei2;
                }
                return n;
            } else if strhash_get(
                ns_prims,
                (*n).u.sym,
                strlen((*n).u.sym) as libc::c_int,
                &mut prim as *mut Primitive as *mut *mut libc::c_void,
            ) != 0
            {
                n = (Some(prim.unwrap()))
                    .unwrap()(
                    (*n).u.sym,
                    (*expr).u.fcall.args,
                    env,
                    (*expr).filename,
                    (*expr).linenum,
                );
                return n;
            } else {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: error: undefined procedure `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*expr).filename,
                    (*expr).linenum,
                    (*n).u.sym,
                );
                exit(1 as libc::c_int);
            }
        }
        9 => {
            n = eval_expr((*expr).u.assign.expr, env);
            set_var(
                env,
                ns_vars,
                (*expr).u.assign.sym,
                n,
                (*expr).filename,
                (*expr).linenum,
            );
            node_reference(n);
            return n;
        }
        10 | 11 | 12 | 13 => {
            n = eval_expr((*expr).u.assign.expr, env);
            n2 = lookup_var(
                env,
                ns_vars,
                (*expr).u.assign.sym,
                (*expr).filename,
                (*expr).linenum,
            );
            match (*expr).type_0 as libc::c_uint {
                10 => {
                    n2 = calculate_binary(
                        n2,
                        n,
                        ePLUS as libc::c_int as libc::c_uint,
                        (*expr).filename,
                        (*expr).linenum,
                    );
                }
                11 => {
                    n2 = calculate_binary(
                        n2,
                        n,
                        eMINUS as libc::c_int as libc::c_uint,
                        (*expr).filename,
                        (*expr).linenum,
                    );
                }
                12 => {
                    n2 = calculate_binary(
                        n2,
                        n,
                        eMULT as libc::c_int as libc::c_uint,
                        (*expr).filename,
                        (*expr).linenum,
                    );
                }
                13 => {
                    n2 = calculate_binary(
                        n2,
                        n,
                        eDIV as libc::c_int as libc::c_uint,
                        (*expr).filename,
                        (*expr).linenum,
                    );
                }
                _ => {
                    abort();
                }
            }
            set_var(
                env,
                ns_vars,
                (*expr).u.assign.sym,
                n2,
                (*expr).filename,
                (*expr).linenum,
            );
            node_free(n);
            node_reference(n2);
            return n2;
        }
        14 | 15 => {
            sn.type_0 = nINTEGER;
            sn.u.integer = 1 as libc::c_int;
            n2 = lookup_var(
                env,
                ns_vars,
                (*expr).u.node,
                (*expr).filename,
                (*expr).linenum,
            );
            node_reference(n2);
            n = calculate_binary(
                n2,
                &mut sn,
                (if (*expr).type_0 as libc::c_uint
                    == ePOSTFIXADD as libc::c_int as libc::c_uint
                {
                    ePLUS as libc::c_int
                } else {
                    eMINUS as libc::c_int
                }) as libc::c_uint,
                (*expr).filename,
                (*expr).linenum,
            );
            set_var(env, ns_vars, (*expr).u.node, n, (*expr).filename, (*expr).linenum);
            return n2;
        }
        16 | 17 => {
            sn.type_0 = nINTEGER;
            sn.u.integer = 1 as libc::c_int;
            n = lookup_var(
                env,
                ns_vars,
                (*expr).u.node,
                (*expr).filename,
                (*expr).linenum,
            );
            n = calculate_binary(
                n,
                &mut sn,
                (if (*expr).type_0 as libc::c_uint
                    == ePREFIXADD as libc::c_int as libc::c_uint
                {
                    ePLUS as libc::c_int
                } else {
                    eMINUS as libc::c_int
                }) as libc::c_uint,
                (*expr).filename,
                (*expr).linenum,
            );
            set_var(env, ns_vars, (*expr).u.node, n, (*expr).filename, (*expr).linenum);
            node_reference(n);
            return n;
        }
        18 => {
            n = eval_expr((*expr).u.arrayassign.expr1, env);
            if (*n).type_0 as libc::c_uint != nARRAY as libc::c_int as libc::c_uint
                && (*n).type_0 as libc::c_uint != nSTRING as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: error: illegal lvalue for assignment\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*expr).filename,
                    (*expr).linenum,
                );
                exit(1 as libc::c_int);
            }
            n2 = eval_expr((*expr).u.arrayassign.expr2, env);
            if (*n2).type_0 as libc::c_uint != nINTEGER as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: error: array reference index is not integer\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*expr).filename,
                    (*expr).linenum,
                );
                exit(1 as libc::c_int);
            }
            if (*n2).u.integer < 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: error: negative array reference index\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*expr).filename,
                    (*expr).linenum,
                );
                exit(1 as libc::c_int);
            }
            if (*n).type_0 as libc::c_uint == nARRAY as libc::c_int as libc::c_uint {
                if (*n2).u.integer as libc::c_uint >= (*n).u.array.len {
                    if (*n2).u.integer as libc::c_uint >= (*n).u.array.allocated {
                        (*n)
                            .u
                            .array
                            .allocated = ((*n2).u.integer + 100 as libc::c_int)
                            as libc::c_uint;
                        (*n)
                            .u
                            .array
                            .array = xrealloc(
                            (*n).u.array.array as *mut libc::c_void,
                            ((*n).u.array.allocated as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut Node>() as libc::c_ulong,
                                ),
                        ) as *mut *mut Node;
                    }
                    i = (*n).u.array.len as libc::c_int;
                    while i <= (*n2).u.integer {
                        let ref mut fresh1 = *((*n).u.array.array).offset(i as isize);
                        *fresh1 = nvoid;
                        i += 1;
                        i;
                    }
                    (*n)
                        .u
                        .array
                        .len = ((*n2).u.integer + 1 as libc::c_int) as libc::c_uint;
                }
                node_free(*((*n).u.array.array).offset((*n2).u.integer as isize));
                l = eval_expr((*expr).u.arrayassign.expr3, env);
                node_reference(l);
                let ref mut fresh2 = *((*n).u.array.array)
                    .offset((*n2).u.integer as isize);
                *fresh2 = l;
            } else {
                if (*n2).u.integer as libc::c_uint >= (*n).u.str_0.len {
                    i = (*n).u.str_0.len as libc::c_int;
                    (*n)
                        .u
                        .str_0
                        .len = ((*n2).u.integer + 1 as libc::c_int) as libc::c_uint;
                    (*n)
                        .u
                        .str_0
                        .data = xrealloc(
                        (*n).u.str_0.data as *mut libc::c_void,
                        (*n).u.str_0.len as size_t,
                    ) as *mut libc::c_char;
                    while (i as libc::c_uint) < (*n).u.str_0.len {
                        *((*n).u.str_0.data)
                            .offset(i as isize) = ' ' as i32 as libc::c_char;
                        i += 1;
                        i;
                    }
                }
                l = eval_expr((*expr).u.arrayassign.expr3, env);
                if (*l).type_0 as libc::c_uint != nINTEGER as libc::c_int as libc::c_uint
                {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%d: error: illegal rvalue for string assignment\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*expr).filename,
                        (*expr).linenum,
                    );
                    exit(1 as libc::c_int);
                }
                *((*n).u.str_0.data)
                    .offset((*n2).u.integer as isize) = (*l).u.integer as libc::c_char;
            }
            node_free(n);
            node_free(n2);
            return l;
        }
        19 => {
            n = eval_expr((*expr).u.arrayref.expr1, env);
            if (*n).type_0 as libc::c_uint != nARRAY as libc::c_int as libc::c_uint
                && (*n).type_0 as libc::c_uint != nSTRING as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: error: illegal type for array reference\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*expr).filename,
                    (*expr).linenum,
                );
                exit(1 as libc::c_int);
            }
            n2 = eval_expr((*expr).u.arrayref.expr2, env);
            if (*n2).type_0 as libc::c_uint != nINTEGER as libc::c_int as libc::c_uint {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: error: array reference index is not integer\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*expr).filename,
                    (*expr).linenum,
                );
                exit(1 as libc::c_int);
            }
            if (*n2).u.integer < 0 as libc::c_int
                || (*n).type_0 as libc::c_uint == nARRAY as libc::c_int as libc::c_uint
                    && (*n2).u.integer as libc::c_uint >= (*n).u.array.len
                || (*n).type_0 as libc::c_uint == nSTRING as libc::c_int as libc::c_uint
                    && (*n2).u.integer as libc::c_uint >= (*n).u.str_0.len
            {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: error: array reference index out of range\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*expr).filename,
                    (*expr).linenum,
                );
                exit(1 as libc::c_int);
            }
            if (*n).type_0 as libc::c_uint == nARRAY as libc::c_int as libc::c_uint {
                l = *((*n).u.array.array).offset((*n2).u.integer as isize);
                node_reference(l);
            } else {
                l = node_alloc(nINTEGER as libc::c_int as libc::c_uint);
                (*l)
                    .u
                    .integer = *((*n).u.str_0.data as *mut libc::c_uchar)
                    .offset((*n2).u.integer as isize) as libc::c_int;
            }
            node_free(n);
            node_free(n2);
            return l;
        }
        20 => {
            n = eval_expr((*expr).u.questcolon.cond, env);
            i = ((*n).type_0 as libc::c_uint != nINTEGER as libc::c_int as libc::c_uint
                || (*n).u.integer != 0 as libc::c_int) as libc::c_int;
            node_free(n);
            if i != 0 {
                n = eval_expr((*expr).u.questcolon.expr1, env);
            } else {
                n = eval_expr((*expr).u.questcolon.expr2, env);
            }
            return n;
        }
        6 => {
            n = eval_expr((*expr).u.op.left, env);
            if !((*n).type_0 as libc::c_uint != nINTEGER as libc::c_int as libc::c_uint
                || (*n).u.integer != 0 as libc::c_int)
            {
                return n;
            }
            node_free(n);
            return eval_expr((*expr).u.op.right, env);
        }
        7 => {
            n = eval_expr((*expr).u.op.left, env);
            if (*n).type_0 as libc::c_uint != nINTEGER as libc::c_int as libc::c_uint
                || (*n).u.integer != 0 as libc::c_int
            {
                return n;
            }
            node_free(n);
            return eval_expr((*expr).u.op.right, env);
        }
        21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 => {
            l = eval_expr((*expr).u.op.left, env);
            r = eval_expr((*expr).u.op.right, env);
            n = calculate_binary(
                l,
                r,
                (*expr).type_0 as libc::c_uint,
                (*expr).filename,
                (*expr).linenum,
            );
            node_free(l);
            node_free(r);
            return n;
        }
        _ => {}
    }
    return n;
}
pub unsafe extern "C" fn eval_statement(
    mut stmt: *mut Stmt,
    mut env: *mut Environment,
    mut return_seen: *mut libc::c_int,
) -> *mut Node {
    let mut n: *mut Node = nvoid;
    let mut n2: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    match (*stmt).type_0 as libc::c_uint {
        0 => {
            n = eval_expr((*stmt).u.expr, env);
            *return_seen = 1 as libc::c_int;
        }
        1 => {
            define_sub(
                (*stmt).u.defsub.name,
                (*stmt).u.defsub.closure,
                (*stmt).filename,
                (*stmt).linenum,
            );
        }
        2 => {
            n = eval_statement_list((*stmt).u.block, env, return_seen);
        }
        3 => {
            n = eval_expr((*stmt).u.stmt_if.expr, env);
            i = ((*n).type_0 as libc::c_uint != nINTEGER as libc::c_int as libc::c_uint
                || (*n).u.integer != 0 as libc::c_int) as libc::c_int;
            node_free(n);
            if i != 0 {
                n = eval_statement((*stmt).u.stmt_if.then_stmt, env, return_seen);
            } else if !((*stmt).u.stmt_if.else_stmt).is_null() {
                n = eval_statement((*stmt).u.stmt_if.else_stmt, env, return_seen);
            } else {
                n = nvoid;
            }
        }
        5 => {
            loop {
                n2 = eval_expr((*stmt).u.stmt_while.expr, env);
                i = ((*n2).type_0 as libc::c_uint
                    != nINTEGER as libc::c_int as libc::c_uint
                    || (*n2).u.integer != 0 as libc::c_int) as libc::c_int;
                node_free(n2);
                if i == 0 {
                    break;
                }
                node_free(n);
                n = eval_statement((*stmt).u.stmt_while.body, env, return_seen);
                if *return_seen != 0 {
                    break;
                }
            }
        }
        6 => {
            if !((*stmt).u.stmt_for.init).is_null() {
                n2 = eval_expr((*stmt).u.stmt_for.init, env);
                node_free(n2);
            }
            loop {
                n2 = eval_expr((*stmt).u.stmt_for.cond, env);
                i = ((*n2).type_0 as libc::c_uint
                    != nINTEGER as libc::c_int as libc::c_uint
                    || (*n2).u.integer != 0 as libc::c_int) as libc::c_int;
                node_free(n2);
                if i == 0 {
                    break;
                }
                node_free(n);
                n = eval_statement((*stmt).u.stmt_for.body, env, return_seen);
                if *return_seen != 0 {
                    break;
                }
                if !((*stmt).u.stmt_for.incr).is_null() {
                    n2 = eval_expr((*stmt).u.stmt_for.incr, env);
                    node_free(n2);
                }
            }
        }
        4 => {
            n = eval_expr((*stmt).u.expr, env);
        }
        _ => {}
    }
    return n;
}
pub unsafe extern "C" fn eval_statement_list(
    mut lst: *mut List,
    mut env: *mut Environment,
    mut return_seen: *mut libc::c_int,
) -> *mut Node {
    let mut i: *mut ListItem = 0 as *mut ListItem;
    let mut stmt: *mut Stmt = 0 as *mut Stmt;
    let mut n: *mut Node = nvoid;
    if lst.is_null() {
        return nvoid;
    }
    i = (*lst).head;
    while !i.is_null() {
        node_free(n);
        stmt = (*i).data as *mut Stmt;
        n = eval_statement(stmt, env, return_seen);
        if *return_seen != 0 {
            return n;
        }
        i = (*i).next;
    }
    return n;
}
pub unsafe extern "C" fn load_states_file(mut name: *mut libc::c_char) {
    let mut n: *mut Node = 0 as *mut Node;
    let mut return_seen: libc::c_int = 0 as libc::c_int;
    yyin_name = xstrdup(name);
    linenum = 1 as libc::c_int as libc::c_uint;
    yyin = fopen(yyin_name, b"r\0" as *const u8 as *const libc::c_char);
    if yyin.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: couldn't open definition file `%s': %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            program,
            yyin_name,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    yyparse();
    fclose(yyin);
    n = eval_statement_list(global_stmts, 0 as *mut Environment, &mut return_seen);
    node_free(n);
    global_stmts = list();
}
pub unsafe extern "C" fn autoload_file(mut name: *mut libc::c_char) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_uint = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: libc::c_uint = 1024 as libc::c_int as libc::c_uint;
    let mut name_len: libc::c_uint = 0;
    let mut stat_st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut result: libc::c_int = 0 as libc::c_int;
    name_len = strlen(name) as libc::c_uint;
    buf = xmalloc(buflen as size_t) as *mut libc::c_char;
    start = path;
    while !start.is_null() {
        cp = strchr(start, ':' as i32);
        if !cp.is_null() {
            len = cp.offset_from(start) as libc::c_long as libc::c_uint;
            cp = cp.offset(1);
            cp;
        } else {
            len = strlen(start) as libc::c_uint;
        }
        if len
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_add(name_len)
            .wrapping_add(3 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) >= buflen
        {
            buflen = len
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(name_len)
                .wrapping_add(3 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(1024 as libc::c_int as libc::c_uint);
            buf = xrealloc(buf as *mut libc::c_void, buflen as size_t)
                as *mut libc::c_char;
        }
        sprintf(
            buf,
            b"%.*s/%s.st\0" as *const u8 as *const libc::c_char,
            len,
            start,
            name,
        );
        if stat(buf, &mut stat_st) == 0 as libc::c_int {
            if verbose != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: autoloading `%s' from `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    program,
                    name,
                    buf,
                );
            }
            load_states_file(buf);
            result = 1 as libc::c_int;
            break;
        } else {
            start = cp;
        }
    }
    xfree(buf as *mut libc::c_void);
    return result;
}
pub unsafe extern "C" fn lookup_state(mut name: *mut libc::c_char) -> *mut State {
    let mut state: *mut State = 0 as *mut State;
    let mut retry_count: libc::c_int = 0 as libc::c_int;
    loop {
        if strhash_get(
            ns_states,
            name,
            strlen(name) as libc::c_int,
            &mut state as *mut *mut State as *mut *mut libc::c_void,
        ) != 0
        {
            return state;
        }
        if retry_count > 0 as libc::c_int {
            break;
        }
        autoload_file(name);
        retry_count += 1;
        retry_count;
    }
    return 0 as *mut State;
}
