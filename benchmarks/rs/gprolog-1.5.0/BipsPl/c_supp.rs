use ::libc;
use ::c2rust_bitfields;
extern "C" {
    static mut pl_glob_buff: [libc::c_char; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_atom_false: libc::c_int;
    static mut pl_atom_true: libc::c_int;
    static mut pl_atom_end_of_file: libc::c_int;
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Float(n: libc::c_double, start_word: WamWord) -> Bool;
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Put_X_Variable() -> WamWord;
    fn Pl_Put_Atom(atom: libc::c_int) -> WamWord;
    fn Pl_Put_Integer(n: PlLong) -> WamWord;
    fn Pl_Put_Float(n: libc::c_double) -> WamWord;
    fn Pl_Put_List() -> WamWord;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    fn Pl_Unify_Nil() -> Bool;
    fn Pl_Unify_List() -> Bool;
    fn Pl_Obtain_Float(adr: *mut WamWord) -> libc::c_double;
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    static mut pl_type_integer: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Instantiation();
    static mut pl_domain_not_less_than_zero: libc::c_int;
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    static mut pl_representation_integer_32bits: libc::c_int;
    static mut pl_type_float: libc::c_int;
    static mut pl_type_number: libc::c_int;
    static mut pl_type_atom: libc::c_int;
    static mut pl_type_boolean: libc::c_int;
    static mut pl_type_character: libc::c_int;
    static mut pl_type_in_character: libc::c_int;
    static mut pl_representation_character_code: libc::c_int;
    fn Pl_Err_Representation(atom_flag: libc::c_int);
    static mut pl_type_byte: libc::c_int;
    static mut pl_type_in_byte: libc::c_int;
    static mut pl_type_list: libc::c_int;
    static mut pl_type_compound: libc::c_int;
    fn Pl_Blt_Callable(x: WamWord) -> Bool;
    fn Pl_Blt_List_Or_Partial_List(x: WamWord) -> Bool;
    static mut pl_type_callable: libc::c_int;
    fn Pl_Err_Uninstantiation(term: WamWord);
    fn Pl_Blt_Compound(x: WamWord) -> Bool;
    static mut pl_representation_in_character_code: libc::c_int;
    static mut pl_type_pair: libc::c_int;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub unsafe extern "C" fn Pl_Rd_Integer_Check(mut start_word: WamWord) -> PlLong {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_integer, word);
    }
    return word << 0 as libc::c_int >> 3 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_Integer(mut start_word: WamWord) -> PlLong {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    return word << 0 as libc::c_int >> 3 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_Positive_Check(mut start_word: WamWord) -> PlLong {
    let mut n: PlLong = Pl_Rd_Integer_Check(start_word);
    if n < 0 as libc::c_int as libc::c_long {
        Pl_Err_Domain(pl_domain_not_less_than_zero, start_word);
    }
    return n;
}
pub unsafe extern "C" fn Pl_Rd_Positive(mut start_word: WamWord) -> PlLong {
    return Pl_Rd_Integer(start_word);
}
unsafe extern "C" fn PlLong_To_C_Int(
    mut x: PlLong,
    mut start_word: WamWord,
) -> libc::c_int {
    if x
        < -((1 as libc::c_long)
            << 8 as libc::c_int * 4 as libc::c_int - 1 as libc::c_int)
        || x
            > ((1 as libc::c_long)
                << 8 as libc::c_int * 4 as libc::c_int - 1 as libc::c_int)
                - 1 as libc::c_int as libc::c_long
    {
        Pl_Err_Domain(pl_representation_integer_32bits, start_word);
    }
    return x as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_C_Int_Check(mut start_word: WamWord) -> libc::c_int {
    return PlLong_To_C_Int(Pl_Rd_Integer_Check(start_word), start_word);
}
pub unsafe extern "C" fn Pl_Rd_C_Int(mut start_word: WamWord) -> libc::c_int {
    return PlLong_To_C_Int(Pl_Rd_Integer(start_word), start_word);
}
pub unsafe extern "C" fn Pl_Rd_C_Int_Positive_Check(
    mut start_word: WamWord,
) -> libc::c_int {
    return PlLong_To_C_Int(Pl_Rd_Positive_Check(start_word), start_word);
}
pub unsafe extern "C" fn Pl_Rd_C_Int_Positive(mut start_word: WamWord) -> libc::c_int {
    return PlLong_To_C_Int(Pl_Rd_Positive(start_word), start_word);
}
pub unsafe extern "C" fn Pl_Rd_Float_Check(mut start_word: WamWord) -> libc::c_double {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong != 0x4 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_float, word);
    }
    return Pl_Obtain_Float(
        (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
    );
}
pub unsafe extern "C" fn Pl_Rd_Float(mut start_word: WamWord) -> libc::c_double {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    return Pl_Obtain_Float(
        (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
    );
}
pub unsafe extern "C" fn Pl_Rd_Number_Check(mut start_word: WamWord) -> libc::c_double {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong != 0x4 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_number, word);
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        return (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_double;
    }
    return Pl_Obtain_Float(
        (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
    );
}
pub unsafe extern "C" fn Pl_Rd_Number(mut start_word: WamWord) -> libc::c_double {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        return (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_double;
    }
    return Pl_Obtain_Float(
        (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
    );
}
pub unsafe extern "C" fn Pl_Rd_Atom_Check(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_atom, word);
    }
    return (word as PlULong >> 3 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_Atom(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    return (word as PlULong >> 3 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_Boolean_Check(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
    if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
        || atom != pl_atom_true && atom != pl_atom_false
    {
        Pl_Err_Type(pl_type_boolean, word);
    }
    return (atom != pl_atom_false) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_Boolean(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    return (atom != pl_atom_false) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_Char_Check(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
    if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
        || ((*pl_atom_tbl.offset(atom as isize)).prop).length() as libc::c_int
            != 1 as libc::c_int
    {
        Pl_Err_Type(pl_type_character, word);
    }
    return *((*pl_atom_tbl.offset(atom as isize)).name).offset(0 as libc::c_int as isize)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_Char(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    return *((*pl_atom_tbl.offset(atom as isize)).name).offset(0 as libc::c_int as isize)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_In_Char_Check(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
    if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
        || atom != pl_atom_end_of_file
            && ((*pl_atom_tbl.offset(atom as isize)).prop).length() as libc::c_int
                != 1 as libc::c_int
    {
        Pl_Err_Type(pl_type_in_character, word);
    }
    return if atom != pl_atom_end_of_file {
        *((*pl_atom_tbl.offset(atom as isize)).name).offset(0 as libc::c_int as isize)
            as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn Pl_Rd_In_Char(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    return if atom != pl_atom_end_of_file {
        *((*pl_atom_tbl.offset(atom as isize)).name).offset(0 as libc::c_int as isize)
            as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
pub unsafe extern "C" fn Pl_Rd_Code_Check(mut start_word: WamWord) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = Pl_Rd_Integer_Check(start_word) as libc::c_int;
    if !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_character_code);
    }
    return c;
}
pub unsafe extern "C" fn Pl_Rd_Code(mut start_word: WamWord) -> libc::c_int {
    return Pl_Rd_Integer(start_word) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_In_Code_Check(mut start_word: WamWord) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = Pl_Rd_Integer_Check(start_word) as libc::c_int;
    if c != -(1 as libc::c_int)
        && !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_in_character_code);
    }
    return c;
}
pub unsafe extern "C" fn Pl_Rd_In_Code(mut start_word: WamWord) -> libc::c_int {
    return Pl_Rd_Integer(start_word) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_Byte_Check(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        || !((c as PlULong) < 256 as libc::c_int as libc::c_ulong)
    {
        Pl_Err_Type(pl_type_byte, word);
    }
    return c as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_Byte(mut start_word: WamWord) -> libc::c_int {
    return Pl_Rd_Integer(start_word) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_In_Byte_Check(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        || c != -(1 as libc::c_int) as libc::c_long
            && !((c as PlULong) < 256 as libc::c_int as libc::c_ulong)
    {
        Pl_Err_Type(pl_type_in_byte, word);
    }
    return c as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_In_Byte(mut start_word: WamWord) -> libc::c_int {
    return Pl_Rd_Integer(start_word) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Rd_String_Check(
    mut start_word: WamWord,
) -> *mut libc::c_char {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_atom, word);
    }
    return (*pl_atom_tbl.offset((word as PlULong >> 3 as libc::c_int) as isize)).name;
}
pub unsafe extern "C" fn Pl_Rd_String(mut start_word: WamWord) -> *mut libc::c_char {
    return (*pl_atom_tbl.offset(Pl_Rd_Atom(start_word) as isize)).name;
}
pub unsafe extern "C" fn Pl_Rd_Chars_Check(
    mut start_word: WamWord,
) -> *mut libc::c_char {
    Pl_Rd_Chars_Str_Check(start_word, pl_glob_buff.as_mut_ptr());
    return pl_glob_buff.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_Rd_Chars(mut start_word: WamWord) -> *mut libc::c_char {
    Pl_Rd_Chars_Str(start_word, pl_glob_buff.as_mut_ptr());
    return pl_glob_buff.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_Rd_Codes_Check(
    mut start_word: WamWord,
) -> *mut libc::c_char {
    Pl_Rd_Codes_Str_Check(start_word, pl_glob_buff.as_mut_ptr());
    return pl_glob_buff.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_Rd_Codes(mut start_word: WamWord) -> *mut libc::c_char {
    Pl_Rd_Codes_Str(start_word, pl_glob_buff.as_mut_ptr());
    return pl_glob_buff.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_Rd_Chars_Str_Check(
    mut start_word: WamWord,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_start_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
    save_start_word = start_word;
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
            Pl_Err_Type(pl_type_list, save_start_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let fresh0 = str;
        str = str.offset(1);
        *fresh0 = Pl_Rd_Char_Check(*lst_adr.offset(0 as libc::c_int as isize))
            as libc::c_char;
        n += 1;
        n;
        start_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    *str = '\0' as i32 as libc::c_char;
    return n;
}
pub unsafe extern "C" fn Pl_Rd_Chars_Str(
    mut start_word: WamWord,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
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
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            break;
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let fresh1 = str;
        str = str.offset(1);
        *fresh1 = Pl_Rd_Char_Check(*lst_adr.offset(0 as libc::c_int as isize))
            as libc::c_char;
        n += 1;
        n;
        start_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    *str = '\0' as i32 as libc::c_char;
    return n;
}
pub unsafe extern "C" fn Pl_Rd_Codes_Str_Check(
    mut start_word: WamWord,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_start_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
    save_start_word = start_word;
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
            Pl_Err_Type(pl_type_list, save_start_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let fresh2 = str;
        str = str.offset(1);
        *fresh2 = Pl_Rd_Code_Check(*lst_adr.offset(0 as libc::c_int as isize))
            as libc::c_char;
        n += 1;
        n;
        start_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    *str = '\0' as i32 as libc::c_char;
    return n;
}
pub unsafe extern "C" fn Pl_Rd_Codes_Str(
    mut start_word: WamWord,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
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
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            break;
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let fresh3 = str;
        str = str.offset(1);
        *fresh3 = Pl_Rd_Code_Check(*lst_adr.offset(0 as libc::c_int as isize))
            as libc::c_char;
        n += 1;
        n;
        start_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    *str = '\0' as i32 as libc::c_char;
    return n;
}
pub unsafe extern "C" fn Pl_Rd_List_Check(mut start_word: WamWord) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if word as libc::c_ulong
        == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    {
        return 0 as *mut WamWord;
    }
    if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_list, start_word);
    }
    lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord;
    return &mut *lst_adr.offset(0 as libc::c_int as isize) as *mut WamWord;
}
pub unsafe extern "C" fn Pl_Rd_List(mut start_word: WamWord) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if word as libc::c_ulong
        == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    {
        return 0 as *mut WamWord;
    }
    lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord;
    return &mut *lst_adr.offset(0 as libc::c_int as isize) as *mut WamWord;
}
pub unsafe extern "C" fn Pl_Rd_Proper_List_Check(
    mut start_word: WamWord,
    mut arg: *mut WamWord,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_start_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
    save_start_word = start_word;
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
            Pl_Err_Type(pl_type_list, save_start_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let fresh4 = arg;
        arg = arg.offset(1);
        *fresh4 = *lst_adr.offset(0 as libc::c_int as isize);
        n += 1;
        n;
        start_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    return n;
}
pub unsafe extern "C" fn Pl_Rd_Proper_List_Check2(
    mut start_word: WamWord,
    mut arg: *mut WamWord,
    mut elt_fct: Option::<unsafe extern "C" fn(WamWord) -> WamWord>,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_start_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
    save_start_word = start_word;
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
            Pl_Err_Type(pl_type_list, save_start_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let fresh5 = arg;
        arg = arg.offset(1);
        *fresh5 = (Some(elt_fct.unwrap()))
            .unwrap()(*lst_adr.offset(0 as libc::c_int as isize));
        n += 1;
        n;
        start_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    return n;
}
pub unsafe extern "C" fn Pl_Rd_Proper_List(
    mut start_word: WamWord,
    mut arg: *mut WamWord,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
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
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            break;
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let fresh6 = arg;
        arg = arg.offset(1);
        *fresh6 = *lst_adr.offset(0 as libc::c_int as isize);
        n += 1;
        n;
        start_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    return n;
}
pub unsafe extern "C" fn Pl_Rd_Compound_Check(
    mut start_word: WamWord,
    mut func: *mut libc::c_int,
    mut arity: *mut libc::c_int,
) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        *func = '.' as i32 as libc::c_uchar as libc::c_int;
        *arity = 2 as libc::c_int;
        return &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
    }
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        *func = (*adr.offset(0 as libc::c_int as isize) as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        *arity = (*adr.offset(0 as libc::c_int as isize) as PlULong
            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
        return &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
            as *mut WamWord;
    }
    Pl_Err_Type(pl_type_compound, start_word);
    return 0 as *mut WamWord;
}
pub unsafe extern "C" fn Pl_Rd_Compound(
    mut start_word: WamWord,
    mut func: *mut libc::c_int,
    mut arity: *mut libc::c_int,
) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        *func = '.' as i32 as libc::c_uchar as libc::c_int;
        *arity = 2 as libc::c_int;
        return &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
    }
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        *func = (*adr.offset(0 as libc::c_int as isize) as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        *arity = (*adr.offset(0 as libc::c_int as isize) as PlULong
            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
        return &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
            as *mut WamWord;
    }
    return 0 as *mut WamWord;
}
pub unsafe extern "C" fn Pl_Rd_Callable_Check(
    mut start_word: WamWord,
    mut func: *mut libc::c_int,
    mut arity: *mut libc::c_int,
) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        *func = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        *arity = 0 as libc::c_int;
        return 0 as *mut WamWord;
    }
    if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        *func = '.' as i32 as libc::c_uchar as libc::c_int;
        *arity = 2 as libc::c_int;
        return &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
    }
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        *func = (*adr.offset(0 as libc::c_int as isize) as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        *arity = (*adr.offset(0 as libc::c_int as isize) as PlULong
            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
        return &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
            as *mut WamWord;
    }
    Pl_Err_Type(pl_type_callable, start_word);
    return 0 as *mut WamWord;
}
pub unsafe extern "C" fn Pl_Rd_Callable(
    mut start_word: WamWord,
    mut func: *mut libc::c_int,
    mut arity: *mut libc::c_int,
) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        *func = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        *arity = 0 as libc::c_int;
        return arity as *mut WamWord;
    }
    if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        *func = '.' as i32 as libc::c_uchar as libc::c_int;
        *arity = 2 as libc::c_int;
        return &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
    }
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        *func = (*adr.offset(0 as libc::c_int as isize) as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        *arity = (*adr.offset(0 as libc::c_int as isize) as PlULong
            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
        return &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
            as *mut WamWord;
    }
    return 0 as *mut WamWord;
}
pub unsafe extern "C" fn Pl_Check_For_Un_Integer(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Positive(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, word);
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        && (word << 0 as libc::c_int >> 3 as libc::c_int)
            < 0 as libc::c_int as libc::c_long
    {
        Pl_Err_Domain(pl_domain_not_less_than_zero, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Float(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_float, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Number(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_number, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Atom(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        Pl_Err_Type(pl_type_atom, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Boolean(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || atom != pl_atom_true && atom != pl_atom_false)
    {
        Pl_Err_Type(pl_type_boolean, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Char(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || ((*pl_atom_tbl.offset(atom as isize)).prop).length() as libc::c_int
                != 1 as libc::c_int)
    {
        Pl_Err_Type(pl_type_character, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_In_Char(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || atom != pl_atom_end_of_file
                && ((*pl_atom_tbl.offset(atom as isize)).prop).length() as libc::c_int
                    != 1 as libc::c_int)
    {
        Pl_Err_Type(pl_type_in_character, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Code(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, word);
    }
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        && !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_character_code);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_In_Code(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, word);
    }
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        && c != -(1 as libc::c_int) as libc::c_long
        && !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_in_character_code);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Byte(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
            || !((c as PlULong) < 256 as libc::c_int as libc::c_ulong))
    {
        Pl_Err_Type(pl_type_byte, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_In_Byte(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
            || c != -(1 as libc::c_int) as libc::c_long
                && !((c as PlULong) < 256 as libc::c_int as libc::c_ulong))
    {
        Pl_Err_Type(pl_type_in_byte, word);
    }
}
static mut minus_2: WamWord = 0;
pub unsafe extern "C" fn Pl_Check_For_Un_Pair(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && (tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong
            || *((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord)
                .offset(0 as libc::c_int as isize) != minus_2)
    {
        Pl_Err_Type(pl_type_pair, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Chars(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_start_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    save_start_word = start_word;
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
            || word as libc::c_ulong
                == (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            return;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_list, save_start_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        Pl_Check_For_Un_Char(*lst_adr.offset(0 as libc::c_int as isize));
        start_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    };
}
pub unsafe extern "C" fn Pl_Check_For_Un_String(mut start_word: WamWord) {
    Pl_Check_For_Un_Atom(start_word);
}
pub unsafe extern "C" fn Pl_Check_For_Un_Codes(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_start_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    save_start_word = start_word;
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
            || word as libc::c_ulong
                == (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            return;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_list, save_start_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        Pl_Check_For_Un_Code(*lst_adr.offset(0 as libc::c_int as isize));
        start_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    };
}
pub unsafe extern "C" fn Pl_Check_For_Un_List(mut start_word: WamWord) {
    if Pl_Blt_List_Or_Partial_List(start_word) == 0 {
        Pl_Err_Type(pl_type_list, start_word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_List2(
    mut start_word: WamWord,
    mut elt_fct: Option::<unsafe extern "C" fn(WamWord) -> ()>,
) {
    let mut start_word0: WamWord = start_word;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
            || word as libc::c_ulong
                == (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            return;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_list, start_word0);
        }
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        (Some(elt_fct.unwrap())).unwrap()(*adr.offset(0 as libc::c_int as isize));
        start_word = *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    };
}
pub unsafe extern "C" fn Pl_Check_For_Un_Compound(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_compound, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Callable(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_callable, word);
    }
}
pub unsafe extern "C" fn Pl_Check_For_Un_Variable(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        Pl_Err_Uninstantiation(word);
    }
}
pub unsafe extern "C" fn Pl_Un_Integer_Check(
    mut value: PlLong,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, word);
    }
    return Pl_Get_Integer(value, word);
}
pub unsafe extern "C" fn Pl_Un_Integer(
    mut value: PlLong,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Integer(value, start_word);
}
pub unsafe extern "C" fn Pl_Un_Positive_Check(
    mut value: PlLong,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, word);
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        && (word << 0 as libc::c_int >> 3 as libc::c_int)
            < 0 as libc::c_int as libc::c_long
    {
        Pl_Err_Domain(pl_domain_not_less_than_zero, word);
    }
    return Pl_Get_Integer(value, word);
}
pub unsafe extern "C" fn Pl_Un_Positive(
    mut value: PlLong,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Integer(value, start_word);
}
pub unsafe extern "C" fn Pl_Un_Float_Check(
    mut value: libc::c_double,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_float, word);
    }
    return Pl_Get_Float(value, word);
}
pub unsafe extern "C" fn Pl_Un_Float(
    mut value: libc::c_double,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Float(value, start_word);
}
pub unsafe extern "C" fn Pl_Un_Number_Check(
    mut value: libc::c_double,
    mut start_word: WamWord,
) -> Bool {
    let mut n: PlLong = 0;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_number, word);
    }
    n = value as PlLong;
    return if n as libc::c_double == value {
        Pl_Get_Integer(n, word)
    } else {
        Pl_Get_Float(value, word)
    };
}
pub unsafe extern "C" fn Pl_Un_Number(
    mut value: libc::c_double,
    mut start_word: WamWord,
) -> Bool {
    let mut n: PlLong = 0;
    n = value as PlLong;
    return if n as libc::c_double == value {
        Pl_Get_Integer(n, start_word)
    } else {
        Pl_Get_Float(value, start_word)
    };
}
pub unsafe extern "C" fn Pl_Un_Atom_Check(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        Pl_Err_Type(pl_type_atom, word);
    }
    return Pl_Get_Atom(value, word);
}
pub unsafe extern "C" fn Pl_Un_Atom(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Atom(value, start_word);
}
pub unsafe extern "C" fn Pl_Un_Boolean_Check(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || atom != pl_atom_true && atom != pl_atom_false)
    {
        Pl_Err_Type(pl_type_boolean, word);
    }
    return Pl_Get_Atom(if value != 0 { pl_atom_true } else { pl_atom_false }, word);
}
pub unsafe extern "C" fn Pl_Un_Boolean(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Atom(
        if value != 0 { pl_atom_true } else { pl_atom_false },
        start_word,
    );
}
pub unsafe extern "C" fn Pl_Un_Char_Check(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || ((*pl_atom_tbl.offset(atom as isize)).prop).length() as libc::c_int
                != 1 as libc::c_int)
    {
        Pl_Err_Type(pl_type_character, word);
    }
    return Pl_Get_Atom(value as libc::c_uchar as libc::c_int, word);
}
pub unsafe extern "C" fn Pl_Un_Char(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Atom(value as libc::c_uchar as libc::c_int, start_word);
}
pub unsafe extern "C" fn Pl_Un_In_Char_Check(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || atom != pl_atom_end_of_file
                && ((*pl_atom_tbl.offset(atom as isize)).prop).length() as libc::c_int
                    != 1 as libc::c_int)
    {
        Pl_Err_Type(pl_type_in_character, word);
    }
    return Pl_Get_Atom(
        if value == -(1 as libc::c_int) {
            pl_atom_end_of_file
        } else {
            value as libc::c_uchar as libc::c_int
        },
        word,
    );
}
pub unsafe extern "C" fn Pl_Un_In_Char(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Atom(
        if value == -(1 as libc::c_int) {
            pl_atom_end_of_file
        } else {
            value as libc::c_uchar as libc::c_int
        },
        start_word,
    );
}
pub unsafe extern "C" fn Pl_Un_Code_Check(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, word);
    }
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        && !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_character_code);
    }
    return Pl_Get_Integer(value as PlLong, word);
}
pub unsafe extern "C" fn Pl_Un_Code(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Integer(value as PlLong, start_word);
}
pub unsafe extern "C" fn Pl_Un_In_Code_Check(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, word);
    }
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        && c != -(1 as libc::c_int) as libc::c_long
        && !((c as PlULong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (256 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
    {
        Pl_Err_Representation(pl_representation_in_character_code);
    }
    return Pl_Get_Integer(value as PlLong, word);
}
pub unsafe extern "C" fn Pl_Un_In_Code(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Integer(value as PlLong, start_word);
}
pub unsafe extern "C" fn Pl_Un_Byte_Check(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
            || !((c as PlULong) < 256 as libc::c_int as libc::c_ulong))
    {
        Pl_Err_Type(pl_type_byte, word);
    }
    return Pl_Get_Integer(value as PlLong, word);
}
pub unsafe extern "C" fn Pl_Un_Byte(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Integer(value as PlLong, start_word);
}
pub unsafe extern "C" fn Pl_Un_In_Byte_Check(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut c: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    c = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
            || c != -(1 as libc::c_int) as libc::c_long
                && !((c as PlULong) < 256 as libc::c_int as libc::c_ulong))
    {
        Pl_Err_Type(pl_type_in_byte, word);
    }
    return Pl_Get_Integer(value as PlLong, word);
}
pub unsafe extern "C" fn Pl_Un_In_Byte(
    mut value: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Integer(value as PlLong, start_word);
}
pub unsafe extern "C" fn Pl_Un_String_Check(
    mut value: *mut libc::c_char,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
        Pl_Err_Type(pl_type_atom, word);
    }
    return Pl_Get_Atom(Pl_Create_Allocate_Atom(value), word);
}
pub unsafe extern "C" fn Pl_Un_String(
    mut value: *mut libc::c_char,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Atom(Pl_Create_Allocate_Atom(value), start_word);
}
pub unsafe extern "C" fn Pl_Un_Chars_Check(
    mut str: *mut libc::c_char,
    mut start_word: WamWord,
) -> Bool {
    Pl_Check_For_Un_Chars(start_word);
    return Pl_Un_Chars(str, start_word);
}
pub unsafe extern "C" fn Pl_Un_Chars(
    mut str: *mut libc::c_char,
    mut start_word: WamWord,
) -> Bool {
    while *str != 0 {
        if Pl_Get_List(start_word) == 0
            || Pl_Unify_Atom(*str as libc::c_uchar as libc::c_int) == 0
        {
            return 0 as libc::c_int;
        }
        start_word = Pl_Unify_Variable();
        str = str.offset(1);
        str;
    }
    return Pl_Get_Nil(start_word);
}
pub unsafe extern "C" fn Pl_Un_Codes_Check(
    mut str: *mut libc::c_char,
    mut start_word: WamWord,
) -> Bool {
    Pl_Check_For_Un_Codes(start_word);
    return Pl_Un_Codes(str, start_word);
}
pub unsafe extern "C" fn Pl_Un_Codes(
    mut str: *mut libc::c_char,
    mut start_word: WamWord,
) -> Bool {
    while *str != 0 {
        if Pl_Get_List(start_word) == 0 || Pl_Unify_Integer(*str as PlLong) == 0 {
            return 0 as libc::c_int;
        }
        start_word = Pl_Unify_Variable();
        str = str.offset(1);
        str;
    }
    return Pl_Get_Nil(start_word);
}
pub unsafe extern "C" fn Pl_Un_List_Check(
    mut arg: *mut WamWord,
    mut start_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    if arg.is_null() {
        if Pl_Get_Nil(start_word) != 0 {
            return 1 as libc::c_int;
        }
    } else if !(Pl_Get_List(start_word) == 0) {
        return (Pl_Unify_Value(*arg.offset(0 as libc::c_int as isize)) != 0
            && Pl_Unify_Value(*arg.offset(1 as libc::c_int as isize)) != 0)
            as libc::c_int
    }
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if word as libc::c_ulong
        != (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
        && tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_list, start_word);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Un_List(
    mut arg: *mut WamWord,
    mut start_word: WamWord,
) -> Bool {
    if arg.is_null() {
        return Pl_Get_Nil(start_word);
    }
    return (Pl_Get_List(start_word) != 0
        && Pl_Unify_Value(*arg.offset(0 as libc::c_int as isize)) != 0
        && Pl_Unify_Value(*arg.offset(1 as libc::c_int as isize)) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Un_Proper_List_Check(
    mut n: libc::c_int,
    mut arg: *mut WamWord,
    mut start_word: WamWord,
) -> Bool {
    Pl_Check_For_Un_List(start_word);
    return Pl_Un_Proper_List(n, arg, start_word);
}
pub unsafe extern "C" fn Pl_Un_Proper_List(
    mut n: libc::c_int,
    mut arg: *mut WamWord,
    mut start_word: WamWord,
) -> Bool {
    if n < 0 as libc::c_int || arg.is_null() {
        n = 0 as libc::c_int;
    }
    loop {
        let fresh7 = n;
        n = n - 1;
        if !(fresh7 != 0) {
            break;
        }
        if Pl_Get_List(start_word) == 0
            || {
                let fresh8 = arg;
                arg = arg.offset(1);
                Pl_Unify_Value(*fresh8) == 0
            }
        {
            return 0 as libc::c_int;
        }
        start_word = Pl_Unify_Variable();
    }
    return Pl_Get_Nil(start_word);
}
pub unsafe extern "C" fn Pl_Un_Compound_Check(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg: *mut WamWord,
    mut start_word: WamWord,
) -> Bool {
    let mut i: libc::c_int = 0;
    if arity == 0 as libc::c_int {
        return Pl_Un_Atom_Check(func, start_word);
    }
    if arity == 2 as libc::c_int && func == '.' as i32 as libc::c_uchar as libc::c_int {
        return Pl_Un_List_Check(arg, start_word);
    }
    if Pl_Get_Structure(func, arity, start_word) == 0 {
        if Pl_Blt_Compound(start_word) == 0 {
            Pl_Err_Type(pl_type_compound, start_word);
        }
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < arity {
        if Pl_Unify_Value(*arg.offset(i as isize)) == 0 {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Un_Compound(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg: *mut WamWord,
    mut start_word: WamWord,
) -> Bool {
    let mut i: libc::c_int = 0;
    if arity == 0 as libc::c_int {
        return Pl_Un_Atom_Check(func, start_word);
    }
    if arity == 2 as libc::c_int && func == '.' as i32 as libc::c_uchar as libc::c_int {
        return Pl_Un_List(arg, start_word);
    }
    if Pl_Get_Structure(func, arity, start_word) == 0 {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < arity {
        if Pl_Unify_Value(*arg.offset(i as isize)) == 0 {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Un_Callable_Check(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg: *mut WamWord,
    mut start_word: WamWord,
) -> Bool {
    let mut i: libc::c_int = 0;
    if arity == 0 as libc::c_int {
        return Pl_Un_Atom_Check(func, start_word);
    }
    if arity == 2 as libc::c_int && func == '.' as i32 as libc::c_uchar as libc::c_int {
        return Pl_Un_List_Check(arg, start_word);
    }
    if Pl_Get_Structure(func, arity, start_word) == 0 {
        if Pl_Blt_Callable(start_word) == 0 {
            Pl_Err_Type(pl_type_callable, start_word);
        }
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < arity {
        if Pl_Unify_Value(*arg.offset(i as isize)) == 0 {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Un_Callable(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg: *mut WamWord,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Un_Compound(func, arity, arg, start_word);
}
pub unsafe extern "C" fn Pl_Un_Term(
    mut term_word: WamWord,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Unify(term_word, start_word);
}
pub unsafe extern "C" fn Pl_Mk_Integer(mut value: PlLong) -> WamWord {
    return Pl_Put_Integer(value);
}
pub unsafe extern "C" fn Pl_Mk_Positive(mut value: PlLong) -> WamWord {
    return Pl_Put_Integer(value);
}
pub unsafe extern "C" fn Pl_Mk_Float(mut value: libc::c_double) -> WamWord {
    return Pl_Put_Float(value);
}
pub unsafe extern "C" fn Pl_Mk_Number(mut value: libc::c_double) -> WamWord {
    let mut n: libc::c_int = 0;
    n = value as PlLong as libc::c_int;
    if n as libc::c_double == value {
        return Pl_Put_Integer(n as PlLong);
    }
    return Pl_Put_Float(value);
}
pub unsafe extern "C" fn Pl_Mk_Atom(mut value: libc::c_int) -> WamWord {
    return Pl_Put_Atom(value);
}
pub unsafe extern "C" fn Pl_Mk_Boolean(mut value: libc::c_int) -> WamWord {
    return Pl_Put_Atom(if value != 0 { pl_atom_true } else { pl_atom_false });
}
pub unsafe extern "C" fn Pl_Mk_Char(mut value: libc::c_int) -> WamWord {
    return Pl_Put_Atom(value as libc::c_uchar as libc::c_int);
}
pub unsafe extern "C" fn Pl_Mk_In_Char(mut value: libc::c_int) -> WamWord {
    return Pl_Put_Atom(
        if value == -(1 as libc::c_int) {
            pl_atom_end_of_file
        } else {
            value as libc::c_uchar as libc::c_int
        },
    );
}
pub unsafe extern "C" fn Pl_Mk_Code(mut value: libc::c_int) -> WamWord {
    return Pl_Put_Integer(value as PlLong);
}
pub unsafe extern "C" fn Pl_Mk_In_Code(mut value: libc::c_int) -> WamWord {
    return Pl_Put_Integer(value as PlLong);
}
pub unsafe extern "C" fn Pl_Mk_Byte(mut value: libc::c_int) -> WamWord {
    return Pl_Put_Integer(value as PlLong);
}
pub unsafe extern "C" fn Pl_Mk_In_Byte(mut value: libc::c_int) -> WamWord {
    return Pl_Put_Integer(value as PlLong);
}
pub unsafe extern "C" fn Pl_Mk_String(mut value: *mut libc::c_char) -> WamWord {
    return Pl_Put_Atom(Pl_Create_Allocate_Atom(value));
}
pub unsafe extern "C" fn Pl_Mk_Chars(mut str: *mut libc::c_char) -> WamWord {
    let mut res_word: WamWord = 0;
    if *str as libc::c_int == '\0' as i32 {
        return (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    }
    res_word = Pl_Put_List();
    loop {
        Pl_Unify_Atom(*str as libc::c_uchar as libc::c_int);
        str = str.offset(1);
        str;
        if *str as libc::c_int == '\0' as i32 {
            break;
        }
        Pl_Unify_List();
    }
    Pl_Unify_Nil();
    return res_word;
}
pub unsafe extern "C" fn Pl_Mk_Codes(mut str: *mut libc::c_char) -> WamWord {
    let mut res_word: WamWord = 0;
    if *str as libc::c_int == '\0' as i32 {
        return (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    }
    res_word = Pl_Put_List();
    loop {
        Pl_Unify_Integer(*str as PlLong);
        str = str.offset(1);
        str;
        if *str as libc::c_int == '\0' as i32 {
            break;
        }
        Pl_Unify_List();
    }
    Pl_Unify_Nil();
    return res_word;
}
pub unsafe extern "C" fn Pl_Mk_List(mut arg: *mut WamWord) -> WamWord {
    let mut res_word: WamWord = 0;
    if arg.is_null() {
        return (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    }
    res_word = Pl_Put_List();
    Pl_Unify_Value(*arg.offset(0 as libc::c_int as isize));
    Pl_Unify_Value(*arg.offset(1 as libc::c_int as isize));
    return res_word;
}
pub unsafe extern "C" fn Pl_Mk_Proper_List(
    mut n: libc::c_int,
    mut arg: *mut WamWord,
) -> WamWord {
    let mut src: *mut WamWord = 0 as *mut WamWord;
    let mut dst: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    if n <= 0 as libc::c_int || arg.is_null() {
        return (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    }
    src = arg.offset(n as isize);
    H = H.offset((2 as libc::c_int * n) as isize);
    dst = H;
    dst = dst.offset(-1);
    *dst = (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    loop {
        src = src.offset(-1);
        dst = dst.offset(-1);
        *dst = *src;
        n -= 1;
        if !(n != 0) {
            break;
        }
        let fresh9 = dst;
        dst = dst.offset(-1);
        p = fresh9;
        *dst = (p as PlLong as libc::c_ulong).wrapping_add(0x1 as libc::c_int as PlULong)
            as WamWord;
    }
    return (dst as PlLong as libc::c_ulong).wrapping_add(0x1 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Mk_Compound(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg: *mut WamWord,
) -> WamWord {
    let mut res_word: WamWord = 0;
    let mut i: libc::c_int = 0;
    if arity == 0 as libc::c_int {
        return Pl_Put_Atom(func);
    }
    if arity == 2 as libc::c_int && func == '.' as i32 as libc::c_uchar as libc::c_int {
        return Pl_Mk_List(arg);
    }
    res_word = Pl_Put_Structure(func, arity);
    i = 0 as libc::c_int;
    while i < arity {
        Pl_Unify_Value(*arg.offset(i as isize));
        i += 1;
        i;
    }
    return res_word;
}
pub unsafe extern "C" fn Pl_Mk_Callable(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg: *mut WamWord,
) -> WamWord {
    return Pl_Mk_Compound(func, arity, arg);
}
pub unsafe extern "C" fn Pl_Mk_Variable() -> WamWord {
    return Pl_Put_X_Variable();
}
unsafe extern "C" fn run_static_initializers() {
    minus_2 = ((2 as libc::c_int as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)
        | '-' as i32 as libc::c_uchar as libc::c_int as libc::c_ulong) as WamWord;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
