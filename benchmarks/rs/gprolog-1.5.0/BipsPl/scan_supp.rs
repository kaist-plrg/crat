use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_atom_void: libc::c_int;
    static mut pl_char_conv: [libc::c_char; 0];
    static mut pl_char_type: [libc::c_int; 0];
    static mut pl_escape_symbol: [libc::c_char; 0];
    static mut pl_escape_char: [libc::c_char; 0];
    static mut pl_flag_char_conversion: *mut FlagInf;
    static mut pl_flag_strict_iso: *mut FlagInf;
    static mut pl_flag_back_quotes: *mut FlagInf;
    static mut pl_flag_double_quotes: *mut FlagInf;
    fn Pl_Stream_Getc(pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_Ungetc(c_0: libc::c_int, pstm: *mut StmInf);
    fn Pl_Stream_Peekc(pstm: *mut StmInf) -> libc::c_int;
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub static mut pl_token: TokInf = TokInf {
    type_0: TOKEN_VARIABLE,
    name: [0; 10240],
    quoted: 0,
    punct: 0,
    int_num: 0,
    float_num: 0.,
    line: 0,
    col: 0,
};
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
static mut c_orig: libc::c_int = 0;
static mut c: libc::c_int = 0;
static mut c_type: libc::c_int = 0;
static mut err_msg: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub unsafe extern "C" fn Pl_Scan_Peek_Char(
    mut pstm: *mut StmInf,
    mut convert: Bool,
) -> libc::c_int {
    let mut c_look: libc::c_int = 0;
    c_look = Pl_Stream_Peekc(pstm);
    if convert != 0 {
        c_look = if (*pl_flag_char_conversion).value != 0
            && (c_look as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        {
            *pl_char_conv.as_mut_ptr().offset(c_look as isize) as libc::c_int
        } else {
            c_look
        };
    }
    return c_look;
}
unsafe extern "C" fn Read_Next_Char(
    mut pstm: *mut StmInf,
    mut convert: Bool,
) -> libc::c_int {
    c = Pl_Stream_Getc(pstm);
    c_orig = c;
    if c == -(1 as libc::c_int) {
        c_type = 0 as libc::c_int;
    } else {
        if convert != 0 {
            c = if (*pl_flag_char_conversion).value != 0
                && (c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            {
                *pl_char_conv.as_mut_ptr().offset(c as isize) as libc::c_int
            } else {
                c
            };
        }
        c_type = *pl_char_type.as_mut_ptr().offset(c as isize);
    }
    return c;
}
pub unsafe extern "C" fn Pl_Scan_Token(
    mut pstm: *mut StmInf,
    mut comma_is_punct: Bool,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut c0: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut layout_before: Bool = 0 as libc::c_int;
    err_msg = 0 as *mut libc::c_char;
    loop {
        loop {
            Read_Next_Char(pstm, 1 as libc::c_int);
            if c_type != 1 as libc::c_int {
                break;
            }
            layout_before = 1 as libc::c_int;
        }
        pl_token.quoted = 0 as libc::c_int;
        pl_token
            .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        pl_token.col = (*pstm).line_pos as libc::c_int;
        if c == -(1 as libc::c_int) {
            pl_token.type_0 = TOKEN_END_OF_FILE;
            return err_msg;
        }
        match c_type {
            1024 => {
                current_block = 16366488808866765858;
                break;
            }
            256 | 512 => {
                current_block = 16366488808866765858;
                break;
            }
            128 => {
                Scan_Number(pstm, 0 as libc::c_int);
                current_block = 9512719473022792396;
                break;
            }
            4 => {
                current_block = 15860131741026162785;
                break;
            }
            8 | 16 => {
                current_block = 15860131741026162785;
                break;
            }
            32 => {
                c0 = c;
                Read_Next_Char(pstm, 1 as libc::c_int);
                if c0 == '.' as i32
                    && (c == -(1 as libc::c_int)
                        || c_type & (1 as libc::c_int | 2048 as libc::c_int) != 0)
                {
                    if c_type != -(1 as libc::c_int) {
                        Pl_Stream_Ungetc(c_orig, pstm);
                    }
                    pl_token.type_0 = TOKEN_FULL_STOP;
                    current_block = 9512719473022792396;
                    break;
                } else if c0 == '/' as i32 && c == '*' as i32 {
                    Read_Next_Char(pstm, 1 as libc::c_int);
                    if c != -(1 as libc::c_int) {
                        loop {
                            c0 = c;
                            Read_Next_Char(pstm, 1 as libc::c_int);
                            if !(c != -(1 as libc::c_int)
                                && (c0 != '*' as i32 || c != '/' as i32))
                            {
                                break;
                            }
                        }
                    }
                    if c == -(1 as libc::c_int) {
                        pl_token.type_0 = TOKEN_END_OF_FILE;
                        pl_token
                            .line = ((*pstm).line_count
                            + 1 as libc::c_int as libc::c_long) as libc::c_int;
                        pl_token.col = (*pstm).line_pos as libc::c_int;
                        err_msg = b"*/ expected here for /*...*/ comment\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        current_block = 9512719473022792396;
                        break;
                    } else {
                        layout_before = 1 as libc::c_int;
                    }
                } else {
                    pl_token.type_0 = TOKEN_NAME;
                    s = (pl_token.name).as_mut_ptr();
                    let fresh1 = s;
                    s = s.offset(1);
                    *fresh1 = c0 as libc::c_char;
                    while c_type == 32 as libc::c_int {
                        let fresh2 = s;
                        s = s.offset(1);
                        *fresh2 = c as libc::c_char;
                        Read_Next_Char(pstm, 1 as libc::c_int);
                    }
                    *s = '\0' as i32 as libc::c_char;
                    Pl_Stream_Ungetc(c_orig, pstm);
                    current_block = 9512719473022792396;
                    break;
                }
            }
            2048 => {
                loop {
                    Read_Next_Char(pstm, 1 as libc::c_int);
                    if !(c != '\n' as i32 && c != -(1 as libc::c_int)) {
                        break;
                    }
                }
                layout_before = 1 as libc::c_int;
            }
            64 => {
                if c == '(' as i32 && layout_before == 0 {
                    current_block = 2989495919056355252;
                    break;
                } else {
                    current_block = 1423531122933789233;
                    break;
                }
            }
            2 => {
                if c == ',' as i32 && comma_is_punct != 0 {
                    current_block = 2516253395664191498;
                    break;
                } else {
                    current_block = 10930818133215224067;
                    break;
                }
            }
            4096 => {
                pl_token.type_0 = TOKEN_EXTENDED;
                pl_token.name[0 as libc::c_int as usize] = c as libc::c_char;
                pl_token.name[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                current_block = 9512719473022792396;
                break;
            }
            _ => {
                current_block = 9512719473022792396;
                break;
            }
        }
    }
    match current_block {
        10930818133215224067 => {
            pl_token.type_0 = TOKEN_NAME;
            pl_token.name[0 as libc::c_int as usize] = c as libc::c_char;
            pl_token.name[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }
        16366488808866765858 => {
            pl_token
                .type_0 = (if c_type == 1024 as libc::c_int {
                TOKEN_NAME as libc::c_int
            } else {
                TOKEN_VARIABLE as libc::c_int
            }) as TypTok;
            s = (pl_token.name).as_mut_ptr();
            loop {
                let fresh0 = s;
                s = s.offset(1);
                *fresh0 = c as libc::c_char;
                Read_Next_Char(pstm, 1 as libc::c_int);
                if !(c_type
                    & (256 as libc::c_int | 512 as libc::c_int | 1024 as libc::c_int
                        | 128 as libc::c_int) != 0)
                {
                    break;
                }
            }
            *s = '\0' as i32 as libc::c_char;
            Pl_Stream_Ungetc(c_orig, pstm);
        }
        2516253395664191498 => {
            pl_token.type_0 = TOKEN_PUNCTUATION;
            pl_token.punct = c;
        }
        1423531122933789233 => {
            pl_token.type_0 = TOKEN_PUNCTUATION;
            pl_token.punct = c;
        }
        2989495919056355252 => {
            pl_token.type_0 = TOKEN_IMMEDIAT_OPEN;
        }
        15860131741026162785 => {
            Scan_Quoted(pstm);
        }
        _ => {}
    }
    return err_msg;
}
unsafe extern "C" fn Scan_Number(mut pstm: *mut StmInf, mut integer_only: Bool) {
    let mut lg: libc::c_int = 0;
    let mut radix: libc::c_int = 0;
    let mut radix_c: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c_orig0: libc::c_int = 0;
    p = (pl_token.name).as_mut_ptr();
    loop {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = c as libc::c_char;
        Read_Next_Char(pstm, 1 as libc::c_int);
        if !(c_type == 128 as libc::c_int) {
            break;
        }
    }
    lg = p.offset_from((pl_token.name).as_mut_ptr()) as libc::c_long as libc::c_int;
    if integer_only == 0 && c == '.' as i32
        && *(*__ctype_b_loc()).offset(Pl_Scan_Peek_Char(pstm, 1 as libc::c_int) as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        pl_token.type_0 = TOKEN_FLOAT;
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = '.' as i32 as libc::c_char;
        Read_Next_Char(pstm, 1 as libc::c_int);
        while c_type == 128 as libc::c_int {
            let fresh6 = p;
            p = p.offset(1);
            *fresh6 = c as libc::c_char;
            Read_Next_Char(pstm, 1 as libc::c_int);
        }
        if c == 'e' as i32 || c == 'E' as i32 {
            c_orig0 = c_orig;
            Read_Next_Char(pstm, 1 as libc::c_int);
            if !(c_type == 128 as libc::c_int
                || (c == '+' as i32 || c == '-' as i32)
                    && *(*__ctype_b_loc())
                        .offset(Pl_Scan_Peek_Char(pstm, 1 as libc::c_int) as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
            {
                Pl_Stream_Ungetc(c_orig, pstm);
                c_orig = c_orig0;
            } else {
                let fresh7 = p;
                p = p.offset(1);
                *fresh7 = 'e' as i32 as libc::c_char;
                let fresh8 = p;
                p = p.offset(1);
                *fresh8 = c as libc::c_char;
                Read_Next_Char(pstm, 1 as libc::c_int);
                while c_type == 128 as libc::c_int {
                    let fresh9 = p;
                    p = p.offset(1);
                    *fresh9 = c as libc::c_char;
                    Read_Next_Char(pstm, 1 as libc::c_int);
                }
            }
        }
        *p = '\0' as i32 as libc::c_char;
        sscanf(
            (pl_token.name).as_mut_ptr(),
            b"%lf\0" as *const u8 as *const libc::c_char,
            &mut pl_token.float_num as *mut libc::c_double,
        );
    } else {
        pl_token.type_0 = TOKEN_INTEGER;
        *p = '\0' as i32 as libc::c_char;
        pl_token
            .int_num = strtol((pl_token.name).as_mut_ptr(), &mut p, 10 as libc::c_int);
        if !(lg != 1 as libc::c_int
            || pl_token.int_num != 0 as libc::c_int as libc::c_long
            || (strchr(b"'box\0" as *const u8 as *const libc::c_char, c)).is_null())
        {
            if c == '\'' as i32 {
                c = Scan_Quoted_Char(
                    pstm,
                    1 as libc::c_int,
                    '\'' as i32,
                    0 as libc::c_int,
                );
                if c == -(1 as libc::c_int) {
                    if (*pl_flag_strict_iso).value != 0 {
                        if ((*pl_atom_tbl.offset(pl_atom_void as isize)).prop).op_mask()
                            as libc::c_int & (1 as libc::c_int) << 2 as libc::c_int != 0
                            || ((*pl_atom_tbl.offset(pl_atom_void as isize)).prop)
                                .op_mask() as libc::c_int
                                & (1 as libc::c_int) << 1 as libc::c_int != 0
                        {
                            Pl_Stream_Ungetc('\'' as i32, pstm);
                            Pl_Stream_Ungetc('\'' as i32, pstm);
                            return;
                        }
                        pl_token
                            .line = ((*pstm).line_count
                            + 1 as libc::c_int as libc::c_long) as libc::c_int;
                        pl_token
                            .col = ((*pstm).line_pos + 1 as libc::c_int as libc::c_long)
                            as libc::c_int;
                        err_msg = b"quote character expected here\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        return;
                    } else {
                        c = '\'' as i32;
                    }
                } else if c == -(2 as libc::c_int) {
                    Pl_Stream_Ungetc(c_orig, pstm);
                    Pl_Stream_Ungetc('\\' as i32, pstm);
                    Pl_Stream_Ungetc('\'' as i32, pstm);
                    return;
                } else if c < 0 as libc::c_int {
                    Pl_Stream_Ungetc(c_orig, pstm);
                    pl_token.type_0 = TOKEN_FULL_STOP;
                    pl_token
                        .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long)
                        as libc::c_int;
                    pl_token
                        .col = ((*pstm).line_pos + 1 as libc::c_int as libc::c_long)
                        as libc::c_int;
                    err_msg = b"character expected here\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    return;
                }
                pl_token.int_num = c as PlLong;
                return;
            }
            radix_c = c;
            radix = if c == 'b' as i32 {
                f = b"01\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                2 as libc::c_int
            } else if c == 'o' as i32 {
                f = b"01234567\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                8 as libc::c_int
            } else {
                f = b"0123456789abcdefABCDEF\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                16 as libc::c_int
            };
            p = (pl_token.name).as_mut_ptr();
            Read_Next_Char(pstm, 1 as libc::c_int);
            while !(strchr(f, c)).is_null() {
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = c as libc::c_char;
                Read_Next_Char(pstm, 1 as libc::c_int);
            }
            *p = '\0' as i32 as libc::c_char;
            if p == (pl_token.name).as_mut_ptr() {
                Pl_Stream_Ungetc(c_orig, pstm);
                Pl_Stream_Ungetc(radix_c, pstm);
                return;
            }
            pl_token.int_num = strtol((pl_token.name).as_mut_ptr(), &mut p, radix);
        }
    }
    Pl_Stream_Ungetc(c_orig, pstm);
}
unsafe extern "C" fn Scan_Quoted(mut pstm: *mut StmInf) {
    let mut c0: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut convert: Bool = (c_orig != '\'' as i32) as libc::c_int;
    let mut no_escape: Bool = 0;
    let mut error_found: Bool = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    if c_type == 4 as libc::c_int {
        pl_token.type_0 = TOKEN_NAME;
        pl_token.quoted = 1 as libc::c_int;
        i = 0 as libc::c_int;
    } else if c_type == 8 as libc::c_int {
        pl_token.type_0 = TOKEN_STRING;
        i = (*pl_flag_double_quotes).value as libc::c_int;
    } else {
        pl_token.type_0 = TOKEN_BACK_QUOTED;
        i = (*pl_flag_back_quotes).value as libc::c_int;
    }
    s = (pl_token.name).as_mut_ptr();
    c0 = c;
    no_escape = i >> 2 as libc::c_int;
    loop {
        c = Scan_Quoted_Char(pstm, convert, c0, no_escape);
        if c == -(1 as libc::c_int) {
            if error_found != 0 {
                break;
            }
            *s = '\0' as i32 as libc::c_char;
            return;
        } else {
            if c == -(2 as libc::c_int) {
                continue;
            }
            if c == -(3 as libc::c_int) || c == -(4 as libc::c_int) {
                pl_token.type_0 = TOKEN_FULL_STOP;
                *s = '\0' as i32 as libc::c_char;
                return;
            }
            if c == -(5 as libc::c_int) || c == -(6 as libc::c_int) {
                error_found = 1 as libc::c_int;
            } else if error_found == 0 {
                let fresh10 = s;
                s = s.offset(1);
                *fresh10 = c as libc::c_char;
            }
        }
    }
    *s = '\0' as i32 as libc::c_char;
    if !err_msg.is_null() {
        return;
    }
    Pl_Stream_Ungetc(c_orig, pstm);
    pl_token
        .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long) as libc::c_int;
    pl_token.col = ((*pstm).line_pos + 1 as libc::c_int as libc::c_long) as libc::c_int;
    match pl_token.type_0 as libc::c_uint {
        3 => {
            err_msg = b"quote character expected here\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        5 => {
            err_msg = b"back quote character expected here\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        4 => {
            err_msg = b"double quote character expected here\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        _ => {}
    };
}
unsafe extern "C" fn Scan_Quoted_Char(
    mut pstm: *mut StmInf,
    mut convert: Bool,
    mut c0: libc::c_int,
    mut no_escape: Bool,
) -> libc::c_int {
    let mut radix: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    Read_Next_Char(pstm, convert);
    if c == c0 {
        if Pl_Scan_Peek_Char(pstm, convert) != c0 {
            return -(1 as libc::c_int);
        }
        Read_Next_Char(pstm, convert);
        return c;
    }
    if c == -(1 as libc::c_int) {
        if err_msg.is_null() {
            Pl_Stream_Ungetc(c_orig, pstm);
            pl_token
                .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long)
                as libc::c_int;
            pl_token
                .col = ((*pstm).line_pos + 1 as libc::c_int as libc::c_long)
                as libc::c_int;
            err_msg = b"unexpected end of file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        return -(3 as libc::c_int);
    }
    if c == '\n' as i32 {
        if err_msg.is_null() {
            Pl_Stream_Ungetc(c_orig, pstm);
            pl_token
                .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long)
                as libc::c_int;
            pl_token
                .col = ((*pstm).line_pos + 1 as libc::c_int as libc::c_long)
                as libc::c_int;
            err_msg = b"unexpected newline\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        return -(4 as libc::c_int);
    }
    if c == '\t' as i32 {
        if err_msg.is_null() {
            Pl_Stream_Ungetc(c_orig, pstm);
            pl_token
                .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long)
                as libc::c_int;
            pl_token
                .col = ((*pstm).line_pos + 1 as libc::c_int as libc::c_long)
                as libc::c_int;
            err_msg = b"unexpected tab\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        return -(5 as libc::c_int);
    }
    if c != '\\' as i32 || no_escape != 0 {
        return c;
    }
    Read_Next_Char(pstm, convert);
    if c == '\n' as i32 {
        return -(2 as libc::c_int);
    }
    if !(strchr(b"\\'\"`\0" as *const u8 as *const libc::c_char, c)).is_null() {
        return c;
    }
    p = strchr(pl_escape_symbol.as_mut_ptr(), c);
    if !p.is_null() {
        return *pl_escape_char
            .as_mut_ptr()
            .offset(
                p.offset_from(pl_escape_symbol.as_mut_ptr()) as libc::c_long as isize,
            ) as libc::c_int;
    }
    if (*pl_flag_strict_iso).value == 0 {
        if c == 's' as i32 {
            return ' ' as i32;
        }
        if c == 'e' as i32 {
            return 27 as libc::c_int;
        }
    }
    if c == 'x' as i32 || '0' as i32 <= c && c <= '7' as i32 {
        if c == 'x' as i32 {
            radix = 16 as libc::c_int;
            f = b"0123456789abcdefABCDEF\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            x = 0 as libc::c_int;
        } else {
            radix = 8 as libc::c_int;
            f = b"01234567\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            x = c - '0' as i32;
        }
        Read_Next_Char(pstm, convert);
        loop {
            p = strchr(f, c);
            if p.is_null() {
                break;
            }
            i = p.offset_from(f) as libc::c_long as libc::c_int;
            if i >= 16 as libc::c_int {
                i -= 6 as libc::c_int;
            }
            x = x * radix + i;
            Read_Next_Char(pstm, convert);
        }
        if !((x as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        {
            if err_msg.is_null() {
                pl_token
                    .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long)
                    as libc::c_int;
                pl_token.col = (*pstm).line_pos as libc::c_int;
                err_msg = b"invalid character code in \\constant\\ sequence\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
        } else if c != '\\' as i32 {
            if err_msg.is_null() {
                pl_token
                    .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long)
                    as libc::c_int;
                pl_token.col = (*pstm).line_pos as libc::c_int;
                err_msg = b"\\ expected in \\constant\\ sequence\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
        } else {
            return x as libc::c_uchar as libc::c_int
        }
        while c != '\\' as i32 && c != c0 && c != -(1 as libc::c_int) && c != '\n' as i32
        {
            Read_Next_Char(pstm, convert);
        }
        if c == c0 {
            Pl_Stream_Ungetc(c_orig, pstm);
        }
        return -(6 as libc::c_int);
    }
    if err_msg.is_null() {
        pl_token
            .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        pl_token.col = (*pstm).line_pos as libc::c_int;
        err_msg = b"unknown escape sequence\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    return -(6 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Recover_After_Error(mut pstm: *mut StmInf) {
    let mut c0: libc::c_int = 0;
    let mut convert: Bool = 0;
    let mut dot_found: Bool = 0;
    if (*pstm).eof_reached != 0 {
        return;
    }
    loop {
        convert = 0 as libc::c_int;
        Read_Next_Char(pstm, convert);
        if c == -(1 as libc::c_int) {
            return;
        }
        if c == '.' as i32 {
            Read_Next_Char(pstm, convert);
            if c == -(1 as libc::c_int) {
                return;
            }
            if c_type & (1 as libc::c_int | 2048 as libc::c_int) != 0 {
                return;
            }
        }
        if c_type & (4 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int)
            == 0 as libc::c_int
        {
            continue;
        }
        c0 = c;
        convert = (c_orig != '\'' as i32) as libc::c_int;
        dot_found = 0 as libc::c_int;
        's_80: loop {
            Read_Next_Char(pstm, convert);
            if c == -(1 as libc::c_int) {
                return;
            }
            if c == c0 {
                break;
            }
            if c == '.' as i32 {
                dot_found = 1 as libc::c_int;
            } else if c_type & (1 as libc::c_int | 2048 as libc::c_int)
                == 0 as libc::c_int
            {
                dot_found = 0 as libc::c_int;
            }
            if c == '\n' as i32 {
                if dot_found != 0 {
                    return;
                }
                break;
            } else {
                if c != '\\' as i32 {
                    continue;
                }
                Read_Next_Char(pstm, convert);
                if c == -(1 as libc::c_int) {
                    return;
                }
                if !(c == '\n' as i32) {
                    if c == '.' as i32 {
                        dot_found = 1 as libc::c_int;
                    }
                    if (strchr(b"\\'\"`\0" as *const u8 as *const libc::c_char, c))
                        .is_null()
                    {
                        if !(strchr(pl_escape_symbol.as_mut_ptr(), c)).is_null() {
                            continue;
                        }
                        if c != 'x' as i32 && (c < '0' as i32 || c > '7' as i32) {
                            continue;
                        }
                        loop {
                            Read_Next_Char(pstm, convert);
                            if c == -(1 as libc::c_int) {
                                return;
                            }
                            if c == c0 {
                                break 's_80;
                            }
                            if c == '\\' as i32
                                || *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                                    & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                            {
                                break;
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe extern "C" fn Pl_Scan_Next_Atom(mut pstm: *mut StmInf) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    err_msg = 0 as *mut libc::c_char;
    loop {
        Read_Next_Char(pstm, 1 as libc::c_int);
        if !(c_type == 1 as libc::c_int) {
            break;
        }
    }
    pl_token
        .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long) as libc::c_int;
    pl_token.col = (*pstm).line_pos as libc::c_int;
    match c_type {
        1024 => {
            s = (pl_token.name).as_mut_ptr();
            loop {
                let fresh11 = s;
                s = s.offset(1);
                *fresh11 = c as libc::c_char;
                Read_Next_Char(pstm, 1 as libc::c_int);
                if !(c_type
                    & (256 as libc::c_int | 512 as libc::c_int | 1024 as libc::c_int
                        | 128 as libc::c_int) != 0)
                {
                    break;
                }
            }
            *s = '\0' as i32 as libc::c_char;
            Pl_Stream_Ungetc(c_orig, pstm);
            current_block = 14763689060501151050;
        }
        8 => {
            if (*pl_flag_double_quotes).value
                & (((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_long != 2 as libc::c_int as libc::c_long
            {
                current_block = 5912704192914346702;
            } else {
                current_block = 10323116084737547143;
            }
        }
        16 => {
            if (*pl_flag_back_quotes).value
                & (((1 as libc::c_int) << 2 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_long != 2 as libc::c_int as libc::c_long
            {
                current_block = 5912704192914346702;
            } else {
                current_block = 10323116084737547143;
            }
        }
        4 => {
            current_block = 10323116084737547143;
        }
        32 => {
            s = (pl_token.name).as_mut_ptr();
            while c_type == 32 as libc::c_int {
                let fresh12 = s;
                s = s.offset(1);
                *fresh12 = c as libc::c_char;
                Read_Next_Char(pstm, 1 as libc::c_int);
            }
            *s = '\0' as i32 as libc::c_char;
            Pl_Stream_Ungetc(c_orig, pstm);
            current_block = 14763689060501151050;
        }
        2 => {
            pl_token.name[0 as libc::c_int as usize] = c as libc::c_char;
            pl_token.name[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            current_block = 14763689060501151050;
        }
        _ => {
            current_block = 5912704192914346702;
        }
    }
    match current_block {
        10323116084737547143 => {
            err_msg = 0 as *mut libc::c_char;
            Scan_Quoted(pstm);
            if !err_msg.is_null() {
                return err_msg;
            }
        }
        5912704192914346702 => {
            Pl_Stream_Ungetc(c_orig, pstm);
            return b"cannot start an atom (use quotes ?)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        _ => {}
    }
    pl_token.type_0 = TOKEN_NAME;
    return 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_Scan_Next_Number(
    mut pstm: *mut StmInf,
    mut integer_only: Bool,
) -> *mut libc::c_char {
    let mut minus_op: Bool = 0 as libc::c_int;
    err_msg = 0 as *mut libc::c_char;
    loop {
        Read_Next_Char(pstm, 1 as libc::c_int);
        if c_type != 1 as libc::c_int {
            break;
        }
    }
    pl_token
        .line = ((*pstm).line_count + 1 as libc::c_int as libc::c_long) as libc::c_int;
    pl_token.col = (*pstm).line_pos as libc::c_int;
    if c == '-' as i32 {
        loop {
            Read_Next_Char(pstm, 1 as libc::c_int);
            if c_type != 1 as libc::c_int {
                break;
            }
        }
        minus_op = 1 as libc::c_int;
    }
    if c_type != 128 as libc::c_int {
        Pl_Stream_Ungetc(c_orig, pstm);
        return b"cannot start a number\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    Scan_Number(pstm, integer_only);
    if !err_msg.is_null() {
        return err_msg;
    }
    if minus_op != 0 {
        if pl_token.type_0 as libc::c_uint
            == TOKEN_INTEGER as libc::c_int as libc::c_uint
        {
            if pl_token.int_num
                > -(-(((1 as libc::c_int as PlLong)
                    << 8 as libc::c_int * 8 as libc::c_int - 3 as libc::c_int
                        - 1 as libc::c_int) - 1 as libc::c_int as libc::c_long)
                    - 1 as libc::c_int as libc::c_long)
            {
                return b"integer underflow (exceeds min_integer)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            pl_token.int_num = -pl_token.int_num;
        } else {
            pl_token.float_num = -pl_token.float_num;
        }
    } else if pl_token.type_0 as libc::c_uint
        == TOKEN_INTEGER as libc::c_int as libc::c_uint
        && pl_token.int_num
            > ((1 as libc::c_int as PlLong)
                << 8 as libc::c_int * 8 as libc::c_int - 3 as libc::c_int
                    - 1 as libc::c_int) - 1 as libc::c_int as libc::c_long
    {
        return b"integer overflow (exceeds max_integer)\0" as *const u8
            as *const libc::c_char as *mut libc::c_char
    }
    return 0 as *mut libc::c_char;
}
