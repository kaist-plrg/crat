use ::libc;
extern "C" {
    static mut pl_type_list: libc::c_int;
    static mut pl_type_fd_variable: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Instantiation();
    fn Pl_Vector_Empty(vec: Vector);
    fn Pl_Range_Test_Value(range: *mut Range, n: libc::c_int) -> Bool;
    fn Pl_Range_Test_Null_Inter(range: *mut Range, range1: *mut Range) -> Bool;
    fn Pl_Range_Becomes_Sparse(range: *mut Range);
    fn Pl_Range_From_Vector(range: *mut Range);
    fn Pl_Range_Union(range: *mut Range, range1: *mut Range);
    static mut pl_vec_size: WamWord;
    fn Pl_Fd_Tell_Value(fdv_adr: *mut WamWord, n: libc::c_int) -> Bool;
    fn Pl_Fd_Tell_Not_Value(fdv_adr: *mut WamWord, n: libc::c_int) -> Bool;
    fn Pl_Fd_Tell_Int_Range(fdv_adr: *mut WamWord, range: *mut Range) -> Bool;
    fn Pl_Fd_Tell_Range_Range(fdv_adr: *mut WamWord, range: *mut Range) -> Bool;
    fn pl_x_neq_c(x: WamWord, c: WamWord) -> Bool;
    fn pl_x_neq_y(x: WamWord, y: WamWord) -> Bool;
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
pub unsafe extern "C" fn Pl_Fd_All_Different_1(
    mut list_word: WamWord,
    mut save_list_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = list_word;
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
        return 1 as libc::c_int;
    }
    if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_list, save_list_word);
    }
    lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord;
    let mut deref_last_word_0: WamWord = 0;
    word = *lst_adr.offset(0 as libc::c_int as isize);
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    return (Fd_All_Different_Rec(
        *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize),
        tag_mask,
        word,
        save_list_word,
    ) != 0
        && Pl_Fd_All_Different_1(
            *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize),
            save_list_word,
        ) != 0) as libc::c_int;
}
unsafe extern "C" fn Fd_All_Different_Rec(
    mut list_word: WamWord,
    mut x_tag: PlLong,
    mut x_word: WamWord,
    mut save_list_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut ret: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = list_word;
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
        return 1 as libc::c_int;
    }
    if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_list, save_list_word);
    }
    lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord;
    let mut deref_last_word_0: WamWord = 0;
    word = *lst_adr.offset(0 as libc::c_int as isize);
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
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    if x_tag as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        ret = if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
            (x_word != word) as libc::c_int
        } else {
            pl_x_neq_c(word, x_word)
        };
    } else {
        ret = if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
            pl_x_neq_c(x_word, word)
        } else {
            pl_x_neq_y(x_word, word)
        };
    }
    return (ret != 0
        && Fd_All_Different_Rec(
            *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize),
            x_tag,
            x_word,
            save_list_word,
        ) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Element_I(mut i: *mut Range, mut l: *mut WamWord) {
    let mut n: libc::c_int = *l as libc::c_int;
    (*i).extra_cstr = 0 as libc::c_int;
    (*i).min = 1 as libc::c_int;
    (*i).max = n;
    (*i).vec = 0 as Vector;
    Pl_Range_Becomes_Sparse(i);
}
pub unsafe extern "C" fn Pl_Fd_Element_I_To_V(
    mut v: *mut Range,
    mut i: *mut Range,
    mut l: *mut WamWord,
) {
    let mut val: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*v)
        .vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh0 = (*fresh0).offset(pl_vec_size as isize);
    Pl_Vector_Empty((*v).vec);
    if (*i).min == (*i).max || ((*i).vec).is_null() {
        j = (*i).min;
        while j <= (*i).max {
            val = *l.offset(j as isize) as libc::c_int;
            let ref mut fresh1 = *((*v).vec)
                .offset((val as VecWord >> 6 as libc::c_int) as isize);
            *fresh1
                |= (1 as libc::c_int as VecWord)
                    << (val as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            j += 1;
            j;
        }
    } else {
        let mut enum_end: Vector = ((*i).vec).offset(pl_vec_size as isize);
        let mut enum_i: Vector = (*i).vec;
        let mut enum_j: libc::c_int = 0;
        let mut enum_word: VecWord = 0;
        j = 0 as libc::c_int;
        loop {
            enum_word = *enum_i;
            enum_j = 0 as libc::c_int;
            loop {
                let fresh2 = enum_j;
                enum_j = enum_j + 1;
                if !(fresh2 < 8 as libc::c_int * 8 as libc::c_int) {
                    break;
                }
                if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                    val = *l.offset(j as isize) as libc::c_int;
                    let ref mut fresh3 = *((*v).vec)
                        .offset((val as VecWord >> 6 as libc::c_int) as isize);
                    *fresh3
                        |= (1 as libc::c_int as VecWord)
                            << (val as libc::c_ulong
                                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong));
                }
                enum_word >>= 1 as libc::c_int;
                j += 1;
                j;
            }
            enum_i = enum_i.offset(1);
            if !(enum_i < enum_end) {
                break;
            }
        }
    }
    Pl_Range_From_Vector(v);
}
pub unsafe extern "C" fn Pl_Fd_Element_V_To_I(
    mut i: *mut Range,
    mut v: *mut Range,
    mut l: *mut WamWord,
) {
    let mut val: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    (*i)
        .vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh4 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh4 = (*fresh4).offset(pl_vec_size as isize);
    Pl_Vector_Empty((*i).vec);
    n = *l as libc::c_int;
    j = 1 as libc::c_int;
    while j <= n {
        val = *l.offset(j as isize) as libc::c_int;
        if Pl_Range_Test_Value(v, val) != 0 {
            let ref mut fresh5 = *((*i).vec)
                .offset((j as VecWord >> 6 as libc::c_int) as isize);
            *fresh5
                |= (1 as libc::c_int as VecWord)
                    << (j as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
        }
        j += 1;
        j;
    }
    Pl_Range_From_Vector(i);
}
pub unsafe extern "C" fn Pl_Fd_Element_Var_I(mut i: *mut Range, mut l: *mut WamWord) {
    let mut n: libc::c_int = *l as libc::c_int;
    (*i).extra_cstr = 0 as libc::c_int;
    (*i).min = 1 as libc::c_int;
    (*i).max = n;
    (*i).vec = 0 as Vector;
    Pl_Range_Becomes_Sparse(i);
}
pub unsafe extern "C" fn Pl_Fd_Element_Var_I_To_V(
    mut v: *mut Range,
    mut i: *mut Range,
    mut l: *mut *mut WamWord,
) {
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut j: libc::c_int = 0;
    (*v).extra_cstr = 0 as libc::c_int;
    (*v).vec = 0 as Vector;
    (*v)
        .max = (1 as libc::c_int)
        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if (*i).min == (*i).max || ((*i).vec).is_null() {
        j = (*i).min;
        while j <= (*i).max {
            fdv_adr = *l.offset(j as isize);
            Pl_Range_Union(
                v,
                fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range,
            );
            j += 1;
            j;
        }
    } else {
        let mut enum_end: Vector = ((*i).vec).offset(pl_vec_size as isize);
        let mut enum_i: Vector = (*i).vec;
        let mut enum_j: libc::c_int = 0;
        let mut enum_word: VecWord = 0;
        j = 0 as libc::c_int;
        loop {
            enum_word = *enum_i;
            enum_j = 0 as libc::c_int;
            loop {
                let fresh6 = enum_j;
                enum_j = enum_j + 1;
                if !(fresh6 < 8 as libc::c_int * 8 as libc::c_int) {
                    break;
                }
                if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                    fdv_adr = *l.offset(j as isize);
                    Pl_Range_Union(
                        v,
                        fdv_adr
                            .offset(4 as libc::c_int as isize)
                            .offset(2 as libc::c_int as isize) as *mut Range,
                    );
                }
                enum_word >>= 1 as libc::c_int;
                j += 1;
                j;
            }
            enum_i = enum_i.offset(1);
            if !(enum_i < enum_end) {
                break;
            }
        }
    };
}
pub unsafe extern "C" fn Pl_Fd_Element_Var_V_To_I(
    mut i: *mut Range,
    mut v: *mut Range,
    mut l: *mut *mut WamWord,
) {
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: PlLong = 0;
    let mut j: libc::c_int = 0;
    (*i)
        .vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh7 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh7 = (*fresh7).offset(pl_vec_size as isize);
    Pl_Vector_Empty((*i).vec);
    n = *l as PlLong;
    j = 1 as libc::c_int;
    while j as libc::c_long <= n {
        fdv_adr = *l.offset(j as isize);
        if Pl_Range_Test_Null_Inter(
            fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
                as *mut Range,
            v,
        ) == 0
        {
            let ref mut fresh8 = *((*i).vec)
                .offset((j as VecWord >> 6 as libc::c_int) as isize);
            *fresh8
                |= (1 as libc::c_int as VecWord)
                    << (j as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
        }
        j += 1;
        j;
    }
    Pl_Range_From_Vector(i);
}
pub unsafe extern "C" fn Pl_Fd_Element_V_To_Xi(
    mut i: libc::c_int,
    mut array: *mut *mut WamWord,
    mut v: *mut Range,
) -> Bool {
    let mut fdv_adr: *mut WamWord = *array.offset(i as isize);
    if *fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
        & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong
    {
        return Pl_Fd_Tell_Int_Range(fdv_adr, v);
    }
    return Pl_Fd_Tell_Range_Range(fdv_adr, v);
}
pub unsafe extern "C" fn Pl_Fd_Atmost(
    mut n: libc::c_int,
    mut array: *mut *mut WamWord,
    mut v: libc::c_int,
) -> Bool {
    let mut p: *mut *mut WamWord = 0 as *mut *mut WamWord;
    let mut word: WamWord = ((v as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
    let mut size: PlLong = *array.offset(0 as libc::c_int as isize) as PlLong;
    let mut nb: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    array = array.offset(1);
    array;
    p = array;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < size {
        if *(*p).offset(0 as libc::c_int as isize) == word {
            nb += 1;
            nb;
        }
        p = p.offset(1);
        p;
        i += 1;
        i;
    }
    if nb > n {
        return 0 as libc::c_int;
    }
    if nb == n {
        p = array;
        i = 0 as libc::c_int;
        while (i as libc::c_long) < size {
            if !(*(*p).offset(0 as libc::c_int as isize) as libc::c_ulong
                & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong)
            {
                if Pl_Fd_Tell_Not_Value(*p, v) == 0 {
                    return 0 as libc::c_int;
                }
            }
            p = p.offset(1);
            p;
            i += 1;
            i;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Atleast(
    mut n: libc::c_int,
    mut array: *mut *mut WamWord,
    mut v: libc::c_int,
) -> Bool {
    let mut p: *mut *mut WamWord = 0 as *mut *mut WamWord;
    let mut size: PlLong = *array.offset(0 as libc::c_int as isize) as PlLong;
    let mut nb: libc::c_int = size as libc::c_int;
    let mut i: libc::c_int = 0;
    array = array.offset(1);
    array;
    p = array;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < size {
        if Pl_Range_Test_Value(
            (*p).offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
                as *mut Range,
            v,
        ) == 0
        {
            nb -= 1;
            nb;
        }
        p = p.offset(1);
        p;
        i += 1;
        i;
    }
    if nb < n {
        return 0 as libc::c_int;
    }
    if nb == n {
        p = array;
        i = 0 as libc::c_int;
        while (i as libc::c_long) < size {
            if Pl_Range_Test_Value(
                (*p).offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
                    as *mut Range,
                v,
            ) != 0
            {
                if Pl_Fd_Tell_Value(*p, v) == 0 {
                    return 0 as libc::c_int;
                }
            }
            p = p.offset(1);
            p;
            i += 1;
            i;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Exactly(
    mut n: libc::c_int,
    mut array: *mut *mut WamWord,
    mut v: libc::c_int,
) -> Bool {
    let mut p: *mut *mut WamWord = 0 as *mut *mut WamWord;
    let mut word: WamWord = ((v as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
    let mut size: PlLong = *array.offset(0 as libc::c_int as isize) as PlLong;
    let mut nb1: libc::c_int = 0 as libc::c_int;
    let mut nb2: libc::c_int = size as libc::c_int;
    let mut i: libc::c_int = 0;
    array = array.offset(1);
    array;
    p = array;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < size {
        if *(*p).offset(0 as libc::c_int as isize) == word {
            nb1 += 1;
            nb1;
        } else if Pl_Range_Test_Value(
            (*p).offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
                as *mut Range,
            v,
        ) == 0
        {
            nb2 -= 1;
            nb2;
        }
        p = p.offset(1);
        p;
        i += 1;
        i;
    }
    if nb1 > n || nb2 < n {
        return 0 as libc::c_int;
    }
    if nb1 == n {
        p = array;
        i = 0 as libc::c_int;
        while (i as libc::c_long) < size {
            if !(*(*p).offset(0 as libc::c_int as isize) as libc::c_ulong
                & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong)
            {
                if Pl_Fd_Tell_Not_Value(*p, v) == 0 {
                    return 0 as libc::c_int;
                }
            }
            p = p.offset(1);
            p;
            i += 1;
            i;
        }
        return 1 as libc::c_int;
    }
    if nb2 == n {
        p = array;
        i = 0 as libc::c_int;
        while (i as libc::c_long) < size {
            if Pl_Range_Test_Value(
                (*p).offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
                    as *mut Range,
                v,
            ) != 0
            {
                if Pl_Fd_Tell_Value(*p, v) == 0 {
                    return 0 as libc::c_int;
                }
            }
            p = p.offset(1);
            p;
            i += 1;
            i;
        }
    }
    return 1 as libc::c_int;
}
