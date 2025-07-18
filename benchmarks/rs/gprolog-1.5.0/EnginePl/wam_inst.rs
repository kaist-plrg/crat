use ::libc;
extern "C" {
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Calloc_Check(
        nb: libc::c_uint,
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    static mut pl_fd_unify_with_integer: Option::<unsafe extern "C" fn() -> Bool>;
    static mut pl_fd_unify_with_fd_var: Option::<unsafe extern "C" fn() -> Bool>;
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamCont = CodePtr;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SwtInf {
    pub key: PlLong,
    pub codep: CodePtr,
}
pub type SwtTbl = *mut SwtInf;
#[derive(Copy, Clone)]
#[repr(C)]
pub union DblInt {
    pub d: libc::c_double,
    pub i: [WamWord; 2],
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub unsafe extern "C" fn Pl_Create_Functor_Arity_Tagged(
    mut func_str: *mut libc::c_char,
    mut arity: libc::c_int,
) -> WamWord {
    let mut func: libc::c_int = Pl_Create_Atom(func_str);
    return ((arity as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Create_Swt_Table(mut size: libc::c_int) -> SwtTbl {
    let mut t: SwtTbl = 0 as *mut SwtInf;
    size += 1;
    size;
    t = Pl_Calloc_Check(
        size as libc::c_uint,
        ::std::mem::size_of::<SwtInf>() as libc::c_ulong as libc::c_uint,
        b"wam_inst.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        107 as libc::c_int,
    ) as SwtTbl;
    return t;
}
pub unsafe extern "C" fn Pl_Create_Swt_Atm_Element(
    mut t: SwtTbl,
    mut size: libc::c_int,
    mut atom: libc::c_int,
    mut codep: CodePtr,
) {
    let mut swt: *mut SwtInf = Locate_Swt_Element(t, size, atom as PlLong);
    (*swt).key = atom as PlLong;
    (*swt).codep = codep;
}
pub unsafe extern "C" fn Pl_Create_Swt_Stc_Element(
    mut t: SwtTbl,
    mut size: libc::c_int,
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut codep: CodePtr,
) {
    let mut key: PlLong = ((arity as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        as PlLong;
    let mut swt: *mut SwtInf = Locate_Swt_Element(t, size, key);
    (*swt).key = key;
    (*swt).codep = codep;
}
unsafe extern "C" fn Locate_Swt_Element(
    mut t: SwtTbl,
    mut size: libc::c_int,
    mut key: PlLong,
) -> *mut SwtInf {
    let mut n: libc::c_int = 0;
    let mut swt: *mut SwtInf = 0 as *mut SwtInf;
    let mut endt: *mut SwtInf = 0 as *mut SwtInf;
    size += 1;
    size;
    n = (key % size as libc::c_long) as libc::c_int;
    swt = t.offset(n as isize);
    endt = t.offset(size as isize);
    while ((*swt).codep).is_some() && (*swt).key != key {
        swt = swt.offset(1);
        swt;
        if swt == endt {
            swt = t;
        }
    }
    return swt;
}
pub unsafe extern "C" fn Pl_Get_Atom_Tagged(
    mut w: WamWord,
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        if (word as *mut WamWord)
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || word as *mut WamWord
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                && (word as *mut WamWord) < B
        {
            let fresh0 = TR;
            TR = TR.offset(1);
            *fresh0 = (word as *mut WamWord as PlULong
                | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *(word as *mut WamWord) = w;
        return 1 as libc::c_int;
    }
    return (word == w) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Get_Atom(
    mut atom: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Atom_Tagged(
        (((atom as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord,
        start_word,
    );
}
pub unsafe extern "C" fn Pl_Get_Integer_Tagged(
    mut w: WamWord,
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        if (word as *mut WamWord)
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || word as *mut WamWord
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                && (word as *mut WamWord) < B
        {
            let fresh1 = TR;
            TR = TR.offset(1);
            *fresh1 = (word as *mut WamWord as PlULong
                | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *(word as *mut WamWord) = w;
        return 1 as libc::c_int;
    }
    if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
        return ::std::mem::transmute::<
            _,
            fn(_, _) -> Bool,
        >(
            (Some(pl_fd_unify_with_integer.unwrap())).unwrap(),
        )(
            (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord,
            w << 0 as libc::c_int >> 3 as libc::c_int,
        );
    }
    return (word == w) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Get_Integer(mut n: PlLong, mut start_word: WamWord) -> Bool {
    return Pl_Get_Integer_Tagged(
        ((n as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
        start_word,
    );
}
pub unsafe extern "C" fn Pl_Get_Float(
    mut n: libc::c_double,
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        if (word as *mut WamWord)
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || word as *mut WamWord
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                && (word as *mut WamWord) < B
        {
            let fresh2 = TR;
            TR = TR.offset(1);
            *fresh2 = (word as *mut WamWord as PlULong
                | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *(word
            as *mut WamWord) = (H as PlLong as libc::c_ulong)
            .wrapping_add(0x4 as libc::c_int as PlULong) as WamWord;
        Pl_Global_Push_Float(n);
        return 1 as libc::c_int;
    }
    return (tag_mask as libc::c_ulong == 0x4 as libc::c_int as PlULong
        && Pl_Obtain_Float(
            (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        ) == n) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Get_Nil(mut start_word: WamWord) -> Bool {
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
        if (word as *mut WamWord)
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || word as *mut WamWord
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                && (word as *mut WamWord) < B
        {
            let fresh3 = TR;
            TR = TR.offset(1);
            *fresh3 = (word as *mut WamWord as PlULong
                | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *(word
            as *mut WamWord) = (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
            as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
        return 1 as libc::c_int;
    }
    return (word as libc::c_ulong
        == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Get_List(mut start_word: WamWord) -> Bool {
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
        if (word as *mut WamWord)
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || word as *mut WamWord
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                && (word as *mut WamWord) < B
        {
            let fresh4 = TR;
            TR = TR.offset(1);
            *fresh4 = (word as *mut WamWord as PlULong
                | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *(word
            as *mut WamWord) = (H as PlLong as libc::c_ulong)
            .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
        let ref mut fresh5 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        *fresh5 = 0 as WamWordP;
        return 1 as libc::c_int;
    }
    if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        let ref mut fresh6 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        *fresh6 = ((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset(0 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Get_Structure_Tagged(
    mut w: WamWord,
    mut start_word: WamWord,
) -> Bool {
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
        let mut cur_H: *mut WamWord = H;
        *cur_H = w;
        H = H.offset(1);
        H;
        let ref mut fresh7 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        *fresh7 = 0 as WamWordP;
        if (word as *mut WamWord)
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || word as *mut WamWord
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                && (word as *mut WamWord) < B
        {
            let fresh8 = TR;
            TR = TR.offset(1);
            *fresh8 = (word as *mut WamWord as PlULong
                | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *(word
            as *mut WamWord) = (cur_H as PlLong as libc::c_ulong)
            .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
        return 1 as libc::c_int;
    }
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if *adr.offset(0 as libc::c_int as isize) != w {
            return 0 as libc::c_int;
        }
        let ref mut fresh9 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        *fresh9 = adr.offset(1 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Get_Structure(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut start_word: WamWord,
) -> Bool {
    return Pl_Get_Structure_Tagged(
        ((arity as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
            | func as libc::c_ulong) as WamWord,
        start_word,
    );
}
pub unsafe extern "C" fn Pl_Put_X_Variable() -> WamWord {
    let mut res_word: WamWord = 0;
    let mut cur_H: *mut WamWord = H;
    res_word = (cur_H as PlLong as libc::c_ulong)
        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    *cur_H = res_word;
    H = H.offset(1);
    H;
    return res_word;
}
pub unsafe extern "C" fn Pl_Put_Y_Variable(mut y_adr: *mut WamWord) -> WamWord {
    *y_adr = (y_adr as PlLong as libc::c_ulong).wrapping_add(0 as libc::c_int as PlULong)
        as WamWord;
    return *y_adr;
}
pub unsafe extern "C" fn Pl_Put_Unsafe_Value(mut start_word: WamWord) -> WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut res_word: WamWord = 0;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        && {
            adr = word as *mut WamWord;
            adr
                >= *(&mut *(*(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                    .offset(-(3 as libc::c_int) as isize) as *mut WamWord
                    as *mut *mut WamWord)
        }
    {
        let mut cur_H: *mut WamWord = H;
        res_word = (cur_H as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        *cur_H = res_word;
        H = H.offset(1);
        H;
        if adr
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || adr
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize) && adr < B
        {
            let fresh10 = TR;
            TR = TR.offset(1);
            *fresh10 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *adr = res_word;
        return res_word;
    }
    if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
        word = ((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    return word;
}
pub unsafe extern "C" fn Pl_Put_Atom_Tagged(mut w: WamWord) -> WamWord {
    return w;
}
pub unsafe extern "C" fn Pl_Put_Atom(mut atom: libc::c_int) -> WamWord {
    return (((atom as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Put_Integer_Tagged(mut w: WamWord) -> WamWord {
    return w;
}
pub unsafe extern "C" fn Pl_Put_Integer(mut n: PlLong) -> WamWord {
    return ((n as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Put_Float(mut n: libc::c_double) -> WamWord {
    let mut res_word: WamWord = 0;
    res_word = (H as PlLong as libc::c_ulong).wrapping_add(0x4 as libc::c_int as PlULong)
        as WamWord;
    Pl_Global_Push_Float(n);
    return res_word;
}
pub unsafe extern "C" fn Pl_Put_Nil() -> WamWord {
    return (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Put_List() -> WamWord {
    let ref mut fresh11 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
    *fresh11 = 0 as WamWordP;
    return (H as PlLong as libc::c_ulong).wrapping_add(0x1 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Put_Structure_Tagged(mut w: WamWord) -> WamWord {
    let mut cur_H: *mut WamWord = H;
    *cur_H = w;
    H = H.offset(1);
    H;
    let ref mut fresh12 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
    *fresh12 = 0 as WamWordP;
    return (cur_H as PlLong as libc::c_ulong).wrapping_add(0x2 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Put_Structure(
    mut func: libc::c_int,
    mut arity: libc::c_int,
) -> WamWord {
    return Pl_Put_Structure_Tagged(
        ((arity as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
            | func as libc::c_ulong) as WamWord,
    );
}
pub unsafe extern "C" fn Pl_Unify_Variable() -> WamWord {
    let mut tag_mask: WamWord = 0;
    let mut word: WamWord = 0;
    let mut res_word: WamWord = 0;
    let mut cur_H: *mut WamWord = 0 as *mut WamWord;
    if !(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize))
        .is_null()
    {
        let ref mut fresh13 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        let fresh14 = *fresh13;
        *fresh13 = (*fresh13).offset(1);
        word = *fresh14;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
            word = ((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        }
        return word;
    }
    cur_H = H;
    res_word = (cur_H as PlLong as libc::c_ulong)
        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    *cur_H = res_word;
    H = H.offset(1);
    H;
    return res_word;
}
pub unsafe extern "C" fn Pl_Unify_Void(mut n: libc::c_int) {
    let mut cur_H: *mut WamWord = 0 as *mut WamWord;
    if !(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize))
        .is_null()
    {
        let ref mut fresh15 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        *fresh15 = (*fresh15).offset(n as isize);
        return;
    }
    cur_H = H;
    H = H.offset(n as isize);
    loop {
        *cur_H = (cur_H as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        cur_H = cur_H.offset(1);
        cur_H;
        n -= 1;
        if !(n > 0 as libc::c_int) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Unify_Value(mut start_word: WamWord) -> Bool {
    if !(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize))
        .is_null()
    {
        let ref mut fresh16 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        let fresh17 = *fresh16;
        *fresh16 = (*fresh16).offset(1);
        return Pl_Unify(start_word, *fresh17);
    }
    let fresh18 = H;
    H = H.offset(1);
    *fresh18 = start_word;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Unify_Local_Value(mut start_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    if !(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize))
        .is_null()
    {
        let ref mut fresh19 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        let fresh20 = *fresh19;
        *fresh19 = (*fresh19).offset(1);
        return Pl_Unify(start_word, *fresh20);
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        && {
            adr = word as *mut WamWord;
            adr
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
        }
    {
        let mut cur_H: *mut WamWord = H;
        word = (cur_H as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        *cur_H = word;
        H = H.offset(1);
        H;
        if adr
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || adr
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize) && adr < B
        {
            let fresh21 = TR;
            TR = TR.offset(1);
            *fresh21 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *adr = word;
    } else {
        if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
            word = ((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        }
        let fresh22 = H;
        H = H.offset(1);
        *fresh22 = word;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Unify_Atom_Tagged(mut w: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    if !(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize))
        .is_null()
    {
        let mut deref_last_word: WamWord = 0;
        word = **(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
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
        let ref mut fresh23 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        *fresh23 = (*fresh23).offset(1);
        *fresh23;
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            if (word as *mut WamWord)
                < *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                || word as *mut WamWord
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                    && (word as *mut WamWord) < B
            {
                let fresh24 = TR;
                TR = TR.offset(1);
                *fresh24 = (word as *mut WamWord as PlULong
                    | 0 as libc::c_int as libc::c_ulong) as WamWord;
            }
            *(word as *mut WamWord) = w;
            return 1 as libc::c_int;
        }
        return (word == w) as libc::c_int;
    }
    let fresh25 = H;
    H = H.offset(1);
    *fresh25 = w;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Unify_Atom(mut atom: libc::c_int) -> Bool {
    return Pl_Unify_Atom_Tagged(
        (((atom as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord,
    );
}
pub unsafe extern "C" fn Pl_Unify_Integer_Tagged(mut w: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    if !(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize))
        .is_null()
    {
        let mut deref_last_word: WamWord = 0;
        word = **(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
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
        let ref mut fresh26 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        *fresh26 = (*fresh26).offset(1);
        *fresh26;
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            if (word as *mut WamWord)
                < *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                || word as *mut WamWord
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                    && (word as *mut WamWord) < B
            {
                let fresh27 = TR;
                TR = TR.offset(1);
                *fresh27 = (word as *mut WamWord as PlULong
                    | 0 as libc::c_int as libc::c_ulong) as WamWord;
            }
            *(word as *mut WamWord) = w;
            return 1 as libc::c_int;
        }
        if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
            return ::std::mem::transmute::<
                _,
                fn(_, _) -> Bool,
            >(
                (Some(pl_fd_unify_with_integer.unwrap())).unwrap(),
            )(
                (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord,
                w << 0 as libc::c_int >> 3 as libc::c_int,
            );
        }
        return (word == w) as libc::c_int;
    }
    let fresh28 = H;
    H = H.offset(1);
    *fresh28 = w;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Unify_Integer(mut n: PlLong) -> Bool {
    return Pl_Unify_Integer_Tagged(
        ((n as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord,
    );
}
pub unsafe extern "C" fn Pl_Unify_Nil() -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    if !(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize))
        .is_null()
    {
        let mut deref_last_word: WamWord = 0;
        word = **(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
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
        let ref mut fresh29 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 4 as libc::c_int) as isize);
        *fresh29 = (*fresh29).offset(1);
        *fresh29;
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            if (word as *mut WamWord)
                < *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                || word as *mut WamWord
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                    && (word as *mut WamWord) < B
            {
                let fresh30 = TR;
                TR = TR.offset(1);
                *fresh30 = (word as *mut WamWord as PlULong
                    | 0 as libc::c_int as libc::c_ulong) as WamWord;
            }
            *(word
                as *mut WamWord) = (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
            return 1 as libc::c_int;
        }
        return (word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)) as libc::c_int;
    }
    let fresh31 = H;
    H = H.offset(1);
    *fresh31 = (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Unify_List() -> Bool {
    let mut cur_H: *mut WamWord = 0 as *mut WamWord;
    if !(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize))
        .is_null()
    {
        return Pl_Get_List(
            **(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 4 as libc::c_int) as isize),
        );
    }
    cur_H = H;
    *cur_H = (cur_H.offset(1 as libc::c_int as isize) as PlLong as libc::c_ulong)
        .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
    H = H.offset(1);
    H;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Unify_Structure_Tagged(mut w: WamWord) -> Bool {
    let mut cur_H: *mut WamWord = 0 as *mut WamWord;
    if !(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 4 as libc::c_int) as isize))
        .is_null()
    {
        return Pl_Get_Structure_Tagged(
            w,
            **(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 4 as libc::c_int) as isize),
        );
    }
    cur_H = H;
    *cur_H = (cur_H.offset(1 as libc::c_int as isize) as PlLong as libc::c_ulong)
        .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
    *cur_H.offset(1 as libc::c_int as isize) = w;
    H = H.offset(2 as libc::c_int as isize);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Unify_Structure(
    mut func: libc::c_int,
    mut arity: libc::c_int,
) -> Bool {
    return Pl_Unify_Structure_Tagged(
        ((arity as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
            | func as libc::c_ulong) as WamWord,
    );
}
pub unsafe extern "C" fn Pl_Globalize_If_In_Local(mut start_word: WamWord) -> WamWord {
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
        adr = word as *mut WamWord;
        if adr
            >= *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
        {
            let mut cur_H: *mut WamWord = H;
            start_word = (cur_H as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
            *cur_H = start_word;
            H = H.offset(1);
            H;
            if adr
                < *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                || adr
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                    && adr < B
            {
                let fresh32 = TR;
                TR = TR.offset(1);
                *fresh32 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                    as WamWord;
            }
            *adr = start_word;
        }
    }
    return start_word;
}
pub unsafe extern "C" fn Pl_Allocate(mut n: libc::c_int) {
    let mut old_E: *mut WamWord = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    let mut cur_E: *mut WamWord = (if B
        >= *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    {
        B
    } else {
        *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    })
        .offset(3 as libc::c_int as isize)
        .offset(n as isize);
    let ref mut fresh33 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh33 = cur_E;
    let ref mut fresh34 = *(&mut *cur_E.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut WamCont);
    *fresh34 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *(&mut *cur_E.offset(-(2 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank
        .offset((256 as libc::c_int + 6 as libc::c_int) as isize);
    let ref mut fresh35 = *(&mut *cur_E.offset(-(3 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh35 = old_E;
}
pub unsafe extern "C" fn Pl_Deallocate() {
    let mut cur_E: *mut WamWord = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    let ref mut fresh36 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh36 = *(&mut *cur_E.offset(-(1 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_E.offset(-(2 as libc::c_int) as isize) as *mut WamWord);
    let ref mut fresh37 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh37 = *(&mut *cur_E.offset(-(3 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
}
pub unsafe extern "C" fn Pl_Switch_On_Term(
    mut c_var: CodePtr,
    mut c_atm: CodePtr,
    mut c_int: CodePtr,
    mut c_lst: CodePtr,
    mut c_stc: CodePtr,
) -> CodePtr {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut codep: CodePtr = None;
    let mut deref_last_word: WamWord = 0;
    word = *pl_reg_bank.offset(0 as libc::c_int as isize);
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
    *pl_reg_bank.offset(0 as libc::c_int as isize) = word;
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        codep = c_int;
    } else if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        codep = c_atm;
    } else if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        codep = c_lst;
    } else if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        codep = c_stc;
    } else {
        codep = c_var;
    }
    return if codep.is_some() {
        codep
    } else {
        *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord as *mut CodePtr)
    };
}
pub unsafe extern "C" fn Pl_Switch_On_Term_Var_Atm(
    mut c_var: CodePtr,
    mut c_atm: CodePtr,
) -> CodePtr {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = *pl_reg_bank.offset(0 as libc::c_int as isize);
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
    *pl_reg_bank.offset(0 as libc::c_int as isize) = word;
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        return c_atm;
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        return c_var;
    }
    return *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord
        as *mut CodePtr);
}
pub unsafe extern "C" fn Pl_Switch_On_Term_Var_Stc(
    mut c_var: CodePtr,
    mut c_stc: CodePtr,
) -> CodePtr {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = *pl_reg_bank.offset(0 as libc::c_int as isize);
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
    *pl_reg_bank.offset(0 as libc::c_int as isize) = word;
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        return c_stc;
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        return c_var;
    }
    return *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord
        as *mut CodePtr);
}
pub unsafe extern "C" fn Pl_Switch_On_Term_Var_Atm_Lst(
    mut c_var: CodePtr,
    mut c_atm: CodePtr,
    mut c_lst: CodePtr,
) -> CodePtr {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = *pl_reg_bank.offset(0 as libc::c_int as isize);
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
    *pl_reg_bank.offset(0 as libc::c_int as isize) = word;
    if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        return c_lst;
    }
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        return c_atm;
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        return c_var;
    }
    return *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord
        as *mut CodePtr);
}
pub unsafe extern "C" fn Pl_Switch_On_Term_Var_Atm_Stc(
    mut c_var: CodePtr,
    mut c_atm: CodePtr,
    mut c_stc: CodePtr,
) -> CodePtr {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = *pl_reg_bank.offset(0 as libc::c_int as isize);
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
    *pl_reg_bank.offset(0 as libc::c_int as isize) = word;
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        return c_stc;
    }
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        return c_atm;
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        return c_var;
    }
    return *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord
        as *mut CodePtr);
}
pub unsafe extern "C" fn Pl_Switch_On_Atom(
    mut t: SwtTbl,
    mut size: libc::c_int,
) -> CodePtr {
    let mut swt: *mut SwtInf = 0 as *mut SwtInf;
    swt = Locate_Swt_Element(
        t,
        size,
        (*pl_reg_bank.offset(0 as libc::c_int as isize) as PlULong >> 3 as libc::c_int)
            as PlLong,
    );
    return if ((*swt).codep).is_some() {
        (*swt).codep
    } else {
        *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord as *mut CodePtr)
    };
}
pub unsafe extern "C" fn Pl_Switch_On_Integer() -> PlLong {
    return *pl_reg_bank.offset(0 as libc::c_int as isize) << 0 as libc::c_int
        >> 3 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Switch_On_Structure(
    mut t: SwtTbl,
    mut size: libc::c_int,
) -> CodePtr {
    let mut swt: *mut SwtInf = 0 as *mut SwtInf;
    swt = Locate_Swt_Element(
        t,
        size,
        *((*pl_reg_bank.offset(0 as libc::c_int as isize) as libc::c_ulong
            & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord)
            .offset(0 as libc::c_int as isize),
    );
    return if ((*swt).codep).is_some() {
        (*swt).codep
    } else {
        *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord as *mut CodePtr)
    };
}
pub unsafe extern "C" fn Pl_Get_Current_Choice() -> WamWord {
    return ((B
        .offset_from(
            *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 7 as libc::c_int) as isize),
        ) as libc::c_long as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Cut(mut b_word: WamWord) {
    B = (*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
        .offset((b_word << 0 as libc::c_int >> 3 as libc::c_int) as isize);
    let ref mut fresh38 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh38 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
}
pub unsafe extern "C" fn Pl_Soft_Cut(mut b_word: WamWord) {
    let mut kill_B: *mut WamWord = (*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
        .offset((b_word << 0 as libc::c_int >> 3 as libc::c_int) as isize);
    let mut cur_B: *mut WamWord = B;
    let mut prev_B: *mut WamWord = 0 as *mut WamWord;
    if cur_B == kill_B {
        B = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh39 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh39 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        return;
    }
    loop {
        prev_B = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        if prev_B == kill_B {
            let ref mut fresh40 = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize)
                as *mut WamWord as *mut *mut WamWord);
            *fresh40 = *(&mut *kill_B.offset(-(5 as libc::c_int) as isize)
                as *mut WamWord as *mut *mut WamWord);
            break;
        } else {
            if cur_B < kill_B {
                break;
            }
            cur_B = prev_B;
        }
    };
}
pub unsafe extern "C" fn Pl_Global_Push_Float(mut n: libc::c_double) {
    let mut di: DblInt = DblInt { d: 0. };
    di.d = n;
    let fresh41 = H;
    H = H.offset(1);
    *fresh41 = di.i[0 as libc::c_int as usize];
}
pub unsafe extern "C" fn Pl_Obtain_Float(mut adr: *mut WamWord) -> libc::c_double {
    let mut di: DblInt = DblInt { d: 0. };
    di.i[0 as libc::c_int as usize] = *adr.offset(0 as libc::c_int as isize);
    return di.d;
}
pub unsafe extern "C" fn Pl_Create_Choice_Point(
    mut codep_alt: CodePtr,
    mut arity: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut old_B: *mut WamWord = B;
    let mut cur_B: *mut WamWord = (if B
        >= *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    {
        B
    } else {
        *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    })
        .offset(8 as libc::c_int as isize)
        .offset(arity as isize);
    B = cur_B;
    let ref mut fresh42 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh42 = codep_alt;
    let ref mut fresh43 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize)
        as *mut WamWord as *mut WamCont);
    *fresh43 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *(&mut *cur_B.offset(-(3 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank
        .offset((256 as libc::c_int + 6 as libc::c_int) as isize);
    let ref mut fresh44 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh44 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    let ref mut fresh45 = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh45 = old_B;
    let ref mut fresh46 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh46 = H;
    let ref mut fresh47 = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh47 = *fresh46;
    let ref mut fresh48 = *(&mut *cur_B.offset(-(7 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh48 = TR;
    let ref mut fresh49 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh49 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh50 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh50 += 1;
    *fresh50;
    i = 0 as libc::c_int;
    while i < arity {
        *(&mut *cur_B.offset((-(9 as libc::c_int) - i) as isize)
            as *mut WamWord) = *pl_reg_bank.offset(i as isize);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn Pl_Create_Choice_Point0(mut codep_alt: CodePtr) {
    let mut old_B: *mut WamWord = B;
    let mut cur_B: *mut WamWord = (if B
        >= *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    {
        B
    } else {
        *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    })
        .offset(8 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize);
    B = cur_B;
    let ref mut fresh51 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh51 = codep_alt;
    let ref mut fresh52 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize)
        as *mut WamWord as *mut WamCont);
    *fresh52 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *(&mut *cur_B.offset(-(3 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank
        .offset((256 as libc::c_int + 6 as libc::c_int) as isize);
    let ref mut fresh53 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh53 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    let ref mut fresh54 = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh54 = old_B;
    let ref mut fresh55 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh55 = H;
    let ref mut fresh56 = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh56 = *fresh55;
    let ref mut fresh57 = *(&mut *cur_B.offset(-(7 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh57 = TR;
    let ref mut fresh58 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh58 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh59 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh59 += 1;
    *fresh59;
}
pub unsafe extern "C" fn Pl_Create_Choice_Point1(mut codep_alt: CodePtr) {
    let mut old_B: *mut WamWord = B;
    let mut cur_B: *mut WamWord = (if B
        >= *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    {
        B
    } else {
        *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    })
        .offset(8 as libc::c_int as isize)
        .offset(1 as libc::c_int as isize);
    B = cur_B;
    let ref mut fresh60 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh60 = codep_alt;
    let ref mut fresh61 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize)
        as *mut WamWord as *mut WamCont);
    *fresh61 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *(&mut *cur_B.offset(-(3 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank
        .offset((256 as libc::c_int + 6 as libc::c_int) as isize);
    let ref mut fresh62 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh62 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    let ref mut fresh63 = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh63 = old_B;
    let ref mut fresh64 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh64 = H;
    let ref mut fresh65 = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh65 = *fresh64;
    let ref mut fresh66 = *(&mut *cur_B.offset(-(7 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh66 = TR;
    let ref mut fresh67 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh67 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh68 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh68 += 1;
    *fresh68;
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(0 as libc::c_int as isize);
}
pub unsafe extern "C" fn Pl_Create_Choice_Point2(mut codep_alt: CodePtr) {
    let mut old_B: *mut WamWord = B;
    let mut cur_B: *mut WamWord = (if B
        >= *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    {
        B
    } else {
        *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    })
        .offset(8 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize);
    B = cur_B;
    let ref mut fresh69 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh69 = codep_alt;
    let ref mut fresh70 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize)
        as *mut WamWord as *mut WamCont);
    *fresh70 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *(&mut *cur_B.offset(-(3 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank
        .offset((256 as libc::c_int + 6 as libc::c_int) as isize);
    let ref mut fresh71 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh71 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    let ref mut fresh72 = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh72 = old_B;
    let ref mut fresh73 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh73 = H;
    let ref mut fresh74 = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh74 = *fresh73;
    let ref mut fresh75 = *(&mut *cur_B.offset(-(7 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh75 = TR;
    let ref mut fresh76 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh76 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh77 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh77 += 1;
    *fresh77;
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(0 as libc::c_int as isize);
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(1 as libc::c_int as isize);
}
pub unsafe extern "C" fn Pl_Create_Choice_Point3(mut codep_alt: CodePtr) {
    let mut old_B: *mut WamWord = B;
    let mut cur_B: *mut WamWord = (if B
        >= *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    {
        B
    } else {
        *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    })
        .offset(8 as libc::c_int as isize)
        .offset(3 as libc::c_int as isize);
    B = cur_B;
    let ref mut fresh78 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh78 = codep_alt;
    let ref mut fresh79 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize)
        as *mut WamWord as *mut WamCont);
    *fresh79 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *(&mut *cur_B.offset(-(3 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank
        .offset((256 as libc::c_int + 6 as libc::c_int) as isize);
    let ref mut fresh80 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh80 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    let ref mut fresh81 = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh81 = old_B;
    let ref mut fresh82 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh82 = H;
    let ref mut fresh83 = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh83 = *fresh82;
    let ref mut fresh84 = *(&mut *cur_B.offset(-(7 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh84 = TR;
    let ref mut fresh85 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh85 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh86 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh86 += 1;
    *fresh86;
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(0 as libc::c_int as isize);
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(1 as libc::c_int as isize);
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(2 as libc::c_int as isize);
}
pub unsafe extern "C" fn Pl_Create_Choice_Point4(mut codep_alt: CodePtr) {
    let mut old_B: *mut WamWord = B;
    let mut cur_B: *mut WamWord = (if B
        >= *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    {
        B
    } else {
        *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    })
        .offset(8 as libc::c_int as isize)
        .offset(4 as libc::c_int as isize);
    B = cur_B;
    let ref mut fresh87 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh87 = codep_alt;
    let ref mut fresh88 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize)
        as *mut WamWord as *mut WamCont);
    *fresh88 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *(&mut *cur_B.offset(-(3 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank
        .offset((256 as libc::c_int + 6 as libc::c_int) as isize);
    let ref mut fresh89 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh89 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    let ref mut fresh90 = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh90 = old_B;
    let ref mut fresh91 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh91 = H;
    let ref mut fresh92 = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh92 = *fresh91;
    let ref mut fresh93 = *(&mut *cur_B.offset(-(7 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh93 = TR;
    let ref mut fresh94 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize)
        as *mut WamWord as *mut *mut WamWord);
    *fresh94 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh95 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh95 += 1;
    *fresh95;
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(0 as libc::c_int as isize);
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(1 as libc::c_int as isize);
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(2 as libc::c_int as isize);
    *(&mut *cur_B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
        as *mut WamWord) = *pl_reg_bank.offset(3 as libc::c_int as isize);
}
pub unsafe extern "C" fn Pl_Update_Choice_Point(
    mut codep_alt: CodePtr,
    mut arity: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh96 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh96 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh97 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh97 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh98 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh98 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh99 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh99 = codep_alt;
    let ref mut fresh100 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh100 = H;
    i = 0 as libc::c_int;
    while i < arity {
        *pl_reg_bank
            .offset(
                i as isize,
            ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - i) as isize)
            as *mut WamWord);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn Pl_Update_Choice_Point0(mut codep_alt: CodePtr) {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh101 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh101 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh102 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh102 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh103 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh103 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh104 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh104 = codep_alt;
    let ref mut fresh105 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh105 = H;
}
pub unsafe extern "C" fn Pl_Update_Choice_Point1(mut codep_alt: CodePtr) {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh106 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh106 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh107 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh107 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh108 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh108 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh109 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh109 = codep_alt;
    let ref mut fresh110 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh110 = H;
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
}
pub unsafe extern "C" fn Pl_Update_Choice_Point2(mut codep_alt: CodePtr) {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh111 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh111 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh112 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh112 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh113 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh113 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh114 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh114 = codep_alt;
    let ref mut fresh115 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh115 = H;
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            1 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
}
pub unsafe extern "C" fn Pl_Update_Choice_Point3(mut codep_alt: CodePtr) {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh116 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh116 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh117 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh117 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh118 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh118 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh119 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh119 = codep_alt;
    let ref mut fresh120 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh120 = H;
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            1 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            2 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord);
}
pub unsafe extern "C" fn Pl_Update_Choice_Point4(mut codep_alt: CodePtr) {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh121 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh121 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh122 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh122 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh123 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh123 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh124 = *(&mut *cur_B.offset(-(1 as libc::c_int) as isize)
        as *mut WamWord as *mut CodePtr);
    *fresh124 = codep_alt;
    let ref mut fresh125 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh125 = H;
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            1 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            2 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            3 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
        as *mut WamWord);
}
pub unsafe extern "C" fn Pl_Delete_Choice_Point(mut arity: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh126 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh126 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh127 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh127 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh128 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh128 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    B = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh129 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh129 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh130 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh130 -= 1;
    *fresh130;
    i = 0 as libc::c_int;
    while i < arity {
        *pl_reg_bank
            .offset(
                i as isize,
            ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - i) as isize)
            as *mut WamWord);
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn Pl_Delete_Choice_Point0() {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh131 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh131 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh132 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh132 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh133 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh133 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    B = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh134 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh134 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh135 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh135 -= 1;
    *fresh135;
}
pub unsafe extern "C" fn Pl_Delete_Choice_Point1() {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh136 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh136 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh137 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh137 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh138 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh138 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    B = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh139 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh139 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh140 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh140 -= 1;
    *fresh140;
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
}
pub unsafe extern "C" fn Pl_Delete_Choice_Point2() {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh141 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh141 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh142 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh142 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh143 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh143 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    B = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh144 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh144 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh145 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh145 -= 1;
    *fresh145;
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            1 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
}
pub unsafe extern "C" fn Pl_Delete_Choice_Point3() {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh146 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh146 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh147 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh147 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh148 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh148 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    B = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh149 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh149 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh150 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh150 -= 1;
    *fresh150;
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            1 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            2 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord);
}
pub unsafe extern "C" fn Pl_Delete_Choice_Point4() {
    let mut cur_B: *mut WamWord = B;
    Pl_Untrail(
        *(&mut *cur_B.offset(-(7 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord),
    );
    let ref mut fresh151 = *(pl_reg_bank as *mut WamCont)
        .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
    *fresh151 = *(&mut *cur_B.offset(-(2 as libc::c_int) as isize) as *mut WamWord
        as *mut WamCont);
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 6 as libc::c_int) as isize,
        ) = *(&mut *cur_B.offset(-(3 as libc::c_int) as isize) as *mut WamWord);
    H = *(&mut *cur_B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh152 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 2 as libc::c_int) as isize);
    *fresh152 = *(&mut *cur_B.offset(-(4 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh153 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh153 = *(&mut *cur_B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    B = *(&mut *cur_B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh154 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh154 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh155 = *pl_reg_bank
        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fresh155 -= 1;
    *fresh155;
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            1 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            2 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord);
    *pl_reg_bank
        .offset(
            3 as libc::c_int as isize,
        ) = *(&mut *cur_B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
        as *mut WamWord);
}
pub unsafe extern "C" fn Pl_Defeasible_Open() {
    Pl_Create_Choice_Point0(None);
}
pub unsafe extern "C" fn Pl_Defeasible_Undo() {
    Pl_Update_Choice_Point0(None);
}
pub unsafe extern "C" fn Pl_Defeasible_Close(mut success: Bool) {
    if success != 0 {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh156 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh156 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    } else {
        Pl_Delete_Choice_Point0();
    };
}
pub unsafe extern "C" fn Pl_Untrail(mut low_adr: *mut WamWord) {
    let mut word: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut nb: libc::c_int = 0;
    while TR > low_adr {
        TR = TR.offset(-1);
        word = *TR;
        adr = (word as PlULong & !(0x3 as libc::c_int) as libc::c_ulong) as *mut WamWord;
        match word as PlULong & 0x3 as libc::c_int as libc::c_ulong {
            0 => {
                *adr = (adr as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
            }
            1 => {
                TR = TR.offset(-1);
                *adr = *TR;
            }
            2 => {
                TR = TR.offset(-1);
                nb = *TR as libc::c_int;
                TR = TR.offset(-(nb as isize));
                let mut s: *mut PlLong = TR as *mut PlLong;
                let mut d: *mut PlLong = adr as *mut PlLong;
                let mut counter: libc::c_int = nb;
                loop {
                    let fresh157 = s;
                    s = s.offset(1);
                    let fresh158 = d;
                    d = d.offset(1);
                    *fresh158 = *fresh157;
                    counter -= 1;
                    if !(counter != 0) {
                        break;
                    }
                }
            }
            _ => {
                TR = TR.offset(-1);
                adr = *TR as *mut WamWord;
                TR = TR.offset(-1);
                nb = *TR as libc::c_int;
                TR = TR.offset(-(nb as isize));
                ::std::mem::transmute::<
                    _,
                    fn(_, _) -> libc::c_int,
                >(
                    (Some(
                        (::std::mem::transmute::<
                            *mut WamWord,
                            Option::<unsafe extern "C" fn() -> libc::c_int>,
                        >(adr))
                            .unwrap(),
                    ))
                        .unwrap(),
                )(nb, TR);
            }
        }
    }
}
pub unsafe extern "C" fn Pl_Unify_Occurs_Check(
    mut start_u_word: WamWord,
    mut start_v_word: WamWord,
) -> Bool {
    let mut u_word: WamWord = 0;
    let mut u_tag_mask: WamWord = 0;
    let mut v_word: WamWord = 0;
    let mut v_tag_mask: WamWord = 0;
    let mut u_adr: *mut WamWord = 0 as *mut WamWord;
    let mut v_adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    loop {
        let mut deref_last_word: WamWord = 0;
        u_word = start_u_word;
        loop {
            deref_last_word = u_word;
            u_tag_mask = (u_word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if u_tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            u_word = *(u_word as *mut WamWord);
            if !(u_word != deref_last_word) {
                break;
            }
        }
        let mut deref_last_word_0: WamWord = 0;
        v_word = start_v_word;
        loop {
            deref_last_word_0 = v_word;
            v_tag_mask = (v_word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if v_tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            v_word = *(v_word as *mut WamWord);
            if !(v_word != deref_last_word_0) {
                break;
            }
        }
        if u_tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            u_adr = u_word as *mut WamWord;
            if v_tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
                v_adr = v_word as *mut WamWord;
                if u_adr > v_adr {
                    if u_adr
                        < *(pl_reg_bank as *mut WamWordP)
                            .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                        || u_adr
                            >= *(pl_reg_bank as *mut WamWordP)
                                .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                            && u_adr < B
                    {
                        let fresh159 = TR;
                        TR = TR.offset(1);
                        *fresh159 = (u_adr as PlULong
                            | 0 as libc::c_int as libc::c_ulong) as WamWord;
                    }
                    *u_adr = (v_adr as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                } else if v_adr > u_adr {
                    if v_adr
                        < *(pl_reg_bank as *mut WamWordP)
                            .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                        || v_adr
                            >= *(pl_reg_bank as *mut WamWordP)
                                .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                            && v_adr < B
                    {
                        let fresh160 = TR;
                        TR = TR.offset(1);
                        *fresh160 = (v_adr as PlULong
                            | 0 as libc::c_int as libc::c_ulong) as WamWord;
                    }
                    *v_adr = (u_adr as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                }
            } else {
                if !(u_adr
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
                    && Check_If_Var_Occurs(u_adr, v_word) != 0
                {
                    return 0 as libc::c_int;
                }
                if v_tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
                    v_word = ((v_word as libc::c_ulong
                        & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord as PlLong
                        as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                }
                if u_adr
                    < *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                    || u_adr
                        >= *(pl_reg_bank as *mut WamWordP)
                            .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                        && u_adr < B
                {
                    let fresh161 = TR;
                    TR = TR.offset(1);
                    *fresh161 = (u_adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                        as WamWord;
                }
                *u_adr = v_word;
            }
            return 1 as libc::c_int;
        }
        if v_tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            v_adr = v_word as *mut WamWord;
            if !(v_adr
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize))
                && Check_If_Var_Occurs(v_adr, u_word) != 0
            {
                return 0 as libc::c_int;
            }
            if u_tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
                u_word = ((u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
            }
            if v_adr
                < *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                || v_adr
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                    && v_adr < B
            {
                let fresh162 = TR;
                TR = TR.offset(1);
                *fresh162 = (v_adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                    as WamWord;
            }
            *v_adr = u_word;
            return 1 as libc::c_int;
        }
        if u_word == v_word {
            return 1 as libc::c_int;
        }
        if v_tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
            if u_tag_mask != v_tag_mask {
                return 0 as libc::c_int;
            }
            u_adr = (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            v_adr = (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            u_adr = &mut *u_adr.offset(0 as libc::c_int as isize) as *mut WamWord;
            v_adr = &mut *v_adr.offset(0 as libc::c_int as isize) as *mut WamWord;
            let fresh163 = u_adr;
            u_adr = u_adr.offset(1);
            let fresh164 = v_adr;
            v_adr = v_adr.offset(1);
            if Pl_Unify_Occurs_Check(*fresh163, *fresh164) == 0 {
                return 0 as libc::c_int;
            }
            start_u_word = *u_adr;
            start_v_word = *v_adr;
        } else {
            if !(v_tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong) {
                break;
            }
            if u_tag_mask != v_tag_mask {
                return 0 as libc::c_int;
            }
            u_adr = (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            v_adr = (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            if *u_adr.offset(0 as libc::c_int as isize)
                != *v_adr.offset(0 as libc::c_int as isize)
            {
                return 0 as libc::c_int;
            }
            i = (*u_adr.offset(0 as libc::c_int as isize) as PlULong
                >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
            u_adr = &mut *u_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut WamWord;
            v_adr = &mut *v_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut WamWord;
            loop {
                i -= 1;
                if !(i != 0) {
                    break;
                }
                let fresh165 = u_adr;
                u_adr = u_adr.offset(1);
                let fresh166 = v_adr;
                v_adr = v_adr.offset(1);
                if Pl_Unify_Occurs_Check(*fresh165, *fresh166) == 0 {
                    return 0 as libc::c_int;
                }
            }
            start_u_word = *u_adr;
            start_v_word = *v_adr;
        }
    }
    if v_tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        && u_tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        return ::std::mem::transmute::<
            _,
            fn(_, _) -> Bool,
        >(
            (Some(pl_fd_unify_with_integer.unwrap())).unwrap(),
        )(
            (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord,
            v_word << 0 as libc::c_int >> 3 as libc::c_int,
        );
    }
    if v_tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
        v_adr = (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if u_tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
            return ::std::mem::transmute::<
                _,
                fn(_, _) -> Bool,
            >(
                (Some(pl_fd_unify_with_integer.unwrap())).unwrap(),
            )(v_adr, u_word << 0 as libc::c_int >> 3 as libc::c_int);
        }
        if u_tag_mask != v_tag_mask {
            return 0 as libc::c_int;
        }
        return ::std::mem::transmute::<
            _,
            fn(_, _) -> Bool,
        >(
            (Some(pl_fd_unify_with_fd_var.unwrap())).unwrap(),
        )(
            (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord,
            v_adr,
        );
    }
    if v_tag_mask as libc::c_ulong == 0x4 as libc::c_int as PlULong {
        return (u_tag_mask == v_tag_mask
            && Pl_Obtain_Float(
                (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord,
            )
                == Pl_Obtain_Float(
                    (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord,
                )) as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Unify(
    mut start_u_word: WamWord,
    mut start_v_word: WamWord,
) -> Bool {
    let mut u_word: WamWord = 0;
    let mut u_tag_mask: WamWord = 0;
    let mut v_word: WamWord = 0;
    let mut v_tag_mask: WamWord = 0;
    let mut u_adr: *mut WamWord = 0 as *mut WamWord;
    let mut v_adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    loop {
        let mut deref_last_word: WamWord = 0;
        u_word = start_u_word;
        loop {
            deref_last_word = u_word;
            u_tag_mask = (u_word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if u_tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            u_word = *(u_word as *mut WamWord);
            if !(u_word != deref_last_word) {
                break;
            }
        }
        let mut deref_last_word_0: WamWord = 0;
        v_word = start_v_word;
        loop {
            deref_last_word_0 = v_word;
            v_tag_mask = (v_word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if v_tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            v_word = *(v_word as *mut WamWord);
            if !(v_word != deref_last_word_0) {
                break;
            }
        }
        if u_tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            u_adr = u_word as *mut WamWord;
            if v_tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
                v_adr = v_word as *mut WamWord;
                if u_adr > v_adr {
                    if u_adr
                        < *(pl_reg_bank as *mut WamWordP)
                            .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                        || u_adr
                            >= *(pl_reg_bank as *mut WamWordP)
                                .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                            && u_adr < B
                    {
                        let fresh167 = TR;
                        TR = TR.offset(1);
                        *fresh167 = (u_adr as PlULong
                            | 0 as libc::c_int as libc::c_ulong) as WamWord;
                    }
                    *u_adr = (v_adr as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                } else if v_adr > u_adr {
                    if v_adr
                        < *(pl_reg_bank as *mut WamWordP)
                            .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                        || v_adr
                            >= *(pl_reg_bank as *mut WamWordP)
                                .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                            && v_adr < B
                    {
                        let fresh168 = TR;
                        TR = TR.offset(1);
                        *fresh168 = (v_adr as PlULong
                            | 0 as libc::c_int as libc::c_ulong) as WamWord;
                    }
                    *v_adr = (u_adr as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                }
            } else {
                if v_tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
                    v_word = ((v_word as libc::c_ulong
                        & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord as PlLong
                        as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                }
                if u_adr
                    < *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                    || u_adr
                        >= *(pl_reg_bank as *mut WamWordP)
                            .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                        && u_adr < B
                {
                    let fresh169 = TR;
                    TR = TR.offset(1);
                    *fresh169 = (u_adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                        as WamWord;
                }
                *u_adr = v_word;
            }
            return 1 as libc::c_int;
        }
        if v_tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            v_adr = v_word as *mut WamWord;
            if u_tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
                u_word = ((u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
            }
            if v_adr
                < *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                || v_adr
                    >= *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                    && v_adr < B
            {
                let fresh170 = TR;
                TR = TR.offset(1);
                *fresh170 = (v_adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                    as WamWord;
            }
            *v_adr = u_word;
            return 1 as libc::c_int;
        }
        if u_word == v_word {
            return 1 as libc::c_int;
        }
        if v_tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
            if u_tag_mask != v_tag_mask {
                return 0 as libc::c_int;
            }
            u_adr = (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            v_adr = (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            u_adr = &mut *u_adr.offset(0 as libc::c_int as isize) as *mut WamWord;
            v_adr = &mut *v_adr.offset(0 as libc::c_int as isize) as *mut WamWord;
            let fresh171 = u_adr;
            u_adr = u_adr.offset(1);
            let fresh172 = v_adr;
            v_adr = v_adr.offset(1);
            if Pl_Unify(*fresh171, *fresh172) == 0 {
                return 0 as libc::c_int;
            }
            start_u_word = *u_adr;
            start_v_word = *v_adr;
        } else {
            if !(v_tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong) {
                break;
            }
            if u_tag_mask != v_tag_mask {
                return 0 as libc::c_int;
            }
            u_adr = (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            v_adr = (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            if *u_adr.offset(0 as libc::c_int as isize)
                != *v_adr.offset(0 as libc::c_int as isize)
            {
                return 0 as libc::c_int;
            }
            i = (*u_adr.offset(0 as libc::c_int as isize) as PlULong
                >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
            u_adr = &mut *u_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut WamWord;
            v_adr = &mut *v_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut WamWord;
            loop {
                i -= 1;
                if !(i != 0) {
                    break;
                }
                let fresh173 = u_adr;
                u_adr = u_adr.offset(1);
                let fresh174 = v_adr;
                v_adr = v_adr.offset(1);
                if Pl_Unify(*fresh173, *fresh174) == 0 {
                    return 0 as libc::c_int;
                }
            }
            start_u_word = *u_adr;
            start_v_word = *v_adr;
        }
    }
    if v_tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        && u_tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        return ::std::mem::transmute::<
            _,
            fn(_, _) -> Bool,
        >(
            (Some(pl_fd_unify_with_integer.unwrap())).unwrap(),
        )(
            (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord,
            v_word << 0 as libc::c_int >> 3 as libc::c_int,
        );
    }
    if v_tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
        v_adr = (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if u_tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
            return ::std::mem::transmute::<
                _,
                fn(_, _) -> Bool,
            >(
                (Some(pl_fd_unify_with_integer.unwrap())).unwrap(),
            )(v_adr, u_word << 0 as libc::c_int >> 3 as libc::c_int);
        }
        if u_tag_mask != v_tag_mask {
            return 0 as libc::c_int;
        }
        return ::std::mem::transmute::<
            _,
            fn(_, _) -> Bool,
        >(
            (Some(pl_fd_unify_with_fd_var.unwrap())).unwrap(),
        )(
            (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord,
            v_adr,
        );
    }
    if v_tag_mask as libc::c_ulong == 0x4 as libc::c_int as PlULong {
        return (u_tag_mask == v_tag_mask
            && Pl_Obtain_Float(
                (u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord,
            )
                == Pl_Obtain_Float(
                    (v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord,
                )) as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn Check_If_Var_Occurs(
    mut var_adr: *mut WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = term_word;
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
            return (word as *mut WamWord == var_adr) as libc::c_int;
        }
        if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            adr = &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
            let fresh175 = adr;
            adr = adr.offset(1);
            if Check_If_Var_Occurs(var_adr, *fresh175) != 0 {
                return 1 as libc::c_int;
            }
            term_word = *adr;
        } else {
            if !(tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong) {
                break;
            }
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            i = (*adr.offset(0 as libc::c_int as isize) as PlULong
                >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
            adr = &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut WamWord;
            loop {
                i -= 1;
                if !(i != 0) {
                    break;
                }
                let fresh176 = adr;
                adr = adr.offset(1);
                if Check_If_Var_Occurs(var_adr, *fresh176) != 0 {
                    return 1 as libc::c_int;
                }
            }
            term_word = *adr;
        }
    }
    return 0 as libc::c_int;
}
