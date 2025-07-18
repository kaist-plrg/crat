use ::libc;
use ::c2rust_bitfields;
extern "C" {
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_last_output_sora: WamWord;
    static mut pl_stm_output: libc::c_int;
    fn Pl_Get_Stream_Or_Alias(sora_word: WamWord, test_mask: libc::c_int) -> libc::c_int;
    fn Pl_Check_Stream_Type(stm: libc::c_int, check_text: Bool, for_input: Bool);
    fn Pl_Stream_Putc(c: libc::c_int, pstm: *mut StmInf);
    fn Pl_Write_Term(
        pstm: *mut StmInf,
        depth: libc::c_int,
        prec: libc::c_int,
        mask: libc::c_int,
        above_H: *mut WamWord,
        term_word: WamWord,
    );
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
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub unsafe extern "C" fn Pl_Write_Term_2(
    mut sora_word: WamWord,
    mut term_word: WamWord,
) {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut above_H: *mut WamWord = 0 as *mut WamWord;
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
    if *pl_sys_var.as_mut_ptr().offset(3 as libc::c_int as isize)
        > 0 as libc::c_int as libc::c_long
    {
        let mut b: *mut WamWord = (*(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
            .offset(*pl_sys_var.as_mut_ptr().offset(3 as libc::c_int as isize) as isize);
        above_H = *(&mut *b.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    }
    Pl_Write_Term(
        pstm,
        *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize) as libc::c_int,
        *pl_sys_var.as_mut_ptr().offset(2 as libc::c_int as isize) as libc::c_int,
        *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize) as libc::c_int,
        above_H,
        term_word,
    );
}
pub unsafe extern "C" fn Pl_Write_Term_1(mut term_word: WamWord) {
    Pl_Write_Term_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        term_word,
    );
}
pub unsafe extern "C" fn Pl_Write_1(mut term_word: WamWord) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ) = (4 as libc::c_int | 8 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        term_word,
    );
}
pub unsafe extern "C" fn Pl_Write_2(mut sora_word: WamWord, mut term_word: WamWord) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ) = (4 as libc::c_int | 8 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(sora_word, term_word);
}
pub unsafe extern "C" fn Pl_Writeq_1(mut term_word: WamWord) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ) = (4 as libc::c_int | 8 as libc::c_int | 1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        term_word,
    );
}
pub unsafe extern "C" fn Pl_Writeq_2(mut sora_word: WamWord, mut term_word: WamWord) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ) = (4 as libc::c_int | 8 as libc::c_int | 1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(sora_word, term_word);
}
pub unsafe extern "C" fn Pl_Write_Canonical_1(mut term_word: WamWord) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ) = (2 as libc::c_int | 1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        term_word,
    );
}
pub unsafe extern "C" fn Pl_Write_Canonical_2(
    mut sora_word: WamWord,
    mut term_word: WamWord,
) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ) = (2 as libc::c_int | 1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(sora_word, term_word);
}
pub unsafe extern "C" fn Pl_Display_1(mut term_word: WamWord) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = 2 as libc::c_int as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        term_word,
    );
}
pub unsafe extern "C" fn Pl_Display_2(mut sora_word: WamWord, mut term_word: WamWord) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = 2 as libc::c_int as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(sora_word, term_word);
}
pub unsafe extern "C" fn Pl_Print_1(mut term_word: WamWord) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ) = (4 as libc::c_int | 8 as libc::c_int | 32 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        term_word,
    );
}
pub unsafe extern "C" fn Pl_Print_2(mut sora_word: WamWord, mut term_word: WamWord) {
    *pl_sys_var
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ) = (4 as libc::c_int | 8 as libc::c_int | 32 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    *pl_sys_var
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize) = 1200 as libc::c_int as PlLong;
    Pl_Write_Term_2(sora_word, term_word);
}
pub unsafe extern "C" fn Pl_Nl_1(mut sora_word: WamWord) {
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
    Pl_Stream_Putc('\n' as i32, *pl_stm_tbl.offset(stm as isize));
}
pub unsafe extern "C" fn Pl_Nl_0() {
    Pl_Nl_1(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
    );
}
