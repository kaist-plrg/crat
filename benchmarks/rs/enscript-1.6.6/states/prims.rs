use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stringhash_st;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn execute_state(name: *mut libc::c_char) -> *mut Node;
    fn node_copy(node: *mut Node) -> *mut Node;
    fn lookup_state(name: *mut libc::c_char) -> *mut State;
    fn compile_regexp(regexp: *mut Node);
    fn node_free(node: *mut Node);
    fn eval_expr(expr: *mut Expr, env: *mut Environment) -> *mut Node;
    fn node_alloc(type_0: NodeType) -> *mut Node;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn re_set_character_syntax(ch: libc::c_uchar, syntax: libc::c_char);
    fn re_search(
        buffer: *mut re_pattern_buffer,
        string: *const libc::c_char,
        length: libc::c_int,
        start: libc::c_int,
        range: libc::c_int,
        regs: *mut re_registers,
    ) -> libc::c_int;
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
    static mut program: *mut libc::c_char;
    static mut ofp: *mut FILE;
    static mut ns_prims: StringHashPtr;
    static mut startrules: *mut List;
    static mut namerules: *mut List;
    static mut nvoid: *mut Node;
    static mut inbuf: *mut libc::c_char;
    static mut data_in_buffer: libc::c_uint;
    static mut current_fname: *mut libc::c_char;
    static mut current_match: *mut re_registers;
    static mut current_match_buf: *mut libc::c_char;
    static mut start_state: *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub str_0: C2RustUnnamed_2,
    pub re: C2RustUnnamed_1,
    pub integer: libc::c_int,
    pub real: libc::c_double,
    pub sym: *mut libc::c_char,
    pub array: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub array: *mut *mut node_st,
    pub len: libc::c_uint,
    pub allocated: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub data: *mut libc::c_char,
    pub len: libc::c_uint,
    pub flags: libc::c_uint,
    pub compiled: regex_t,
    pub matches: re_registers,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub node: *mut Node,
    pub not: *mut expr_st,
    pub fcall: C2RustUnnamed_9,
    pub assign: C2RustUnnamed_8,
    pub arrayassign: C2RustUnnamed_7,
    pub arrayref: C2RustUnnamed_6,
    pub questcolon: C2RustUnnamed_5,
    pub op: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub left: *mut expr_st,
    pub right: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub cond: *mut expr_st,
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub expr1: *mut expr_st,
    pub expr2: *mut expr_st,
    pub expr3: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub sym: *mut Node,
    pub expr: *mut expr_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub name: *mut Node,
    pub args: *mut List,
}
pub type Expr = expr_st;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub name: *mut libc::c_char,
    pub prim: Primitive,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
