use ::libc;
extern "C" {
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    static mut pl_atom_false: libc::c_int;
    static mut pl_atom_true: libc::c_int;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    static mut pl_atom_error: libc::c_int;
    static mut pl_domain_flag_value: libc::c_int;
    static mut pl_domain_prolog_flag: libc::c_int;
    static mut pl_permission_operation_modify: libc::c_int;
    static mut pl_permission_type_flag: libc::c_int;
    fn Pl_Err_Instantiation();
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn Pl_Err_Permission(atom_oper: libc::c_int, atom_perm: libc::c_int, term: WamWord);
    fn X1_2463757272656E745F70726F6C6F675F666C61675F616C74__a0();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamWordP = *mut WamWord;
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
static mut flag_tbl: [FlagInf; 64] = [FlagInf {
    atom_name: 0,
    modifiable: 0,
    type_0: PF_TYPE_INTEGER,
    value: 0,
    fct_get: None,
    fct_chk: None,
    fct_set: None,
}; 64];
static mut nb_flag: libc::c_int = 0;
static mut atom_on: libc::c_int = 0;
static mut atom_off: libc::c_int = 0;
static mut atom_warning: libc::c_int = 0;
static mut atom_fail: libc::c_int = 0;
static mut atom_chars: libc::c_int = 0;
static mut atom_codes: libc::c_int = 0;
static mut atom_atom: libc::c_int = 0;
static mut atom_chars_no_escape: libc::c_int = 0;
static mut atom_codes_no_escape: libc::c_int = 0;
static mut atom_atom_no_escape: libc::c_int = 0;
static mut atom_toward_zero: libc::c_int = 0;
static mut atom_down: libc::c_int = 0;
unsafe extern "C" fn Init_Flag_Supp() {
    static mut initialized: Bool = 0 as libc::c_int;
    if initialized != 0 {
        return;
    }
    initialized = 1 as libc::c_int;
    atom_toward_zero = Pl_Create_Atom(
        b"toward_zero\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_down = Pl_Create_Atom(
        b"down\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_on = Pl_Create_Atom(
        b"on\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_off = Pl_Create_Atom(
        b"off\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_warning = Pl_Create_Atom(
        b"warning\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_fail = Pl_Create_Atom(
        b"fail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_chars = Pl_Create_Atom(
        b"chars\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_codes = Pl_Create_Atom(
        b"codes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_atom = Pl_Create_Atom(
        b"atom\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_chars_no_escape = Pl_Create_Atom(
        b"chars_no_escape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_codes_no_escape = Pl_Create_Atom(
        b"codes_no_escape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    atom_atom_no_escape = Pl_Create_Atom(
        b"atom_no_escape\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn Fct_Get_Integer(mut flag: *mut FlagInf) -> WamWord {
    return (((*flag).value as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}
unsafe extern "C" fn Fct_Chk_Integer(
    mut flag: *mut FlagInf,
    mut tag_mask: WamWord,
    mut value_word: WamWord,
) -> Bool {
    return (tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong) as libc::c_int;
}
unsafe extern "C" fn Fct_Set_Integer(
    mut flag: *mut FlagInf,
    mut value_word: WamWord,
) -> Bool {
    (*flag).value = value_word << 0 as libc::c_int >> 3 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn Fct_Get_Atom(mut flag: *mut FlagInf) -> WamWord {
    return (((*flag).value << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
}
unsafe extern "C" fn Fct_Chk_Atom(
    mut flag: *mut FlagInf,
    mut tag_mask: WamWord,
    mut value_word: WamWord,
) -> Bool {
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong) as libc::c_int;
}
unsafe extern "C" fn Fct_Set_Atom(
    mut flag: *mut FlagInf,
    mut value_word: WamWord,
) -> Bool {
    (*flag).value = (value_word as PlULong >> 3 as libc::c_int) as PlLong;
    return 1 as libc::c_int;
}
unsafe extern "C" fn Fct_Get_Round(mut flag: *mut FlagInf) -> WamWord {
    return (if (*flag).value == 0 as libc::c_int as libc::c_long {
        (((atom_toward_zero as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    } else {
        (((atom_down as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    }) as WamWord;
}
unsafe extern "C" fn Fct_Chk_Round(
    mut flag: *mut FlagInf,
    mut tag_mask: WamWord,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        && (atom == atom_toward_zero || atom == atom_down)) as libc::c_int;
}
unsafe extern "C" fn Fct_Set_Round(
    mut flag: *mut FlagInf,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    (*flag)
        .value = (if atom == atom_toward_zero {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    }) as PlLong;
    return 1 as libc::c_int;
}
unsafe extern "C" fn Fct_Get_Bool(mut flag: *mut FlagInf) -> WamWord {
    return (if (*flag).value != 0 {
        (((pl_atom_true as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    } else {
        (((pl_atom_false as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    }) as WamWord;
}
unsafe extern "C" fn Fct_Chk_Bool(
    mut flag: *mut FlagInf,
    mut tag_mask: WamWord,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        && (atom == pl_atom_true || atom == pl_atom_false)) as libc::c_int;
}
unsafe extern "C" fn Fct_Set_Bool(
    mut flag: *mut FlagInf,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    (*flag).value = (atom == pl_atom_true) as libc::c_int as PlLong;
    return 1 as libc::c_int;
}
unsafe extern "C" fn Fct_Get_On_Off(mut flag: *mut FlagInf) -> WamWord {
    return (if (*flag).value != 0 {
        (((atom_on as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    } else {
        (((atom_off as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    }) as WamWord;
}
unsafe extern "C" fn Fct_Chk_On_Off(
    mut flag: *mut FlagInf,
    mut tag_mask: WamWord,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        && (atom == atom_on || atom == atom_off)) as libc::c_int;
}
unsafe extern "C" fn Fct_Set_On_Off(
    mut flag: *mut FlagInf,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    (*flag).value = (atom == atom_on) as libc::c_int as PlLong;
    return 1 as libc::c_int;
}
unsafe extern "C" fn Fct_Get_Err(mut flag: *mut FlagInf) -> WamWord {
    let mut atom: libc::c_int = 0 as libc::c_int;
    match (*flag).value {
        0 => {
            atom = pl_atom_error;
        }
        1 => {
            atom = atom_warning;
        }
        2 => {
            atom = atom_fail;
        }
        _ => {}
    }
    return (((atom as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
}
unsafe extern "C" fn Fct_Chk_Err(
    mut flag: *mut FlagInf,
    mut tag_mask: WamWord,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        && (atom == pl_atom_error || atom == atom_warning || atom == atom_fail))
        as libc::c_int;
}
unsafe extern "C" fn Fct_Set_Err(
    mut flag: *mut FlagInf,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    if atom == pl_atom_error {
        (*flag).value = 0 as libc::c_int as PlLong;
    } else if atom == atom_warning {
        (*flag).value = 1 as libc::c_int as PlLong;
    } else {
        (*flag).value = 2 as libc::c_int as PlLong;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Fct_Get_Quotes(mut flag: *mut FlagInf) -> WamWord {
    let mut atom: libc::c_int = 0 as libc::c_int;
    match (*flag).value {
        0 => {
            atom = atom_codes;
        }
        4 => {
            atom = atom_codes_no_escape;
        }
        1 => {
            atom = atom_chars;
        }
        5 => {
            atom = atom_chars_no_escape;
        }
        2 => {
            atom = atom_atom;
        }
        6 => {
            atom = atom_atom_no_escape;
        }
        _ => {}
    }
    return (((atom as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
}
unsafe extern "C" fn Fct_Chk_Quotes(
    mut flag: *mut FlagInf,
    mut tag_mask: WamWord,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        && (atom == atom_codes || atom == atom_codes_no_escape || atom == atom_chars
            || atom == atom_chars_no_escape || atom == atom_atom
            || atom == atom_atom_no_escape)) as libc::c_int;
}
unsafe extern "C" fn Fct_Set_Quotes(
    mut flag: *mut FlagInf,
    mut value_word: WamWord,
) -> Bool {
    let mut atom: libc::c_int = (value_word as PlULong >> 3 as libc::c_int)
        as libc::c_int;
    if atom == atom_codes {
        (*flag).value = 0 as libc::c_int as PlLong;
    } else if atom == atom_codes_no_escape {
        (*flag)
            .value = (0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
            as PlLong;
    } else if atom == atom_chars {
        (*flag).value = 1 as libc::c_int as PlLong;
    } else if atom == atom_chars_no_escape {
        (*flag)
            .value = (1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
            as PlLong;
    } else if atom == atom_atom {
        (*flag).value = 2 as libc::c_int as PlLong;
    } else {
        (*flag)
            .value = (2 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
            as PlLong;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_New_Prolog_Flag(
    mut name: *mut libc::c_char,
    mut modifiable: Bool,
    mut type_0: FlagType,
    mut value: PlLong,
    mut fct_get: FlagFctGet,
    mut fct_chk: FlagFctChk,
    mut fct_set: FlagFctSet,
) -> *mut FlagInf {
    let mut atom_name: libc::c_int = 0;
    let mut flag: *mut FlagInf = 0 as *mut FlagInf;
    Init_Flag_Supp();
    atom_name = Pl_Create_Atom(name);
    if nb_flag == 64 as libc::c_int {
        Pl_Fatal_Error(
            b"Flag table full - increase NB_OF_FLAGS = %d\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            64 as libc::c_int,
        );
    }
    let fresh0 = nb_flag;
    nb_flag = nb_flag + 1;
    flag = flag_tbl.as_mut_ptr().offset(fresh0 as isize);
    (*flag).atom_name = atom_name;
    (*flag).modifiable = modifiable;
    (*flag).type_0 = type_0;
    (*flag).value = value;
    (*flag).fct_get = fct_get;
    (*flag).fct_chk = fct_chk;
    (*flag).fct_set = fct_set;
    if fct_get.is_none() {
        match type_0 as libc::c_uint {
            0 => {
                (*flag)
                    .fct_get = Some(
                    Fct_Get_Integer as unsafe extern "C" fn(*mut FlagInf) -> WamWord,
                );
            }
            1 => {
                (*flag)
                    .fct_get = Some(
                    Fct_Get_Atom as unsafe extern "C" fn(*mut FlagInf) -> WamWord,
                );
            }
            3 => {
                (*flag)
                    .fct_get = Some(
                    Fct_Get_Bool as unsafe extern "C" fn(*mut FlagInf) -> WamWord,
                );
            }
            4 => {
                (*flag)
                    .fct_get = Some(
                    Fct_Get_On_Off as unsafe extern "C" fn(*mut FlagInf) -> WamWord,
                );
            }
            5 => {
                (*flag)
                    .fct_get = Some(
                    Fct_Get_Err as unsafe extern "C" fn(*mut FlagInf) -> WamWord,
                );
            }
            6 => {
                (*flag)
                    .fct_get = Some(
                    Fct_Get_Quotes as unsafe extern "C" fn(*mut FlagInf) -> WamWord,
                );
            }
            2 => {
                (*flag)
                    .fct_get = Some(
                    Fct_Get_Round as unsafe extern "C" fn(*mut FlagInf) -> WamWord,
                );
            }
            7 | _ => {}
        }
    }
    if fct_chk.is_none() {
        match type_0 as libc::c_uint {
            0 => {
                (*flag)
                    .fct_chk = Some(
                    Fct_Chk_Integer
                        as unsafe extern "C" fn(*mut FlagInf, WamWord, WamWord) -> Bool,
                );
            }
            1 => {
                (*flag)
                    .fct_chk = Some(
                    Fct_Chk_Atom
                        as unsafe extern "C" fn(*mut FlagInf, WamWord, WamWord) -> Bool,
                );
            }
            3 => {
                (*flag)
                    .fct_chk = Some(
                    Fct_Chk_Bool
                        as unsafe extern "C" fn(*mut FlagInf, WamWord, WamWord) -> Bool,
                );
            }
            4 => {
                (*flag)
                    .fct_chk = Some(
                    Fct_Chk_On_Off
                        as unsafe extern "C" fn(*mut FlagInf, WamWord, WamWord) -> Bool,
                );
            }
            5 => {
                (*flag)
                    .fct_chk = Some(
                    Fct_Chk_Err
                        as unsafe extern "C" fn(*mut FlagInf, WamWord, WamWord) -> Bool,
                );
            }
            6 => {
                (*flag)
                    .fct_chk = Some(
                    Fct_Chk_Quotes
                        as unsafe extern "C" fn(*mut FlagInf, WamWord, WamWord) -> Bool,
                );
            }
            2 => {
                (*flag)
                    .fct_chk = Some(
                    Fct_Chk_Round
                        as unsafe extern "C" fn(*mut FlagInf, WamWord, WamWord) -> Bool,
                );
            }
            7 | _ => {}
        }
    }
    if modifiable != 0 && fct_set.is_none() {
        match type_0 as libc::c_uint {
            0 => {
                (*flag)
                    .fct_set = Some(
                    Fct_Set_Integer
                        as unsafe extern "C" fn(*mut FlagInf, WamWord) -> Bool,
                );
            }
            1 => {
                (*flag)
                    .fct_set = Some(
                    Fct_Set_Atom as unsafe extern "C" fn(*mut FlagInf, WamWord) -> Bool,
                );
            }
            3 => {
                (*flag)
                    .fct_set = Some(
                    Fct_Set_Bool as unsafe extern "C" fn(*mut FlagInf, WamWord) -> Bool,
                );
            }
            4 => {
                (*flag)
                    .fct_set = Some(
                    Fct_Set_On_Off as unsafe extern "C" fn(*mut FlagInf, WamWord) -> Bool,
                );
            }
            5 => {
                (*flag)
                    .fct_set = Some(
                    Fct_Set_Err as unsafe extern "C" fn(*mut FlagInf, WamWord) -> Bool,
                );
            }
            6 => {
                (*flag)
                    .fct_set = Some(
                    Fct_Set_Quotes as unsafe extern "C" fn(*mut FlagInf, WamWord) -> Bool,
                );
            }
            2 => {
                (*flag)
                    .fct_set = Some(
                    Fct_Set_Round as unsafe extern "C" fn(*mut FlagInf, WamWord) -> Bool,
                );
            }
            7 | _ => {}
        }
    }
    return flag;
}
unsafe extern "C" fn Prolog_Flag_Lookup(mut atom_name: libc::c_int) -> *mut FlagInf {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nb_flag {
        if flag_tbl[i as usize].atom_name == atom_name {
            return flag_tbl.as_mut_ptr().offset(i as isize);
        }
        i += 1;
        i;
    }
    return 0 as *mut FlagInf;
}
pub unsafe extern "C" fn Pl_Set_Prolog_Flag_2(
    mut flag_word: WamWord,
    mut value_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom_name: libc::c_int = 0;
    let mut flag: *mut FlagInf = 0 as *mut FlagInf;
    atom_name = Pl_Rd_Atom_Check(flag_word);
    flag = Prolog_Flag_Lookup(atom_name);
    if flag.is_null() {
        Pl_Err_Domain(pl_domain_prolog_flag, flag_word);
    }
    let mut deref_last_word: WamWord = 0;
    word = value_word;
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
    value_word = word;
    if ((*flag).fct_chk).is_some()
        && (Some(((*flag).fct_chk).unwrap())).unwrap()(flag, tag_mask, value_word) == 0
    {
        word = Pl_Put_Structure(
            '+' as i32 as libc::c_uchar as libc::c_int,
            2 as libc::c_int,
        );
        Pl_Unify_Value(flag_word);
        Pl_Unify_Value(value_word);
        Pl_Err_Domain(pl_domain_flag_value, word);
    }
    if (*flag).modifiable == 0 {
        Pl_Err_Permission(
            pl_permission_operation_modify,
            pl_permission_type_flag,
            flag_word,
        );
    }
    return (Some(((*flag).fct_set).unwrap())).unwrap()(flag, value_word);
}
pub unsafe extern "C" fn Pl_Current_Prolog_Flag_2(
    mut flag_word: WamWord,
    mut value_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom_name: libc::c_int = 0;
    let mut flag: *mut FlagInf = 0 as *mut FlagInf;
    let mut i: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = flag_word;
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
        atom_name = Pl_Rd_Atom_Check(word);
        flag = Prolog_Flag_Lookup(atom_name);
        if flag.is_null() {
            Pl_Err_Domain(pl_domain_prolog_flag, flag_word);
        }
        return Pl_Unify((Some(((*flag).fct_get).unwrap())).unwrap()(flag), value_word);
    }
    i = 0 as libc::c_int;
    *pl_reg_bank.offset(0 as libc::c_int as isize) = flag_word;
    *pl_reg_bank.offset(1 as libc::c_int as isize) = value_word;
    *pl_reg_bank.offset(2 as libc::c_int as isize) = (i + 1 as libc::c_int) as WamWord;
    Pl_Create_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F70726F6C6F675F666C61675F616C74__a0),
            ),
        ),
        3 as libc::c_int,
    );
    flag = flag_tbl.as_mut_ptr().offset(i as isize);
    Pl_Get_Atom((*flag).atom_name, flag_word);
    return Pl_Unify((Some(((*flag).fct_get).unwrap())).unwrap()(flag), value_word);
}
pub unsafe extern "C" fn Pl_Current_Prolog_Flag_Alt_0() -> Bool {
    let mut flag_word: WamWord = 0;
    let mut value_word: WamWord = 0;
    let mut flag: *mut FlagInf = 0 as *mut FlagInf;
    let mut i: libc::c_int = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F70726F6C6F675F666C61675F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    flag_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    value_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    i = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    flag = flag_tbl.as_mut_ptr().offset(i as isize);
    if i + 1 as libc::c_int == nb_flag {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh1 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh1 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    } else {
        *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
            as *mut WamWord) = (i + 1 as libc::c_int) as WamWord;
    }
    Pl_Get_Atom((*flag).atom_name, flag_word);
    return Pl_Unify((Some(((*flag).fct_get).unwrap())).unwrap()(flag), value_word);
}
