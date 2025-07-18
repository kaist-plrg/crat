use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    static mut pl_atom_end_of_file: libc::c_int;
    fn Pl_Rd_Char_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Code_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Byte_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Check_For_Un_Integer(start_word: WamWord);
    fn Pl_Check_For_Un_In_Char(start_word: WamWord);
    fn Pl_Check_For_Un_In_Code(start_word: WamWord);
    fn Pl_Check_For_Un_In_Byte(start_word: WamWord);
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_last_input_sora: WamWord;
    static mut pl_last_output_sora: WamWord;
    static mut pl_stm_input: libc::c_int;
    static mut pl_stm_output: libc::c_int;
    static mut pl_representation_character: libc::c_int;
    fn Pl_Err_Representation(atom_flag: libc::c_int);
    fn Pl_Check_Stream_Type(stm: libc::c_int, check_text: Bool, for_input: Bool);
    fn Pl_Get_Stream_Or_Alias(sora_word: WamWord, test_mask: libc::c_int) -> libc::c_int;
    fn Pl_Stream_Get_Key(
        pstm: *mut StmInf,
        echo: Bool,
        catch_ctrl_c: Bool,
    ) -> libc::c_int;
    fn Pl_Stream_Getc(pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_Ungetc(c: libc::c_int, pstm: *mut StmInf);
    fn Pl_Stream_Peekc(pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_Putc(c: libc::c_int, pstm: *mut StmInf);
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Get_Key_2(
    mut sora_word: WamWord,
    mut code_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut c: libc::c_int = 0;
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
    Pl_Check_For_Un_Integer(code_word);
    c = Pl_Stream_Get_Key(
        *pl_stm_tbl.offset(stm as isize),
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if c == -(1 as libc::c_int) {
        c = -(1 as libc::c_int);
    }
    return Pl_Get_Integer(c as PlLong, code_word);
}
pub unsafe extern "C" fn Pl_Get_Key_1(mut code_word: WamWord) -> Bool {
    return Pl_Get_Key_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        code_word,
    );
}
pub unsafe extern "C" fn Pl_Get_Key_No_Echo_2(
    mut sora_word: WamWord,
    mut code_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut c: libc::c_int = 0;
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
    Pl_Check_For_Un_Integer(code_word);
    c = Pl_Stream_Get_Key(
        *pl_stm_tbl.offset(stm as isize),
        0 as libc::c_int,
        1 as libc::c_int,
    );
    if c == -(1 as libc::c_int) {
        c = -(1 as libc::c_int);
    }
    return Pl_Get_Integer(c as PlLong, code_word);
}
pub unsafe extern "C" fn Pl_Get_Key_No_Echo_1(mut code_word: WamWord) -> Bool {
    return Pl_Get_Key_No_Echo_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        code_word,
    );
}
pub unsafe extern "C" fn Pl_Get_Char_2(
    mut sora_word: WamWord,
    mut char_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut c: libc::c_int = 0;
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
    Pl_Check_For_Un_In_Char(char_word);
    c = Pl_Stream_Getc(*pl_stm_tbl.offset(stm as isize));
    if c != -(1 as libc::c_int)
        && !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_character);
    }
    return Pl_Get_Atom(
        if c == -(1 as libc::c_int) {
            pl_atom_end_of_file
        } else {
            c as libc::c_uchar as libc::c_int
        },
        char_word,
    );
}
pub unsafe extern "C" fn Pl_Get_Char_1(mut char_word: WamWord) -> Bool {
    return Pl_Get_Char_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        char_word,
    );
}
pub unsafe extern "C" fn Pl_Get_Code_2(
    mut sora_word: WamWord,
    mut code_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut c: libc::c_int = 0;
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
    Pl_Check_For_Un_In_Code(code_word);
    c = Pl_Stream_Getc(*pl_stm_tbl.offset(stm as isize));
    if c != -(1 as libc::c_int)
        && !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_character);
    }
    if c == -(1 as libc::c_int) {
        c = -(1 as libc::c_int);
    }
    return Pl_Get_Integer(c as PlLong, code_word);
}
pub unsafe extern "C" fn Pl_Get_Code_1(mut code_word: WamWord) -> Bool {
    return Pl_Get_Code_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        code_word,
    );
}
pub unsafe extern "C" fn Pl_Get_Byte_2(
    mut sora_word: WamWord,
    mut byte_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_input
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int)
    };
    pl_last_input_sora = sora_word;
    Pl_Check_Stream_Type(stm, 0 as libc::c_int, 1 as libc::c_int);
    Pl_Check_For_Un_In_Byte(byte_word);
    c = Pl_Stream_Getc(*pl_stm_tbl.offset(stm as isize));
    if c == -(1 as libc::c_int) {
        c = -(1 as libc::c_int);
    }
    return Pl_Get_Integer(c as PlLong, byte_word);
}
pub unsafe extern "C" fn Pl_Get_Byte_1(mut byte_word: WamWord) -> Bool {
    return Pl_Get_Byte_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        byte_word,
    );
}
pub unsafe extern "C" fn Pl_Unget_Char_2(
    mut sora_word: WamWord,
    mut char_word: WamWord,
) {
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
    Pl_Stream_Ungetc(Pl_Rd_Char_Check(char_word), *pl_stm_tbl.offset(stm as isize));
}
pub unsafe extern "C" fn Pl_Unget_Char_1(mut char_word: WamWord) {
    Pl_Unget_Char_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        char_word,
    );
}
pub unsafe extern "C" fn Pl_Unget_Code_2(
    mut sora_word: WamWord,
    mut code_word: WamWord,
) {
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
    Pl_Stream_Ungetc(Pl_Rd_Code_Check(code_word), *pl_stm_tbl.offset(stm as isize));
}
pub unsafe extern "C" fn Pl_Unget_Code_1(mut code_word: WamWord) {
    Pl_Unget_Code_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        code_word,
    );
}
pub unsafe extern "C" fn Pl_Unget_Byte_2(
    mut sora_word: WamWord,
    mut byte_word: WamWord,
) {
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
    Pl_Check_Stream_Type(stm, 0 as libc::c_int, 1 as libc::c_int);
    Pl_Stream_Ungetc(Pl_Rd_Byte_Check(byte_word), *pl_stm_tbl.offset(stm as isize));
}
pub unsafe extern "C" fn Pl_Unget_Byte_1(mut byte_word: WamWord) {
    Pl_Unget_Byte_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        byte_word,
    );
}
pub unsafe extern "C" fn Pl_Peek_Char_2(
    mut sora_word: WamWord,
    mut char_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut c: libc::c_int = 0;
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
    Pl_Check_For_Un_In_Char(char_word);
    c = Pl_Stream_Peekc(*pl_stm_tbl.offset(stm as isize));
    if c != -(1 as libc::c_int)
        && !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_character);
    }
    return Pl_Get_Atom(
        if c == -(1 as libc::c_int) {
            pl_atom_end_of_file
        } else {
            c as libc::c_uchar as libc::c_int
        },
        char_word,
    );
}
pub unsafe extern "C" fn Pl_Peek_Char_1(mut char_word: WamWord) -> Bool {
    return Pl_Peek_Char_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        char_word,
    );
}
pub unsafe extern "C" fn Pl_Peek_Code_2(
    mut sora_word: WamWord,
    mut code_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut c: libc::c_int = 0;
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
    Pl_Check_For_Un_In_Code(code_word);
    c = Pl_Stream_Peekc(*pl_stm_tbl.offset(stm as isize));
    if c != -(1 as libc::c_int)
        && !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_character);
    }
    if c == -(1 as libc::c_int) {
        c = -(1 as libc::c_int);
    }
    return Pl_Get_Integer(c as PlLong, code_word);
}
pub unsafe extern "C" fn Pl_Peek_Code_1(mut code_word: WamWord) -> Bool {
    return Pl_Peek_Code_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        code_word,
    );
}
pub unsafe extern "C" fn Pl_Peek_Byte_2(
    mut sora_word: WamWord,
    mut byte_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_input
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int)
    };
    pl_last_input_sora = sora_word;
    Pl_Check_Stream_Type(stm, 0 as libc::c_int, 1 as libc::c_int);
    Pl_Check_For_Un_In_Byte(byte_word);
    c = Pl_Stream_Peekc(*pl_stm_tbl.offset(stm as isize));
    if c == -(1 as libc::c_int) {
        c = -(1 as libc::c_int);
    }
    return Pl_Get_Integer(c as PlLong, byte_word);
}
pub unsafe extern "C" fn Pl_Peek_Byte_1(mut byte_word: WamWord) -> Bool {
    return Pl_Peek_Byte_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        byte_word,
    );
}
pub unsafe extern "C" fn Pl_Put_Char_2(mut sora_word: WamWord, mut char_word: WamWord) {
    let mut stm: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_output
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 3 as libc::c_int)
    };
    pl_last_output_sora = sora_word;
    Pl_Check_Stream_Type(stm, 1 as libc::c_int, 0 as libc::c_int);
    Pl_Stream_Putc(Pl_Rd_Char_Check(char_word), *pl_stm_tbl.offset(stm as isize));
}
pub unsafe extern "C" fn Pl_Put_Char_1(mut char_word: WamWord) {
    Pl_Put_Char_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        char_word,
    );
}
pub unsafe extern "C" fn Pl_Put_Code_2(mut sora_word: WamWord, mut code_word: WamWord) {
    let mut stm: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_output
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 3 as libc::c_int)
    };
    pl_last_output_sora = sora_word;
    Pl_Check_Stream_Type(stm, 1 as libc::c_int, 0 as libc::c_int);
    Pl_Stream_Putc(Pl_Rd_Code_Check(code_word), *pl_stm_tbl.offset(stm as isize));
}
pub unsafe extern "C" fn Pl_Put_Code_1(mut code_word: WamWord) {
    Pl_Put_Code_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        code_word,
    );
}
pub unsafe extern "C" fn Pl_Put_Byte_2(mut sora_word: WamWord, mut byte_word: WamWord) {
    let mut stm: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_output
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 3 as libc::c_int)
    };
    pl_last_output_sora = sora_word;
    Pl_Check_Stream_Type(stm, 0 as libc::c_int, 0 as libc::c_int);
    Pl_Stream_Putc(Pl_Rd_Byte_Check(byte_word), *pl_stm_tbl.offset(stm as isize));
}
pub unsafe extern "C" fn Pl_Put_Byte_1(mut byte_word: WamWord) {
    Pl_Put_Byte_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        byte_word,
    );
}
