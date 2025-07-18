use ::libc;
extern "C" {
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    static mut pl_vec_size: WamWord;
    static mut pl_vec_max_integer: WamWord;
    fn Pl_Fd_Use_Vector(fdv_adr: *mut WamWord) -> Bool;
    fn Pl_Define_Vector_Size(max_val: libc::c_int);
    static mut pl_type_fd_variable: libc::c_int;
    fn Pl_Rd_Positive_Check(start_word: WamWord) -> PlLong;
    fn Pl_Check_For_Un_List(start_word: WamWord);
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Err_Instantiation();
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
pub type VecWord = PlULong;
pub type Vector = *mut VecWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub extra_cstr: Bool,
    pub min: libc::c_int,
    pub max: libc::c_int,
    pub vec: Vector,
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
pub unsafe extern "C" fn Pl_Fd_Vector_Max_1(mut max_word: WamWord) -> Bool {
    return Pl_Un_Integer_Check(pl_vec_max_integer, max_word);
}
pub unsafe extern "C" fn Pl_Fd_Set_Vector_Max_1(mut max_word: WamWord) -> Bool {
    let mut max: PlLong = Pl_Rd_Positive_Check(max_word);
    if max > 1000000 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    Pl_Define_Vector_Size(max as libc::c_int);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Max_Integer_1(mut inf_word: WamWord) -> Bool {
    return Pl_Un_Integer_Check(
        (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int as PlLong,
        inf_word,
    );
}
pub unsafe extern "C" fn Pl_Fd_Min_2(
    mut fdv_word: WamWord,
    mut min_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut n: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = fdv_word;
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
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        n = (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
    } else {
        n = (*(((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range))
            .min;
    }
    return Pl_Un_Integer_Check(n as PlLong, min_word);
}
pub unsafe extern "C" fn Pl_Fd_Max_2(
    mut fdv_word: WamWord,
    mut max_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut n: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = fdv_word;
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
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        n = (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
    } else {
        n = (*(((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range))
            .max;
    }
    return Pl_Un_Integer_Check(n as PlLong, max_word);
}
pub unsafe extern "C" fn Pl_Fd_Dom_2(
    mut fdv_word: WamWord,
    mut list_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut x: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut vec_elem: libc::c_int = 0;
    Pl_Check_For_Un_List(list_word);
    let mut deref_last_word: WamWord = 0;
    word = fdv_word;
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
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        x = (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
        if Pl_Get_List(list_word) == 0 || Pl_Unify_Integer(x as PlLong) == 0 {
            return 0 as libc::c_int;
        }
        list_word = Pl_Unify_Variable();
    } else {
        fdv_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if ((*(fdv_adr
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range))
            .vec)
            .is_null()
        {
            end = (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .max;
            x = (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .min;
            while x <= end {
                if Pl_Get_List(list_word) == 0 || Pl_Unify_Integer(x as PlLong) == 0 {
                    return 0 as libc::c_int;
                }
                list_word = Pl_Unify_Variable();
                x += 1;
                x;
            }
        } else {
            let mut enum_end: Vector = ((*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .vec)
                .offset(pl_vec_size as isize);
            let mut enum_i: Vector = (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .vec;
            let mut enum_j: libc::c_int = 0;
            let mut enum_word: VecWord = 0;
            vec_elem = 0 as libc::c_int;
            loop {
                enum_word = *enum_i;
                enum_j = 0 as libc::c_int;
                loop {
                    let fresh0 = enum_j;
                    enum_j = enum_j + 1;
                    if !(fresh0 < 8 as libc::c_int * 8 as libc::c_int) {
                        break;
                    }
                    if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                        if Pl_Get_List(list_word) == 0
                            || Pl_Unify_Integer(vec_elem as PlLong) == 0
                        {
                            return 0 as libc::c_int;
                        }
                        list_word = Pl_Unify_Variable();
                    }
                    enum_word >>= 1 as libc::c_int;
                    vec_elem += 1;
                    vec_elem;
                }
                enum_i = enum_i.offset(1);
                if !(enum_i < enum_end) {
                    break;
                }
            }
        }
    }
    return Pl_Get_Nil(list_word);
}
pub unsafe extern "C" fn Pl_Fd_Size_2(
    mut fdv_word: WamWord,
    mut size_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut n: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = fdv_word;
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
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        n = 1 as libc::c_int;
    } else {
        n = *((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset((4 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    }
    return Pl_Un_Integer_Check(n as PlLong, size_word);
}
pub unsafe extern "C" fn Pl_Fd_Has_Extra_Cstr_1(mut fdv_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = fdv_word;
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
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    return (tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
        && (*(((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range))
            .extra_cstr != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Has_Vector_1(mut fdv_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = fdv_word;
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
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    return (tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
        && !((*(((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range))
            .vec)
            .is_null()) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Use_Vector_1(mut fdv_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = fdv_word;
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
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    return (tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        || Pl_Fd_Use_Vector(
            (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        ) != 0) as libc::c_int;
}
