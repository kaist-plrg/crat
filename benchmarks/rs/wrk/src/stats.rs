use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn powl(_: f128::f128, _: f128::f128) -> f128::f128;
    fn sqrtl(_: f128::f128) -> f128::f128;
    fn round(_: libc::c_double) -> libc::c_double;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub count: uint64_t,
    pub limit: uint64_t,
    pub min: uint64_t,
    pub max: uint64_t,
    pub data: [uint64_t; 0],
}
pub unsafe extern "C" fn stats_alloc(mut max: uint64_t) -> *mut stats {
    let mut limit: uint64_t = max.wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut s: *mut stats = zcalloc(
        (::std::mem::size_of::<stats>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<uint64_t>() as libc::c_ulong).wrapping_mul(limit),
            ),
    ) as *mut stats;
    (*s).limit = limit;
    (*s).min = 18446744073709551615 as libc::c_ulong;
    return s;
}
pub unsafe extern "C" fn stats_free(mut stats: *mut stats) {
    zfree(stats as *mut libc::c_void);
}
pub unsafe extern "C" fn stats_record(
    mut stats: *mut stats,
    mut n: uint64_t,
) -> libc::c_int {
    if n >= (*stats).limit {
        return 0 as libc::c_int;
    }
    ::std::intrinsics::atomic_xadd::<_, { std::intrinsics::AtomicOrdering::SeqCst }>(
        &mut *((*stats).data).as_mut_ptr().offset(n as isize) as *mut uint64_t,
        1 as libc::c_int as uint64_t,
    );
    ::std::intrinsics::atomic_xadd::<_, { std::intrinsics::AtomicOrdering::SeqCst }>(
        &mut (*stats).count,
        1 as libc::c_int as uint64_t,
    );
    let mut min: uint64_t = (*stats).min;
    let mut max: uint64_t = (*stats).max;
    while n < min {
        min = (::std::intrinsics::atomic_cxchg::<_, { std::intrinsics::AtomicOrdering::SeqCst }, { std::intrinsics::AtomicOrdering::SeqCst }>(&mut (*stats).min, min, n))
            .0;
    }
    while n > max {
        max = (::std::intrinsics::atomic_cxchg::<_, { std::intrinsics::AtomicOrdering::SeqCst }, { std::intrinsics::AtomicOrdering::SeqCst }>(&mut (*stats).max, max, n))
            .0;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn stats_correct(mut stats: *mut stats, mut expected: int64_t) {
    let mut n: uint64_t = (expected * 2 as libc::c_int as libc::c_long) as uint64_t;
    while n <= (*stats).max {
        let mut count: uint64_t = *((*stats).data).as_mut_ptr().offset(n as isize);
        let mut m: int64_t = n as int64_t - expected;
        while count != 0 && m > expected {
            let ref mut fresh0 = *((*stats).data).as_mut_ptr().offset(m as isize);
            *fresh0 = (*fresh0 as libc::c_ulong).wrapping_add(count) as uint64_t
                as uint64_t;
            (*stats)
                .count = ((*stats).count as libc::c_ulong).wrapping_add(count)
                as uint64_t as uint64_t;
            m -= expected;
        }
        n = n.wrapping_add(1);
        n;
    }
}
pub unsafe extern "C" fn stats_mean(mut stats: *mut stats) -> f128::f128 {
    if (*stats).count == 0 as libc::c_int as libc::c_ulong {
        return f128::f128::new(0.0f64);
    }
    let mut sum: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint64_t = (*stats).min;
    while i <= (*stats).max {
        sum = (sum as libc::c_ulong)
            .wrapping_add(
                (*((*stats).data).as_mut_ptr().offset(i as isize)).wrapping_mul(i),
            ) as uint64_t as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
    return f128::f128::new(sum) / f128::f128::new((*stats).count);
}
pub unsafe extern "C" fn stats_stdev(
    mut stats: *mut stats,
    mut mean: f128::f128,
) -> f128::f128 {
    let mut sum: f128::f128 = f128::f128::new(0.0f64);
    if (*stats).count < 2 as libc::c_int as libc::c_ulong {
        return f128::f128::new(0.0f64);
    }
    let mut i: uint64_t = (*stats).min;
    while i <= (*stats).max {
        if *((*stats).data).as_mut_ptr().offset(i as isize) != 0 {
            sum
                += powl(f128::f128::new(i) - mean, f128::f128::new(2 as libc::c_int))
                    * f128::f128::new(*((*stats).data).as_mut_ptr().offset(i as isize));
        }
        i = i.wrapping_add(1);
        i;
    }
    return sqrtl(
        sum
            / f128::f128::new(
                ((*stats).count).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ),
    );
}
pub unsafe extern "C" fn stats_within_stdev(
    mut stats: *mut stats,
    mut mean: f128::f128,
    mut stdev: f128::f128,
    mut n: uint64_t,
) -> f128::f128 {
    let mut upper: f128::f128 = mean + stdev * f128::f128::new(n);
    let mut lower: f128::f128 = mean - stdev * f128::f128::new(n);
    let mut sum: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint64_t = (*stats).min;
    while i <= (*stats).max {
        if f128::f128::new(i) >= lower && f128::f128::new(i) <= upper {
            sum = (sum as libc::c_ulong)
                .wrapping_add(*((*stats).data).as_mut_ptr().offset(i as isize))
                as uint64_t as uint64_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return f128::f128::new(sum) / f128::f128::new((*stats).count)
        * f128::f128::new(100 as libc::c_int);
}
pub unsafe extern "C" fn stats_percentile(
    mut stats: *mut stats,
    mut p: f128::f128,
) -> uint64_t {
    let mut rank: uint64_t = round(
        (p / f128::f128::new(100.0f64) * f128::f128::new((*stats).count)
            + f128::f128::new(0.5f64))
            .to_f64()
            .unwrap(),
    ) as uint64_t;
    let mut total: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint64_t = (*stats).min;
    while i <= (*stats).max {
        total = (total as libc::c_ulong)
            .wrapping_add(*((*stats).data).as_mut_ptr().offset(i as isize)) as uint64_t
            as uint64_t;
        if total >= rank {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int as uint64_t;
}
pub unsafe extern "C" fn stats_popcount(mut stats: *mut stats) -> uint64_t {
    let mut count: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint64_t = (*stats).min;
    while i <= (*stats).max {
        if *((*stats).data).as_mut_ptr().offset(i as isize) != 0 {
            count = count.wrapping_add(1);
            count;
        }
        i = i.wrapping_add(1);
        i;
    }
    return count;
}
pub unsafe extern "C" fn stats_value_at(
    mut stats: *mut stats,
    mut index: uint64_t,
    mut count: *mut uint64_t,
) -> uint64_t {
    *count = 0 as libc::c_int as uint64_t;
    let mut i: uint64_t = (*stats).min;
    while i <= (*stats).max {
        if *((*stats).data).as_mut_ptr().offset(i as isize) != 0
            && {
                let fresh1 = *count;
                *count = (*count).wrapping_add(1);
                fresh1 == index
            }
        {
            *count = *((*stats).data).as_mut_ptr().offset(i as isize);
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int as uint64_t;
}
