use ::libc;
extern "C" {
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Rd_Proper_List_Check(start_word: WamWord, arg: *mut WamWord) -> libc::c_int;
    fn Pl_Rd_Proper_List_Check2(
        start_word: WamWord,
        arg: *mut WamWord,
        elt_fct: Option::<unsafe extern "C" fn(WamWord) -> WamWord>,
    ) -> libc::c_int;
    fn Pl_Check_For_Un_List(start_word: WamWord);
    fn Pl_Check_For_Un_List2(
        start_word: WamWord,
        elt_fct: Option::<unsafe extern "C" fn(WamWord) -> ()>,
    );
    fn Pl_Un_Atom(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Mk_Proper_List(n: libc::c_int, arg: *mut WamWord) -> WamWord;
    fn Pl_Term_Compare(start_u_word: WamWord, start_v_word: WamWord) -> PlLong;
    static mut pl_type_pair: libc::c_int;
    fn Pl_Err_Instantiation();
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    static mut pl_sys_var: [PlLong; 0];
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
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
static mut minus_2: WamWord = 0;
unsafe extern "C" fn Chk_Pair(mut start_word: WamWord) {
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
unsafe extern "C" fn Get_Pair(mut start_word: WamWord) -> WamWord {
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
    if tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong
        || *((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord)
            .offset(0 as libc::c_int as isize) != minus_2
    {
        Pl_Err_Type(pl_type_pair, word);
    }
    return word;
}
pub unsafe extern "C" fn Pl_Sort_List_2(
    mut list1_word: WamWord,
    mut list2_word: WamWord,
) -> Bool {
    let mut arg: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0;
    let mut sort_type: libc::c_int = 0;
    sort_type = *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
        as libc::c_int;
    arg = H;
    if sort_type != 2 as libc::c_int {
        Pl_Check_For_Un_List(list2_word);
        n = Pl_Rd_Proper_List_Check(list1_word, arg);
    } else {
        Pl_Check_For_Un_List2(
            list2_word,
            Some(Chk_Pair as unsafe extern "C" fn(WamWord) -> ()),
        );
        n = Pl_Rd_Proper_List_Check2(
            list1_word,
            arg,
            Some(Get_Pair as unsafe extern "C" fn(WamWord) -> WamWord),
        );
    }
    if n == 0 as libc::c_int {
        return Pl_Un_Atom(256 as libc::c_int, list2_word);
    }
    if n == 1 as libc::c_int {
        return Pl_Unify(list1_word, list2_word);
    }
    n = Merge_Sort(
        arg,
        arg.offset(n as isize),
        n,
        sort_type,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(WamWord, WamWord) -> PlLong>,
            Option::<unsafe extern "C" fn() -> PlLong>,
        >(
            if sort_type != 2 as libc::c_int {
                Some(Pl_Term_Compare as unsafe extern "C" fn(WamWord, WamWord) -> PlLong)
            } else {
                Some(Keysort_Cmp as unsafe extern "C" fn(WamWord, WamWord) -> PlLong)
            },
        ),
    );
    return Pl_Unify(Pl_Mk_Proper_List(n, arg), list2_word);
}
pub unsafe extern "C" fn Pl_Sort_List_1(mut list_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut arg: *mut WamWord = 0 as *mut WamWord;
    let mut prev: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0;
    let mut sort_type: libc::c_int = 0;
    sort_type = *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
        as libc::c_int;
    arg = H;
    if sort_type != 2 as libc::c_int {
        n = Pl_Rd_Proper_List_Check(list_word, arg);
    } else {
        n = Pl_Rd_Proper_List_Check2(
            list_word,
            arg,
            Some(Get_Pair as unsafe extern "C" fn(WamWord) -> WamWord),
        );
    }
    if n <= 1 as libc::c_int {
        return;
    }
    n = Merge_Sort(
        arg,
        arg.offset(n as isize),
        n,
        sort_type,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(WamWord, WamWord) -> PlLong>,
            Option::<unsafe extern "C" fn() -> PlLong>,
        >(
            if sort_type != 2 as libc::c_int {
                Some(Pl_Term_Compare as unsafe extern "C" fn(WamWord, WamWord) -> PlLong)
            } else {
                Some(Keysort_Cmp as unsafe extern "C" fn(WamWord, WamWord) -> PlLong)
            },
        ),
    );
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
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let fresh0 = arg;
        arg = arg.offset(1);
        *adr.offset(0 as libc::c_int as isize) = *fresh0;
        prev = &mut *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize)
            as *mut WamWord;
        list_word = *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
        n -= 1;
        if !(n != 0) {
            break;
        }
    }
    *prev = (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
}
unsafe extern "C" fn Keysort_Cmp(mut u_word: WamWord, mut v_word: WamWord) -> PlLong {
    u_word = *((u_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord)
        .offset((1 as libc::c_int + 0 as libc::c_int) as isize);
    v_word = *((v_word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord)
        .offset((1 as libc::c_int + 0 as libc::c_int) as isize);
    return Pl_Term_Compare(u_word, v_word);
}
unsafe extern "C" fn Merge_Sort(
    mut base: *mut WamWord,
    mut aux: *mut WamWord,
    mut n: libc::c_int,
    mut keep_dup: Bool,
    mut cmp: Option::<unsafe extern "C" fn() -> PlLong>,
) -> libc::c_int {
    let mut l1: *mut WamWord = 0 as *mut WamWord;
    let mut l2: *mut WamWord = 0 as *mut WamWord;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    if n <= 1 as libc::c_int {
        return n;
    }
    n1 = n / 2 as libc::c_int;
    n2 = n - n1;
    l1 = base;
    l2 = base.offset(n1 as isize);
    n1 = Merge_Sort(l1, aux, n1, keep_dup, cmp);
    n2 = Merge_Sort(l2, aux, n2, keep_dup, cmp);
    n = n1 + n2;
    p = aux;
    while n1 > 0 as libc::c_int && n2 > 0 as libc::c_int {
        if ::std::mem::transmute::<
            _,
            fn(_, _) -> PlLong,
        >((Some(cmp.unwrap())).unwrap())(*l1, *l2) <= 0 as libc::c_int as libc::c_long
        {
            let fresh1 = l1;
            l1 = l1.offset(1);
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = *fresh1;
            n1 -= 1;
            n1;
        } else {
            let fresh3 = l2;
            l2 = l2.offset(1);
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = *fresh3;
            n2 -= 1;
            n2;
        }
    }
    loop {
        let fresh5 = n1;
        n1 = n1 - 1;
        if !(fresh5 > 0 as libc::c_int) {
            break;
        }
        let fresh6 = l1;
        l1 = l1.offset(1);
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = *fresh6;
    }
    if keep_dup != 0 {
        n1 = n - n2;
        p = aux;
        loop {
            let fresh8 = n1;
            n1 = n1 - 1;
            if !(fresh8 > 0 as libc::c_int) {
                break;
            }
            let fresh9 = p;
            p = p.offset(1);
            let fresh10 = base;
            base = base.offset(1);
            *fresh10 = *fresh9;
        }
        return n;
    }
    loop {
        let fresh11 = n2;
        n2 = n2 - 1;
        if !(fresh11 > 0 as libc::c_int) {
            break;
        }
        let fresh12 = l2;
        l2 = l2.offset(1);
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = *fresh12;
    }
    p = aux;
    let fresh14 = p;
    p = p.offset(1);
    *base = *fresh14;
    n1 = n - 1 as libc::c_int;
    loop {
        let fresh15 = n1;
        n1 = n1 - 1;
        if !(fresh15 > 0 as libc::c_int) {
            break;
        }
        if ::std::mem::transmute::<_, fn(_, _) -> PlLong>(cmp.unwrap())(*base, *p)
            < 0 as libc::c_int as libc::c_long
        {
            let fresh16 = p;
            p = p.offset(1);
            base = base.offset(1);
            *base = *fresh16;
        } else {
            n -= 1;
            n;
            p = p.offset(1);
            p;
        }
    }
    return n;
}
