use ::libc;
extern "C" {
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type __int32_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type score_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_struct {
    pub needle_len: libc::c_int,
    pub haystack_len: libc::c_int,
    pub lower_needle: [libc::c_char; 1024],
    pub lower_haystack: [libc::c_char; 1024],
    pub match_bonus: [score_t; 1024],
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
pub static mut bonus_states: [[score_t; 256]; 3] = [
    [
        0 as libc::c_int as score_t,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ],
    [
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.8f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.8f64,
        0.6f64,
        0.9f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.8f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ],
    [
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.8f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.8f64,
        0.6f64,
        0.9f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.8f64,
        0.,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ],
];
pub static mut bonus_index: [size_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub unsafe extern "C" fn strcasechr(
    mut s: *const libc::c_char,
    mut c: libc::c_char,
) -> *mut libc::c_char {
    let accept: [libc::c_char; 3] = [
        c,
        ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = c as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(c as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(c as libc::c_int as isize);
            }
            __res
        }) as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    return strpbrk(s, accept.as_ptr());
}
pub unsafe extern "C" fn has_match(
    mut needle: *const libc::c_char,
    mut haystack: *const libc::c_char,
) -> libc::c_int {
    while *needle != 0 {
        let fresh0 = needle;
        needle = needle.offset(1);
        let mut nch: libc::c_char = *fresh0;
        haystack = strcasechr(haystack, nch);
        if haystack.is_null() {
            return 0 as libc::c_int;
        }
        haystack = haystack.offset(1);
        haystack;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn precompute_bonus(
    mut haystack: *const libc::c_char,
    mut match_bonus: *mut score_t,
) {
    let mut last_ch: libc::c_char = '/' as i32 as libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while *haystack.offset(i as isize) != 0 {
        let mut ch: libc::c_char = *haystack.offset(i as isize);
        *match_bonus
            .offset(
                i as isize,
            ) = bonus_states[bonus_index[ch as libc::c_uchar as usize]
            as usize][last_ch as libc::c_uchar as usize];
        last_ch = ch;
        i += 1;
        i;
    }
}
unsafe extern "C" fn setup_match_struct(
    mut match_1: *mut match_struct,
    mut needle: *const libc::c_char,
    mut haystack: *const libc::c_char,
) {
    (*match_1).needle_len = strlen(needle) as libc::c_int;
    (*match_1).haystack_len = strlen(haystack) as libc::c_int;
    if (*match_1).haystack_len > 1024 as libc::c_int
        || (*match_1).needle_len > (*match_1).haystack_len
    {
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*match_1).needle_len {
        (*match_1)
            .lower_needle[i
            as usize] = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *needle.offset(i as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*needle.offset(i as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*needle.offset(i as isize) as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < (*match_1).haystack_len {
        (*match_1)
            .lower_haystack[i_0
            as usize] = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *haystack.offset(i_0 as isize)
                        as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*haystack.offset(i_0 as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*haystack.offset(i_0 as isize) as libc::c_int as isize);
            }
            __res
        }) as libc::c_char;
        i_0 += 1;
        i_0;
    }
    precompute_bonus(haystack, ((*match_1).match_bonus).as_mut_ptr());
}
#[inline]
unsafe extern "C" fn match_row(
    mut match_1: *const match_struct,
    mut row: libc::c_int,
    mut curr_D: *mut score_t,
    mut curr_M: *mut score_t,
    mut last_D: *const score_t,
    mut last_M: *const score_t,
) {
    let mut n: libc::c_int = (*match_1).needle_len;
    let mut m: libc::c_int = (*match_1).haystack_len;
    let mut i: libc::c_int = row;
    let mut lower_needle: *const libc::c_char = ((*match_1).lower_needle).as_ptr();
    let mut lower_haystack: *const libc::c_char = ((*match_1).lower_haystack).as_ptr();
    let mut match_bonus: *const score_t = ((*match_1).match_bonus).as_ptr();
    let mut prev_score: score_t = -::std::f32::INFINITY as score_t;
    let mut gap_score: score_t = if i == n - 1 as libc::c_int {
        -0.005f64
    } else {
        -0.01f64
    };
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < m {
        if *lower_needle.offset(i as isize) as libc::c_int
            == *lower_haystack.offset(j as isize) as libc::c_int
        {
            let mut score: score_t = -::std::f32::INFINITY as score_t;
            if i == 0 {
                score = j as libc::c_double * -0.005f64
                    + *match_bonus.offset(j as isize);
            } else if j != 0 {
                score = if *last_M.offset((j - 1 as libc::c_int) as isize)
                    + *match_bonus.offset(j as isize)
                    > *last_D.offset((j - 1 as libc::c_int) as isize) + 1.0f64
                {
                    *last_M.offset((j - 1 as libc::c_int) as isize)
                        + *match_bonus.offset(j as isize)
                } else {
                    *last_D.offset((j - 1 as libc::c_int) as isize) + 1.0f64
                };
            }
            *curr_D.offset(j as isize) = score;
            prev_score = if score > prev_score + gap_score {
                score
            } else {
                prev_score + gap_score
            };
            *curr_M.offset(j as isize) = prev_score;
        } else {
            *curr_D.offset(j as isize) = -::std::f32::INFINITY as score_t;
            prev_score = prev_score + gap_score;
            *curr_M.offset(j as isize) = prev_score;
        }
        j += 1;
        j;
    }
}
#[export_name = "match"]
pub unsafe extern "C" fn match_0(
    mut needle: *const libc::c_char,
    mut haystack: *const libc::c_char,
) -> score_t {
    if *needle == 0 {
        return -::std::f32::INFINITY as score_t;
    }
    let mut match_1: match_struct = match_struct {
        needle_len: 0,
        haystack_len: 0,
        lower_needle: [0; 1024],
        lower_haystack: [0; 1024],
        match_bonus: [0.; 1024],
    };
    setup_match_struct(&mut match_1, needle, haystack);
    let mut n: libc::c_int = match_1.needle_len;
    let mut m: libc::c_int = match_1.haystack_len;
    if m > 1024 as libc::c_int || n > m {
        return -::std::f32::INFINITY as score_t
    } else if n == m {
        return ::std::f32::INFINITY as score_t
    }
    let mut D: [[score_t; 1024]; 2] = [[0.; 1024]; 2];
    let mut M: [[score_t; 1024]; 2] = [[0.; 1024]; 2];
    let mut last_D: *mut score_t = 0 as *mut score_t;
    let mut last_M: *mut score_t = 0 as *mut score_t;
    let mut curr_D: *mut score_t = 0 as *mut score_t;
    let mut curr_M: *mut score_t = 0 as *mut score_t;
    last_D = (D[0 as libc::c_int as usize]).as_mut_ptr();
    last_M = (M[0 as libc::c_int as usize]).as_mut_ptr();
    curr_D = (D[1 as libc::c_int as usize]).as_mut_ptr();
    curr_M = (M[1 as libc::c_int as usize]).as_mut_ptr();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        match_row(&mut match_1, i, curr_D, curr_M, last_D, last_M);
        let mut SWAP: *mut score_t = curr_D;
        curr_D = last_D;
        last_D = SWAP;
        let mut SWAP_0: *mut score_t = curr_M;
        curr_M = last_M;
        last_M = SWAP_0;
        i += 1;
        i;
    }
    return *last_M.offset((m - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn match_positions(
    mut needle: *const libc::c_char,
    mut haystack: *const libc::c_char,
    mut positions: *mut size_t,
) -> score_t {
    if *needle == 0 {
        return -::std::f32::INFINITY as score_t;
    }
    let mut match_1: match_struct = match_struct {
        needle_len: 0,
        haystack_len: 0,
        lower_needle: [0; 1024],
        lower_haystack: [0; 1024],
        match_bonus: [0.; 1024],
    };
    setup_match_struct(&mut match_1, needle, haystack);
    let mut n: libc::c_int = match_1.needle_len;
    let mut m: libc::c_int = match_1.haystack_len;
    if m > 1024 as libc::c_int || n > m {
        return -::std::f32::INFINITY as score_t
    } else if n == m {
        if !positions.is_null() {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < n {
                *positions.offset(i as isize) = i as size_t;
                i += 1;
                i;
            }
        }
        return ::std::f32::INFINITY as score_t;
    }
    let mut D: *mut [score_t; 1024] = 0 as *mut [score_t; 1024];
    let mut M: *mut [score_t; 1024] = 0 as *mut [score_t; 1024];
    M = malloc(
        (::std::mem::size_of::<score_t>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut [score_t; 1024];
    D = malloc(
        (::std::mem::size_of::<score_t>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_int as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut [score_t; 1024];
    let mut last_D: *mut score_t = 0 as *mut score_t;
    let mut last_M: *mut score_t = 0 as *mut score_t;
    let mut curr_D: *mut score_t = 0 as *mut score_t;
    let mut curr_M: *mut score_t = 0 as *mut score_t;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        curr_D = &mut *(*D.offset(i_0 as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut score_t;
        curr_M = &mut *(*M.offset(i_0 as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut score_t;
        match_row(&mut match_1, i_0, curr_D, curr_M, last_D, last_M);
        last_D = curr_D;
        last_M = curr_M;
        i_0 += 1;
        i_0;
    }
    if !positions.is_null() {
        let mut match_required: libc::c_int = 0 as libc::c_int;
        let mut i_1: libc::c_int = n - 1 as libc::c_int;
        let mut j: libc::c_int = m - 1 as libc::c_int;
        while i_1 >= 0 as libc::c_int {
            while j >= 0 as libc::c_int {
                if (*D.offset(i_1 as isize))[j as usize]
                    != -::std::f32::INFINITY as libc::c_double
                    && (match_required != 0
                        || (*D.offset(i_1 as isize))[j as usize]
                            == (*M.offset(i_1 as isize))[j as usize])
                {
                    match_required = (i_1 != 0 && j != 0
                        && (*M.offset(i_1 as isize))[j as usize]
                            == (*D
                                .offset(
                                    (i_1 - 1 as libc::c_int) as isize,
                                ))[(j - 1 as libc::c_int) as usize] + 1.0f64)
                        as libc::c_int;
                    let fresh1 = j;
                    j = j - 1;
                    *positions.offset(i_1 as isize) = fresh1 as size_t;
                    break;
                } else {
                    j -= 1;
                    j;
                }
            }
            i_1 -= 1;
            i_1;
        }
    }
    let mut result: score_t = (*M
        .offset((n - 1 as libc::c_int) as isize))[(m - 1 as libc::c_int) as usize];
    free(M as *mut libc::c_void);
    free(D as *mut libc::c_void);
    return result;
}
