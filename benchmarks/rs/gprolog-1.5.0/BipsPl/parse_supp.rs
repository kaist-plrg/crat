use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: libc::c_int) -> libc::c_int;
    fn siglongjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_atom_curly_brackets: libc::c_int;
    static mut pl_atom_end_of_file: libc::c_int;
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Lookup_Oper(atom_op: libc::c_int, type_0: libc::c_int) -> *mut OperInf;
    fn Pl_Put_X_Variable() -> WamWord;
    fn Pl_Put_Atom(atom: libc::c_int) -> WamWord;
    fn Pl_Put_Integer(n: PlLong) -> WamWord;
    fn Pl_Put_Float(n: libc::c_double) -> WamWord;
    fn Pl_Put_List() -> WamWord;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    static mut pl_flag_double_quotes: *mut FlagInf;
    static mut pl_flag_back_quotes: *mut FlagInf;
    fn Pl_Set_Last_Syntax_Error(
        file_name: *mut libc::c_char,
        err_line: libc::c_int,
        err_col: libc::c_int,
        err_msg: *mut libc::c_char,
    );
    static mut pl_token: TokInf;
    fn Pl_Scan_Peek_Char(pstm: *mut StmInf, convert: Bool) -> libc::c_int;
    fn Pl_Scan_Token(pstm: *mut StmInf, comma_is_punct: Bool) -> *mut libc::c_char;
    fn Pl_Recover_After_Error(pstm: *mut StmInf);
    fn Pl_Scan_Next_Atom(pstm: *mut StmInf) -> *mut libc::c_char;
    fn Pl_Scan_Next_Number(pstm: *mut StmInf, integer_only: Bool) -> *mut libc::c_char;
    static mut pl_use_le_prompt: libc::c_int;
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub type WamWordP = *mut WamWord;
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
pub struct OperInf {
    pub a_t: PlLong,
    pub prec: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
}
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
pub type TypTok = libc::c_uint;
pub const TOKEN_EXTENDED: TypTok = 10;
pub const TOKEN_END_OF_FILE: TypTok = 9;
pub const TOKEN_FULL_STOP: TypTok = 8;
pub const TOKEN_IMMEDIAT_OPEN: TypTok = 7;
pub const TOKEN_PUNCTUATION: TypTok = 6;
pub const TOKEN_BACK_QUOTED: TypTok = 5;
pub const TOKEN_STRING: TypTok = 4;
pub const TOKEN_NAME: TypTok = 3;
pub const TOKEN_FLOAT: TypTok = 2;
pub const TOKEN_INTEGER: TypTok = 1;
pub const TOKEN_VARIABLE: TypTok = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokInf {
    pub type_0: TypTok,
    pub name: [libc::c_char; 10240],
    pub quoted: libc::c_int,
    pub punct: libc::c_int,
    pub int_num: PlLong,
    pub float_num: libc::c_double,
    pub line: libc::c_int,
    pub col: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfVar {
    pub name: [libc::c_char; 1024],
    pub word: WamWord,
    pub named: Bool,
    pub nb_of_uses: libc::c_int,
}
pub type FlagInf = flag_inf;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_parse_dico_var: [InfVar; 32768] = [InfVar {
    name: [0; 1024],
    word: 0,
    named: 0,
    nb_of_uses: 0,
}; 32768];
pub static mut pl_parse_nb_var: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut pstm_i: *mut StmInf = 0 as *const StmInf as *mut StmInf;
static mut tok_present: Bool = 0;
static mut unget_tok: TokInf = TokInf {
    type_0: TOKEN_VARIABLE,
    name: [0; 10240],
    quoted: 0,
    punct: 0,
    int_num: 0,
    float_num: 0.,
    line: 0,
    col: 0,
};
static mut jumper: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
static mut buff_save_machine_regs: [WamWord; 4] = [0; 4];
static mut atom_var: libc::c_int = 0;
static mut atom_string: libc::c_int = 0;
static mut atom_punct: libc::c_int = 0;
static mut atom_back_quotes: libc::c_int = 0;
static mut atom_full_stop: libc::c_int = 0;
static mut atom_extend: libc::c_int = 0;
unsafe extern "C" fn Read_Next_Token(mut comma_is_punct: Bool) {
    let mut err_msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if tok_present != 0 {
        tok_present = 0 as libc::c_int;
        pl_token = unget_tok;
        if comma_is_punct != 0
            && pl_token.type_0 as libc::c_uint
                == TOKEN_NAME as libc::c_int as libc::c_uint && pl_token.quoted == 0
            && pl_token.name[0 as libc::c_int as usize] as libc::c_int == ',' as i32
            && pl_token.name[1 as libc::c_int as usize] as libc::c_int == '\0' as i32
        {
            pl_token.type_0 = TOKEN_PUNCTUATION;
            pl_token.punct = ',' as i32;
        } else if comma_is_punct == 0
            && pl_token.type_0 as libc::c_uint
                == TOKEN_PUNCTUATION as libc::c_int as libc::c_uint
            && pl_token.punct == ',' as i32
        {
            pl_token.type_0 = TOKEN_NAME;
            pl_token.quoted = 0 as libc::c_int;
            pl_token.name[0 as libc::c_int as usize] = ',' as i32 as libc::c_char;
            pl_token.name[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }
    } else {
        err_msg = Pl_Scan_Token(pstm_i, comma_is_punct);
        if !err_msg.is_null() {
            Parse_Error(err_msg);
        }
    };
}
pub unsafe extern "C" fn Pl_Read_Term(
    mut pstm: *mut StmInf,
    mut parse_end_of_term: libc::c_int,
) -> WamWord {
    let mut current_block: u64;
    let mut jmp_val: libc::c_int = 0;
    let mut term: WamWord = 0;
    let mut save_use_le_prompt: libc::c_int = pl_use_le_prompt;
    pl_use_le_prompt = 1 as libc::c_int;
    pl_parse_nb_var = 0 as libc::c_int;
    pl_last_read_line = -(1 as libc::c_int);
    pstm_i = pstm;
    tok_present = 0 as libc::c_int;
    buff_save_machine_regs[0 as libc::c_int as usize] = pl_reg_bank as WamWord;
    buff_save_machine_regs[1 as libc::c_int as usize] = TR as WamWord;
    buff_save_machine_regs[2 as libc::c_int as usize] = B as WamWord;
    buff_save_machine_regs[3 as libc::c_int as usize] = H as WamWord;
    jmp_val = __sigsetjmp(jumper.as_mut_ptr(), 1 as libc::c_int);
    pl_reg_bank = buff_save_machine_regs[0 as libc::c_int as usize] as WamWordP;
    TR = buff_save_machine_regs[1 as libc::c_int as usize] as WamWordP;
    B = buff_save_machine_regs[2 as libc::c_int as usize] as WamWordP;
    H = buff_save_machine_regs[3 as libc::c_int as usize] as WamWordP;
    if jmp_val == 0 as libc::c_int {
        term = Parse_Term(1200 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
        Read_Next_Token(0 as libc::c_int);
        if term as libc::c_ulong
            == (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong)
        {
            if pl_token.type_0 as libc::c_uint
                == TOKEN_END_OF_FILE as libc::c_int as libc::c_uint
            {
                term = Pl_Put_Atom(pl_atom_end_of_file);
                current_block = 1162829850052840842;
            } else {
                Parse_Error(
                    b"expression expected\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                current_block = 26972500619410423;
            }
        } else {
            current_block = 26972500619410423;
        }
        match current_block {
            1162829850052840842 => {}
            _ => {
                if parse_end_of_term == 0 as libc::c_int {
                    if pl_token.type_0 as libc::c_uint
                        == TOKEN_FULL_STOP as libc::c_int as libc::c_uint
                    {
                        current_block = 1162829850052840842;
                    } else {
                        Parse_Error(
                            b". or operator expected after expression\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        current_block = 13472856163611868459;
                    }
                } else {
                    current_block = 13472856163611868459;
                }
                match current_block {
                    1162829850052840842 => {}
                    _ => {
                        if !(pl_token.type_0 as libc::c_uint
                            == TOKEN_END_OF_FILE as libc::c_int as libc::c_uint)
                        {
                            Parse_Error(
                                b"eof or operator expected after expression\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                    }
                }
            }
        }
    } else {
        pl_reg_bank = buff_save_machine_regs[0 as libc::c_int as usize] as WamWordP;
        TR = buff_save_machine_regs[1 as libc::c_int as usize] as WamWordP;
        B = buff_save_machine_regs[2 as libc::c_int as usize] as WamWordP;
        H = buff_save_machine_regs[3 as libc::c_int as usize] as WamWordP;
        term = (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    pl_use_le_prompt = save_use_le_prompt;
    return term;
}
unsafe extern "C" fn Parse_Term(
    mut cur_prec: libc::c_int,
    mut context: libc::c_int,
    mut comma_is_punct: Bool,
) -> WamWord {
    let mut current_block: u64;
    let mut bracket: Bool = 0;
    let mut atom: libc::c_int = 0;
    let mut oper: *mut OperInf = 0 as *mut OperInf;
    let mut infix_op: Bool = 0;
    let mut cur_left: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut flag_value: libc::c_int = 0;
    let mut term: WamWord = 0;
    let mut term1: WamWord = 0;
    let mut w: [WamWord; 2] = [0; 2];
    let mut left_is_op: Bool = 0 as libc::c_int;
    Read_Next_Token((context != 1 as libc::c_int || comma_is_punct != 0) as libc::c_int);
    if pl_last_read_line == -(1 as libc::c_int) {
        pl_last_read_line = pl_token.line;
        pl_last_read_col = pl_token.col;
    }
    match pl_token.type_0 as libc::c_uint {
        0 => {
            i = Lookup_In_Dico_Var((pl_token.name).as_mut_ptr());
            pl_parse_dico_var[i as usize].nb_of_uses += 1;
            if pl_parse_dico_var[i as usize].nb_of_uses == 1 as libc::c_int {
                term = Pl_Put_X_Variable();
                pl_parse_dico_var[i as usize].word = term;
            } else {
                term = pl_parse_dico_var[i as usize].word;
            }
            current_block = 7189308829251266000;
        }
        1 => {
            if pl_token.int_num
                > ((1 as libc::c_int as PlLong)
                    << 8 as libc::c_int * 8 as libc::c_int - 3 as libc::c_int
                        - 1 as libc::c_int) - 1 as libc::c_int as libc::c_long
            {
                Parse_Error(
                    b"integer overflow (exceeds max_integer)\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            if pl_token.int_num
                < -(((1 as libc::c_int as PlLong)
                    << 8 as libc::c_int * 8 as libc::c_int - 3 as libc::c_int
                        - 1 as libc::c_int) - 1 as libc::c_int as libc::c_long)
                    - 1 as libc::c_int as libc::c_long
            {
                Parse_Error(
                    b"integer underflow (exceeds min_integer)\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            term = Pl_Put_Integer(pl_token.int_num);
            current_block = 7189308829251266000;
        }
        2 => {
            term = Pl_Put_Float(pl_token.float_num);
            current_block = 7189308829251266000;
        }
        4 | 5 => {
            flag_value = (if pl_token.type_0 as libc::c_uint
                == TOKEN_STRING as libc::c_int as libc::c_uint
            {
                (*pl_flag_double_quotes).value
            } else {
                (*pl_flag_back_quotes).value
            }) as libc::c_int;
            flag_value &= ((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int;
            if flag_value == 2 as libc::c_int {
                atom = Pl_Create_Allocate_Atom((pl_token.name).as_mut_ptr());
                current_block = 9939965761344058454;
            } else {
                i = strlen((pl_token.name).as_mut_ptr()) as libc::c_int;
                term = (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
                loop {
                    let fresh0 = i;
                    i = i - 1;
                    if !(fresh0 != 0) {
                        break;
                    }
                    term1 = Pl_Put_List();
                    if flag_value == 0 as libc::c_int {
                        Pl_Unify_Integer(pl_token.name[i as usize] as PlLong);
                    } else {
                        Pl_Unify_Atom(
                            pl_token.name[i as usize] as libc::c_uchar as libc::c_int,
                        );
                    }
                    Pl_Unify_Value(term);
                    term = term1;
                }
                current_block = 7189308829251266000;
            }
        }
        7 => {
            pl_token.punct = '(' as i32;
            current_block = 1271714645385924685;
        }
        6 => {
            current_block = 1271714645385924685;
        }
        3 => {
            atom = Pl_Create_Allocate_Atom((pl_token.name).as_mut_ptr());
            current_block = 9939965761344058454;
        }
        _ => {
            term = (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
            current_block = 7727112833634905465;
        }
    }
    match current_block {
        1271714645385924685 => {
            if (strchr(b"({[\0" as *const u8 as *const libc::c_char, pl_token.punct))
                .is_null()
            {
                term = (0 as libc::c_int as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                current_block = 7727112833634905465;
            } else {
                atom = if pl_token.punct == '{' as i32 {
                    pl_atom_curly_brackets
                } else {
                    256 as libc::c_int
                };
                term = Parse_Bracketed_Term();
                if term as libc::c_ulong
                    == (0 as libc::c_int as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong)
                {
                    current_block = 9939965761344058454;
                } else {
                    current_block = 7189308829251266000;
                }
            }
        }
        _ => {}
    }
    match current_block {
        9939965761344058454 => {
            bracket = (Pl_Scan_Peek_Char(pstm_i, 1 as libc::c_int) == '(' as i32)
                as libc::c_int;
            if bracket != 0 {
                term = Parse_Args_Of_Functor(atom);
            } else {
                if pl_token.name[0 as libc::c_int as usize] as libc::c_int == '-' as i32
                    && pl_token.name[1 as libc::c_int as usize] as libc::c_int
                        == '\0' as i32
                {
                    let mut save_line: libc::c_int = pl_token.line;
                    let mut save_col: libc::c_int = pl_token.col;
                    Read_Next_Token(0 as libc::c_int);
                    if pl_token.type_0 as libc::c_uint
                        == TOKEN_INTEGER as libc::c_int as libc::c_uint
                    {
                        if pl_token.int_num
                            > -(-(((1 as libc::c_int as PlLong)
                                << 8 as libc::c_int * 8 as libc::c_int - 3 as libc::c_int
                                    - 1 as libc::c_int) - 1 as libc::c_int as libc::c_long)
                                - 1 as libc::c_int as libc::c_long)
                        {
                            Parse_Error(
                                b"integer underflow (exceeds min_integer)\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                        term = Pl_Put_Integer(-pl_token.int_num);
                        current_block = 7189308829251266000;
                    } else if pl_token.type_0 as libc::c_uint
                        == TOKEN_FLOAT as libc::c_int as libc::c_uint
                    {
                        term = Pl_Put_Float(-pl_token.float_num);
                        current_block = 7189308829251266000;
                    } else {
                        tok_present = 1 as libc::c_int;
                        unget_tok = pl_token;
                        pl_token.type_0 = TOKEN_NAME;
                        strcpy(
                            (pl_token.name).as_mut_ptr(),
                            b"-\0" as *const u8 as *const libc::c_char,
                        );
                        pl_token.line = save_line;
                        pl_token.col = save_col;
                        current_block = 12829669402821218572;
                    }
                } else {
                    current_block = 12829669402821218572;
                }
                match current_block {
                    7189308829251266000 => {}
                    _ => {
                        oper = Pl_Lookup_Oper(atom, 0 as libc::c_int);
                        if !oper.is_null() && cur_prec >= (*oper).prec {
                            cur_left = (*oper).prec;
                            term = Parse_Term(
                                (*oper).right,
                                1 as libc::c_int,
                                comma_is_punct,
                            );
                            if term as libc::c_ulong
                                != (0 as libc::c_int as PlLong as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as PlULong)
                            {
                                term = Create_Structure(atom, 1 as libc::c_int, &mut term);
                                current_block = 7189308829251266000;
                            } else {
                                if context != 0 as libc::c_int {
                                    Parse_Error(
                                        b"expression expected or previous operator needs brackets\0"
                                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    );
                                }
                                current_block = 9925100494328262799;
                            }
                        } else {
                            current_block = 9925100494328262799;
                        }
                        match current_block {
                            7189308829251266000 => {}
                            _ => {
                                left_is_op = ((*pl_atom_tbl.offset(atom as isize)).prop)
                                    .op_mask() as Bool;
                                if left_is_op != 0 && context != 0 as libc::c_int {
                                    if ((*pl_atom_tbl.offset(atom as isize)).prop).op_mask()
                                        as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0
                                        || ((*pl_atom_tbl.offset(atom as isize)).prop).op_mask()
                                            as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int != 0
                                    {
                                        Parse_Error(
                                            b"current or previous operator needs brackets\0"
                                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                                        );
                                    } else {
                                        Parse_Error(
                                            b"current operator needs brackets\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                        );
                                    }
                                }
                                term = Pl_Put_Atom(atom);
                            }
                        }
                    }
                }
            }
            current_block = 7189308829251266000;
        }
        _ => {}
    }
    match current_block {
        7189308829251266000 => {
            loop {
                Read_Next_Token(comma_is_punct);
                if pl_token.type_0 as libc::c_uint
                    == TOKEN_PUNCTUATION as libc::c_int as libc::c_uint
                    && pl_token.punct == '|' as i32
                    && {
                        atom = '|' as i32 as libc::c_uchar as libc::c_int;
                        oper = Pl_Lookup_Oper(atom, 2 as libc::c_int);
                        !oper.is_null()
                    } && (*oper).prec > 1000 as libc::c_int && cur_prec >= (*oper).prec
                {
                    infix_op = 1 as libc::c_int;
                } else {
                    if pl_token.type_0 as libc::c_uint
                        != TOKEN_NAME as libc::c_int as libc::c_uint
                    {
                        break;
                    }
                    atom = Pl_Create_Allocate_Atom((pl_token.name).as_mut_ptr());
                    oper = Pl_Lookup_Oper(atom, 2 as libc::c_int);
                    if !oper.is_null() {
                        infix_op = 1 as libc::c_int;
                    } else {
                        oper = Pl_Lookup_Oper(atom, 1 as libc::c_int);
                        if oper.is_null() {
                            break;
                        }
                        infix_op = 0 as libc::c_int;
                    }
                }
                if left_is_op != 0 {
                    Parse_Error(
                        b"previous operator needs brackets\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                if cur_prec < (*oper).prec || cur_left > (*oper).left {
                    break;
                }
                if infix_op != 0 {
                    w[0 as libc::c_int as usize] = term;
                    w[1 as libc::c_int
                        as usize] = Parse_Term(
                        (*oper).right,
                        2 as libc::c_int,
                        comma_is_punct,
                    );
                    if w[1 as libc::c_int as usize] as libc::c_ulong
                        == (0 as libc::c_int as PlLong as libc::c_ulong)
                            .wrapping_add(0 as libc::c_int as PlULong)
                    {
                        Parse_Error(
                            b"right operand expected for infix operator\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    if atom == '.' as i32 as libc::c_uchar as libc::c_int {
                        term = Pl_Put_List();
                        Pl_Unify_Value(w[0 as libc::c_int as usize]);
                        Pl_Unify_Value(w[1 as libc::c_int as usize]);
                    } else {
                        term = Create_Structure(atom, 2 as libc::c_int, w.as_mut_ptr());
                    }
                } else {
                    term = Create_Structure(atom, 1 as libc::c_int, &mut term);
                }
                cur_left = (*oper).prec;
            }
        }
        _ => {}
    }
    tok_present = 1 as libc::c_int;
    unget_tok = pl_token;
    return term;
}
unsafe extern "C" fn Parse_Args_Of_Functor(mut atom: libc::c_int) -> WamWord {
    let mut w: [WamWord; 256] = [0; 256];
    let mut i: libc::c_int = 0;
    Read_Next_Token(0 as libc::c_int);
    i = 0 as libc::c_int;
    loop {
        if i >= 256 as libc::c_int - 1 as libc::c_int {
            Parse_Error(
                b"too big compound term (exceeds max_arity)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        w[i
            as usize] = Parse_Term(
            999 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        let fresh1 = i;
        i = i + 1;
        if w[fresh1 as usize] as libc::c_ulong
            == (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong)
        {
            Parse_Error(
                b"expression expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        Read_Next_Token(1 as libc::c_int);
        if !(pl_token.type_0 as libc::c_uint
            == TOKEN_PUNCTUATION as libc::c_int as libc::c_uint
            && pl_token.punct == ',' as i32)
        {
            break;
        }
    }
    if pl_token.type_0 as libc::c_uint
        != TOKEN_PUNCTUATION as libc::c_int as libc::c_uint
        || pl_token.punct != ')' as i32
    {
        Parse_Error(
            b", or ) expected\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return Create_Structure(atom, i, w.as_mut_ptr());
}
unsafe extern "C" fn Parse_Bracketed_Term() -> WamWord {
    let mut term: WamWord = 0 as libc::c_int as WamWord;
    match pl_token.punct {
        40 => {
            term = Parse_Term(1200 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
            if term as libc::c_ulong
                == (0 as libc::c_int as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong)
            {
                Parse_Error(
                    b"expression expected\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            Read_Next_Token(0 as libc::c_int);
            if pl_token.type_0 as libc::c_uint
                != TOKEN_PUNCTUATION as libc::c_int as libc::c_uint
                || pl_token.punct != ')' as i32
            {
                Parse_Error(
                    b") or operator expected\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        123 => {
            term = Parse_Term(1200 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
            Read_Next_Token(0 as libc::c_int);
            if pl_token.type_0 as libc::c_uint
                != TOKEN_PUNCTUATION as libc::c_int as libc::c_uint
                || pl_token.punct != '}' as i32
            {
                Parse_Error(
                    b"} or operator expected\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            if term as libc::c_ulong
                != (0 as libc::c_int as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong)
            {
                term = Create_Structure(
                    pl_atom_curly_brackets,
                    1 as libc::c_int,
                    &mut term,
                );
            }
        }
        91 => {
            term = Parse_List(1 as libc::c_int);
        }
        _ => {}
    }
    return term;
}
unsafe extern "C" fn Parse_List(mut can_be_empty: Bool) -> WamWord {
    let mut term: WamWord = 0;
    let mut car_word: WamWord = 0;
    let mut cdr_word: WamWord = 0 as libc::c_int as WamWord;
    car_word = Parse_Term(999 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
    Read_Next_Token(1 as libc::c_int);
    if car_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        if can_be_empty == 0 {
            Parse_Error(
                b"expression expected in list\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else if pl_token.type_0 as libc::c_uint
            != TOKEN_PUNCTUATION as libc::c_int as libc::c_uint
            || pl_token.punct != ']' as i32
        {
            Parse_Error(
                b"expression or ] expected in list\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        return (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    if pl_token.type_0 as libc::c_uint
        != TOKEN_PUNCTUATION as libc::c_int as libc::c_uint
        || (strchr(b",|]\0" as *const u8 as *const libc::c_char, pl_token.punct))
            .is_null()
    {
        Parse_Error(
            b", | ] or operator expected in list\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    match pl_token.punct {
        44 => {
            cdr_word = Parse_List(0 as libc::c_int);
        }
        124 => {
            cdr_word = Parse_Term(
                999 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            if cdr_word as libc::c_ulong
                == (0 as libc::c_int as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong)
            {
                Parse_Error(
                    b"expression expected in list\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            Read_Next_Token(1 as libc::c_int);
            if pl_token.type_0 as libc::c_uint
                != TOKEN_PUNCTUATION as libc::c_int as libc::c_uint
                || pl_token.punct != ']' as i32
            {
                Parse_Error(
                    b"] or operator expected in list\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        93 => {
            cdr_word = (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
        }
        _ => {}
    }
    term = Pl_Put_List();
    Pl_Unify_Value(car_word);
    Pl_Unify_Value(cdr_word);
    return term;
}
unsafe extern "C" fn Create_Structure(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg: *mut WamWord,
) -> WamWord {
    let mut res_word: WamWord = 0;
    let mut i: libc::c_int = 0;
    if arity == 2 as libc::c_int && func == '.' as i32 as libc::c_uchar as libc::c_int {
        res_word = Pl_Put_List();
        Pl_Unify_Value(*arg.offset(0 as libc::c_int as isize));
        Pl_Unify_Value(*arg.offset(1 as libc::c_int as isize));
    } else {
        res_word = Pl_Put_Structure(func, arity);
        i = 0 as libc::c_int;
        while i < arity {
            Pl_Unify_Value(*arg.offset(i as isize));
            i += 1;
            i;
        }
    }
    return res_word;
}
unsafe extern "C" fn Lookup_In_Dico_Var(mut name: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut named: Bool = 0;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int != '_' as i32
        || *name.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        named = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < pl_parse_nb_var
            && strcmp(name, (pl_parse_dico_var[i as usize].name).as_mut_ptr())
                != 0 as libc::c_int
        {
            i += 1;
            i;
        }
    } else {
        named = 0 as libc::c_int;
        i = pl_parse_nb_var;
    }
    if i == pl_parse_nb_var {
        if pl_parse_nb_var >= 32768 as libc::c_int {
            Parse_Error(
                b"too many variables in a term\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        strcpy((pl_parse_dico_var[pl_parse_nb_var as usize].name).as_mut_ptr(), name);
        pl_parse_dico_var[pl_parse_nb_var as usize]
            .word = (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        pl_parse_dico_var[pl_parse_nb_var as usize].named = named;
        pl_parse_dico_var[pl_parse_nb_var as usize].nb_of_uses = 0 as libc::c_int;
        pl_parse_nb_var += 1;
        pl_parse_nb_var;
    }
    return i;
}
unsafe extern "C" fn Parse_Error(mut err_msg: *mut libc::c_char) {
    Pl_Set_Last_Syntax_Error(
        (*pl_atom_tbl.offset((*pstm_i).atom_file_name as isize)).name,
        pl_token.line,
        pl_token.col,
        err_msg,
    );
    if pl_token.type_0 as libc::c_uint != TOKEN_FULL_STOP as libc::c_int as libc::c_uint
    {
        Pl_Recover_After_Error(pstm_i);
    }
    buff_save_machine_regs[0 as libc::c_int as usize] = pl_reg_bank as WamWord;
    buff_save_machine_regs[1 as libc::c_int as usize] = TR as WamWord;
    buff_save_machine_regs[2 as libc::c_int as usize] = B as WamWord;
    buff_save_machine_regs[3 as libc::c_int as usize] = H as WamWord;
    siglongjmp(jumper.as_mut_ptr(), 1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Read_Atom(mut pstm: *mut StmInf) -> WamWord {
    let mut err_msg: *mut libc::c_char = 0 as *mut libc::c_char;
    err_msg = Pl_Scan_Next_Atom(pstm);
    if !err_msg.is_null() {
        Pl_Set_Last_Syntax_Error(
            (*pl_atom_tbl.offset((*pstm).atom_file_name as isize)).name,
            pl_token.line,
            pl_token.col,
            err_msg,
        );
        return (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    pl_last_read_line = pl_token.line;
    pl_last_read_col = pl_token.col;
    return Pl_Put_Atom(Pl_Create_Allocate_Atom((pl_token.name).as_mut_ptr()));
}
pub unsafe extern "C" fn Pl_Read_Integer(mut pstm: *mut StmInf) -> WamWord {
    let mut err_msg: *mut libc::c_char = 0 as *mut libc::c_char;
    err_msg = Pl_Scan_Next_Number(pstm, 1 as libc::c_int);
    if !err_msg.is_null() {
        Pl_Set_Last_Syntax_Error(
            (*pl_atom_tbl.offset((*pstm).atom_file_name as isize)).name,
            pl_token.line,
            pl_token.col,
            err_msg,
        );
        return (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    pl_last_read_line = pl_token.line;
    pl_last_read_col = pl_token.col;
    return Pl_Put_Integer(pl_token.int_num);
}
pub unsafe extern "C" fn Pl_Read_Number(mut pstm: *mut StmInf) -> WamWord {
    let mut err_msg: *mut libc::c_char = 0 as *mut libc::c_char;
    err_msg = Pl_Scan_Next_Number(pstm, 0 as libc::c_int);
    if !err_msg.is_null() {
        Pl_Set_Last_Syntax_Error(
            (*pl_atom_tbl.offset((*pstm).atom_file_name as isize)).name,
            pl_token.line,
            pl_token.col,
            err_msg,
        );
        return (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    pl_last_read_line = pl_token.line;
    pl_last_read_col = pl_token.col;
    return if pl_token.type_0 as libc::c_uint
        == TOKEN_INTEGER as libc::c_int as libc::c_uint
    {
        Pl_Put_Integer(pl_token.int_num)
    } else {
        Pl_Put_Float(pl_token.float_num)
    };
}
pub unsafe extern "C" fn Pl_Read_Token(mut pstm: *mut StmInf) -> WamWord {
    let mut current_block: u64;
    let mut term: WamWord = 0;
    let mut arg: WamWord = 0 as libc::c_int as WamWord;
    let mut func: libc::c_int = 0 as libc::c_int;
    let mut atom: libc::c_int = 0;
    let mut err_msg: *mut libc::c_char = 0 as *mut libc::c_char;
    err_msg = Pl_Scan_Token(pstm, 0 as libc::c_int);
    if !err_msg.is_null() {
        Pl_Set_Last_Syntax_Error(
            (*pl_atom_tbl.offset((*pstm).atom_file_name as isize)).name,
            pl_token.line,
            pl_token.col,
            err_msg,
        );
        return (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    pl_last_read_line = pl_token.line;
    pl_last_read_col = pl_token.col;
    term = (0 as libc::c_int as PlLong as libc::c_ulong)
        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    match pl_token.type_0 as libc::c_uint {
        0 => {
            func = atom_var;
            current_block = 14503086552496295514;
        }
        1 => {
            term = Pl_Put_Integer(pl_token.int_num);
            current_block = 14763689060501151050;
        }
        2 => {
            term = Pl_Put_Float(pl_token.float_num);
            current_block = 14763689060501151050;
        }
        4 => {
            func = atom_string;
            current_block = 14503086552496295514;
        }
        7 => {
            pl_token.punct = '(' as i32;
            current_block = 3356221950756584563;
        }
        6 => {
            current_block = 3356221950756584563;
        }
        3 => {
            atom = Pl_Create_Allocate_Atom((pl_token.name).as_mut_ptr());
            term = Pl_Put_Atom(atom);
            current_block = 14763689060501151050;
        }
        5 => {
            func = atom_back_quotes;
            current_block = 14503086552496295514;
        }
        8 => {
            func = atom_punct;
            arg = Pl_Put_Atom(atom_full_stop);
            current_block = 14763689060501151050;
        }
        9 => {
            func = atom_punct;
            arg = Pl_Put_Atom(pl_atom_end_of_file);
            current_block = 14763689060501151050;
        }
        10 => {
            func = atom_extend;
            current_block = 14503086552496295514;
        }
        _ => {
            current_block = 14763689060501151050;
        }
    }
    match current_block {
        3356221950756584563 => {
            func = atom_punct;
            atom = pl_token.punct as libc::c_uchar as libc::c_int;
            arg = Pl_Put_Atom(atom);
        }
        14503086552496295514 => {
            atom = Pl_Create_Allocate_Atom((pl_token.name).as_mut_ptr());
            arg = Pl_Put_Atom(atom);
        }
        _ => {}
    }
    if term as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        term = Pl_Put_Structure(func, 1 as libc::c_int);
        Pl_Unify_Value(arg);
    }
    return term;
}
