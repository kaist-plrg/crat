use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn yylex() -> libc::c_int;
    fn define_state(sym: *mut Node, super_0: *mut Node, rules: *mut List);
    fn cons(car: *mut libc::c_void, cdr: *mut libc::c_void) -> *mut Cons;
    fn mk_stmt(
        type_0: StmtType,
        arg1: *mut libc::c_void,
        arg2: *mut libc::c_void,
        arg3: *mut libc::c_void,
        arg4: *mut libc::c_void,
    ) -> *mut Stmt;
    fn mk_expr(
        type_0: ExprType,
        arg1: *mut libc::c_void,
        arg2: *mut libc::c_void,
        arg3: *mut libc::c_void,
    ) -> *mut Expr;
    fn list() -> *mut List;
    fn list_append(list_0: *mut List, data: *mut libc::c_void);
    static mut linenum: libc::c_uint;
    static mut yyin_name: *mut libc::c_char;
    static mut global_stmts: *mut List;
    static mut start_stmts: *mut List;
    static mut startrules: *mut List;
    static mut namerules: *mut List;
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
    pub u: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub expr: *mut Expr,
    pub defsub: C2RustUnnamed_14,
    pub stmt_if: C2RustUnnamed_13,
    pub stmt_while: C2RustUnnamed_12,
    pub stmt_for: C2RustUnnamed_11,
    pub block: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub init: *mut Expr,
    pub cond: *mut Expr,
    pub incr: *mut Expr,
    pub body: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub expr: *mut Expr,
    pub body: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub expr: *mut Expr,
    pub then_stmt: *mut stmt_st,
    pub else_stmt: *mut stmt_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub name: *mut Node,
    pub closure: *mut Cons,
}
pub type Stmt = stmt_st;
pub type yytype_int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub lst: *mut List,
    pub node: *mut Node,
    pub cons: *mut Cons,
    pub stmt: *mut Stmt,
    pub expr: *mut Expr,
}
pub type yytype_uint8 = libc::c_uchar;
pub type yytype_int8 = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: YYSTYPE,
}
pub unsafe extern "C" fn yyerror(mut msg: *mut libc::c_char) {
    fprintf(
        stderr,
        b"%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
        yyin_name,
        linenum,
        msg,
    );
}
static mut yytranslate: [yytype_uint8; 290] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
];
static mut yyr1: [yytype_uint8; 77] = [
    0 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
];
static mut yyr2: [yytype_uint8; 77] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
];
static mut yydefact: [yytype_uint8; 167] = [
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
];
static mut yydefgoto: [yytype_int16; 17] = [
    -(1 as libc::c_int) as yytype_int16,
    1 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
];
static mut yypact: [yytype_int16; 167] = [
    -(41 as libc::c_int) as yytype_int16,
    71 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    300 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    1 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(40 as libc::c_int) as yytype_int16,
    -(28 as libc::c_int) as yytype_int16,
    -(27 as libc::c_int) as yytype_int16,
    18 as libc::c_int as yytype_int16,
    -(24 as libc::c_int) as yytype_int16,
    -(18 as libc::c_int) as yytype_int16,
    -(17 as libc::c_int) as yytype_int16,
    296 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    296 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    391 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    296 as libc::c_int as yytype_int16,
    -(15 as libc::c_int) as yytype_int16,
    -(19 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    411 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    126 as libc::c_int as yytype_int16,
    322 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    486 as libc::c_int as yytype_int16,
    486 as libc::c_int as yytype_int16,
    486 as libc::c_int as yytype_int16,
    486 as libc::c_int as yytype_int16,
    486 as libc::c_int as yytype_int16,
    486 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    146 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    345 as libc::c_int as yytype_int16,
    368 as libc::c_int as yytype_int16,
    486 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    470 as libc::c_int as yytype_int16,
    501 as libc::c_int as yytype_int16,
    515 as libc::c_int as yytype_int16,
    527 as libc::c_int as yytype_int16,
    527 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    451 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    296 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    21 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    94 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    290 as libc::c_int as yytype_int16,
    290 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    486 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    70 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    75 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    431 as libc::c_int as yytype_int16,
    486 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    290 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    486 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    175 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    223 as libc::c_int as yytype_int16,
    242 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    84 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    -(38 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    271 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    290 as libc::c_int as yytype_int16,
    296 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    122 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
    486 as libc::c_int as yytype_int16,
    -(41 as libc::c_int) as yytype_int16,
];
static mut yypgoto: [yytype_int8; 17] = [
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    99 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(23 as libc::c_int) as yytype_int8,
    -(32 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(12 as libc::c_int) as yytype_int8,
    0 as libc::c_int as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
    -(41 as libc::c_int) as yytype_int8,
];
static mut yytable: [yytype_uint8; 572] = [
    23 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    161 as libc::c_int as yytype_uint8,
    162 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    119 as libc::c_int as yytype_uint8,
    120 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    121 as libc::c_int as yytype_uint8,
    122 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    116 as libc::c_int as yytype_uint8,
    117 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    115 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    123 as libc::c_int as yytype_uint8,
    125 as libc::c_int as yytype_uint8,
    144 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    128 as libc::c_int as yytype_uint8,
    129 as libc::c_int as yytype_uint8,
    145 as libc::c_int as yytype_uint8,
    146 as libc::c_int as yytype_uint8,
    147 as libc::c_int as yytype_uint8,
    148 as libc::c_int as yytype_uint8,
    118 as libc::c_int as yytype_uint8,
    130 as libc::c_int as yytype_uint8,
    131 as libc::c_int as yytype_uint8,
    126 as libc::c_int as yytype_uint8,
    127 as libc::c_int as yytype_uint8,
    154 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    132 as libc::c_int as yytype_uint8,
    134 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    141 as libc::c_int as yytype_uint8,
    135 as libc::c_int as yytype_uint8,
    136 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    137 as libc::c_int as yytype_uint8,
    138 as libc::c_int as yytype_uint8,
    139 as libc::c_int as yytype_uint8,
    151 as libc::c_int as yytype_uint8,
    142 as libc::c_int as yytype_uint8,
    160 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    159 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    133 as libc::c_int as yytype_uint8,
    149 as libc::c_int as yytype_uint8,
    166 as libc::c_int as yytype_uint8,
    150 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    165 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    164 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    155 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    156 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    157 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    158 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    163 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    140 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    114 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    113 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
];
static mut yycheck: [yytype_int16; 572] = [
    1 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    136 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    162 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    160 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    159 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    50 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    50 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    50 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    50 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    50 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
];
static mut yystos: [yytype_uint8; 167] = [
    0 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut YYSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
    match yytype {
        _ => {}
    };
}
pub static mut yychar: libc::c_int = 0;
pub static mut yylval: YYSTYPE = YYSTYPE {
    lst: 0 as *const List as *mut List,
};
pub static mut yynerrs: libc::c_int = 0;
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: libc::c_int = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yytype_int16; 200] = [0; 200];
    let mut yyss: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyssp: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
        lst: 0 as *const List as *mut List,
    }; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_ulong = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0;
    let mut yyval: YYSTYPE = YYSTYPE {
        lst: 0 as *const List as *mut List,
    };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yytoken = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyvs = yyvsa.as_mut_ptr();
    yystacksize = 200 as libc::c_int as libc::c_ulong;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    yynerrs = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    yyssp = yyss;
    yyvsp = yyvs;
    '_yysetstate: loop {
        *yyssp = yystate as yytype_int16;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_ulong = (yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong;
            if 10000 as libc::c_int as libc::c_ulong <= yystacksize {
                current_block = 14943789958231152532;
                break;
            }
            yystacksize = yystacksize.wrapping_mul(2 as libc::c_int as libc::c_ulong);
            if (10000 as libc::c_int as libc::c_ulong) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_ulong;
            }
            let mut yyss1: *mut yytype_int16 = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                yystacksize
                    .wrapping_mul(
                        (::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 14943789958231152532;
                break;
            }
            let mut yynewbytes: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yytype_int16 as *mut libc::c_void,
                yyss as *const libc::c_void,
                yysize
                    .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            let mut yynewbytes_0: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                yysize.wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                .wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes_0
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 10183098462120787247;
                break;
            }
        }
        if yystate == 2 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 8687881773506231277;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(41 as libc::c_int) {
                current_block = 17163642088967549025;
            } else {
                if yychar == -(2 as libc::c_int) {
                    yychar = yylex();
                }
                if yychar <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    yychar = yytoken;
                } else {
                    yytoken = if yychar as libc::c_uint
                        <= 289 as libc::c_int as libc::c_uint
                    {
                        yytranslate[yychar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int || (571 as libc::c_int) < yyn
                    || yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 17163642088967549025;
                } else {
                    yyn = yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        yyn = -yyn;
                        current_block = 7236539850177963128;
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                            yyerrstatus;
                        }
                        yychar = -(2 as libc::c_int);
                        yystate = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yylval;
                        current_block = 2248553395737596606;
                    }
                }
            }
            match current_block {
                17163642088967549025 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = if yychar == -(2 as libc::c_int) {
                            -(2 as libc::c_int)
                        } else if yychar as libc::c_uint
                            <= 289 as libc::c_int as libc::c_uint
                        {
                            yytranslate[yychar as usize] as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if yyerrstatus == 0 {
                            yynerrs += 1;
                            yynerrs;
                            yyerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if yychar <= 0 as libc::c_int {
                                if yychar == 0 as libc::c_int {
                                    current_block = 10183098462120787247;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut yylval,
                                );
                                yychar = -(2 as libc::c_int);
                            }
                        }
                        yyerrstatus = 3 as libc::c_int;
                        loop {
                            yyn = yypact[yystate as usize] as libc::c_int;
                            if !(yyn == -(41 as libc::c_int)) {
                                yyn += 1 as libc::c_int;
                                if 0 as libc::c_int <= yyn && yyn <= 571 as libc::c_int
                                    && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                                {
                                    yyn = yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn {
                                        break;
                                    }
                                }
                            }
                            if yyssp == yyss {
                                current_block = 10183098462120787247;
                                break '_yysetstate;
                            }
                            yydestruct(
                                b"Error: popping\0" as *const u8 as *const libc::c_char,
                                yystos[yystate as usize] as libc::c_int,
                                yyvsp,
                            );
                            yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                            yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                            yystate = *yyssp as libc::c_int;
                        }
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yylval;
                        yystate = yyn;
                        current_block = 2248553395737596606;
                    } else {
                        current_block = 7236539850177963128;
                    }
                }
                _ => {}
            }
            match current_block {
                7236539850177963128 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        4 => {
                            start_stmts = (*yyvsp
                                .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                .lst;
                        }
                        5 => {
                            startrules = (*yyvsp
                                .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                .lst;
                        }
                        6 => {
                            namerules = (*yyvsp
                                .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                .lst;
                        }
                        7 => {
                            define_state(
                                (*yyvsp
                                    .offset((2 as libc::c_int - 5 as libc::c_int) as isize))
                                    .node,
                                0 as *mut Node,
                                (*yyvsp
                                    .offset((4 as libc::c_int - 5 as libc::c_int) as isize))
                                    .lst,
                            );
                        }
                        8 => {
                            define_state(
                                (*yyvsp
                                    .offset((2 as libc::c_int - 7 as libc::c_int) as isize))
                                    .node,
                                (*yyvsp
                                    .offset((4 as libc::c_int - 7 as libc::c_int) as isize))
                                    .node,
                                (*yyvsp
                                    .offset((6 as libc::c_int - 7 as libc::c_int) as isize))
                                    .lst,
                            );
                        }
                        9 => {
                            list_append(
                                global_stmts,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .stmt as *mut libc::c_void,
                            );
                        }
                        10 => {
                            yyval.lst = list();
                        }
                        11 => {
                            list_append(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 4 as libc::c_int) as isize))
                                    .lst,
                                cons(
                                    (*yyvsp
                                        .offset((2 as libc::c_int - 4 as libc::c_int) as isize))
                                        .node as *mut libc::c_void,
                                    (*yyvsp
                                        .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                        .node as *mut libc::c_void,
                                ) as *mut libc::c_void,
                            );
                        }
                        12 => {
                            yyval.lst = list();
                        }
                        13 => {
                            list_append(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 2 as libc::c_int) as isize))
                                    .lst,
                                (*yyvsp
                                    .offset((2 as libc::c_int - 2 as libc::c_int) as isize))
                                    .cons as *mut libc::c_void,
                            );
                        }
                        14 => {
                            yyval
                                .cons = cons(
                                0 as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                    .lst as *mut libc::c_void,
                            );
                        }
                        15 => {
                            yyval
                                .cons = cons(
                                1 as libc::c_int as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                    .lst as *mut libc::c_void,
                            );
                        }
                        16 => {
                            yyval
                                .cons = cons(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 4 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                    .lst as *mut libc::c_void,
                            );
                        }
                        17 => {
                            yyval
                                .cons = cons(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 4 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                    .lst as *mut libc::c_void,
                            );
                        }
                        18 => {
                            yyval.lst = list();
                        }
                        19 => {
                            yyval
                                .lst = (*yyvsp
                                .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                .lst;
                        }
                        20 => {
                            yyval.lst = list();
                            list_append(
                                yyval.lst,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                            );
                        }
                        21 => {
                            list_append(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .lst,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                            );
                        }
                        22 => {
                            yyval.lst = list();
                        }
                        23 => {
                            yyval
                                .lst = (*yyvsp
                                .offset((2 as libc::c_int - 3 as libc::c_int) as isize))
                                .lst;
                        }
                        24 => {
                            yyval.lst = list();
                            list_append(
                                yyval.lst,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .cons as *mut libc::c_void,
                            );
                        }
                        25 => {
                            list_append(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .lst,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .cons as *mut libc::c_void,
                            );
                        }
                        26 => {
                            yyval
                                .cons = cons(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        27 => {
                            yyval
                                .cons = cons(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                            );
                        }
                        28 => {
                            yyval.lst = list();
                        }
                        29 => {
                            list_append(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 2 as libc::c_int) as isize))
                                    .lst,
                                (*yyvsp
                                    .offset((2 as libc::c_int - 2 as libc::c_int) as isize))
                                    .stmt as *mut libc::c_void,
                            );
                        }
                        30 => {
                            yyval
                                .stmt = mk_stmt(
                                sRETURN,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        31 => {
                            yyval
                                .stmt = mk_stmt(
                                sRETURN,
                                (*yyvsp
                                    .offset((2 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        32 => {
                            yyval
                                .stmt = mk_stmt(
                                sDEFSUB,
                                (*yyvsp
                                    .offset((2 as libc::c_int - 9 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                cons(
                                    cons(
                                        (*yyvsp
                                            .offset((4 as libc::c_int - 9 as libc::c_int) as isize))
                                            .lst as *mut libc::c_void,
                                        (*yyvsp
                                            .offset((7 as libc::c_int - 9 as libc::c_int) as isize))
                                            .lst as *mut libc::c_void,
                                    ) as *mut libc::c_void,
                                    (*yyvsp
                                        .offset((8 as libc::c_int - 9 as libc::c_int) as isize))
                                        .lst as *mut libc::c_void,
                                ) as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        33 => {
                            yyval
                                .stmt = mk_stmt(
                                sBLOCK,
                                (*yyvsp
                                    .offset((2 as libc::c_int - 3 as libc::c_int) as isize))
                                    .lst as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        34 => {
                            yyval
                                .stmt = mk_stmt(
                                sIF,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 5 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((5 as libc::c_int - 5 as libc::c_int) as isize))
                                    .stmt as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        35 => {
                            yyval
                                .stmt = mk_stmt(
                                sIF,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 7 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((5 as libc::c_int - 7 as libc::c_int) as isize))
                                    .stmt as *mut libc::c_void,
                                (*yyvsp
                                    .offset((7 as libc::c_int - 7 as libc::c_int) as isize))
                                    .stmt as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        36 => {
                            yyval
                                .stmt = mk_stmt(
                                sWHILE,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 5 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((5 as libc::c_int - 5 as libc::c_int) as isize))
                                    .stmt as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        37 => {
                            yyval
                                .stmt = mk_stmt(
                                sFOR,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 9 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((5 as libc::c_int - 9 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((7 as libc::c_int - 9 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((9 as libc::c_int - 9 as libc::c_int) as isize))
                                    .stmt as *mut libc::c_void,
                            );
                        }
                        38 => {
                            yyval
                                .stmt = mk_stmt(
                                sEXPR,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 2 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        39 => {
                            yyval
                                .expr = mk_expr(
                                eSTRING,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        40 => {
                            yyval
                                .expr = mk_expr(
                                eREGEXP,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        41 => {
                            yyval
                                .expr = mk_expr(
                                eINTEGER,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        42 => {
                            yyval
                                .expr = mk_expr(
                                eREAL,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        43 => {
                            yyval
                                .expr = mk_expr(
                                eSYMBOL,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        44 => {
                            yyval
                                .expr = mk_expr(
                                eNOT,
                                (*yyvsp
                                    .offset((2 as libc::c_int - 2 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        45 => {
                            yyval
                                .expr = mk_expr(
                                eAND,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        46 => {
                            yyval
                                .expr = mk_expr(
                                eOR,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        47 => {
                            yyval
                                .expr = mk_expr(
                                eFCALL,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 4 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                    .lst as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        48 => {
                            yyval
                                .expr = mk_expr(
                                eASSIGN,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        49 => {
                            yyval
                                .expr = mk_expr(
                                eADDASSIGN,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        50 => {
                            yyval
                                .expr = mk_expr(
                                eSUBASSIGN,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        51 => {
                            yyval
                                .expr = mk_expr(
                                eMULASSIGN,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        52 => {
                            yyval
                                .expr = mk_expr(
                                eDIVASSIGN,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        53 => {
                            yyval
                                .expr = mk_expr(
                                ePOSTFIXADD,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 2 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        54 => {
                            yyval
                                .expr = mk_expr(
                                ePOSTFIXSUB,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 2 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        55 => {
                            yyval
                                .expr = mk_expr(
                                ePREFIXADD,
                                (*yyvsp
                                    .offset((2 as libc::c_int - 2 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        56 => {
                            yyval
                                .expr = mk_expr(
                                ePREFIXSUB,
                                (*yyvsp
                                    .offset((2 as libc::c_int - 2 as libc::c_int) as isize))
                                    .node as *mut libc::c_void,
                                0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        57 => {
                            yyval
                                .expr = mk_expr(
                                eARRAYASSIGN,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 6 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 6 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((6 as libc::c_int - 6 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                            );
                        }
                        58 => {
                            yyval
                                .expr = (*yyvsp
                                .offset((2 as libc::c_int - 3 as libc::c_int) as isize))
                                .expr;
                        }
                        59 => {
                            yyval
                                .expr = mk_expr(
                                eARRAYREF,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 4 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 4 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        60 => {
                            yyval
                                .expr = mk_expr(
                                eQUESTCOLON,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 5 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 5 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((5 as libc::c_int - 5 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                            );
                        }
                        61 => {
                            yyval
                                .expr = mk_expr(
                                eMULT,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        62 => {
                            yyval
                                .expr = mk_expr(
                                eDIV,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        63 => {
                            yyval
                                .expr = mk_expr(
                                ePLUS,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        64 => {
                            yyval
                                .expr = mk_expr(
                                eMINUS,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        65 => {
                            yyval
                                .expr = mk_expr(
                                eLT,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        66 => {
                            yyval
                                .expr = mk_expr(
                                eGT,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        67 => {
                            yyval
                                .expr = mk_expr(
                                eEQ,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        68 => {
                            yyval
                                .expr = mk_expr(
                                eNE,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        69 => {
                            yyval
                                .expr = mk_expr(
                                eGE,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        70 => {
                            yyval
                                .expr = mk_expr(
                                eLE,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            );
                        }
                        71 => {
                            yyval.expr = 0 as *mut Expr;
                        }
                        72 => {
                            yyval
                                .expr = (*yyvsp
                                .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                .expr;
                        }
                        73 => {
                            yyval.lst = list();
                        }
                        74 => {
                            yyval
                                .lst = (*yyvsp
                                .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                .lst;
                        }
                        75 => {
                            yyval.lst = list();
                            list_append(
                                yyval.lst,
                                (*yyvsp
                                    .offset((1 as libc::c_int - 1 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                            );
                        }
                        76 => {
                            list_append(
                                (*yyvsp
                                    .offset((1 as libc::c_int - 3 as libc::c_int) as isize))
                                    .lst,
                                (*yyvsp
                                    .offset((3 as libc::c_int - 3 as libc::c_int) as isize))
                                    .expr as *mut libc::c_void,
                            );
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    yyn = yyr1[yyn as usize] as libc::c_int;
                    yystate = yypgoto[(yyn - 52 as libc::c_int) as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    if 0 as libc::c_int <= yystate && yystate <= 571 as libc::c_int
                        && yycheck[yystate as usize] as libc::c_int
                            == *yyssp as libc::c_int
                    {
                        yystate = yytable[yystate as usize] as libc::c_int;
                    } else {
                        yystate = yydefgoto[(yyn - 52 as libc::c_int) as usize]
                            as libc::c_int;
                    }
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
            yyssp;
        }
    }
    match current_block {
        14943789958231152532 => {
            yyerror(
                b"memory exhausted\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            yyresult = 2 as libc::c_int;
        }
        10183098462120787247 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if yychar != -(2 as libc::c_int) {
        yytoken = if yychar as libc::c_uint <= 289 as libc::c_int as libc::c_uint {
            yytranslate[yychar as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as usize] as libc::c_int,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
