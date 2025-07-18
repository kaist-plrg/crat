use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: libc::c_int) -> libc::c_int;
    fn siglongjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    static mut pl_tag_tbl: [InfTag; 0];
    static mut pl_stk_tbl: [InfStack; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_max_atom: PlULong;
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    static mut pl_pred_tbl: *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut pl_reg_tbl: [*mut libc::c_char; 0];
    fn Pl_Push_SIGSEGV_Handler(handler: SegvHdlr);
    fn Pl_Pop_SIGSEGV_Handler();
    fn Pl_Hash_Next(scan: *mut HashScan) -> *mut libc::c_char;
    fn Pl_Hash_First(tbl: *mut libc::c_char, scan: *mut HashScan) -> *mut libc::c_char;
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Stream_Printf(
        pstm: *mut StmInf,
        format: *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_stm_debugger_input: libc::c_int;
    static mut pl_stm_debugger_output: libc::c_int;
    fn Pl_Stream_Gets_Prompt(
        prompt: *mut libc::c_char,
        pstm_o_0: *mut StmInf,
        str: *mut libc::c_char,
        size: libc::c_int,
        pstm_i_0: *mut StmInf,
    ) -> *mut libc::c_char;
    fn Pl_Write_Term(
        pstm: *mut StmInf,
        depth: libc::c_int,
        prec: libc::c_int,
        mask: libc::c_int,
        above_H: *mut WamWord,
        term_word: WamWord,
    );
    static mut pl_flag_debug: *mut FlagInf;
    fn Pl_Scan_Choice_Point_Pred(
        b: *mut WamWord,
        arity: *mut libc::c_int,
    ) -> libc::c_int;
    fn X1_2464656275675F63616C6C__a2();
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashScan {
    pub endt: *mut libc::c_char,
    pub cur_t: *mut libc::c_char,
    pub cur_p: *mut libc::c_char,
}
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
pub struct AtomInf {
    pub name: *mut libc::c_char,
    pub hash: libc::c_uint,
    pub prop: AtomProp,
    pub info: *mut libc::c_void,
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
pub type SegvHdlr = Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct StmProp {
    #[bitfield(name = "mode", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "input", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "output", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "text", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "reposition", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "eof_action", ty = "libc::c_uint", bits = "6..=7")]
    #[bitfield(name = "buffering", ty = "libc::c_uint", bits = "8..=9")]
    #[bitfield(name = "special_close", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "other", ty = "libc::c_uint", bits = "11..=15")]
    pub mode_input_output_text_reposition_eof_action_buffering_special_close_other: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PbStk {
    pub buff: [libc::c_int; 8],
    pub ptr: *mut libc::c_int,
    pub nb_elems: libc::c_int,
}
pub type StmFct = Option::<unsafe extern "C" fn() -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_lst {
    pub stm: libc::c_int,
    pub next: PStmLst,
}
pub type PStmLst = *mut stm_lst;
pub type StmLst = stm_lst;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_inf {
    pub atom_file_name: libc::c_int,
    pub file: PlLong,
    pub prop: StmProp,
    pub mirror: *mut StmLst,
    pub mirror_of: *mut StmLst,
    pub fct_getc: StmFct,
    pub fct_putc: StmFct,
    pub fct_flush: StmFct,
    pub fct_close: StmFct,
    pub fct_tell: StmFct,
    pub fct_seek: StmFct,
    pub fct_clearerr: StmFct,
    pub eof_reached: Bool,
    pub pb_char: PbStk,
    pub char_count: PlLong,
    pub line_count: PlLong,
    pub line_pos: PlLong,
    pub pb_line_pos: PbStk,
}
pub type StmInf = stm_inf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flag_inf {
    pub atom_name: libc::c_int,
    pub modifiable: Bool,
    pub type_0: FlagType,
    pub value: PlLong,
    pub fct_get: FlagFctGet,
    pub fct_chk: FlagFctChk,
    pub fct_set: FlagFctSet,
}
pub type FlagFctSet = Option::<unsafe extern "C" fn(FlagInfP, WamWord) -> Bool>;
pub type FlagInfP = *mut flag_inf;
pub type FlagFctChk = Option::<unsafe extern "C" fn(FlagInfP, WamWord, WamWord) -> Bool>;
pub type FlagFctGet = Option::<unsafe extern "C" fn(FlagInfP) -> WamWord>;
pub type FlagType = libc::c_uint;
pub const PF_TYPE_ANY: FlagType = 7;
pub const PF_TYPE_QUOTES: FlagType = 6;
pub const PF_TYPE_ERR: FlagType = 5;
pub const PF_TYPE_ON_OFF: FlagType = 4;
pub const PF_TYPE_BOOL: FlagType = 3;
pub const PF_TYPE_ROUND: FlagType = 2;
pub const PF_TYPE_ATOM: FlagType = 1;
pub const PF_TYPE_INTEGER: FlagType = 0;
pub type FlagInf = flag_inf;
pub type FctPtr = Option::<unsafe extern "C" fn() -> Bool>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfCmd {
    pub name: *mut libc::c_char,
    pub fct: FctPtr,
}
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_debug_call_code: WamCont = None;
static mut nb_read_arg: libc::c_int = 0;
static mut read_arg: [[libc::c_char; 80]; 30] = [[0; 80]; 30];
static mut envir_name: [*mut libc::c_char; 3] = [
    b"CPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"BCIE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut choice_name: [*mut libc::c_char; 8] = [
    b"ALTB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CPB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"BCIB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"BB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"HB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"TRB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CSB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut trail_tag_name: [*mut libc::c_char; 4] = [
    b"TUV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"TOV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"TMV\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"TFC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut reg_copy: [WamWord; 11] = [0; 11];
static mut pstm_i: *mut StmInf = 0 as *const StmInf as *mut StmInf;
static mut pstm_o: *mut StmInf = 0 as *const StmInf as *mut StmInf;
static mut dbg_jumper: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut invalid_addr: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
pub unsafe extern "C" fn Pl_Set_Debug_Call_Code_0() {
    pl_debug_call_code = Some(
        ::std::mem::transmute::<
            unsafe extern "C" fn() -> (),
            unsafe extern "C" fn() -> (),
        >(X1_2464656275675F63616C6C__a2),
    );
    (*pl_flag_debug).value = 1 as libc::c_int as PlLong;
}
pub unsafe extern "C" fn Pl_Reset_Debug_Call_Code_0() {
    pl_debug_call_code = None;
    (*pl_flag_debug).value = 0 as libc::c_int as PlLong;
}
pub unsafe extern "C" fn Pl_Remove_One_Choice_Point_1(mut b_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut b: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = b_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    b = (*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
        .offset((word << 0 as libc::c_int >> 3 as libc::c_int) as isize);
    B = *(&mut *b.offset(-(5 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
}
pub unsafe extern "C" fn Pl_Choice_Point_Info_4(
    mut b_word: WamWord,
    mut name_word: WamWord,
    mut arity_word: WamWord,
    mut lastb_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut b: *mut WamWord = 0 as *mut WamWord;
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut last_pred: *mut PredInf = 0 as *mut PredInf;
    let mut code: PlULong = 0;
    let mut code1: PlULong = 0;
    let mut last_code: PlULong = 0 as libc::c_int as PlULong;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = b_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    b = (*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
        .offset((word << 0 as libc::c_int >> 3 as libc::c_int) as isize);
    code = ::std::mem::transmute::<
        CodePtr,
        PlULong,
    >(*(&mut *b.offset(-(1 as libc::c_int) as isize) as *mut WamWord as *mut CodePtr));
    pred = Pl_Hash_First(pl_pred_tbl, &mut scan) as *mut PredInf;
    while !pred.is_null() {
        code1 = (*pred).codep as PlULong;
        if code >= code1 && code1 >= last_code {
            last_pred = pred;
            last_code = code1;
        }
        pred = Pl_Hash_Next(&mut scan) as *mut PredInf;
    }
    func = ((*last_pred).f_n as PlULong
        & ((1 as libc::c_int as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
    arity = ((*last_pred).f_n as PlULong
        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
    Pl_Get_Atom(func, name_word);
    Pl_Get_Integer(arity as PlLong, arity_word);
    Pl_Unify(
        (((*(&mut *b.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord))
            .offset_from(
                *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize),
            ) as libc::c_long as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord,
        lastb_word,
    );
}
pub unsafe extern "C" fn Pl_Scan_Choice_Point_Info_3(
    mut b_word: WamWord,
    mut name_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut b: *mut WamWord = 0 as *mut WamWord;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = b_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    b = (*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
        .offset((word << 0 as libc::c_int >> 3 as libc::c_int) as isize);
    func = Pl_Scan_Choice_Point_Pred(b, &mut arity);
    if func < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    Pl_Get_Atom(func, name_word);
    Pl_Get_Integer(arity as PlLong, arity_word);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Choice_Point_Arg_3(
    mut b_word: WamWord,
    mut i_word: WamWord,
    mut arg_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut b: *mut WamWord = 0 as *mut WamWord;
    let mut i: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = b_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    b = (*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
        .offset((word << 0 as libc::c_int >> 3 as libc::c_int) as isize);
    let mut deref_last_word_0: WamWord = 0;
    word = i_word;
    loop {
        deref_last_word_0 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_0) {
            break;
        }
    }
    i = (word << 0 as libc::c_int >> 3 as libc::c_int)
        - 1 as libc::c_int as libc::c_long;
    Pl_Unify(
        arg_word,
        *(&mut *b.offset((-(9 as libc::c_int) as libc::c_long - i) as isize)
            as *mut WamWord),
    );
}
unsafe extern "C" fn Debugger_SIGSEGV_Handler(
    mut bad_addr: *mut libc::c_void,
) -> libc::c_int {
    invalid_addr = bad_addr;
    siglongjmp(dbg_jumper.as_mut_ptr(), 1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Debug_Wam() {
    let mut command: FctPtr = None;
    let mut str: [libc::c_char; 80] = *::std::mem::transmute::<
        &[u8; 80],
        &mut [libc::c_char; 80],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut prompt: *mut libc::c_char = b"(wam debug) \0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    pstm_i = *pl_stm_tbl.offset(pl_stm_debugger_input as isize);
    pstm_o = *pl_stm_tbl.offset(pl_stm_debugger_output as isize);
    Pl_Stream_Printf(
        pstm_o,
        b"Welcome to the WAM debugger - experts only\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    Pl_Push_SIGSEGV_Handler(
        Some(
            Debugger_SIGSEGV_Handler
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
        ),
    );
    loop {
        ret = __sigsetjmp(dbg_jumper.as_mut_ptr(), 1 as libc::c_int);
        if !(ret != 0 as libc::c_int) {
            break;
        }
        Pl_Stream_Printf(
            pstm_o,
            b"SIGSEGV occured at: %p\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            invalid_addr,
        );
    }
    while !(Pl_Stream_Gets_Prompt(
        prompt,
        pstm_o,
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int,
        pstm_i,
    ))
        .is_null()
    {
        Scan_Command(str.as_mut_ptr());
        command = Find_Function();
        if command
            == ::std::mem::transmute::<
                libc::intptr_t,
                FctPtr,
            >(-(1 as libc::c_int) as libc::intptr_t)
        {
            break;
        }
        if command.is_some() {
            ::std::mem::transmute::<
                _,
                fn() -> Bool,
            >((Some(command.unwrap())).unwrap())();
        }
    }
    Pl_Pop_SIGSEGV_Handler();
}
unsafe extern "C" fn Scan_Command(mut source_str: *mut libc::c_char) {
    let mut str: [libc::c_char; 80] = [0; 80];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    strcpy(str.as_mut_ptr(), source_str);
    nb_read_arg = 0 as libc::c_int;
    p = strtok(str.as_mut_ptr(), b" ,[]\n\0" as *const u8 as *const libc::c_char);
    while !p.is_null() {
        q = p;
        p = strtok(
            0 as *mut libc::c_char,
            b" ,[]\n\0" as *const u8 as *const libc::c_char,
        );
        let fresh1 = nb_read_arg;
        nb_read_arg = nb_read_arg + 1;
        strcpy((read_arg[fresh1 as usize]).as_mut_ptr(), q);
    }
}
static mut cmd: [InfCmd; 10] = [InfCmd {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    fct: None,
}; 10];
unsafe extern "C" fn Find_Function() -> FctPtr {
    let mut lg: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if nb_read_arg == 0 as libc::c_int {
        return None;
    }
    lg = strlen((read_arg[0 as libc::c_int as usize]).as_mut_ptr()) as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[InfCmd; 10]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<InfCmd>() as libc::c_ulong)
    {
        if strncmp(
            cmd[i as usize].name,
            (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
            lg as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return cmd[i as usize].fct;
        }
        i += 1;
        i;
    }
    Pl_Stream_Printf(
        pstm_o,
        b"Unknown command - try help\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return None;
}
unsafe extern "C" fn Write_Data_Modify() -> Bool {
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut bank_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut offset: libc::c_int = 0;
    let mut nb: libc::c_int = 0;
    let mut incr: libc::c_int = 1 as libc::c_int;
    adr = Read_Bank_Adr(0 as libc::c_int, 1 as libc::c_int, &mut bank_name);
    if !adr.is_null() {
        offset = (if nb_read_arg < 3 as libc::c_int {
            0 as libc::c_int as libc::c_long
        } else {
            Read_An_Integer(2 as libc::c_int)
        }) as libc::c_int;
        nb = (if nb_read_arg < 4 as libc::c_int {
            1 as libc::c_int as libc::c_long
        } else {
            Read_An_Integer(3 as libc::c_int)
        }) as libc::c_int;
        if adr == reg_copy.as_mut_ptr() {
            if offset >= 11 as libc::c_int {
                offset = 0 as libc::c_int;
            }
            if nb_read_arg < 4 as libc::c_int
                && *(read_arg[0 as libc::c_int as usize]).as_mut_ptr() as libc::c_int
                    != 'm' as i32
            {
                nb = 11 as libc::c_int - offset;
            } else if nb > 11 as libc::c_int - offset {
                nb = 11 as libc::c_int - offset;
            }
        } else if strcmp(bank_name, b"y\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(bank_name, b"ab\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            incr = -(1 as libc::c_int);
        }
        loop {
            let fresh2 = nb;
            nb = nb - 1;
            if !(fresh2 != 0) {
                break;
            }
            Print_Bank_Name_Offset(
                (if adr == reg_copy.as_mut_ptr() {
                    *pl_reg_tbl.as_mut_ptr().offset(offset as isize)
                        as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char,
                bank_name,
                offset,
            );
            Pl_Stream_Printf(
                pstm_o,
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if *(read_arg[0 as libc::c_int as usize]).as_mut_ptr() as libc::c_int
                == 'w' as i32
            {
                Pl_Write_Term(
                    pstm_o,
                    -(1 as libc::c_int),
                    1200 as libc::c_int,
                    4 as libc::c_int | 8 as libc::c_int,
                    0 as *mut WamWord,
                    *adr.offset(offset as isize),
                );
            } else {
                Print_Wam_Word(adr.offset(offset as isize));
                if *(read_arg[0 as libc::c_int as usize]).as_mut_ptr() as libc::c_int
                    == 'm' as i32
                {
                    Modify_Wam_Word(adr.offset(offset as isize));
                }
            }
            Pl_Stream_Printf(
                pstm_o,
                b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            offset += incr;
        }
    }
    if adr == reg_copy.as_mut_ptr() {
        TR = reg_copy[0 as libc::c_int as usize] as WamWordP;
        B = reg_copy[1 as libc::c_int as usize] as WamWordP;
        H = reg_copy[2 as libc::c_int as usize] as WamWordP;
        let ref mut fresh3 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh3 = reg_copy[3 as libc::c_int as usize] as WamWordP;
        let ref mut fresh4 = *(pl_reg_bank as *mut WamCont)
            .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
        *fresh4 = ::std::mem::transmute::<
            libc::intptr_t,
            WamCont,
        >(reg_copy[4 as libc::c_int as usize] as libc::intptr_t);
        let ref mut fresh5 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
        *fresh5 = reg_copy[5 as libc::c_int as usize] as WamWordP;
        let ref mut fresh6 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
        *fresh6 = reg_copy[6 as libc::c_int as usize] as WamWordP;
        let ref mut fresh7 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        *fresh7 = reg_copy[7 as libc::c_int as usize] as WamWordP;
        *pl_reg_bank
            .offset(
                (256 as libc::c_int + 5 as libc::c_int) as isize,
            ) = reg_copy[8 as libc::c_int as usize];
        *pl_reg_bank
            .offset(
                (256 as libc::c_int + 6 as libc::c_int) as isize,
            ) = reg_copy[9 as libc::c_int as usize];
        let ref mut fresh8 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 7 as libc::c_int) as isize);
        *fresh8 = reg_copy[10 as libc::c_int as usize] as WamWordP;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn What() -> Bool {
    let mut adr: *mut PlLong = 0 as *mut PlLong;
    let mut adr1: *mut WamWord = 0 as *mut WamWord;
    let mut stack_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    if nb_read_arg < 2 as libc::c_int {
        Pl_Stream_Printf(
            pstm_o,
            b"integer expected\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    adr = Read_An_Integer(1 as libc::c_int) as *mut PlLong;
    Pl_Stream_Printf(
        pstm_o,
        b" %#lx = \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        adr as PlLong,
    );
    adr1 = Detect_Stack(adr, &mut stack_name);
    if !adr1.is_null() {
        Print_Bank_Name_Offset(
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stack_name,
            adr.offset_from(adr1) as libc::c_long as libc::c_int,
        );
        Pl_Stream_Printf(
            pstm_o,
            b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    pred = Detect_Pred_From_Code(adr);
    if !pred.is_null() {
        func = ((*pred).f_n as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        arity = ((*pred).f_n as PlULong
            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
        Pl_Stream_Printf(
            pstm_o,
            b"%s/%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*pl_atom_tbl.offset(func as isize)).name,
            arity,
        );
        if adr > (*pred).codep {
            Pl_Stream_Printf(
                pstm_o,
                b"+%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (adr as *mut libc::c_char)
                    .offset_from((*pred).codep as *mut libc::c_char) as libc::c_long,
            );
        }
        Pl_Stream_Printf(
            pstm_o,
            b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    Pl_Stream_Printf(
        pstm_o,
        b"???\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn Where() -> Bool {
    let mut bank_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut offset: libc::c_int = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    adr = Read_Bank_Adr(0 as libc::c_int, 1 as libc::c_int, &mut bank_name);
    if !adr.is_null() {
        offset = (if nb_read_arg < 3 as libc::c_int {
            0 as libc::c_int as libc::c_long
        } else {
            Read_An_Integer(2 as libc::c_int)
        }) as libc::c_int;
        if strcmp(bank_name, b"y\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(bank_name, b"ab\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            offset = -offset;
        }
        Print_Bank_Name_Offset(
            (if adr == reg_copy.as_mut_ptr() {
                *pl_reg_tbl.as_mut_ptr().offset(offset as isize) as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
            bank_name,
            offset,
        );
        Pl_Stream_Printf(
            pstm_o,
            b" at %#lx\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            adr.offset(offset as isize) as PlLong,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Dereference() -> Bool {
    let mut bank_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stack_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut offset: libc::c_int = 0;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut word1: WamWord = 0;
    let mut d_adr: *mut WamWord = 0 as *mut WamWord;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    adr = Read_Bank_Adr(0 as libc::c_int, 1 as libc::c_int, &mut bank_name);
    if !adr.is_null() {
        offset = (if nb_read_arg < 3 as libc::c_int {
            0 as libc::c_int as libc::c_long
        } else {
            Read_An_Integer(2 as libc::c_int)
        }) as libc::c_int;
        if strcmp(bank_name, b"y\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(bank_name, b"ab\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            offset = -offset;
        }
        d_adr = 0 as *mut WamWord;
        word = *adr.offset(offset as isize);
        loop {
            word1 = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            d_adr = word as *mut WamWord;
            word = *d_adr;
            if !(word != word1) {
                break;
            }
        }
        Print_Bank_Name_Offset(
            (if adr == reg_copy.as_mut_ptr() {
                *pl_reg_tbl.as_mut_ptr().offset(offset as isize) as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
            bank_name,
            offset,
        );
        Pl_Stream_Printf(
            pstm_o,
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !d_adr.is_null()
            && {
                adr = Detect_Stack(d_adr, &mut stack_name);
                !adr.is_null()
            }
        {
            Pl_Stream_Printf(
                pstm_o,
                b" --> \n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            Print_Bank_Name_Offset(
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                stack_name,
                d_adr.offset_from(adr) as libc::c_long as libc::c_int,
            );
            Pl_Stream_Printf(
                pstm_o,
                b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        Print_Wam_Word(
            if !d_adr.is_null() { d_adr } else { adr.offset(offset as isize) },
        );
        Pl_Stream_Printf(
            pstm_o,
            b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Environment() -> Bool {
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut offset: libc::c_int = 0;
    let mut stack_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if nb_read_arg == 1 as libc::c_int {
        adr = Detect_Stack(
            *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 2 as libc::c_int) as isize),
            &mut stack_name,
        );
        offset = (*(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
            .offset_from(adr) as libc::c_long as libc::c_int;
        adr = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    } else {
        adr = Read_Bank_Adr(1 as libc::c_int, 1 as libc::c_int, &mut stack_name);
        if adr.is_null() {
            return 0 as libc::c_int;
        }
        offset = (if nb_read_arg < 3 as libc::c_int {
            0 as libc::c_int as libc::c_long
        } else {
            Read_An_Integer(2 as libc::c_int)
        }) as libc::c_int;
        adr = adr.offset(offset as isize);
    }
    i = 3 as libc::c_int;
    while i > 0 as libc::c_int {
        Print_Bank_Name_Offset(
            envir_name[(i - 1 as libc::c_int) as usize],
            stack_name,
            offset - i,
        );
        Pl_Stream_Printf(
            pstm_o,
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        Print_Wam_Word(adr.offset(-(i as isize)));
        Pl_Stream_Printf(
            pstm_o,
            b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        i -= 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Backtrack() -> Bool {
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut offset: libc::c_int = 0;
    let mut stack_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    if nb_read_arg == 2 as libc::c_int
        && strncmp(
            (read_arg[1 as libc::c_int as usize]).as_mut_ptr(),
            b"all\0" as *const u8 as *const libc::c_char,
            strlen((read_arg[1 as libc::c_int as usize]).as_mut_ptr()),
        ) == 0 as libc::c_int
    {
        Detect_Stack(B, &mut stack_name);
        adr = B;
        while adr
            > ((*pl_stk_tbl.as_mut_ptr().offset(3 as libc::c_int as isize)).stack)
                .offset(10 as libc::c_int as isize)
        {
            pred = Detect_Pred_From_Code(
                ::std::mem::transmute::<
                    CodePtr,
                    *mut PlLong,
                >(
                    *(&mut *adr.offset(-(1 as libc::c_int) as isize) as *mut WamWord
                        as *mut CodePtr),
                ),
            );
            func = ((*pred).f_n as PlULong
                & ((1 as libc::c_int as PlULong)
                    << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
            arity = ((*pred).f_n as PlULong
                >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
            Print_Bank_Name_Offset(
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                stack_name,
                adr
                    .offset_from(
                        (*pl_stk_tbl.as_mut_ptr().offset(3 as libc::c_int as isize))
                            .stack,
                    ) as libc::c_long as libc::c_int,
            );
            Pl_Stream_Printf(
                pstm_o,
                b": %s/%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*pl_atom_tbl.offset(func as isize)).name,
                arity,
            );
            if arity == 0 as libc::c_int
                && strcmp(
                    (*pl_atom_tbl.offset(func as isize)).name,
                    b"$clause_alt\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                Pl_Stream_Printf(
                    pstm_o,
                    b" for \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                Pl_Write_Term(
                    pstm_o,
                    -(1 as libc::c_int),
                    1200 as libc::c_int,
                    4 as libc::c_int | 8 as libc::c_int,
                    0 as *mut WamWord,
                    *(&mut *adr.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
                        as *mut WamWord),
                );
            }
            Pl_Stream_Printf(
                pstm_o,
                b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            adr = *(&mut *adr.offset(-(5 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
        }
        return 0 as libc::c_int;
    }
    if nb_read_arg == 1 as libc::c_int {
        adr = Detect_Stack(B, &mut stack_name);
        offset = B.offset_from(adr) as libc::c_long as libc::c_int;
        adr = B;
    } else {
        adr = Read_Bank_Adr(1 as libc::c_int, 1 as libc::c_int, &mut stack_name);
        if adr.is_null() {
            return 0 as libc::c_int;
        }
        offset = (if nb_read_arg < 3 as libc::c_int {
            0 as libc::c_int as libc::c_long
        } else {
            Read_An_Integer(2 as libc::c_int)
        }) as libc::c_int;
        adr = adr.offset(offset as isize);
    }
    pred = Detect_Pred_From_Code(
        ::std::mem::transmute::<
            CodePtr,
            *mut PlLong,
        >(
            *(&mut *adr.offset(-(1 as libc::c_int) as isize) as *mut WamWord
                as *mut CodePtr),
        ),
    );
    func = ((*pred).f_n as PlULong
        & ((1 as libc::c_int as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
    arity = ((*pred).f_n as PlULong
        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
    Pl_Stream_Printf(
        pstm_o,
        b"Created by  %s/%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*pl_atom_tbl.offset(func as isize)).name,
        arity,
    );
    if arity == 0 as libc::c_int
        && strcmp(
            (*pl_atom_tbl.offset(func as isize)).name,
            b"$clause_alt\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        Pl_Stream_Printf(
            pstm_o,
            b" for \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        Pl_Write_Term(
            pstm_o,
            -(1 as libc::c_int),
            1200 as libc::c_int,
            4 as libc::c_int | 8 as libc::c_int,
            0 as *mut WamWord,
            *(&mut *adr.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
                as *mut WamWord),
        );
    }
    Pl_Stream_Printf(
        pstm_o,
        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 8 as libc::c_int;
    while i > 0 as libc::c_int {
        Print_Bank_Name_Offset(
            choice_name[(i - 1 as libc::c_int) as usize],
            stack_name,
            offset - i,
        );
        Pl_Stream_Printf(
            pstm_o,
            b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        Print_Wam_Word(adr.offset(-(i as isize)));
        Pl_Stream_Printf(
            pstm_o,
            b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        i -= 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Read_Bank_Adr(
    mut only_stack: Bool,
    mut arg_nb: libc::c_int,
    mut bank_name: *mut *mut libc::c_char,
) -> *mut WamWord {
    let mut lg: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if nb_read_arg < arg_nb + 1 as libc::c_int {
        Pl_Stream_Printf(
            pstm_o,
            b"%s name expected\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            if only_stack != 0 {
                b"Stack\0" as *const u8 as *const libc::c_char
            } else {
                b"Bank\0" as *const u8 as *const libc::c_char
            },
        );
        return 0 as *mut WamWord;
    }
    lg = strlen((read_arg[arg_nb as usize]).as_mut_ptr()) as libc::c_int;
    if only_stack == 0 {
        if read_arg[arg_nb as usize][0 as libc::c_int as usize] as libc::c_int
            == 'x' as i32 && lg == 1 as libc::c_int
        {
            *bank_name = b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            return &mut *pl_reg_bank.offset(0 as libc::c_int as isize) as *mut WamWord;
        }
        if read_arg[arg_nb as usize][0 as libc::c_int as usize] as libc::c_int
            == 'y' as i32 && lg == 1 as libc::c_int
        {
            *bank_name = b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            return &mut *(*(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                .offset((-(4 as libc::c_int) - 0 as libc::c_int) as isize)
                as *mut WamWord;
        }
        if strncmp(
            b"ab\0" as *const u8 as *const libc::c_char,
            (read_arg[arg_nb as usize]).as_mut_ptr(),
            lg as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            *bank_name = b"ab\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            return &mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
                as *mut WamWord;
        }
        if strncmp(
            b"reg\0" as *const u8 as *const libc::c_char,
            (read_arg[arg_nb as usize]).as_mut_ptr(),
            lg as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            *bank_name = b"reg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            reg_copy[0 as libc::c_int as usize] = TR as WamWord;
            reg_copy[1 as libc::c_int as usize] = B as WamWord;
            reg_copy[2 as libc::c_int as usize] = H as WamWord;
            reg_copy[3 as libc::c_int
                as usize] = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize) as WamWord;
            reg_copy[4 as libc::c_int
                as usize] = ::std::mem::transmute::<
                WamCont,
                WamWord,
            >(
                *(pl_reg_bank as *mut WamCont)
                    .offset((256 as libc::c_int + 1 as libc::c_int) as isize),
            );
            reg_copy[5 as libc::c_int
                as usize] = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 2 as libc::c_int) as isize) as WamWord;
            reg_copy[6 as libc::c_int
                as usize] = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as WamWord;
            reg_copy[7 as libc::c_int
                as usize] = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 4 as libc::c_int) as isize) as WamWord;
            reg_copy[8 as libc::c_int
                as usize] = *pl_reg_bank
                .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
            reg_copy[9 as libc::c_int
                as usize] = *pl_reg_bank
                .offset((256 as libc::c_int + 6 as libc::c_int) as isize);
            reg_copy[10 as libc::c_int
                as usize] = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 7 as libc::c_int) as isize) as WamWord;
            return reg_copy.as_mut_ptr();
        }
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if strncmp(
            (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).name,
            (read_arg[arg_nb as usize]).as_mut_ptr(),
            lg as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            *bank_name = (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).name;
            return (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack;
        }
        i += 1;
        i;
    }
    Pl_Stream_Printf(
        pstm_o,
        b"Incorrect %s name\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        if only_stack != 0 {
            b"stack\0" as *const u8 as *const libc::c_char
        } else {
            b"bank\0" as *const u8 as *const libc::c_char
        },
    );
    return 0 as *mut WamWord;
}
unsafe extern "C" fn Read_An_Integer(mut arg_nb: libc::c_int) -> PlLong {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: PlLong = 0 as libc::c_int as PlLong;
    val = strtol((read_arg[arg_nb as usize]).as_mut_ptr(), &mut p, 0 as libc::c_int);
    if *p != 0 {
        Pl_Stream_Printf(
            pstm_o,
            b"Incorrect integer\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return val;
}
unsafe extern "C" fn Print_Bank_Name_Offset(
    mut prefix: *mut libc::c_char,
    mut bank_name: *mut libc::c_char,
    mut offset: libc::c_int,
) {
    let mut str: [libc::c_char; 80] = [0; 80];
    let mut lg: libc::c_int = strlen(prefix) as libc::c_int;
    if lg != 0 {
        Pl_Stream_Printf(
            pstm_o,
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            prefix,
        );
    }
    sprintf(
        str.as_mut_ptr(),
        b"%s[%d]\0" as *const u8 as *const libc::c_char,
        bank_name,
        offset,
    );
    lg = (lg as libc::c_ulong).wrapping_add(strlen(str.as_mut_ptr())) as libc::c_int
        as libc::c_int;
    if lg > 15 as libc::c_int {
        lg = 15 as libc::c_int;
    }
    Pl_Stream_Printf(
        pstm_o,
        b"%*s%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        15 as libc::c_int - lg,
        b"\0" as *const u8 as *const libc::c_char,
        str.as_mut_ptr(),
    );
}
unsafe extern "C" fn Print_Wam_Word(mut word_adr: *mut WamWord) {
    let mut word: WamWord = *word_adr;
    let mut tag: WamWord = 0;
    let mut value: WamWord = 0;
    let mut stack_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut functor: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    Pl_Stream_Printf(
        pstm_o,
        b"%#*lx  %*ld  \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        20 as libc::c_int,
        word,
        21 as libc::c_int,
        word,
    );
    adr = Detect_Stack(word as *mut WamWord, &mut stack_name);
    if !adr.is_null() {
        Print_Bank_Name_Offset(
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            stack_name,
            (word as *mut WamWord).offset_from(adr) as libc::c_long as libc::c_int,
        );
    } else {
        Pl_Stream_Printf(
            pstm_o,
            b"%*s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            b"?[?]\0" as *const u8 as *const libc::c_char,
        );
    }
    Pl_Stream_Printf(
        pstm_o,
        b"  \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    tag = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        if (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).value as libc::c_long == tag {
            break;
        }
        i += 1;
        i;
    }
    if i < 7 as libc::c_int {
        match (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).type_0 as libc::c_uint {
            0 => {
                value = word << 0 as libc::c_int >> 3 as libc::c_int;
                Pl_Stream_Printf(
                    pstm_o,
                    b"%s,%*ld\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).name,
                    15 as libc::c_int,
                    value,
                );
            }
            1 => {
                value = word << 0 as libc::c_int >> 3 as libc::c_int;
                if tag == 3 as libc::c_int as libc::c_long
                    && ((value as PlULong) < pl_max_atom
                        && !((*pl_atom_tbl.offset(value as isize)).name).is_null())
                    && !((*pl_atom_tbl.offset(value as isize)).name).is_null()
                {
                    Pl_Stream_Printf(
                        pstm_o,
                        b"ATM,%*s (%ld\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        15 as libc::c_int,
                        (*pl_atom_tbl.offset(value as isize)).name,
                        value,
                    );
                } else if tag == 3 as libc::c_int as libc::c_long {
                    tag = -(1 as libc::c_int) as WamWord;
                } else {
                    Pl_Stream_Printf(
                        pstm_o,
                        b"%s,%*lu\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).name,
                        15 as libc::c_int,
                        value,
                    );
                }
            }
            2 => {
                value = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord as WamWord;
                adr = Detect_Stack(value as *mut WamWord, &mut stack_name);
                if !adr.is_null() {
                    Pl_Stream_Printf(
                        pstm_o,
                        b"%s,\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).name,
                    );
                    Print_Bank_Name_Offset(
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        stack_name,
                        (value as *mut WamWord).offset_from(adr) as libc::c_long
                            as libc::c_int,
                    );
                } else {
                    tag = -(1 as libc::c_int) as WamWord;
                }
            }
            _ => {}
        }
    } else {
        tag = -(1 as libc::c_int) as WamWord;
    }
    if tag == -(1 as libc::c_int) as libc::c_long {
        Pl_Stream_Printf(
            pstm_o,
            b"???,%*s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            b"?\0" as *const u8 as *const libc::c_char,
        );
    }
    Pl_Stream_Printf(
        pstm_o,
        b"  \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if word_adr >= (*pl_stk_tbl.as_mut_ptr().offset(0 as libc::c_int as isize)).stack
        && word_adr
            < ((*pl_stk_tbl.as_mut_ptr().offset(0 as libc::c_int as isize)).stack)
                .offset(
                    (*pl_stk_tbl.as_mut_ptr().offset(0 as libc::c_int as isize)).size
                        as isize,
                )
    {
        tag = (word as PlULong & 0x3 as libc::c_int as libc::c_ulong) as WamWord;
        value = (word as PlULong & !(0x3 as libc::c_int) as libc::c_ulong) as WamWord;
        if tag == 3 as libc::c_int as libc::c_long {
            Pl_Stream_Printf(
                pstm_o,
                b"%s,%#*lx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                trail_tag_name[tag as usize],
                15 as libc::c_int,
                value,
            );
        } else if tag < 4 as libc::c_int as libc::c_long
            && {
                adr = Detect_Stack(value as *mut WamWord, &mut stack_name);
                !adr.is_null()
            } && *stack_name as libc::c_int != 't' as i32
        {
            Pl_Stream_Printf(
                pstm_o,
                b"%s,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                trail_tag_name[tag as usize],
            );
            Print_Bank_Name_Offset(
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                stack_name,
                (value as *mut WamWord).offset_from(adr) as libc::c_long as libc::c_int,
            );
        } else {
            Pl_Stream_Printf(
                pstm_o,
                b"???,%*s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                15 as libc::c_int,
                b"?\0" as *const u8 as *const libc::c_char,
            );
        }
        Pl_Stream_Printf(
            pstm_o,
            b"  \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    functor = (word as PlULong
        & ((1 as libc::c_int as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
    arity = (word as PlULong
        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
    if (functor as PlULong) < pl_max_atom
        && !((*pl_atom_tbl.offset(functor as isize)).name).is_null()
        && !((*pl_atom_tbl.offset(functor as isize)).name).is_null()
        && arity >= 0 as libc::c_int && arity <= 256 as libc::c_int - 1 as libc::c_int
    {
        Pl_Stream_Printf(
            pstm_o,
            b"%12s/%-3d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*pl_atom_tbl.offset(functor as isize)).name,
            arity,
        );
    }
}
unsafe extern "C" fn Modify_Wam_Word(mut word_adr: *mut WamWord) {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut bank_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut offset: libc::c_int = 0;
    let mut str: [libc::c_char; 80] = [0; 80];
    let mut comma: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    loop {
        Pl_Stream_Printf(
            pstm_o,
            b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if (Pl_Stream_Gets_Prompt(
            b"New value: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            pstm_o,
            str.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int,
            pstm_i,
        ))
            .is_null() || *str.as_mut_ptr() as libc::c_int == '\0' as i32
            || *str.as_mut_ptr() as libc::c_int == '\n' as i32
        {
            break;
        }
        Scan_Command(str.as_mut_ptr());
        comma = strchr(str.as_mut_ptr(), ',' as i32);
        if !comma.is_null() {
            i = 0 as libc::c_int;
            while i < 7 as libc::c_int {
                if strcmp(
                    (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).name,
                    (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
                ) == 0 as libc::c_int
                {
                    break;
                }
                i += 1;
                i;
            }
            if i < 7 as libc::c_int {
                match (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).type_0
                    as libc::c_uint
                {
                    0 => {
                        current_block = 12444791199452447874;
                        match current_block {
                            3201577399599230911 => {
                                adr = Read_Bank_Adr(
                                    1 as libc::c_int,
                                    1 as libc::c_int,
                                    &mut bank_name,
                                );
                                if !adr.is_null() {
                                    offset = (if nb_read_arg < 3 as libc::c_int {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        Read_An_Integer(2 as libc::c_int)
                                    }) as libc::c_int;
                                    *word_adr = adr.offset(offset as isize) as PlLong
                                        + (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).tag_mask;
                                    return;
                                }
                            }
                            4156078072197177645 => {
                                word = strtol(
                                    (read_arg[1 as libc::c_int as usize]).as_mut_ptr(),
                                    &mut p,
                                    0 as libc::c_int,
                                );
                                if *p as libc::c_int == '\0' as i32 {
                                    j = Read_An_Integer(1 as libc::c_int) as libc::c_int;
                                    current_block = 4488286894823169796;
                                } else if strcmp(
                                    (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
                                    b"ATM\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    j = Pl_Create_Allocate_Atom(
                                        comma.offset(1 as libc::c_int as isize),
                                    );
                                    current_block = 4488286894823169796;
                                } else {
                                    current_block = 3053566293396475224;
                                }
                                match current_block {
                                    3053566293396475224 => {}
                                    _ => {
                                        *word_adr = ((j as PlLong) << 3 as libc::c_int)
                                            + (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).tag_mask;
                                        return;
                                    }
                                }
                            }
                            _ => {
                                word = strtol(
                                    (read_arg[1 as libc::c_int as usize]).as_mut_ptr(),
                                    &mut p,
                                    0 as libc::c_int,
                                );
                                if !(*p as libc::c_int != '\0' as i32) {
                                    *word_adr = Read_An_Integer(1 as libc::c_int)
                                        << 3 as libc::c_int >> 0 as libc::c_int
                                        | (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).tag_mask;
                                    return;
                                }
                            }
                        }
                        current_block = 3053566293396475224;
                    }
                    1 => {
                        current_block = 4156078072197177645;
                        match current_block {
                            3201577399599230911 => {
                                adr = Read_Bank_Adr(
                                    1 as libc::c_int,
                                    1 as libc::c_int,
                                    &mut bank_name,
                                );
                                if !adr.is_null() {
                                    offset = (if nb_read_arg < 3 as libc::c_int {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        Read_An_Integer(2 as libc::c_int)
                                    }) as libc::c_int;
                                    *word_adr = adr.offset(offset as isize) as PlLong
                                        + (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).tag_mask;
                                    return;
                                }
                            }
                            4156078072197177645 => {
                                word = strtol(
                                    (read_arg[1 as libc::c_int as usize]).as_mut_ptr(),
                                    &mut p,
                                    0 as libc::c_int,
                                );
                                if *p as libc::c_int == '\0' as i32 {
                                    j = Read_An_Integer(1 as libc::c_int) as libc::c_int;
                                    current_block = 4488286894823169796;
                                } else if strcmp(
                                    (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
                                    b"ATM\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    j = Pl_Create_Allocate_Atom(
                                        comma.offset(1 as libc::c_int as isize),
                                    );
                                    current_block = 4488286894823169796;
                                } else {
                                    current_block = 3053566293396475224;
                                }
                                match current_block {
                                    3053566293396475224 => {}
                                    _ => {
                                        *word_adr = ((j as PlLong) << 3 as libc::c_int)
                                            + (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).tag_mask;
                                        return;
                                    }
                                }
                            }
                            _ => {
                                word = strtol(
                                    (read_arg[1 as libc::c_int as usize]).as_mut_ptr(),
                                    &mut p,
                                    0 as libc::c_int,
                                );
                                if !(*p as libc::c_int != '\0' as i32) {
                                    *word_adr = Read_An_Integer(1 as libc::c_int)
                                        << 3 as libc::c_int >> 0 as libc::c_int
                                        | (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).tag_mask;
                                    return;
                                }
                            }
                        }
                        current_block = 3053566293396475224;
                    }
                    2 => {
                        current_block = 3201577399599230911;
                        match current_block {
                            3201577399599230911 => {
                                adr = Read_Bank_Adr(
                                    1 as libc::c_int,
                                    1 as libc::c_int,
                                    &mut bank_name,
                                );
                                if !adr.is_null() {
                                    offset = (if nb_read_arg < 3 as libc::c_int {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        Read_An_Integer(2 as libc::c_int)
                                    }) as libc::c_int;
                                    *word_adr = adr.offset(offset as isize) as PlLong
                                        + (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).tag_mask;
                                    return;
                                }
                            }
                            4156078072197177645 => {
                                word = strtol(
                                    (read_arg[1 as libc::c_int as usize]).as_mut_ptr(),
                                    &mut p,
                                    0 as libc::c_int,
                                );
                                if *p as libc::c_int == '\0' as i32 {
                                    j = Read_An_Integer(1 as libc::c_int) as libc::c_int;
                                    current_block = 4488286894823169796;
                                } else if strcmp(
                                    (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
                                    b"ATM\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    j = Pl_Create_Allocate_Atom(
                                        comma.offset(1 as libc::c_int as isize),
                                    );
                                    current_block = 4488286894823169796;
                                } else {
                                    current_block = 3053566293396475224;
                                }
                                match current_block {
                                    3053566293396475224 => {}
                                    _ => {
                                        *word_adr = ((j as PlLong) << 3 as libc::c_int)
                                            + (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).tag_mask;
                                        return;
                                    }
                                }
                            }
                            _ => {
                                word = strtol(
                                    (read_arg[1 as libc::c_int as usize]).as_mut_ptr(),
                                    &mut p,
                                    0 as libc::c_int,
                                );
                                if !(*p as libc::c_int != '\0' as i32) {
                                    *word_adr = Read_An_Integer(1 as libc::c_int)
                                        << 3 as libc::c_int >> 0 as libc::c_int
                                        | (*pl_tag_tbl.as_mut_ptr().offset(i as isize)).tag_mask;
                                    return;
                                }
                            }
                        }
                        current_block = 3053566293396475224;
                    }
                    _ => {
                        current_block = 2604890879466389055;
                    }
                }
            } else {
                current_block = 2604890879466389055;
            }
            match current_block {
                3053566293396475224 => {}
                _ => {
                    i = 0 as libc::c_int;
                    while i < 4 as libc::c_int {
                        if strcmp(
                            trail_tag_name[i as usize],
                            (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
                        ) == 0 as libc::c_int
                        {
                            adr = Read_Bank_Adr(
                                1 as libc::c_int,
                                1 as libc::c_int,
                                &mut bank_name,
                            );
                            if !adr.is_null() {
                                offset = (if nb_read_arg < 3 as libc::c_int {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    Read_An_Integer(2 as libc::c_int)
                                }) as libc::c_int;
                                *word_adr = (adr.offset(offset as isize) as PlULong
                                    | i as libc::c_ulong) as WamWord;
                                return;
                            }
                        }
                        i += 1;
                        i;
                    }
                }
            }
        } else {
            slash = strchr(
                (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
                '/' as i32,
            );
            if !slash.is_null() {
                *slash = '\0' as i32 as libc::c_char;
                i = strtol(
                    slash.offset(1 as libc::c_int as isize),
                    &mut p,
                    0 as libc::c_int,
                ) as libc::c_int;
                if !(*p as libc::c_int != '\0' as i32 || i < 1 as libc::c_int
                    || i > 256 as libc::c_int - 1 as libc::c_int)
                {
                    word = strtol(
                        (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
                        &mut p,
                        0 as libc::c_int,
                    );
                    if *p as libc::c_int != '\0' as i32 {
                        word = Pl_Create_Allocate_Atom(
                            (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
                        ) as PlLong;
                        current_block = 1924505913685386279;
                    } else if !((word as PlULong) < pl_max_atom
                        && !((*pl_atom_tbl.offset(word as isize)).name).is_null())
                    {
                        current_block = 3053566293396475224;
                    } else {
                        current_block = 1924505913685386279;
                    }
                    match current_block {
                        3053566293396475224 => {}
                        _ => {
                            *word_adr = ((i as PlULong)
                                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                                | word as libc::c_ulong) as WamWord;
                            Pl_Stream_Printf(
                                pstm_o,
                                b"--> %s/%d\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                (*pl_atom_tbl.offset(word as isize)).name,
                                i,
                            );
                            return;
                        }
                    }
                }
            } else if nb_read_arg == 1 as libc::c_int
                && *(read_arg[0 as libc::c_int as usize]).as_mut_ptr() as libc::c_int
                    >= '0' as i32
                && *(read_arg[0 as libc::c_int as usize]).as_mut_ptr() as libc::c_int
                    <= '9' as i32
            {
                word = strtol(
                    (read_arg[0 as libc::c_int as usize]).as_mut_ptr(),
                    &mut p,
                    0 as libc::c_int,
                );
                if *p as libc::c_int == '\0' as i32 {
                    *word_adr = word;
                    return;
                }
            } else {
                adr = Read_Bank_Adr(1 as libc::c_int, 0 as libc::c_int, &mut bank_name);
                if !adr.is_null() {
                    offset = (if nb_read_arg < 2 as libc::c_int {
                        0 as libc::c_int as libc::c_long
                    } else {
                        Read_An_Integer(1 as libc::c_int)
                    }) as libc::c_int;
                    *word_adr = adr.offset(offset as isize) as WamWord;
                    return;
                }
            }
        }
        Pl_Stream_Printf(
            pstm_o,
            b"Error...\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    };
}
unsafe extern "C" fn Detect_Stack(
    mut adr: *mut WamWord,
    mut stack_name: *mut *mut libc::c_char,
) -> *mut WamWord {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if adr >= (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack
            && adr
                < ((*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack)
                    .offset((*pl_stk_tbl.as_mut_ptr().offset(i as isize)).size as isize)
        {
            *stack_name = (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).name;
            return (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).stack;
        }
        i += 1;
        i;
    }
    return 0 as *mut WamWord;
}
unsafe extern "C" fn Detect_Pred_From_Code(mut codep: *mut PlLong) -> *mut PredInf {
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut last_pred: *mut PredInf = 0 as *mut PredInf;
    let mut dist: PlLong = 0 as libc::c_int as PlLong;
    let mut d: PlLong = 0;
    pred = Pl_Hash_First(pl_pred_tbl, &mut scan) as *mut PredInf;
    while !pred.is_null() {
        d = codep.offset_from((*pred).codep) as libc::c_long;
        if !((*pred).prop & 2 as libc::c_int != 0
            || d < 0 as libc::c_int as libc::c_long)
        {
            if last_pred.is_null() || d < dist {
                last_pred = pred;
                dist = d;
            }
        }
        pred = Pl_Hash_Next(&mut scan) as *mut PredInf;
    }
    return last_pred;
}
unsafe extern "C" fn Help() -> Bool {
    let mut i: libc::c_int = 0;
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Wam debugging options:\0" as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   write     A [N] write   N (or 1) Prolog terms starting at A\0" as *const u8
            as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   data      A [N] display N (or 1) words starting at A\0" as *const u8
            as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   modify    A [N] display and modify N (or 1) words starting at A\0"
            as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   where     A     display the real address corresponding to SA\0" as *const u8
            as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   what      RA    display what corresponds to the real address RA\0"
            as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   deref     A     display the dereferenced word starting at A\0" as *const u8
            as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   envir     [SA]  display an environment located at SA (or current)\0"
            as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   backtrack [SA]  display a choice point located at SA (or current)\0"
            as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   backtrack all   display all choice points\0" as *const u8
            as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   quit            return to Prolog debugger\0" as *const u8
            as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"A WAM address (A) has the following syntax: bank_name [N]\0" as *const u8
            as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   bank_name  is either reg/x/y/ab/stack_name (see below)\0" as *const u8
            as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"   N          is an optional index (default 0)\0" as *const u8
            as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"   stack_name is either:\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        Pl_Stream_Printf(
            pstm_o,
            b" %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*pl_stk_tbl.as_mut_ptr().offset(i as isize)).name,
        );
        i += 1;
        i;
    }
    Pl_Stream_Printf(
        pstm_o,
        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"A WAM stack address (SA) has the following syntax: stack_name [N]\0"
            as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    Pl_Stream_Printf(
        pstm_o,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"A real address (RA) is a C integer (0x... notation is allowed)\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    cmd = [
        {
            let mut init = InfCmd {
                name: b"write\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> Bool>,
                    FctPtr,
                >(Some(Write_Data_Modify as unsafe extern "C" fn() -> Bool)),
            };
            init
        },
        {
            let mut init = InfCmd {
                name: b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> Bool>,
                    FctPtr,
                >(Some(Write_Data_Modify as unsafe extern "C" fn() -> Bool)),
            };
            init
        },
        {
            let mut init = InfCmd {
                name: b"modify\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> Bool>,
                    FctPtr,
                >(Some(Write_Data_Modify as unsafe extern "C" fn() -> Bool)),
            };
            init
        },
        {
            let mut init = InfCmd {
                name: b"where\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> Bool>,
                    FctPtr,
                >(Some(Where as unsafe extern "C" fn() -> Bool)),
            };
            init
        },
        {
            let mut init = InfCmd {
                name: b"what\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> Bool>,
                    FctPtr,
                >(Some(What as unsafe extern "C" fn() -> Bool)),
            };
            init
        },
        {
            let mut init = InfCmd {
                name: b"deref\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> Bool>,
                    FctPtr,
                >(Some(Dereference as unsafe extern "C" fn() -> Bool)),
            };
            init
        },
        {
            let mut init = InfCmd {
                name: b"envir\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> Bool>,
                    FctPtr,
                >(Some(Environment as unsafe extern "C" fn() -> Bool)),
            };
            init
        },
        {
            let mut init = InfCmd {
                name: b"backtrack\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> Bool>,
                    FctPtr,
                >(Some(Backtrack as unsafe extern "C" fn() -> Bool)),
            };
            init
        },
        {
            let mut init = InfCmd {
                name: b"quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    libc::intptr_t,
                    FctPtr,
                >(-(1 as libc::c_int) as libc::intptr_t),
            };
            init
        },
        {
            let mut init = InfCmd {
                name: b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                fct: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> Bool>,
                    FctPtr,
                >(Some(Help as unsafe extern "C" fn() -> Bool)),
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
