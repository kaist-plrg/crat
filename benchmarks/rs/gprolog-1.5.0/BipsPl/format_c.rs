use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Number_Check(start_word: WamWord) -> libc::c_double;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Chars_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Rd_Codes_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Math_Load_Value(start_word: WamWord, word_adr: *mut WamWord);
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_last_output_sora: WamWord;
    static mut pl_stm_output: libc::c_int;
    static mut pl_type_list: libc::c_int;
    static mut pl_domain_non_empty_list: libc::c_int;
    static mut pl_domain_format_control_sequence: libc::c_int;
    static mut pl_representation_character_code: libc::c_int;
    fn Pl_Err_Instantiation();
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn Pl_Err_Representation(atom_flag: libc::c_int);
    fn Pl_Write_Term(
        pstm: *mut StmInf,
        depth: libc::c_int,
        prec: libc::c_int,
        mask: libc::c_int,
        above_H: *mut WamWord,
        term_word: WamWord,
    );
    fn Pl_Check_Stream_Type(stm: libc::c_int, check_text: Bool, for_input: Bool);
    fn Pl_Get_Stream_Or_Alias(sora_word: WamWord, test_mask: libc::c_int) -> libc::c_int;
    fn Pl_Stream_Puts(str: *mut libc::c_char, pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_Printf(
        pstm: *mut StmInf,
        format: *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
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
pub unsafe extern "C" fn Pl_Format_3(
    mut sora_word: WamWord,
    mut format_word: WamWord,
    mut args_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buff: [libc::c_char; 2048] = [0; 2048];
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
    let mut deref_last_word: WamWord = 0;
    word = format_word;
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
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        && word as libc::c_ulong
            != (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
    {
        str = (*pl_atom_tbl.offset((word as PlULong >> 3 as libc::c_int) as isize)).name;
    } else {
        strcpy(buff.as_mut_ptr(), Pl_Rd_Codes_Check(format_word));
        str = buff.as_mut_ptr();
    }
    Format(pstm, str, &mut args_word);
}
pub unsafe extern "C" fn Pl_Format_2(mut format_word: WamWord, mut args_word: WamWord) {
    Pl_Format_3(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        format_word,
        args_word,
    );
}
unsafe extern "C" fn Format(
    mut pstm: *mut StmInf,
    mut format: *mut libc::c_char,
    mut lst_adr: *mut WamWord,
) {
    let mut word: WamWord = 0;
    let mut has_n: Bool = 0;
    let mut generic: PlLong = 0;
    let mut n: PlLong = 0;
    let mut n1: PlLong = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: PlLong = 0;
    let mut d: libc::c_double = 0.;
    let mut lg: libc::c_int = 0;
    let mut stop: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut format_stack: [*mut libc::c_char; 256] = [0 as *mut libc::c_char; 256];
    let mut top_stack: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut buff: [libc::c_char; 2048] = [0; 2048];
    top_stack = format_stack.as_mut_ptr();
    let fresh0 = top_stack;
    top_stack = top_stack.offset(1);
    *fresh0 = format;
    loop {
        top_stack = top_stack.offset(-1);
        format = *top_stack;
        let mut current_block_122: u64;
        while *format != 0 {
            if *format as libc::c_int == '%' as i32 {
                if *format.offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32
                {
                    Pl_Stream_Putc('%' as i32, pstm);
                    format = format.offset(2 as libc::c_int as isize);
                } else {
                    p = buff.as_mut_ptr();
                    n1 = -(12345678 as libc::c_int) as PlLong;
                    n = n1;
                    loop {
                        let fresh1 = format;
                        format = format.offset(1);
                        let fresh2 = p;
                        p = p.offset(1);
                        *fresh2 = *fresh1;
                        if *fresh2 as libc::c_int == '*' as i32 {
                            if n == -(12345678 as libc::c_int) as libc::c_long {
                                n = Arg_Integer(&mut lst_adr);
                            } else {
                                n1 = Arg_Integer(&mut lst_adr);
                            }
                        }
                        if !(strchr(
                            b"diouxXpnceEfgGs\0" as *const u8 as *const libc::c_char,
                            *p.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                        ))
                            .is_null()
                        {
                            break;
                        }
                    }
                    *p = '\0' as i32 as libc::c_char;
                    if (strchr(
                        b"eEfgG\0" as *const u8 as *const libc::c_char,
                        *p.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                    ))
                        .is_null()
                    {
                        generic = if *p.offset(-(1 as libc::c_int) as isize)
                            as libc::c_int == 's' as i32
                        {
                            Arg_Atom(&mut lst_adr) as PlLong
                        } else {
                            Arg_Integer(&mut lst_adr)
                        };
                        if n != -(12345678 as libc::c_int) as libc::c_long {
                            if n1 != -(12345678 as libc::c_int) as libc::c_long {
                                Pl_Stream_Printf(pstm, buff.as_mut_ptr(), n, n1, generic);
                            } else {
                                Pl_Stream_Printf(pstm, buff.as_mut_ptr(), n, generic);
                            }
                        } else {
                            Pl_Stream_Printf(pstm, buff.as_mut_ptr(), generic);
                        }
                    } else {
                        d = Arg_Float(&mut lst_adr);
                        if n != -(12345678 as libc::c_int) as libc::c_long {
                            if n1 != -(12345678 as libc::c_int) as libc::c_long {
                                Pl_Stream_Printf(pstm, buff.as_mut_ptr(), n, n1, d);
                            } else {
                                Pl_Stream_Printf(pstm, buff.as_mut_ptr(), n, d);
                            }
                        } else {
                            Pl_Stream_Printf(pstm, buff.as_mut_ptr(), d);
                        }
                    }
                }
            } else if *format as libc::c_int != '~' as i32 {
                Pl_Stream_Putc(*format as libc::c_int, pstm);
                format = format.offset(1);
                format;
            } else {
                format = format.offset(1);
                if *format as libc::c_int == '*' as i32 {
                    n = Arg_Integer(&mut lst_adr);
                    format = format.offset(1);
                    format;
                    has_n = 1 as libc::c_int;
                } else {
                    p = format;
                    n = strtol(format, &mut format, 10 as libc::c_int);
                    has_n = (format != p) as libc::c_int;
                }
                match *format as libc::c_int {
                    97 => {
                        p = Arg_Atom(&mut lst_adr);
                        if has_n != 0 {
                            Pl_Stream_Printf(
                                pstm,
                                b"%*s\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                -n,
                                p,
                            );
                        } else {
                            Pl_Stream_Puts(p, pstm);
                        }
                        current_block_122 = 13014351284863956202;
                    }
                    99 => {
                        x = Arg_Integer(&mut lst_adr);
                        if !((x as PlULong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                        {
                            Pl_Err_Representation(pl_representation_character_code);
                        }
                        loop {
                            Pl_Stream_Putc(x as libc::c_int, pstm);
                            n -= 1;
                            if !(n > 0 as libc::c_int as libc::c_long) {
                                break;
                            }
                        }
                        current_block_122 = 13014351284863956202;
                    }
                    101 | 69 | 102 | 103 | 71 => {
                        x = *format as PlLong;
                        d = Arg_Float(&mut lst_adr);
                        if has_n != 0 {
                            sprintf(
                                buff.as_mut_ptr(),
                                b"%%.%ld%c\0" as *const u8 as *const libc::c_char,
                                n,
                                x as libc::c_char as libc::c_int,
                            );
                        } else {
                            sprintf(
                                buff.as_mut_ptr(),
                                b"%%%c\0" as *const u8 as *const libc::c_char,
                                x as libc::c_char as libc::c_int,
                            );
                        }
                        Pl_Stream_Printf(pstm, buff.as_mut_ptr(), d);
                        current_block_122 = 13014351284863956202;
                    }
                    100 | 68 => {
                        x = Arg_Integer(&mut lst_adr);
                        if n == 0 as libc::c_int as libc::c_long
                            && *format as libc::c_int == 'd' as i32
                        {
                            Pl_Stream_Printf(
                                pstm,
                                b"%ld\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                x,
                            );
                        } else {
                            if x < 0 as libc::c_int as libc::c_long {
                                Pl_Stream_Putc('-' as i32, pstm);
                                x = -x;
                            }
                            sprintf(
                                buff.as_mut_ptr(),
                                b"%ld\0" as *const u8 as *const libc::c_char,
                                x,
                            );
                            lg = (strlen(buff.as_mut_ptr()))
                                .wrapping_sub(n as libc::c_ulong) as libc::c_int;
                            if lg <= 0 as libc::c_int {
                                Pl_Stream_Puts(
                                    b"0.\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    pstm,
                                );
                                i = 0 as libc::c_int;
                                while i < -lg {
                                    Pl_Stream_Putc('0' as i32, pstm);
                                    i += 1;
                                    i;
                                }
                                Pl_Stream_Printf(
                                    pstm,
                                    b"%ld\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    x,
                                );
                            } else {
                                stop = if *format as libc::c_int == 'D' as i32 {
                                    lg % 3 as libc::c_int
                                } else {
                                    -(1 as libc::c_int)
                                };
                                if stop == 0 as libc::c_int {
                                    stop = 3 as libc::c_int;
                                }
                                p = buff.as_mut_ptr();
                                i = 0 as libc::c_int;
                                while *p != 0 {
                                    if i == lg {
                                        Pl_Stream_Putc('.' as i32, pstm);
                                        stop = -(1 as libc::c_int);
                                    }
                                    if i == stop {
                                        Pl_Stream_Putc(',' as i32, pstm);
                                        stop += 3 as libc::c_int;
                                    }
                                    Pl_Stream_Putc(*p as libc::c_int, pstm);
                                    p = p.offset(1);
                                    p;
                                    i += 1;
                                    i;
                                }
                            }
                        }
                        current_block_122 = 13014351284863956202;
                    }
                    114 | 82 => {
                        x = Arg_Integer(&mut lst_adr);
                        if has_n == 0 || n < 2 as libc::c_int as libc::c_long
                            || n > 36 as libc::c_int as libc::c_long
                        {
                            n = 8 as libc::c_int as PlLong;
                        }
                        k = (if *format as libc::c_int == 'r' as i32 {
                            'a' as i32
                        } else {
                            'A' as i32
                        }) - 10 as libc::c_int;
                        if x < 0 as libc::c_int as libc::c_long {
                            Pl_Stream_Putc('-' as i32, pstm);
                            x = -x;
                        }
                        p = buff
                            .as_mut_ptr()
                            .offset(
                                ::std::mem::size_of::<[libc::c_char; 2048]>()
                                    as libc::c_ulong as isize,
                            )
                            .offset(-(1 as libc::c_int as isize));
                        *p = '\0' as i32 as libc::c_char;
                        loop {
                            i = (x % n) as libc::c_int;
                            x = x / n;
                            p = p.offset(-1);
                            p;
                            *p = (if i < 10 as libc::c_int {
                                i + '0' as i32
                            } else {
                                i + k
                            }) as libc::c_char;
                            if !(x != 0) {
                                break;
                            }
                        }
                        Pl_Stream_Puts(p, pstm);
                        current_block_122 = 13014351284863956202;
                    }
                    115 | 83 => {
                        word = Read_Arg(&mut lst_adr);
                        if *format as libc::c_int == 's' as i32 {
                            p = Pl_Rd_Codes_Check(word);
                        } else {
                            p = Pl_Rd_Chars_Check(word);
                        }
                        if has_n != 0 {
                            Pl_Stream_Printf(
                                pstm,
                                b"%-*.*s\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                n,
                                n,
                                p,
                            );
                        } else {
                            Pl_Stream_Printf(
                                pstm,
                                b"%s\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                p,
                            );
                        }
                        current_block_122 = 13014351284863956202;
                    }
                    105 => {
                        loop {
                            Read_Arg(&mut lst_adr);
                            n -= 1;
                            if !(n > 0 as libc::c_int as libc::c_long) {
                                break;
                            }
                        }
                        current_block_122 = 13014351284863956202;
                    }
                    107 => {
                        word = Read_Arg(&mut lst_adr);
                        Pl_Write_Term(
                            pstm,
                            -(1 as libc::c_int),
                            1200 as libc::c_int,
                            2 as libc::c_int | 1 as libc::c_int,
                            0 as *mut WamWord,
                            word,
                        );
                        current_block_122 = 13014351284863956202;
                    }
                    113 => {
                        word = Read_Arg(&mut lst_adr);
                        Pl_Write_Term(
                            pstm,
                            -(1 as libc::c_int),
                            1200 as libc::c_int,
                            4 as libc::c_int | 8 as libc::c_int | 1 as libc::c_int,
                            0 as *mut WamWord,
                            word,
                        );
                        current_block_122 = 13014351284863956202;
                    }
                    112 => {
                        word = Read_Arg(&mut lst_adr);
                        Pl_Write_Term(
                            pstm,
                            -(1 as libc::c_int),
                            1200 as libc::c_int,
                            4 as libc::c_int | 8 as libc::c_int | 32 as libc::c_int,
                            0 as *mut WamWord,
                            word,
                        );
                        current_block_122 = 13014351284863956202;
                    }
                    119 => {
                        word = Read_Arg(&mut lst_adr);
                        Pl_Write_Term(
                            pstm,
                            -(1 as libc::c_int),
                            1200 as libc::c_int,
                            4 as libc::c_int | 8 as libc::c_int,
                            0 as *mut WamWord,
                            word,
                        );
                        current_block_122 = 13014351284863956202;
                    }
                    126 => {
                        Pl_Stream_Putc('~' as i32, pstm);
                        current_block_122 = 13014351284863956202;
                    }
                    78 => {
                        if (*pstm).line_pos == 0 as libc::c_int as libc::c_long {
                            current_block_122 = 13014351284863956202;
                        } else {
                            current_block_122 = 3069832380127351737;
                        }
                    }
                    110 => {
                        current_block_122 = 3069832380127351737;
                    }
                    63 => {
                        if *format.offset(1 as libc::c_int as isize) != 0 {
                            let fresh3 = top_stack;
                            top_stack = top_stack.offset(1);
                            *fresh3 = format.offset(1 as libc::c_int as isize);
                        }
                        format = Arg_Atom(&mut lst_adr);
                        continue;
                    }
                    _ => {
                        Pl_Err_Domain(
                            pl_domain_format_control_sequence,
                            (((*format as libc::c_uchar as libc::c_int as PlLong)
                                << 3 as libc::c_int) as libc::c_ulong)
                                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord,
                        );
                        current_block_122 = 13014351284863956202;
                    }
                }
                match current_block_122 {
                    3069832380127351737 => {
                        loop {
                            Pl_Stream_Putc('\n' as i32, pstm);
                            n -= 1;
                            if !(n > 0 as libc::c_int as libc::c_long) {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
                format = format.offset(1);
                format;
            }
        }
        if !(top_stack > format_stack.as_mut_ptr()) {
            break;
        }
    };
}
unsafe extern "C" fn Read_Arg(mut lst_adr: *mut *mut WamWord) -> WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut car_word: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = **lst_adr;
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
    if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            Pl_Err_Instantiation();
        }
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            Pl_Err_Domain(pl_domain_non_empty_list, word);
        }
        Pl_Err_Type(pl_type_list, word);
    }
    adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord;
    car_word = *adr.offset(0 as libc::c_int as isize);
    *lst_adr = &mut *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize)
        as *mut WamWord;
    let mut deref_last_word_0: WamWord = 0;
    word = car_word;
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
    return word;
}
unsafe extern "C" fn Arg_Atom(mut lst_adr: *mut *mut WamWord) -> *mut libc::c_char {
    let mut word: WamWord = 0;
    word = Read_Arg(lst_adr);
    return (*pl_atom_tbl.offset(Pl_Rd_Atom_Check(word) as isize)).name;
}
unsafe extern "C" fn Arg_Integer(mut lst_adr: *mut *mut WamWord) -> PlLong {
    let mut word: WamWord = 0;
    word = Read_Arg(lst_adr);
    Pl_Math_Load_Value(word, &mut word);
    return Pl_Rd_Integer_Check(word);
}
unsafe extern "C" fn Arg_Float(mut lst_adr: *mut *mut WamWord) -> libc::c_double {
    let mut word: WamWord = 0;
    word = Read_Arg(lst_adr);
    Pl_Math_Load_Value(word, &mut word);
    return Pl_Rd_Number_Check(word);
}
