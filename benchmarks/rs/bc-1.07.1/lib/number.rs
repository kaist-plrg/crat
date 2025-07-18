use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn rt_warn(mesg: *const libc::c_char, _: ...);
    fn rt_error(mesg: *const libc::c_char, _: ...);
    fn out_of_memory();
}
pub type sign = libc::c_uint;
pub const MINUS: sign = 1;
pub const PLUS: sign = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_struct {
    pub n_sign: sign,
    pub n_len: libc::c_int,
    pub n_scale: libc::c_int,
    pub n_refs: libc::c_int,
    pub n_next: bc_num,
    pub n_ptr: *mut libc::c_char,
    pub n_value: *mut libc::c_char,
}
pub type bc_num = *mut bc_struct;
pub const _ISdigit: C2RustUnnamed = 2048;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stk_rec {
    pub digit: libc::c_long,
    pub next: *mut stk_rec,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub static mut _zero_: bc_num = 0 as *const bc_struct as *mut bc_struct;
pub static mut _one_: bc_num = 0 as *const bc_struct as *mut bc_struct;
pub static mut _two_: bc_num = 0 as *const bc_struct as *mut bc_struct;
static mut _bc_Free_list: bc_num = 0 as *const bc_struct as bc_num;
pub unsafe extern "C" fn bc_new_num(
    mut length: libc::c_int,
    mut scale: libc::c_int,
) -> bc_num {
    let mut temp: bc_num = 0 as *mut bc_struct;
    if !_bc_Free_list.is_null() {
        temp = _bc_Free_list;
        _bc_Free_list = (*temp).n_next;
    } else {
        temp = malloc(::std::mem::size_of::<bc_struct>() as libc::c_ulong) as bc_num;
        if temp.is_null() {
            out_of_memory();
        }
    }
    (*temp).n_sign = PLUS;
    (*temp).n_len = length;
    (*temp).n_scale = scale;
    (*temp).n_refs = 1 as libc::c_int;
    (*temp).n_ptr = malloc((length + scale) as libc::c_ulong) as *mut libc::c_char;
    if ((*temp).n_ptr).is_null() {
        out_of_memory();
    }
    (*temp).n_value = (*temp).n_ptr;
    memset(
        (*temp).n_ptr as *mut libc::c_void,
        0 as libc::c_int,
        (length + scale) as libc::c_ulong,
    );
    return temp;
}
pub unsafe extern "C" fn bc_free_num(mut num: *mut bc_num) {
    if (*num).is_null() {
        return;
    }
    (**num).n_refs -= 1;
    (**num).n_refs;
    if (**num).n_refs == 0 as libc::c_int {
        if !((**num).n_ptr).is_null() {
            free((**num).n_ptr as *mut libc::c_void);
        }
        (**num).n_next = _bc_Free_list;
        _bc_Free_list = *num;
    }
    *num = 0 as bc_num;
}
pub unsafe extern "C" fn bc_init_numbers() {
    _zero_ = bc_new_num(1 as libc::c_int, 0 as libc::c_int);
    _one_ = bc_new_num(1 as libc::c_int, 0 as libc::c_int);
    *((*_one_).n_value)
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_char;
    _two_ = bc_new_num(1 as libc::c_int, 0 as libc::c_int);
    *((*_two_).n_value)
        .offset(0 as libc::c_int as isize) = 2 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn bc_copy_num(mut num: bc_num) -> bc_num {
    (*num).n_refs += 1;
    (*num).n_refs;
    return num;
}
pub unsafe extern "C" fn bc_init_num(mut num: *mut bc_num) {
    *num = bc_copy_num(_zero_);
}
unsafe extern "C" fn _bc_rm_leading_zeros(mut num: bc_num) {
    while *(*num).n_value as libc::c_int == 0 as libc::c_int
        && (*num).n_len > 1 as libc::c_int
    {
        (*num).n_value = ((*num).n_value).offset(1);
        (*num).n_value;
        (*num).n_len -= 1;
        (*num).n_len;
    }
}
unsafe extern "C" fn _bc_do_compare(
    mut n1: bc_num,
    mut n2: bc_num,
    mut use_sign: libc::c_int,
    mut ignore_last: libc::c_int,
) -> libc::c_int {
    let mut n1ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    if use_sign != 0 && (*n1).n_sign as libc::c_uint != (*n2).n_sign as libc::c_uint {
        if (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        } else {
            return -(1 as libc::c_int)
        }
    }
    if (*n1).n_len != (*n2).n_len {
        if (*n1).n_len > (*n2).n_len {
            if use_sign == 0
                || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int
            } else {
                return -(1 as libc::c_int)
            }
        } else if use_sign == 0
            || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int)
        } else {
            return 1 as libc::c_int
        }
    }
    count = (*n1).n_len
        + (if (*n1).n_scale > (*n2).n_scale { (*n2).n_scale } else { (*n1).n_scale });
    n1ptr = (*n1).n_value;
    n2ptr = (*n2).n_value;
    while count > 0 as libc::c_int && *n1ptr as libc::c_int == *n2ptr as libc::c_int {
        n1ptr = n1ptr.offset(1);
        n1ptr;
        n2ptr = n2ptr.offset(1);
        n2ptr;
        count -= 1;
        count;
    }
    if ignore_last != 0 && count == 1 as libc::c_int && (*n1).n_scale == (*n2).n_scale {
        return 0 as libc::c_int;
    }
    if count != 0 as libc::c_int {
        if *n1ptr as libc::c_int > *n2ptr as libc::c_int {
            if use_sign == 0
                || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int
            } else {
                return -(1 as libc::c_int)
            }
        } else if use_sign == 0
            || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int)
        } else {
            return 1 as libc::c_int
        }
    }
    if (*n1).n_scale != (*n2).n_scale {
        if (*n1).n_scale > (*n2).n_scale {
            count = (*n1).n_scale - (*n2).n_scale;
            while count > 0 as libc::c_int {
                let fresh0 = n1ptr;
                n1ptr = n1ptr.offset(1);
                if *fresh0 as libc::c_int != 0 as libc::c_int {
                    if use_sign == 0
                        || (*n1).n_sign as libc::c_uint
                            == PLUS as libc::c_int as libc::c_uint
                    {
                        return 1 as libc::c_int
                    } else {
                        return -(1 as libc::c_int)
                    }
                }
                count -= 1;
                count;
            }
        } else {
            count = (*n2).n_scale - (*n1).n_scale;
            while count > 0 as libc::c_int {
                let fresh1 = n2ptr;
                n2ptr = n2ptr.offset(1);
                if *fresh1 as libc::c_int != 0 as libc::c_int {
                    if use_sign == 0
                        || (*n1).n_sign as libc::c_uint
                            == PLUS as libc::c_int as libc::c_uint
                    {
                        return -(1 as libc::c_int)
                    } else {
                        return 1 as libc::c_int
                    }
                }
                count -= 1;
                count;
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bc_compare(mut n1: bc_num, mut n2: bc_num) -> libc::c_int {
    return _bc_do_compare(n1, n2, 1 as libc::c_int, 0 as libc::c_int);
}
pub unsafe extern "C" fn bc_is_neg(mut num: bc_num) -> libc::c_char {
    return ((*num).n_sign as libc::c_uint == MINUS as libc::c_int as libc::c_uint)
        as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn bc_is_zero(mut num: bc_num) -> libc::c_char {
    let mut count: libc::c_int = 0;
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if num == _zero_ {
        return 1 as libc::c_int as libc::c_char;
    }
    count = (*num).n_len + (*num).n_scale;
    nptr = (*num).n_value;
    while count > 0 as libc::c_int
        && {
            let fresh2 = nptr;
            nptr = nptr.offset(1);
            *fresh2 as libc::c_int == 0 as libc::c_int
        }
    {
        count -= 1;
        count;
    }
    if count != 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_char
    } else {
        return 1 as libc::c_int as libc::c_char
    };
}
pub unsafe extern "C" fn bc_is_near_zero(
    mut num: bc_num,
    mut scale: libc::c_int,
) -> libc::c_char {
    let mut count: libc::c_int = 0;
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if scale > (*num).n_scale {
        scale = (*num).n_scale;
    }
    count = (*num).n_len + scale;
    nptr = (*num).n_value;
    while count > 0 as libc::c_int
        && {
            let fresh3 = nptr;
            nptr = nptr.offset(1);
            *fresh3 as libc::c_int == 0 as libc::c_int
        }
    {
        count -= 1;
        count;
    }
    if count != 0 as libc::c_int
        && (count != 1 as libc::c_int
            || {
                nptr = nptr.offset(-1);
                *nptr as libc::c_int != 1 as libc::c_int
            })
    {
        return 0 as libc::c_int as libc::c_char
    } else {
        return 1 as libc::c_int as libc::c_char
    };
}
unsafe extern "C" fn _bc_do_add(
    mut n1: bc_num,
    mut n2: bc_num,
    mut scale_min: libc::c_int,
) -> bc_num {
    let mut sum: bc_num = 0 as *mut bc_struct;
    let mut sum_scale: libc::c_int = 0;
    let mut sum_digits: libc::c_int = 0;
    let mut n1ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sumptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut carry: libc::c_int = 0;
    let mut n1bytes: libc::c_int = 0;
    let mut n2bytes: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    sum_scale = if (*n1).n_scale > (*n2).n_scale {
        (*n1).n_scale
    } else {
        (*n2).n_scale
    };
    sum_digits = (if (*n1).n_len > (*n2).n_len { (*n1).n_len } else { (*n2).n_len })
        + 1 as libc::c_int;
    sum = bc_new_num(
        sum_digits,
        if sum_scale > scale_min { sum_scale } else { scale_min },
    );
    if scale_min > sum_scale {
        sumptr = ((*sum).n_value).offset(sum_scale as isize).offset(sum_digits as isize);
        count = scale_min - sum_scale;
        while count > 0 as libc::c_int {
            let fresh4 = sumptr;
            sumptr = sumptr.offset(1);
            *fresh4 = 0 as libc::c_int as libc::c_char;
            count -= 1;
            count;
        }
    }
    n1bytes = (*n1).n_scale;
    n2bytes = (*n2).n_scale;
    n1ptr = ((*n1).n_value)
        .offset((*n1).n_len as isize)
        .offset(n1bytes as isize)
        .offset(-(1 as libc::c_int as isize));
    n2ptr = ((*n2).n_value)
        .offset((*n2).n_len as isize)
        .offset(n2bytes as isize)
        .offset(-(1 as libc::c_int as isize));
    sumptr = ((*sum).n_value)
        .offset(sum_scale as isize)
        .offset(sum_digits as isize)
        .offset(-(1 as libc::c_int as isize));
    if n1bytes != n2bytes {
        if n1bytes > n2bytes {
            while n1bytes > n2bytes {
                let fresh5 = n1ptr;
                n1ptr = n1ptr.offset(-1);
                let fresh6 = sumptr;
                sumptr = sumptr.offset(-1);
                *fresh6 = *fresh5;
                n1bytes -= 1;
                n1bytes;
            }
        } else {
            while n2bytes > n1bytes {
                let fresh7 = n2ptr;
                n2ptr = n2ptr.offset(-1);
                let fresh8 = sumptr;
                sumptr = sumptr.offset(-1);
                *fresh8 = *fresh7;
                n2bytes -= 1;
                n2bytes;
            }
        }
    }
    n1bytes += (*n1).n_len;
    n2bytes += (*n2).n_len;
    carry = 0 as libc::c_int;
    while n1bytes > 0 as libc::c_int && n2bytes > 0 as libc::c_int {
        let fresh9 = n1ptr;
        n1ptr = n1ptr.offset(-1);
        let fresh10 = n2ptr;
        n2ptr = n2ptr.offset(-1);
        *sumptr = (*fresh9 as libc::c_int + *fresh10 as libc::c_int + carry)
            as libc::c_char;
        if *sumptr as libc::c_int > 10 as libc::c_int - 1 as libc::c_int {
            carry = 1 as libc::c_int;
            *sumptr = (*sumptr as libc::c_int - 10 as libc::c_int) as libc::c_char;
        } else {
            carry = 0 as libc::c_int;
        }
        sumptr = sumptr.offset(-1);
        sumptr;
        n1bytes -= 1;
        n1bytes;
        n2bytes -= 1;
        n2bytes;
    }
    if n1bytes == 0 as libc::c_int {
        n1bytes = n2bytes;
        n1ptr = n2ptr;
    }
    loop {
        let fresh11 = n1bytes;
        n1bytes = n1bytes - 1;
        if !(fresh11 > 0 as libc::c_int) {
            break;
        }
        let fresh12 = n1ptr;
        n1ptr = n1ptr.offset(-1);
        *sumptr = (*fresh12 as libc::c_int + carry) as libc::c_char;
        if *sumptr as libc::c_int > 10 as libc::c_int - 1 as libc::c_int {
            carry = 1 as libc::c_int;
            *sumptr = (*sumptr as libc::c_int - 10 as libc::c_int) as libc::c_char;
        } else {
            carry = 0 as libc::c_int;
        }
        sumptr = sumptr.offset(-1);
        sumptr;
    }
    if carry == 1 as libc::c_int {
        *sumptr = (*sumptr as libc::c_int + 1 as libc::c_int) as libc::c_char;
    }
    _bc_rm_leading_zeros(sum);
    return sum;
}
unsafe extern "C" fn _bc_do_sub(
    mut n1: bc_num,
    mut n2: bc_num,
    mut scale_min: libc::c_int,
) -> bc_num {
    let mut diff: bc_num = 0 as *mut bc_struct;
    let mut diff_scale: libc::c_int = 0;
    let mut diff_len: libc::c_int = 0;
    let mut min_scale: libc::c_int = 0;
    let mut min_len: libc::c_int = 0;
    let mut n1ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut diffptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut borrow: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    diff_len = if (*n1).n_len > (*n2).n_len { (*n1).n_len } else { (*n2).n_len };
    diff_scale = if (*n1).n_scale > (*n2).n_scale {
        (*n1).n_scale
    } else {
        (*n2).n_scale
    };
    min_len = if (*n1).n_len > (*n2).n_len { (*n2).n_len } else { (*n1).n_len };
    min_scale = if (*n1).n_scale > (*n2).n_scale {
        (*n2).n_scale
    } else {
        (*n1).n_scale
    };
    diff = bc_new_num(
        diff_len,
        if diff_scale > scale_min { diff_scale } else { scale_min },
    );
    if scale_min > diff_scale {
        diffptr = ((*diff).n_value)
            .offset(diff_len as isize)
            .offset(diff_scale as isize);
        count = scale_min - diff_scale;
        while count > 0 as libc::c_int {
            let fresh13 = diffptr;
            diffptr = diffptr.offset(1);
            *fresh13 = 0 as libc::c_int as libc::c_char;
            count -= 1;
            count;
        }
    }
    n1ptr = ((*n1).n_value)
        .offset((*n1).n_len as isize)
        .offset((*n1).n_scale as isize)
        .offset(-(1 as libc::c_int as isize));
    n2ptr = ((*n2).n_value)
        .offset((*n2).n_len as isize)
        .offset((*n2).n_scale as isize)
        .offset(-(1 as libc::c_int as isize));
    diffptr = ((*diff).n_value)
        .offset(diff_len as isize)
        .offset(diff_scale as isize)
        .offset(-(1 as libc::c_int as isize));
    borrow = 0 as libc::c_int;
    if (*n1).n_scale != min_scale {
        count = (*n1).n_scale - min_scale;
        while count > 0 as libc::c_int {
            let fresh14 = n1ptr;
            n1ptr = n1ptr.offset(-1);
            let fresh15 = diffptr;
            diffptr = diffptr.offset(-1);
            *fresh15 = *fresh14;
            count -= 1;
            count;
        }
    } else {
        count = (*n2).n_scale - min_scale;
        while count > 0 as libc::c_int {
            let fresh16 = n2ptr;
            n2ptr = n2ptr.offset(-1);
            val = -(*fresh16 as libc::c_int) - borrow;
            if val < 0 as libc::c_int {
                val += 10 as libc::c_int;
                borrow = 1 as libc::c_int;
            } else {
                borrow = 0 as libc::c_int;
            }
            let fresh17 = diffptr;
            diffptr = diffptr.offset(-1);
            *fresh17 = val as libc::c_char;
            count -= 1;
            count;
        }
    }
    count = 0 as libc::c_int;
    while count < min_len + min_scale {
        let fresh18 = n1ptr;
        n1ptr = n1ptr.offset(-1);
        let fresh19 = n2ptr;
        n2ptr = n2ptr.offset(-1);
        val = *fresh18 as libc::c_int - *fresh19 as libc::c_int - borrow;
        if val < 0 as libc::c_int {
            val += 10 as libc::c_int;
            borrow = 1 as libc::c_int;
        } else {
            borrow = 0 as libc::c_int;
        }
        let fresh20 = diffptr;
        diffptr = diffptr.offset(-1);
        *fresh20 = val as libc::c_char;
        count += 1;
        count;
    }
    if diff_len != min_len {
        count = diff_len - min_len;
        while count > 0 as libc::c_int {
            let fresh21 = n1ptr;
            n1ptr = n1ptr.offset(-1);
            val = *fresh21 as libc::c_int - borrow;
            if val < 0 as libc::c_int {
                val += 10 as libc::c_int;
                borrow = 1 as libc::c_int;
            } else {
                borrow = 0 as libc::c_int;
            }
            let fresh22 = diffptr;
            diffptr = diffptr.offset(-1);
            *fresh22 = val as libc::c_char;
            count -= 1;
            count;
        }
    }
    _bc_rm_leading_zeros(diff);
    return diff;
}
pub unsafe extern "C" fn bc_sub(
    mut n1: bc_num,
    mut n2: bc_num,
    mut result: *mut bc_num,
    mut scale_min: libc::c_int,
) {
    let mut diff: bc_num = 0 as bc_num;
    let mut cmp_res: libc::c_int = 0;
    let mut res_scale: libc::c_int = 0;
    if (*n1).n_sign as libc::c_uint != (*n2).n_sign as libc::c_uint {
        diff = _bc_do_add(n1, n2, scale_min);
        (*diff).n_sign = (*n1).n_sign;
    } else {
        cmp_res = _bc_do_compare(n1, n2, 0 as libc::c_int, 0 as libc::c_int);
        match cmp_res {
            -1 => {
                diff = _bc_do_sub(n2, n1, scale_min);
                (*diff)
                    .n_sign = (if (*n2).n_sign as libc::c_uint
                    == PLUS as libc::c_int as libc::c_uint
                {
                    MINUS as libc::c_int
                } else {
                    PLUS as libc::c_int
                }) as sign;
            }
            0 => {
                res_scale = if scale_min
                    > (if (*n1).n_scale > (*n2).n_scale {
                        (*n1).n_scale
                    } else {
                        (*n2).n_scale
                    })
                {
                    scale_min
                } else if (*n1).n_scale > (*n2).n_scale {
                    (*n1).n_scale
                } else {
                    (*n2).n_scale
                };
                diff = bc_new_num(1 as libc::c_int, res_scale);
                memset(
                    (*diff).n_value as *mut libc::c_void,
                    0 as libc::c_int,
                    (res_scale + 1 as libc::c_int) as libc::c_ulong,
                );
            }
            1 => {
                diff = _bc_do_sub(n1, n2, scale_min);
                (*diff).n_sign = (*n1).n_sign;
            }
            _ => {}
        }
    }
    bc_free_num(result);
    *result = diff;
}
pub unsafe extern "C" fn bc_add(
    mut n1: bc_num,
    mut n2: bc_num,
    mut result: *mut bc_num,
    mut scale_min: libc::c_int,
) {
    let mut sum: bc_num = 0 as bc_num;
    let mut cmp_res: libc::c_int = 0;
    let mut res_scale: libc::c_int = 0;
    if (*n1).n_sign as libc::c_uint == (*n2).n_sign as libc::c_uint {
        sum = _bc_do_add(n1, n2, scale_min);
        (*sum).n_sign = (*n1).n_sign;
    } else {
        cmp_res = _bc_do_compare(n1, n2, 0 as libc::c_int, 0 as libc::c_int);
        match cmp_res {
            -1 => {
                sum = _bc_do_sub(n2, n1, scale_min);
                (*sum).n_sign = (*n2).n_sign;
            }
            0 => {
                res_scale = if scale_min
                    > (if (*n1).n_scale > (*n2).n_scale {
                        (*n1).n_scale
                    } else {
                        (*n2).n_scale
                    })
                {
                    scale_min
                } else if (*n1).n_scale > (*n2).n_scale {
                    (*n1).n_scale
                } else {
                    (*n2).n_scale
                };
                sum = bc_new_num(1 as libc::c_int, res_scale);
                memset(
                    (*sum).n_value as *mut libc::c_void,
                    0 as libc::c_int,
                    (res_scale + 1 as libc::c_int) as libc::c_ulong,
                );
            }
            1 => {
                sum = _bc_do_sub(n1, n2, scale_min);
                (*sum).n_sign = (*n1).n_sign;
            }
            _ => {}
        }
    }
    bc_free_num(result);
    *result = sum;
}
pub static mut mul_base_digits: libc::c_int = 80 as libc::c_int;
unsafe extern "C" fn new_sub_num(
    mut length: libc::c_int,
    mut scale: libc::c_int,
    mut value: *mut libc::c_char,
) -> bc_num {
    let mut temp: bc_num = 0 as *mut bc_struct;
    if !_bc_Free_list.is_null() {
        temp = _bc_Free_list;
        _bc_Free_list = (*temp).n_next;
    } else {
        temp = malloc(::std::mem::size_of::<bc_struct>() as libc::c_ulong) as bc_num;
        if temp.is_null() {
            out_of_memory();
        }
    }
    (*temp).n_sign = PLUS;
    (*temp).n_len = length;
    (*temp).n_scale = scale;
    (*temp).n_refs = 1 as libc::c_int;
    (*temp).n_ptr = 0 as *mut libc::c_char;
    (*temp).n_value = value;
    return temp;
}
unsafe extern "C" fn _bc_simp_mul(
    mut n1: bc_num,
    mut n1len: libc::c_int,
    mut n2: bc_num,
    mut n2len: libc::c_int,
    mut prod: *mut bc_num,
) {
    let mut n1ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pvptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n1end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut indx: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut prodlen: libc::c_int = 0;
    prodlen = n1len + n2len + 1 as libc::c_int;
    *prod = bc_new_num(prodlen, 0 as libc::c_int);
    n1end = ((*n1).n_value).offset(n1len as isize).offset(-(1 as libc::c_int as isize));
    n2end = ((*n2).n_value).offset(n2len as isize).offset(-(1 as libc::c_int as isize));
    pvptr = ((**prod).n_value)
        .offset(prodlen as isize)
        .offset(-(1 as libc::c_int as isize));
    sum = 0 as libc::c_int;
    indx = 0 as libc::c_int;
    while indx < prodlen - 1 as libc::c_int {
        n1ptr = n1end
            .offset(
                -((if 0 as libc::c_int > indx - n2len + 1 as libc::c_int {
                    0 as libc::c_int
                } else {
                    indx - n2len + 1 as libc::c_int
                }) as isize),
            );
        n2ptr = n2end
            .offset(
                -((if indx > n2len - 1 as libc::c_int {
                    n2len - 1 as libc::c_int
                } else {
                    indx
                }) as isize),
            );
        while n1ptr >= (*n1).n_value && n2ptr <= n2end {
            let fresh23 = n1ptr;
            n1ptr = n1ptr.offset(-1);
            let fresh24 = n2ptr;
            n2ptr = n2ptr.offset(1);
            sum += *fresh23 as libc::c_int * *fresh24 as libc::c_int;
        }
        let fresh25 = pvptr;
        pvptr = pvptr.offset(-1);
        *fresh25 = (sum % 10 as libc::c_int) as libc::c_char;
        sum = sum / 10 as libc::c_int;
        indx += 1;
        indx;
    }
    *pvptr = sum as libc::c_char;
}
unsafe extern "C" fn _bc_shift_addsub(
    mut accum: bc_num,
    mut val: bc_num,
    mut shift: libc::c_int,
    mut sub: libc::c_int,
) {
    let mut accp: *mut libc::c_schar = 0 as *mut libc::c_schar;
    let mut valp: *mut libc::c_schar = 0 as *mut libc::c_schar;
    let mut count: libc::c_int = 0;
    let mut carry: libc::c_int = 0;
    count = (*val).n_len;
    if *((*val).n_value).offset(0 as libc::c_int as isize) as libc::c_int
        == 0 as libc::c_int
    {
        count -= 1;
        count;
    }
    if (*accum).n_len + (*accum).n_scale >= shift + count {} else {
        __assert_fail(
            b"accum->n_len+accum->n_scale >= shift+count\0" as *const u8
                as *const libc::c_char,
            b"number.c\0" as *const u8 as *const libc::c_char,
            682 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void _bc_shift_addsub(bc_num, bc_num, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_4411: {
        if (*accum).n_len + (*accum).n_scale >= shift + count {} else {
            __assert_fail(
                b"accum->n_len+accum->n_scale >= shift+count\0" as *const u8
                    as *const libc::c_char,
                b"number.c\0" as *const u8 as *const libc::c_char,
                682 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void _bc_shift_addsub(bc_num, bc_num, int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    accp = ((*accum).n_value)
        .offset((*accum).n_len as isize)
        .offset((*accum).n_scale as isize)
        .offset(-(shift as isize))
        .offset(-(1 as libc::c_int as isize)) as *mut libc::c_schar;
    valp = ((*val).n_value)
        .offset((*val).n_len as isize)
        .offset(-(1 as libc::c_int as isize)) as *mut libc::c_schar;
    carry = 0 as libc::c_int;
    if sub != 0 {
        loop {
            let fresh26 = count;
            count = count - 1;
            if !(fresh26 != 0) {
                break;
            }
            let fresh27 = valp;
            valp = valp.offset(-1);
            *accp = (*accp as libc::c_int - (*fresh27 as libc::c_int + carry))
                as libc::c_schar;
            if (*accp as libc::c_int) < 0 as libc::c_int {
                carry = 1 as libc::c_int;
                let fresh28 = accp;
                accp = accp.offset(-1);
                *fresh28 = (*fresh28 as libc::c_int + 10 as libc::c_int)
                    as libc::c_schar;
            } else {
                carry = 0 as libc::c_int;
                accp = accp.offset(-1);
                accp;
            }
        }
        while carry != 0 {
            *accp = (*accp as libc::c_int - carry) as libc::c_schar;
            if (*accp as libc::c_int) < 0 as libc::c_int {
                let fresh29 = accp;
                accp = accp.offset(-1);
                *fresh29 = (*fresh29 as libc::c_int + 10 as libc::c_int)
                    as libc::c_schar;
            } else {
                carry = 0 as libc::c_int;
            }
        }
    } else {
        loop {
            let fresh30 = count;
            count = count - 1;
            if !(fresh30 != 0) {
                break;
            }
            let fresh31 = valp;
            valp = valp.offset(-1);
            *accp = (*accp as libc::c_int + (*fresh31 as libc::c_int + carry))
                as libc::c_schar;
            if *accp as libc::c_int > 10 as libc::c_int - 1 as libc::c_int {
                carry = 1 as libc::c_int;
                let fresh32 = accp;
                accp = accp.offset(-1);
                *fresh32 = (*fresh32 as libc::c_int - 10 as libc::c_int)
                    as libc::c_schar;
            } else {
                carry = 0 as libc::c_int;
                accp = accp.offset(-1);
                accp;
            }
        }
        while carry != 0 {
            *accp = (*accp as libc::c_int + carry) as libc::c_schar;
            if *accp as libc::c_int > 10 as libc::c_int - 1 as libc::c_int {
                let fresh33 = accp;
                accp = accp.offset(-1);
                *fresh33 = (*fresh33 as libc::c_int - 10 as libc::c_int)
                    as libc::c_schar;
            } else {
                carry = 0 as libc::c_int;
            }
        }
    };
}
unsafe extern "C" fn _bc_rec_mul(
    mut u: bc_num,
    mut ulen: libc::c_int,
    mut v: bc_num,
    mut vlen: libc::c_int,
    mut prod: *mut bc_num,
) {
    let mut u0: bc_num = 0 as *mut bc_struct;
    let mut u1: bc_num = 0 as *mut bc_struct;
    let mut v0: bc_num = 0 as *mut bc_struct;
    let mut v1: bc_num = 0 as *mut bc_struct;
    let mut m1: bc_num = 0 as *mut bc_struct;
    let mut m2: bc_num = 0 as *mut bc_struct;
    let mut m3: bc_num = 0 as *mut bc_struct;
    let mut d1: bc_num = 0 as *mut bc_struct;
    let mut d2: bc_num = 0 as *mut bc_struct;
    let mut n: libc::c_int = 0;
    let mut prodlen: libc::c_int = 0;
    let mut m1zero: libc::c_int = 0;
    let mut d1len: libc::c_int = 0;
    let mut d2len: libc::c_int = 0;
    if ulen + vlen < mul_base_digits || ulen < mul_base_digits / 4 as libc::c_int
        || vlen < mul_base_digits / 4 as libc::c_int
    {
        _bc_simp_mul(u, ulen, v, vlen, prod);
        return;
    }
    n = ((if ulen > vlen { ulen } else { vlen }) + 1 as libc::c_int) / 2 as libc::c_int;
    if ulen < n {
        u1 = bc_copy_num(_zero_);
        u0 = new_sub_num(ulen, 0 as libc::c_int, (*u).n_value);
    } else {
        u1 = new_sub_num(ulen - n, 0 as libc::c_int, (*u).n_value);
        u0 = new_sub_num(
            n,
            0 as libc::c_int,
            ((*u).n_value).offset(ulen as isize).offset(-(n as isize)),
        );
    }
    if vlen < n {
        v1 = bc_copy_num(_zero_);
        v0 = new_sub_num(vlen, 0 as libc::c_int, (*v).n_value);
    } else {
        v1 = new_sub_num(vlen - n, 0 as libc::c_int, (*v).n_value);
        v0 = new_sub_num(
            n,
            0 as libc::c_int,
            ((*v).n_value).offset(vlen as isize).offset(-(n as isize)),
        );
    }
    _bc_rm_leading_zeros(u1);
    _bc_rm_leading_zeros(u0);
    _bc_rm_leading_zeros(v1);
    _bc_rm_leading_zeros(v0);
    m1zero = (bc_is_zero(u1) as libc::c_int != 0 || bc_is_zero(v1) as libc::c_int != 0)
        as libc::c_int;
    bc_init_num(&mut d1);
    bc_init_num(&mut d2);
    bc_sub(u1, u0, &mut d1, 0 as libc::c_int);
    d1len = (*d1).n_len;
    bc_sub(v0, v1, &mut d2, 0 as libc::c_int);
    d2len = (*d2).n_len;
    if m1zero != 0 {
        m1 = bc_copy_num(_zero_);
    } else {
        _bc_rec_mul(u1, (*u1).n_len, v1, (*v1).n_len, &mut m1);
    }
    if bc_is_zero(d1) as libc::c_int != 0 || bc_is_zero(d2) as libc::c_int != 0 {
        m2 = bc_copy_num(_zero_);
    } else {
        _bc_rec_mul(d1, d1len, d2, d2len, &mut m2);
    }
    if bc_is_zero(u0) as libc::c_int != 0 || bc_is_zero(v0) as libc::c_int != 0 {
        m3 = bc_copy_num(_zero_);
    } else {
        _bc_rec_mul(u0, (*u0).n_len, v0, (*v0).n_len, &mut m3);
    }
    prodlen = ulen + vlen + 1 as libc::c_int;
    *prod = bc_new_num(prodlen, 0 as libc::c_int);
    if m1zero == 0 {
        _bc_shift_addsub(*prod, m1, 2 as libc::c_int * n, 0 as libc::c_int);
        _bc_shift_addsub(*prod, m1, n, 0 as libc::c_int);
    }
    _bc_shift_addsub(*prod, m3, n, 0 as libc::c_int);
    _bc_shift_addsub(*prod, m3, 0 as libc::c_int, 0 as libc::c_int);
    _bc_shift_addsub(
        *prod,
        m2,
        n,
        ((*d1).n_sign as libc::c_uint != (*d2).n_sign as libc::c_uint) as libc::c_int,
    );
    bc_free_num(&mut u1);
    bc_free_num(&mut u0);
    bc_free_num(&mut v1);
    bc_free_num(&mut m1);
    bc_free_num(&mut v0);
    bc_free_num(&mut m2);
    bc_free_num(&mut m3);
    bc_free_num(&mut d1);
    bc_free_num(&mut d2);
}
pub unsafe extern "C" fn bc_multiply(
    mut n1: bc_num,
    mut n2: bc_num,
    mut prod: *mut bc_num,
    mut scale: libc::c_int,
) {
    let mut pval: bc_num = 0 as *mut bc_struct;
    let mut len1: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    let mut full_scale: libc::c_int = 0;
    let mut prod_scale: libc::c_int = 0;
    len1 = (*n1).n_len + (*n1).n_scale;
    len2 = (*n2).n_len + (*n2).n_scale;
    full_scale = (*n1).n_scale + (*n2).n_scale;
    prod_scale = if full_scale
        > (if scale
            > (if (*n1).n_scale > (*n2).n_scale { (*n1).n_scale } else { (*n2).n_scale })
        {
            scale
        } else {
            (if (*n1).n_scale > (*n2).n_scale { (*n1).n_scale } else { (*n2).n_scale })
        })
    {
        if scale
            > (if (*n1).n_scale > (*n2).n_scale { (*n1).n_scale } else { (*n2).n_scale })
        {
            scale
        } else if (*n1).n_scale > (*n2).n_scale {
            (*n1).n_scale
        } else {
            (*n2).n_scale
        }
    } else {
        full_scale
    };
    _bc_rec_mul(n1, len1, n2, len2, &mut pval);
    (*pval)
        .n_sign = (if (*n1).n_sign as libc::c_uint == (*n2).n_sign as libc::c_uint {
        PLUS as libc::c_int
    } else {
        MINUS as libc::c_int
    }) as sign;
    (*pval).n_value = (*pval).n_ptr;
    (*pval).n_len = len2 + len1 + 1 as libc::c_int - full_scale;
    (*pval).n_scale = prod_scale;
    _bc_rm_leading_zeros(pval);
    if bc_is_zero(pval) != 0 {
        (*pval).n_sign = PLUS;
    }
    bc_free_num(prod);
    *prod = pval;
}
unsafe extern "C" fn _one_mult(
    mut num: *mut libc::c_uchar,
    mut size: libc::c_int,
    mut digit: libc::c_int,
    mut result: *mut libc::c_uchar,
) {
    let mut carry: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut nptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut rptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if digit == 0 as libc::c_int {
        memset(result as *mut libc::c_void, 0 as libc::c_int, size as libc::c_ulong);
    } else if digit == 1 as libc::c_int {
        memcpy(
            result as *mut libc::c_void,
            num as *const libc::c_void,
            size as libc::c_ulong,
        );
    } else {
        nptr = num.offset(size as isize).offset(-(1 as libc::c_int as isize));
        rptr = result.offset(size as isize).offset(-(1 as libc::c_int as isize));
        carry = 0 as libc::c_int;
        loop {
            let fresh34 = size;
            size = size - 1;
            if !(fresh34 > 0 as libc::c_int) {
                break;
            }
            let fresh35 = nptr;
            nptr = nptr.offset(-1);
            value = *fresh35 as libc::c_int * digit + carry;
            let fresh36 = rptr;
            rptr = rptr.offset(-1);
            *fresh36 = (value % 10 as libc::c_int) as libc::c_uchar;
            carry = value / 10 as libc::c_int;
        }
        if carry != 0 as libc::c_int {
            *rptr = carry as libc::c_uchar;
        }
    };
}
pub unsafe extern "C" fn bc_divide(
    mut n1: bc_num,
    mut n2: bc_num,
    mut quot: *mut bc_num,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut qval: bc_num = 0 as *mut bc_struct;
    let mut num1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut num2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ptr1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ptr2: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut n2ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut qptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut scale1: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut len1: libc::c_uint = 0;
    let mut len2: libc::c_uint = 0;
    let mut scale2: libc::c_uint = 0;
    let mut qdigits: libc::c_uint = 0;
    let mut extra: libc::c_uint = 0;
    let mut count: libc::c_uint = 0;
    let mut qdig: libc::c_uint = 0;
    let mut qguess: libc::c_uint = 0;
    let mut borrow: libc::c_uint = 0;
    let mut carry: libc::c_uint = 0;
    let mut mval: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut zero: libc::c_char = 0;
    let mut norm: libc::c_uint = 0;
    if bc_is_zero(n2) != 0 {
        return -(1 as libc::c_int);
    }
    if (*n2).n_scale == 0 as libc::c_int {
        if (*n2).n_len == 1 as libc::c_int
            && *(*n2).n_value as libc::c_int == 1 as libc::c_int
        {
            qval = bc_new_num((*n1).n_len, scale);
            (*qval)
                .n_sign = (if (*n1).n_sign as libc::c_uint
                == (*n2).n_sign as libc::c_uint
            {
                PLUS as libc::c_int
            } else {
                MINUS as libc::c_int
            }) as sign;
            memset(
                &mut *((*qval).n_value).offset((*n1).n_len as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                0 as libc::c_int,
                scale as libc::c_ulong,
            );
            memcpy(
                (*qval).n_value as *mut libc::c_void,
                (*n1).n_value as *const libc::c_void,
                ((*n1).n_len
                    + (if (*n1).n_scale > scale { scale } else { (*n1).n_scale }))
                    as libc::c_ulong,
            );
            bc_free_num(quot);
            *quot = qval;
        }
    }
    scale2 = (*n2).n_scale as libc::c_uint;
    n2ptr = ((*n2).n_value as *mut libc::c_uchar)
        .offset((*n2).n_len as isize)
        .offset(scale2 as isize)
        .offset(-(1 as libc::c_int as isize));
    while scale2 > 0 as libc::c_int as libc::c_uint
        && {
            let fresh37 = n2ptr;
            n2ptr = n2ptr.offset(-1);
            *fresh37 as libc::c_int == 0 as libc::c_int
        }
    {
        scale2 = scale2.wrapping_sub(1);
        scale2;
    }
    len1 = ((*n1).n_len as libc::c_uint).wrapping_add(scale2);
    scale1 = ((*n1).n_scale as libc::c_uint).wrapping_sub(scale2) as libc::c_int;
    if scale1 < scale {
        extra = (scale - scale1) as libc::c_uint;
    } else {
        extra = 0 as libc::c_int as libc::c_uint;
    }
    num1 = malloc(
        (((*n1).n_len + (*n1).n_scale) as libc::c_uint)
            .wrapping_add(extra)
            .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if num1.is_null() {
        out_of_memory();
    }
    memset(
        num1 as *mut libc::c_void,
        0 as libc::c_int,
        (((*n1).n_len + (*n1).n_scale) as libc::c_uint)
            .wrapping_add(extra)
            .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
    memcpy(
        num1.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        (*n1).n_value as *const libc::c_void,
        ((*n1).n_len + (*n1).n_scale) as libc::c_ulong,
    );
    len2 = ((*n2).n_len as libc::c_uint).wrapping_add(scale2);
    num2 = malloc(len2.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        as *mut libc::c_uchar;
    if num2.is_null() {
        out_of_memory();
    }
    memcpy(
        num2 as *mut libc::c_void,
        (*n2).n_value as *const libc::c_void,
        len2 as libc::c_ulong,
    );
    *num2.offset(len2 as isize) = 0 as libc::c_int as libc::c_uchar;
    n2ptr = num2;
    while *n2ptr as libc::c_int == 0 as libc::c_int {
        n2ptr = n2ptr.offset(1);
        n2ptr;
        len2 = len2.wrapping_sub(1);
        len2;
    }
    if len2 > len1.wrapping_add(scale as libc::c_uint) {
        qdigits = (scale + 1 as libc::c_int) as libc::c_uint;
        zero = 1 as libc::c_int as libc::c_char;
    } else {
        zero = 0 as libc::c_int as libc::c_char;
        if len2 > len1 {
            qdigits = (scale + 1 as libc::c_int) as libc::c_uint;
        } else {
            qdigits = len1
                .wrapping_sub(len2)
                .wrapping_add(scale as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint);
        }
    }
    qval = bc_new_num(qdigits.wrapping_sub(scale as libc::c_uint) as libc::c_int, scale);
    memset(
        (*qval).n_value as *mut libc::c_void,
        0 as libc::c_int,
        qdigits as libc::c_ulong,
    );
    mval = malloc(len2.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        as *mut libc::c_uchar;
    if mval.is_null() {
        out_of_memory();
    }
    if zero == 0 {
        norm = (10 as libc::c_int / (*n2ptr as libc::c_int + 1 as libc::c_int))
            as libc::c_uint;
        if norm != 1 as libc::c_int as libc::c_uint {
            _one_mult(
                num1,
                len1
                    .wrapping_add(scale1 as libc::c_uint)
                    .wrapping_add(extra)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
                norm as libc::c_int,
                num1,
            );
            _one_mult(n2ptr, len2 as libc::c_int, norm as libc::c_int, n2ptr);
        }
        qdig = 0 as libc::c_int as libc::c_uint;
        if len2 > len1 {
            qptr = ((*qval).n_value as *mut libc::c_uchar)
                .offset(len2 as isize)
                .offset(-(len1 as isize));
        } else {
            qptr = (*qval).n_value as *mut libc::c_uchar;
        }
        while qdig <= len1.wrapping_add(scale as libc::c_uint).wrapping_sub(len2) {
            if *n2ptr as libc::c_int == *num1.offset(qdig as isize) as libc::c_int {
                qguess = 9 as libc::c_int as libc::c_uint;
            } else {
                qguess = ((*num1.offset(qdig as isize) as libc::c_int * 10 as libc::c_int
                    + *num1
                        .offset(
                            qdig.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int) / *n2ptr as libc::c_int) as libc::c_uint;
            }
            if (*n2ptr.offset(1 as libc::c_int as isize) as libc::c_uint)
                .wrapping_mul(qguess)
                > ((*num1.offset(qdig as isize) as libc::c_int * 10 as libc::c_int
                    + *num1
                        .offset(
                            qdig.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int) as libc::c_uint)
                    .wrapping_sub((*n2ptr as libc::c_uint).wrapping_mul(qguess))
                    .wrapping_mul(10 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        *num1
                            .offset(
                                qdig.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_uint,
                    )
            {
                qguess = qguess.wrapping_sub(1);
                qguess;
                if (*n2ptr.offset(1 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_mul(qguess)
                    > ((*num1.offset(qdig as isize) as libc::c_int * 10 as libc::c_int
                        + *num1
                            .offset(
                                qdig.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int) as libc::c_uint)
                        .wrapping_sub((*n2ptr as libc::c_uint).wrapping_mul(qguess))
                        .wrapping_mul(10 as libc::c_int as libc::c_uint)
                        .wrapping_add(
                            *num1
                                .offset(
                                    qdig.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_uint,
                        )
                {
                    qguess = qguess.wrapping_sub(1);
                    qguess;
                }
            }
            borrow = 0 as libc::c_int as libc::c_uint;
            if qguess != 0 as libc::c_int as libc::c_uint {
                *mval = 0 as libc::c_int as libc::c_uchar;
                _one_mult(
                    n2ptr,
                    len2 as libc::c_int,
                    qguess as libc::c_int,
                    mval.offset(1 as libc::c_int as isize),
                );
                ptr1 = num1.offset(qdig as isize).offset(len2 as isize);
                ptr2 = mval.offset(len2 as isize);
                count = 0 as libc::c_int as libc::c_uint;
                while count < len2.wrapping_add(1 as libc::c_int as libc::c_uint) {
                    let fresh38 = ptr2;
                    ptr2 = ptr2.offset(-1);
                    val = ((*ptr1 as libc::c_int - *fresh38 as libc::c_int)
                        as libc::c_uint)
                        .wrapping_sub(borrow) as libc::c_int;
                    if val < 0 as libc::c_int {
                        val += 10 as libc::c_int;
                        borrow = 1 as libc::c_int as libc::c_uint;
                    } else {
                        borrow = 0 as libc::c_int as libc::c_uint;
                    }
                    let fresh39 = ptr1;
                    ptr1 = ptr1.offset(-1);
                    *fresh39 = val as libc::c_uchar;
                    count = count.wrapping_add(1);
                    count;
                }
            }
            if borrow == 1 as libc::c_int as libc::c_uint {
                qguess = qguess.wrapping_sub(1);
                qguess;
                ptr1 = num1.offset(qdig as isize).offset(len2 as isize);
                ptr2 = n2ptr.offset(len2 as isize).offset(-(1 as libc::c_int as isize));
                carry = 0 as libc::c_int as libc::c_uint;
                count = 0 as libc::c_int as libc::c_uint;
                while count < len2 {
                    let fresh40 = ptr2;
                    ptr2 = ptr2.offset(-1);
                    val = ((*ptr1 as libc::c_int + *fresh40 as libc::c_int)
                        as libc::c_uint)
                        .wrapping_add(carry) as libc::c_int;
                    if val > 9 as libc::c_int {
                        val -= 10 as libc::c_int;
                        carry = 1 as libc::c_int as libc::c_uint;
                    } else {
                        carry = 0 as libc::c_int as libc::c_uint;
                    }
                    let fresh41 = ptr1;
                    ptr1 = ptr1.offset(-1);
                    *fresh41 = val as libc::c_uchar;
                    count = count.wrapping_add(1);
                    count;
                }
                if carry == 1 as libc::c_int as libc::c_uint {
                    *ptr1 = ((*ptr1 as libc::c_int + 1 as libc::c_int)
                        % 10 as libc::c_int) as libc::c_uchar;
                }
            }
            let fresh42 = qptr;
            qptr = qptr.offset(1);
            *fresh42 = qguess as libc::c_uchar;
            qdig = qdig.wrapping_add(1);
            qdig;
        }
    }
    (*qval)
        .n_sign = (if (*n1).n_sign as libc::c_uint == (*n2).n_sign as libc::c_uint {
        PLUS as libc::c_int
    } else {
        MINUS as libc::c_int
    }) as sign;
    if bc_is_zero(qval) != 0 {
        (*qval).n_sign = PLUS;
    }
    _bc_rm_leading_zeros(qval);
    bc_free_num(quot);
    *quot = qval;
    free(mval as *mut libc::c_void);
    free(num1 as *mut libc::c_void);
    free(num2 as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bc_divmod(
    mut num1: bc_num,
    mut num2: bc_num,
    mut quot: *mut bc_num,
    mut rem: *mut bc_num,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut quotient: bc_num = 0 as bc_num;
    let mut temp: bc_num = 0 as *mut bc_struct;
    let mut rscale: libc::c_int = 0;
    if bc_is_zero(num2) != 0 {
        return -(1 as libc::c_int);
    }
    rscale = if (*num1).n_scale > (*num2).n_scale + scale {
        (*num1).n_scale
    } else {
        (*num2).n_scale + scale
    };
    bc_init_num(&mut temp);
    bc_divide(num1, num2, &mut temp, scale);
    if !quot.is_null() {
        quotient = bc_copy_num(temp);
    }
    bc_multiply(temp, num2, &mut temp, rscale);
    bc_sub(num1, temp, rem, rscale);
    bc_free_num(&mut temp);
    if !quot.is_null() {
        bc_free_num(quot);
        *quot = quotient;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bc_modulo(
    mut num1: bc_num,
    mut num2: bc_num,
    mut result: *mut bc_num,
    mut scale: libc::c_int,
) -> libc::c_int {
    return bc_divmod(num1, num2, 0 as *mut bc_num, result, scale);
}
pub unsafe extern "C" fn bc_raisemod(
    mut base: bc_num,
    mut expo: bc_num,
    mut mod_0: bc_num,
    mut result: *mut bc_num,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut power: bc_num = 0 as *mut bc_struct;
    let mut exponent: bc_num = 0 as *mut bc_struct;
    let mut parity: bc_num = 0 as *mut bc_struct;
    let mut temp: bc_num = 0 as *mut bc_struct;
    let mut rscale: libc::c_int = 0;
    if bc_is_zero(mod_0) != 0 {
        return -(1 as libc::c_int);
    }
    if bc_is_neg(expo) != 0 {
        return -(1 as libc::c_int);
    }
    power = bc_copy_num(base);
    exponent = bc_copy_num(expo);
    temp = bc_copy_num(_one_);
    bc_init_num(&mut parity);
    if (*base).n_scale != 0 as libc::c_int {
        rt_warn(b"non-zero scale in base\0" as *const u8 as *const libc::c_char);
    }
    if (*exponent).n_scale != 0 as libc::c_int {
        rt_warn(b"non-zero scale in exponent\0" as *const u8 as *const libc::c_char);
        bc_divide(exponent, _one_, &mut exponent, 0 as libc::c_int);
    }
    if (*mod_0).n_scale != 0 as libc::c_int {
        rt_warn(b"non-zero scale in modulus\0" as *const u8 as *const libc::c_char);
    }
    rscale = if scale > (*base).n_scale { scale } else { (*base).n_scale };
    while bc_is_zero(exponent) == 0 {
        bc_divmod(exponent, _two_, &mut exponent, &mut parity, 0 as libc::c_int);
        if bc_is_zero(parity) == 0 {
            bc_multiply(temp, power, &mut temp, rscale);
            bc_modulo(temp, mod_0, &mut temp, scale);
        }
        bc_multiply(power, power, &mut power, rscale);
        bc_modulo(power, mod_0, &mut power, scale);
    }
    bc_free_num(&mut power);
    bc_free_num(&mut exponent);
    bc_free_num(result);
    *result = temp;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn bc_raise(
    mut num1: bc_num,
    mut num2: bc_num,
    mut result: *mut bc_num,
    mut scale: libc::c_int,
) {
    let mut temp: bc_num = 0 as *mut bc_struct;
    let mut power: bc_num = 0 as *mut bc_struct;
    let mut exponent: libc::c_long = 0;
    let mut uexponent: libc::c_ulong = 0;
    let mut rscale: libc::c_int = 0;
    let mut pwrscale: libc::c_int = 0;
    let mut calcscale: libc::c_int = 0;
    let mut neg: libc::c_char = 0;
    if (*num2).n_scale != 0 as libc::c_int {
        rt_warn(b"non-zero scale in exponent\0" as *const u8 as *const libc::c_char);
    }
    exponent = bc_num2long(num2);
    if exponent == 0 as libc::c_int as libc::c_long
        && ((*num2).n_len > 1 as libc::c_int
            || *((*num2).n_value).offset(0 as libc::c_int as isize) as libc::c_int
                != 0 as libc::c_int)
    {
        rt_error(b"exponent too large in raise\0" as *const u8 as *const libc::c_char);
    }
    if exponent == 0 as libc::c_int as libc::c_long {
        bc_free_num(result);
        *result = bc_copy_num(_one_);
        return;
    }
    if exponent < 0 as libc::c_int as libc::c_long {
        neg = 1 as libc::c_int as libc::c_char;
        uexponent = -exponent as libc::c_ulong;
        rscale = scale;
    } else {
        neg = 0 as libc::c_int as libc::c_char;
        uexponent = exponent as libc::c_ulong;
        rscale = (if ((*num1).n_scale as libc::c_ulong).wrapping_mul(uexponent)
            > (if scale > (*num1).n_scale { scale } else { (*num1).n_scale })
                as libc::c_ulong
        {
            (if scale > (*num1).n_scale { scale } else { (*num1).n_scale })
                as libc::c_ulong
        } else {
            ((*num1).n_scale as libc::c_ulong).wrapping_mul(uexponent)
        }) as libc::c_int;
    }
    power = bc_copy_num(num1);
    pwrscale = (*num1).n_scale;
    while uexponent & 1 as libc::c_int as libc::c_ulong
        == 0 as libc::c_int as libc::c_ulong
    {
        pwrscale <<= 1 as libc::c_int;
        bc_multiply(power, power, &mut power, pwrscale);
        uexponent = uexponent >> 1 as libc::c_int;
    }
    temp = bc_copy_num(power);
    calcscale = pwrscale;
    uexponent >>= 1 as libc::c_int;
    while uexponent > 0 as libc::c_int as libc::c_ulong {
        pwrscale <<= 1 as libc::c_int;
        bc_multiply(power, power, &mut power, pwrscale);
        if uexponent & 1 as libc::c_int as libc::c_ulong
            == 1 as libc::c_int as libc::c_ulong
        {
            calcscale = pwrscale + calcscale;
            bc_multiply(temp, power, &mut temp, calcscale);
        }
        uexponent >>= 1 as libc::c_int;
    }
    if neg != 0 {
        bc_divide(_one_, temp, result, rscale);
        bc_free_num(&mut temp);
    } else {
        bc_free_num(result);
        *result = temp;
        if (**result).n_scale > rscale {
            (**result).n_scale = rscale;
        }
    }
    bc_free_num(&mut power);
}
pub unsafe extern "C" fn bc_sqrt(
    mut num: *mut bc_num,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut rscale: libc::c_int = 0;
    let mut cmp_res: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    let mut cscale: libc::c_int = 0;
    let mut guess: bc_num = 0 as *mut bc_struct;
    let mut guess1: bc_num = 0 as *mut bc_struct;
    let mut point5: bc_num = 0 as *mut bc_struct;
    let mut diff: bc_num = 0 as *mut bc_struct;
    cmp_res = bc_compare(*num, _zero_);
    if cmp_res < 0 as libc::c_int {
        return 0 as libc::c_int
    } else if cmp_res == 0 as libc::c_int {
        bc_free_num(num);
        *num = bc_copy_num(_zero_);
        return 1 as libc::c_int;
    }
    cmp_res = bc_compare(*num, _one_);
    if cmp_res == 0 as libc::c_int {
        bc_free_num(num);
        *num = bc_copy_num(_one_);
        return 1 as libc::c_int;
    }
    rscale = if scale > (**num).n_scale { scale } else { (**num).n_scale };
    bc_init_num(&mut guess);
    bc_init_num(&mut guess1);
    bc_init_num(&mut diff);
    point5 = bc_new_num(1 as libc::c_int, 1 as libc::c_int);
    *((*point5).n_value)
        .offset(1 as libc::c_int as isize) = 5 as libc::c_int as libc::c_char;
    if cmp_res < 0 as libc::c_int {
        guess = bc_copy_num(_one_);
        cscale = (**num).n_scale;
    } else {
        bc_int2num(&mut guess, 10 as libc::c_int);
        bc_int2num(&mut guess1, (**num).n_len);
        bc_multiply(guess1, point5, &mut guess1, 0 as libc::c_int);
        (*guess1).n_scale = 0 as libc::c_int;
        bc_raise(guess, guess1, &mut guess, 0 as libc::c_int);
        bc_free_num(&mut guess1);
        cscale = 3 as libc::c_int;
    }
    done = 0 as libc::c_int;
    while done == 0 {
        bc_free_num(&mut guess1);
        guess1 = bc_copy_num(guess);
        bc_divide(*num, guess, &mut guess, cscale);
        bc_add(guess, guess1, &mut guess, 0 as libc::c_int);
        bc_multiply(guess, point5, &mut guess, cscale);
        bc_sub(guess, guess1, &mut diff, cscale + 1 as libc::c_int);
        if bc_is_near_zero(diff, cscale) != 0 {
            if cscale < rscale + 1 as libc::c_int {
                cscale = if cscale * 3 as libc::c_int > rscale + 1 as libc::c_int {
                    rscale + 1 as libc::c_int
                } else {
                    cscale * 3 as libc::c_int
                };
            } else {
                done = 1 as libc::c_int;
            }
        }
    }
    bc_free_num(num);
    bc_divide(guess, _one_, num, rscale);
    bc_free_num(&mut guess);
    bc_free_num(&mut guess1);
    bc_free_num(&mut point5);
    bc_free_num(&mut diff);
    return 1 as libc::c_int;
}
static mut ref_str: [libc::c_char; 17] = unsafe {
    *::std::mem::transmute::<&[u8; 17], &mut [libc::c_char; 17]>(b"0123456789ABCDEF\0")
};
pub unsafe extern "C" fn bc_out_long(
    mut val: libc::c_long,
    mut size: libc::c_int,
    mut space: libc::c_int,
    mut out_char: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
) {
    let mut digits: [libc::c_char; 40] = [0; 40];
    let mut len: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    if space != 0 {
        (Some(out_char.unwrap())).unwrap()(' ' as i32);
    }
    snprintf(
        digits.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
        b"%ld\0" as *const u8 as *const libc::c_char,
        val,
    );
    len = strlen(digits.as_mut_ptr()) as libc::c_int;
    while size > len {
        (Some(out_char.unwrap())).unwrap()('0' as i32);
        size -= 1;
        size;
    }
    ix = 0 as libc::c_int;
    while ix < len {
        (Some(out_char.unwrap())).unwrap()(digits[ix as usize] as libc::c_int);
        ix += 1;
        ix;
    }
}
pub unsafe extern "C" fn bc_out_num(
    mut num: bc_num,
    mut o_base: libc::c_int,
    mut out_char: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    mut leading_zero: libc::c_int,
) {
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ix: libc::c_int = 0;
    let mut fdigit: libc::c_int = 0;
    let mut pre_space: libc::c_int = 0;
    let mut digits: *mut stk_rec = 0 as *mut stk_rec;
    let mut temp: *mut stk_rec = 0 as *mut stk_rec;
    let mut int_part: bc_num = 0 as *mut bc_struct;
    let mut frac_part: bc_num = 0 as *mut bc_struct;
    let mut base: bc_num = 0 as *mut bc_struct;
    let mut cur_dig: bc_num = 0 as *mut bc_struct;
    let mut t_num: bc_num = 0 as *mut bc_struct;
    let mut max_o_digit: bc_num = 0 as *mut bc_struct;
    if (*num).n_sign as libc::c_uint == MINUS as libc::c_int as libc::c_uint {
        (Some(out_char.unwrap())).unwrap()('-' as i32);
    }
    if bc_is_zero(num) != 0 {
        (Some(out_char.unwrap())).unwrap()('0' as i32);
    } else if o_base == 10 as libc::c_int {
        nptr = (*num).n_value;
        if (*num).n_len > 1 as libc::c_int || *nptr as libc::c_int != 0 as libc::c_int {
            ix = (*num).n_len;
            while ix > 0 as libc::c_int {
                let fresh43 = nptr;
                nptr = nptr.offset(1);
                (Some(out_char.unwrap())).unwrap()(*fresh43 as libc::c_int + '0' as i32);
                ix -= 1;
                ix;
            }
        } else {
            nptr = nptr.offset(1);
            nptr;
        }
        if leading_zero != 0 && bc_is_zero(num) as libc::c_int != 0 {
            (Some(out_char.unwrap())).unwrap()('0' as i32);
        }
        if (*num).n_scale > 0 as libc::c_int {
            (Some(out_char.unwrap())).unwrap()('.' as i32);
            ix = 0 as libc::c_int;
            while ix < (*num).n_scale {
                let fresh44 = nptr;
                nptr = nptr.offset(1);
                (Some(out_char.unwrap())).unwrap()(*fresh44 as libc::c_int + '0' as i32);
                ix += 1;
                ix;
            }
        }
    } else {
        if leading_zero != 0 && bc_is_zero(num) as libc::c_int != 0 {
            (Some(out_char.unwrap())).unwrap()('0' as i32);
        }
        digits = 0 as *mut stk_rec;
        bc_init_num(&mut int_part);
        bc_divide(num, _one_, &mut int_part, 0 as libc::c_int);
        bc_init_num(&mut frac_part);
        bc_init_num(&mut cur_dig);
        bc_init_num(&mut base);
        bc_sub(num, int_part, &mut frac_part, 0 as libc::c_int);
        (*int_part).n_sign = PLUS;
        (*frac_part).n_sign = PLUS;
        bc_int2num(&mut base, o_base);
        bc_init_num(&mut max_o_digit);
        bc_int2num(&mut max_o_digit, o_base - 1 as libc::c_int);
        while bc_is_zero(int_part) == 0 {
            bc_modulo(int_part, base, &mut cur_dig, 0 as libc::c_int);
            temp = malloc(::std::mem::size_of::<stk_rec>() as libc::c_ulong)
                as *mut stk_rec;
            if temp.is_null() {
                out_of_memory();
            }
            (*temp).digit = bc_num2long(cur_dig);
            (*temp).next = digits;
            digits = temp;
            bc_divide(int_part, base, &mut int_part, 0 as libc::c_int);
        }
        if !digits.is_null() {
            while !digits.is_null() {
                temp = digits;
                digits = (*digits).next;
                if o_base <= 16 as libc::c_int {
                    (Some(out_char.unwrap()))
                        .unwrap()(
                        ref_str[(*temp).digit as libc::c_int as usize] as libc::c_int,
                    );
                } else {
                    bc_out_long(
                        (*temp).digit,
                        (*max_o_digit).n_len,
                        1 as libc::c_int,
                        out_char,
                    );
                }
                free(temp as *mut libc::c_void);
            }
        }
        if (*num).n_scale > 0 as libc::c_int {
            (Some(out_char.unwrap())).unwrap()('.' as i32);
            pre_space = 0 as libc::c_int;
            t_num = bc_copy_num(_one_);
            while (*t_num).n_len <= (*num).n_scale {
                bc_multiply(frac_part, base, &mut frac_part, (*num).n_scale);
                fdigit = bc_num2long(frac_part) as libc::c_int;
                bc_int2num(&mut int_part, fdigit);
                bc_sub(frac_part, int_part, &mut frac_part, 0 as libc::c_int);
                if o_base <= 16 as libc::c_int {
                    (Some(out_char.unwrap()))
                        .unwrap()(ref_str[fdigit as usize] as libc::c_int);
                } else {
                    bc_out_long(
                        fdigit as libc::c_long,
                        (*max_o_digit).n_len,
                        pre_space,
                        out_char,
                    );
                    pre_space = 1 as libc::c_int;
                }
                bc_multiply(t_num, base, &mut t_num, 0 as libc::c_int);
            }
            bc_free_num(&mut t_num);
        }
        bc_free_num(&mut int_part);
        bc_free_num(&mut frac_part);
        bc_free_num(&mut base);
        bc_free_num(&mut cur_dig);
        bc_free_num(&mut max_o_digit);
    };
}
pub unsafe extern "C" fn bc_num2long(mut num: bc_num) -> libc::c_long {
    let mut val: libc::c_long = 0;
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    val = 0 as libc::c_int as libc::c_long;
    nptr = (*num).n_value;
    i = (*num).n_len;
    while i > 0 as libc::c_int
        && val <= (0x7fffffff as libc::c_int / 10 as libc::c_int) as libc::c_long
    {
        let fresh45 = nptr;
        nptr = nptr.offset(1);
        val = val * 10 as libc::c_int as libc::c_long + *fresh45 as libc::c_long;
        i -= 1;
        i;
    }
    if i > 0 as libc::c_int {
        val = 0 as libc::c_int as libc::c_long;
    }
    if val < 0 as libc::c_int as libc::c_long {
        val = 0 as libc::c_int as libc::c_long;
    }
    if (*num).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint {
        return val
    } else {
        return -val
    };
}
pub unsafe extern "C" fn bc_int2num(mut num: *mut bc_num, mut val: libc::c_int) {
    let mut buffer: [libc::c_char; 30] = [0; 30];
    let mut bptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ix: libc::c_int = 1 as libc::c_int;
    let mut neg: libc::c_char = 0 as libc::c_int as libc::c_char;
    if val < 0 as libc::c_int {
        neg = 1 as libc::c_int as libc::c_char;
        val = -val;
    }
    bptr = buffer.as_mut_ptr();
    let fresh46 = bptr;
    bptr = bptr.offset(1);
    *fresh46 = (val % 10 as libc::c_int) as libc::c_char;
    val = val / 10 as libc::c_int;
    while val != 0 as libc::c_int {
        let fresh47 = bptr;
        bptr = bptr.offset(1);
        *fresh47 = (val % 10 as libc::c_int) as libc::c_char;
        val = val / 10 as libc::c_int;
        ix += 1;
        ix;
    }
    bc_free_num(num);
    *num = bc_new_num(ix, 0 as libc::c_int);
    if neg != 0 {
        (**num).n_sign = MINUS;
    }
    vptr = (**num).n_value;
    loop {
        let fresh48 = ix;
        ix = ix - 1;
        if !(fresh48 > 0 as libc::c_int) {
            break;
        }
        bptr = bptr.offset(-1);
        let fresh49 = vptr;
        vptr = vptr.offset(1);
        *fresh49 = *bptr;
    };
}
pub unsafe extern "C" fn bc_num2str(mut num: bc_num) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut signch: libc::c_int = 0;
    signch = if (*num).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    if (*num).n_scale > 0 as libc::c_int {
        str = malloc(
            ((*num).n_len + (*num).n_scale + 2 as libc::c_int + signch) as libc::c_ulong,
        ) as *mut libc::c_char;
    } else {
        str = malloc(((*num).n_len + 1 as libc::c_int + signch) as libc::c_ulong)
            as *mut libc::c_char;
    }
    if str.is_null() {
        out_of_memory();
    }
    sptr = str;
    if signch != 0 {
        let fresh50 = sptr;
        sptr = sptr.offset(1);
        *fresh50 = '-' as i32 as libc::c_char;
    }
    nptr = (*num).n_value;
    i = (*num).n_len;
    while i > 0 as libc::c_int {
        let fresh51 = nptr;
        nptr = nptr.offset(1);
        let fresh52 = sptr;
        sptr = sptr.offset(1);
        *fresh52 = (*fresh51 as libc::c_int + '0' as i32) as libc::c_char;
        i -= 1;
        i;
    }
    if (*num).n_scale > 0 as libc::c_int {
        let fresh53 = sptr;
        sptr = sptr.offset(1);
        *fresh53 = '.' as i32 as libc::c_char;
        i = 0 as libc::c_int;
        while i < (*num).n_scale {
            let fresh54 = nptr;
            nptr = nptr.offset(1);
            let fresh55 = sptr;
            sptr = sptr.offset(1);
            *fresh55 = (*fresh54 as libc::c_int + '0' as i32) as libc::c_char;
            i += 1;
            i;
        }
    }
    *sptr = '\0' as i32 as libc::c_char;
    return str;
}
pub unsafe extern "C" fn bc_str2num(
    mut num: *mut bc_num,
    mut str: *mut libc::c_char,
    mut scale: libc::c_int,
) {
    let mut digits: libc::c_int = 0;
    let mut strscale: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zero_int: libc::c_char = 0;
    bc_free_num(num);
    ptr = str;
    digits = 0 as libc::c_int;
    strscale = 0 as libc::c_int;
    zero_int = 0 as libc::c_int as libc::c_char;
    if *ptr as libc::c_int == '+' as i32 || *ptr as libc::c_int == '-' as i32 {
        ptr = ptr.offset(1);
        ptr;
    }
    while *ptr as libc::c_int == '0' as i32 {
        ptr = ptr.offset(1);
        ptr;
    }
    while *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(1);
        ptr;
        digits += 1;
        digits;
    }
    if *ptr as libc::c_int == '.' as i32 {
        ptr = ptr.offset(1);
        ptr;
    }
    while *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(1);
        ptr;
        strscale += 1;
        strscale;
    }
    if *ptr as libc::c_int != '\0' as i32 || digits + strscale == 0 as libc::c_int {
        *num = bc_copy_num(_zero_);
        return;
    }
    strscale = if strscale > scale { scale } else { strscale };
    if digits == 0 as libc::c_int {
        zero_int = 1 as libc::c_int as libc::c_char;
        digits = 1 as libc::c_int;
    }
    *num = bc_new_num(digits, strscale);
    ptr = str;
    if *ptr as libc::c_int == '-' as i32 {
        (**num).n_sign = MINUS;
        ptr = ptr.offset(1);
        ptr;
    } else {
        (**num).n_sign = PLUS;
        if *ptr as libc::c_int == '+' as i32 {
            ptr = ptr.offset(1);
            ptr;
        }
    }
    while *ptr as libc::c_int == '0' as i32 {
        ptr = ptr.offset(1);
        ptr;
    }
    nptr = (**num).n_value;
    if zero_int != 0 {
        let fresh56 = nptr;
        nptr = nptr.offset(1);
        *fresh56 = 0 as libc::c_int as libc::c_char;
        digits = 0 as libc::c_int;
    }
    while digits > 0 as libc::c_int {
        let fresh57 = ptr;
        ptr = ptr.offset(1);
        let fresh58 = nptr;
        nptr = nptr.offset(1);
        *fresh58 = (*fresh57 as libc::c_int - '0' as i32) as libc::c_char;
        digits -= 1;
        digits;
    }
    if strscale > 0 as libc::c_int {
        ptr = ptr.offset(1);
        ptr;
        while strscale > 0 as libc::c_int {
            let fresh59 = ptr;
            ptr = ptr.offset(1);
            let fresh60 = nptr;
            nptr = nptr.offset(1);
            *fresh60 = (*fresh59 as libc::c_int - '0' as i32) as libc::c_char;
            strscale -= 1;
            strscale;
        }
    }
}
