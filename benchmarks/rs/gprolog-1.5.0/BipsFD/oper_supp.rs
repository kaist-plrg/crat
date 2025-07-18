use ::libc;
extern "C" {
    static mut pl_vec_size: WamWord;
    static mut pl_vec_max_integer: WamWord;
    fn Pl_Range_Copy(range: *mut Range, range1: *mut Range);
    fn Pl_Range_Div_Range(range: *mut Range, range1: *mut Range);
    fn Pl_Vector_Empty(vec: Vector);
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
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Power(
    mut x: libc::c_uint,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut xn: libc::c_uint = 0;
    let mut xp: libc::c_uint = 0;
    if n == 0 as libc::c_int as libc::c_uint || x == 1 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as libc::c_uint;
    }
    if x == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    if n as libc::c_ulong
        >= (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        return (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_uint;
    }
    xn = 1 as libc::c_int as libc::c_uint;
    xp = x;
    while n != 0 {
        if n & 1 as libc::c_int as libc::c_uint != 0 {
            xn = xn.wrapping_mul(xp);
        }
        xp = xp.wrapping_mul(xp);
        n >>= 1 as libc::c_int;
    }
    return if xn as PlLong > 0 as libc::c_int as libc::c_long
        && xn as PlLong
            <= (((1 as libc::c_int as PlLong)
                << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
                - 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
    {
        xn
    } else {
        (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_uint
    };
}
pub unsafe extern "C" fn Pl_Nth_Root_Dn(
    mut y: libc::c_uint,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut old: libc::c_uint = 0;
    let mut new: libc::c_uint = 0;
    let mut n1: libc::c_uint = n.wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut oldn1: libc::c_uint = 0;
    let mut bit: libc::c_int = 0;
    let mut nb: libc::c_int = 0;
    if y == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    if n == 0 as libc::c_int as libc::c_uint {
        return (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_uint;
    }
    if n as libc::c_ulong
        >= (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
    {
        return 1 as libc::c_int as libc::c_uint;
    }
    bit = 8 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int
        - (y as libc::c_ulong).leading_zeros() as i32;
    if ((bit + 1 as libc::c_int) as libc::c_uint) < n {
        return 1 as libc::c_int as libc::c_uint;
    }
    nb = (bit as libc::c_uint).wrapping_div(n) as libc::c_int;
    new = ((1 as libc::c_int) << nb) as libc::c_uint;
    old = new;
    oldn1 = Pl_Power(old, n1);
    new = n1.wrapping_mul(old).wrapping_add(y.wrapping_div(oldn1)).wrapping_div(n);
    loop {
        old = new;
        oldn1 = Pl_Power(old, n1);
        new = n1.wrapping_mul(old).wrapping_add(y.wrapping_div(oldn1)).wrapping_div(n);
        if !(new < old) {
            break;
        }
    }
    return old;
}
pub unsafe extern "C" fn Pl_Nth_Root_Up(
    mut y: libc::c_uint,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut x: libc::c_uint = 0;
    if y == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    if n == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    x = Pl_Nth_Root_Dn(y, n);
    if Pl_Power(x, n) != y {
        x = x.wrapping_add(1);
        x;
    }
    return x;
}
pub unsafe extern "C" fn Pl_Nth_Root_Exact(
    mut y: libc::c_uint,
    mut n: libc::c_uint,
) -> libc::c_uint {
    let mut x: libc::c_uint = 0;
    if y == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    x = Pl_Nth_Root_Dn(y, n);
    if Pl_Power(x, n) != y {
        return -(1 as libc::c_int) as libc::c_uint;
    }
    return x;
}
pub unsafe extern "C" fn Pl_Sqrt_Dn(mut y: libc::c_uint) -> libc::c_uint {
    let mut old: libc::c_uint = 0;
    let mut new: libc::c_uint = 0;
    if y == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    new = y;
    loop {
        old = new;
        new = old.wrapping_add(y.wrapping_div(old)) >> 1 as libc::c_int;
        if !(new < old) {
            break;
        }
    }
    return old;
}
pub unsafe extern "C" fn Pl_Sqrt_Up(mut y: libc::c_uint) -> libc::c_uint {
    let mut x: libc::c_uint = 0;
    x = Pl_Sqrt_Dn(y);
    if x.wrapping_mul(x) != y {
        x = x.wrapping_add(1);
        x;
    }
    return x;
}
pub unsafe extern "C" fn Pl_Sqrt_Exact(mut y: libc::c_uint) -> libc::c_uint {
    let mut x: libc::c_uint = 0;
    x = Pl_Sqrt_Dn(y);
    if x.wrapping_mul(x) != y {
        return -(1 as libc::c_int) as libc::c_uint;
    }
    return x;
}
pub unsafe extern "C" fn Pl_Find_Expon_Dn(
    mut x: libc::c_uint,
    mut y: libc::c_uint,
) -> libc::c_uint {
    let mut n: libc::c_uint = 0;
    let mut xn: libc::c_uint = 0;
    if x <= 1 as libc::c_int as libc::c_uint || y == 0 as libc::c_int as libc::c_uint {
        return (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_uint;
    }
    n = Find_Expon_General(x, y, &mut xn);
    return n;
}
pub unsafe extern "C" fn Pl_Find_Expon_Up(
    mut x: libc::c_uint,
    mut y: libc::c_uint,
) -> libc::c_uint {
    let mut n: libc::c_uint = 0;
    let mut xn: libc::c_uint = 0;
    if x <= 1 as libc::c_int as libc::c_uint || y == 0 as libc::c_int as libc::c_uint {
        return (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_uint;
    }
    n = Find_Expon_General(x, y, &mut xn);
    return n.wrapping_add((y != xn) as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn Pl_Find_Expon_Exact(
    mut x: libc::c_uint,
    mut y: libc::c_uint,
) -> libc::c_uint {
    let mut n: libc::c_uint = 0;
    let mut xn: libc::c_uint = 0;
    if x <= 1 as libc::c_int as libc::c_uint || y == 0 as libc::c_int as libc::c_uint {
        return (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_uint;
    }
    n = Find_Expon_General(x, y, &mut xn);
    if y != xn {
        return -(1 as libc::c_int) as libc::c_uint;
    }
    return n;
}
unsafe extern "C" fn Find_Expon_General(
    mut x: libc::c_uint,
    mut y: libc::c_uint,
    mut pxn: *mut libc::c_uint,
) -> libc::c_uint {
    static mut txp: [libc::c_uint; 32] = [0; 32];
    let mut p: *mut libc::c_uint = txp.as_mut_ptr();
    let mut xp: libc::c_uint = 0;
    let mut prod: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut xn: libc::c_uint = 0;
    p = txp.as_mut_ptr();
    xp = x;
    prod = 1 as libc::c_int as libc::c_uint;
    while prod < y && xp as PlLong > 0 as libc::c_int as libc::c_long {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = xp;
        prod = prod.wrapping_mul(xp);
        xp = xp.wrapping_mul(xp);
    }
    n = 0 as libc::c_int as libc::c_uint;
    xn = 1 as libc::c_int as libc::c_uint;
    loop {
        p = p.offset(-1);
        if !(p >= txp.as_mut_ptr()) {
            break;
        }
        xp = *p;
        n <<= 1 as libc::c_int;
        if y >= xp {
            y = y.wrapping_div(xp);
            xn = xn.wrapping_mul(xp);
            n |= 1 as libc::c_int as libc::c_uint;
        }
    }
    *pxn = xn;
    return n;
}
pub unsafe extern "C" fn Pl_Full_Coeff_Power_Var(
    mut y: *mut Range,
    mut a: libc::c_int,
    mut n: *mut Range,
) {
    let mut an: libc::c_uint = 0;
    let mut an0: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut vec_elem: libc::c_int = 0;
    an = Pl_Power(a as libc::c_uint, (*n).min as libc::c_uint);
    (*y)
        .vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh1 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh1 = (*fresh1).offset(pl_vec_size as isize);
    if an > pl_vec_max_integer as libc::c_uint {
        (*y).extra_cstr = 1 as libc::c_int;
        (*y)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    Pl_Vector_Empty((*y).vec);
    (*y).extra_cstr = 0 as libc::c_int;
    (*y).min = an as libc::c_int;
    if ((*n).vec).is_null() {
        an0 = an;
        i = (*n).min;
        while i <= (*n).max {
            if an0 > pl_vec_max_integer as libc::c_uint {
                break;
            }
            an = an0;
            let ref mut fresh2 = *((*y).vec)
                .offset((an as VecWord >> 6 as libc::c_int) as isize);
            *fresh2
                |= (1 as libc::c_int as VecWord)
                    << (an as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            an0 = an0.wrapping_mul(a as libc::c_uint);
            i += 1;
            i;
        }
    } else {
        (*y).extra_cstr = (*n).extra_cstr;
        let mut enum_end: Vector = ((*n).vec).offset(pl_vec_size as isize);
        let mut enum_i: Vector = (*n).vec;
        let mut enum_j: libc::c_int = 0;
        let mut enum_word: VecWord = 0;
        vec_elem = 0 as libc::c_int;
        's_102: loop {
            enum_word = *enum_i;
            enum_j = 0 as libc::c_int;
            loop {
                let fresh3 = enum_j;
                enum_j = enum_j + 1;
                if !(fresh3 < 8 as libc::c_int * 8 as libc::c_int) {
                    break;
                }
                if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                    an = Pl_Power(a as libc::c_uint, vec_elem as libc::c_uint);
                    if an > pl_vec_max_integer as libc::c_uint {
                        break 's_102;
                    }
                    let ref mut fresh4 = *((*y).vec)
                        .offset((an as VecWord >> 6 as libc::c_int) as isize);
                    *fresh4
                        |= (1 as libc::c_int as VecWord)
                            << (an as libc::c_ulong
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
        }
    }
    (*y).max = an as libc::c_int;
}
pub unsafe extern "C" fn Pl_Full_Find_Expon(
    mut n: *mut Range,
    mut a: libc::c_int,
    mut y: *mut Range,
) {
    let mut e: libc::c_int = 0 as libc::c_int;
    let mut min: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut vec_elem: libc::c_int = 0;
    (*n)
        .vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh5 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh5 = (*fresh5).offset(pl_vec_size as isize);
    Pl_Vector_Empty((*n).vec);
    (*n).extra_cstr = (*y).extra_cstr;
    min = -(1 as libc::c_int);
    if ((*y).vec).is_null() {
        i = (*y).min;
        while i <= (*y).max {
            e = Pl_Find_Expon_Exact(a as libc::c_uint, i as libc::c_uint) as libc::c_int;
            if e >= 0 as libc::c_int {
                if min < 0 as libc::c_int {
                    min = e;
                }
                let ref mut fresh6 = *((*n).vec)
                    .offset((e as VecWord >> 6 as libc::c_int) as isize);
                *fresh6
                    |= (1 as libc::c_int as VecWord)
                        << (e as libc::c_ulong
                            & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            }
            i += 1;
            i;
        }
    } else {
        let mut enum_end: Vector = ((*y).vec).offset(pl_vec_size as isize);
        let mut enum_i: Vector = (*y).vec;
        let mut enum_j: libc::c_int = 0;
        let mut enum_word: VecWord = 0;
        vec_elem = 0 as libc::c_int;
        loop {
            enum_word = *enum_i;
            enum_j = 0 as libc::c_int;
            loop {
                let fresh7 = enum_j;
                enum_j = enum_j + 1;
                if !(fresh7 < 8 as libc::c_int * 8 as libc::c_int) {
                    break;
                }
                if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                    e = Pl_Find_Expon_Exact(a as libc::c_uint, vec_elem as libc::c_uint)
                        as libc::c_int;
                    if e >= 0 as libc::c_int {
                        if min < 0 as libc::c_int {
                            min = e;
                        }
                        let ref mut fresh8 = *((*n).vec)
                            .offset((e as VecWord >> 6 as libc::c_int) as isize);
                        *fresh8
                            |= (1 as libc::c_int as VecWord)
                                << (e as libc::c_ulong
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
        }
    }
    (*n).min = min;
    (*n).max = e;
}
pub unsafe extern "C" fn Pl_Full_Var_Power_Coeff(
    mut y: *mut Range,
    mut x: *mut Range,
    mut a: libc::c_int,
) {
    let mut xa: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut vec_elem: libc::c_int = 0;
    xa = Pl_Power((*x).min as libc::c_uint, a as libc::c_uint);
    (*y)
        .vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh9 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh9 = (*fresh9).offset(pl_vec_size as isize);
    if xa > pl_vec_max_integer as libc::c_uint {
        (*y).extra_cstr = 1 as libc::c_int;
        (*y)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    Pl_Vector_Empty((*y).vec);
    (*y).extra_cstr = 0 as libc::c_int;
    (*y).min = xa as libc::c_int;
    if ((*x).vec).is_null() {
        i = (*x).min;
        while i <= (*x).max {
            xa = Pl_Power(i as libc::c_uint, a as libc::c_uint);
            if xa > pl_vec_max_integer as libc::c_uint {
                break;
            }
            let ref mut fresh10 = *((*y).vec)
                .offset((xa as VecWord >> 6 as libc::c_int) as isize);
            *fresh10
                |= (1 as libc::c_int as VecWord)
                    << (xa as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            i += 1;
            i;
        }
    } else {
        (*y).extra_cstr = (*x).extra_cstr;
        let mut enum_end: Vector = ((*x).vec).offset(pl_vec_size as isize);
        let mut enum_i: Vector = (*x).vec;
        let mut enum_j: libc::c_int = 0;
        let mut enum_word: VecWord = 0;
        vec_elem = 0 as libc::c_int;
        's_96: loop {
            enum_word = *enum_i;
            enum_j = 0 as libc::c_int;
            loop {
                let fresh11 = enum_j;
                enum_j = enum_j + 1;
                if !(fresh11 < 8 as libc::c_int * 8 as libc::c_int) {
                    break;
                }
                if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                    xa = Pl_Power(vec_elem as libc::c_uint, a as libc::c_uint);
                    if xa > pl_vec_max_integer as libc::c_uint {
                        break 's_96;
                    }
                    let ref mut fresh12 = *((*y).vec)
                        .offset((xa as VecWord >> 6 as libc::c_int) as isize);
                    *fresh12
                        |= (1 as libc::c_int as VecWord)
                            << (xa as libc::c_ulong
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
        }
    }
    (*y).max = xa as libc::c_int;
}
pub unsafe extern "C" fn Pl_Full_Nth_Root(
    mut x: *mut Range,
    mut y: *mut Range,
    mut a: libc::c_int,
) {
    let mut e: libc::c_int = 0 as libc::c_int;
    let mut min: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut vec_elem: libc::c_int = 0;
    (*x)
        .vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh13 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh13 = (*fresh13).offset(pl_vec_size as isize);
    Pl_Vector_Empty((*x).vec);
    (*x).extra_cstr = (*y).extra_cstr;
    min = -(1 as libc::c_int);
    if ((*y).vec).is_null() {
        i = (*y).min;
        while i <= (*y).max {
            e = Pl_Nth_Root_Exact(i as libc::c_uint, a as libc::c_uint) as libc::c_int;
            if e >= 0 as libc::c_int {
                if min < 0 as libc::c_int {
                    min = e;
                }
                let ref mut fresh14 = *((*x).vec)
                    .offset((e as VecWord >> 6 as libc::c_int) as isize);
                *fresh14
                    |= (1 as libc::c_int as VecWord)
                        << (e as libc::c_ulong
                            & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            }
            i += 1;
            i;
        }
    } else {
        let mut enum_end: Vector = ((*y).vec).offset(pl_vec_size as isize);
        let mut enum_i: Vector = (*y).vec;
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
                    e = Pl_Nth_Root_Exact(vec_elem as libc::c_uint, a as libc::c_uint)
                        as libc::c_int;
                    if e >= 0 as libc::c_int {
                        if min < 0 as libc::c_int {
                            min = e;
                        }
                        let ref mut fresh16 = *((*x).vec)
                            .offset((e as VecWord >> 6 as libc::c_int) as isize);
                        *fresh16
                            |= (1 as libc::c_int as VecWord)
                                << (e as libc::c_ulong
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
        }
    }
    (*x).min = min;
    (*x).max = e;
}
pub unsafe extern "C" fn Pl_Full_Var_Power_2(mut y: *mut Range, mut x: *mut Range) {
    let mut x2: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut vec_elem: libc::c_int = 0;
    x2 = ((*x).min * (*x).min) as libc::c_uint;
    (*y)
        .vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh17 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh17 = (*fresh17).offset(pl_vec_size as isize);
    if x2 > pl_vec_max_integer as libc::c_uint {
        (*y).extra_cstr = 1 as libc::c_int;
        (*y)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return;
    }
    Pl_Vector_Empty((*y).vec);
    (*y).extra_cstr = 0 as libc::c_int;
    (*y).min = x2 as libc::c_int;
    if ((*x).vec).is_null() {
        i = (*x).min;
        while i <= (*x).max {
            x2 = (i * i) as libc::c_uint;
            if x2 > pl_vec_max_integer as libc::c_uint {
                break;
            }
            let ref mut fresh18 = *((*y).vec)
                .offset((x2 as VecWord >> 6 as libc::c_int) as isize);
            *fresh18
                |= (1 as libc::c_int as VecWord)
                    << (x2 as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            i += 1;
            i;
        }
    } else {
        (*y).extra_cstr = (*x).extra_cstr;
        let mut enum_end: Vector = ((*x).vec).offset(pl_vec_size as isize);
        let mut enum_i: Vector = (*x).vec;
        let mut enum_j: libc::c_int = 0;
        let mut enum_word: VecWord = 0;
        vec_elem = 0 as libc::c_int;
        's_96: loop {
            enum_word = *enum_i;
            enum_j = 0 as libc::c_int;
            loop {
                let fresh19 = enum_j;
                enum_j = enum_j + 1;
                if !(fresh19 < 8 as libc::c_int * 8 as libc::c_int) {
                    break;
                }
                if enum_word & 1 as libc::c_int as libc::c_ulong != 0 {
                    x2 = (vec_elem * vec_elem) as libc::c_uint;
                    if x2 > pl_vec_max_integer as libc::c_uint {
                        break 's_96;
                    }
                    let ref mut fresh20 = *((*y).vec)
                        .offset((x2 as VecWord >> 6 as libc::c_int) as isize);
                    *fresh20
                        |= (1 as libc::c_int as VecWord)
                            << (x2 as libc::c_ulong
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
        }
    }
    (*y).max = x2 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Full_Sqrt_Var(mut x: *mut Range, mut y: *mut Range) {
    let mut e: libc::c_int = 0 as libc::c_int;
    let mut min: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut vec_elem: libc::c_int = 0;
    (*x)
        .vec = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
    let ref mut fresh21 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh21 = (*fresh21).offset(pl_vec_size as isize);
    Pl_Vector_Empty((*x).vec);
    (*x).extra_cstr = (*y).extra_cstr;
    min = -(1 as libc::c_int);
    if ((*y).vec).is_null() {
        i = (*y).min;
        while i <= (*y).max {
            e = Pl_Sqrt_Exact(i as libc::c_uint) as libc::c_int;
            if e >= 0 as libc::c_int {
                if min < 0 as libc::c_int {
                    min = e;
                }
                let ref mut fresh22 = *((*x).vec)
                    .offset((e as VecWord >> 6 as libc::c_int) as isize);
                *fresh22
                    |= (1 as libc::c_int as VecWord)
                        << (e as libc::c_ulong
                            & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            }
            i += 1;
            i;
        }
    } else {
        let mut enum_end: Vector = ((*y).vec).offset(pl_vec_size as isize);
        let mut enum_i: Vector = (*y).vec;
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
                    e = Pl_Sqrt_Exact(vec_elem as libc::c_uint) as libc::c_int;
                    if e >= 0 as libc::c_int {
                        if min < 0 as libc::c_int {
                            min = e;
                        }
                        let ref mut fresh24 = *((*x).vec)
                            .offset((e as VecWord >> 6 as libc::c_int) as isize);
                        *fresh24
                            |= (1 as libc::c_int as VecWord)
                                << (e as libc::c_ulong
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
        }
    }
    (*x).min = min;
    (*x).max = e;
}
pub unsafe extern "C" fn Pl_Full_Var_Div_Var(
    mut x: *mut Range,
    mut z: *mut Range,
    mut y: *mut Range,
) {
    if (*y).min == 0 as libc::c_int {
        (*x).extra_cstr = 0 as libc::c_int;
        (*x).min = 0 as libc::c_int;
        (*x)
            .max = (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int;
        (*x).vec = 0 as Vector;
        return;
    }
    Pl_Range_Copy(x, z);
    Pl_Range_Div_Range(x, y);
}
