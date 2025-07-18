use ::libc;
extern "C" {
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn Pl_Get_Integer_Tagged(w: WamWord, start_word: WamWord) -> Bool;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Blt_Compound(x: WamWord) -> Bool;
    static mut pl_type_fd_evaluable: libc::c_int;
    static mut pl_resource_too_big_fd_constraint: libc::c_int;
    fn Pl_Err_Resource(atom_resource: libc::c_int);
    fn Pl_Err_Instantiation();
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Fd_New_Variable() -> *mut WamWord;
    fn pl_x_eq_y(x: WamWord, y: WamWord) -> Bool;
    fn pl_x_plus_c_eq_y(x: WamWord, c: WamWord, y: WamWord) -> Bool;
    fn pl_x_eq_y_F(x: WamWord, y: WamWord) -> Bool;
    fn pl_x_plus_c_eq_y_F(x: WamWord, c: WamWord, y: WamWord) -> Bool;
    fn pl_x_gte_c(x: WamWord, c: WamWord) -> Bool;
    fn pl_ax_eq_y(a: WamWord, x: WamWord, y: WamWord) -> Bool;
    fn pl_x_plus_y_eq_z(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_ax_plus_y_eq_z(a: WamWord, x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_ax_plus_by_eq_z(
        a: WamWord,
        x: WamWord,
        b: WamWord,
        y: WamWord,
        z: WamWord,
    ) -> Bool;
    fn pl_x_plus_y_plus_z_eq_t(x: WamWord, y: WamWord, z: WamWord, t: WamWord) -> Bool;
    fn pl_ax_plus_y_plus_z_eq_t(
        a: WamWord,
        x: WamWord,
        y: WamWord,
        z: WamWord,
        t: WamWord,
    ) -> Bool;
    fn pl_ax_plus_by_plus_z_eq_t(
        a: WamWord,
        x: WamWord,
        b: WamWord,
        y: WamWord,
        z: WamWord,
        t: WamWord,
    ) -> Bool;
    fn pl_ax_eq_y_F(a: WamWord, x: WamWord, y: WamWord) -> Bool;
    fn pl_x_plus_y_eq_z_F(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_ax_plus_y_eq_z_F(a: WamWord, x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_ax_plus_by_eq_z_F(
        a: WamWord,
        x: WamWord,
        b: WamWord,
        y: WamWord,
        z: WamWord,
    ) -> Bool;
    fn pl_x_plus_y_plus_z_eq_t_F(x: WamWord, y: WamWord, z: WamWord, t: WamWord) -> Bool;
    fn pl_ax_plus_y_plus_z_eq_t_F(
        a: WamWord,
        x: WamWord,
        y: WamWord,
        z: WamWord,
        t: WamWord,
    ) -> Bool;
    fn pl_zero_power_n_eq_y(n: WamWord, y: WamWord) -> Bool;
    fn pl_a_power_n_eq_y(a: WamWord, n: WamWord, y: WamWord) -> Bool;
    fn pl_x_power_a_eq_y(x: WamWord, a: WamWord, y: WamWord) -> Bool;
    fn pl_x2_eq_y(x: WamWord, y: WamWord) -> Bool;
    fn pl_xy_eq_z(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_a_power_n_eq_y_F(a: WamWord, n: WamWord, y: WamWord) -> Bool;
    fn pl_x_power_a_eq_y_F(x: WamWord, a: WamWord, y: WamWord) -> Bool;
    fn pl_x2_eq_y_F(x: WamWord, y: WamWord) -> Bool;
    fn pl_xy_eq_z_F(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_min_x_a_eq_z(x: WamWord, a: WamWord, z: WamWord) -> Bool;
    fn pl_min_x_y_eq_z(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_min_x_a_eq_z_F(x: WamWord, a: WamWord, z: WamWord) -> Bool;
    fn pl_min_x_y_eq_z_F(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_max_x_a_eq_z(x: WamWord, a: WamWord, z: WamWord) -> Bool;
    fn pl_max_x_y_eq_z(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_max_x_a_eq_z_F(x: WamWord, a: WamWord, z: WamWord) -> Bool;
    fn pl_max_x_y_eq_z_F(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_abs_x_minus_a_eq_z(x: WamWord, a: WamWord, z: WamWord) -> Bool;
    fn pl_abs_x_minus_y_eq_z(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_abs_x_minus_a_eq_z_F(x: WamWord, a: WamWord, z: WamWord) -> Bool;
    fn pl_abs_x_minus_y_eq_z_F(x: WamWord, y: WamWord, z: WamWord) -> Bool;
    fn pl_quot_rem_a_y_r_eq_z(a: WamWord, y: WamWord, r: WamWord, z: WamWord) -> Bool;
    fn pl_quot_rem_x_a_r_eq_z(x: WamWord, a: WamWord, r: WamWord, z: WamWord) -> Bool;
    fn pl_quot_rem_x_y_r_eq_z(x: WamWord, y: WamWord, r: WamWord, z: WamWord) -> Bool;
    fn pl_quot_rem_a_y_r_eq_z_F(a: WamWord, y: WamWord, r: WamWord, z: WamWord) -> Bool;
    fn pl_quot_rem_x_a_r_eq_z_F(x: WamWord, a: WamWord, r: WamWord, z: WamWord) -> Bool;
    fn pl_quot_rem_x_y_r_eq_z_F(x: WamWord, y: WamWord, r: WamWord, z: WamWord) -> Bool;
    fn Pl_Power(x: libc::c_uint, n: libc::c_uint) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Poly {
    pub c: PlLong,
    pub nb_monom: libc::c_int,
    pub m: [Monom; 2000],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Monom {
    pub a: PlLong,
    pub x_word: WamWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NonLin {
    pub cstr: libc::c_int,
    pub a1: WamWord,
    pub a2: WamWord,
    pub a3: WamWord,
    pub res: WamWord,
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
pub static mut pl_full_ac: Bool = 0;
static mut arith_tbl: [WamWord; 13] = [0; 13];
static mut delay_cstr_stack: [NonLin; 1000] = [NonLin {
    cstr: 0,
    a1: 0,
    a2: 0,
    a3: 0,
    res: 0,
}; 1000];
static mut delay_sp: *mut NonLin = 0 as *const NonLin as *mut NonLin;
static mut vars_tbl: [WamWord; 100000] = [0; 100000];
static mut vars_sp: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut sort: Bool = 0;
pub unsafe extern "C" fn Pl_Load_Left_Right(
    mut optim_eq: Bool,
    mut le_word: WamWord,
    mut re_word: WamWord,
    mut mask: *mut libc::c_int,
    mut c: *mut PlLong,
    mut l_word: *mut WamWord,
    mut r_word: *mut WamWord,
) -> Bool {
    delay_sp = delay_cstr_stack.as_mut_ptr();
    vars_sp = vars_tbl.as_mut_ptr();
    return Load_Left_Right_Rec(optim_eq, le_word, re_word, mask, c, l_word, r_word);
}
pub unsafe extern "C" fn Pl_Term_Math_Loading(
    mut l_word: WamWord,
    mut r_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    if delay_sp != delay_cstr_stack.as_mut_ptr() {
        if Load_Delay_Cstr_Part() == 0 {
            return 0 as libc::c_int;
        }
    }
    loop {
        vars_sp = vars_sp.offset(-1);
        if !(vars_sp >= vars_tbl.as_mut_ptr()) {
            break;
        }
        let mut deref_last_word: WamWord = 0;
        word = *vars_sp;
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong && word != l_word
            && word != r_word
        {
            adr = word as *mut WamWord;
            fdv_adr = Pl_Fd_New_Variable();
            if adr
                < *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                || adr
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                    && adr < B
            {
                let fresh0 = TR;
                TR = TR.offset(1);
                *fresh0 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                    as WamWord;
            }
            *adr = (fdv_adr as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Load_Left_Right_Rec(
    mut optim_eq: Bool,
    mut le_word: WamWord,
    mut re_word: WamWord,
    mut mask: *mut libc::c_int,
    mut c: *mut PlLong,
    mut l_word: *mut WamWord,
    mut r_word: *mut WamWord,
) -> Bool {
    let mut p: Poly = Poly {
        c: 0,
        nb_monom: 0,
        m: [Monom { a: 0, x_word: 0 }; 2000],
    };
    let mut l_m: *mut Monom = 0 as *mut Monom;
    let mut r_m: *mut Monom = 0 as *mut Monom;
    let mut cur: *mut Monom = 0 as *mut Monom;
    let mut pos: *mut Monom = 0 as *mut Monom;
    let mut neg: *mut Monom = 0 as *mut Monom;
    let mut end: *mut Monom = 0 as *mut Monom;
    let mut l_nb_monom: libc::c_int = 0;
    let mut r_nb_monom: libc::c_int = 0;
    let mut pref_load_word: WamWord = 0;
    let mut i: libc::c_int = 0;
    sort = 0 as libc::c_int;
    p.nb_monom = 0 as libc::c_int;
    p.c = p.nb_monom as PlLong;
    if Normalize(le_word, 1 as libc::c_int, &mut p) == 0 {
        return 0 as libc::c_int;
    }
    if re_word as libc::c_ulong
        != (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
        && Normalize(re_word, -(1 as libc::c_int), &mut p) == 0
    {
        return 0 as libc::c_int;
    }
    if sort != 0 || p.nb_monom > 2000 as libc::c_int / 2 as libc::c_int {
        qsort(
            (p.m).as_mut_ptr() as *mut libc::c_void,
            p.nb_monom as size_t,
            ::std::mem::size_of::<Monom>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut Monom, *mut Monom) -> libc::c_int>,
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
            >(
                Some(
                    Compar_Monom
                        as unsafe extern "C" fn(*mut Monom, *mut Monom) -> libc::c_int,
                ),
            ),
        );
        i = 0 as libc::c_int;
        while i < p.nb_monom {
            if p.m[i as usize].a <= 0 as libc::c_int as libc::c_long {
                break;
            }
            i += 1;
            i;
        }
        l_m = (p.m).as_mut_ptr();
        l_nb_monom = i;
        while i < p.nb_monom {
            if p.m[i as usize].a >= 0 as libc::c_int as libc::c_long {
                break;
            }
            p.m[i as usize].a = -p.m[i as usize].a;
            i += 1;
            i;
        }
        r_m = l_m.offset(l_nb_monom as isize);
        r_nb_monom = i - l_nb_monom;
    } else {
        pos = (p.m).as_mut_ptr();
        end = pos.offset(p.nb_monom as isize);
        neg = end;
        cur = pos;
        while cur < end {
            if (*cur).a < 0 as libc::c_int as libc::c_long {
                (*neg).a = -(*cur).a;
                (*neg).x_word = (*cur).x_word;
                neg = neg.offset(1);
                neg;
            } else if (*cur).a > 0 as libc::c_int as libc::c_long {
                if cur != pos {
                    *pos = *cur;
                }
                pos = pos.offset(1);
                pos;
            }
            cur = cur.offset(1);
            cur;
        }
        l_m = (p.m).as_mut_ptr();
        l_nb_monom = pos.offset_from(l_m) as libc::c_long as libc::c_int;
        r_m = end;
        r_nb_monom = neg.offset_from(r_m) as libc::c_long as libc::c_int;
    }
    pref_load_word = (0 as libc::c_int as PlLong as libc::c_ulong)
        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    *mask = 0 as libc::c_int;
    if l_nb_monom != 0 {
        *mask |= 1 as libc::c_int;
        if optim_eq != 0 && p.c == 0 as libc::c_int as libc::c_long
            && r_nb_monom == 1 as libc::c_int
            && (*r_m.offset(0 as libc::c_int as isize)).a
                == 1 as libc::c_int as libc::c_long
        {
            pref_load_word = (*r_m.offset(0 as libc::c_int as isize)).x_word;
        }
        if Load_Poly(l_nb_monom, l_m, pref_load_word, l_word) == 0 {
            return 0 as libc::c_int;
        }
    }
    if r_nb_monom != 0 {
        *mask |= 2 as libc::c_int;
        if pref_load_word as libc::c_ulong
            == (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong)
        {
            if optim_eq != 0 && p.c == 0 as libc::c_int as libc::c_long
                && l_nb_monom != 0
            {
                pref_load_word = *l_word;
            }
            if Load_Poly(r_nb_monom, r_m, pref_load_word, r_word) == 0 {
                return 0 as libc::c_int;
            }
        }
    }
    *c = p.c;
    return 1 as libc::c_int;
}
unsafe extern "C" fn Load_Term_Into_Word(
    mut e_word: WamWord,
    mut load_word: *mut WamWord,
) -> Bool {
    let mut mask: libc::c_int = 0;
    let mut l_word: WamWord = 0;
    let mut r_word: WamWord = 0;
    let mut word: WamWord = 0;
    let mut c: PlLong = 0;
    if Load_Left_Right_Rec(
        0 as libc::c_int,
        e_word,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        &mut mask,
        &mut c,
        &mut l_word,
        &mut r_word,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if mask == 0 as libc::c_int {
        if c < 0 as libc::c_int as libc::c_long {
            return 0 as libc::c_int;
        }
        *load_word = ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
            as WamWord;
        return 1 as libc::c_int;
    }
    if mask == 1 as libc::c_int && c == 0 as libc::c_int as libc::c_long {
        *load_word = l_word;
        return 1 as libc::c_int;
    }
    *load_word = (Pl_Fd_New_Variable() as PlLong as libc::c_ulong)
        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    match mask {
        1 => {
            if c > 0 as libc::c_int as libc::c_long {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_x_plus_c_eq_y(
                        l_word,
                        ((c as PlULong) << 3 as libc::c_int
                            | 0x7 as libc::c_int as PlULong) as WamWord,
                        *load_word,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                } else if pl_x_plus_c_eq_y_F(
                    l_word,
                    ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                        as WamWord,
                    *load_word,
                ) == 0
                {
                    return 0 as libc::c_int
                }
            } else if pl_full_ac == 0 as libc::c_int {
                if pl_x_plus_c_eq_y(
                    *load_word,
                    ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                        as WamWord,
                    l_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if pl_x_plus_c_eq_y_F(
                *load_word,
                ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                    as WamWord,
                l_word,
            ) == 0
            {
                return 0 as libc::c_int
            }
            return 1 as libc::c_int;
        }
        2 => {
            if c < 0 as libc::c_int as libc::c_long {
                return 0 as libc::c_int;
            }
            word = (Pl_Fd_New_Variable() as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
            if pl_full_ac == 0 as libc::c_int {
                if pl_x_plus_y_eq_z(r_word, *load_word, word) == 0 {
                    return 0 as libc::c_int;
                }
            } else if pl_x_plus_y_eq_z_F(r_word, *load_word, word) == 0 {
                return 0 as libc::c_int
            }
            if pl_x_eq_c(
                word,
                ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                    as WamWord,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            return 1 as libc::c_int;
        }
        _ => {}
    }
    if c == 0 as libc::c_int as libc::c_long {
        if pl_full_ac == 0 as libc::c_int {
            if pl_x_plus_y_eq_z(r_word, *load_word, l_word) == 0 {
                return 0 as libc::c_int;
            }
        } else if pl_x_plus_y_eq_z_F(r_word, *load_word, l_word) == 0 {
            return 0 as libc::c_int
        }
        return 1 as libc::c_int;
    }
    word = (Pl_Fd_New_Variable() as PlLong as libc::c_ulong)
        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    if pl_full_ac == 0 as libc::c_int {
        if pl_x_plus_y_eq_z(r_word, *load_word, word) == 0 {
            return 0 as libc::c_int;
        }
    } else if pl_x_plus_y_eq_z_F(r_word, *load_word, word) == 0 {
        return 0 as libc::c_int
    }
    if c > 0 as libc::c_int as libc::c_long {
        if pl_full_ac == 0 as libc::c_int {
            if pl_x_plus_c_eq_y(
                l_word,
                ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                    as WamWord,
                word,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        } else if pl_x_plus_c_eq_y_F(
            l_word,
            ((c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                as WamWord,
            word,
        ) == 0
        {
            return 0 as libc::c_int
        }
    } else if pl_full_ac == 0 as libc::c_int {
        if pl_x_plus_c_eq_y(
            word,
            ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
                as WamWord,
            l_word,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    } else if pl_x_plus_c_eq_y_F(
        word,
        ((-c as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
        l_word,
    ) == 0
    {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Compar_Monom(
    mut m1: *mut Monom,
    mut m2: *mut Monom,
) -> libc::c_int {
    let mut cmp: PlLong = 0;
    if (*m1).a > 0 as libc::c_int as libc::c_long {
        cmp = if (*m2).a > 0 as libc::c_int as libc::c_long {
            (*m2).a - (*m1).a
        } else {
            -(1 as libc::c_int) as libc::c_long
        };
    } else {
        cmp = if (*m2).a > 0 as libc::c_int as libc::c_long {
            1 as libc::c_int as libc::c_long
        } else {
            (*m1).a - (*m2).a
        };
    }
    return if cmp > 0 as libc::c_int as libc::c_long {
        1 as libc::c_int
    } else if cmp == 0 as libc::c_int as libc::c_long {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
unsafe extern "C" fn Push_Delayed_Cstr(
    mut cstr: libc::c_int,
    mut a1: WamWord,
    mut a2: WamWord,
    mut a3: WamWord,
) -> WamWord {
    let mut res_word: WamWord = 0;
    res_word = (H as PlLong as libc::c_ulong).wrapping_add(0 as libc::c_int as PlULong)
        as WamWord;
    let fresh1 = H;
    H = H.offset(1);
    *fresh1 = res_word;
    if delay_sp.offset_from(delay_cstr_stack.as_mut_ptr()) as libc::c_long
        >= 1000 as libc::c_int as libc::c_long
    {
        Pl_Err_Resource(pl_resource_too_big_fd_constraint);
    }
    (*delay_sp).cstr = cstr;
    (*delay_sp).a1 = a1;
    (*delay_sp).a2 = a2;
    (*delay_sp).a3 = a3;
    (*delay_sp).res = res_word;
    delay_sp = delay_sp.offset(1);
    delay_sp;
    return res_word;
}
unsafe extern "C" fn Add_Monom(
    mut p: *mut Poly,
    mut sign: libc::c_int,
    mut a: PlLong,
    mut x_word: WamWord,
) {
    let mut i: libc::c_int = 0;
    if a == 0 as libc::c_int as libc::c_long {
        return;
    }
    if sign < 0 as libc::c_int {
        a = -a;
    }
    i = 0 as libc::c_int;
    while i < (*p).nb_monom {
        if (*p).m[i as usize].x_word == x_word {
            (*p).m[i as usize].a += a;
            return;
        }
        i += 1;
        i;
    }
    if (*p).nb_monom >= 2000 as libc::c_int {
        Pl_Err_Resource(pl_resource_too_big_fd_constraint);
    }
    (*p).m[(*p).nb_monom as usize].a = a;
    (*p).m[(*p).nb_monom as usize].x_word = x_word;
    (*p).nb_monom += 1;
    (*p).nb_monom;
}
unsafe extern "C" fn Add_Multiply_Monom(
    mut p: *mut Poly,
    mut sign: libc::c_int,
    mut m1: *mut Monom,
    mut m2: *mut Monom,
) -> Bool {
    let mut a: PlLong = 0;
    let mut x_word: WamWord = 0;
    a = (*m1).a * (*m2).a;
    if a == 0 as libc::c_int as libc::c_long {
        return 1 as libc::c_int;
    }
    x_word = if (*m1).x_word == (*m2).x_word {
        Push_Delayed_Cstr(
            0 as libc::c_int,
            (*m1).x_word,
            0 as libc::c_int as WamWord,
            0 as libc::c_int as WamWord,
        )
    } else {
        Push_Delayed_Cstr(
            1 as libc::c_int,
            (*m1).x_word,
            (*m2).x_word,
            0 as libc::c_int as WamWord,
        )
    };
    Add_Monom(p, sign, a, x_word);
    return 1 as libc::c_int;
}
unsafe extern "C" fn Normalize(
    mut e_word: WamWord,
    mut sign: libc::c_int,
    mut p: *mut Poly,
) -> Bool {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut word1: WamWord = 0;
    let mut word2: WamWord = 0;
    let mut word3: WamWord = 0;
    let mut f_n: WamWord = 0;
    let mut le_word: WamWord = 0;
    let mut re_word: WamWord = 0;
    let mut i: libc::c_int = 0;
    let mut n1: PlLong = 0;
    let mut n2: PlLong = 0;
    let mut n3: PlLong = 0;
    '_terminal_rec: loop {
        let mut deref_last_word: WamWord = 0;
        word = e_word;
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
        if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
            fdv_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            Add_Monom(
                p,
                sign,
                1 as libc::c_int as PlLong,
                (fdv_adr as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
            );
            return 1 as libc::c_int;
        }
        if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
            n1 = word << 0 as libc::c_int >> 3 as libc::c_int;
            if n1 > 100 as libc::c_int as libc::c_long {
                sort = 1 as libc::c_int;
            }
            (*p).c += sign as libc::c_long * n1;
            return 1 as libc::c_int;
        }
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            if vars_sp.offset_from(vars_tbl.as_mut_ptr()) as libc::c_long
                >= 100000 as libc::c_int as libc::c_long
            {
                Pl_Err_Resource(pl_resource_too_big_fd_constraint);
            }
            let fresh2 = vars_sp;
            vars_sp = vars_sp.offset(1);
            *fresh2 = word;
            Add_Monom(p, sign, 1 as libc::c_int as PlLong, word);
            return 1 as libc::c_int;
        }
        if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
            word = Pl_Put_Structure(
                '/' as i32 as libc::c_uchar as libc::c_int,
                2 as libc::c_int,
            );
            Pl_Unify_Value(e_word);
            Pl_Unify_Integer(0 as libc::c_int as PlLong);
            current_block = 16667723949402893533;
        } else {
            current_block = 14359455889292382949;
        }
        loop {
            match current_block {
                16667723949402893533 => {
                    Pl_Err_Type(pl_type_fd_evaluable, word);
                    current_block = 14359455889292382949;
                }
                _ => {
                    if tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong {
                        current_block = 16667723949402893533;
                        continue;
                    }
                    adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord;
                    f_n = *adr.offset(0 as libc::c_int as isize);
                    i = 0 as libc::c_int;
                    while i < 13 as libc::c_int {
                        if arith_tbl[i as usize] == f_n {
                            break;
                        }
                        i += 1;
                        i;
                    }
                    le_word = *adr
                        .offset((1 as libc::c_int + 0 as libc::c_int) as isize);
                    re_word = *adr
                        .offset((1 as libc::c_int + 1 as libc::c_int) as isize);
                    match i {
                        0 => {
                            e_word = le_word;
                            break;
                        }
                        1 => {
                            if Pl_Blt_Compound(le_word) == 0 {
                                if Normalize(le_word, sign, p) == 0 {
                                    return 0 as libc::c_int;
                                }
                                e_word = re_word;
                            } else {
                                if Normalize(re_word, sign, p) == 0 {
                                    return 0 as libc::c_int;
                                }
                                e_word = le_word;
                            }
                            break;
                        }
                        3 => {
                            if Pl_Blt_Compound(le_word) == 0 {
                                if Normalize(le_word, sign, p) == 0 {
                                    return 0 as libc::c_int;
                                }
                                e_word = re_word;
                                sign = -sign;
                            } else {
                                if Normalize(re_word, -sign, p) == 0 {
                                    return 0 as libc::c_int;
                                }
                                e_word = le_word;
                            }
                            break;
                        }
                        2 => {
                            e_word = le_word;
                            sign = -sign;
                            break;
                        }
                        4 => {
                            let mut deref_last_word_0: WamWord = 0;
                            word = le_word;
                            loop {
                                deref_last_word_0 = word;
                                tag_mask = (word as libc::c_ulong
                                    & 0x7 as libc::c_int as PlULong) as WamWord;
                                if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
                                {
                                    break;
                                }
                                word = *(word as *mut WamWord);
                                if !(word != deref_last_word_0) {
                                    break;
                                }
                            }
                            if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
                            {
                                current_block = 12921940486369188726;
                                break '_terminal_rec;
                            } else {
                                current_block = 317151059986244064;
                                break '_terminal_rec;
                            }
                        }
                        6 => {
                            if Load_Term_Into_Word(le_word, &mut word1) == 0
                                || Load_Term_Into_Word(re_word, &mut word2) == 0
                            {
                                return 0 as libc::c_int;
                            }
                            if word1 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                                == 0x7 as libc::c_int as PlULong
                            {
                                current_block = 1658462350791934405;
                                break '_terminal_rec;
                            } else {
                                current_block = 3575278370434307847;
                                break '_terminal_rec;
                            }
                        }
                        7 => {
                            if Load_Term_Into_Word(le_word, &mut word1) == 0
                                || Load_Term_Into_Word(re_word, &mut word2) == 0
                            {
                                return 0 as libc::c_int;
                            }
                            if word1 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                                == 0x7 as libc::c_int as PlULong
                            {
                                current_block = 14648249180243006330;
                                break '_terminal_rec;
                            } else {
                                current_block = 16910810822589621899;
                                break '_terminal_rec;
                            }
                        }
                        8 => {
                            if Load_Term_Into_Word(le_word, &mut word1) == 0
                                || Load_Term_Into_Word(re_word, &mut word2) == 0
                            {
                                return 0 as libc::c_int;
                            }
                            if word1 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                                == 0x7 as libc::c_int as PlULong
                            {
                                current_block = 12890877304563811856;
                                break '_terminal_rec;
                            } else {
                                current_block = 7157669805658135323;
                                break '_terminal_rec;
                            }
                        }
                        9 => {
                            if Load_Term_Into_Word(le_word, &mut word1) == 0
                                || Load_Term_Into_Word(re_word, &mut word2) == 0
                            {
                                return 0 as libc::c_int;
                            }
                            if word1 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                                == 0x7 as libc::c_int as PlULong
                            {
                                current_block = 1915186496383530739;
                                break '_terminal_rec;
                            } else {
                                current_block = 4534765400774009001;
                                break '_terminal_rec;
                            }
                        }
                        10 => {
                            word3 = (H as PlLong as libc::c_ulong)
                                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                            let fresh3 = H;
                            H = H.offset(1);
                            *fresh3 = word3;
                            current_block = 14540508397515857883;
                            break '_terminal_rec;
                        }
                        11 => {
                            word3 = (H as PlLong as libc::c_ulong)
                                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                            let fresh4 = H;
                            H = H.offset(1);
                            *fresh4 = word3;
                            current_block = 14540508397515857883;
                            break '_terminal_rec;
                        }
                        12 => {
                            current_block = 14540508397515857883;
                            break '_terminal_rec;
                        }
                        5 => {
                            if Load_Term_Into_Word(le_word, &mut word1) == 0
                                || Load_Term_Into_Word(re_word, &mut word2) == 0
                            {
                                return 0 as libc::c_int;
                            }
                            if word1 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                                == 0x7 as libc::c_int as PlULong
                            {
                                current_block = 2182835884935087477;
                                break '_terminal_rec;
                            } else {
                                current_block = 1587619384396752891;
                                break '_terminal_rec;
                            }
                        }
                        _ => {
                            word = Pl_Put_Structure(
                                '/' as i32 as libc::c_uchar as libc::c_int,
                                2 as libc::c_int,
                            );
                            Pl_Unify_Atom(
                                (*adr.offset(0 as libc::c_int as isize) as PlULong
                                    & ((1 as libc::c_int as PlULong)
                                        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                    as libc::c_int,
                            );
                            Pl_Unify_Integer(
                                (*adr.offset(0 as libc::c_int as isize) as PlULong
                                    >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as PlLong,
                            );
                            current_block = 16667723949402893533;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        1915186496383530739 => {
            n1 = word1 << 0 as libc::c_int >> 3 as libc::c_int;
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                n2 = word2 << 0 as libc::c_int >> 3 as libc::c_int;
                n1 = if n1 >= n2 { n1 - n2 } else { n2 - n1 };
                (*p).c += sign as libc::c_long * n1;
                return 1 as libc::c_int;
            }
            word = Push_Delayed_Cstr(
                12 as libc::c_int,
                word2,
                word1,
                0 as libc::c_int as WamWord,
            );
            current_block = 10316986533718329794;
        }
        14540508397515857883 => {
            if Load_Term_Into_Word(le_word, &mut word1) == 0
                || Load_Term_Into_Word(re_word, &mut word2) == 0
                || i == 12 as libc::c_int
                    && Load_Term_Into_Word(
                        *adr.offset((1 as libc::c_int + 2 as libc::c_int) as isize),
                        &mut word3,
                    ) == 0
            {
                return 0 as libc::c_int;
            }
            if word1 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                n1 = word1 << 0 as libc::c_int >> 3 as libc::c_int;
                if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                    == 0x7 as libc::c_int as PlULong
                {
                    n2 = word2 << 0 as libc::c_int >> 3 as libc::c_int;
                    if n2 == 0 as libc::c_int as libc::c_long {
                        return 0 as libc::c_int;
                    }
                    n3 = n1 % n2;
                    if i == 10 as libc::c_int || i == 12 as libc::c_int {
                        if i == 12 as libc::c_int {
                            if pl_x_eq_c(word3, word) == 0 {
                                return 0 as libc::c_int;
                            }
                        } else {
                            H = H.offset(-1);
                            H;
                        }
                        n3 = n1 / n2;
                    }
                    (*p).c += sign as libc::c_long * n3;
                    return 1 as libc::c_int;
                }
                word = Push_Delayed_Cstr(14 as libc::c_int, word1, word2, word3);
            } else if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                word = Push_Delayed_Cstr(15 as libc::c_int, word1, word2, word3);
            } else {
                word = Push_Delayed_Cstr(16 as libc::c_int, word1, word2, word3);
            }
            Add_Monom(
                p,
                sign,
                1 as libc::c_int as PlLong,
                if i == 11 as libc::c_int { word3 } else { word },
            );
            return 1 as libc::c_int;
        }
        12890877304563811856 => {
            n1 = word1 << 0 as libc::c_int >> 3 as libc::c_int;
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                n2 = word2 << 0 as libc::c_int >> 3 as libc::c_int;
                n1 = if n1 >= n2 { n1 } else { n2 };
                (*p).c += sign as libc::c_long * n1;
                return 1 as libc::c_int;
            }
            word = Push_Delayed_Cstr(
                10 as libc::c_int,
                word2,
                word1,
                0 as libc::c_int as WamWord,
            );
            current_block = 12949667897002365026;
        }
        14648249180243006330 => {
            n1 = word1 << 0 as libc::c_int >> 3 as libc::c_int;
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                n2 = word2 << 0 as libc::c_int >> 3 as libc::c_int;
                n1 = if n1 <= n2 { n1 } else { n2 };
                (*p).c += sign as libc::c_long * n1;
                return 1 as libc::c_int;
            }
            word = Push_Delayed_Cstr(
                8 as libc::c_int,
                word2,
                word1,
                0 as libc::c_int as WamWord,
            );
            current_block = 11404956602049528948;
        }
        1658462350791934405 => {
            n1 = word1 << 0 as libc::c_int >> 3 as libc::c_int;
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                n2 = word2 << 0 as libc::c_int >> 3 as libc::c_int;
                n1 = Pl_Power(n1 as libc::c_uint, n2 as libc::c_uint) as PlLong;
                if n1 < 0 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int;
                }
                (*p).c += sign as libc::c_long * n1;
                return 1 as libc::c_int;
            }
            if n1 == 1 as libc::c_int as libc::c_long {
                (*p).c += (sign * 1 as libc::c_int) as libc::c_long;
                return 1 as libc::c_int;
            }
            word = if n1 == 0 as libc::c_int as libc::c_long {
                Push_Delayed_Cstr(
                    5 as libc::c_int,
                    word2,
                    0 as libc::c_int as WamWord,
                    0 as libc::c_int as WamWord,
                )
            } else {
                Push_Delayed_Cstr(
                    6 as libc::c_int,
                    word1,
                    word2,
                    0 as libc::c_int as WamWord,
                )
            };
            current_block = 2914678948653924482;
        }
        317151059986244064 => {
            n1 = word << 0 as libc::c_int >> 3 as libc::c_int;
            if n1 > 100 as libc::c_int as libc::c_long {
                sort = 1 as libc::c_int;
            }
            let mut deref_last_word_1: WamWord = 0;
            word = re_word;
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
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                if tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong {
                    current_block = 12921940486369188726;
                } else {
                    fdv_adr = (word as libc::c_ulong
                        & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord;
                    word = (fdv_adr as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                    current_block = 8464383504555462953;
                }
            } else {
                current_block = 8464383504555462953;
            }
            match current_block {
                12921940486369188726 => {}
                _ => {
                    Add_Monom(p, sign, n1, word);
                    return 1 as libc::c_int;
                }
            }
        }
        2182835884935087477 => {
            n1 = word1 << 0 as libc::c_int >> 3 as libc::c_int;
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                n2 = word2 << 0 as libc::c_int >> 3 as libc::c_int;
                if n2 == 0 as libc::c_int as libc::c_long
                    || n1 % n2 != 0 as libc::c_int as libc::c_long
                {
                    return 0 as libc::c_int;
                }
                n1 /= n2;
                (*p).c += sign as libc::c_long * n1;
                return 1 as libc::c_int;
            }
            word = Push_Delayed_Cstr(
                2 as libc::c_int,
                word1,
                word2,
                0 as libc::c_int as WamWord,
            );
            current_block = 6055983839697916517;
        }
        3575278370434307847 => {
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                != 0x7 as libc::c_int as PlULong
            {
                Pl_Err_Instantiation();
            } else {
                n2 = word2 << 0 as libc::c_int >> 3 as libc::c_int;
                if n2 == 0 as libc::c_int as libc::c_long {
                    (*p).c += (sign * 1 as libc::c_int) as libc::c_long;
                    return 1 as libc::c_int;
                }
                word = if n2 == 1 as libc::c_int as libc::c_long {
                    word1
                } else if n2 == 2 as libc::c_int as libc::c_long {
                    Push_Delayed_Cstr(
                        0 as libc::c_int,
                        word1,
                        0 as libc::c_int as WamWord,
                        0 as libc::c_int as WamWord,
                    )
                } else {
                    Push_Delayed_Cstr(
                        7 as libc::c_int,
                        word1,
                        word2,
                        0 as libc::c_int as WamWord,
                    )
                };
            }
            current_block = 2914678948653924482;
        }
        16910810822589621899 => {
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                word = Push_Delayed_Cstr(
                    8 as libc::c_int,
                    word1,
                    word2,
                    0 as libc::c_int as WamWord,
                );
            } else {
                word = Push_Delayed_Cstr(
                    9 as libc::c_int,
                    word1,
                    word2,
                    0 as libc::c_int as WamWord,
                );
            }
            current_block = 11404956602049528948;
        }
        7157669805658135323 => {
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                word = Push_Delayed_Cstr(
                    10 as libc::c_int,
                    word1,
                    word2,
                    0 as libc::c_int as WamWord,
                );
            } else {
                word = Push_Delayed_Cstr(
                    11 as libc::c_int,
                    word1,
                    word2,
                    0 as libc::c_int as WamWord,
                );
            }
            current_block = 12949667897002365026;
        }
        4534765400774009001 => {
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                word = Push_Delayed_Cstr(
                    12 as libc::c_int,
                    word1,
                    word2,
                    0 as libc::c_int as WamWord,
                );
            } else {
                word = Push_Delayed_Cstr(
                    13 as libc::c_int,
                    word1,
                    word2,
                    0 as libc::c_int as WamWord,
                );
            }
            current_block = 10316986533718329794;
        }
        1587619384396752891 => {
            if word2 as libc::c_ulong & 0x7 as libc::c_int as PlULong
                == 0x7 as libc::c_int as PlULong
            {
                n2 = word2 << 0 as libc::c_int >> 3 as libc::c_int;
                if n2 == 0 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int;
                }
                word = Push_Delayed_Cstr(
                    3 as libc::c_int,
                    word1,
                    word2,
                    0 as libc::c_int as WamWord,
                );
            } else {
                word = Push_Delayed_Cstr(
                    4 as libc::c_int,
                    word1,
                    word2,
                    0 as libc::c_int as WamWord,
                );
            }
            current_block = 6055983839697916517;
        }
        _ => {}
    }
    match current_block {
        12921940486369188726 => {
            let mut p1: Poly = Poly {
                c: 0,
                nb_monom: 0,
                m: [Monom { a: 0, x_word: 0 }; 2000],
            };
            let mut p2: Poly = Poly {
                c: 0,
                nb_monom: 0,
                m: [Monom { a: 0, x_word: 0 }; 2000],
            };
            let mut i1: libc::c_int = 0;
            let mut i2: libc::c_int = 0;
            p1.nb_monom = 0 as libc::c_int;
            p1.c = p1.nb_monom as PlLong;
            p2.nb_monom = 0 as libc::c_int;
            p2.c = p2.nb_monom as PlLong;
            if Normalize(le_word, 1 as libc::c_int, &mut p1) == 0
                || Normalize(re_word, 1 as libc::c_int, &mut p2) == 0
            {
                return 0 as libc::c_int;
            }
            (*p).c += sign as libc::c_long * p1.c * p2.c;
            i1 = 0 as libc::c_int;
            while i1 < p1.nb_monom {
                Add_Monom(p, sign, p1.m[i1 as usize].a * p2.c, p1.m[i1 as usize].x_word);
                i2 = 0 as libc::c_int;
                while i2 < p2.nb_monom {
                    if Add_Multiply_Monom(
                        p,
                        sign,
                        (p1.m).as_mut_ptr().offset(i1 as isize),
                        (p2.m).as_mut_ptr().offset(i2 as isize),
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                    i2 += 1;
                    i2;
                }
                i1 += 1;
                i1;
            }
            i2 = 0 as libc::c_int;
            while i2 < p2.nb_monom {
                Add_Monom(p, sign, p2.m[i2 as usize].a * p1.c, p2.m[i2 as usize].x_word);
                i2 += 1;
                i2;
            }
            return 1 as libc::c_int;
        }
        12949667897002365026 => {
            Add_Monom(p, sign, 1 as libc::c_int as PlLong, word);
            return 1 as libc::c_int;
        }
        11404956602049528948 => {
            Add_Monom(p, sign, 1 as libc::c_int as PlLong, word);
            return 1 as libc::c_int;
        }
        2914678948653924482 => {
            Add_Monom(p, sign, 1 as libc::c_int as PlLong, word);
            return 1 as libc::c_int;
        }
        6055983839697916517 => {
            Add_Monom(p, sign, 1 as libc::c_int as PlLong, word);
            return 1 as libc::c_int;
        }
        _ => {
            Add_Monom(p, sign, 1 as libc::c_int as PlLong, word);
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn Load_Poly(
    mut nb_monom: libc::c_int,
    mut m: *mut Monom,
    mut pref_load_word: WamWord,
    mut load_word: *mut WamWord,
) -> Bool {
    if nb_monom == 1 as libc::c_int
        && (*m.offset(0 as libc::c_int as isize)).a == 1 as libc::c_int as libc::c_long
    {
        if pref_load_word as libc::c_ulong
            != (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong)
        {
            if Pl_Fd_Math_Unify_X_Y(
                (*m.offset(0 as libc::c_int as isize)).x_word,
                pref_load_word,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            *load_word = pref_load_word;
            return 1 as libc::c_int;
        }
        *load_word = (*m.offset(0 as libc::c_int as isize)).x_word;
        return 1 as libc::c_int;
    }
    if pref_load_word as libc::c_ulong
        != (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        *load_word = pref_load_word;
    } else {
        *load_word = (Pl_Fd_New_Variable() as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    return Load_Poly_Rec(nb_monom, m, *load_word);
}
unsafe extern "C" fn Load_Poly_Rec(
    mut nb_monom: libc::c_int,
    mut m: *mut Monom,
    mut load_word: WamWord,
) -> Bool {
    let mut load_word1: WamWord = 0;
    if nb_monom == 1 as libc::c_int {
        if pl_full_ac == 0 as libc::c_int {
            if pl_ax_eq_y(
                (((*m.offset(0 as libc::c_int as isize)).a as PlULong)
                    << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                (*m.offset(0 as libc::c_int as isize)).x_word,
                load_word,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        } else if pl_ax_eq_y_F(
            (((*m.offset(0 as libc::c_int as isize)).a as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord,
            (*m.offset(0 as libc::c_int as isize)).x_word,
            load_word,
        ) == 0
        {
            return 0 as libc::c_int
        }
        return 1 as libc::c_int;
    }
    if nb_monom == 2 as libc::c_int {
        if (*m.offset(0 as libc::c_int as isize)).a == 1 as libc::c_int as libc::c_long {
            if (*m.offset(1 as libc::c_int as isize)).a
                == 1 as libc::c_int as libc::c_long
            {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_x_plus_y_eq_z(
                        (*m.offset(0 as libc::c_int as isize)).x_word,
                        (*m.offset(1 as libc::c_int as isize)).x_word,
                        load_word,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                } else if pl_x_plus_y_eq_z_F(
                    (*m.offset(0 as libc::c_int as isize)).x_word,
                    (*m.offset(1 as libc::c_int as isize)).x_word,
                    load_word,
                ) == 0
                {
                    return 0 as libc::c_int
                }
            } else if pl_full_ac == 0 as libc::c_int {
                if pl_ax_plus_y_eq_z(
                    (((*m.offset(1 as libc::c_int as isize)).a as PlULong)
                        << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                    (*m.offset(1 as libc::c_int as isize)).x_word,
                    (*m.offset(0 as libc::c_int as isize)).x_word,
                    load_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if pl_ax_plus_y_eq_z_F(
                (((*m.offset(1 as libc::c_int as isize)).a as PlULong)
                    << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                (*m.offset(1 as libc::c_int as isize)).x_word,
                (*m.offset(0 as libc::c_int as isize)).x_word,
                load_word,
            ) == 0
            {
                return 0 as libc::c_int
            }
        } else if (*m.offset(1 as libc::c_int as isize)).a
            == 1 as libc::c_int as libc::c_long
        {
            if pl_full_ac == 0 as libc::c_int {
                if pl_ax_plus_y_eq_z(
                    (((*m.offset(0 as libc::c_int as isize)).a as PlULong)
                        << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                    (*m.offset(0 as libc::c_int as isize)).x_word,
                    (*m.offset(1 as libc::c_int as isize)).x_word,
                    load_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if pl_ax_plus_y_eq_z_F(
                (((*m.offset(0 as libc::c_int as isize)).a as PlULong)
                    << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                (*m.offset(0 as libc::c_int as isize)).x_word,
                (*m.offset(1 as libc::c_int as isize)).x_word,
                load_word,
            ) == 0
            {
                return 0 as libc::c_int
            }
        } else if pl_full_ac == 0 as libc::c_int {
            if pl_ax_plus_by_eq_z(
                (((*m.offset(0 as libc::c_int as isize)).a as PlULong)
                    << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                (*m.offset(0 as libc::c_int as isize)).x_word,
                (((*m.offset(1 as libc::c_int as isize)).a as PlULong)
                    << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                (*m.offset(1 as libc::c_int as isize)).x_word,
                load_word,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        } else if pl_ax_plus_by_eq_z_F(
            (((*m.offset(0 as libc::c_int as isize)).a as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord,
            (*m.offset(0 as libc::c_int as isize)).x_word,
            (((*m.offset(1 as libc::c_int as isize)).a as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord,
            (*m.offset(1 as libc::c_int as isize)).x_word,
            load_word,
        ) == 0
        {
            return 0 as libc::c_int
        }
        return 1 as libc::c_int;
    }
    if nb_monom == 3 as libc::c_int
        && (*m.offset(2 as libc::c_int as isize)).a == 1 as libc::c_int as libc::c_long
    {
        load_word1 = (*m.offset(2 as libc::c_int as isize)).x_word;
    } else {
        load_word1 = (Pl_Fd_New_Variable() as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    if (*m.offset(0 as libc::c_int as isize)).a == 1 as libc::c_int as libc::c_long {
        if (*m.offset(1 as libc::c_int as isize)).a == 1 as libc::c_int as libc::c_long {
            if pl_full_ac == 0 as libc::c_int {
                if pl_x_plus_y_plus_z_eq_t(
                    (*m.offset(0 as libc::c_int as isize)).x_word,
                    (*m.offset(1 as libc::c_int as isize)).x_word,
                    load_word1,
                    load_word,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if pl_x_plus_y_plus_z_eq_t_F(
                (*m.offset(0 as libc::c_int as isize)).x_word,
                (*m.offset(1 as libc::c_int as isize)).x_word,
                load_word1,
                load_word,
            ) == 0
            {
                return 0 as libc::c_int
            }
        } else if pl_full_ac == 0 as libc::c_int {
            if pl_ax_plus_y_plus_z_eq_t(
                (((*m.offset(1 as libc::c_int as isize)).a as PlULong)
                    << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                (*m.offset(1 as libc::c_int as isize)).x_word,
                (*m.offset(0 as libc::c_int as isize)).x_word,
                load_word1,
                load_word,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        } else if pl_ax_plus_y_plus_z_eq_t_F(
            (((*m.offset(1 as libc::c_int as isize)).a as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord,
            (*m.offset(1 as libc::c_int as isize)).x_word,
            (*m.offset(0 as libc::c_int as isize)).x_word,
            load_word1,
            load_word,
        ) == 0
        {
            return 0 as libc::c_int
        }
    } else if (*m.offset(1 as libc::c_int as isize)).a
        == 1 as libc::c_int as libc::c_long
    {
        if pl_full_ac == 0 as libc::c_int {
            if pl_ax_plus_y_plus_z_eq_t(
                (((*m.offset(0 as libc::c_int as isize)).a as PlULong)
                    << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
                (*m.offset(0 as libc::c_int as isize)).x_word,
                (*m.offset(1 as libc::c_int as isize)).x_word,
                load_word1,
                load_word,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        } else if pl_ax_plus_y_plus_z_eq_t_F(
            (((*m.offset(0 as libc::c_int as isize)).a as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord,
            (*m.offset(0 as libc::c_int as isize)).x_word,
            (*m.offset(1 as libc::c_int as isize)).x_word,
            load_word1,
            load_word,
        ) == 0
        {
            return 0 as libc::c_int
        }
    } else if pl_ax_plus_by_plus_z_eq_t(
        (((*m.offset(0 as libc::c_int as isize)).a as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord,
        (*m.offset(0 as libc::c_int as isize)).x_word,
        (((*m.offset(1 as libc::c_int as isize)).a as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord,
        (*m.offset(1 as libc::c_int as isize)).x_word,
        load_word1,
        load_word,
    ) == 0
    {
        return 0 as libc::c_int
    }
    if !(nb_monom == 3 as libc::c_int
        && (*m.offset(2 as libc::c_int as isize)).a == 1 as libc::c_int as libc::c_long)
    {
        return Load_Poly_Rec(
            nb_monom - 2 as libc::c_int,
            m.offset(2 as libc::c_int as isize),
            load_word1,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Load_Delay_Cstr_Part() -> Bool {
    let mut i: *mut NonLin = 0 as *mut NonLin;
    i = delay_cstr_stack.as_mut_ptr();
    while i < delay_sp {
        match (*i).cstr {
            0 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_x2_eq_y((*i).a1, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_x2_eq_y_F((*i).a1, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            1 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_xy_eq_z((*i).a1, (*i).a2, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_xy_eq_z_F((*i).a1, (*i).a2, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            2 => {
                if pl_x_gte_c(
                    (*i).a2,
                    ((1 as libc::c_int as PlULong) << 3 as libc::c_int
                        | 0x7 as libc::c_int as PlULong) as WamWord,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                if pl_full_ac == 0 as libc::c_int {
                    if pl_xy_eq_z((*i).res, (*i).a2, (*i).a1) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_xy_eq_z_F((*i).res, (*i).a2, (*i).a1) == 0 {
                    return 0 as libc::c_int
                }
            }
            3 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_ax_eq_y((*i).a2, (*i).res, (*i).a1) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_ax_eq_y_F((*i).a2, (*i).res, (*i).a1) == 0 {
                    return 0 as libc::c_int
                }
            }
            4 => {
                if pl_x_gte_c(
                    (*i).a2,
                    ((1 as libc::c_int as PlULong) << 3 as libc::c_int
                        | 0x7 as libc::c_int as PlULong) as WamWord,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                if pl_full_ac == 0 as libc::c_int {
                    if pl_xy_eq_z((*i).res, (*i).a2, (*i).a1) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_xy_eq_z_F((*i).res, (*i).a2, (*i).a1) == 0 {
                    return 0 as libc::c_int
                }
            }
            5 => {
                if pl_zero_power_n_eq_y((*i).a1, (*i).res) == 0 {
                    return 0 as libc::c_int;
                }
            }
            6 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_a_power_n_eq_y((*i).a1, (*i).a2, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_a_power_n_eq_y_F((*i).a1, (*i).a2, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            7 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_x_power_a_eq_y((*i).a1, (*i).a2, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_x_power_a_eq_y_F((*i).a1, (*i).a2, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            8 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_min_x_a_eq_z((*i).a1, (*i).a2, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_min_x_a_eq_z_F((*i).a1, (*i).a2, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            9 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_min_x_y_eq_z((*i).a1, (*i).a2, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_min_x_y_eq_z_F((*i).a1, (*i).a2, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            10 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_max_x_a_eq_z((*i).a1, (*i).a2, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_max_x_a_eq_z_F((*i).a1, (*i).a2, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            11 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_max_x_y_eq_z((*i).a1, (*i).a2, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_max_x_y_eq_z_F((*i).a1, (*i).a2, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            12 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_abs_x_minus_a_eq_z((*i).a1, (*i).a2, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_abs_x_minus_a_eq_z_F((*i).a1, (*i).a2, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            13 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_abs_x_minus_y_eq_z((*i).a1, (*i).a2, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_abs_x_minus_y_eq_z_F((*i).a1, (*i).a2, (*i).res) == 0 {
                    return 0 as libc::c_int
                }
            }
            14 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_quot_rem_a_y_r_eq_z((*i).a1, (*i).a2, (*i).a3, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_quot_rem_a_y_r_eq_z_F((*i).a1, (*i).a2, (*i).a3, (*i).res)
                    == 0
                {
                    return 0 as libc::c_int
                }
            }
            15 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_quot_rem_x_a_r_eq_z((*i).a1, (*i).a2, (*i).a3, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_quot_rem_x_a_r_eq_z_F((*i).a1, (*i).a2, (*i).a3, (*i).res)
                    == 0
                {
                    return 0 as libc::c_int
                }
            }
            16 => {
                if pl_full_ac == 0 as libc::c_int {
                    if pl_quot_rem_x_y_r_eq_z((*i).a1, (*i).a2, (*i).a3, (*i).res) == 0 {
                        return 0 as libc::c_int;
                    }
                } else if pl_quot_rem_x_y_r_eq_z_F((*i).a1, (*i).a2, (*i).a3, (*i).res)
                    == 0
                {
                    return 0 as libc::c_int
                }
            }
            _ => {}
        }
        i = i.offset(1);
        i;
    }
    delay_sp = delay_cstr_stack.as_mut_ptr();
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Math_Unify_X_Y(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut x_word: WamWord = 0;
    let mut x_tag: WamWord = 0;
    let mut y_word: WamWord = 0;
    let mut y_tag: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    x_word = x;
    loop {
        deref_last_word = x_word;
        x_tag = (x_word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if x_tag as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        x_word = *(x_word as *mut WamWord);
        if !(x_word != deref_last_word) {
            break;
        }
    }
    let mut deref_last_word_0: WamWord = 0;
    y_word = y;
    loop {
        deref_last_word_0 = y_word;
        y_tag = (y_word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if y_tag as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        y_word = *(y_word as *mut WamWord);
        if !(y_word != deref_last_word_0) {
            break;
        }
    }
    if x_tag as libc::c_ulong == 0x5 as libc::c_int as PlULong
        && y_tag as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        if pl_full_ac == 0 as libc::c_int {
            if pl_x_eq_y(x, y) == 0 {
                return 0 as libc::c_int;
            }
        } else if pl_x_eq_y_F(x, y) == 0 {
            return 0 as libc::c_int
        }
        return 1 as libc::c_int;
    }
    return Pl_Unify(x_word, y_word);
}
pub unsafe extern "C" fn pl_x_eq_c(mut x: WamWord, mut c: WamWord) -> Bool {
    return Pl_Get_Integer_Tagged(c, x);
}
