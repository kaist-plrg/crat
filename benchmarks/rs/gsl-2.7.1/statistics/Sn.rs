use ::libc;
use ::num_traits;
use ::f128;
use num_traits::ToPrimitive;
extern "C" {
    fn gsl_sort_long_double(data: *mut f128::f128, stride: size_t, n: size_t);
    fn gsl_sort_ushort(data: *mut libc::c_ushort, stride: size_t, n: size_t);
    fn gsl_sort(data: *mut libc::c_double, stride: size_t, n: size_t);
    fn gsl_sort_float(data: *mut libc::c_float, stride: size_t, n: size_t);
    fn gsl_sort_int(data: *mut libc::c_int, stride: size_t, n: size_t);
    fn gsl_sort_ulong(data: *mut libc::c_ulong, stride: size_t, n: size_t);
    fn gsl_sort_char(data: *mut libc::c_char, stride: size_t, n: size_t);
    fn gsl_sort_long(data: *mut libc::c_long, stride: size_t, n: size_t);
    fn gsl_sort_short(data: *mut libc::c_short, stride: size_t, n: size_t);
    fn gsl_sort_uchar(data: *mut libc::c_uchar, stride: size_t, n: size_t);
    fn gsl_sort_uint(data: *mut libc::c_uint, stride: size_t, n: size_t);
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn gsl_stats_ulong_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ulong,
) -> libc::c_ulong {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ))
        .wrapping_sub(*sorted_data.offset(0 as libc::c_int as isize));
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_ulong;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_ulong;
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ))
        .wrapping_sub(
            *sorted_data
                .offset(
                    ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ),
        );
    gsl_sort_ulong(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_uchar_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_uchar,
) -> libc::c_uchar {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) as libc::c_int - *sorted_data.offset(0 as libc::c_int as isize) as libc::c_int)
        as libc::c_uchar;
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_uchar;
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_uchar;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_uchar;
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_uchar;
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) as libc::c_int
        - *sorted_data
            .offset(
                ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            ) as libc::c_int) as libc::c_uchar;
    gsl_sort_uchar(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) - *sorted_data.offset(0 as libc::c_int as isize);
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        );
                medB = *sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        );
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    );
        } else {
            medA = *sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    );
            medB = *sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    );
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = if medA < medB { medA } else { medB };
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = *sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        );
                medB = *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        );
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    );
        } else {
            medA = *sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    );
            medB = *sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    );
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = if medA < medB { medA } else { medB };
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        )
        - *sorted_data
            .offset(
                ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            );
    gsl_sort(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_uint_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_uint,
) -> libc::c_uint {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ))
        .wrapping_sub(*sorted_data.offset(0 as libc::c_int as isize));
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_uint;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ))
                    .wrapping_sub(
                        *sorted_data
                            .offset(
                                ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(stride) as isize,
                            ),
                    ) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ))
                .wrapping_sub(
                    *sorted_data
                        .offset(
                            ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ),
                ) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_uint;
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ))
        .wrapping_sub(
            *sorted_data
                .offset(
                    ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ),
        );
    gsl_sort_uint(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_int_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_int,
) -> libc::c_int {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) - *sorted_data.offset(0 as libc::c_int as isize);
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_int;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_int;
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        )
        - *sorted_data
            .offset(
                ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            );
    gsl_sort_int(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_ushort_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ushort,
) -> libc::c_ushort {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) as libc::c_int - *sorted_data.offset(0 as libc::c_int as isize) as libc::c_int)
        as libc::c_ushort;
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_ushort;
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_ushort;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_ushort;
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_ushort;
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) as libc::c_int
        - *sorted_data
            .offset(
                ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            ) as libc::c_int) as libc::c_ushort;
    gsl_sort_ushort(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_float_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_float,
) -> libc::c_float {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) - *sorted_data.offset(0 as libc::c_int as isize);
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_float;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_float;
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        )
        - *sorted_data
            .offset(
                ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            );
    gsl_sort_float(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_long_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_long,
) -> libc::c_long {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) - *sorted_data.offset(0 as libc::c_int as isize);
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_long;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        )) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_long;
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        )
        - *sorted_data
            .offset(
                ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            );
    gsl_sort_long(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_short_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_short,
) -> libc::c_short {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) as libc::c_int - *sorted_data.offset(0 as libc::c_int as isize) as libc::c_int)
        as libc::c_short;
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_short;
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_short;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_short;
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_short;
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) as libc::c_int
        - *sorted_data
            .offset(
                ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            ) as libc::c_int) as libc::c_short;
    gsl_sort_short(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_char_Sn0_from_sorted_data(
    mut sorted_data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_char,
) -> libc::c_char {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) as libc::c_int - *sorted_data.offset(0 as libc::c_int as isize) as libc::c_int)
        as libc::c_char;
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_char;
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_char;
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ) as libc::c_int) as libc::c_double;
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_char;
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ) as libc::c_int) as libc::c_double;
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                ) as libc::c_int
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ) as libc::c_int) as libc::c_double;
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = (if medA < medB { medA } else { medB }) as libc::c_char;
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (*sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) as libc::c_int
        - *sorted_data
            .offset(
                ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            ) as libc::c_int) as libc::c_char;
    gsl_sort_char(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_long_double_Sn0_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut f128::f128,
) -> f128::f128 {
    let mut medA: libc::c_double = 0.;
    let mut medB: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut Amin: libc::c_int = 0;
    let mut Amax: libc::c_int = 0;
    let mut even: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut leftA: libc::c_int = 0;
    let mut leftB: libc::c_int = 0;
    let mut nA: libc::c_int = 0;
    let mut nB: libc::c_int = 0;
    let mut tryA: libc::c_int = 0;
    let mut tryB: libc::c_int = 0;
    let mut rightA: libc::c_int = 0;
    let mut rightB: libc::c_int = 0;
    let mut np1_2: libc::c_int = n
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    *work
        .offset(
            0 as libc::c_int as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        ) - *sorted_data.offset(0 as libc::c_int as isize);
    i = 2 as libc::c_int;
    while i <= np1_2 {
        nA = i - 1 as libc::c_int;
        nB = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryA + Amin - 2 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ))
                    .to_f64()
                    .unwrap();
                medB = (*sorted_data
                    .offset(
                        ((tryB + i - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ))
                    .to_f64()
                    .unwrap();
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftA + Amin - 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ))
                .to_f64()
                .unwrap();
            medB = (*sorted_data
                .offset(
                    ((leftB + i - 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ))
                .to_f64()
                .unwrap();
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = f128::f128::new(if medA < medB { medA } else { medB });
        }
        i += 1;
        i;
    }
    i = np1_2 + 1 as libc::c_int;
    while i <= n as libc::c_int - 1 as libc::c_int {
        nA = n.wrapping_sub(i as libc::c_ulong) as libc::c_int;
        nB = i - 1 as libc::c_int;
        diff = nB - nA;
        leftB = 1 as libc::c_int;
        leftA = leftB;
        rightB = nB;
        rightA = rightB;
        Amin = diff / 2 as libc::c_int + 1 as libc::c_int;
        Amax = diff / 2 as libc::c_int + nA;
        while leftA < rightA {
            length = rightA - leftA + 1 as libc::c_int;
            even = 1 as libc::c_int - length % 2 as libc::c_int;
            half = (length - 1 as libc::c_int) / 2 as libc::c_int;
            tryA = leftA + half;
            tryB = leftB + half;
            if tryA < Amin {
                rightB = tryB;
                leftA = tryA + even;
            } else if tryA > Amax {
                rightA = tryA;
                leftB = tryB + even;
            } else {
                medA = (*sorted_data
                    .offset(
                        ((i + tryA - Amin) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ))
                    .to_f64()
                    .unwrap();
                medB = (*sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    )
                    - *sorted_data
                        .offset(
                            ((i - tryB - 1 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(stride) as isize,
                        ))
                    .to_f64()
                    .unwrap();
                if medA >= medB {
                    rightA = tryA;
                    leftB = tryB + even;
                } else {
                    rightB = tryB;
                    leftA = tryA + even;
                }
            }
        }
        if leftA > Amax {
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = *sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    );
        } else {
            medA = (*sorted_data
                .offset(
                    ((i + leftA - Amin) as libc::c_ulong).wrapping_mul(stride) as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                            as isize,
                    ))
                .to_f64()
                .unwrap();
            medB = (*sorted_data
                .offset(
                    ((i - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                        as isize,
                )
                - *sorted_data
                    .offset(
                        ((i - leftB - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(stride) as isize,
                    ))
                .to_f64()
                .unwrap();
            *work
                .offset(
                    (i - 1 as libc::c_int) as isize,
                ) = f128::f128::new(if medA < medB { medA } else { medB });
        }
        i += 1;
        i;
    }
    *work
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = *sorted_data
        .offset(
            n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(stride)
                as isize,
        )
        - *sorted_data
            .offset(
                ((np1_2 - 1 as libc::c_int) as libc::c_ulong).wrapping_mul(stride)
                    as isize,
            );
    gsl_sort_long_double(work, 1 as libc::c_int as size_t, n);
    return *work.offset((np1_2 - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn gsl_stats_char_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_char,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_char,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_char_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_uint_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_uint,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_uint,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_uint_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_float_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_float,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_float_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_int_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_int,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_int,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_int_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_ulong_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_ulong,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ulong,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_ulong_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    );
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_uchar_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_uchar,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_uchar,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_uchar_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_long_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_long,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_long,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_long_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_long_double_Sn_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mut work: *mut f128::f128,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = (gsl_stats_long_double_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ))
        .to_f64()
        .unwrap();
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_ushort_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_ushort,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_ushort_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
pub unsafe extern "C" fn gsl_stats_short_Sn_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mut work: *mut libc::c_short,
) -> libc::c_double {
    let scale: libc::c_double = 1.1926f64;
    let mut Sn0: libc::c_double = gsl_stats_short_Sn0_from_sorted_data(
        sorted_data,
        stride,
        n,
        work,
    ) as libc::c_double;
    let mut cn: libc::c_double = 1.0f64;
    let mut Sn: libc::c_double = 0.;
    if n <= 9 as libc::c_int as libc::c_ulong {
        if n == 2 as libc::c_int as libc::c_ulong {
            cn = 0.743f64;
        } else if n == 3 as libc::c_int as libc::c_ulong {
            cn = 1.851f64;
        } else if n == 4 as libc::c_int as libc::c_ulong {
            cn = 0.954f64;
        } else if n == 5 as libc::c_int as libc::c_ulong {
            cn = 1.351f64;
        } else if n == 6 as libc::c_int as libc::c_ulong {
            cn = 0.993f64;
        } else if n == 7 as libc::c_int as libc::c_ulong {
            cn = 1.198f64;
        } else if n == 8 as libc::c_int as libc::c_ulong {
            cn = 1.005f64;
        } else if n == 9 as libc::c_int as libc::c_ulong {
            cn = 1.131f64;
        }
    } else if n.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        cn = n as libc::c_double / (n as libc::c_double - 0.9f64);
    }
    Sn = scale * cn * Sn0;
    return Sn;
}
