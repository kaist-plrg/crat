use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut pl_glob_buff: [libc::c_char; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_nb_atom: PlULong;
    static mut pl_atom_void: libc::c_int;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Gen_New_Atom(prefix: *mut libc::c_char) -> libc::c_int;
    fn Pl_Find_Next_Atom(last_atom: libc::c_int) -> libc::c_int;
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn Pl_Get_Atom_Tagged(w: WamWord, start_word: WamWord) -> Bool;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Number_Check(start_word: WamWord) -> libc::c_double;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Char_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Code_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_String_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Rd_Chars_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Rd_Codes_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Check_For_Un_Number(start_word: WamWord);
    fn Pl_Check_For_Un_Variable(start_word: WamWord);
    fn Pl_Un_Positive_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Char_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Un_Code_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Un_String_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Un_String(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Un_Chars_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Un_Codes_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_flag_syntax_error: *mut FlagInf;
    static mut pl_type_atom: libc::c_int;
    static mut pl_type_atomic: libc::c_int;
    static mut pl_type_integer: libc::c_int;
    static mut pl_domain_not_less_than_zero: libc::c_int;
    fn Pl_Set_Last_Syntax_Error(
        file_name: *mut libc::c_char,
        err_line: libc::c_int,
        err_col: libc::c_int,
        err_msg: *mut libc::c_char,
    );
    fn Pl_Syntax_Error(flag_value: libc::c_int);
    fn Pl_Err_Instantiation();
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn Pl_Read_Number(pstm: *mut StmInf) -> WamWord;
    fn Pl_Float_To_String(d: libc::c_double) -> *mut libc::c_char;
    fn Pl_Stream_Peekc(pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Add_Str_Stream(
        buff: *mut libc::c_char,
        prop_other: libc::c_int,
    ) -> libc::c_int;
    fn Pl_Delete_Str_Stream(stm: libc::c_int);
    fn X1_2461746F6D5F636F6E6361745F616C74__a0();
    fn X1_247375625F61746F6D5F616C74__a0();
    fn X1_2463757272656E745F61746F6D5F616C74__a0();
}
pub type __int32_t = libc::c_int;
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
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Atom_Length_2(
    mut atom_word: WamWord,
    mut length_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = 0;
    atom = Pl_Rd_Atom_Check(atom_word);
    return Pl_Un_Positive_Check(
        ((*pl_atom_tbl.offset(atom as isize)).prop).length() as PlLong,
        length_word,
    );
}
pub unsafe extern "C" fn Pl_New_Atom_2(
    mut prefix_word: WamWord,
    mut atom_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = 0;
    atom = Pl_Rd_Atom_Check(prefix_word);
    Pl_Check_For_Un_Variable(atom_word);
    return Pl_Get_Atom(
        Pl_Gen_New_Atom((*pl_atom_tbl.offset(atom as isize)).name),
        atom_word,
    );
}
pub unsafe extern "C" fn Pl_Atom_Concat_3(
    mut atom1_word: WamWord,
    mut atom2_word: WamWord,
    mut atom3_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut tag1: libc::c_int = 0;
    let mut tag2: libc::c_int = 0;
    let mut tag3: libc::c_int = 0;
    let mut patom1: *mut AtomInf = 0 as *mut AtomInf;
    let mut patom2: *mut AtomInf = 0 as *mut AtomInf;
    let mut patom3: *mut AtomInf = 0 as *mut AtomInf;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = atom1_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_atom, atom1_word);
    }
    tag1 = tag_mask as libc::c_int;
    atom1_word = word;
    let mut deref_last_word_0: WamWord = 0;
    word = atom2_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_atom, atom2_word);
    }
    tag2 = tag_mask as libc::c_int;
    atom2_word = word;
    let mut deref_last_word_1: WamWord = 0;
    word = atom3_word;
    loop {
        deref_last_word_1 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_1) {
            break;
        }
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_atom, atom3_word);
    }
    tag3 = tag_mask as libc::c_int;
    atom3_word = word;
    if tag3 as libc::c_ulong == 0 as libc::c_int as PlULong
        && (tag1 as libc::c_ulong == 0 as libc::c_int as PlULong
            || tag2 as libc::c_ulong == 0 as libc::c_int as PlULong)
    {
        Pl_Err_Instantiation();
    }
    if tag1 as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        patom1 = pl_atom_tbl
            .offset((atom1_word as PlULong >> 3 as libc::c_int) as isize);
        if tag2 as libc::c_ulong == 0x3 as libc::c_int as PlULong {
            patom2 = pl_atom_tbl
                .offset((atom2_word as PlULong >> 3 as libc::c_int) as isize);
            l = ((*patom1).prop).length() as libc::c_int
                + ((*patom2).prop).length() as libc::c_int;
            if l < 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            str = Pl_Malloc_Check(
                (l + 1 as libc::c_int) as libc::c_uint,
                b"atom_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                178 as libc::c_int,
            );
            strcpy(str, (*patom1).name);
            strcpy(
                str.offset(((*patom1).prop).length() as libc::c_int as isize),
                (*patom2).name,
            );
            return Pl_Get_Atom(Create_Malloc_Atom(str), atom3_word);
        }
        patom3 = pl_atom_tbl
            .offset((atom3_word as PlULong >> 3 as libc::c_int) as isize);
        l = ((*patom3).prop).length() as libc::c_int
            - ((*patom1).prop).length() as libc::c_int;
        if l < 0 as libc::c_int
            || strncmp(
                (*patom1).name,
                (*patom3).name,
                ((*patom1).prop).length() as libc::c_ulong,
            ) != 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if l < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        str = Pl_Malloc_Check(
            (l + 1 as libc::c_int) as libc::c_uint,
            b"atom_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            188 as libc::c_int,
        );
        strcpy(
            str,
            ((*patom3).name).offset(((*patom1).prop).length() as libc::c_int as isize),
        );
        return Pl_Get_Atom(Create_Malloc_Atom(str), atom2_word);
    }
    if tag2 as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        patom2 = pl_atom_tbl
            .offset((atom2_word as PlULong >> 3 as libc::c_int) as isize);
        patom3 = pl_atom_tbl
            .offset((atom3_word as PlULong >> 3 as libc::c_int) as isize);
        l = ((*patom3).prop).length() as libc::c_int
            - ((*patom2).prop).length() as libc::c_int;
        if l < 0 as libc::c_int
            || strncmp(
                (*patom2).name,
                ((*patom3).name).offset(l as isize),
                ((*patom2).prop).length() as libc::c_ulong,
            ) != 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if l < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        str = Pl_Malloc_Check(
            (l + 1 as libc::c_int) as libc::c_uint,
            b"atom_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            202 as libc::c_int,
        );
        strncpy(str, (*patom3).name, l as libc::c_ulong);
        *str.offset(l as isize) = '\0' as i32 as libc::c_char;
        return Pl_Get_Atom(Create_Malloc_Atom(str), atom1_word);
    }
    patom3 = pl_atom_tbl.offset((atom3_word as PlULong >> 3 as libc::c_int) as isize);
    if ((*patom3).prop).length() as libc::c_int > 0 as libc::c_int {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = atom1_word;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = atom2_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = patom3 as WamWord;
        *pl_reg_bank
            .offset(
                3 as libc::c_int as isize,
            ) = ((*patom3).name).offset(1 as libc::c_int as isize) as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2461746F6D5F636F6E6361745F616C74__a0),
                ),
            ),
            4 as libc::c_int,
        );
    }
    return (Pl_Get_Atom(pl_atom_void, atom1_word) != 0
        && Pl_Get_Atom_Tagged(atom3_word, atom2_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Atom_Concat_Alt_0() -> Bool {
    let mut atom1_word: WamWord = 0;
    let mut atom2_word: WamWord = 0;
    let mut patom3: *mut AtomInf = 0 as *mut AtomInf;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2461746F6D5F636F6E6361745F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    atom1_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    atom2_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    patom3 = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) as *mut AtomInf;
    p = *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
        as *mut WamWord) as *mut libc::c_char;
    if *p as libc::c_int == '\0' as i32 {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    } else {
        *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
            as *mut WamWord) = p.offset(1 as libc::c_int as isize) as WamWord;
    }
    name = (*patom3).name;
    l = p.offset_from(name) as libc::c_long as libc::c_int;
    if l < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    str = Pl_Malloc_Check(
        (l + 1 as libc::c_int) as libc::c_uint,
        b"atom_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        265 as libc::c_int,
    );
    strncpy(str, name, (l + 1 as libc::c_int) as libc::c_ulong);
    *str.offset(l as isize) = '\0' as i32 as libc::c_char;
    if Pl_Get_Atom(Create_Malloc_Atom(str), atom1_word) == 0 {
        return 0 as libc::c_int;
    }
    l = ((*patom3).prop).length() as libc::c_int - l;
    if l < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    str = Pl_Malloc_Check(
        (l + 1 as libc::c_int) as libc::c_uint,
        b"atom_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        272 as libc::c_int,
    );
    strcpy(str, p);
    return Pl_Get_Atom(Create_Malloc_Atom(str), atom2_word);
}
pub unsafe extern "C" fn Pl_Sub_Atom_5(
    mut atom_word: WamWord,
    mut before_word: WamWord,
    mut length_word: WamWord,
    mut after_word: WamWord,
    mut sub_atom_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut patom: *mut AtomInf = 0 as *mut AtomInf;
    let mut psub_atom: *mut AtomInf = 0 as *mut AtomInf;
    let mut length: libc::c_int = 0;
    let mut b: PlLong = 0;
    let mut l: PlLong = 0;
    let mut a: PlLong = 0;
    let mut b1: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut a1: libc::c_int = 0;
    let mut nondet: Bool = 0;
    let mut mask: libc::c_int = 0 as libc::c_int;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    patom = pl_atom_tbl.offset(Pl_Rd_Atom_Check(atom_word) as isize);
    length = ((*patom).prop).length() as libc::c_int;
    let mut deref_last_word: WamWord = 0;
    word = before_word;
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
    mask <<= 1 as libc::c_int;
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        b = word << 0 as libc::c_int >> 3 as libc::c_int;
        if b < 0 as libc::c_int as libc::c_long {
            Pl_Err_Domain(pl_domain_not_less_than_zero, word);
        }
        mask |= 1 as libc::c_int;
    } else {
        b = 0 as libc::c_int as PlLong;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_integer, word);
        }
    }
    before_word = word;
    let mut deref_last_word_0: WamWord = 0;
    word = length_word;
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
    mask <<= 1 as libc::c_int;
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        l = word << 0 as libc::c_int >> 3 as libc::c_int;
        if l < 0 as libc::c_int as libc::c_long {
            Pl_Err_Domain(pl_domain_not_less_than_zero, word);
        }
        mask |= 1 as libc::c_int;
    } else {
        l = 0 as libc::c_int as PlLong;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_integer, word);
        }
    }
    length_word = word;
    let mut deref_last_word_1: WamWord = 0;
    word = after_word;
    loop {
        deref_last_word_1 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_1) {
            break;
        }
    }
    mask <<= 1 as libc::c_int;
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        a = word << 0 as libc::c_int >> 3 as libc::c_int;
        if a < 0 as libc::c_int as libc::c_long {
            Pl_Err_Domain(pl_domain_not_less_than_zero, word);
        }
        mask |= 1 as libc::c_int;
    } else {
        a = 0 as libc::c_int as PlLong;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_integer, word);
        }
    }
    after_word = word;
    let mut deref_last_word_2: WamWord = 0;
    word = sub_atom_word;
    loop {
        deref_last_word_2 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_2) {
            break;
        }
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_atom, word);
    }
    sub_atom_word = word;
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        psub_atom = pl_atom_tbl.offset((word as PlULong >> 3 as libc::c_int) as isize);
        l = ((*psub_atom).prop).length() as PlLong;
        if Pl_Get_Integer(l, length_word) == 0 {
            return 0 as libc::c_int;
        }
        if mask & 5 as libc::c_int == 5 as libc::c_int
            && length as libc::c_long != b + l + a
        {
            return 0 as libc::c_int;
        }
        if mask & 4 as libc::c_int != 0 {
            a = length as libc::c_long - b - l;
            return (strncmp(
                ((*patom).name).offset(b as isize),
                (*psub_atom).name,
                l as libc::c_ulong,
            ) == 0 as libc::c_int && Pl_Get_Integer(a, after_word) != 0) as libc::c_int;
        }
        if mask & 1 as libc::c_int != 0 {
            b = length as libc::c_long - l - a;
            return (strncmp(
                ((*patom).name).offset(b as isize),
                (*psub_atom).name,
                l as libc::c_ulong,
            ) == 0 as libc::c_int && Pl_Get_Integer(b, before_word) != 0) as libc::c_int;
        }
        mask = 8 as libc::c_int;
    }
    let mut current_block_105: u64;
    match mask {
        0 => {
            current_block_105 = 1197948843457537213;
        }
        2 => {
            current_block_105 = 1197948843457537213;
        }
        4 => {
            current_block_105 = 258419738100199176;
        }
        1 => {
            l = length as libc::c_long - b - a;
            nondet = 1 as libc::c_int;
            current_block_105 = 6014157347423944569;
        }
        3 => {
            b = length as libc::c_long - l - a;
            nondet = 0 as libc::c_int;
            current_block_105 = 6014157347423944569;
        }
        5 => {
            l = length as libc::c_long - b - a;
            nondet = 0 as libc::c_int;
            current_block_105 = 6014157347423944569;
        }
        6 | 7 => {
            a = length as libc::c_long - b - l;
            nondet = 0 as libc::c_int;
            current_block_105 = 6014157347423944569;
        }
        _ => {
            str = strstr(((*patom).name).offset(b as isize), (*psub_atom).name);
            if str.is_null() {
                return 0 as libc::c_int;
            }
            b = str.offset_from((*patom).name) as libc::c_long;
            a = length as libc::c_long - b - l;
            nondet = 1 as libc::c_int;
            current_block_105 = 6014157347423944569;
        }
    }
    match current_block_105 {
        1197948843457537213 => {
            current_block_105 = 258419738100199176;
        }
        _ => {}
    }
    match current_block_105 {
        258419738100199176 => {
            a = length as libc::c_long - b - l;
            nondet = 1 as libc::c_int;
        }
        _ => {}
    }
    if b < 0 as libc::c_int as libc::c_long || l < 0 as libc::c_int as libc::c_long
        || a < 0 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int;
    }
    if nondet != 0
        && Compute_Next_BLA(
            mask,
            patom,
            psub_atom,
            b as libc::c_int,
            l as libc::c_int,
            a as libc::c_int,
            &mut b1,
            &mut l1,
            &mut a1,
        ) != 0
    {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = before_word;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = length_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = after_word;
        *pl_reg_bank.offset(3 as libc::c_int as isize) = sub_atom_word;
        *pl_reg_bank.offset(4 as libc::c_int as isize) = patom as WamWord;
        *pl_reg_bank.offset(5 as libc::c_int as isize) = psub_atom as WamWord;
        *pl_reg_bank.offset(6 as libc::c_int as isize) = mask as WamWord;
        *pl_reg_bank.offset(7 as libc::c_int as isize) = b1 as WamWord;
        *pl_reg_bank.offset(8 as libc::c_int as isize) = l1 as WamWord;
        *pl_reg_bank.offset(9 as libc::c_int as isize) = a1 as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_247375625F61746F6D5F616C74__a0),
                ),
            ),
            10 as libc::c_int,
        );
    }
    if mask <= 7 as libc::c_int {
        if l < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int;
        }
        str = Pl_Malloc_Check(
            (l + 1 as libc::c_int as libc::c_long) as libc::c_uint,
            b"atom_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            420 as libc::c_int,
        );
        strncpy(str, ((*patom).name).offset(b as isize), l as libc::c_ulong);
        *str.offset(l as isize) = '\0' as i32 as libc::c_char;
        Pl_Get_Atom(Create_Malloc_Atom(str), sub_atom_word);
        Pl_Get_Integer(l, length_word);
    }
    return (Pl_Get_Integer(b, before_word) != 0 && Pl_Get_Integer(a, after_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Sub_Atom_Alt_0() -> Bool {
    let mut before_word: WamWord = 0;
    let mut length_word: WamWord = 0;
    let mut after_word: WamWord = 0;
    let mut sub_atom_word: WamWord = 0;
    let mut patom: *mut AtomInf = 0 as *mut AtomInf;
    let mut psub_atom: *mut AtomInf = 0 as *mut AtomInf;
    let mut b: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut a1: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_247375625F61746F6D5F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    before_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    length_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    after_word = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord);
    sub_atom_word = *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
        as *mut WamWord);
    patom = *(&mut *B.offset((-(9 as libc::c_int) - 4 as libc::c_int) as isize)
        as *mut WamWord) as *mut AtomInf;
    psub_atom = *(&mut *B.offset((-(9 as libc::c_int) - 5 as libc::c_int) as isize)
        as *mut WamWord) as *mut AtomInf;
    mask = *(&mut *B.offset((-(9 as libc::c_int) - 6 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    b = *(&mut *B.offset((-(9 as libc::c_int) - 7 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    l = *(&mut *B.offset((-(9 as libc::c_int) - 8 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    a = *(&mut *B.offset((-(9 as libc::c_int) - 9 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    if Compute_Next_BLA(mask, patom, psub_atom, b, l, a, &mut b1, &mut l1, &mut a1) == 0
    {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh1 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh1 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    } else {
        *(&mut *B.offset((-(9 as libc::c_int) - 7 as libc::c_int) as isize)
            as *mut WamWord) = b1 as WamWord;
        *(&mut *B.offset((-(9 as libc::c_int) - 8 as libc::c_int) as isize)
            as *mut WamWord) = l1 as WamWord;
        *(&mut *B.offset((-(9 as libc::c_int) - 9 as libc::c_int) as isize)
            as *mut WamWord) = a1 as WamWord;
    }
    if mask <= 7 as libc::c_int {
        if l < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        str = Pl_Malloc_Check(
            (l + 1 as libc::c_int) as libc::c_uint,
            b"atom_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            482 as libc::c_int,
        );
        strncpy(str, ((*patom).name).offset(b as isize), l as libc::c_ulong);
        *str.offset(l as isize) = '\0' as i32 as libc::c_char;
        Pl_Get_Atom(Create_Malloc_Atom(str), sub_atom_word);
        Pl_Get_Integer(l as PlLong, length_word);
    }
    return (Pl_Get_Integer(b as PlLong, before_word) != 0
        && Pl_Get_Integer(a as PlLong, after_word) != 0) as libc::c_int;
}
unsafe extern "C" fn Compute_Next_BLA(
    mut mask: libc::c_int,
    mut patom: *mut AtomInf,
    mut psub_atom: *mut AtomInf,
    mut b: libc::c_int,
    mut l: libc::c_int,
    mut a: libc::c_int,
    mut b1: *mut libc::c_int,
    mut l1: *mut libc::c_int,
    mut a1: *mut libc::c_int,
) -> Bool {
    let mut length: libc::c_int = ((*patom).prop).length() as libc::c_int;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    match mask {
        0 => {
            l += 1;
            if l > length - b {
                l = 0 as libc::c_int;
                b += 1;
                if b > length {
                    return 0 as libc::c_int;
                }
            }
            a = length - b - l;
        }
        1 => {
            b += 1;
            if b > length - a {
                return 0 as libc::c_int;
            }
            l = length - b - a;
        }
        2 => {
            b += 1;
            if b > length - l {
                return 0 as libc::c_int;
            }
            a = length - b - l;
        }
        4 => {
            l += 1;
            if l > length - b {
                return 0 as libc::c_int;
            }
            a = length - b - l;
        }
        _ => {
            b += 1;
            if b > length - l {
                return 0 as libc::c_int;
            }
            str = strstr(((*patom).name).offset(b as isize), (*psub_atom).name);
            if str.is_null() {
                return 0 as libc::c_int;
            }
            b = str.offset_from((*patom).name) as libc::c_long as libc::c_int;
            a = length - b - l;
        }
    }
    *b1 = b;
    *l1 = l;
    *a1 = a;
    return 1 as libc::c_int;
}
unsafe extern "C" fn Create_Malloc_Atom(mut str: *mut libc::c_char) -> libc::c_int {
    let mut atom: libc::c_int = 0;
    let mut nb: libc::c_int = pl_nb_atom as libc::c_int;
    atom = Pl_Create_Atom(str);
    if nb as libc::c_ulong == pl_nb_atom {
        free(str as *mut libc::c_void);
    }
    return atom;
}
pub unsafe extern "C" fn Pl_Atom_Chars_2(
    mut atom_word: WamWord,
    mut chars_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = atom_word;
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
        return Pl_Un_Chars_Check(Pl_Rd_String_Check(word), chars_word);
    }
    return Pl_Un_String_Check(Pl_Rd_Chars_Check(chars_word), atom_word);
}
pub unsafe extern "C" fn Pl_Atom_Codes_2(
    mut atom_word: WamWord,
    mut codes_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = atom_word;
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
        return Pl_Un_Codes_Check(Pl_Rd_String_Check(word), codes_word);
    }
    return Pl_Un_String_Check(Pl_Rd_Codes_Check(codes_word), atom_word);
}
pub unsafe extern "C" fn Pl_Number_Atom_2(
    mut number_word: WamWord,
    mut atom_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut deref_last_word: WamWord = 0;
    word = atom_word;
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
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        return String_To_Number(
            (*pl_atom_tbl.offset((word as PlULong >> 3 as libc::c_int) as isize)).name,
            number_word,
        );
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_atom, word);
    }
    let mut deref_last_word_0: WamWord = 0;
    word = number_word;
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
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        sprintf(
            pl_glob_buff.as_mut_ptr(),
            b"%ld\0" as *const u8 as *const libc::c_char,
            word << 0 as libc::c_int >> 3 as libc::c_int,
        );
        return Pl_Un_String_Check(pl_glob_buff.as_mut_ptr(), atom_word);
    }
    str = Pl_Float_To_String(Pl_Rd_Number_Check(word));
    return Pl_Un_String_Check(str, atom_word);
}
pub unsafe extern "C" fn Pl_Number_Chars_2(
    mut number_word: WamWord,
    mut chars_word: WamWord,
) -> Bool {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut list_word: WamWord = 0;
    let mut str: *mut libc::c_char = pl_glob_buff.as_mut_ptr();
    let mut atom: libc::c_int = 0;
    list_word = chars_word;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = list_word;
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
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            current_block = 15125582407903384992;
            break;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            current_block = 7593243920211945454;
            break;
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
        atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || ((*pl_atom_tbl.offset(atom as isize)).prop).length() as libc::c_int
                != 1 as libc::c_int
        {
            current_block = 7593243920211945454;
            break;
        }
        let fresh2 = str;
        str = str.offset(1);
        *fresh2 = *((*pl_atom_tbl.offset(atom as isize)).name)
            .offset(0 as libc::c_int as isize);
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    match current_block {
        7593243920211945454 => {
            let mut deref_last_word_1: WamWord = 0;
            word = number_word;
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
            if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
                sprintf(
                    pl_glob_buff.as_mut_ptr(),
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    word << 0 as libc::c_int >> 3 as libc::c_int,
                );
                return Pl_Un_Chars_Check(pl_glob_buff.as_mut_ptr(), chars_word);
            }
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                str = Pl_Float_To_String(Pl_Rd_Number_Check(word));
                return Pl_Un_Chars_Check(str, chars_word);
            }
            Pl_Rd_Chars_Check(chars_word);
            return 0 as libc::c_int;
        }
        _ => {
            *str = '\0' as i32 as libc::c_char;
            return String_To_Number(pl_glob_buff.as_mut_ptr(), number_word);
        }
    };
}
pub unsafe extern "C" fn Pl_Number_Codes_2(
    mut number_word: WamWord,
    mut codes_word: WamWord,
) -> Bool {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut list_word: WamWord = 0;
    let mut str: *mut libc::c_char = pl_glob_buff.as_mut_ptr();
    let mut c: PlLong = 0;
    list_word = codes_word;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = list_word;
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
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            current_block = 15125582407903384992;
            break;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            current_block = 15635990770605955751;
            break;
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
        c = word << 0 as libc::c_int >> 3 as libc::c_int;
        if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
            || !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        {
            current_block = 15635990770605955751;
            break;
        }
        let fresh3 = str;
        str = str.offset(1);
        *fresh3 = c as libc::c_char;
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    match current_block {
        15635990770605955751 => {
            let mut deref_last_word_1: WamWord = 0;
            word = number_word;
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
            if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
                sprintf(
                    pl_glob_buff.as_mut_ptr(),
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    word << 0 as libc::c_int >> 3 as libc::c_int,
                );
                return Pl_Un_Codes_Check(pl_glob_buff.as_mut_ptr(), codes_word);
            }
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                str = Pl_Float_To_String(Pl_Rd_Number_Check(word));
                return Pl_Un_Codes_Check(str, codes_word);
            }
            Pl_Rd_Codes_Check(codes_word);
            return 0 as libc::c_int;
        }
        _ => {
            *str = '\0' as i32 as libc::c_char;
            return String_To_Number(pl_glob_buff.as_mut_ptr(), number_word);
        }
    };
}
pub unsafe extern "C" fn Pl_Char_Code_2(
    mut char_word: WamWord,
    mut code_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = char_word;
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
        return Pl_Un_Code_Check(Pl_Rd_Char_Check(word), code_word);
    }
    return Pl_Un_Char_Check(Pl_Rd_Code_Check(code_word), char_word);
}
pub unsafe extern "C" fn Pl_Name_2(
    mut atomic_word: WamWord,
    mut codes_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut syn_flag: libc::c_int = 0;
    let mut is_number: Bool = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut deref_last_word: WamWord = 0;
    word = atomic_word;
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
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        return Pl_Atom_Codes_2(word, codes_word);
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x4 as libc::c_int as PlULong
    {
        return Pl_Number_Codes_2(word, codes_word);
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_atomic, word);
    }
    str = Pl_Rd_Codes_Check(codes_word);
    syn_flag = (*pl_flag_syntax_error).value as libc::c_int;
    (*pl_flag_syntax_error).value = 2 as libc::c_int as PlLong;
    is_number = String_To_Number(str, word);
    (*pl_flag_syntax_error).value = syn_flag as PlLong;
    if is_number != 0 {
        return 1 as libc::c_int;
    }
    return Pl_Un_String(str, word);
}
pub unsafe extern "C" fn Pl_Lower_Upper_2(
    mut lower_word: WamWord,
    mut upper_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = lower_word;
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
        return Pl_Un_Char_Check(
            ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = Pl_Rd_Char_Check(word);
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(Pl_Rd_Char_Check(word));
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(Pl_Rd_Char_Check(word) as isize);
                }
                __res
            }),
            upper_word,
        );
    }
    return Pl_Un_Char_Check(
        ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = Pl_Rd_Char_Check(upper_word);
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(Pl_Rd_Char_Check(upper_word));
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(Pl_Rd_Char_Check(upper_word) as isize);
            }
            __res
        }),
        lower_word,
    );
}
unsafe extern "C" fn String_To_Number(
    mut str: *mut libc::c_char,
    mut number_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut eof: Bool = 0;
    Pl_Check_For_Un_Number(number_word);
    stm = Pl_Add_Str_Stream(str, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    word = Pl_Read_Number(pstm);
    eof = (Pl_Stream_Peekc(pstm) == -(1 as libc::c_int)) as libc::c_int;
    if word as libc::c_ulong
        != (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) && eof == 0
    {
        Pl_Set_Last_Syntax_Error(
            (*pl_atom_tbl.offset((*pstm).atom_file_name as isize)).name,
            ((*pstm).line_count + 1 as libc::c_int as libc::c_long) as libc::c_int,
            ((*pstm).line_pos + 1 as libc::c_int as libc::c_long) as libc::c_int,
            b"non numeric character\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    Pl_Delete_Str_Stream(stm);
    if word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) || eof == 0
    {
        Pl_Syntax_Error((*pl_flag_syntax_error).value as libc::c_int);
        return 0 as libc::c_int;
    }
    return Pl_Unify(word, number_word);
}
pub unsafe extern "C" fn Pl_Current_Atom_2(
    mut atom_word: WamWord,
    mut hide_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut hide: Bool = 0;
    let mut atom: libc::c_int = 0;
    hide = Pl_Rd_Integer_Check(hide_word) as Bool;
    let mut deref_last_word: WamWord = 0;
    word = atom_word;
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
        return (*Pl_Rd_String_Check(word) as libc::c_int != '$' as i32 || hide == 0)
            as libc::c_int;
    }
    atom = -(1 as libc::c_int);
    loop {
        atom = Pl_Find_Next_Atom(atom);
        if atom == -(1 as libc::c_int) {
            return 0 as libc::c_int;
        }
        if hide == 0
            || *((*pl_atom_tbl.offset(atom as isize)).name)
                .offset(0 as libc::c_int as isize) as libc::c_int != '$' as i32
        {
            break;
        }
    }
    *pl_reg_bank.offset(0 as libc::c_int as isize) = atom_word;
    *pl_reg_bank.offset(1 as libc::c_int as isize) = hide as WamWord;
    *pl_reg_bank.offset(2 as libc::c_int as isize) = atom as WamWord;
    Pl_Create_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F61746F6D5F616C74__a0),
            ),
        ),
        3 as libc::c_int,
    );
    return Pl_Get_Atom(atom, atom_word);
}
pub unsafe extern "C" fn Pl_Current_Atom_Alt_0() -> Bool {
    let mut atom_word: WamWord = 0;
    let mut hide: Bool = 0;
    let mut atom: libc::c_int = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F61746F6D5F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    atom_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    hide = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord) as Bool;
    atom = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    loop {
        atom = Pl_Find_Next_Atom(atom);
        if atom == -(1 as libc::c_int) {
            B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
            let ref mut fresh4 = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
            *fresh4 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
            return 0 as libc::c_int;
        }
        if hide == 0
            || *((*pl_atom_tbl.offset(atom as isize)).name)
                .offset(0 as libc::c_int as isize) as libc::c_int != '$' as i32
        {
            break;
        }
    }
    *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) = atom as WamWord;
    return Pl_Get_Atom(atom, atom_word);
}
pub unsafe extern "C" fn Pl_Atom_Property_6(
    mut atom_word: WamWord,
    mut prefix_op_word: WamWord,
    mut infix_op_word: WamWord,
    mut postfix_op_word: WamWord,
    mut needs_quote_word: WamWord,
    mut needs_scan_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = atom_word;
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
    atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
    Pl_Get_Integer(
        (((*pl_atom_tbl.offset(atom as isize)).prop).op_mask() as libc::c_int
            & (1 as libc::c_int) << 0 as libc::c_int != 0 as libc::c_int) as libc::c_int
            as PlLong,
        prefix_op_word,
    );
    Pl_Get_Integer(
        (((*pl_atom_tbl.offset(atom as isize)).prop).op_mask() as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int != 0 as libc::c_int) as libc::c_int
            as PlLong,
        infix_op_word,
    );
    Pl_Get_Integer(
        (((*pl_atom_tbl.offset(atom as isize)).prop).op_mask() as libc::c_int
            & (1 as libc::c_int) << 1 as libc::c_int != 0 as libc::c_int) as libc::c_int
            as PlLong,
        postfix_op_word,
    );
    Pl_Get_Integer(
        ((*pl_atom_tbl.offset(atom as isize)).prop).needs_quote() as PlLong,
        needs_quote_word,
    );
    Pl_Get_Integer(
        ((*pl_atom_tbl.offset(atom as isize)).prop).needs_scan() as PlLong,
        needs_scan_word,
    );
}
