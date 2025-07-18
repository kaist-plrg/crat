use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    static mut pl_char_conv: [libc::c_char; 0];
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Rd_Char_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Check_For_Un_Integer(start_word: WamWord);
    fn Pl_Check_For_Un_Number(start_word: WamWord);
    fn Pl_Check_For_Un_Atom(start_word: WamWord);
    fn Pl_Check_For_Un_Char(start_word: WamWord);
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Char_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    static mut pl_glob_dico_var: [PlLong; 0];
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_last_input_sora: WamWord;
    static mut pl_stm_input: libc::c_int;
    fn Pl_Syntax_Error(flag_value: libc::c_int);
    static mut pl_parse_dico_var: [InfVar; 0];
    static mut pl_parse_nb_var: libc::c_int;
    fn Pl_Read_Term(pstm: *mut StmInf, parse_end_of_term: libc::c_int) -> WamWord;
    fn Pl_Read_Atom(pstm: *mut StmInf) -> WamWord;
    fn Pl_Read_Integer(pstm: *mut StmInf) -> WamWord;
    fn Pl_Read_Number(pstm: *mut StmInf) -> WamWord;
    fn Pl_Read_Token(pstm: *mut StmInf) -> WamWord;
    static mut pl_sys_var: [PlLong; 0];
    static mut pl_flag_syntax_error: *mut FlagInf;
    fn Pl_Get_Stream_Or_Alias(sora_word: WamWord, test_mask: libc::c_int) -> libc::c_int;
    fn Pl_Check_Stream_Type(stm: libc::c_int, check_text: Bool, for_input: Bool);
    fn X1_2463757272656E745F636861725F636F6E76657273696F6E5F616C74__a0();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamWordP = *mut WamWord;
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
pub struct InfVar {
    pub name: [libc::c_char; 1024],
    pub word: WamWord,
    pub named: Bool,
    pub nb_of_uses: libc::c_int,
}
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Read_Term_5(
    mut sora_word: WamWord,
    mut term_word: WamWord,
    mut vars_word: WamWord,
    mut var_names_word: WamWord,
    mut sing_names_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut stm: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut parse_end_of_term: libc::c_int = (*pl_sys_var
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) >> 3 as libc::c_int
        & 1 as libc::c_int as libc::c_long) as libc::c_int;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_input
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int)
    };
    pl_last_input_sora = sora_word;
    Pl_Check_Stream_Type(stm, 1 as libc::c_int, 1 as libc::c_int);
    word = Pl_Read_Term(*pl_stm_tbl.offset(stm as isize), parse_end_of_term);
    if word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        Pl_Syntax_Error(
            (if *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
                < 0 as libc::c_int as libc::c_long
            {
                (*pl_flag_syntax_error).value
            } else {
                *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
            }) as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if Pl_Unify(word, term_word) == 0 {
        return 0 as libc::c_int;
    }
    if *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
        & 1 as libc::c_int as libc::c_long != 0
    {
        i = 0 as libc::c_int;
        while i < pl_parse_nb_var {
            if Pl_Get_List(vars_word) == 0
                || Pl_Unify_Value(
                    (*pl_parse_dico_var.as_mut_ptr().offset(i as isize)).word,
                ) == 0
            {
                return 0 as libc::c_int;
            }
            vars_word = Pl_Unify_Variable();
            i += 1;
            i;
        }
        if Pl_Get_Nil(vars_word) == 0 {
            return 0 as libc::c_int;
        }
    }
    if *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
        & 2 as libc::c_int as libc::c_long != 0
    {
        i = 0 as libc::c_int;
        while i < pl_parse_nb_var {
            if !((*pl_parse_dico_var.as_mut_ptr().offset(i as isize)).named == 0) {
                *pl_glob_dico_var
                    .as_mut_ptr()
                    .offset(
                        i as isize,
                    ) = Pl_Create_Allocate_Atom(
                    ((*pl_parse_dico_var.as_mut_ptr().offset(i as isize)).name)
                        .as_mut_ptr(),
                ) as PlLong;
                word = Pl_Put_Structure(
                    '=' as i32 as libc::c_uchar as libc::c_int,
                    2 as libc::c_int,
                );
                Pl_Unify_Atom(
                    *pl_glob_dico_var.as_mut_ptr().offset(i as isize) as libc::c_int,
                );
                Pl_Unify_Value(
                    (*pl_parse_dico_var.as_mut_ptr().offset(i as isize)).word,
                );
                if Pl_Get_List(var_names_word) == 0 || Pl_Unify_Value(word) == 0 {
                    return 0 as libc::c_int;
                }
                var_names_word = Pl_Unify_Variable();
            }
            i += 1;
            i;
        }
        if Pl_Get_Nil(var_names_word) == 0 {
            return 0 as libc::c_int;
        }
    }
    if *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
        & 4 as libc::c_int as libc::c_long != 0
    {
        i = 0 as libc::c_int;
        while i < pl_parse_nb_var {
            if !((*pl_parse_dico_var.as_mut_ptr().offset(i as isize)).named == 0
                || (*pl_parse_dico_var.as_mut_ptr().offset(i as isize)).nb_of_uses
                    > 1 as libc::c_int)
            {
                if *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
                    & 2 as libc::c_int as libc::c_long
                    == 0 as libc::c_int as libc::c_long
                {
                    *pl_glob_dico_var
                        .as_mut_ptr()
                        .offset(
                            i as isize,
                        ) = Pl_Create_Allocate_Atom(
                        ((*pl_parse_dico_var.as_mut_ptr().offset(i as isize)).name)
                            .as_mut_ptr(),
                    ) as PlLong;
                }
                word = Pl_Put_Structure(
                    '=' as i32 as libc::c_uchar as libc::c_int,
                    2 as libc::c_int,
                );
                Pl_Unify_Atom(
                    *pl_glob_dico_var.as_mut_ptr().offset(i as isize) as libc::c_int,
                );
                Pl_Unify_Value(
                    (*pl_parse_dico_var.as_mut_ptr().offset(i as isize)).word,
                );
                if Pl_Get_List(sing_names_word) == 0 || Pl_Unify_Value(word) == 0 {
                    return 0 as libc::c_int;
                }
                sing_names_word = Pl_Unify_Variable();
            }
            i += 1;
            i;
        }
        if Pl_Get_Nil(sing_names_word) == 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Read_Term_4(
    mut term_word: WamWord,
    mut vars_word: WamWord,
    mut var_names_word: WamWord,
    mut sing_names_word: WamWord,
) -> Bool {
    return Pl_Read_Term_5(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        term_word,
        vars_word,
        var_names_word,
        sing_names_word,
    );
}
pub unsafe extern "C" fn Pl_Read_1(mut term_word: WamWord) -> Bool {
    return Pl_Read_Term_5(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        term_word,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
    );
}
pub unsafe extern "C" fn Pl_Read_2(
    mut sora_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    return Pl_Read_Term_5(
        sora_word,
        term_word,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
    );
}
pub unsafe extern "C" fn Pl_Read_Atom_2(
    mut sora_word: WamWord,
    mut atom_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut stm: libc::c_int = 0;
    Pl_Check_For_Un_Atom(atom_word);
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_input
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int)
    };
    pl_last_input_sora = sora_word;
    Pl_Check_Stream_Type(stm, 1 as libc::c_int, 1 as libc::c_int);
    word = Pl_Read_Atom(*pl_stm_tbl.offset(stm as isize));
    if word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        Pl_Syntax_Error(
            (if *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
                < 0 as libc::c_int as libc::c_long
            {
                (*pl_flag_syntax_error).value
            } else {
                *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
            }) as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if Pl_Unify(word, atom_word) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Read_Atom_1(mut atom_word: WamWord) -> Bool {
    return Pl_Read_Atom_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        atom_word,
    );
}
pub unsafe extern "C" fn Pl_Read_Integer_2(
    mut sora_word: WamWord,
    mut integer_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut stm: libc::c_int = 0;
    Pl_Check_For_Un_Integer(integer_word);
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_input
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int)
    };
    pl_last_input_sora = sora_word;
    Pl_Check_Stream_Type(stm, 1 as libc::c_int, 1 as libc::c_int);
    word = Pl_Read_Integer(*pl_stm_tbl.offset(stm as isize));
    if word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        Pl_Syntax_Error(
            (if *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
                < 0 as libc::c_int as libc::c_long
            {
                (*pl_flag_syntax_error).value
            } else {
                *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
            }) as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if Pl_Unify(word, integer_word) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Read_Integer_1(mut integer_word: WamWord) -> Bool {
    return Pl_Read_Integer_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        integer_word,
    );
}
pub unsafe extern "C" fn Pl_Read_Number_2(
    mut sora_word: WamWord,
    mut number_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut stm: libc::c_int = 0;
    Pl_Check_For_Un_Number(number_word);
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_input
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int)
    };
    pl_last_input_sora = sora_word;
    Pl_Check_Stream_Type(stm, 1 as libc::c_int, 1 as libc::c_int);
    word = Pl_Read_Number(*pl_stm_tbl.offset(stm as isize));
    if word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        Pl_Syntax_Error(
            (if *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
                < 0 as libc::c_int as libc::c_long
            {
                (*pl_flag_syntax_error).value
            } else {
                *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
            }) as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if Pl_Unify(word, number_word) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Read_Number_1(mut number_word: WamWord) -> Bool {
    return Pl_Read_Number_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        number_word,
    );
}
pub unsafe extern "C" fn Pl_Read_Token_2(
    mut sora_word: WamWord,
    mut token_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut stm: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_input
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int)
    };
    pl_last_input_sora = sora_word;
    Pl_Check_Stream_Type(stm, 1 as libc::c_int, 1 as libc::c_int);
    word = Pl_Read_Token(*pl_stm_tbl.offset(stm as isize));
    if word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        Pl_Syntax_Error(
            (if *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
                < 0 as libc::c_int as libc::c_long
            {
                (*pl_flag_syntax_error).value
            } else {
                *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize)
            }) as libc::c_int,
        );
        return 0 as libc::c_int;
    }
    if Pl_Unify(word, token_word) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Read_Token_1(mut token_word: WamWord) -> Bool {
    return Pl_Read_Token_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        token_word,
    );
}
pub unsafe extern "C" fn Pl_Last_Read_Start_Line_Column_2(
    mut line_word: WamWord,
    mut col_word: WamWord,
) -> Bool {
    return (Pl_Un_Integer_Check(pl_last_read_line as PlLong, line_word) != 0
        && Pl_Un_Integer_Check(pl_last_read_col as PlLong, col_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Char_Conversion_2(
    mut in_char_word: WamWord,
    mut out_char_word: WamWord,
) {
    let mut c_in: libc::c_int = 0;
    let mut c_out: libc::c_int = 0;
    c_in = Pl_Rd_Char_Check(in_char_word);
    c_out = Pl_Rd_Char_Check(out_char_word);
    *pl_char_conv.as_mut_ptr().offset(c_in as isize) = c_out as libc::c_char;
}
pub unsafe extern "C" fn Pl_Current_Char_Conversion_2(
    mut in_char_word: WamWord,
    mut out_char_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c_in: libc::c_int = 0;
    let mut c_out: libc::c_int = 0;
    let mut c_in1: libc::c_int = 0;
    let mut c_out1: libc::c_int = 0;
    Pl_Check_For_Un_Char(out_char_word);
    let mut deref_last_word: WamWord = 0;
    word = in_char_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        c_in = Pl_Rd_Char_Check(word);
        c_out = *pl_char_conv.as_mut_ptr().offset(c_in as isize) as libc::c_int;
        return (c_in != c_out && Pl_Un_Char_Check(c_out, out_char_word) != 0)
            as libc::c_int;
    }
    c_in = -(1 as libc::c_int);
    loop {
        c_in += 1;
        if !(c_in < 256 as libc::c_int) {
            break;
        }
        c_out = *pl_char_conv.as_mut_ptr().offset(c_in as isize) as libc::c_int;
        if c_in != c_out {
            break;
        }
    }
    if c_in >= 256 as libc::c_int {
        return 0 as libc::c_int;
    }
    c_in1 = c_in;
    loop {
        c_in1 += 1;
        if !(c_in1 < 256 as libc::c_int) {
            break;
        }
        c_out1 = *pl_char_conv.as_mut_ptr().offset(c_in1 as isize) as libc::c_int;
        if c_in1 != c_out1 {
            break;
        }
    }
    if c_in1 < 256 as libc::c_int {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = in_char_word;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = out_char_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = c_in1 as WamWord;
        *pl_reg_bank.offset(3 as libc::c_int as isize) = c_out1 as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2463757272656E745F636861725F636F6E76657273696F6E5F616C74__a0),
                ),
            ),
            4 as libc::c_int,
        );
    }
    return (Pl_Get_Atom(c_in as libc::c_uchar as libc::c_int, in_char_word) != 0
        && Pl_Get_Atom(c_out as libc::c_uchar as libc::c_int, out_char_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Current_Char_Conversion_Alt_0() -> Bool {
    let mut in_char_word: WamWord = 0;
    let mut out_char_word: WamWord = 0;
    let mut c_in: libc::c_int = 0;
    let mut c_out: libc::c_int = 0;
    let mut c_in1: libc::c_int = 0;
    let mut c_out1: libc::c_int = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F636861725F636F6E76657273696F6E5F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    in_char_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    out_char_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    c_in = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    c_out = *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    c_in1 = c_in;
    loop {
        c_in1 += 1;
        if !(c_in1 < 256 as libc::c_int) {
            break;
        }
        c_out1 = *pl_char_conv.as_mut_ptr().offset(c_in1 as isize) as libc::c_int;
        if c_in1 != c_out1 {
            break;
        }
    }
    if c_in1 >= 256 as libc::c_int {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    } else {
        *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
            as *mut WamWord) = c_in1 as WamWord;
        *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
            as *mut WamWord) = c_out1 as WamWord;
    }
    return (Pl_Get_Atom(c_in as libc::c_uchar as libc::c_int, in_char_word) != 0
        && Pl_Get_Atom(c_out as libc::c_uchar as libc::c_int, out_char_word) != 0)
        as libc::c_int;
}
