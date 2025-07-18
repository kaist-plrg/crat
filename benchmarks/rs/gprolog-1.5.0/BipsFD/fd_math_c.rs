use ::libc;
extern "C" {
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_Fd_Prolog_To_Fd_Var(arg_word: WamWord, pl_var_ok: Bool) -> *mut WamWord;
    static mut pl_full_ac: Bool;
    fn Pl_Load_Left_Right(
        optim_eq: Bool,
        le_word: WamWord,
        re_word: WamWord,
        mask: *mut libc::c_int,
        c: *mut PlLong,
        l_word: *mut WamWord,
        r_word: *mut WamWord,
    ) -> Bool;
    fn Pl_Term_Math_Loading(l_word: WamWord, r_word: WamWord) -> Bool;
    fn pl_x_eq_c(x: WamWord, c: WamWord) -> Bool;
    fn pl_x_plus_c_eq_y(x: WamWord, c: WamWord, y: WamWord) -> Bool;
    fn pl_x_plus_c_eq_y_F(x: WamWord, c: WamWord, y: WamWord) -> Bool;
    fn pl_x_neq_c(x: WamWord, c: WamWord) -> Bool;
    fn pl_x_neq_y(x: WamWord, y: WamWord) -> Bool;
    fn pl_x_plus_c_neq_y(x: WamWord, c: WamWord, y: WamWord) -> Bool;
    fn pl_x_lt_y(x: WamWord, y: WamWord) -> Bool;
    fn pl_x_lte_c(x: WamWord, c: WamWord) -> Bool;
    fn pl_x_lte_y(x: WamWord, y: WamWord) -> Bool;
    fn pl_x_plus_c_lte_y(x: WamWord, c: WamWord, y: WamWord) -> Bool;
    fn pl_x_gte_c(x: WamWord, c: WamWord) -> Bool;
    fn pl_x_plus_c_gte_y(x: WamWord, c: WamWord, y: WamWord) -> Bool;
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Fd_Set_Full_Ac_Flag_1(mut full_ac_word: WamWord) {
    pl_full_ac = Pl_Rd_Integer(full_ac_word) as Bool;
}
pub unsafe extern "C" fn Pl_Fd_Eq_2(mut le_word: WamWord, mut re_word: WamWord) -> Bool {
    let mut mask: libc::c_int = 0;
    let mut l_word: WamWord = 0;
    let mut r_word: WamWord = 0;
    let mut c: PlLong = 0;
    if Pl_Load_Left_Right(
        1 as libc::c_int,
        le_word,
        re_word,
        &mut mask,
        &mut c,
        &mut l_word,
        &mut r_word,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    match mask {
        0 => {
            if c != 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
        }
        1 => {
            if c > 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
            if pl_x_eq_c(
                l_word,
                ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                    as WamWord,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        2 => {
            if c < 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
            if pl_x_eq_c(
                r_word,
                ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                    as WamWord,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        _ => {
            if c > 0 as libc::c_int as libc::c_long {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_x_plus_c_eq_y(
                        l_word,
                        ((c as PlULong) << 3 as libc::c_int
                            | 0x7 as libc::c_int as PlULong) as WamWord,
                        r_word,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                } else if pl_x_plus_c_eq_y_F(
                    l_word,
                    ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                        as WamWord,
                    r_word,
                ) == 0
                {
                    return 0 as libc::c_int
                }
            } else if c < 0 as libc::c_int as libc::c_long {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_x_plus_c_eq_y(
                        r_word,
                        ((-c as PlULong) << 3 as libc::c_int
                            | 0x7 as libc::c_int as PlULong) as WamWord,
                        l_word,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                } else if pl_x_plus_c_eq_y_F(
                    r_word,
                    ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                        as WamWord,
                    l_word,
                ) == 0
                {
                    return 0 as libc::c_int
                }
            }
        }
    }
    return Pl_Term_Math_Loading(l_word, r_word);
}
pub unsafe extern "C" fn Pl_Fd_Neq_2(
    mut le_word: WamWord,
    mut re_word: WamWord,
) -> Bool {
    let mut mask: libc::c_int = 0;
    let mut l_word: WamWord = 0;
    let mut r_word: WamWord = 0;
    let mut c: PlLong = 0;
    if Pl_Load_Left_Right(
        0 as libc::c_int,
        le_word,
        re_word,
        &mut mask,
        &mut c,
        &mut l_word,
        &mut r_word,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    match mask {
        0 => {
            if c == 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
        }
        1 => {
            if c > 0 as libc::c_int as libc::c_long {
                Pl_Fd_Prolog_To_Fd_Var(l_word, 1 as libc::c_int);
            } else if pl_x_neq_c(
                l_word,
                ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                    as WamWord,
            ) == 0
            {
                return 0 as libc::c_int
            }
        }
        2 => {
            if c < 0 as libc::c_int as libc::c_long {
                Pl_Fd_Prolog_To_Fd_Var(r_word, 1 as libc::c_int);
            } else if pl_x_neq_c(
                r_word,
                ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                    as WamWord,
            ) == 0
            {
                return 0 as libc::c_int
            }
        }
        _ => {
            if c > 0 as libc::c_int as libc::c_long {
                if pl_x_plus_c_neq_y(
                    l_word,
                    ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                        as WamWord,
                    r_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if c < 0 as libc::c_int as libc::c_long {
                if pl_x_plus_c_neq_y(
                    r_word,
                    ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                        as WamWord,
                    l_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if pl_x_neq_y(l_word, r_word) == 0 {
                return 0 as libc::c_int
            }
        }
    }
    return Pl_Term_Math_Loading(l_word, r_word);
}
pub unsafe extern "C" fn Pl_Fd_Lt_2(mut le_word: WamWord, mut re_word: WamWord) -> Bool {
    let mut mask: libc::c_int = 0;
    let mut l_word: WamWord = 0;
    let mut r_word: WamWord = 0;
    let mut c: PlLong = 0;
    if Pl_Load_Left_Right(
        0 as libc::c_int,
        le_word,
        re_word,
        &mut mask,
        &mut c,
        &mut l_word,
        &mut r_word,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    match mask {
        0 => {
            if c >= 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
        }
        1 => {
            if c >= 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
            if pl_x_lte_c(
                l_word,
                (((-c - 1 as libc::c_int as libc::c_long) as PlULong) << 3 as libc::c_int
                    | 0x7 as libc::c_int as PlULong) as WamWord,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        2 => {
            if c < 0 as libc::c_int as libc::c_long {
                Pl_Fd_Prolog_To_Fd_Var(r_word, 1 as libc::c_int);
            } else if pl_x_gte_c(
                r_word,
                (((c + 1 as libc::c_int as libc::c_long) as PlULong) << 3 as libc::c_int
                    | 0x7 as libc::c_int as PlULong) as WamWord,
            ) == 0
            {
                return 0 as libc::c_int
            }
        }
        _ => {
            if c > 0 as libc::c_int as libc::c_long {
                if pl_x_plus_c_lte_y(
                    l_word,
                    (((c + 1 as libc::c_int as libc::c_long) as PlULong)
                        << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                    r_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if c < 0 as libc::c_int as libc::c_long {
                if pl_x_plus_c_gte_y(
                    r_word,
                    (((-c - 1 as libc::c_int as libc::c_long) as PlULong)
                        << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                    l_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if pl_x_lt_y(l_word, r_word) == 0 {
                return 0 as libc::c_int
            }
        }
    }
    return Pl_Term_Math_Loading(l_word, r_word);
}
pub unsafe extern "C" fn Pl_Fd_Lte_2(
    mut le_word: WamWord,
    mut re_word: WamWord,
) -> Bool {
    let mut mask: libc::c_int = 0;
    let mut l_word: WamWord = 0;
    let mut r_word: WamWord = 0;
    let mut c: PlLong = 0;
    if Pl_Load_Left_Right(
        0 as libc::c_int,
        le_word,
        re_word,
        &mut mask,
        &mut c,
        &mut l_word,
        &mut r_word,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    match mask {
        0 => {
            if c > 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
        }
        1 => {
            if c > 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
            if pl_x_lte_c(
                l_word,
                ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                    as WamWord,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        }
        2 => {
            if c <= 0 as libc::c_int as libc::c_long {
                Pl_Fd_Prolog_To_Fd_Var(r_word, 1 as libc::c_int);
            } else if pl_x_gte_c(
                r_word,
                ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                    as WamWord,
            ) == 0
            {
                return 0 as libc::c_int
            }
        }
        _ => {
            if c > 0 as libc::c_int as libc::c_long {
                if pl_x_plus_c_lte_y(
                    l_word,
                    ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                        as WamWord,
                    r_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if c < 0 as libc::c_int as libc::c_long {
                if pl_x_plus_c_gte_y(
                    r_word,
                    ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                        as WamWord,
                    l_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if pl_x_lte_y(l_word, r_word) == 0 {
                return 0 as libc::c_int
            }
        }
    }
    return Pl_Term_Math_Loading(l_word, r_word);
}
