use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Exit_With_Exception();
    fn Pl_Cut(b_word: WamWord);
    fn Pl_Write_Term(
        pstm: *mut StmInf,
        depth: libc::c_int,
        prec: libc::c_int,
        mask: libc::c_int,
        above_H: *mut WamWord,
        term_word: WamWord,
    );
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_stm_top_level_output: libc::c_int;
    fn Pl_Stream_Printf(
        pstm: *mut StmInf,
        format: *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
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
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_query_top_b: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_query_exception: WamWord = 0;
pub unsafe extern "C" fn Pl_Throw_2(mut ball_word: WamWord, mut b_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut b: *mut WamWord = 0 as *mut WamWord;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
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
    if b <= pl_query_top_b && !pl_query_top_b.is_null() {
        B = pl_query_top_b;
        let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        pl_query_exception = ball_word;
        Pl_Exit_With_Exception();
    }
    if b
        == *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
    {
        pstm = *pl_stm_tbl.offset(pl_stm_top_level_output as isize);
        Pl_Stream_Printf(
            pstm,
            b"\nsystem_error(cannot_catch_throw(\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        Pl_Write_Term(
            pstm,
            -(1 as libc::c_int),
            1200 as libc::c_int,
            4 as libc::c_int | 8 as libc::c_int | 1 as libc::c_int,
            0 as *mut WamWord,
            ball_word,
        );
        Pl_Stream_Printf(
            pstm,
            b"))\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    Pl_Cut(b_word);
}
