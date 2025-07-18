use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn gsl_sort_long_double(data: *mut f128::f128, stride: size_t, n: size_t);
    fn gsl_sort(data: *mut libc::c_double, stride: size_t, n: size_t);
    fn gsl_sort_short(data: *mut libc::c_short, stride: size_t, n: size_t);
    fn gsl_sort_float(data: *mut libc::c_float, stride: size_t, n: size_t);
    fn gsl_sort_char(data: *mut libc::c_char, stride: size_t, n: size_t);
    fn gsl_sort_ulong(data: *mut libc::c_ulong, stride: size_t, n: size_t);
    fn gsl_sort_uint(data: *mut libc::c_uint, stride: size_t, n: size_t);
    fn gsl_sort_long(data: *mut libc::c_long, stride: size_t, n: size_t);
    fn gsl_sort_int(data: *mut libc::c_int, stride: size_t, n: size_t);
    fn gsl_sort_ushort(data: *mut libc::c_ushort, stride: size_t, n: size_t);
    fn gsl_sort_uchar(data: *mut libc::c_uchar, stride: size_t, n: size_t);
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_double = &mut *work.offset(n as isize)
        as *mut libc::c_double;
    let mut a_cand: *mut libc::c_double = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_double;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_double = 0.0f64;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) < trial
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while *sorted_data.offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) > trial
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_short_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_short,
    mut work_int: *mut libc::c_int,
) -> libc::c_short {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_short = &mut *work.offset(n as isize)
        as *mut libc::c_short;
    let mut a_cand: *mut libc::c_short = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_short;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_short = 0.0f64 as libc::c_short;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64 as libc::c_short;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_short;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_short_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && ((*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double)
                    < trial as libc::c_int as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while (*sorted_data
                .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                as libc::c_int
                - *sorted_data
                    .offset(
                        ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double
                > trial as libc::c_int as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_short;
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_short(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_uchar_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_uchar,
    mut work_int: *mut libc::c_int,
) -> libc::c_uchar {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_uchar = &mut *work.offset(n as isize)
        as *mut libc::c_uchar;
    let mut a_cand: *mut libc::c_uchar = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_uchar;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_uchar = 0.0f64 as libc::c_uchar;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64 as libc::c_uchar;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_uchar;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_uchar_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && ((*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double)
                    < trial as libc::c_int as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while (*sorted_data
                .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                as libc::c_int
                - *sorted_data
                    .offset(
                        ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double
                > trial as libc::c_int as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_uchar;
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_uchar(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_float_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_float,
    mut work_int: *mut libc::c_int,
) -> libc::c_float {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_float = &mut *work.offset(n as isize)
        as *mut libc::c_float;
    let mut a_cand: *mut libc::c_float = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_float;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_float = 0.0f64 as libc::c_float;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64 as libc::c_float;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_float_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && ((*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while (*sorted_data
                .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )) as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_float(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_long_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_long,
    mut work_int: *mut libc::c_int,
) -> libc::c_long {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_long = &mut *work.offset(n as isize)
        as *mut libc::c_long;
    let mut a_cand: *mut libc::c_long = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_long;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_long = 0.0f64 as libc::c_long;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64 as libc::c_long;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_long_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && ((*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while (*sorted_data
                .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )) as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_long(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_int_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_int,
    mut work_int: *mut libc::c_int,
) -> libc::c_int {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_int = &mut *work.offset(n as isize) as *mut libc::c_int;
    let mut a_cand: *mut libc::c_int = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_int = 0.0f64 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64 as libc::c_int;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_int_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && ((*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while (*sorted_data
                .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                - *sorted_data
                    .offset(
                        ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )) as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_int(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_uint_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_uint,
    mut work_int: *mut libc::c_int,
) -> libc::c_uint {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_uint = &mut *work.offset(n as isize)
        as *mut libc::c_uint;
    let mut a_cand: *mut libc::c_uint = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_uint;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_uint = 0.0f64 as libc::c_uint;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64 as libc::c_uint;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                            ),
                    );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_uint_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && ((*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while (*sorted_data
                .offset((i as libc::c_ulong).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                            ),
                    );
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_uint(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_long_double_Qn0_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut f128::f128,
    mut work_int: *mut libc::c_int,
) -> f128::f128 {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut f128::f128 = &mut *work.offset(n as isize) as *mut f128::f128;
    let mut a_cand: *mut f128::f128 = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut f128::f128;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: f128::f128 = f128::f128::new(0.0f64);
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return f128::f128::new(0.0f64);
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_long_double_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && f128::f128::new(
                    (*sorted_data
                        .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                        - *sorted_data
                            .offset(
                                ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ))
                        .to_f64()
                        .unwrap(),
                ) < trial
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while f128::f128::new(
                (*sorted_data.offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ))
                    .to_f64()
                    .unwrap(),
            ) > trial
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = *sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    - *sorted_data
                        .offset(
                            ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                        );
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_long_double(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_ushort_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ushort,
    mut work_int: *mut libc::c_int,
) -> libc::c_ushort {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_ushort = &mut *work.offset(n as isize)
        as *mut libc::c_ushort;
    let mut a_cand: *mut libc::c_ushort = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_ushort;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_ushort = 0.0f64 as libc::c_ushort;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64 as libc::c_ushort;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_ushort;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_ushort_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && ((*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double)
                    < trial as libc::c_int as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while (*sorted_data
                .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                as libc::c_int
                - *sorted_data
                    .offset(
                        ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double
                > trial as libc::c_int as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_ushort;
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_ushort(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_char_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_char,
    mut work_int: *mut libc::c_int,
) -> libc::c_char {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_char = &mut *work.offset(n as isize)
        as *mut libc::c_char;
    let mut a_cand: *mut libc::c_char = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_char;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_char = 0.0f64 as libc::c_char;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64 as libc::c_char;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_char;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_char_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && ((*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double)
                    < trial as libc::c_int as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while (*sorted_data
                .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                as libc::c_int
                - *sorted_data
                    .offset(
                        ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double
                > trial as libc::c_int as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize)
                    as libc::c_int
                    - *sorted_data
                        .offset(
                            ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_char;
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_char(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_ulong_Qn0_from_sorted_data(
    mut sorted_data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ulong,
    mut work_int: *mut libc::c_int,
) -> libc::c_ulong {
    let ni: libc::c_int = n as libc::c_int;
    let mut a_srt: *mut libc::c_ulong = &mut *work.offset(n as isize)
        as *mut libc::c_ulong;
    let mut a_cand: *mut libc::c_ulong = &mut *work
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_ulong;
    let mut left: *mut libc::c_int = &mut *work_int.offset(0 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut right: *mut libc::c_int = &mut *work_int.offset(n as isize)
        as *mut libc::c_int;
    let mut p: *mut libc::c_int = &mut *work_int
        .offset((2 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut q: *mut libc::c_int = &mut *work_int
        .offset((3 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut weight: *mut libc::c_int = &mut *work_int
        .offset((4 as libc::c_int as libc::c_ulong).wrapping_mul(n) as isize)
        as *mut libc::c_int;
    let mut trial: libc::c_ulong = 0.0f64 as libc::c_ulong;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jh: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut knew: libc::c_long = 0;
    let mut nl: libc::c_long = 0;
    let mut nr: libc::c_long = 0;
    let mut sump: libc::c_long = 0;
    let mut sumq: libc::c_long = 0;
    if n < 2 as libc::c_int as libc::c_ulong {
        return 0.0f64 as libc::c_ulong;
    }
    h = n
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    k = h as libc::c_long * (h - 1 as libc::c_int) as libc::c_long
        / 2 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < ni {
        *left.offset(i as isize) = ni - i + 1 as libc::c_int;
        *right.offset(i as isize) = if i <= h { ni } else { ni - (i - h) };
        i += 1;
        i;
    }
    nl = (n as libc::c_long as libc::c_ulong)
        .wrapping_mul(n.wrapping_add(1 as libc::c_int as libc::c_ulong))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_long;
    nr = (n as libc::c_long as libc::c_ulong).wrapping_mul(n) as libc::c_long;
    knew = k + nl;
    while found == 0 && nr - nl > ni as libc::c_long {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            if *left.offset(i as isize) <= *right.offset(i as isize) {
                *weight
                    .offset(
                        j as isize,
                    ) = *right.offset(i as isize) - *left.offset(i as isize)
                    + 1 as libc::c_int;
                jh = *left.offset(i as isize)
                    + *weight.offset(j as isize) / 2 as libc::c_int;
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((ni - jh) as libc::c_ulong).wrapping_mul(stride) as isize,
                            ),
                    );
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        trial = Qn_ulong_whimed(work, weight, j, a_cand, a_srt, p);
        j = 0 as libc::c_int;
        i = ni - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            while j < ni
                && ((*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((ni - j - 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double) < trial as libc::c_double
            {
                j += 1;
                j;
            }
            *p.offset(i as isize) = j;
            i -= 1;
            i;
        }
        j = ni + 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < ni {
            while (*sorted_data
                .offset((i as libc::c_ulong).wrapping_mul(stride) as isize))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((ni - j + 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double > trial as libc::c_double
            {
                j -= 1;
                j;
            }
            *q.offset(i as isize) = j;
            i += 1;
            i;
        }
        sump = 0 as libc::c_int as libc::c_long;
        sumq = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < ni {
            sump += *p.offset(i as isize) as libc::c_long;
            sumq += (*q.offset(i as isize) - 1 as libc::c_int) as libc::c_long;
            i += 1;
            i;
        }
        if knew <= sump {
            i = 0 as libc::c_int;
            while i < ni {
                *right.offset(i as isize) = *p.offset(i as isize);
                i += 1;
                i;
            }
            nr = sump;
        } else if knew > sumq {
            i = 0 as libc::c_int;
            while i < ni {
                *left.offset(i as isize) = *q.offset(i as isize);
                i += 1;
                i;
            }
            nl = sumq;
        } else {
            found = 1 as libc::c_int;
        }
    }
    if found != 0 {
        return trial
    } else {
        j = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i < ni {
            let mut jj: libc::c_int = 0;
            jj = *left.offset(i as isize);
            while jj <= *right.offset(i as isize) {
                *work
                    .offset(
                        j as isize,
                    ) = (*sorted_data
                    .offset((i as libc::c_ulong).wrapping_mul(stride) as isize))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((ni - jj) as libc::c_ulong).wrapping_mul(stride) as isize,
                            ),
                    );
                j += 1;
                j;
                jj += 1;
                jj;
            }
            i += 1;
            i;
        }
        knew -= nl + 1 as libc::c_int as libc::c_long;
        gsl_sort_ulong(work, 1 as libc::c_int as size_t, j as size_t);
        return *work.offset(knew as isize);
    };
}
pub unsafe extern "C" fn gsl_stats_ulong_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ulong,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_ulong_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_uint_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_uint,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_uint_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_ushort_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ushort,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_ushort_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_uchar_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_uchar,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_uchar_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_long_double_Qn_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut f128::f128,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = (gsl_stats_long_double_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ))
        .to_f64()
        .unwrap();
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_int_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_int,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_int_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_float_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_float,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_float_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_short_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_short,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_short_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_char_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_char,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_char_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_long_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_long,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_long_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    ) as libc::c_double;
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
pub unsafe extern "C" fn gsl_stats_Qn_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
    mut work_int: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 2.21914f64;
    let mut Qn0: libc::c_double = gsl_stats_Qn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
        work_int,
    );
    let mut dn: libc::c_double = 1.0f64;
    let mut Qn: libc::c_double = 0.;
    if n <= 12 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            dn = 0.399356f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            dn = 0.99365f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            dn = 0.51321f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            dn = 0.84401f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            dn = 0.61220f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            dn = 0.85877f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            dn = 0.66993f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            dn = 0.87344f64;
        } else if n == 10 as libc::c_int as libc::c_ulong {
            dn = 0.72014f64;
        } else if n == 11 as libc::c_int as libc::c_ulong {
            dn = 0.88906f64;
        } else if n == 12 as libc::c_int as libc::c_ulong {
            dn = 0.75743f64;
        }
    } else {
        if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            dn = 1.60188f64
                + (-2.1284f64 - 5.172f64 / n as libc::c_double) / n as libc::c_double;
        } else {
            dn = 3.67561f64
                + (1.9654f64
                    + (6.987f64 - 77.0f64 / n as libc::c_double) / n as libc::c_double)
                    / n as libc::c_double;
        }
        dn = 1.0f64 / (dn / n as libc::c_double + 1.0f64);
    }
    Qn = scale * dn * Qn0;
    return Qn;
}
unsafe extern "C" fn Qn_uint_whimed(
    mut a: *mut libc::c_uint,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_uint,
    mut a_srt: *mut libc::c_uint,
    mut w_cand: *mut libc::c_int,
) -> libc::c_uint {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_uint = 0;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_uint(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_whimed(
    mut a: *mut libc::c_double,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_double,
    mut a_srt: *mut libc::c_double,
    mut w_cand: *mut libc::c_int,
) -> libc::c_double {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_double = 0.;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_int_whimed(
    mut a: *mut libc::c_int,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_int,
    mut a_srt: *mut libc::c_int,
    mut w_cand: *mut libc::c_int,
) -> libc::c_int {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_int = 0;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_int(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_float_whimed(
    mut a: *mut libc::c_float,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_float,
    mut a_srt: *mut libc::c_float,
    mut w_cand: *mut libc::c_int,
) -> libc::c_float {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_float = 0.;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_float(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_long_whimed(
    mut a: *mut libc::c_long,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_long,
    mut a_srt: *mut libc::c_long,
    mut w_cand: *mut libc::c_int,
) -> libc::c_long {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_long = 0;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_long(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_ulong_whimed(
    mut a: *mut libc::c_ulong,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_ulong,
    mut a_srt: *mut libc::c_ulong,
    mut w_cand: *mut libc::c_int,
) -> libc::c_ulong {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_ulong = 0;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_ulong(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_long_double_whimed(
    mut a: *mut f128::f128,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut f128::f128,
    mut a_srt: *mut f128::f128,
    mut w_cand: *mut libc::c_int,
) -> f128::f128 {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: f128::f128 = f128::f128::ZERO;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_long_double(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if *a.offset(i as isize) < trial {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) > trial {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) < trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) > trial {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_char_whimed(
    mut a: *mut libc::c_char,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_char,
    mut a_srt: *mut libc::c_char,
    mut w_cand: *mut libc::c_int,
) -> libc::c_char {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_char = 0;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_char(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if (*a.offset(i as isize) as libc::c_int) < trial as libc::c_int {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) as libc::c_int > trial as libc::c_int {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if (*a.offset(i as isize) as libc::c_int) < trial as libc::c_int {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) as libc::c_int > trial as libc::c_int {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_short_whimed(
    mut a: *mut libc::c_short,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_short,
    mut a_srt: *mut libc::c_short,
    mut w_cand: *mut libc::c_int,
) -> libc::c_short {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_short = 0;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_short(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if (*a.offset(i as isize) as libc::c_int) < trial as libc::c_int {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) as libc::c_int > trial as libc::c_int {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if (*a.offset(i as isize) as libc::c_int) < trial as libc::c_int {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) as libc::c_int > trial as libc::c_int {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_uchar_whimed(
    mut a: *mut libc::c_uchar,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_uchar,
    mut a_srt: *mut libc::c_uchar,
    mut w_cand: *mut libc::c_int,
) -> libc::c_uchar {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_uchar = 0;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_uchar(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if (*a.offset(i as isize) as libc::c_int) < trial as libc::c_int {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) as libc::c_int > trial as libc::c_int {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if (*a.offset(i as isize) as libc::c_int) < trial as libc::c_int {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) as libc::c_int > trial as libc::c_int {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Qn_ushort_whimed(
    mut a: *mut libc::c_ushort,
    mut w: *mut libc::c_int,
    mut n: libc::c_int,
    mut a_cand: *mut libc::c_ushort,
    mut a_srt: *mut libc::c_ushort,
    mut w_cand: *mut libc::c_int,
) -> libc::c_ushort {
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut kcand: libc::c_int = 0;
    let mut wleft: libc::c_long = 0;
    let mut wmid: libc::c_long = 0;
    let mut wright: libc::c_long = 0;
    let mut w_tot: libc::c_long = 0;
    let mut wrest: libc::c_long = 0;
    let mut trial: libc::c_ushort = 0;
    w_tot = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while i < n {
        w_tot += *w.offset(i as isize) as libc::c_long;
        i += 1;
        i;
    }
    wrest = 0 as libc::c_int as libc::c_long;
    loop {
        i = 0 as libc::c_int;
        while i < n {
            *a_srt.offset(i as isize) = *a.offset(i as isize);
            i += 1;
            i;
        }
        n2 = n / 2 as libc::c_int;
        gsl_sort_ushort(a_srt, 1 as libc::c_int as size_t, n as size_t);
        trial = *a_srt.offset(n2 as isize);
        wleft = 0 as libc::c_int as libc::c_long;
        wmid = 0 as libc::c_int as libc::c_long;
        wright = 0 as libc::c_int as libc::c_long;
        i = 0 as libc::c_int;
        while i < n {
            if (*a.offset(i as isize) as libc::c_int) < trial as libc::c_int {
                wleft += *w.offset(i as isize) as libc::c_long;
            } else if *a.offset(i as isize) as libc::c_int > trial as libc::c_int {
                wright += *w.offset(i as isize) as libc::c_long;
            } else {
                wmid += *w.offset(i as isize) as libc::c_long;
            }
            i += 1;
            i;
        }
        kcand = 0 as libc::c_int;
        if 2 as libc::c_int as libc::c_long * (wrest + wleft) > w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if (*a.offset(i as isize) as libc::c_int) < trial as libc::c_int {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
        } else if 2 as libc::c_int as libc::c_long * (wrest + wleft + wmid) <= w_tot {
            i = 0 as libc::c_int;
            while i < n {
                if *a.offset(i as isize) as libc::c_int > trial as libc::c_int {
                    *a_cand.offset(kcand as isize) = *a.offset(i as isize);
                    *w_cand.offset(kcand as isize) = *w.offset(i as isize);
                    kcand += 1;
                    kcand;
                }
                i += 1;
                i;
            }
            wrest += wleft + wmid;
        } else {
            return trial
        }
        n = kcand;
        i = 0 as libc::c_int;
        while i < n {
            *a.offset(i as isize) = *a_cand.offset(i as isize);
            *w.offset(i as isize) = *w_cand.offset(i as isize);
            i += 1;
            i;
        }
    };
}
