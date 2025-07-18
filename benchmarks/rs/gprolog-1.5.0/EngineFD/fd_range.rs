use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut pl_vec_max_integer: WamWord;
    static mut pl_vec_size: WamWord;
}
pub type Bool = libc::c_int;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
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
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub unsafe extern "C" fn Pl_Define_Vector_Size(mut max_val: libc::c_int) {
    pl_vec_size = (max_val / (8 as libc::c_int * 8 as libc::c_int) + 1 as libc::c_int)
        as WamWord;
    pl_vec_max_integer = pl_vec_size
        * (8 as libc::c_int * 8 as libc::c_int) as libc::c_long
        - 1 as libc::c_int as libc::c_long;
}
pub unsafe extern "C" fn Pl_Vector_From_Interval(
    mut vec: Vector,
    mut min: libc::c_int,
    mut max: libc::c_int,
) {
    let mut w_min: Vector = vec.offset((min as VecWord >> 6 as libc::c_int) as isize);
    let mut w_max: Vector = vec.offset((max as VecWord >> 6 as libc::c_int) as isize);
    let mut end: Vector = vec.offset(pl_vec_size as isize);
    while !(vec == w_min) {
        let fresh0 = vec;
        vec = vec.offset(1);
        *fresh0 = 0 as libc::c_int as VecWord;
    }
    while !(vec > w_max) {
        let fresh1 = vec;
        vec = vec.offset(1);
        *fresh1 = -(1 as libc::c_int) as VecWord;
    }
    while !(vec == end) {
        let fresh2 = vec;
        vec = vec.offset(1);
        *fresh2 = 0 as libc::c_int as VecWord;
    }
    *w_min
        &= (-(1 as libc::c_int) as VecWord)
            << (min as libc::c_ulong
                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong));
    *w_max
        &= -(1 as libc::c_int) as VecWord
            >> ((8 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_sub(
                    max as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
}
pub unsafe extern "C" fn Pl_Vector_Nb_Elem(mut vec: Vector) -> libc::c_int {
    let mut end: Vector = vec.offset(pl_vec_size as isize);
    let mut nb_elem: libc::c_int = 0 as libc::c_int;
    loop {
        nb_elem += (*vec).count_ones() as i32;
        vec = vec.offset(1);
        vec;
        if !(vec < end) {
            break;
        }
    }
    return nb_elem;
}
pub unsafe extern "C" fn Pl_Vector_Ith_Elem(
    mut vec: Vector,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut vec_elem: libc::c_int = 0;
    if i > 0 as libc::c_int {
        let mut enum_end: Vector = vec.offset(pl_vec_size as isize);
        let mut enum_i: Vector = vec;
        let mut enum_j: libc::c_int = 0;
        let mut enum_word: VecWord = 0;
        vec_elem = 0 as libc::c_int;
        loop {
            enum_word = *enum_i;
            enum_j = 0 as libc::c_int;
            loop {
                let fresh3 = enum_j;
                enum_j = enum_j + 1;
                if !(fresh3 < 8 as libc::c_int * 8 as libc::c_int) {
                    break;
                }
                if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                    i -= 1;
                    if i == 0 as libc::c_int {
                        return vec_elem;
                    }
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
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Vector_Next_After(
    mut vec: Vector,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut word_no: libc::c_int = 0;
    let mut bit_no: libc::c_int = 0;
    let mut start: Vector = 0 as *mut VecWord;
    let mut end: Vector = 0 as *mut VecWord;
    let mut word: VecWord = 0;
    let mut bit: libc::c_int = 0;
    if n >= 0 as libc::c_int {
        if n as libc::c_long > pl_vec_max_integer {
            return -(1 as libc::c_int);
        }
        word_no = (n as VecWord >> 6 as libc::c_int) as libc::c_int;
        bit_no = (n as libc::c_ulong
            & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        start = vec.offset(word_no as isize);
        word = if bit_no == 8 as libc::c_int * 8 as libc::c_int {
            0 as libc::c_int as libc::c_ulong
        } else {
            *start
                & !(((1 as libc::c_int as PlLong) << bit_no)
                    - 1 as libc::c_int as libc::c_long) as libc::c_ulong
        };
    } else {
        start = vec;
        word = *start;
    }
    end = vec.offset(pl_vec_size as isize);
    while word == 0 as libc::c_int as libc::c_ulong {
        start = start.offset(1);
        if start >= end {
            return -(1 as libc::c_int);
        }
        word = *start;
    }
    bit = word.trailing_zeros() as i32;
    n = ((start.offset_from(vec) as libc::c_long as VecWord) << 6 as libc::c_int
        | bit as VecWord) as libc::c_int;
    return n;
}
pub unsafe extern "C" fn Pl_Vector_Next_Before(
    mut vec: Vector,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut word_no: libc::c_int = 0;
    let mut bit_no: libc::c_int = 0;
    let mut start: Vector = 0 as *mut VecWord;
    let mut end: Vector = 0 as *mut VecWord;
    let mut word: VecWord = 0;
    let mut bit: libc::c_int = 0;
    if n as libc::c_long <= pl_vec_max_integer {
        if n < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        word_no = (n as VecWord >> 6 as libc::c_int) as libc::c_int;
        bit_no = (n as libc::c_ulong
            & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        end = vec.offset(word_no as isize);
        word = *end
            & (((1 as libc::c_int as PlLong) << bit_no)
                - 1 as libc::c_int as libc::c_long) as libc::c_ulong;
    } else {
        end = vec.offset(pl_vec_size as isize).offset(-(1 as libc::c_int as isize));
        word = *end;
    }
    start = vec;
    while word == 0 as libc::c_int as libc::c_ulong {
        end = end.offset(-1);
        if end < start {
            return -(1 as libc::c_int);
        }
        word = *end;
    }
    bit = 8 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int
        - word.leading_zeros() as i32;
    n = ((end.offset_from(vec) as libc::c_long as VecWord) << 6 as libc::c_int
        | bit as VecWord) as libc::c_int;
    return n;
}
pub unsafe extern "C" fn Pl_Vector_Empty(mut vec: Vector) {
    let mut end: Vector = vec.offset(pl_vec_size as isize);
    loop {
        let fresh4 = vec;
        vec = vec.offset(1);
        *fresh4 = 0 as libc::c_int as VecWord;
        if !(vec < end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Full(mut vec: Vector) {
    let mut end: Vector = vec.offset(pl_vec_size as isize);
    loop {
        let fresh5 = vec;
        vec = vec.offset(1);
        *fresh5 = -(1 as libc::c_int) as VecWord;
        if !(vec < end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Test_Null_Inter(
    mut vec: Vector,
    mut vec1: Vector,
) -> Bool {
    let mut end: Vector = vec.offset(pl_vec_size as isize);
    loop {
        let fresh6 = vec;
        vec = vec.offset(1);
        let fresh7 = vec1;
        vec1 = vec1.offset(1);
        if *fresh6 & *fresh7 != 0 {
            return 0 as libc::c_int;
        }
        if !(vec < end) {
            break;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Vector_Copy(mut vec: Vector, mut vec1: Vector) {
    let mut end: Vector = vec.offset(pl_vec_size as isize);
    loop {
        let fresh8 = vec1;
        vec1 = vec1.offset(1);
        let fresh9 = vec;
        vec = vec.offset(1);
        *fresh9 = *fresh8;
        if !(vec < end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Union(mut vec: Vector, mut vec1: Vector) {
    let mut end: Vector = vec.offset(pl_vec_size as isize);
    loop {
        let fresh10 = vec1;
        vec1 = vec1.offset(1);
        let fresh11 = vec;
        vec = vec.offset(1);
        *fresh11 |= *fresh10;
        if !(vec < end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Inter(mut vec: Vector, mut vec1: Vector) {
    let mut end: Vector = vec.offset(pl_vec_size as isize);
    loop {
        let fresh12 = vec1;
        vec1 = vec1.offset(1);
        let fresh13 = vec;
        vec = vec.offset(1);
        *fresh13 &= *fresh12;
        if !(vec < end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Compl(mut vec: Vector) {
    let mut end: Vector = vec.offset(pl_vec_size as isize);
    loop {
        *vec = !*vec;
        vec = vec.offset(1);
        vec;
        if !(vec < end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Add_Vector(mut vec: Vector, mut vec1: Vector) {
    let mut aux_vec: Vector = 0 as *mut VecWord;
    let mut vec_elem: libc::c_int = 0;
    let mut vec_elem1: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    aux_vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh14 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh14 = (*fresh14).offset(pl_vec_size as isize);
    Pl_Vector_Copy(aux_vec, vec);
    Pl_Vector_Empty(vec);
    let mut enum_end: Vector = aux_vec.offset(pl_vec_size as isize);
    let mut enum_i: Vector = aux_vec;
    let mut enum_j: libc::c_int = 0;
    let mut enum_word: VecWord = 0;
    vec_elem = 0 as libc::c_int;
    loop {
        enum_word = *enum_i;
        enum_j = 0 as libc::c_int;
        loop {
            let fresh15 = enum_j;
            enum_j = enum_j + 1;
            if !(fresh15 < 8 as libc::c_int * 8 as libc::c_int) {
                break;
            }
            if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                let mut enum_end_0: Vector = vec1.offset(pl_vec_size as isize);
                let mut enum_i_0: Vector = vec1;
                let mut enum_j_0: libc::c_int = 0;
                let mut enum_word_0: VecWord = 0;
                vec_elem1 = 0 as libc::c_int;
                's_74: loop {
                    enum_word_0 = *enum_i_0;
                    enum_j_0 = 0 as libc::c_int;
                    loop {
                        let fresh16 = enum_j_0;
                        enum_j_0 = enum_j_0 + 1;
                        if !(fresh16 < 8 as libc::c_int * 8 as libc::c_int) {
                            break;
                        }
                        if enum_word_0 & 1 as libc::c_int as libc::c_ulong != 0 {
                            x = vec_elem + vec_elem1;
                            if x as libc::c_long > pl_vec_max_integer {
                                break 's_74;
                            }
                            let ref mut fresh17 = *vec
                                .offset((x as VecWord >> 6 as libc::c_int) as isize);
                            *fresh17
                                |= (1 as libc::c_int as VecWord)
                                    << (x as libc::c_ulong
                                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
                        }
                        enum_word_0 >>= 1 as libc::c_int;
                        vec_elem1 += 1;
                        vec_elem1;
                    }
                    enum_i_0 = enum_i_0.offset(1);
                    if !(enum_i_0 < enum_end_0) {
                        break;
                    }
                }
            }
            enum_word >>= 1 as libc::c_int;
            vec_elem += 1;
            vec_elem;
        }
        enum_i = enum_i.offset(1);
        if !(enum_i < enum_end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Sub_Vector(mut vec: Vector, mut vec1: Vector) {
    let mut aux_vec: Vector = 0 as *mut VecWord;
    let mut vec_elem: libc::c_int = 0;
    let mut vec_elem1: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    aux_vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh18 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh18 = (*fresh18).offset(pl_vec_size as isize);
    Pl_Vector_Copy(aux_vec, vec);
    Pl_Vector_Empty(vec);
    let mut enum_end: Vector = aux_vec.offset(pl_vec_size as isize);
    let mut enum_i: Vector = aux_vec;
    let mut enum_j: libc::c_int = 0;
    let mut enum_word: VecWord = 0;
    vec_elem = 0 as libc::c_int;
    loop {
        enum_word = *enum_i;
        enum_j = 0 as libc::c_int;
        loop {
            let fresh19 = enum_j;
            enum_j = enum_j + 1;
            if !(fresh19 < 8 as libc::c_int * 8 as libc::c_int) {
                break;
            }
            if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                let mut enum_end_0: Vector = vec1.offset(pl_vec_size as isize);
                let mut enum_i_0: Vector = vec1;
                let mut enum_j_0: libc::c_int = 0;
                let mut enum_word_0: VecWord = 0;
                vec_elem1 = 0 as libc::c_int;
                's_74: loop {
                    enum_word_0 = *enum_i_0;
                    enum_j_0 = 0 as libc::c_int;
                    loop {
                        let fresh20 = enum_j_0;
                        enum_j_0 = enum_j_0 + 1;
                        if !(fresh20 < 8 as libc::c_int * 8 as libc::c_int) {
                            break;
                        }
                        if enum_word_0 & 1 as libc::c_int as libc::c_ulong != 0 {
                            x = vec_elem - vec_elem1;
                            if x < 0 as libc::c_int {
                                break 's_74;
                            }
                            let ref mut fresh21 = *vec
                                .offset((x as VecWord >> 6 as libc::c_int) as isize);
                            *fresh21
                                |= (1 as libc::c_int as VecWord)
                                    << (x as libc::c_ulong
                                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
                        }
                        enum_word_0 >>= 1 as libc::c_int;
                        vec_elem1 += 1;
                        vec_elem1;
                    }
                    enum_i_0 = enum_i_0.offset(1);
                    if !(enum_i_0 < enum_end_0) {
                        break;
                    }
                }
            }
            enum_word >>= 1 as libc::c_int;
            vec_elem += 1;
            vec_elem;
        }
        enum_i = enum_i.offset(1);
        if !(enum_i < enum_end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Mul_Vector(mut vec: Vector, mut vec1: Vector) {
    let mut aux_vec: Vector = 0 as *mut VecWord;
    let mut vec_elem: libc::c_int = 0;
    let mut vec_elem1: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    aux_vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh22 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh22 = (*fresh22).offset(pl_vec_size as isize);
    Pl_Vector_Copy(aux_vec, vec);
    Pl_Vector_Empty(vec);
    let mut enum_end: Vector = aux_vec.offset(pl_vec_size as isize);
    let mut enum_i: Vector = aux_vec;
    let mut enum_j: libc::c_int = 0;
    let mut enum_word: VecWord = 0;
    vec_elem = 0 as libc::c_int;
    loop {
        enum_word = *enum_i;
        enum_j = 0 as libc::c_int;
        loop {
            let fresh23 = enum_j;
            enum_j = enum_j + 1;
            if !(fresh23 < 8 as libc::c_int * 8 as libc::c_int) {
                break;
            }
            if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                let mut enum_end_0: Vector = vec1.offset(pl_vec_size as isize);
                let mut enum_i_0: Vector = vec1;
                let mut enum_j_0: libc::c_int = 0;
                let mut enum_word_0: VecWord = 0;
                vec_elem1 = 0 as libc::c_int;
                's_74: loop {
                    enum_word_0 = *enum_i_0;
                    enum_j_0 = 0 as libc::c_int;
                    loop {
                        let fresh24 = enum_j_0;
                        enum_j_0 = enum_j_0 + 1;
                        if !(fresh24 < 8 as libc::c_int * 8 as libc::c_int) {
                            break;
                        }
                        if enum_word_0 & 1 as libc::c_int as libc::c_ulong != 0 {
                            x = vec_elem * vec_elem1;
                            if x as libc::c_long > pl_vec_max_integer {
                                break 's_74;
                            }
                            let ref mut fresh25 = *vec
                                .offset((x as VecWord >> 6 as libc::c_int) as isize);
                            *fresh25
                                |= (1 as libc::c_int as VecWord)
                                    << (x as libc::c_ulong
                                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
                        }
                        enum_word_0 >>= 1 as libc::c_int;
                        vec_elem1 += 1;
                        vec_elem1;
                    }
                    enum_i_0 = enum_i_0.offset(1);
                    if !(enum_i_0 < enum_end_0) {
                        break;
                    }
                }
            }
            enum_word >>= 1 as libc::c_int;
            vec_elem += 1;
            vec_elem;
        }
        enum_i = enum_i.offset(1);
        if !(enum_i < enum_end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Div_Vector(mut vec: Vector, mut vec1: Vector) {
    let mut aux_vec: Vector = 0 as *mut VecWord;
    let mut vec_elem: libc::c_int = 0;
    let mut vec_elem1: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    aux_vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh26 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh26 = (*fresh26).offset(pl_vec_size as isize);
    Pl_Vector_Copy(aux_vec, vec);
    Pl_Vector_Empty(vec);
    let mut enum_end: Vector = aux_vec.offset(pl_vec_size as isize);
    let mut enum_i: Vector = aux_vec;
    let mut enum_j: libc::c_int = 0;
    let mut enum_word: VecWord = 0;
    vec_elem = 0 as libc::c_int;
    loop {
        enum_word = *enum_i;
        enum_j = 0 as libc::c_int;
        loop {
            let fresh27 = enum_j;
            enum_j = enum_j + 1;
            if !(fresh27 < 8 as libc::c_int * 8 as libc::c_int) {
                break;
            }
            if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                if vec_elem == 0 as libc::c_int {
                    let ref mut fresh28 = *vec
                        .offset(
                            (0 as libc::c_int as VecWord >> 6 as libc::c_int) as isize,
                        );
                    *fresh28
                        |= (1 as libc::c_int as VecWord)
                            << (0 as libc::c_int as libc::c_ulong
                                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong));
                } else {
                    let mut enum_end_0: Vector = vec1.offset(pl_vec_size as isize);
                    let mut enum_i_0: Vector = vec1;
                    let mut enum_j_0: libc::c_int = 0;
                    let mut enum_word_0: VecWord = 0;
                    vec_elem1 = 0 as libc::c_int;
                    loop {
                        enum_word_0 = *enum_i_0;
                        enum_j_0 = 0 as libc::c_int;
                        loop {
                            let fresh29 = enum_j_0;
                            enum_j_0 = enum_j_0 + 1;
                            if !(fresh29 < 8 as libc::c_int * 8 as libc::c_int) {
                                break;
                            }
                            if enum_word_0 & 1 as libc::c_int as libc::c_ulong != 0 {
                                if vec_elem1 != 0 as libc::c_int
                                    && vec_elem % vec_elem1 == 0 as libc::c_int
                                {
                                    x = vec_elem / vec_elem1;
                                    let ref mut fresh30 = *vec
                                        .offset((x as VecWord >> 6 as libc::c_int) as isize);
                                    *fresh30
                                        |= (1 as libc::c_int as VecWord)
                                            << (x as libc::c_ulong
                                                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong));
                                }
                            }
                            enum_word_0 >>= 1 as libc::c_int;
                            vec_elem1 += 1;
                            vec_elem1;
                        }
                        enum_i_0 = enum_i_0.offset(1);
                        if !(enum_i_0 < enum_end_0) {
                            break;
                        }
                    }
                }
            }
            enum_word >>= 1 as libc::c_int;
            vec_elem += 1;
            vec_elem;
        }
        enum_i = enum_i.offset(1);
        if !(enum_i < enum_end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Mod_Vector(mut vec: Vector, mut vec1: Vector) {
    let mut aux_vec: Vector = 0 as *mut VecWord;
    let mut vec_elem: libc::c_int = 0;
    let mut vec_elem1: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    aux_vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh31 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh31 = (*fresh31).offset(pl_vec_size as isize);
    Pl_Vector_Copy(aux_vec, vec);
    Pl_Vector_Empty(vec);
    let mut enum_end: Vector = aux_vec.offset(pl_vec_size as isize);
    let mut enum_i: Vector = aux_vec;
    let mut enum_j: libc::c_int = 0;
    let mut enum_word: VecWord = 0;
    vec_elem = 0 as libc::c_int;
    loop {
        enum_word = *enum_i;
        enum_j = 0 as libc::c_int;
        loop {
            let fresh32 = enum_j;
            enum_j = enum_j + 1;
            if !(fresh32 < 8 as libc::c_int * 8 as libc::c_int) {
                break;
            }
            if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                let mut enum_end_0: Vector = vec1.offset(pl_vec_size as isize);
                let mut enum_i_0: Vector = vec1;
                let mut enum_j_0: libc::c_int = 0;
                let mut enum_word_0: VecWord = 0;
                vec_elem1 = 0 as libc::c_int;
                loop {
                    enum_word_0 = *enum_i_0;
                    enum_j_0 = 0 as libc::c_int;
                    loop {
                        let fresh33 = enum_j_0;
                        enum_j_0 = enum_j_0 + 1;
                        if !(fresh33 < 8 as libc::c_int * 8 as libc::c_int) {
                            break;
                        }
                        if enum_word_0 & 1 as libc::c_int as libc::c_ulong != 0 {
                            if vec_elem1 != 0 as libc::c_int {
                                x = vec_elem % vec_elem1;
                                let ref mut fresh34 = *vec
                                    .offset((x as VecWord >> 6 as libc::c_int) as isize);
                                *fresh34
                                    |= (1 as libc::c_int as VecWord)
                                        << (x as libc::c_ulong
                                            & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong));
                            }
                        }
                        enum_word_0 >>= 1 as libc::c_int;
                        vec_elem1 += 1;
                        vec_elem1;
                    }
                    enum_i_0 = enum_i_0.offset(1);
                    if !(enum_i_0 < enum_end_0) {
                        break;
                    }
                }
            }
            enum_word >>= 1 as libc::c_int;
            vec_elem += 1;
            vec_elem;
        }
        enum_i = enum_i.offset(1);
        if !(enum_i < enum_end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Add_Value(mut vec: Vector, mut n: libc::c_int) {
    let mut word_no: libc::c_int = 0;
    let mut bit_no: libc::c_int = 0;
    let mut rem: VecWord = 0;
    let mut rem1: VecWord = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if n >= 0 as libc::c_int {
        word_no = (n as VecWord >> 6 as libc::c_int) as libc::c_int;
        bit_no = (n as libc::c_ulong
            & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        if word_no != 0 {
            i = (pl_vec_size - 1 as libc::c_int as libc::c_long) as libc::c_int;
            j = (pl_vec_size - 1 as libc::c_int as libc::c_long
                - word_no as libc::c_long) as libc::c_int;
            while j >= 0 as libc::c_int {
                let fresh35 = j;
                j = j - 1;
                let fresh36 = i;
                i = i - 1;
                *vec.offset(fresh36 as isize) = *vec.offset(fresh35 as isize);
            }
            while i >= 0 as libc::c_int {
                let fresh37 = i;
                i = i - 1;
                *vec.offset(fresh37 as isize) = 0 as libc::c_int as VecWord;
            }
        }
        if bit_no != 0 {
            rem = 0 as libc::c_int as VecWord;
            i = word_no;
            while (i as libc::c_long) < pl_vec_size {
                rem1 = *vec.offset(i as isize)
                    >> 8 as libc::c_int * 8 as libc::c_int - bit_no;
                *vec.offset(i as isize) = *vec.offset(i as isize) << bit_no | rem;
                rem = rem1;
                i += 1;
                i;
            }
        }
    } else {
        word_no = (-n as VecWord >> 6 as libc::c_int) as libc::c_int;
        bit_no = (-n as libc::c_ulong
            & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        if word_no != 0 {
            i = 0 as libc::c_int;
            j = word_no;
            while (j as libc::c_long) < pl_vec_size {
                let fresh38 = j;
                j = j + 1;
                let fresh39 = i;
                i = i + 1;
                *vec.offset(fresh39 as isize) = *vec.offset(fresh38 as isize);
            }
            while (i as libc::c_long) < pl_vec_size {
                let fresh40 = i;
                i = i + 1;
                *vec.offset(fresh40 as isize) = 0 as libc::c_int as VecWord;
            }
        }
        if bit_no != 0 {
            rem = 0 as libc::c_int as VecWord;
            i = (pl_vec_size - 1 as libc::c_int as libc::c_long
                - word_no as libc::c_long) as libc::c_int;
            while i >= 0 as libc::c_int {
                rem1 = *vec.offset(i as isize)
                    << 8 as libc::c_int * 8 as libc::c_int - bit_no;
                *vec.offset(i as isize) = *vec.offset(i as isize) >> bit_no | rem;
                rem = rem1;
                i -= 1;
                i;
            }
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Mul_Value(mut vec: Vector, mut n: libc::c_int) {
    let mut aux_vec: Vector = 0 as *mut VecWord;
    let mut vec_elem: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    aux_vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh41 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh41 = (*fresh41).offset(pl_vec_size as isize);
    Pl_Vector_Copy(aux_vec, vec);
    Pl_Vector_Empty(vec);
    let mut enum_end: Vector = aux_vec.offset(pl_vec_size as isize);
    let mut enum_i: Vector = aux_vec;
    let mut enum_j: libc::c_int = 0;
    let mut enum_word: VecWord = 0;
    vec_elem = 0 as libc::c_int;
    loop {
        enum_word = *enum_i;
        enum_j = 0 as libc::c_int;
        loop {
            let fresh42 = enum_j;
            enum_j = enum_j + 1;
            if !(fresh42 < 8 as libc::c_int * 8 as libc::c_int) {
                break;
            }
            if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                x = vec_elem * n;
                if x as libc::c_uint > pl_vec_max_integer as libc::c_uint {
                    return;
                }
                let ref mut fresh43 = *vec
                    .offset((x as VecWord >> 6 as libc::c_int) as isize);
                *fresh43
                    |= (1 as libc::c_int as VecWord)
                        << (x as libc::c_ulong
                            & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            }
            enum_word >>= 1 as libc::c_int;
            vec_elem += 1;
            vec_elem;
        }
        enum_i = enum_i.offset(1);
        if !(enum_i < enum_end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Div_Value(mut vec: Vector, mut n: libc::c_int) {
    let mut aux_vec: Vector = 0 as *mut VecWord;
    let mut vec_elem: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    aux_vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh44 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh44 = (*fresh44).offset(pl_vec_size as isize);
    Pl_Vector_Copy(aux_vec, vec);
    Pl_Vector_Empty(vec);
    if n == 0 as libc::c_int {
        return;
    }
    let mut enum_end: Vector = aux_vec.offset(pl_vec_size as isize);
    let mut enum_i: Vector = aux_vec;
    let mut enum_j: libc::c_int = 0;
    let mut enum_word: VecWord = 0;
    vec_elem = 0 as libc::c_int;
    loop {
        enum_word = *enum_i;
        enum_j = 0 as libc::c_int;
        loop {
            let fresh45 = enum_j;
            enum_j = enum_j + 1;
            if !(fresh45 < 8 as libc::c_int * 8 as libc::c_int) {
                break;
            }
            if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                if vec_elem % n == 0 as libc::c_int {
                    x = vec_elem / n;
                    let ref mut fresh46 = *vec
                        .offset((x as VecWord >> 6 as libc::c_int) as isize);
                    *fresh46
                        |= (1 as libc::c_int as VecWord)
                            << (x as libc::c_ulong
                                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong));
                }
            }
            enum_word >>= 1 as libc::c_int;
            vec_elem += 1;
            vec_elem;
        }
        enum_i = enum_i.offset(1);
        if !(enum_i < enum_end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Vector_Mod_Value(mut vec: Vector, mut n: libc::c_int) {
    let mut aux_vec: Vector = 0 as *mut VecWord;
    let mut vec_elem: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    aux_vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh47 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh47 = (*fresh47).offset(pl_vec_size as isize);
    Pl_Vector_Copy(aux_vec, vec);
    Pl_Vector_Empty(vec);
    if n == 0 as libc::c_int {
        return;
    }
    let mut enum_end: Vector = aux_vec.offset(pl_vec_size as isize);
    let mut enum_i: Vector = aux_vec;
    let mut enum_j: libc::c_int = 0;
    let mut enum_word: VecWord = 0;
    vec_elem = 0 as libc::c_int;
    loop {
        enum_word = *enum_i;
        enum_j = 0 as libc::c_int;
        loop {
            let fresh48 = enum_j;
            enum_j = enum_j + 1;
            if !(fresh48 < 8 as libc::c_int * 8 as libc::c_int) {
                break;
            }
            if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                x = vec_elem % n;
                if x as libc::c_uint <= pl_vec_max_integer as libc::c_uint {
                    let ref mut fresh49 = *vec
                        .offset((x as VecWord >> 6 as libc::c_int) as isize);
                    *fresh49
                        |= (1 as libc::c_int as VecWord)
                            << (x as libc::c_ulong
                                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong));
                }
            }
            enum_word >>= 1 as libc::c_int;
            vec_elem += 1;
            vec_elem;
        }
        enum_i = enum_i.offset(1);
        if !(enum_i < enum_end) {
            break;
        }
    };
}
pub unsafe extern "C" fn Pl_Range_Test_Value(
    mut range: *mut Range,
    mut n: libc::c_int,
) -> Bool {
    let mut min: libc::c_int = (*range).min;
    let mut max: libc::c_int = (*range).max;
    if n < min || n > max {
        return 0 as libc::c_int;
    }
    if ((*range).vec).is_null() || n == min || n == max {
        return 1 as libc::c_int;
    }
    return (*((*range).vec).offset((n as VecWord >> 6 as libc::c_int) as isize)
        & (1 as libc::c_int as VecWord)
            << (n as libc::c_ulong
                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Range_Test_Null_Inter(
    mut range: *mut Range,
    mut range1: *mut Range,
) -> Bool {
    let mut swt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if (*range).min > (*range1).max || (*range1).min > (*range).max {
        return 1 as libc::c_int;
    }
    if (*range).min == (*range1).min || (*range).min == (*range1).max
        || (*range).max == (*range1).min || (*range).max == (*range1).max
    {
        return 0 as libc::c_int;
    }
    swt = ((((*range).vec != 0 as *mut libc::c_void as Vector) as libc::c_int)
        << 1 as libc::c_int)
        + ((*range1).vec != 0 as *mut libc::c_void as Vector) as libc::c_int;
    if swt == 3 as libc::c_int {
        return Pl_Vector_Test_Null_Inter((*range1).vec, (*range).vec);
    }
    if (*range).min >= (*range1).min && (*range).max >= (*range1).max
        || (*range1).min >= (*range).min && (*range1).max >= (*range).max
    {
        return 0 as libc::c_int;
    }
    if swt == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if swt == 2 as libc::c_int {
        return Pl_Range_Test_Null_Inter(range1, range);
    }
    if (*range).min <= (*range1).min {
        return 0 as libc::c_int;
    }
    i = (*range).min;
    while i <= (*range).max {
        if *((*range1).vec).offset((i as VecWord >> 6 as libc::c_int) as isize)
            & (1 as libc::c_int as VecWord)
                << (i as libc::c_ulong
                    & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
            != 0 as libc::c_int as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Range_Copy(mut range: *mut Range, mut range1: *mut Range) {
    (*range).extra_cstr = (*range1).extra_cstr;
    (*range).min = (*range1).min;
    (*range).max = (*range1).max;
    if ((*range1).vec).is_null() {
        (*range).vec = 0 as Vector;
    } else {
        if ((*range).vec).is_null() {
            (*range)
                .vec = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
            let ref mut fresh50 = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
            *fresh50 = (*fresh50).offset(pl_vec_size as isize);
        }
        Pl_Vector_Copy((*range).vec, (*range1).vec);
    };
}
pub unsafe extern "C" fn Pl_Range_Nb_Elem(mut range: *mut Range) -> libc::c_int {
    if ((*range).vec).is_null() {
        return (*range).max - (*range).min + 1 as libc::c_int;
    }
    return Pl_Vector_Nb_Elem((*range).vec);
}
pub unsafe extern "C" fn Pl_Range_Ith_Elem(
    mut range: *mut Range,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if (*range).min > (*range).max {
        return -(1 as libc::c_int);
    }
    if ((*range).vec).is_null() {
        n = (*range).min + i - 1 as libc::c_int;
        return if n < (*range).min || n > (*range).max {
            -(1 as libc::c_int)
        } else {
            n
        };
    }
    return Pl_Vector_Ith_Elem((*range).vec, i);
}
pub unsafe extern "C" fn Pl_Range_Next_After(
    mut range: *mut Range,
    mut n: libc::c_int,
) -> libc::c_int {
    if (*range).min > (*range).max {
        return -(1 as libc::c_int);
    }
    if ((*range).vec).is_null() {
        if n >= (*range).max {
            return -(1 as libc::c_int);
        }
        n += 1;
        n;
        if n < (*range).min {
            n = (*range).min;
        }
        return n;
    }
    return Pl_Vector_Next_After((*range).vec, n);
}
pub unsafe extern "C" fn Pl_Range_Next_Before(
    mut range: *mut Range,
    mut n: libc::c_int,
) -> libc::c_int {
    if (*range).min > (*range).max {
        return -(1 as libc::c_int);
    }
    if ((*range).vec).is_null() {
        if n <= (*range).min {
            return -(1 as libc::c_int);
        }
        n -= 1;
        n;
        if n > (*range).max {
            n = (*range).max;
        }
        return n;
    }
    return Pl_Vector_Next_Before((*range).vec, n);
}
pub unsafe extern "C" fn Pl_Range_Becomes_Sparse(mut range: *mut Range) {
    if ((*range).vec).is_null() {
        (*range)
            .vec = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
        let ref mut fresh51 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
        *fresh51 = (*fresh51).offset(pl_vec_size as isize);
    }
    if (*range).min < 0 as libc::c_int {
        (*range).min = 0 as libc::c_int;
    }
    (*range)
        .extra_cstr = ((*range).max as libc::c_long > pl_vec_max_integer) as libc::c_int;
    if (*range).extra_cstr != 0 {
        (*range).max = pl_vec_max_integer as libc::c_int;
    }
    if (*range).max >= (*range).min {
        Pl_Vector_From_Interval((*range).vec, (*range).min, (*range).max);
    }
}
pub unsafe extern "C" fn Pl_Range_From_Vector(mut range: *mut Range) {
    let mut start: Vector = 0 as *mut VecWord;
    let mut end: Vector = 0 as *mut VecWord;
    let mut bit: libc::c_int = 0;
    start = ((*range).vec).offset(-(1 as libc::c_int as isize));
    end = ((*range).vec).offset(pl_vec_size as isize);
    loop {
        start = start.offset(1);
        if *start != 0 {
            break;
        }
        if start >= end {
            (*range)
                .max = (1 as libc::c_int)
                << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            return;
        }
    }
    loop {
        end = end.offset(-1);
        if *end != 0 {
            break;
        }
    }
    bit = (*start).trailing_zeros() as i32;
    (*range)
        .min = ((start.offset_from((*range).vec) as libc::c_long as VecWord)
        << 6 as libc::c_int | bit as VecWord) as libc::c_int;
    bit = 8 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int
        - (*end).leading_zeros() as i32;
    (*range)
        .max = ((end.offset_from((*range).vec) as libc::c_long as VecWord)
        << 6 as libc::c_int | bit as VecWord) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Range_Set_Value(mut range: *mut Range, mut n: libc::c_int) {
    if (*range).min > (*range).max {
        (*range).extra_cstr = 0 as libc::c_int;
        (*range).min = n;
        (*range).max = n;
        (*range).vec = 0 as Vector;
        return;
    }
    if ((*range).vec).is_null() {
        if n >= (*range).min && n <= (*range).max {
            return;
        }
        if n == (*range).min - 1 as libc::c_int {
            (*range).min -= 1;
            (*range).min;
            return;
        }
        if n == (*range).max + 1 as libc::c_int {
            (*range).max += 1;
            (*range).max;
            return;
        }
        Pl_Range_Becomes_Sparse(range);
        if n as libc::c_uint <= pl_vec_max_integer as libc::c_uint {
            let ref mut fresh52 = *((*range).vec)
                .offset((n as VecWord >> 6 as libc::c_int) as isize);
            *fresh52
                |= (1 as libc::c_int as VecWord)
                    << (n as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            Pl_Range_From_Vector(range);
        } else {
            (*range).extra_cstr = 1 as libc::c_int;
        }
        return;
    }
    if n as libc::c_uint > pl_vec_max_integer as libc::c_uint {
        (*range).extra_cstr = 1 as libc::c_int;
        return;
    }
    let ref mut fresh53 = *((*range).vec)
        .offset((n as VecWord >> 6 as libc::c_int) as isize);
    *fresh53
        |= (1 as libc::c_int as VecWord)
            << (n as libc::c_ulong
                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong));
    if n < (*range).min || n > (*range).max {
        Pl_Range_From_Vector(range);
    }
}
pub unsafe extern "C" fn Pl_Range_Reset_Value(
    mut range: *mut Range,
    mut n: libc::c_int,
) {
    if (*range).min > (*range).max || n < (*range).min || n > (*range).max {
        return;
    }
    if (*range).min == (*range).max {
        (*range)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    if ((*range).vec).is_null() {
        if n == (*range).min {
            (*range).min += 1;
            (*range).min;
            return;
        }
        if n == (*range).max {
            (*range).max -= 1;
            (*range).max;
            return;
        }
        Pl_Range_Becomes_Sparse(range);
        if n as libc::c_uint <= pl_vec_max_integer as libc::c_uint {
            let ref mut fresh54 = *((*range).vec)
                .offset((n as VecWord >> 6 as libc::c_int) as isize);
            *fresh54
                &= !((1 as libc::c_int as VecWord)
                    << (n as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)));
        }
        return;
    }
    if n as libc::c_uint > pl_vec_max_integer as libc::c_uint {
        return;
    }
    let ref mut fresh55 = *((*range).vec)
        .offset((n as VecWord >> 6 as libc::c_int) as isize);
    *fresh55
        &= !((1 as libc::c_int as VecWord)
            << (n as libc::c_ulong
                & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)));
    if n == (*range).min || n == (*range).max {
        Pl_Range_From_Vector(range);
    }
}
pub unsafe extern "C" fn Pl_Range_Union(mut range: *mut Range, mut range1: *mut Range) {
    let mut swt: libc::c_int = ((((*range).vec != 0 as *mut libc::c_void as Vector)
        as libc::c_int) << 1 as libc::c_int)
        + ((*range1).vec != 0 as *mut libc::c_void as Vector) as libc::c_int;
    let mut r: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    let mut extra_cstr: Bool = 0;
    if swt == 0 as libc::c_int {
        if (*range).max >= (*range).min && (*range1).max >= (*range1).min
            && (*range1).min <= (*range).max + 1 as libc::c_int
            && (*range).min <= (*range1).max + 1 as libc::c_int
        {
            (*range)
                .min = if (*range).min <= (*range1).min {
                (*range).min
            } else {
                (*range1).min
            };
            (*range)
                .max = if (*range).max >= (*range1).max {
                (*range).max
            } else {
                (*range1).max
            };
            return;
        }
        Pl_Range_Becomes_Sparse(range);
        r.vec = 0 as Vector;
        Pl_Range_Copy(&mut r, range1);
        range1 = &mut r;
        Pl_Range_Becomes_Sparse(range1);
    } else if swt == 1 as libc::c_int {
        Pl_Range_Becomes_Sparse(range);
    } else if swt == 2 as libc::c_int {
        r.vec = 0 as Vector;
        Pl_Range_Copy(&mut r, range1);
        range1 = &mut r;
        Pl_Range_Becomes_Sparse(range1);
    }
    extra_cstr = (*range).extra_cstr | (*range1).extra_cstr;
    if (*range).min > (*range).max {
        Pl_Range_Copy(range, range1);
        (*range).extra_cstr = extra_cstr;
        return;
    }
    (*range).extra_cstr = extra_cstr;
    if (*range1).min > (*range1).max {
        return;
    }
    (*range)
        .min = if (*range).min <= (*range1).min { (*range).min } else { (*range1).min };
    (*range)
        .max = if (*range).max >= (*range1).max { (*range).max } else { (*range1).max };
    Pl_Vector_Union((*range).vec, (*range1).vec);
}
pub unsafe extern "C" fn Pl_Range_Inter(mut range: *mut Range, mut range1: *mut Range) {
    let mut swt: libc::c_int = ((((*range).vec != 0 as *mut libc::c_void as Vector)
        as libc::c_int) << 1 as libc::c_int)
        + ((*range1).vec != 0 as *mut libc::c_void as Vector) as libc::c_int;
    let mut r: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    if swt == 0 as libc::c_int {
        (*range)
            .min = if (*range).min >= (*range1).min {
            (*range).min
        } else {
            (*range1).min
        };
        (*range)
            .max = if (*range).max <= (*range1).max {
            (*range).max
        } else {
            (*range1).max
        };
        return;
    }
    if swt == 1 as libc::c_int {
        Pl_Range_Becomes_Sparse(range);
    } else if swt == 2 as libc::c_int {
        r.vec = 0 as Vector;
        Pl_Range_Copy(&mut r, range1);
        range1 = &mut r;
        Pl_Range_Becomes_Sparse(range1);
    }
    (*range).extra_cstr &= (*range1).extra_cstr;
    if (*range).min > (*range).max {
        return;
    }
    if (*range1).min > (*range1).max {
        (*range)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    Pl_Vector_Inter((*range).vec, (*range1).vec);
    Pl_Range_From_Vector(range);
}
pub unsafe extern "C" fn Pl_Range_Compl(mut range: *mut Range) {
    if ((*range).vec).is_null() {
        if (*range).min > (*range).max {
            (*range).min = 0 as libc::c_int;
            (*range)
                .max = (((1 as libc::c_int as PlLong)
                << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
                - 1 as libc::c_int as libc::c_long) as libc::c_int;
            return;
        }
        if (*range).min <= 0 as libc::c_int {
            if (*range).max
                >= (((1 as libc::c_int as PlLong)
                    << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
                    - 1 as libc::c_int as libc::c_long) as libc::c_int
            {
                (*range)
                    .max = (1 as libc::c_int)
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            } else {
                (*range).min = (*range).max + 1 as libc::c_int;
                (*range)
                    .max = (((1 as libc::c_int as PlLong)
                    << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
                    - 1 as libc::c_int as libc::c_long) as libc::c_int;
            }
            return;
        }
        if (*range).max
            >= (((1 as libc::c_int as PlLong)
                << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
                - 1 as libc::c_int as libc::c_long) as libc::c_int
        {
            (*range).max = (*range).min - 1 as libc::c_int;
            (*range).min = 0 as libc::c_int;
            return;
        }
        Pl_Range_Becomes_Sparse(range);
    }
    (*range).extra_cstr = 1 as libc::c_int;
    if (*range).min > (*range).max {
        (*range).min = 0 as libc::c_int;
        (*range).max = pl_vec_max_integer as libc::c_int;
        Pl_Vector_Full((*range).vec);
    } else {
        Pl_Vector_Compl((*range).vec);
        Pl_Range_From_Vector(range);
    };
}
pub unsafe extern "C" fn Pl_Range_Add_Range(
    mut range: *mut Range,
    mut range1: *mut Range,
) {
    let mut swt: libc::c_int = ((((*range).vec != 0 as *mut libc::c_void as Vector)
        as libc::c_int) << 1 as libc::c_int)
        + ((*range1).vec != 0 as *mut libc::c_void as Vector) as libc::c_int;
    let mut r: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    if (*range).min > (*range).max {
        return;
    }
    if (*range1).min > (*range1).max {
        (*range)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    if swt == 0 as libc::c_int {
        (*range).min += (*range1).min;
        (*range).max += (*range1).max;
        return;
    } else if swt == 1 as libc::c_int {
        Pl_Range_Becomes_Sparse(range);
    } else if swt == 2 as libc::c_int {
        r.vec = 0 as Vector;
        Pl_Range_Copy(&mut r, range1);
        range1 = &mut r;
        Pl_Range_Becomes_Sparse(range1);
    }
    Pl_Vector_Add_Vector((*range).vec, (*range1).vec);
    (*range).min += (*range1).min;
    (*range).max += (*range1).max;
    (*range).extra_cstr
        |= (*range1).extra_cstr
            | ((*range).max as libc::c_long > pl_vec_max_integer) as libc::c_int;
    if (*range).extra_cstr != 0 || (*range).min < 0 as libc::c_int {
        Pl_Range_From_Vector(range);
    }
}
pub unsafe extern "C" fn Pl_Range_Sub_Range(
    mut range: *mut Range,
    mut range1: *mut Range,
) {
    let mut swt: libc::c_int = ((((*range).vec != 0 as *mut libc::c_void as Vector)
        as libc::c_int) << 1 as libc::c_int)
        + ((*range1).vec != 0 as *mut libc::c_void as Vector) as libc::c_int;
    let mut r: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    if (*range).min > (*range).max {
        return;
    }
    if (*range1).min > (*range1).max {
        (*range)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    if swt == 0 as libc::c_int {
        (*range).min -= (*range1).max;
        (*range).max -= (*range1).min;
        return;
    } else if swt == 1 as libc::c_int {
        Pl_Range_Becomes_Sparse(range);
    } else if swt == 2 as libc::c_int {
        r.vec = 0 as Vector;
        Pl_Range_Copy(&mut r, range1);
        range1 = &mut r;
        Pl_Range_Becomes_Sparse(range1);
    }
    Pl_Vector_Sub_Vector((*range).vec, (*range1).vec);
    (*range).min -= (*range1).max;
    (*range).max -= (*range1).min;
    (*range).extra_cstr |= (*range1).extra_cstr;
    if (*range).extra_cstr != 0 || (*range).min < 0 as libc::c_int {
        Pl_Range_From_Vector(range);
    }
}
pub unsafe extern "C" fn Pl_Range_Mul_Range(
    mut range: *mut Range,
    mut range1: *mut Range,
) {
    let mut r: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    if (*range).min > (*range).max {
        return;
    }
    if (*range1).min > (*range1).max {
        (*range)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    if ((*range).vec).is_null() {
        Pl_Range_Becomes_Sparse(range);
    }
    if ((*range1).vec).is_null() {
        r.vec = 0 as Vector;
        Pl_Range_Copy(&mut r, range1);
        range1 = &mut r;
        Pl_Range_Becomes_Sparse(range1);
    }
    Pl_Vector_Mul_Vector((*range).vec, (*range1).vec);
    (*range).extra_cstr |= (*range1).extra_cstr;
    Pl_Range_From_Vector(range);
}
pub unsafe extern "C" fn Pl_Range_Div_Range(
    mut range: *mut Range,
    mut range1: *mut Range,
) {
    let mut r: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    if (*range).min > (*range).max {
        return;
    }
    if (*range1).min > (*range1).max {
        (*range)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    if ((*range).vec).is_null() {
        Pl_Range_Becomes_Sparse(range);
    }
    if ((*range1).vec).is_null() {
        r.vec = 0 as Vector;
        Pl_Range_Copy(&mut r, range1);
        range1 = &mut r;
        Pl_Range_Becomes_Sparse(range1);
    }
    Pl_Vector_Div_Vector((*range).vec, (*range1).vec);
    (*range).extra_cstr |= (*range1).extra_cstr;
    Pl_Range_From_Vector(range);
}
pub unsafe extern "C" fn Pl_Range_Mod_Range(
    mut range: *mut Range,
    mut range1: *mut Range,
) {
    let mut r: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    if (*range).min > (*range).max {
        return;
    }
    if (*range1).min > (*range1).max {
        (*range)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    if ((*range).vec).is_null() {
        Pl_Range_Becomes_Sparse(range);
    }
    if ((*range1).vec).is_null() {
        r.vec = 0 as Vector;
        Pl_Range_Copy(&mut r, range1);
        range1 = &mut r;
        Pl_Range_Becomes_Sparse(range1);
    }
    Pl_Vector_Mod_Vector((*range).vec, (*range1).vec);
    (*range).extra_cstr |= (*range1).extra_cstr;
    Pl_Range_From_Vector(range);
}
pub unsafe extern "C" fn Pl_Range_Add_Value(mut range: *mut Range, mut n: libc::c_int) {
    if n == 0 as libc::c_int || (*range).min > (*range).max {
        return;
    }
    if ((*range).vec).is_null() {
        (*range).min += n;
        (*range).max += n;
        return;
    }
    Pl_Vector_Add_Value((*range).vec, n);
    (*range).min += n;
    (*range).max += n;
    (*range).extra_cstr
        |= ((*range).max as libc::c_long > pl_vec_max_integer) as libc::c_int;
    if (*range).extra_cstr != 0 || (*range).min < 0 as libc::c_int {
        Pl_Range_From_Vector(range);
    }
}
pub unsafe extern "C" fn Pl_Range_Mul_Value(mut range: *mut Range, mut n: libc::c_int) {
    if n == 1 as libc::c_int || (*range).min > (*range).max {
        return;
    }
    if ((*range).vec).is_null() {
        Pl_Range_Becomes_Sparse(range);
    }
    Pl_Vector_Mul_Value((*range).vec, n);
    (*range).min = (*range).min * n;
    (*range).max = (*range).max * n;
    (*range).extra_cstr
        |= ((*range).max as libc::c_long > pl_vec_max_integer) as libc::c_int;
    if (*range).extra_cstr != 0 {
        Pl_Range_From_Vector(range);
    }
}
pub unsafe extern "C" fn Pl_Range_Div_Value(mut range: *mut Range, mut n: libc::c_int) {
    if n == 1 as libc::c_int || (*range).min > (*range).max {
        return;
    }
    if ((*range).vec).is_null() {
        Pl_Range_Becomes_Sparse(range);
    }
    Pl_Vector_Div_Value((*range).vec, n);
    (*range).min = ((*range).min + n - 1 as libc::c_int) / n;
    (*range).max = (*range).max / n;
    (*range).extra_cstr
        |= ((*range).max as libc::c_long > pl_vec_max_integer) as libc::c_int;
    if (*range).extra_cstr != 0 {
        Pl_Range_From_Vector(range);
    }
}
pub unsafe extern "C" fn Pl_Range_Mod_Value(mut range: *mut Range, mut n: libc::c_int) {
    let mut aux: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    if (*range).min > (*range).max {
        return;
    }
    if n < 0 as libc::c_int {
        n = -n;
    }
    if ((*range).vec).is_null() {
        if (*range).min >= 0 as libc::c_int {
            if (*range).max - (*range).min + 1 as libc::c_int >= n {
                (*range).min = 0 as libc::c_int;
                (*range).max = n - 1 as libc::c_int;
                return;
            }
            (*range).min = (*range).min % n;
            (*range).max = (*range).max % n;
            if (*range).min > (*range).max {
                aux.extra_cstr = 0 as libc::c_int;
                aux.min = 0 as libc::c_int;
                aux.max = (*range).max;
                aux.vec = 0 as Vector;
                (*range).max = n - 1 as libc::c_int;
                Pl_Range_Union(range, &mut aux);
            }
            return;
        }
        if (*range).max <= 0 as libc::c_int {
            if (*range).max - (*range).min + 1 as libc::c_int >= n {
                (*range).min = -(n - 1 as libc::c_int);
                (*range).max = 0 as libc::c_int;
                return;
            }
            (*range).min = (*range).min % n;
            (*range).max = (*range).max % n;
            if (*range).min > (*range).max {
                aux.extra_cstr = 0 as libc::c_int;
                aux.min = -(n - 1 as libc::c_int);
                aux.max = (*range).max;
                aux.vec = 0 as Vector;
                (*range).max = 0 as libc::c_int;
                Pl_Range_Union(range, &mut aux);
            }
            return;
        }
        (*range)
            .min = if (*range).min >= -n + 1 as libc::c_int {
            (*range).min
        } else {
            -n + 1 as libc::c_int
        };
        (*range)
            .max = if (*range).max <= n - 1 as libc::c_int {
            (*range).max
        } else {
            n - 1 as libc::c_int
        };
        return;
    }
    Pl_Vector_Mod_Value((*range).vec, n);
    Pl_Range_From_Vector(range);
}
pub unsafe extern "C" fn Pl_Range_To_String(mut range: *mut Range) -> *mut libc::c_char {
    let mut vec_elem: libc::c_int = 0;
    let mut limit1: libc::c_int = -(1 as libc::c_int);
    let mut limit2: libc::c_int = 0;
    static mut buff: [libc::c_char; 4096] = [0; 4096];
    if (*range).min > (*range).max {
        strcpy(
            buff.as_mut_ptr(),
            b"<empty range>\0" as *const u8 as *const libc::c_char,
        );
        return buff.as_mut_ptr();
    }
    if (*range).min == (*range).max {
        sprintf(
            buff.as_mut_ptr(),
            b"{%d}\0" as *const u8 as *const libc::c_char,
            (*range).min,
        );
        return buff.as_mut_ptr();
    }
    if ((*range).vec).is_null() {
        sprintf(
            buff.as_mut_ptr(),
            b"%s%d%s%d%s\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            (*range).min,
            b"..\0" as *const u8 as *const libc::c_char,
            (*range).max,
            b"\0" as *const u8 as *const libc::c_char,
        );
        return buff.as_mut_ptr();
    }
    strcpy(buff.as_mut_ptr(), b"\0" as *const u8 as *const libc::c_char);
    let mut enum_end: Vector = ((*range).vec).offset(pl_vec_size as isize);
    let mut enum_i: Vector = (*range).vec;
    let mut enum_j: libc::c_int = 0;
    let mut enum_word: VecWord = 0;
    vec_elem = 0 as libc::c_int;
    loop {
        enum_word = *enum_i;
        enum_j = 0 as libc::c_int;
        loop {
            let fresh56 = enum_j;
            enum_j = enum_j + 1;
            if !(fresh56 < 8 as libc::c_int * 8 as libc::c_int) {
                break;
            }
            if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                if limit1 == -(1 as libc::c_int) {
                    limit2 = vec_elem;
                    limit1 = limit2;
                } else if vec_elem == limit2 + 1 as libc::c_int {
                    limit2 = vec_elem;
                } else {
                    if limit2 == limit1 {
                        sprintf(
                            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
                            b"%d%s\0" as *const u8 as *const libc::c_char,
                            limit1,
                            b":\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        sprintf(
                            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
                            b"%d%s%d%s\0" as *const u8 as *const libc::c_char,
                            limit1,
                            b"..\0" as *const u8 as *const libc::c_char,
                            limit2,
                            b":\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    limit2 = vec_elem;
                    limit1 = limit2;
                }
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
    if limit1 != -(1 as libc::c_int) {
        if limit2 == limit1 {
            sprintf(
                buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
                b"%d%s\0" as *const u8 as *const libc::c_char,
                limit1,
                b"\0" as *const u8 as *const libc::c_char,
            );
        } else {
            sprintf(
                buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
                b"%d%s%d%s\0" as *const u8 as *const libc::c_char,
                limit1,
                b"..\0" as *const u8 as *const libc::c_char,
                limit2,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (*range).extra_cstr != 0 {
        sprintf(
            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
            b"%s\0" as *const u8 as *const libc::c_char,
            b"@\0" as *const u8 as *const libc::c_char,
        );
    }
    return buff.as_mut_ptr();
}