unsafe extern "C" fn match_arg(
    mut prim_name: *mut libc::c_char,
    mut type_0: NodeType,
    mut argp: *mut *mut ListItem,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = *argp;
    let mut n: *mut Node = 0 as *mut Node;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    n = eval_expr((*arg).data as *mut Expr, env);
    if type_0 as libc::c_uint != nVOID as libc::c_int as libc::c_uint
        && (*n).type_0 as libc::c_uint != type_0 as libc::c_uint
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: illegal argument type\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    *argp = (*arg).next;
    return n;
}
unsafe extern "C" fn prim_call(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut e: *mut Expr = 0 as *mut Expr;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    e = (*arg).data as *mut Expr;
    if (*e).type_0 as libc::c_uint != eSYMBOL as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: illegal argument type\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    cp = (*(*e).u.node).u.sym;
    arg = (*arg).next;
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    return execute_state(cp);
}
unsafe extern "C" fn prim_calln(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    n = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    cp = xmalloc(
        ((*n).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut libc::c_char;
    memcpy(
        cp as *mut libc::c_void,
        (*n).u.str_0.data as *const libc::c_void,
        (*n).u.str_0.len as libc::c_ulong,
    );
    *cp.offset((*n).u.str_0.len as isize) = '\0' as i32 as libc::c_char;
    node_free(n);
    n = execute_state(cp);
    xfree(cp as *mut libc::c_void);
    return n;
}
unsafe extern "C" fn prim_check_namerules(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut i: *mut ListItem = 0 as *mut ListItem;
    let mut c: *mut Cons = 0 as *mut Cons;
    let mut n: *mut Node = 0 as *mut Node;
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    if start_state.is_null() {
        i = (*namerules).head;
        while !i.is_null() {
            c = (*i).data as *mut Cons;
            n = (*c).car as *mut Node;
            if re_search(
                (if ((*n).u.re.compiled).fastmap_accurate() as libc::c_int != 0 {
                    &mut (*n).u.re.compiled
                } else {
                    compile_regexp(n);
                    &mut (*n).u.re.compiled
                }),
                current_fname,
                strlen(current_fname) as libc::c_int,
                0 as libc::c_int,
                strlen(current_fname) as libc::c_int,
                0 as *mut re_registers,
            ) >= 0 as libc::c_int
            {
                n = (*c).cdr as *mut Node;
                start_state = (*n).u.sym;
                n = node_alloc(nINTEGER);
                (*n).u.integer = 1 as libc::c_int;
                return n;
            }
            i = (*i).next;
        }
    }
    n = node_alloc(nINTEGER);
    (*n).u.integer = 0 as libc::c_int;
    return n;
}
unsafe extern "C" fn prim_check_startrules(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut i: *mut ListItem = 0 as *mut ListItem;
    let mut c: *mut Cons = 0 as *mut Cons;
    let mut n: *mut Node = 0 as *mut Node;
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    if start_state.is_null() {
        i = (*startrules).head;
        while !i.is_null() {
            c = (*i).data as *mut Cons;
            n = (*c).car as *mut Node;
            if re_search(
                (if ((*n).u.re.compiled).fastmap_accurate() as libc::c_int != 0 {
                    &mut (*n).u.re.compiled
                } else {
                    compile_regexp(n);
                    &mut (*n).u.re.compiled
                }),
                inbuf,
                data_in_buffer as libc::c_int,
                0 as libc::c_int,
                data_in_buffer as libc::c_int,
                0 as *mut re_registers,
            ) >= 0 as libc::c_int
            {
                n = (*c).cdr as *mut Node;
                start_state = (*n).u.sym;
                n = node_alloc(nINTEGER);
                (*n).u.integer = 1 as libc::c_int;
                return n;
            }
            i = (*i).next;
        }
    }
    n = node_alloc(nINTEGER);
    (*n).u.integer = 0 as libc::c_int;
    return n;
}
unsafe extern "C" fn prim_concat(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    while !arg.is_null() {
        n = eval_expr((*arg).data as *mut Expr, env);
        if (*n).type_0 as libc::c_uint != nSTRING as libc::c_int as libc::c_uint {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s:%d: %s: illegal argument type\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
                linenum,
                prim_name,
            );
            exit(1 as libc::c_int);
        }
        if (*n).u.str_0.len > 0 as libc::c_int as libc::c_uint {
            data = xrealloc(
                data as *mut libc::c_void,
                (len as libc::c_uint).wrapping_add((*n).u.str_0.len) as size_t,
            ) as *mut libc::c_char;
            memcpy(
                data.offset(len as isize) as *mut libc::c_void,
                (*n).u.str_0.data as *const libc::c_void,
                (*n).u.str_0.len as libc::c_ulong,
            );
            len = (len as libc::c_uint).wrapping_add((*n).u.str_0.len) as libc::c_int
                as libc::c_int;
        }
        node_free(n);
        arg = (*arg).next;
    }
    n = node_alloc(nSTRING);
    (*n).u.str_0.data = data;
    (*n).u.str_0.len = len as libc::c_uint;
    return n;
}
unsafe extern "C" fn prim_float(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut r: *mut Node = 0 as *mut Node;
    let mut buf: [libc::c_char; 512] = [0; 512];
    n = match_arg(
        prim_name,
        nVOID as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    r = node_alloc(nREAL);
    match (*n).type_0 as libc::c_uint {
        0 | 2 | 5 => {
            (*r).u.real = 0.0f64;
        }
        6 => {
            (*r).u.real = (*n).u.array.len as libc::c_double;
        }
        1 => {
            if (*n).u.str_0.len as libc::c_ulong
                > (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                (*r).u.real = 0.0f64;
            } else {
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    (*n).u.str_0.data as *const libc::c_void,
                    (*n).u.str_0.len as libc::c_ulong,
                );
                buf[(*n).u.str_0.len as usize] = '\0' as i32 as libc::c_char;
                (*r).u.real = atof(buf.as_mut_ptr());
            }
        }
        3 => {
            (*r).u.real = (*n).u.integer as libc::c_double;
        }
        4 => {
            (*r).u.real = (*n).u.real;
        }
        _ => {}
    }
    node_free(n);
    return r;
}
unsafe extern "C" fn prim_getenv(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut var: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    var = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    key = xcalloc(
        1 as libc::c_int as size_t,
        ((*var).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut libc::c_char;
    memcpy(
        key as *mut libc::c_void,
        (*var).u.str_0.data as *const libc::c_void,
        (*var).u.str_0.len as libc::c_ulong,
    );
    cp = getenv(key);
    node_free(var);
    xfree(key as *mut libc::c_void);
    n = node_alloc(nSTRING);
    if cp.is_null() {
        (*n).u.str_0.data = xmalloc(1 as libc::c_int as size_t) as *mut libc::c_char;
        (*n).u.str_0.len = 0 as libc::c_int as libc::c_uint;
    } else {
        (*n).u.str_0.data = xstrdup(cp);
        (*n).u.str_0.len = strlen(cp) as libc::c_uint;
    }
    return n;
}
unsafe extern "C" fn prim_int(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut r: *mut Node = 0 as *mut Node;
    let mut buf: [libc::c_char; 512] = [0; 512];
    n = match_arg(
        prim_name,
        nVOID as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    r = node_alloc(nINTEGER);
    match (*n).type_0 as libc::c_uint {
        0 | 2 | 5 => {
            (*r).u.integer = 0 as libc::c_int;
        }
        6 => {
            (*r).u.integer = (*n).u.array.len as libc::c_int;
        }
        1 => {
            if (*n).u.str_0.len as libc::c_ulong
                > (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                (*r).u.integer = 0 as libc::c_int;
            } else {
                memcpy(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    (*n).u.str_0.data as *const libc::c_void,
                    (*n).u.str_0.len as libc::c_ulong,
                );
                buf[(*n).u.str_0.len as usize] = '\0' as i32 as libc::c_char;
                (*r).u.integer = atoi(buf.as_mut_ptr());
            }
        }
        3 => {
            (*r).u.integer = (*n).u.integer;
        }
        4 => {
            (*r).u.integer = (*n).u.real as libc::c_int;
        }
        _ => {}
    }
    node_free(n);
    return r;
}
unsafe extern "C" fn prim_length(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut result: libc::c_int = 0 as libc::c_int;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    while !arg.is_null() {
        n = eval_expr((*arg).data as *mut Expr, env);
        match (*n).type_0 as libc::c_uint {
            1 => {
                result = (result as libc::c_uint).wrapping_add((*n).u.str_0.len)
                    as libc::c_int as libc::c_int;
            }
            6 => {
                result = (result as libc::c_uint).wrapping_add((*n).u.array.len)
                    as libc::c_int as libc::c_int;
            }
            _ => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s:%d: %s: illegal argument type\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename,
                    linenum,
                    prim_name,
                );
                exit(1 as libc::c_int);
            }
        }
        node_free(n);
        arg = (*arg).next;
    }
    n = node_alloc(nINTEGER);
    (*n).u.integer = result;
    return n;
}
unsafe extern "C" fn prim_list(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut len: libc::c_uint = 0;
    let mut n: *mut Node = 0 as *mut Node;
    len = 0 as libc::c_int as libc::c_uint;
    while !arg.is_null() {
        len = len.wrapping_add(1);
        len;
        arg = (*arg).next;
    }
    arg = (*args).head;
    n = node_alloc(nARRAY);
    (*n)
        .u
        .array
        .array = xcalloc(
        len.wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
        ::std::mem::size_of::<*mut Node>() as libc::c_ulong,
    ) as *mut *mut Node;
    (*n).u.array.allocated = len.wrapping_add(1 as libc::c_int as libc::c_uint);
    (*n).u.array.len = len;
    len = 0 as libc::c_int as libc::c_uint;
    while !arg.is_null() {
        let ref mut fresh0 = *((*n).u.array.array).offset(len as isize);
        *fresh0 = eval_expr((*arg).data as *mut Expr, env);
        len = len.wrapping_add(1);
        len;
        arg = (*arg).next;
    }
    return n;
}
unsafe extern "C" fn prim_panic(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: panic: \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program,
    );
    ofp = stderr;
    prim_print(prim_name, args, env, filename, linenum);
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn prim_prereq(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut s: *mut Node = 0 as *mut Node;
    let mut over: [libc::c_int; 3] = [0; 3];
    let mut rver: [libc::c_int; 3] = [0; 3];
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    s = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    sscanf(
        b"1.6.6\0" as *const u8 as *const libc::c_char,
        b"%d.%d.%d\0" as *const u8 as *const libc::c_char,
        &mut *over.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_int,
        &mut *over.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_int,
        &mut *over.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut libc::c_int,
    );
    cp = xcalloc(
        1 as libc::c_int as size_t,
        ((*s).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut libc::c_char;
    memcpy(
        cp as *mut libc::c_void,
        (*s).u.str_0.data as *const libc::c_void,
        (*s).u.str_0.len as libc::c_ulong,
    );
    if sscanf(
        cp,
        b"%d.%d.%d\0" as *const u8 as *const libc::c_char,
        &mut *rver.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_int,
        &mut *rver.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_int,
        &mut *rver.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut libc::c_int,
    ) != 3 as libc::c_int
    {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: malformed version string `%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
            cp,
        );
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if over[i as usize] > rver[i as usize] {
            break;
        }
        if over[i as usize] < rver[i as usize] {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: FATAL ERROR: States version %s or higher is required for this script\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                program,
                cp,
            );
            exit(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    xfree(cp as *mut libc::c_void);
    return nvoid;
}
unsafe extern "C" fn print_node(mut n: *mut Node) {
    let mut i: libc::c_uint = 0;
    match (*n).type_0 as libc::c_uint {
        1 => {
            fwrite(
                (*n).u.str_0.data as *const libc::c_void,
                (*n).u.str_0.len as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                ofp,
            );
        }
        2 => {
            fputc('/' as i32, ofp);
            fwrite(
                (*n).u.re.data as *const libc::c_void,
                (*n).u.re.len as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                ofp,
            );
            fputc('/' as i32, ofp);
        }
        3 => {
            fprintf(ofp, b"%d\0" as *const u8 as *const libc::c_char, (*n).u.integer);
        }
        4 => {
            fprintf(ofp, b"%f\0" as *const u8 as *const libc::c_char, (*n).u.real);
        }
        5 => {
            fprintf(ofp, b"%s\0" as *const u8 as *const libc::c_char, (*n).u.sym);
        }
        6 => {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*n).u.array.len {
                print_node(*((*n).u.array.array).offset(i as isize));
                if i.wrapping_add(1 as libc::c_int as libc::c_uint) < (*n).u.array.len {
                    fprintf(ofp, b" \0" as *const u8 as *const libc::c_char);
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn prim_print(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    while !arg.is_null() {
        n = eval_expr((*arg).data as *mut Expr, env);
        print_node(n);
        node_free(n);
        arg = (*arg).next;
    }
    return nvoid;
}
unsafe extern "C" fn prim_range(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut from: *mut Node = 0 as *mut Node;
    let mut start: *mut Node = 0 as *mut Node;
    let mut end: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    if arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too few arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    from = eval_expr((*arg).data as *mut Expr, env);
    arg = (*arg).next;
    start = match_arg(
        prim_name,
        nINTEGER as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    end = match_arg(
        prim_name,
        nINTEGER as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    if (*start).u.integer > (*end).u.integer {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: start offset is bigger than end offset\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    if (*from).type_0 as libc::c_uint == nSTRING as libc::c_int as libc::c_uint {
        if (*end).u.integer as libc::c_uint > (*from).u.str_0.len {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s:%d: %s: offset out of range\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
                linenum,
                prim_name,
            );
            exit(1 as libc::c_int);
        }
        n = node_alloc(nSTRING);
        (*n).u.str_0.len = ((*end).u.integer - (*start).u.integer) as libc::c_uint;
        (*n)
            .u
            .str_0
            .data = xmalloc(
            ((*n).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
        ) as *mut libc::c_char;
        memcpy(
            (*n).u.str_0.data as *mut libc::c_void,
            ((*from).u.str_0.data).offset((*start).u.integer as isize)
                as *const libc::c_void,
            (*n).u.str_0.len as libc::c_ulong,
        );
    } else if (*from).type_0 as libc::c_uint == nARRAY as libc::c_int as libc::c_uint {
        if (*end).u.integer as libc::c_uint > (*from).u.array.len {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s:%d: %s: offset out of range\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
                linenum,
                prim_name,
            );
            exit(1 as libc::c_int);
        }
        n = node_alloc(nARRAY);
        (*n).u.array.len = ((*end).u.integer - (*start).u.integer) as libc::c_uint;
        (*n)
            .u
            .array
            .allocated = ((*n).u.array.len)
            .wrapping_add(1 as libc::c_int as libc::c_uint);
        (*n)
            .u
            .array
            .array = xcalloc(
            (*n).u.array.allocated as size_t,
            ::std::mem::size_of::<*mut Node>() as libc::c_ulong,
        ) as *mut *mut Node;
        i = 0 as libc::c_int;
        while (i as libc::c_uint) < (*n).u.array.len {
            let ref mut fresh1 = *((*n).u.array.array).offset(i as isize);
            *fresh1 = node_copy(
                *((*from).u.array.array).offset((i + (*start).u.integer) as isize),
            );
            i += 1;
            i;
        }
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: illegal argument\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    node_free(from);
    node_free(start);
    node_free(end);
    return n;
}
unsafe extern "C" fn prim_regexp(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    str = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    n = node_alloc(nREGEXP);
    (*n)
        .u
        .re
        .data = xmalloc(
        ((*str).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut libc::c_char;
    (*n).u.re.len = (*str).u.str_0.len;
    memcpy(
        (*n).u.re.data as *mut libc::c_void,
        (*str).u.str_0.data as *const libc::c_void,
        (*str).u.str_0.len as libc::c_ulong,
    );
    *((*n).u.re.data).offset((*str).u.str_0.len as isize) = '\0' as i32 as libc::c_char;
    return n;
}
unsafe extern "C" fn prim_regexp_syntax(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut ch: *mut Node = 0 as *mut Node;
    let mut st: *mut Node = 0 as *mut Node;
    let mut syntax: libc::c_char = 0;
    ch = match_arg(
        prim_name,
        nINTEGER as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    st = match_arg(
        prim_name,
        nINTEGER as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    syntax = (*st).u.integer as libc::c_char;
    if syntax as libc::c_int != 'w' as i32 && syntax as libc::c_int != ' ' as i32 {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: illegal regexp character syntax: %c\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
            syntax as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    re_set_character_syntax((*ch).u.integer as libc::c_uchar, syntax);
    return nvoid;
}
unsafe extern "C" fn prim_regmatch(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut re: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    static mut matches: re_registers = {
        let mut init = re_registers {
            num_regs: 0 as libc::c_int as libc::c_uint,
            start: 0 as *const regoff_t as *mut regoff_t,
            end: 0 as *const regoff_t as *mut regoff_t,
        };
        init
    };
    static mut current_match_node: *mut Node = 0 as *const Node as *mut Node;
    let mut i: libc::c_int = 0;
    str = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    re = match_arg(
        prim_name,
        nREGEXP as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    i = re_search(
        if ((*re).u.re.compiled).fastmap_accurate() as libc::c_int != 0 {
            &mut (*re).u.re.compiled
        } else {
            compile_regexp(re);
            &mut (*re).u.re.compiled
        },
        (*str).u.str_0.data,
        (*str).u.str_0.len as libc::c_int,
        0 as libc::c_int,
        (*str).u.str_0.len as libc::c_int,
        &mut matches,
    );
    if i < 0 as libc::c_int {
        current_match = 0 as *mut re_registers;
        node_free(str);
    } else {
        node_free(current_match_node);
        current_match_node = str;
        current_match = &mut matches;
        current_match_buf = (*str).u.str_0.data;
    }
    node_free(re);
    n = node_alloc(nINTEGER);
    (*n).u.integer = (i >= 0 as libc::c_int) as libc::c_int;
    return n;
}
pub unsafe extern "C" fn do_regsubsts(
    mut str: *mut Node,
    mut re: *mut Node,
    mut subst: *mut Node,
    mut allp: libc::c_int,
) -> *mut Node {
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    static mut matches: re_registers = {
        let mut init = re_registers {
            num_regs: 0 as libc::c_int as libc::c_uint,
            start: 0 as *const regoff_t as *mut regoff_t,
            end: 0 as *const regoff_t as *mut regoff_t,
        };
        init
    };
    static mut result: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut result_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut result_pos: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut num_matches: libc::c_int = 0 as libc::c_int;
    let mut do_expansions_in_substs: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*subst).u.str_0.len {
        if *((*subst).u.str_0.data).offset(i as isize) as libc::c_int == '$' as i32 {
            do_expansions_in_substs = 1 as libc::c_int;
            break;
        } else {
            i += 1;
            i;
        }
    }
    pos = 0 as libc::c_int;
    loop {
        i = re_search(
            if ((*re).u.re.compiled).fastmap_accurate() as libc::c_int != 0 {
                &mut (*re).u.re.compiled
            } else {
                compile_regexp(re);
                &mut (*re).u.re.compiled
            },
            (*str).u.str_0.data,
            (*str).u.str_0.len as libc::c_int,
            pos,
            ((*str).u.str_0.len).wrapping_sub(pos as libc::c_uint) as libc::c_int,
            &mut matches,
        );
        if i < 0 as libc::c_int {
            break;
        }
        num_matches += 1;
        num_matches;
        if result_len
            < result_pos
                .wrapping_add(
                    (*(matches.start).offset(0 as libc::c_int as isize) - pos)
                        as libc::c_uint,
                )
        {
            result_len = result_len
                .wrapping_add(
                    (*(matches.start).offset(0 as libc::c_int as isize) - pos
                        + 1024 as libc::c_int) as libc::c_uint,
                );
            result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                as *mut libc::c_char;
        }
        memcpy(
            result.offset(result_pos as isize) as *mut libc::c_void,
            ((*str).u.str_0.data).offset(pos as isize) as *const libc::c_void,
            (*(matches.start).offset(0 as libc::c_int as isize) - pos) as libc::c_ulong,
        );
        result_pos = result_pos
            .wrapping_add(
                (*(matches.start).offset(0 as libc::c_int as isize) - pos)
                    as libc::c_uint,
            );
        if do_expansions_in_substs == 0 {
            if result_len < result_pos.wrapping_add((*subst).u.str_0.len) {
                result_len = result_len
                    .wrapping_add(
                        ((*subst).u.str_0.len)
                            .wrapping_add(1024 as libc::c_int as libc::c_uint),
                    );
                result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                    as *mut libc::c_char;
            }
            memcpy(
                result.offset(result_pos as isize) as *mut libc::c_void,
                (*subst).u.str_0.data as *const libc::c_void,
                (*subst).u.str_0.len as libc::c_ulong,
            );
            result_pos = result_pos.wrapping_add((*subst).u.str_0.len);
        } else {
            i = 0 as libc::c_int;
            while (i as libc::c_uint) < (*subst).u.str_0.len {
                if *((*subst).u.str_0.data).offset(i as isize) as libc::c_int
                    == '$' as i32
                    && ((i + 1 as libc::c_int) as libc::c_uint) < (*subst).u.str_0.len
                {
                    i += 1;
                    i;
                    match *((*subst).u.str_0.data).offset(i as isize) as libc::c_int {
                        36 => {
                            if result_len
                                < result_pos.wrapping_add(1 as libc::c_int as libc::c_uint)
                            {
                                result_len = result_len
                                    .wrapping_add(
                                        (1 as libc::c_int + 1024 as libc::c_int) as libc::c_uint,
                                    );
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut libc::c_char;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                b"$\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                            );
                            result_pos = result_pos
                                .wrapping_add(1 as libc::c_int as libc::c_uint);
                        }
                        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                            j = *((*subst).u.str_0.data).offset(i as isize)
                                as libc::c_int - '0' as i32;
                            if *(matches.start).offset(j as isize) >= 0 as libc::c_int {
                                if result_len
                                    < result_pos
                                        .wrapping_add(
                                            (*(matches.end).offset(j as isize)
                                                - *(matches.start).offset(j as isize)) as libc::c_uint,
                                        )
                                {
                                    result_len = result_len
                                        .wrapping_add(
                                            (*(matches.end).offset(j as isize)
                                                - *(matches.start).offset(j as isize) + 1024 as libc::c_int)
                                                as libc::c_uint,
                                        );
                                    result = xrealloc(
                                        result as *mut libc::c_void,
                                        result_len as size_t,
                                    ) as *mut libc::c_char;
                                }
                                memcpy(
                                    result.offset(result_pos as isize) as *mut libc::c_void,
                                    ((*str).u.str_0.data)
                                        .offset(*(matches.start).offset(j as isize) as isize)
                                        as *const libc::c_void,
                                    (*(matches.end).offset(j as isize)
                                        - *(matches.start).offset(j as isize)) as libc::c_ulong,
                                );
                                result_pos = result_pos
                                    .wrapping_add(
                                        (*(matches.end).offset(j as isize)
                                            - *(matches.start).offset(j as isize)) as libc::c_uint,
                                    );
                            }
                        }
                        _ => {
                            if result_len
                                < result_pos.wrapping_add(1 as libc::c_int as libc::c_uint)
                            {
                                result_len = result_len
                                    .wrapping_add(
                                        (1 as libc::c_int + 1024 as libc::c_int) as libc::c_uint,
                                    );
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut libc::c_char;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                b"$\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                            );
                            result_pos = result_pos
                                .wrapping_add(1 as libc::c_int as libc::c_uint);
                            if result_len
                                < result_pos.wrapping_add(1 as libc::c_int as libc::c_uint)
                            {
                                result_len = result_len
                                    .wrapping_add(
                                        (1 as libc::c_int + 1024 as libc::c_int) as libc::c_uint,
                                    );
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut libc::c_char;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                ((*subst).u.str_0.data).offset(i as isize)
                                    as *const libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                            );
                            result_pos = result_pos
                                .wrapping_add(1 as libc::c_int as libc::c_uint);
                        }
                    }
                } else {
                    if result_len
                        < result_pos.wrapping_add(1 as libc::c_int as libc::c_uint)
                    {
                        result_len = result_len
                            .wrapping_add(
                                (1 as libc::c_int + 1024 as libc::c_int) as libc::c_uint,
                            );
                        result = xrealloc(
                            result as *mut libc::c_void,
                            result_len as size_t,
                        ) as *mut libc::c_char;
                    }
                    memcpy(
                        result.offset(result_pos as isize) as *mut libc::c_void,
                        ((*subst).u.str_0.data).offset(i as isize)
                            as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                    );
                    result_pos = result_pos
                        .wrapping_add(1 as libc::c_int as libc::c_uint);
                }
                i += 1;
                i;
            }
        }
        pos = *(matches.end).offset(0 as libc::c_int as isize);
        if allp == 0 {
            break;
        }
    }
    if num_matches == 0 as libc::c_int {
        node_free(re);
        node_free(subst);
        return str;
    }
    if result_len
        < result_pos.wrapping_add(((*str).u.str_0.len).wrapping_sub(pos as libc::c_uint))
    {
        result_len = result_len
            .wrapping_add(
                ((*str).u.str_0.len)
                    .wrapping_sub(pos as libc::c_uint)
                    .wrapping_add(1024 as libc::c_int as libc::c_uint),
            );
        result = xrealloc(result as *mut libc::c_void, result_len as size_t)
            as *mut libc::c_char;
    }
    memcpy(
        result.offset(result_pos as isize) as *mut libc::c_void,
        ((*str).u.str_0.data).offset(pos as isize) as *const libc::c_void,
        ((*str).u.str_0.len).wrapping_sub(pos as libc::c_uint) as libc::c_ulong,
    );
    result_pos = result_pos
        .wrapping_add(((*str).u.str_0.len).wrapping_sub(pos as libc::c_uint));
    node_free(str);
    node_free(re);
    node_free(subst);
    str = node_alloc(nSTRING);
    (*str).u.str_0.len = result_pos;
    (*str).u.str_0.data = xmalloc(result_pos as size_t) as *mut libc::c_char;
    memcpy(
        (*str).u.str_0.data as *mut libc::c_void,
        result as *const libc::c_void,
        result_pos as libc::c_ulong,
    );
    return str;
}
unsafe extern "C" fn prim_regsub(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut re: *mut Node = 0 as *mut Node;
    let mut subst: *mut Node = 0 as *mut Node;
    str = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    re = match_arg(
        prim_name,
        nREGEXP as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    subst = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    return do_regsubsts(str, re, subst, 0 as libc::c_int);
}
unsafe extern "C" fn prim_regsuball(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut re: *mut Node = 0 as *mut Node;
    let mut subst: *mut Node = 0 as *mut Node;
    str = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    re = match_arg(
        prim_name,
        nREGEXP as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    subst = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    return do_regsubsts(str, re, subst, 1 as libc::c_int);
}
unsafe extern "C" fn prim_require_state(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut e: *mut Expr = 0 as *mut Expr;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut state: *mut State = 0 as *mut State;
    e = (*arg).data as *mut Expr;
    if (*e).type_0 as libc::c_uint != eSYMBOL as libc::c_int as libc::c_uint {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: illegal argument type\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    cp = (*(*e).u.node).u.sym;
    arg = (*arg).next;
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    state = lookup_state(cp);
    if state.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: couldn't define state `%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
            cp,
        );
        exit(1 as libc::c_int);
    }
    return nvoid;
}
unsafe extern "C" fn prim_split(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut re: *mut Node = 0 as *mut Node;
    let mut str: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut n2: *mut Node = 0 as *mut Node;
    let mut pos: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    re = match_arg(
        prim_name,
        nREGEXP as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    str = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    n = node_alloc(nARRAY);
    (*n).u.array.allocated = 100 as libc::c_int as libc::c_uint;
    (*n)
        .u
        .array
        .array = xcalloc(
        (*n).u.array.allocated as size_t,
        ::std::mem::size_of::<*mut Node>() as libc::c_ulong,
    ) as *mut *mut Node;
    pos = 0 as libc::c_int;
    while (pos as libc::c_uint) < (*str).u.str_0.len {
        i = re_search(
            if ((*re).u.re.compiled).fastmap_accurate() as libc::c_int != 0 {
                &mut (*re).u.re.compiled
            } else {
                compile_regexp(re);
                &mut (*re).u.re.compiled
            },
            (*str).u.str_0.data,
            (*str).u.str_0.len as libc::c_int,
            pos,
            ((*str).u.str_0.len).wrapping_sub(pos as libc::c_uint) as libc::c_int,
            &mut (*re).u.re.matches,
        );
        if i < 0 as libc::c_int {
            break;
        }
        n2 = node_alloc(nSTRING);
        (*n2).u.str_0.len = (i - pos) as libc::c_uint;
        (*n2)
            .u
            .str_0
            .data = xmalloc(
            ((*n2).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
        ) as *mut libc::c_char;
        memcpy(
            (*n2).u.str_0.data as *mut libc::c_void,
            ((*str).u.str_0.data).offset(pos as isize) as *const libc::c_void,
            (*n2).u.str_0.len as libc::c_ulong,
        );
        pos = *((*re).u.re.matches.end).offset(0 as libc::c_int as isize);
        if ((*n).u.array.len).wrapping_add(1 as libc::c_int as libc::c_uint)
            >= (*n).u.array.allocated
        {
            (*n)
                .u
                .array
                .allocated = ((*n).u.array.allocated)
                .wrapping_add(100 as libc::c_int as libc::c_uint);
            (*n)
                .u
                .array
                .array = xrealloc(
                (*n).u.array.array as *mut libc::c_void,
                ((*n).u.array.allocated as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut Node>() as libc::c_ulong),
            ) as *mut *mut Node;
        }
        let fresh2 = (*n).u.array.len;
        (*n).u.array.len = ((*n).u.array.len).wrapping_add(1);
        let ref mut fresh3 = *((*n).u.array.array).offset(fresh2 as isize);
        *fresh3 = n2;
    }
    n2 = node_alloc(nSTRING);
    (*n2).u.str_0.len = ((*str).u.str_0.len).wrapping_sub(pos as libc::c_uint);
    (*n2)
        .u
        .str_0
        .data = xmalloc(
        ((*n2).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut libc::c_char;
    memcpy(
        (*n2).u.str_0.data as *mut libc::c_void,
        ((*str).u.str_0.data).offset(pos as isize) as *const libc::c_void,
        (*n2).u.str_0.len as libc::c_ulong,
    );
    let fresh4 = (*n).u.array.len;
    (*n).u.array.len = ((*n).u.array.len).wrapping_add(1);
    let ref mut fresh5 = *((*n).u.array.array).offset(fresh4 as isize);
    *fresh5 = n2;
    return n;
}
unsafe extern "C" fn prim_sprintf(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut current_block: u64;
    let mut arg: *mut ListItem = (*args).head;
    let mut fmt: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut ifmt: [libc::c_char; 256] = [0; 256];
    let mut ifmtopts: [libc::c_char; 256] = [0; 256];
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result_pos: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut result_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut argument_count: libc::c_int = 0 as libc::c_int;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    fmt = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    cp = (*fmt).u.str_0.data;
    i = 0 as libc::c_int;
    while (i as libc::c_uint) < (*fmt).u.str_0.len {
        if *cp.offset(i as isize) as libc::c_int == '%' as i32
            && ((i + 1 as libc::c_int) as libc::c_uint >= (*fmt).u.str_0.len
                || *cp.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    == '%' as i32)
        {
            i += 1;
            i;
            if result_len < result_pos.wrapping_add(1 as libc::c_int as libc::c_uint) {
                result_len = result_len
                    .wrapping_add(
                        (1 as libc::c_int + 1024 as libc::c_int) as libc::c_uint,
                    );
                result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                    as *mut libc::c_char;
            }
            memcpy(
                result.offset(result_pos as isize) as *mut libc::c_void,
                cp.offset(i as isize) as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
            );
            result_pos = result_pos.wrapping_add(1 as libc::c_int as libc::c_uint);
        } else if *cp.offset(i as isize) as libc::c_int == '%' as i32 {
            argument_count += 1;
            argument_count;
            if arg.is_null() {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: primitive `%s': too few arguments for format\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    program,
                    prim_name,
                );
                exit(1 as libc::c_int);
            }
            n = eval_expr((*arg).data as *mut Expr, env);
            arg = (*arg).next;
            i += 1;
            i;
            j = 0 as libc::c_int;
            while (i as libc::c_uint) < (*fmt).u.str_0.len
                && ('0' as i32 <= *cp.offset(i as isize) as libc::c_int
                    && *cp.offset(i as isize) as libc::c_int <= '9' as i32
                    || *cp.offset(i as isize) as libc::c_int == '.' as i32
                    || *cp.offset(i as isize) as libc::c_int == '-' as i32)
            {
                ifmtopts[j as usize] = *cp.offset(i as isize);
                i += 1;
                i;
                j += 1;
                j;
            }
            ifmtopts[j as usize] = '\0' as i32 as libc::c_char;
            if i as libc::c_uint >= (*fmt).u.str_0.len {
                if result_len < result_pos.wrapping_add(1 as libc::c_int as libc::c_uint)
                {
                    result_len = result_len
                        .wrapping_add(
                            (1 as libc::c_int + 1024 as libc::c_int) as libc::c_uint,
                        );
                    result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                        as *mut libc::c_char;
                }
                memcpy(
                    result.offset(result_pos as isize) as *mut libc::c_void,
                    b"%\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                );
                result_pos = result_pos.wrapping_add(1 as libc::c_int as libc::c_uint);
                if result_len < result_pos.wrapping_add(j as libc::c_uint) {
                    result_len = result_len
                        .wrapping_add((j + 1024 as libc::c_int) as libc::c_uint);
                    result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                        as *mut libc::c_char;
                }
                memcpy(
                    result.offset(result_pos as isize) as *mut libc::c_void,
                    ifmtopts.as_mut_ptr() as *const libc::c_void,
                    j as libc::c_ulong,
                );
                result_pos = result_pos.wrapping_add(j as libc::c_uint);
            } else {
                match *cp.offset(i as isize) as libc::c_int {
                    120 | 88 | 100 => {
                        if (*n).type_0 as libc::c_uint
                            != nINTEGER as libc::c_int as libc::c_uint
                        {
                            current_block = 7859798996885084794;
                        } else {
                            sprintf(
                                ifmt.as_mut_ptr(),
                                b"%%%s%c\0" as *const u8 as *const libc::c_char,
                                ifmtopts.as_mut_ptr(),
                                *cp.offset(i as isize) as libc::c_int,
                            );
                            sprintf(buf.as_mut_ptr(), ifmt.as_mut_ptr(), (*n).u.integer);
                            if (result_len as libc::c_ulong)
                                < (result_pos as libc::c_ulong)
                                    .wrapping_add(strlen(buf.as_mut_ptr()))
                            {
                                result_len = (result_len as libc::c_ulong)
                                    .wrapping_add(
                                        (strlen(buf.as_mut_ptr()))
                                            .wrapping_add(1024 as libc::c_int as libc::c_ulong),
                                    ) as libc::c_uint as libc::c_uint;
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut libc::c_char;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                buf.as_mut_ptr() as *const libc::c_void,
                                strlen(buf.as_mut_ptr()),
                            );
                            result_pos = (result_pos as libc::c_ulong)
                                .wrapping_add(strlen(buf.as_mut_ptr())) as libc::c_uint
                                as libc::c_uint;
                            current_block = 15619007995458559411;
                        }
                    }
                    99 => {
                        if (*n).type_0 as libc::c_uint
                            != nINTEGER as libc::c_int as libc::c_uint
                        {
                            current_block = 7859798996885084794;
                        } else {
                            sprintf(
                                ifmt.as_mut_ptr(),
                                b"%%%s%c\0" as *const u8 as *const libc::c_char,
                                ifmtopts.as_mut_ptr(),
                                *cp.offset(i as isize) as libc::c_int,
                            );
                            sprintf(buf.as_mut_ptr(), ifmt.as_mut_ptr(), (*n).u.integer);
                            if (result_len as libc::c_ulong)
                                < (result_pos as libc::c_ulong)
                                    .wrapping_add(strlen(buf.as_mut_ptr()))
                            {
                                result_len = (result_len as libc::c_ulong)
                                    .wrapping_add(
                                        (strlen(buf.as_mut_ptr()))
                                            .wrapping_add(1024 as libc::c_int as libc::c_ulong),
                                    ) as libc::c_uint as libc::c_uint;
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut libc::c_char;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                buf.as_mut_ptr() as *const libc::c_void,
                                strlen(buf.as_mut_ptr()),
                            );
                            result_pos = (result_pos as libc::c_ulong)
                                .wrapping_add(strlen(buf.as_mut_ptr())) as libc::c_uint
                                as libc::c_uint;
                            current_block = 15619007995458559411;
                        }
                    }
                    102 | 103 | 101 | 69 => {
                        if (*n).type_0 as libc::c_uint
                            != nREAL as libc::c_int as libc::c_uint
                        {
                            current_block = 7859798996885084794;
                        } else {
                            sprintf(
                                ifmt.as_mut_ptr(),
                                b"%%%s%c\0" as *const u8 as *const libc::c_char,
                                ifmtopts.as_mut_ptr(),
                                *cp.offset(i as isize) as libc::c_int,
                            );
                            sprintf(buf.as_mut_ptr(), ifmt.as_mut_ptr(), (*n).u.real);
                            if (result_len as libc::c_ulong)
                                < (result_pos as libc::c_ulong)
                                    .wrapping_add(strlen(buf.as_mut_ptr()))
                            {
                                result_len = (result_len as libc::c_ulong)
                                    .wrapping_add(
                                        (strlen(buf.as_mut_ptr()))
                                            .wrapping_add(1024 as libc::c_int as libc::c_ulong),
                                    ) as libc::c_uint as libc::c_uint;
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut libc::c_char;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                buf.as_mut_ptr() as *const libc::c_void,
                                strlen(buf.as_mut_ptr()),
                            );
                            result_pos = (result_pos as libc::c_ulong)
                                .wrapping_add(strlen(buf.as_mut_ptr())) as libc::c_uint
                                as libc::c_uint;
                            current_block = 15619007995458559411;
                        }
                    }
                    115 => {
                        if (*n).type_0 as libc::c_uint
                            != nSTRING as libc::c_int as libc::c_uint
                        {
                            current_block = 7859798996885084794;
                        } else {
                            if ifmtopts[0 as libc::c_int as usize] as libc::c_int
                                != '\0' as i32
                            {
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"%s:%d: %s: no extra options can be specified for %%s\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    filename,
                                    linenum,
                                    prim_name,
                                );
                                exit(1 as libc::c_int);
                            }
                            if result_len < result_pos.wrapping_add((*n).u.str_0.len) {
                                result_len = result_len
                                    .wrapping_add(
                                        ((*n).u.str_0.len)
                                            .wrapping_add(1024 as libc::c_int as libc::c_uint),
                                    );
                                result = xrealloc(
                                    result as *mut libc::c_void,
                                    result_len as size_t,
                                ) as *mut libc::c_char;
                            }
                            memcpy(
                                result.offset(result_pos as isize) as *mut libc::c_void,
                                (*n).u.str_0.data as *const libc::c_void,
                                (*n).u.str_0.len as libc::c_ulong,
                            );
                            result_pos = result_pos.wrapping_add((*n).u.str_0.len);
                            current_block = 15619007995458559411;
                        }
                    }
                    _ => {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s:%d: %s: illegal type specifier `%c'\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            filename,
                            linenum,
                            prim_name,
                            *cp.offset(i as isize) as libc::c_int,
                        );
                        exit(1 as libc::c_int);
                    }
                }
                match current_block {
                    15619007995458559411 => {}
                    _ => {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s:%d: %s: argument %d doesn't match format\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            filename,
                            linenum,
                            prim_name,
                            argument_count,
                        );
                        exit(1 as libc::c_int);
                    }
                }
            }
        } else {
            if result_len < result_pos.wrapping_add(1 as libc::c_int as libc::c_uint) {
                result_len = result_len
                    .wrapping_add(
                        (1 as libc::c_int + 1024 as libc::c_int) as libc::c_uint,
                    );
                result = xrealloc(result as *mut libc::c_void, result_len as size_t)
                    as *mut libc::c_char;
            }
            memcpy(
                result.offset(result_pos as isize) as *mut libc::c_void,
                cp.offset(i as isize) as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
            );
            result_pos = result_pos.wrapping_add(1 as libc::c_int as libc::c_uint);
        }
        i += 1;
        i;
    }
    node_free(fmt);
    n = node_alloc(nSTRING);
    (*n).u.str_0.len = result_pos;
    (*n).u.str_0.data = result;
    return n;
}
unsafe extern "C" fn prim_strcmp(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut current_block: u64;
    let mut arg: *mut ListItem = (*args).head;
    let mut s1: *mut Node = 0 as *mut Node;
    let mut s2: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut cp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    s1 = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    s2 = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    cp1 = (*s1).u.str_0.data;
    cp2 = (*s2).u.str_0.data;
    i = 0 as libc::c_int;
    loop {
        if !((i as libc::c_uint) < (*s1).u.str_0.len
            && (i as libc::c_uint) < (*s2).u.str_0.len)
        {
            current_block = 9606288038608642794;
            break;
        }
        if (*cp1.offset(i as isize) as libc::c_int)
            < *cp2.offset(i as isize) as libc::c_int
        {
            result = -(1 as libc::c_int);
            current_block = 16444319343210183351;
            break;
        } else if *cp1.offset(i as isize) as libc::c_int
            > *cp2.offset(i as isize) as libc::c_int
        {
            result = 1 as libc::c_int;
            current_block = 16444319343210183351;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        9606288038608642794 => {
            if (*s1).u.str_0.len < (*s2).u.str_0.len {
                result = -(1 as libc::c_int);
            } else if (*s1).u.str_0.len > (*s2).u.str_0.len {
                result = 1 as libc::c_int;
            } else {
                result = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    node_free(s1);
    node_free(s2);
    n = node_alloc(nINTEGER);
    (*n).u.integer = result;
    return n;
}
unsafe extern "C" fn prim_string(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut n: *mut Node = 0 as *mut Node;
    let mut r: *mut Node = 0 as *mut Node;
    let mut buf: [libc::c_char; 512] = [0; 512];
    n = match_arg(
        prim_name,
        nVOID as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    r = node_alloc(nSTRING);
    match (*n).type_0 as libc::c_uint {
        0 | 2 | 6 => {
            (*r)
                .u
                .str_0
                .data = xcalloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t)
                as *mut libc::c_char;
            (*r).u.str_0.len = 0 as libc::c_int as libc::c_uint;
        }
        5 => {
            (*r).u.str_0.len = strlen((*n).u.sym) as libc::c_uint;
            (*r).u.str_0.data = xmalloc((*r).u.str_0.len as size_t) as *mut libc::c_char;
            memcpy(
                (*r).u.str_0.data as *mut libc::c_void,
                (*n).u.sym as *const libc::c_void,
                (*r).u.str_0.len as libc::c_ulong,
            );
        }
        1 => {
            (*r).u.str_0.len = (*n).u.str_0.len;
            (*r).u.str_0.data = xmalloc((*n).u.str_0.len as size_t) as *mut libc::c_char;
            memcpy(
                (*r).u.str_0.data as *mut libc::c_void,
                (*n).u.str_0.data as *const libc::c_void,
                (*n).u.str_0.len as libc::c_ulong,
            );
        }
        3 => {
            sprintf(
                buf.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                (*n).u.integer,
            );
            (*r).u.str_0.len = strlen(buf.as_mut_ptr()) as libc::c_uint;
            (*r).u.str_0.data = xmalloc((*r).u.str_0.len as size_t) as *mut libc::c_char;
            memcpy(
                (*r).u.str_0.data as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                (*r).u.str_0.len as libc::c_ulong,
            );
        }
        4 => {
            sprintf(
                buf.as_mut_ptr(),
                b"%f\0" as *const u8 as *const libc::c_char,
                (*n).u.real,
            );
            (*r).u.str_0.len = strlen(buf.as_mut_ptr()) as libc::c_uint;
            (*r).u.str_0.data = xmalloc((*r).u.str_0.len as size_t) as *mut libc::c_char;
            memcpy(
                (*r).u.str_0.data as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                (*r).u.str_0.len as libc::c_ulong,
            );
        }
        _ => {}
    }
    node_free(n);
    return r;
}
unsafe extern "C" fn prim_strncmp(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut current_block: u64;
    let mut arg: *mut ListItem = (*args).head;
    let mut s1: *mut Node = 0 as *mut Node;
    let mut s2: *mut Node = 0 as *mut Node;
    let mut len: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut cp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    s1 = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    s2 = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    len = match_arg(
        prim_name,
        nINTEGER as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    cp1 = (*s1).u.str_0.data;
    cp2 = (*s2).u.str_0.data;
    i = 0 as libc::c_int;
    loop {
        if !((i as libc::c_uint) < (*s1).u.str_0.len
            && (i as libc::c_uint) < (*s2).u.str_0.len && i < (*len).u.integer)
        {
            current_block = 1054647088692577877;
            break;
        }
        if (*cp1.offset(i as isize) as libc::c_int)
            < *cp2.offset(i as isize) as libc::c_int
        {
            result = -(1 as libc::c_int);
            current_block = 2115460689147116344;
            break;
        } else if *cp1.offset(i as isize) as libc::c_int
            > *cp2.offset(i as isize) as libc::c_int
        {
            result = 1 as libc::c_int;
            current_block = 2115460689147116344;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        1054647088692577877 => {
            if i >= (*len).u.integer {
                result = 0 as libc::c_int;
            } else if (*s1).u.str_0.len < (*s2).u.str_0.len {
                result = -(1 as libc::c_int);
            } else if (*s1).u.str_0.len > (*s2).u.str_0.len {
                result = 1 as libc::c_int;
            } else {
                result = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    node_free(s1);
    node_free(s2);
    node_free(len);
    n = node_alloc(nINTEGER);
    (*n).u.integer = result;
    return n;
}
unsafe extern "C" fn prim_substring(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut start: *mut Node = 0 as *mut Node;
    let mut end: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    str = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    start = match_arg(
        prim_name,
        nINTEGER as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    end = match_arg(
        prim_name,
        nINTEGER as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    if (*start).u.integer > (*end).u.integer {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: start offset is bigger than end offset\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    if (*end).u.integer as libc::c_uint > (*str).u.str_0.len {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: offset out of range\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    n = node_alloc(nSTRING);
    (*n).u.str_0.len = ((*end).u.integer - (*start).u.integer) as libc::c_uint;
    (*n)
        .u
        .str_0
        .data = xmalloc(
        ((*n).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut libc::c_char;
    memcpy(
        (*n).u.str_0.data as *mut libc::c_void,
        ((*str).u.str_0.data).offset((*start).u.integer as isize) as *const libc::c_void,
        (*n).u.str_0.len as libc::c_ulong,
    );
    node_free(str);
    node_free(start);
    node_free(end);
    return n;
}
unsafe extern "C" fn prim_system(
    mut prim_name: *mut libc::c_char,
    mut args: *mut List,
    mut env: *mut Environment,
    mut filename: *mut libc::c_char,
    mut linenum: libc::c_uint,
) -> *mut Node {
    let mut arg: *mut ListItem = (*args).head;
    let mut str: *mut Node = 0 as *mut Node;
    let mut n: *mut Node = 0 as *mut Node;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: libc::c_int = 0;
    str = match_arg(
        prim_name,
        nSTRING as libc::c_int as libc::c_uint,
        &mut arg,
        env,
        filename,
        linenum,
    );
    if !arg.is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%d: %s: too many arguments\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
            linenum,
            prim_name,
        );
        exit(1 as libc::c_int);
    }
    cmd = xcalloc(
        1 as libc::c_int as size_t,
        ((*str).u.str_0.len).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut libc::c_char;
    memcpy(
        cmd as *mut libc::c_void,
        (*str).u.str_0.data as *const libc::c_void,
        (*str).u.str_0.len as libc::c_ulong,
    );
    result = system(cmd);
    xfree(cmd as *mut libc::c_void);
    n = node_alloc(nINTEGER);
    (*n).u.integer = result;
    return n;
}
static mut prims: [C2RustUnnamed_10; 28] = unsafe {
    [
        {
            let mut init = C2RustUnnamed_10 {
                name: b"call\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_call),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"calln\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_calln),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"check_namerules\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_check_namerules),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"check_startrules\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_check_startrules),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"concat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_concat),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"float\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_float),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"getenv\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_getenv),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"int\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_int),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"length\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_length),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_list),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"panic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_panic),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"prereq\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_prereq),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"print\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_print),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"range\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_range),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regexp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regexp),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regexp_syntax\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regexp_syntax),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regmatch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regmatch),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regsub\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regsub),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"regsuball\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_regsuball),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"require_state\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_require_state),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"split\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_split),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"sprintf\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_sprintf),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"strcmp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_strcmp),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"string\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_string),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"strncmp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_strncmp),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"substring\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_substring),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: b"system\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                prim: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> *mut Node>,
                    Primitive,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn(
                                *mut libc::c_char,
                                *mut List,
                                *mut Environment,
                                *mut libc::c_char,
                                libc::c_uint,
                            ) -> *mut Node,
                            unsafe extern "C" fn() -> *mut Node,
                        >(prim_system),
                    ),
                ),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_10 {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                prim: None,
            };
            init
        },
    ]
};
pub unsafe extern "C" fn init_primitives() {
    let mut old: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(prims[i as usize].name).is_null() {
        if strhash_put(
            ns_prims,
            prims[i as usize].name,
            strlen(prims[i as usize].name) as libc::c_int,
            ::std::mem::transmute::<
                Primitive,
                *mut libc::c_void,
            >(prims[i as usize].prim),
            &mut old,
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
        i += 1;
        i;
    }
}
