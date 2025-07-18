use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut linenum: libc::c_uint;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn xstrdup(_: *mut libc::c_char) -> *mut libc::c_char;
    fn xfree(ptr: *mut libc::c_void);
    fn xrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn yyerror(msg: *mut libc::c_char);
    fn node_alloc(type_0: NodeType) -> *mut Node;
    static mut yylval: YYSTYPE;
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
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
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type flex_int16_t = int16_t;
pub type flex_int32_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: yy_size_t,
    pub yy_n_chars: libc::c_int,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_bs_lineno: libc::c_int,
    pub yy_bs_column: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
pub type yy_size_t = size_t;
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
pub type YY_CHAR = libc::c_uchar;
pub type yy_state_type = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub lst: *mut List,
    pub node: *mut Node,
    pub cons: *mut Cons,
    pub stmt: *mut Stmt,
    pub expr: *mut Expr,
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn eat_comment() {
    let mut c: libc::c_int = 0;
    loop {
        c = input();
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if c == '\n' as i32 {
            linenum = linenum.wrapping_add(1);
            linenum;
        } else {
            if !(c == '*' as i32) {
                continue;
            }
            c = input();
            if c == '/' as i32 {
                return;
            }
            if c == -(1 as libc::c_int) {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error: EOF in comment\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                break;
            } else {
                yyunput(c, yytext);
            }
        }
    }
    yyerror(
        dcgettext(
            0 as *const libc::c_char,
            b"error: EOF in comment\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn yywrap() -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_string(
    mut len_return: *mut libc::c_uint,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: libc::c_int = 0 as libc::c_int;
    let mut bufpos: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_int = 0;
    let mut done: libc::c_int = 0 as libc::c_int;
    while done == 0 {
        ch = input();
        if ch == '\n' as i32 {
            linenum = linenum.wrapping_add(1);
            linenum;
        }
        let mut current_block_26: u64;
        match ch {
            -1 => {
                current_block_26 = 5795957800649065769;
            }
            34 => {
                done = 1 as libc::c_int;
                current_block_26 = 6417057564578538666;
            }
            92 => {
                ch = input();
                match ch {
                    110 => {
                        current_block_26 = 9979660290433074231;
                        match current_block_26 {
                            11849288559721163958 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            9979660290433074231 => {
                                ch = '\n' as i32;
                            }
                            14974146472005020892 => {
                                ch = '\t' as i32;
                            }
                            12114988635476690406 => {
                                ch = '\u{8}' as i32;
                            }
                            4782948518012976739 => {
                                ch = '\r' as i32;
                            }
                            6348304431286729986 => {
                                ch = '\u{c}' as i32;
                            }
                            7229422940172154008 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\u{b}' as i32;
                            }
                        }
                        current_block_26 = 4734241427616982247;
                    }
                    116 => {
                        current_block_26 = 14974146472005020892;
                        match current_block_26 {
                            11849288559721163958 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            9979660290433074231 => {
                                ch = '\n' as i32;
                            }
                            14974146472005020892 => {
                                ch = '\t' as i32;
                            }
                            12114988635476690406 => {
                                ch = '\u{8}' as i32;
                            }
                            4782948518012976739 => {
                                ch = '\r' as i32;
                            }
                            6348304431286729986 => {
                                ch = '\u{c}' as i32;
                            }
                            7229422940172154008 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\u{b}' as i32;
                            }
                        }
                        current_block_26 = 4734241427616982247;
                    }
                    118 => {
                        current_block_26 = 16642496812133611714;
                        match current_block_26 {
                            11849288559721163958 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            9979660290433074231 => {
                                ch = '\n' as i32;
                            }
                            14974146472005020892 => {
                                ch = '\t' as i32;
                            }
                            12114988635476690406 => {
                                ch = '\u{8}' as i32;
                            }
                            4782948518012976739 => {
                                ch = '\r' as i32;
                            }
                            6348304431286729986 => {
                                ch = '\u{c}' as i32;
                            }
                            7229422940172154008 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\u{b}' as i32;
                            }
                        }
                        current_block_26 = 4734241427616982247;
                    }
                    98 => {
                        current_block_26 = 12114988635476690406;
                        match current_block_26 {
                            11849288559721163958 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            9979660290433074231 => {
                                ch = '\n' as i32;
                            }
                            14974146472005020892 => {
                                ch = '\t' as i32;
                            }
                            12114988635476690406 => {
                                ch = '\u{8}' as i32;
                            }
                            4782948518012976739 => {
                                ch = '\r' as i32;
                            }
                            6348304431286729986 => {
                                ch = '\u{c}' as i32;
                            }
                            7229422940172154008 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\u{b}' as i32;
                            }
                        }
                        current_block_26 = 4734241427616982247;
                    }
                    114 => {
                        current_block_26 = 4782948518012976739;
                        match current_block_26 {
                            11849288559721163958 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            9979660290433074231 => {
                                ch = '\n' as i32;
                            }
                            14974146472005020892 => {
                                ch = '\t' as i32;
                            }
                            12114988635476690406 => {
                                ch = '\u{8}' as i32;
                            }
                            4782948518012976739 => {
                                ch = '\r' as i32;
                            }
                            6348304431286729986 => {
                                ch = '\u{c}' as i32;
                            }
                            7229422940172154008 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\u{b}' as i32;
                            }
                        }
                        current_block_26 = 4734241427616982247;
                    }
                    102 => {
                        current_block_26 = 6348304431286729986;
                        match current_block_26 {
                            11849288559721163958 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            9979660290433074231 => {
                                ch = '\n' as i32;
                            }
                            14974146472005020892 => {
                                ch = '\t' as i32;
                            }
                            12114988635476690406 => {
                                ch = '\u{8}' as i32;
                            }
                            4782948518012976739 => {
                                ch = '\r' as i32;
                            }
                            6348304431286729986 => {
                                ch = '\u{c}' as i32;
                            }
                            7229422940172154008 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\u{b}' as i32;
                            }
                        }
                        current_block_26 = 4734241427616982247;
                    }
                    97 => {
                        current_block_26 = 7229422940172154008;
                        match current_block_26 {
                            11849288559721163958 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            9979660290433074231 => {
                                ch = '\n' as i32;
                            }
                            14974146472005020892 => {
                                ch = '\t' as i32;
                            }
                            12114988635476690406 => {
                                ch = '\u{8}' as i32;
                            }
                            4782948518012976739 => {
                                ch = '\r' as i32;
                            }
                            6348304431286729986 => {
                                ch = '\u{c}' as i32;
                            }
                            7229422940172154008 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\u{b}' as i32;
                            }
                        }
                        current_block_26 = 4734241427616982247;
                    }
                    -1 => {
                        current_block_26 = 5795957800649065769;
                    }
                    _ => {
                        current_block_26 = 11849288559721163958;
                        match current_block_26 {
                            11849288559721163958 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                }
                            }
                            9979660290433074231 => {
                                ch = '\n' as i32;
                            }
                            14974146472005020892 => {
                                ch = '\t' as i32;
                            }
                            12114988635476690406 => {
                                ch = '\u{8}' as i32;
                            }
                            4782948518012976739 => {
                                ch = '\r' as i32;
                            }
                            6348304431286729986 => {
                                ch = '\u{c}' as i32;
                            }
                            7229422940172154008 => {
                                ch = '\u{7}' as i32;
                            }
                            _ => {
                                ch = '\u{b}' as i32;
                            }
                        }
                        current_block_26 = 4734241427616982247;
                    }
                }
            }
            _ => {
                current_block_26 = 4734241427616982247;
            }
        }
        match current_block_26 {
            4734241427616982247 => {
                if bufpos >= buflen {
                    buflen += 1024 as libc::c_int;
                    buf = xrealloc(buf as *mut libc::c_void, buflen as size_t)
                        as *mut libc::c_char;
                }
                let fresh0 = bufpos;
                bufpos = bufpos + 1;
                *buf.offset(fresh0 as isize) = ch as libc::c_char;
            }
            5795957800649065769 => {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error: EOF in string constant\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                done = 1 as libc::c_int;
            }
            _ => {}
        }
    }
    buf2 = xmalloc((bufpos + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        buf2 as *mut libc::c_void,
        buf as *const libc::c_void,
        bufpos as libc::c_ulong,
    );
    *buf2.offset(bufpos as isize) = '\0' as i32 as libc::c_char;
    xfree(buf as *mut libc::c_void);
    *len_return = bufpos as libc::c_uint;
    return buf2;
}
static mut yy_buffer_stack_top: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack_max: size_t = 0 as libc::c_int as size_t;
static mut yy_buffer_stack: *mut YY_BUFFER_STATE = 0 as *const YY_BUFFER_STATE
    as *mut YY_BUFFER_STATE;
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: libc::c_int = 0;
pub static mut yyleng: libc::c_int = 0;
static mut yy_c_buf_p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut yy_init: libc::c_int = 0 as libc::c_int;
static mut yy_start: libc::c_int = 0 as libc::c_int;
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
unsafe extern "C" fn read_regexp(mut node: *mut Node) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buflen: libc::c_int = 0 as libc::c_int;
    let mut bufpos: libc::c_int = 0 as libc::c_int;
    let mut ch: libc::c_int = 0;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut current_block_25: u64;
    while done == 0 {
        ch = input();
        match ch {
            -1 => {
                current_block_25 = 3528022185904443641;
            }
            47 => {
                done = 1 as libc::c_int;
                continue;
            }
            92 => {
                ch = input();
                match ch {
                    10 => {
                        current_block_25 = 4224242011178487887;
                        match current_block_25 {
                            295228879941021236 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            17210414846622777755 => {
                                ch = '\n' as i32;
                            }
                            5729957922015300560 => {
                                ch = '\r' as i32;
                            }
                            12134179792900939658 => {
                                ch = '\u{c}' as i32;
                            }
                            13202300793409042723 => {
                                ch = '\t' as i32;
                            }
                            9670367195697757662 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 4203005064024670655;
                    }
                    110 => {
                        current_block_25 = 17210414846622777755;
                        match current_block_25 {
                            295228879941021236 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            17210414846622777755 => {
                                ch = '\n' as i32;
                            }
                            5729957922015300560 => {
                                ch = '\r' as i32;
                            }
                            12134179792900939658 => {
                                ch = '\u{c}' as i32;
                            }
                            13202300793409042723 => {
                                ch = '\t' as i32;
                            }
                            9670367195697757662 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 4203005064024670655;
                    }
                    114 => {
                        current_block_25 = 5729957922015300560;
                        match current_block_25 {
                            295228879941021236 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            17210414846622777755 => {
                                ch = '\n' as i32;
                            }
                            5729957922015300560 => {
                                ch = '\r' as i32;
                            }
                            12134179792900939658 => {
                                ch = '\u{c}' as i32;
                            }
                            13202300793409042723 => {
                                ch = '\t' as i32;
                            }
                            9670367195697757662 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 4203005064024670655;
                    }
                    102 => {
                        current_block_25 = 12134179792900939658;
                        match current_block_25 {
                            295228879941021236 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            17210414846622777755 => {
                                ch = '\n' as i32;
                            }
                            5729957922015300560 => {
                                ch = '\r' as i32;
                            }
                            12134179792900939658 => {
                                ch = '\u{c}' as i32;
                            }
                            13202300793409042723 => {
                                ch = '\t' as i32;
                            }
                            9670367195697757662 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 4203005064024670655;
                    }
                    116 => {
                        current_block_25 = 13202300793409042723;
                        match current_block_25 {
                            295228879941021236 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            17210414846622777755 => {
                                ch = '\n' as i32;
                            }
                            5729957922015300560 => {
                                ch = '\r' as i32;
                            }
                            12134179792900939658 => {
                                ch = '\u{c}' as i32;
                            }
                            13202300793409042723 => {
                                ch = '\t' as i32;
                            }
                            9670367195697757662 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 4203005064024670655;
                    }
                    47 | 92 => {
                        current_block_25 = 9670367195697757662;
                        match current_block_25 {
                            295228879941021236 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            17210414846622777755 => {
                                ch = '\n' as i32;
                            }
                            5729957922015300560 => {
                                ch = '\r' as i32;
                            }
                            12134179792900939658 => {
                                ch = '\u{c}' as i32;
                            }
                            13202300793409042723 => {
                                ch = '\t' as i32;
                            }
                            9670367195697757662 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 4203005064024670655;
                    }
                    -1 => {
                        current_block_25 = 3528022185904443641;
                    }
                    _ => {
                        current_block_25 = 295228879941021236;
                        match current_block_25 {
                            295228879941021236 => {
                                if ch == '0' as i32 {
                                    let mut i: libc::c_int = 0;
                                    let mut val: libc::c_int = 0 as libc::c_int;
                                    i = 0 as libc::c_int;
                                    while i < 3 as libc::c_int {
                                        ch = input();
                                        if '0' as i32 <= ch && ch <= '7' as i32 {
                                            val = val * 8 as libc::c_int + ch - '0' as i32;
                                            i += 1;
                                            i;
                                        } else {
                                            yyunput(ch, yytext);
                                            break;
                                        }
                                    }
                                    ch = val;
                                } else {
                                    yyunput(ch, yytext);
                                    ch = '\\' as i32;
                                }
                            }
                            17210414846622777755 => {
                                ch = '\n' as i32;
                            }
                            5729957922015300560 => {
                                ch = '\r' as i32;
                            }
                            12134179792900939658 => {
                                ch = '\u{c}' as i32;
                            }
                            13202300793409042723 => {
                                ch = '\t' as i32;
                            }
                            9670367195697757662 => {}
                            _ => {
                                linenum = linenum.wrapping_add(1);
                                linenum;
                                continue;
                            }
                        }
                        current_block_25 = 4203005064024670655;
                    }
                }
            }
            _ => {
                current_block_25 = 4203005064024670655;
            }
        }
        match current_block_25 {
            4203005064024670655 => {
                if bufpos >= buflen {
                    buflen += 1024 as libc::c_int;
                    buf = xrealloc(buf as *mut libc::c_void, buflen as size_t)
                        as *mut libc::c_char;
                }
                let fresh1 = bufpos;
                bufpos = bufpos + 1;
                *buf.offset(fresh1 as isize) = ch as libc::c_char;
            }
            _ => {
                yyerror(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error: EOF in regular expression\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                done = 1 as libc::c_int;
            }
        }
    }
    done = 0 as libc::c_int;
    while done == 0 {
        ch = input();
        match ch {
            105 => {
                (*node).u.re.flags |= 1 as libc::c_int as libc::c_uint;
            }
            _ => {
                yyunput(ch, yytext);
                done = 1 as libc::c_int;
            }
        }
    }
    buf2 = xmalloc((bufpos + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    memcpy(
        buf2 as *mut libc::c_void,
        buf as *const libc::c_void,
        bufpos as libc::c_ulong,
    );
    *buf2.offset(bufpos as isize) = '\0' as i32 as libc::c_char;
    xfree(buf as *mut libc::c_void);
    (*node).u.re.data = buf2;
    (*node).u.re.len = bufpos as libc::c_uint;
}
pub static mut yyin: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut yyout: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut yylineno: libc::c_int = 1 as libc::c_int;
static mut yy_accept: [flex_int16_t; 114] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    2 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
];
static mut yy_ec: [flex_int32_t; 256] = [
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    1 as libc::c_int,
    6 as libc::c_int,
    1 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    1 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    14 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    15 as libc::c_int,
    16 as libc::c_int,
    17 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    18 as libc::c_int,
    19 as libc::c_int,
    18 as libc::c_int,
    20 as libc::c_int,
    21 as libc::c_int,
    18 as libc::c_int,
    22 as libc::c_int,
    18 as libc::c_int,
    23 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    24 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    1 as libc::c_int,
    25 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    18 as libc::c_int,
    1 as libc::c_int,
    26 as libc::c_int,
    27 as libc::c_int,
    28 as libc::c_int,
    29 as libc::c_int,
    30 as libc::c_int,
    31 as libc::c_int,
    18 as libc::c_int,
    32 as libc::c_int,
    33 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    34 as libc::c_int,
    35 as libc::c_int,
    36 as libc::c_int,
    37 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    38 as libc::c_int,
    39 as libc::c_int,
    40 as libc::c_int,
    41 as libc::c_int,
    42 as libc::c_int,
    43 as libc::c_int,
    44 as libc::c_int,
    18 as libc::c_int,
    18 as libc::c_int,
    1 as libc::c_int,
    45 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
];
static mut yy_meta: [flex_int32_t; 46] = [
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
];
static mut yy_base: [flex_int16_t; 118] = [
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    137 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    120 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    128 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    118 as libc::c_int as flex_int16_t,
    122 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    138 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
];
static mut yy_def: [flex_int16_t; 118] = [
    0 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    1 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    0 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
];
static mut yy_nxt: [flex_int16_t; 184] = [
    0 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    5 as libc::c_int as flex_int16_t,
    6 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    8 as libc::c_int as flex_int16_t,
    9 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    4 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    21 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    40 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    43 as libc::c_int as flex_int16_t,
    44 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    45 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    52 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    112 as libc::c_int as flex_int16_t,
    35 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    110 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    105 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    103 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    100 as libc::c_int as flex_int16_t,
    99 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    95 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    93 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    85 as libc::c_int as flex_int16_t,
    84 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    82 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    80 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    75 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    71 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    67 as libc::c_int as flex_int16_t,
    66 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    59 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    51 as libc::c_int as flex_int16_t,
    50 as libc::c_int as flex_int16_t,
    49 as libc::c_int as flex_int16_t,
    47 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    39 as libc::c_int as flex_int16_t,
    38 as libc::c_int as flex_int16_t,
    36 as libc::c_int as flex_int16_t,
    34 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
];
static mut yy_chk: [flex_int16_t; 184] = [
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
    13 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    17 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    13 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    42 as libc::c_int as flex_int16_t,
    14 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    31 as libc::c_int as flex_int16_t,
    25 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    116 as libc::c_int as flex_int16_t,
    79 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    111 as libc::c_int as flex_int16_t,
    114 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    115 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    109 as libc::c_int as flex_int16_t,
    117 as libc::c_int as flex_int16_t,
    108 as libc::c_int as flex_int16_t,
    107 as libc::c_int as flex_int16_t,
    106 as libc::c_int as flex_int16_t,
    104 as libc::c_int as flex_int16_t,
    102 as libc::c_int as flex_int16_t,
    101 as libc::c_int as flex_int16_t,
    98 as libc::c_int as flex_int16_t,
    97 as libc::c_int as flex_int16_t,
    96 as libc::c_int as flex_int16_t,
    94 as libc::c_int as flex_int16_t,
    92 as libc::c_int as flex_int16_t,
    91 as libc::c_int as flex_int16_t,
    90 as libc::c_int as flex_int16_t,
    89 as libc::c_int as flex_int16_t,
    88 as libc::c_int as flex_int16_t,
    87 as libc::c_int as flex_int16_t,
    86 as libc::c_int as flex_int16_t,
    83 as libc::c_int as flex_int16_t,
    81 as libc::c_int as flex_int16_t,
    78 as libc::c_int as flex_int16_t,
    77 as libc::c_int as flex_int16_t,
    76 as libc::c_int as flex_int16_t,
    74 as libc::c_int as flex_int16_t,
    73 as libc::c_int as flex_int16_t,
    72 as libc::c_int as flex_int16_t,
    70 as libc::c_int as flex_int16_t,
    69 as libc::c_int as flex_int16_t,
    68 as libc::c_int as flex_int16_t,
    65 as libc::c_int as flex_int16_t,
    64 as libc::c_int as flex_int16_t,
    63 as libc::c_int as flex_int16_t,
    62 as libc::c_int as flex_int16_t,
    61 as libc::c_int as flex_int16_t,
    60 as libc::c_int as flex_int16_t,
    58 as libc::c_int as flex_int16_t,
    57 as libc::c_int as flex_int16_t,
    56 as libc::c_int as flex_int16_t,
    55 as libc::c_int as flex_int16_t,
    54 as libc::c_int as flex_int16_t,
    53 as libc::c_int as flex_int16_t,
    48 as libc::c_int as flex_int16_t,
    46 as libc::c_int as flex_int16_t,
    41 as libc::c_int as flex_int16_t,
    37 as libc::c_int as flex_int16_t,
    33 as libc::c_int as flex_int16_t,
    32 as libc::c_int as flex_int16_t,
    30 as libc::c_int as flex_int16_t,
    29 as libc::c_int as flex_int16_t,
    28 as libc::c_int as flex_int16_t,
    27 as libc::c_int as flex_int16_t,
    26 as libc::c_int as flex_int16_t,
    24 as libc::c_int as flex_int16_t,
    23 as libc::c_int as flex_int16_t,
    22 as libc::c_int as flex_int16_t,
    20 as libc::c_int as flex_int16_t,
    19 as libc::c_int as flex_int16_t,
    18 as libc::c_int as flex_int16_t,
    16 as libc::c_int as flex_int16_t,
    15 as libc::c_int as flex_int16_t,
    12 as libc::c_int as flex_int16_t,
    11 as libc::c_int as flex_int16_t,
    10 as libc::c_int as flex_int16_t,
    7 as libc::c_int as flex_int16_t,
    3 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
    113 as libc::c_int as flex_int16_t,
];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut yy_flex_debug: libc::c_int = 0 as libc::c_int;
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
            let ref mut fresh2 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            *fresh2 = yy_create_buffer(yyin, 16384 as libc::c_int);
        }
        yy_load_buffer_state();
    }
    loop {
        yy_cp = yy_c_buf_p;
        *yy_cp = yy_hold_char;
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        '_yy_match: loop {
            loop {
                let mut yy_c: YY_CHAR = yy_ec[*yy_cp as libc::c_uchar as libc::c_uint
                    as usize] as YY_CHAR;
                if yy_accept[yy_current_state as usize] != 0 {
                    yy_last_accepting_state = yy_current_state;
                    yy_last_accepting_cpos = yy_cp;
                }
                while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int
                    + yy_c as libc::c_int) as usize] as libc::c_int != yy_current_state
                {
                    yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
                    if yy_current_state >= 114 as libc::c_int {
                        yy_c = yy_meta[yy_c as libc::c_uint as usize] as YY_CHAR;
                    }
                }
                yy_current_state = yy_nxt[(yy_base[yy_current_state as usize]
                    as libc::c_uint)
                    .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
                yy_cp = yy_cp.offset(1);
                yy_cp;
                if !(yy_base[yy_current_state as usize] as libc::c_int
                    != 138 as libc::c_int)
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
                yyleng = yy_cp.offset_from(yy_bp) as libc::c_long as size_t
                    as libc::c_int;
                yy_hold_char = *yy_cp;
                *yy_cp = '\0' as i32 as libc::c_char;
                yy_c_buf_p = yy_cp;
                loop {
                    match yy_act {
                        0 => {
                            *yy_cp = yy_hold_char;
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            continue '_yy_find_action;
                        }
                        1 => {
                            eat_comment();
                            break '_yy_match;
                        }
                        2 => {
                            break '_yy_match;
                        }
                        3 => {
                            linenum = linenum.wrapping_add(1);
                            linenum;
                            break '_yy_match;
                        }
                        4 => {
                            yylval.node = node_alloc(nSTRING);
                            (*yylval.node)
                                .u
                                .str_0
                                .data = read_string(&mut (*yylval.node).u.str_0.len);
                            return 260 as libc::c_int;
                        }
                        5 => {
                            yylval.node = node_alloc(nINTEGER);
                            (*yylval.node)
                                .u
                                .integer = *yytext.offset(1 as libc::c_int as isize)
                                as libc::c_int;
                            return 261 as libc::c_int;
                        }
                        6 => {
                            yylval.node = node_alloc(nINTEGER);
                            match *yytext.offset(2 as libc::c_int as isize)
                                as libc::c_int
                            {
                                110 => {
                                    (*yylval.node).u.integer = '\n' as i32;
                                }
                                116 => {
                                    (*yylval.node).u.integer = '\t' as i32;
                                }
                                118 => {
                                    (*yylval.node).u.integer = '\u{b}' as i32;
                                }
                                98 => {
                                    (*yylval.node).u.integer = '\u{8}' as i32;
                                }
                                114 => {
                                    (*yylval.node).u.integer = '\r' as i32;
                                }
                                102 => {
                                    (*yylval.node).u.integer = '\u{c}' as i32;
                                }
                                97 => {
                                    (*yylval.node).u.integer = '\u{7}' as i32;
                                }
                                _ => {
                                    (*yylval.node)
                                        .u
                                        .integer = *yytext.offset(2 as libc::c_int as isize)
                                        as libc::c_int;
                                }
                            }
                            return 261 as libc::c_int;
                        }
                        7 => {
                            yylval.node = node_alloc(nREGEXP);
                            read_regexp(yylval.node);
                            return 259 as libc::c_int;
                        }
                        8 => return 268 as libc::c_int,
                        9 => return 269 as libc::c_int,
                        10 => return 287 as libc::c_int,
                        11 => return 272 as libc::c_int,
                        12 => return 276 as libc::c_int,
                        13 => return 275 as libc::c_int,
                        14 => return 271 as libc::c_int,
                        15 => return 273 as libc::c_int,
                        16 => return 267 as libc::c_int,
                        17 => return 270 as libc::c_int,
                        18 => return 265 as libc::c_int,
                        19 => return 266 as libc::c_int,
                        20 => return 264 as libc::c_int,
                        21 => return 263 as libc::c_int,
                        22 => return 274 as libc::c_int,
                        23 => return 284 as libc::c_int,
                        24 => return 283 as libc::c_int,
                        25 => return 285 as libc::c_int,
                        26 => return 286 as libc::c_int,
                        27 => return 282 as libc::c_int,
                        28 => return 281 as libc::c_int,
                        29 => return 289 as libc::c_int,
                        30 => return 288 as libc::c_int,
                        31 => return 280 as libc::c_int,
                        32 => return 279 as libc::c_int,
                        33 => return 278 as libc::c_int,
                        34 => return 277 as libc::c_int,
                        35 => {
                            yylval.node = node_alloc(nREAL);
                            (*yylval.node).u.real = atof(yytext);
                            return 262 as libc::c_int;
                        }
                        36 => {
                            yylval.node = node_alloc(nINTEGER);
                            (*yylval.node).u.integer = atoi(yytext);
                            return 261 as libc::c_int;
                        }
                        37 => {
                            yylval.node = node_alloc(nSYMBOL);
                            (*yylval.node).u.sym = xstrdup(yytext);
                            return 258 as libc::c_int;
                        }
                        38 => {
                            return *yytext.offset(0 as libc::c_int as isize)
                                as libc::c_int;
                        }
                        39 => {
                            fwrite(
                                yytext as *const libc::c_void,
                                yyleng as libc::c_ulong,
                                1 as libc::c_int as libc::c_ulong,
                                yyout,
                            ) != 0;
                            break '_yy_match;
                        }
                        41 => return 0 as libc::c_int,
                        40 => {
                            yy_amount_of_matched_text = yy_cp.offset_from(yytext)
                                as libc::c_long as libc::c_int - 1 as libc::c_int;
                            *yy_cp = yy_hold_char;
                            if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                                .yy_buffer_status == 0 as libc::c_int
                            {
                                yy_n_chars = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_n_chars;
                                let ref mut fresh3 = (**yy_buffer_stack
                                    .offset(yy_buffer_stack_top as isize))
                                    .yy_input_file;
                                *fresh3 = yyin;
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
                                    current_block = 16974974966130203269;
                                    break;
                                } else {
                                    current_block = 11227437541145425351;
                                    break;
                                }
                            } else {
                                match yy_get_next_buffer() {
                                    1 => {
                                        yy_did_buffer_switch_on_eof = 0 as libc::c_int;
                                        if yywrap() != 0 {
                                            yy_c_buf_p = yytext.offset(0 as libc::c_int as isize);
                                            yy_act = 40 as libc::c_int
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
                            break '_yy_match;
                        }
                    }
                }
                match current_block {
                    11227437541145425351 => {
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
    let mut source: *mut libc::c_char = yytext;
    let mut number_to_move: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret_val: libc::c_int = 0;
    if yy_c_buf_p
        > &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
            .offset((yy_n_chars + 1 as libc::c_int) as isize) as *mut libc::c_char
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
    number_to_move = yy_c_buf_p.offset_from(yytext) as libc::c_long as libc::c_int
        - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        let fresh4 = source;
        source = source.offset(1);
        let fresh5 = dest;
        dest = dest.offset(1);
        *fresh5 = *fresh4;
        i += 1;
        i;
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buffer_status
        == 2 as libc::c_int
    {
        yy_n_chars = 0 as libc::c_int;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    } else {
        let mut num_to_read: libc::c_int = ((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_buf_size)
            .wrapping_sub(number_to_move as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        while num_to_read <= 0 as libc::c_int {
            let mut b: YY_BUFFER_STATE = if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else {
                0 as YY_BUFFER_STATE
            };
            let mut yy_c_buf_p_offset: libc::c_int = yy_c_buf_p
                .offset_from((*b).yy_ch_buf) as libc::c_long as libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: libc::c_int = ((*b).yy_buf_size)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int;
                if new_size <= 0 as libc::c_int {
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
                .wrapping_sub(number_to_move as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        if num_to_read > 8192 as libc::c_int {
            num_to_read = 8192 as libc::c_int;
        }
        if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_is_interactive
            != 0
        {
            let mut c: libc::c_int = '*' as i32;
            let mut n: size_t = 0;
            n = 0 as libc::c_int as size_t;
            while n < num_to_read as size_t
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
                let fresh6 = n;
                n = n.wrapping_add(1);
                *(&mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                    .yy_ch_buf)
                    .offset(number_to_move as isize) as *mut libc::c_char)
                    .offset(fresh6 as isize) = c as libc::c_char;
            }
            if c == -(1 as libc::c_int) && ferror(yyin) != 0 {
                yy_fatal_error(
                    b"input in flex scanner failed\0" as *const u8 as *const libc::c_char,
                );
            }
            yy_n_chars = n as libc::c_int;
        } else {
            *__errno_location() = 0 as libc::c_int;
            loop {
                yy_n_chars = fread(
                    &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
                        .yy_ch_buf)
                        .offset(number_to_move as isize) as *mut libc::c_char
                        as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    num_to_read as size_t,
                    yyin,
                ) as libc::c_int;
                if !(yy_n_chars == 0 as libc::c_int && ferror(yyin) != 0) {
                    break;
                }
                if *__errno_location() != 4 as libc::c_int {
                    yy_fatal_error(
                        b"input in flex scanner failed\0" as *const u8
                            as *const libc::c_char,
                    );
                    break;
                } else {
                    *__errno_location() = 0 as libc::c_int;
                    clearerr(yyin);
                }
            }
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    if yy_n_chars == 0 as libc::c_int {
        if number_to_move == 0 as libc::c_int {
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
    if (yy_n_chars + number_to_move) as yy_size_t
        > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
    {
        let mut new_size_0: yy_size_t = (yy_n_chars + number_to_move
            + (yy_n_chars >> 1 as libc::c_int)) as yy_size_t;
        let ref mut fresh7 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_ch_buf;
        *fresh7 = yyrealloc(
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
    yy_n_chars += number_to_move;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(yy_n_chars as isize) = 0 as libc::c_int as libc::c_char;
    *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(
            (yy_n_chars + 1 as libc::c_int) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    yytext = &mut *((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf)
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    return ret_val;
}
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_current_state = yy_start;
    yy_cp = yytext.offset(0 as libc::c_int as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR = (if *yy_cp as libc::c_int != 0 {
            yy_ec[*yy_cp as libc::c_uchar as libc::c_uint as usize]
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
            if yy_current_state >= 114 as libc::c_int {
                yy_c = yy_meta[yy_c as libc::c_uint as usize] as YY_CHAR;
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
        if yy_current_state >= 114 as libc::c_int {
            yy_c = yy_meta[yy_c as libc::c_uint as usize] as YY_CHAR;
        }
    }
    yy_current_state = yy_nxt[(yy_base[yy_current_state as usize] as libc::c_uint)
        .wrapping_add(yy_c as libc::c_uint) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 113 as libc::c_int) as libc::c_int;
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
        let mut number_to_move: libc::c_int = yy_n_chars + 2 as libc::c_int;
        let mut dest: *mut libc::c_char = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(
                ((**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut libc::c_char;
        let mut source: *mut libc::c_char = &mut *((**yy_buffer_stack
            .offset(yy_buffer_stack_top as isize))
            .yy_ch_buf)
            .offset(number_to_move as isize) as *mut libc::c_char;
        while source > (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf
        {
            source = source.offset(-1);
            dest = dest.offset(-1);
            *dest = *source;
        }
        yy_cp = yy_cp
            .offset(dest.offset_from(source) as libc::c_long as libc::c_int as isize);
        yy_bp = yy_bp
            .offset(dest.offset_from(source) as libc::c_long as libc::c_int as isize);
        yy_n_chars = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size
            as libc::c_int;
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
            let mut offset: libc::c_int = yy_c_buf_p.offset_from(yytext) as libc::c_long
                as libc::c_int;
            yy_c_buf_p = yy_c_buf_p.offset(1);
            yy_c_buf_p;
            let mut current_block_10: u64;
            match yy_get_next_buffer() {
                2 => {
                    yyrestart(yyin);
                    current_block_10 = 1581192455180053011;
                }
                1 => {
                    current_block_10 = 1581192455180053011;
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
        let ref mut fresh8 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh8 = yy_create_buffer(yyin, 16384 as libc::c_int);
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
        let ref mut fresh9 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh9 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars = yy_n_chars;
    }
    let ref mut fresh10 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh10 = new_buffer;
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
        let ref mut fresh11 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh11 = 0 as YY_BUFFER_STATE;
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
    (*b).yy_n_chars = 0 as libc::c_int;
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
        let ref mut fresh12 = (**yy_buffer_stack.offset(yy_buffer_stack_top as isize))
            .yy_buf_pos;
        *fresh12 = yy_c_buf_p;
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
    let ref mut fresh13 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh13 = new_buffer;
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
    let ref mut fresh14 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh14 = 0 as YY_BUFFER_STATE;
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
    let mut num_to_alloc: libc::c_int = 0;
    if yy_buffer_stack.is_null() {
        num_to_alloc = 1 as libc::c_int;
        yy_buffer_stack = yyalloc(
            (num_to_alloc as libc::c_ulong)
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
            (num_to_alloc as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc as size_t;
        yy_buffer_stack_top = 0 as libc::c_int as size_t;
        return;
    }
    if yy_buffer_stack_top
        >= yy_buffer_stack_max.wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        let mut grow_size: libc::c_int = 8 as libc::c_int;
        num_to_alloc = yy_buffer_stack_max.wrapping_add(grow_size as libc::c_ulong)
            as libc::c_int;
        yy_buffer_stack = yyrealloc(
            yy_buffer_stack as *mut libc::c_void,
            (num_to_alloc as libc::c_ulong)
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
            (grow_size as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut yy_buffer_state>() as libc::c_ulong,
                ),
        );
        yy_buffer_stack_max = num_to_alloc as size_t;
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
    (*b).yy_n_chars = (*b).yy_buf_size as libc::c_int;
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
    return yy_scan_bytes(yystr, strlen(yystr) as libc::c_int);
}
pub unsafe extern "C" fn yy_scan_bytes(
    mut yybytes: *const libc::c_char,
    mut _yybytes_len: libc::c_int,
) -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: libc::c_int = 0;
    n = (_yybytes_len + 2 as libc::c_int) as yy_size_t;
    buf = yyalloc(n) as *mut libc::c_char;
    if buf.is_null() {
        yy_fatal_error(
            b"out of dynamic memory in yy_scan_bytes()\0" as *const u8
                as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < _yybytes_len {
        *buf.offset(i as isize) = *yybytes.offset(i as isize);
        i += 1;
        i;
    }
    let ref mut fresh15 = *buf.offset((_yybytes_len + 1 as libc::c_int) as isize);
    *fresh15 = 0 as libc::c_int as libc::c_char;
    *buf.offset(_yybytes_len as isize) = *fresh15;
    b = yy_scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(
            b"bad buffer in yy_scan_bytes()\0" as *const u8 as *const libc::c_char,
        );
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char) {
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
pub unsafe extern "C" fn yyget_leng() -> libc::c_int {
    return yyleng;
}
pub unsafe extern "C" fn yyget_text() -> *mut libc::c_char {
    return yytext;
}
pub unsafe extern "C" fn yyset_lineno(mut line_number: libc::c_int) {
    yylineno = line_number;
}
pub unsafe extern "C" fn yyset_in(mut in_str: *mut FILE) {
    yyin = in_str;
}
pub unsafe extern "C" fn yyset_out(mut out_str: *mut FILE) {
    yyout = out_str;
}
pub unsafe extern "C" fn yyget_debug() -> libc::c_int {
    return yy_flex_debug;
}
pub unsafe extern "C" fn yyset_debug(mut bdebug: libc::c_int) {
    yy_flex_debug = bdebug;
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
        let ref mut fresh16 = *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh16 = 0 as YY_BUFFER_STATE;
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
