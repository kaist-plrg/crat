use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    static mut pl_glob_buff: [libc::c_char; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_atom_true: libc::c_int;
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Positive_Check(start_word: WamWord) -> PlLong;
    fn Pl_Check_For_Un_List(start_word: WamWord);
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    static mut pl_glob_dico_var: [PlLong; 0];
    fn Pl_Treat_Vars_Of_Term(
        start_word: WamWord,
        generic_var: Bool,
        fct: Option::<unsafe extern "C" fn() -> Bool>,
    ) -> Bool;
    fn Pl_Acyclic_Term_1(start_word: WamWord) -> Bool;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_last_output_sora: WamWord;
    static mut pl_stm_output: libc::c_int;
    fn Pl_Get_Stream_Or_Alias(sora_word: WamWord, test_mask: libc::c_int) -> libc::c_int;
    fn Pl_Check_Stream_Type(stm: libc::c_int, check_text: Bool, for_input: Bool);
    fn Pl_Stream_Putc(c: libc::c_int, pstm: *mut StmInf);
    fn Pl_Stream_Puts(str: *mut libc::c_char, pstm: *mut StmInf) -> libc::c_int;
    static mut pl_type_list: libc::c_int;
    static mut pl_representation_too_many_variables: libc::c_int;
    fn Pl_Err_Instantiation();
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Representation(atom_flag: libc::c_int);
    static mut pl_last_writing: libc::c_int;
    fn Pl_Write_Term(
        pstm: *mut StmInf,
        depth: libc::c_int,
        prec: libc::c_int,
        mask: libc::c_int,
        above_H_0: *mut WamWord,
        term_word: WamWord,
    );
    fn Pl_Write_A_Full_Stop(pstm: *mut StmInf);
    static mut pl_sys_var: [PlLong; 0];
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
static mut atom_clause: libc::c_int = 0;
static mut atom_dcg: libc::c_int = 0;
static mut atom_if: libc::c_int = 0;
static mut atom_soft_if: libc::c_int = 0;
static mut atom_dollar_var: libc::c_int = 0;
static mut atom_dollar_varname: libc::c_int = 0;
static mut dollar_var_1: WamWord = 0;
static mut dollar_varname_1: WamWord = 0;
static mut equal_2: WamWord = 0;
static mut singl_var_ptr: *mut PlLong = 0 as *const PlLong as *mut PlLong;
static mut nb_singl_var: libc::c_int = 0;
static mut nb_to_try: libc::c_int = 0;
static mut above_H: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub unsafe extern "C" fn Pl_Portray_Clause_3(
    mut sora_word: WamWord,
    mut term_word: WamWord,
    mut above_word: WamWord,
) {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut b: *mut WamWord = 0 as *mut WamWord;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_output
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 3 as libc::c_int)
    };
    pstm = *pl_stm_tbl.offset(stm as isize);
    pl_last_output_sora = sora_word;
    Pl_Check_Stream_Type(stm, 1 as libc::c_int, 0 as libc::c_int);
    b = (*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
        .offset(Pl_Rd_Integer(above_word) as isize);
    above_H = *(&mut *b.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    Portray_Clause(pstm, term_word);
}
pub unsafe extern "C" fn Pl_Portray_Clause_2(
    mut term_word: WamWord,
    mut above_word: WamWord,
) {
    Pl_Portray_Clause_3(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        term_word,
        above_word,
    );
}
unsafe extern "C" fn Portray_Clause(mut pstm: *mut StmInf, mut term_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut arg_word: [WamWord; 2] = [0; 2];
    let mut atom: libc::c_int = 0;
    if Check_Structure(term_word, atom_clause, 2 as libc::c_int, arg_word.as_mut_ptr())
        != 0
    {
        Pl_Write_Term(
            pstm,
            -(1 as libc::c_int),
            1200 as libc::c_int - 1 as libc::c_int,
            4 as libc::c_int | 8 as libc::c_int | 1 as libc::c_int | 16 as libc::c_int,
            above_H,
            arg_word[0 as libc::c_int as usize],
        );
        let mut deref_last_word: WamWord = 0;
        word = arg_word[1 as libc::c_int as usize];
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || atom != pl_atom_true
        {
            Pl_Stream_Puts(
                b" :-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                pstm,
            );
            Start_Line(pstm, 0 as libc::c_int, ' ' as i32 as libc::c_char);
            Show_Body(
                pstm,
                0 as libc::c_int,
                0 as libc::c_int,
                arg_word[1 as libc::c_int as usize],
            );
        }
        Pl_Write_A_Full_Stop(pstm);
        return;
    }
    if Check_Structure(term_word, atom_dcg, 2 as libc::c_int, arg_word.as_mut_ptr()) != 0
    {
        Pl_Write_Term(
            pstm,
            -(1 as libc::c_int),
            1200 as libc::c_int - 1 as libc::c_int,
            4 as libc::c_int | 8 as libc::c_int | 1 as libc::c_int | 16 as libc::c_int,
            above_H,
            arg_word[0 as libc::c_int as usize],
        );
        let mut deref_last_word_0: WamWord = 0;
        word = arg_word[1 as libc::c_int as usize];
        loop {
            deref_last_word_0 = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word_0) {
                break;
            }
        }
        atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || atom != pl_atom_true
        {
            Pl_Stream_Puts(
                b" -->\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                pstm,
            );
            Start_Line(pstm, 0 as libc::c_int, ' ' as i32 as libc::c_char);
            Show_Body(
                pstm,
                0 as libc::c_int,
                0 as libc::c_int,
                arg_word[1 as libc::c_int as usize],
            );
        }
        Pl_Write_A_Full_Stop(pstm);
        return;
    }
    if Check_Structure(term_word, atom_clause, 1 as libc::c_int, arg_word.as_mut_ptr())
        != 0
    {
        Pl_Stream_Puts(
            b":-\t\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            pstm,
        );
        Show_Body(
            pstm,
            0 as libc::c_int,
            0 as libc::c_int,
            arg_word[0 as libc::c_int as usize],
        );
        Pl_Write_A_Full_Stop(pstm);
        return;
    }
    Pl_Write_Term(
        pstm,
        -(1 as libc::c_int),
        1200 as libc::c_int,
        4 as libc::c_int | 8 as libc::c_int | 1 as libc::c_int | 16 as libc::c_int,
        above_H,
        term_word,
    );
    Pl_Write_A_Full_Stop(pstm);
}
unsafe extern "C" fn Check_Structure(
    mut term_word: WamWord,
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg_word: *mut WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = term_word;
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
    if tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong {
        return 0 as libc::c_int;
    }
    adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord;
    if *adr.offset(0 as libc::c_int as isize) as libc::c_ulong
        != (arity as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < arity {
        *arg_word.offset(i as isize) = *adr.offset((1 as libc::c_int + i) as isize);
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Is_Cut(mut body_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut arg_word: [WamWord; 2] = [0; 2];
    while Check_Structure(
        body_word,
        ',' as i32 as libc::c_uchar as libc::c_int,
        2 as libc::c_int,
        arg_word.as_mut_ptr(),
    ) != 0
    {
        body_word = arg_word[0 as libc::c_int as usize];
    }
    let mut deref_last_word: WamWord = 0;
    word = body_word;
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
    return (word as libc::c_ulong
        == ((('!' as i32 as libc::c_uchar as libc::c_int as PlLong) << 3 as libc::c_int)
            as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)) as libc::c_int;
}
unsafe extern "C" fn Show_Body(
    mut pstm: *mut StmInf,
    mut level: libc::c_int,
    mut context: libc::c_int,
    mut body_word: WamWord,
) {
    let mut arg_word: [WamWord; 2] = [0; 2];
    let mut soft_cut: libc::c_int = 0;
    static mut prec: [libc::c_int; 7] = [
        1200 as libc::c_int - 1 as libc::c_int,
        1000 as libc::c_int - 1 as libc::c_int,
        1000 as libc::c_int,
        1100 as libc::c_int - 1 as libc::c_int,
        1100 as libc::c_int,
        1050 as libc::c_int - 1 as libc::c_int,
        1050 as libc::c_int,
    ];
    if Check_Structure(
        body_word,
        ',' as i32 as libc::c_uchar as libc::c_int,
        2 as libc::c_int,
        arg_word.as_mut_ptr(),
    ) != 0
    {
        Show_Body(pstm, level, 1 as libc::c_int, arg_word[0 as libc::c_int as usize]);
        Pl_Stream_Putc(',' as i32, pstm);
        if Is_Cut(arg_word[1 as libc::c_int as usize]) != 0 {
            Pl_Stream_Putc(' ' as i32, pstm);
        } else {
            Start_Line(pstm, level, ' ' as i32 as libc::c_char);
        }
        Show_Body(pstm, level, 2 as libc::c_int, arg_word[1 as libc::c_int as usize]);
        return;
    }
    if Check_Structure(
        body_word,
        ';' as i32 as libc::c_uchar as libc::c_int,
        2 as libc::c_int,
        arg_word.as_mut_ptr(),
    ) != 0
    {
        if context != 4 as libc::c_int {
            Pl_Stream_Puts(
                b"(   \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                pstm,
            );
            level += 1;
            level;
        }
        Show_Body(pstm, level, 3 as libc::c_int, arg_word[0 as libc::c_int as usize]);
        Start_Line(pstm, level, ';' as i32 as libc::c_char);
        Show_Body(pstm, level, 4 as libc::c_int, arg_word[1 as libc::c_int as usize]);
        if context != 4 as libc::c_int {
            Start_Line(pstm, level - 1 as libc::c_int, ' ' as i32 as libc::c_char);
            Pl_Stream_Putc(')' as i32, pstm);
        }
        return;
    }
    soft_cut = 0 as libc::c_int;
    if Check_Structure(body_word, atom_if, 2 as libc::c_int, arg_word.as_mut_ptr()) != 0
        || {
            soft_cut = 1 as libc::c_int;
            Check_Structure(
                body_word,
                atom_soft_if,
                2 as libc::c_int,
                arg_word.as_mut_ptr(),
            ) != 0
        }
    {
        if context != 3 as libc::c_int && context != 4 as libc::c_int {
            Pl_Stream_Puts(
                b"(   \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                pstm,
            );
            level += 1;
            level;
        }
        Show_Body(pstm, level, 5 as libc::c_int, arg_word[0 as libc::c_int as usize]);
        Pl_Stream_Puts(
            (if soft_cut == 0 as libc::c_int {
                b" ->\0" as *const u8 as *const libc::c_char
            } else {
                b" *-> \0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
            pstm,
        );
        Start_Line(pstm, level, ' ' as i32 as libc::c_char);
        Show_Body(pstm, level, 6 as libc::c_int, arg_word[1 as libc::c_int as usize]);
        if context != 3 as libc::c_int && context != 4 as libc::c_int {
            Start_Line(pstm, level - 1 as libc::c_int, ' ' as i32 as libc::c_char);
            Pl_Stream_Putc(')' as i32, pstm);
        }
        return;
    }
    Pl_Write_Term(
        pstm,
        -(1 as libc::c_int),
        prec[context as usize],
        4 as libc::c_int | 8 as libc::c_int | 1 as libc::c_int | 16 as libc::c_int,
        above_H,
        body_word,
    );
}
unsafe extern "C" fn Start_Line(
    mut pstm: *mut StmInf,
    mut level: libc::c_int,
    mut c_before: libc::c_char,
) {
    let mut p: *mut libc::c_char = pl_glob_buff.as_mut_ptr();
    let mut i: libc::c_int = 0;
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = '\n' as i32 as libc::c_char;
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = '\t' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int * (level - 1 as libc::c_int) {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = ' ' as i32 as libc::c_char;
        i += 1;
        i;
    }
    if level != 0 as libc::c_int {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = c_before;
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = ' ' as i32 as libc::c_char;
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = ' ' as i32 as libc::c_char;
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = ' ' as i32 as libc::c_char;
    }
    *p = '\0' as i32 as libc::c_char;
    Pl_Stream_Puts(pl_glob_buff.as_mut_ptr(), pstm);
    pl_last_writing = 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Name_Singleton_Vars_1(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    if Pl_Acyclic_Term_1(start_word) == 0 {
        return;
    }
    singl_var_ptr = pl_glob_dico_var.as_mut_ptr();
    nb_singl_var = 0 as libc::c_int;
    Pl_Treat_Vars_Of_Term(
        start_word,
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WamWord) -> Bool>,
            Option::<unsafe extern "C" fn() -> Bool>,
        >(Some(Collect_Singleton as unsafe extern "C" fn(*mut WamWord) -> Bool)),
    );
    if nb_singl_var == 0 as libc::c_int {
        return;
    }
    word = Pl_Put_Structure(atom_dollar_varname, 1 as libc::c_int);
    Pl_Unify_Atom('_' as i32 as libc::c_uchar as libc::c_int);
    loop {
        singl_var_ptr = singl_var_ptr.offset(-1);
        if !(singl_var_ptr >= pl_glob_dico_var.as_mut_ptr()) {
            break;
        }
        if *singl_var_ptr & 1 as libc::c_int as libc::c_long != 0 {
            continue;
        }
        if (*singl_var_ptr as *mut WamWord)
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || *singl_var_ptr as *mut WamWord
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                && (*singl_var_ptr as *mut WamWord) < B
        {
            let fresh7 = TR;
            TR = TR.offset(1);
            *fresh7 = (*singl_var_ptr as *mut WamWord as PlULong
                | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *(*singl_var_ptr as *mut WamWord) = word;
    };
}
unsafe extern "C" fn Collect_Singleton(mut adr: *mut WamWord) -> Bool {
    let mut p: *mut PlLong = 0 as *mut PlLong;
    p = pl_glob_dico_var.as_mut_ptr();
    while p < singl_var_ptr {
        if *p & !(1 as libc::c_int) as libc::c_long == adr as PlLong {
            if *p & 1 as libc::c_int as libc::c_long == 0 as libc::c_int as libc::c_long
            {
                *p |= 1 as libc::c_int as libc::c_long;
                nb_singl_var -= 1;
                nb_singl_var;
            }
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    if singl_var_ptr.offset_from(pl_glob_dico_var.as_mut_ptr()) as libc::c_long
        >= 32768 as libc::c_int as libc::c_long
    {
        Pl_Err_Representation(pl_representation_too_many_variables);
    }
    let fresh8 = singl_var_ptr;
    singl_var_ptr = singl_var_ptr.offset(1);
    *fresh8 = adr as PlLong;
    nb_singl_var += 1;
    nb_singl_var;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Name_Query_Vars_2(
    mut query_list_word: WamWord,
    mut rest_list_word: WamWord,
) -> Bool {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut stc_adr: *mut WamWord = 0 as *mut WamWord;
    save_list_word = query_list_word;
    Pl_Check_For_Un_List(rest_list_word);
    loop {
        let mut deref_last_word: WamWord = 0;
        word = query_list_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            Pl_Err_Instantiation();
        }
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            break;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_list, save_list_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let mut deref_last_word_0: WamWord = 0;
        word = *lst_adr.offset(0 as libc::c_int as isize);
        loop {
            deref_last_word_0 = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word_0) {
                break;
            }
        }
        stc_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong
            && *stc_adr.offset(0 as libc::c_int as isize) == equal_2
        {
            let mut deref_last_word_1: WamWord = 0;
            word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
            loop {
                deref_last_word_1 = word;
                tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                    as WamWord;
                if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                    break;
                }
                word = *(word as *mut WamWord);
                if !(word != deref_last_word_1) {
                    break;
                }
            }
            if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong {
                current_block = 16962796910045331164;
            } else {
                let mut deref_last_word_2: WamWord = 0;
                word = *stc_adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize);
                loop {
                    deref_last_word_2 = word;
                    tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                        as WamWord;
                    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                        break;
                    }
                    word = *(word as *mut WamWord);
                    if !(word != deref_last_word_2) {
                        break;
                    }
                }
                if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                    current_block = 16962796910045331164;
                } else {
                    Pl_Get_Structure(atom_dollar_varname, 1 as libc::c_int, word);
                    Pl_Unify_Value(
                        *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                    );
                    current_block = 11777552016271000781;
                }
            }
        } else {
            current_block = 16962796910045331164;
        }
        match current_block {
            16962796910045331164 => {
                if Pl_Get_List(rest_list_word) == 0
                    || Pl_Unify_Value(*lst_adr.offset(0 as libc::c_int as isize)) == 0
                {
                    return 0 as libc::c_int;
                }
                rest_list_word = Pl_Unify_Variable();
            }
            _ => {}
        }
        query_list_word = *lst_adr
            .offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    return Pl_Get_Nil(rest_list_word);
}
pub unsafe extern "C" fn Pl_Bind_Variables_4(
    mut term_word: WamWord,
    mut exclude_list_word: WamWord,
    mut from_word: WamWord,
    mut next_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut stc_adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32768 as libc::c_int {
        *pl_glob_dico_var.as_mut_ptr().offset(i as isize) = 0 as libc::c_int as PlLong;
        i += 1;
        i;
    }
    nb_to_try = Pl_Rd_Positive_Check(from_word) as libc::c_int;
    save_list_word = exclude_list_word;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = exclude_list_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            Pl_Err_Instantiation();
        }
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            break;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_list, save_list_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let mut deref_last_word_0: WamWord = 0;
        word = *lst_adr.offset(0 as libc::c_int as isize);
        loop {
            deref_last_word_0 = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word_0) {
                break;
            }
        }
        if Pl_Acyclic_Term_1(word) != 0 {
            Collect_Excluded_Rec(word);
        }
        stc_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong
            && *stc_adr.offset(0 as libc::c_int as isize) == equal_2
        {
            let mut deref_last_word_1: WamWord = 0;
            word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
            loop {
                deref_last_word_1 = word;
                tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                    as WamWord;
                if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                    break;
                }
                word = *(word as *mut WamWord);
                if !(word != deref_last_word_1) {
                    break;
                }
            }
            if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
                Exclude_A_Var_Number(
                    Var_Name_To_Var_Number(
                        (word as PlULong >> 3 as libc::c_int) as libc::c_int,
                    ),
                );
            }
        }
        exclude_list_word = *lst_adr
            .offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    if Pl_Acyclic_Term_1(term_word) != 0 {
        Pl_Treat_Vars_Of_Term(
            term_word,
            0 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut WamWord, WamWord) -> Bool>,
                Option::<unsafe extern "C" fn() -> Bool>,
            >(Some(Bind_Variable as unsafe extern "C" fn(*mut WamWord, WamWord) -> Bool)),
        );
    }
    return Pl_Un_Integer_Check(nb_to_try as PlLong, next_word);
}
unsafe extern "C" fn Var_Name_To_Var_Number(mut atom: libc::c_int) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    p = (*pl_atom_tbl.offset(atom as isize)).name;
    if (*p as libc::c_int) < 'A' as i32 || *p as libc::c_int > 'Z' as i32 {
        return -(1 as libc::c_int);
    }
    n = strtol(p.offset(1 as libc::c_int as isize), &mut q, 10 as libc::c_int)
        as libc::c_int;
    if *q != 0 {
        return -(1 as libc::c_int);
    }
    n = n * 26 as libc::c_int + *p as libc::c_int - 'A' as i32;
    return n;
}
unsafe extern "C" fn Exclude_A_Var_Number(mut n: libc::c_int) {
    if n >= 0 as libc::c_int && n < 32768 as libc::c_int {
        *pl_glob_dico_var.as_mut_ptr().offset(n as isize) = 1 as libc::c_int as PlLong;
    }
}
unsafe extern "C" fn Collect_Excluded_Rec(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut stc_adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = start_word;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            adr = &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
            let fresh9 = adr;
            adr = adr.offset(1);
            Collect_Excluded_Rec(*fresh9);
            start_word = *adr;
        } else {
            if tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong {
                return;
            }
            stc_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            if *stc_adr.offset(0 as libc::c_int as isize) == dollar_var_1 {
                let mut deref_last_word_0: WamWord = 0;
                word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
                loop {
                    deref_last_word_0 = word;
                    tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                        as WamWord;
                    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                        break;
                    }
                    word = *(word as *mut WamWord);
                    if !(word != deref_last_word_0) {
                        break;
                    }
                }
                if !(tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong) {
                    Exclude_A_Var_Number(
                        (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int,
                    );
                    return;
                }
            } else if *stc_adr.offset(0 as libc::c_int as isize) == dollar_varname_1 {
                let mut deref_last_word_1: WamWord = 0;
                word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
                loop {
                    deref_last_word_1 = word;
                    tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                        as WamWord;
                    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                        break;
                    }
                    word = *(word as *mut WamWord);
                    if !(word != deref_last_word_1) {
                        break;
                    }
                }
                if !(tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong) {
                    Exclude_A_Var_Number(
                        Var_Name_To_Var_Number(
                            (word as PlULong >> 3 as libc::c_int) as libc::c_int,
                        ),
                    );
                    return;
                }
            }
            i = (*stc_adr.offset(0 as libc::c_int as isize) as PlULong
                >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
            adr = &mut *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut WamWord;
            loop {
                i -= 1;
                if !(i != 0) {
                    break;
                }
                let fresh10 = adr;
                adr = adr.offset(1);
                Collect_Excluded_Rec(*fresh10);
            }
            start_word = *adr;
        }
    };
}
unsafe extern "C" fn Bind_Variable(mut adr: *mut WamWord, mut word: WamWord) -> Bool {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut buff: [libc::c_char; 16] = [0; 16];
    while *pl_glob_dico_var.as_mut_ptr().offset(nb_to_try as isize) != 0
        && nb_to_try < 32768 as libc::c_int
    {
        nb_to_try += 1;
        nb_to_try;
    }
    if *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
        == 0 as libc::c_int as libc::c_long
    {
        Pl_Get_Structure(atom_dollar_var, 1 as libc::c_int, word);
        let fresh11 = nb_to_try;
        nb_to_try = nb_to_try + 1;
        Pl_Unify_Integer(fresh11 as PlLong);
        return 1 as libc::c_int;
    }
    i = nb_to_try % 26 as libc::c_int;
    j = nb_to_try / 26 as libc::c_int;
    nb_to_try += 1;
    nb_to_try;
    buff[0 as libc::c_int as usize] = ('A' as i32 + i) as libc::c_char;
    if j != 0 {
        sprintf(
            buff.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"%d\0" as *const u8 as *const libc::c_char,
            j,
        );
    } else {
        buff[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    Pl_Get_Structure(atom_dollar_varname, 1 as libc::c_int, word);
    Pl_Unify_Atom(Pl_Create_Allocate_Atom(buff.as_mut_ptr()));
    return 1 as libc::c_int;
}
