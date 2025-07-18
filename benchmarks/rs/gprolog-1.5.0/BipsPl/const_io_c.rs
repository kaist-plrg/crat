use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Strdup_Check(
        str: *mut libc::c_char,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Chars_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Rd_Codes_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Un_String_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Un_Chars_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Un_Codes_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Mk_Variable() -> WamWord;
    static mut pl_sys_var: [PlLong; 0];
    fn Pl_Make_Stream_Tagged_Word(stm: libc::c_int) -> WamWord;
    fn Pl_Add_Str_Stream(
        buff: *mut libc::c_char,
        prop_other: libc::c_int,
    ) -> libc::c_int;
    fn Pl_Delete_Str_Stream(stm: libc::c_int);
    fn Pl_Term_Write_Str_Stream(stm: libc::c_int) -> *mut libc::c_char;
    fn Pl_Write_Term_2(sora_word: WamWord, term_word: WamWord);
    fn Pl_Write_2(sora_word: WamWord, term_word: WamWord);
    fn Pl_Writeq_2(sora_word: WamWord, term_word: WamWord);
    fn Pl_Write_Canonical_2(sora_word: WamWord, term_word: WamWord);
    fn Pl_Display_2(sora_word: WamWord, term_word: WamWord);
    fn Pl_Print_2(sora_word: WamWord, term_word: WamWord);
    fn Pl_Format_3(sora_word: WamWord, format_word: WamWord, args_word: WamWord);
    fn Pl_Read_Term_5(
        sora_word: WamWord,
        term_word: WamWord,
        vars_word: WamWord,
        var_names_word: WamWord,
        sing_names_word: WamWord,
    ) -> Bool;
    fn Pl_Read_Token_2(sora_word: WamWord, token_word: WamWord) -> Bool;
}
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Write_To_String(
    mut term_word: WamWord,
) -> *mut libc::c_char {
    let mut ret_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret_str = Pl_Strdup_Check(
        str,
        b"const_io_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        129 as libc::c_int,
    );
    Pl_Delete_Str_Stream(stm);
    return ret_str;
}
pub unsafe extern "C" fn Pl_Writeq_To_String(
    mut term_word: WamWord,
) -> *mut libc::c_char {
    let mut ret_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Writeq_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret_str = Pl_Strdup_Check(
        str,
        b"const_io_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        148 as libc::c_int,
    );
    Pl_Delete_Str_Stream(stm);
    return ret_str;
}
pub unsafe extern "C" fn Pl_Write_Canonical_To_String(
    mut term_word: WamWord,
) -> *mut libc::c_char {
    let mut ret_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_Canonical_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret_str = Pl_Strdup_Check(
        str,
        b"const_io_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        167 as libc::c_int,
    );
    Pl_Delete_Str_Stream(stm);
    return ret_str;
}
pub unsafe extern "C" fn Pl_Display_To_String(
    mut term_word: WamWord,
) -> *mut libc::c_char {
    let mut ret_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Display_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret_str = Pl_Strdup_Check(
        str,
        b"const_io_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        186 as libc::c_int,
    );
    Pl_Delete_Str_Stream(stm);
    return ret_str;
}
pub unsafe extern "C" fn Pl_Write_To_Atom_2(
    mut atom_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_String_Check(str, atom_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Write_To_Chars_2(
    mut chars_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Chars_Check(str, chars_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Write_To_Codes_2(
    mut codes_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Codes_Check(str, codes_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Writeq_To_Atom_2(
    mut atom_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Writeq_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_String_Check(str, atom_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Writeq_To_Chars_2(
    mut chars_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Writeq_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Chars_Check(str, chars_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Writeq_To_Codes_2(
    mut codes_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Writeq_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Codes_Check(str, codes_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Write_Canonical_To_Atom_2(
    mut atom_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_Canonical_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_String_Check(str, atom_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Write_Canonical_To_Chars_2(
    mut chars_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_Canonical_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Chars_Check(str, chars_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Write_Canonical_To_Codes_2(
    mut codes_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_Canonical_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Codes_Check(str, codes_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Display_To_Atom_2(
    mut atom_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Display_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_String_Check(str, atom_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Display_To_Chars_2(
    mut chars_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Display_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Chars_Check(str, chars_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Display_To_Codes_2(
    mut codes_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Display_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Codes_Check(str, codes_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Print_To_Atom_2(
    mut atom_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Print_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_String_Check(str, atom_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Print_To_Chars_2(
    mut chars_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Print_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Chars_Check(str, chars_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Print_To_Codes_2(
    mut codes_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Print_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Codes_Check(str, codes_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Write_Term_To_Atom_2(
    mut atom_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_Term_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_String_Check(str, atom_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Write_Term_To_Chars_2(
    mut chars_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_Term_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Chars_Check(str, chars_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Write_Term_To_Codes_2(
    mut codes_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Write_Term_2(stm_word, term_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Codes_Check(str, codes_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Format_To_Atom_3(
    mut atom_word: WamWord,
    mut format_word: WamWord,
    mut args_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Format_3(stm_word, format_word, args_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_String_Check(str, atom_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Format_To_Chars_3(
    mut chars_word: WamWord,
    mut format_word: WamWord,
    mut args_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Format_3(stm_word, format_word, args_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Chars_Check(str, chars_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Format_To_Codes_3(
    mut codes_word: WamWord,
    mut format_word: WamWord,
    mut args_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Add_Str_Stream(0 as *mut libc::c_char, 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Format_3(stm_word, format_word, args_word);
    str = Pl_Term_Write_Str_Stream(stm);
    ret = Pl_Un_Codes_Check(str, codes_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Read_From_String(mut str: *mut libc::c_char) -> WamWord {
    let mut term_word: WamWord = Pl_Mk_Variable();
    *pl_sys_var
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as PlLong;
    let ref mut fresh0 = *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh0 |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_long;
    *pl_sys_var
        .as_mut_ptr()
        .offset(1 as libc::c_int as isize) = -(1 as libc::c_int) as PlLong;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(str, 1 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    Pl_Read_Term_5(
        stm_word,
        term_word,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
    );
    Pl_Delete_Str_Stream(stm);
    return term_word;
}
pub unsafe extern "C" fn Pl_Read_From_Atom_2(
    mut atom_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(
        (*pl_atom_tbl.offset(Pl_Rd_Atom_Check(atom_word) as isize)).name,
        1 as libc::c_int,
    );
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    ret = Pl_Read_Term_5(
        stm_word,
        term_word,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
    );
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Read_From_Chars_2(
    mut chars_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(Pl_Rd_Chars_Check(chars_word), 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    ret = Pl_Read_Term_5(
        stm_word,
        term_word,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
    );
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Read_From_Codes_2(
    mut codes_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(Pl_Rd_Codes_Check(codes_word), 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    ret = Pl_Read_Term_5(
        stm_word,
        term_word,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
        0 as libc::c_int as WamWord,
    );
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Read_Term_From_Atom_5(
    mut atom_word: WamWord,
    mut term_word: WamWord,
    mut vars_word: WamWord,
    mut var_names_word: WamWord,
    mut sing_names_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(
        (*pl_atom_tbl.offset(Pl_Rd_Atom_Check(atom_word) as isize)).name,
        1 as libc::c_int,
    );
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    ret = Pl_Read_Term_5(
        stm_word,
        term_word,
        vars_word,
        var_names_word,
        sing_names_word,
    );
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Read_Term_From_Chars_5(
    mut chars_word: WamWord,
    mut term_word: WamWord,
    mut vars_word: WamWord,
    mut var_names_word: WamWord,
    mut sing_names_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(Pl_Rd_Chars_Check(chars_word), 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    ret = Pl_Read_Term_5(
        stm_word,
        term_word,
        vars_word,
        var_names_word,
        sing_names_word,
    );
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Read_Term_From_Codes_5(
    mut codes_word: WamWord,
    mut term_word: WamWord,
    mut vars_word: WamWord,
    mut var_names_word: WamWord,
    mut sing_names_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(Pl_Rd_Codes_Check(codes_word), 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    ret = Pl_Read_Term_5(
        stm_word,
        term_word,
        vars_word,
        var_names_word,
        sing_names_word,
    );
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Read_Token_From_Atom_2(
    mut atom_word: WamWord,
    mut token_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(
        (*pl_atom_tbl.offset(Pl_Rd_Atom_Check(atom_word) as isize)).name,
        1 as libc::c_int,
    );
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    ret = Pl_Read_Token_2(stm_word, token_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Read_Token_From_Chars_2(
    mut chars_word: WamWord,
    mut token_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(Pl_Rd_Chars_Check(chars_word), 2 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    ret = Pl_Read_Token_2(stm_word, token_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
pub unsafe extern "C" fn Pl_Read_Token_From_Codes_2(
    mut codes_word: WamWord,
    mut token_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    let mut stm: libc::c_int = 0;
    let mut stm_word: WamWord = 0;
    stm = Pl_Add_Str_Stream(Pl_Rd_Codes_Check(codes_word), 3 as libc::c_int);
    stm_word = Pl_Make_Stream_Tagged_Word(stm);
    ret = Pl_Read_Token_2(stm_word, token_word);
    Pl_Delete_Str_Stream(stm);
    return ret;
}
