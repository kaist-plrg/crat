use ::libc;
extern "C" {
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
unsafe extern "C" fn two_way_long_needle(
    mut haystack: *const libc::c_uchar,
    mut haystack_len: size_t,
    mut needle: *const libc::c_uchar,
    mut needle_len: size_t,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut period: size_t = 0;
    let mut suffix: size_t = 0;
    let mut shift_table: [size_t; 256] = [0; 256];
    suffix = critical_factorization(needle, needle_len, &mut period);
    i = 0 as libc::c_int as size_t;
    while i < ((1 as libc::c_uint) << 8 as libc::c_int) as libc::c_ulong {
        shift_table[i as usize] = needle_len;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < needle_len {
        shift_table[c_tolower(*needle.offset(i as isize) as libc::c_int)
            as usize] = needle_len
            .wrapping_sub(i)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        i = i.wrapping_add(1);
        i;
    }
    if c_strncasecmp(
        needle as *const libc::c_char,
        needle.offset(period as isize) as *const libc::c_char,
        suffix,
    ) == 0 as libc::c_int
    {
        let mut memory: size_t = 0 as libc::c_int as size_t;
        let mut shift: size_t = 0;
        j = 0 as libc::c_int as size_t;
        while (memchr(
            haystack.offset(haystack_len as isize) as *const libc::c_void,
            '\0' as i32,
            j.wrapping_add(needle_len).wrapping_sub(haystack_len),
        ))
            .is_null()
            && {
                haystack_len = j.wrapping_add(needle_len);
                haystack_len != 0
            }
        {
            shift = shift_table[c_tolower(
                *haystack
                    .offset(
                        j
                            .wrapping_add(needle_len)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int,
            ) as usize];
            if (0 as libc::c_int as libc::c_ulong) < shift {
                if memory != 0 && shift < period {
                    shift = needle_len.wrapping_sub(period);
                }
                memory = 0 as libc::c_int as size_t;
                j = (j as libc::c_ulong).wrapping_add(shift) as size_t as size_t;
            } else {
                i = if suffix < memory { memory } else { suffix };
                while i < needle_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_tolower(*needle.offset(i as isize) as libc::c_int)
                        == c_tolower(
                            *haystack.offset(i.wrapping_add(j) as isize) as libc::c_int,
                        )
                {
                    i = i.wrapping_add(1);
                    i;
                }
                if needle_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) <= i {
                    i = suffix.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    while memory < i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        && c_tolower(*needle.offset(i as isize) as libc::c_int)
                            == c_tolower(
                                *haystack.offset(i.wrapping_add(j) as isize) as libc::c_int,
                            )
                    {
                        i = i.wrapping_sub(1);
                        i;
                    }
                    if i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        < memory.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    {
                        return haystack.offset(j as isize) as *mut libc::c_char;
                    }
                    j = (j as libc::c_ulong).wrapping_add(period) as size_t as size_t;
                    memory = needle_len.wrapping_sub(period);
                } else {
                    j = (j as libc::c_ulong)
                        .wrapping_add(
                            i
                                .wrapping_sub(suffix)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                    memory = 0 as libc::c_int as size_t;
                }
            }
        }
    } else {
        let mut shift_0: size_t = 0;
        period = (if suffix < needle_len.wrapping_sub(suffix) {
            needle_len.wrapping_sub(suffix)
        } else {
            suffix
        })
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        j = 0 as libc::c_int as size_t;
        while (memchr(
            haystack.offset(haystack_len as isize) as *const libc::c_void,
            '\0' as i32,
            j.wrapping_add(needle_len).wrapping_sub(haystack_len),
        ))
            .is_null()
            && {
                haystack_len = j.wrapping_add(needle_len);
                haystack_len != 0
            }
        {
            shift_0 = shift_table[c_tolower(
                *haystack
                    .offset(
                        j
                            .wrapping_add(needle_len)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int,
            ) as usize];
            if (0 as libc::c_int as libc::c_ulong) < shift_0 {
                j = (j as libc::c_ulong).wrapping_add(shift_0) as size_t as size_t;
            } else {
                i = suffix;
                while i < needle_len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    && c_tolower(*needle.offset(i as isize) as libc::c_int)
                        == c_tolower(
                            *haystack.offset(i.wrapping_add(j) as isize) as libc::c_int,
                        )
                {
                    i = i.wrapping_add(1);
                    i;
                }
                if needle_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) <= i {
                    i = suffix.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                    while i != 18446744073709551615 as libc::c_ulong
                        && c_tolower(*needle.offset(i as isize) as libc::c_int)
                            == c_tolower(
                                *haystack.offset(i.wrapping_add(j) as isize) as libc::c_int,
                            )
                    {
                        i = i.wrapping_sub(1);
                        i;
                    }
                    if i == 18446744073709551615 as libc::c_ulong {
                        return haystack.offset(j as isize) as *mut libc::c_char;
                    }
                    j = (j as libc::c_ulong).wrapping_add(period) as size_t as size_t;
                } else {
                    j = (j as libc::c_ulong)
                        .wrapping_add(
                            i
                                .wrapping_sub(suffix)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                }
            }
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn two_way_short_needle(
    mut haystack: *const libc::c_uchar,
    mut haystack_len: size_t,
    mut needle: *const libc::c_uchar,
    mut needle_len: size_t,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut period: size_t = 0;
    let mut suffix: size_t = 0;
    suffix = critical_factorization(needle, needle_len, &mut period);
    if c_strncasecmp(
        needle as *const libc::c_char,
        needle.offset(period as isize) as *const libc::c_char,
        suffix,
    ) == 0 as libc::c_int
    {
        let mut memory: size_t = 0 as libc::c_int as size_t;
        j = 0 as libc::c_int as size_t;
        while (memchr(
            haystack.offset(haystack_len as isize) as *const libc::c_void,
            '\0' as i32,
            j.wrapping_add(needle_len).wrapping_sub(haystack_len),
        ))
            .is_null()
            && {
                haystack_len = j.wrapping_add(needle_len);
                haystack_len != 0
            }
        {
            i = if suffix < memory { memory } else { suffix };
            while i < needle_len
                && c_tolower(*needle.offset(i as isize) as libc::c_int)
                    == c_tolower(
                        *haystack.offset(i.wrapping_add(j) as isize) as libc::c_int,
                    )
            {
                i = i.wrapping_add(1);
                i;
            }
            if needle_len <= i {
                i = suffix.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while memory < i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    && c_tolower(*needle.offset(i as isize) as libc::c_int)
                        == c_tolower(
                            *haystack.offset(i.wrapping_add(j) as isize) as libc::c_int,
                        )
                {
                    i = i.wrapping_sub(1);
                    i;
                }
                if i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    < memory.wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    return haystack.offset(j as isize) as *mut libc::c_char;
                }
                j = (j as libc::c_ulong).wrapping_add(period) as size_t as size_t;
                memory = needle_len.wrapping_sub(period);
            } else {
                j = (j as libc::c_ulong)
                    .wrapping_add(
                        i
                            .wrapping_sub(suffix)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
                memory = 0 as libc::c_int as size_t;
            }
        }
    } else {
        period = (if suffix < needle_len.wrapping_sub(suffix) {
            needle_len.wrapping_sub(suffix)
        } else {
            suffix
        })
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        j = 0 as libc::c_int as size_t;
        while (memchr(
            haystack.offset(haystack_len as isize) as *const libc::c_void,
            '\0' as i32,
            j.wrapping_add(needle_len).wrapping_sub(haystack_len),
        ))
            .is_null()
            && {
                haystack_len = j.wrapping_add(needle_len);
                haystack_len != 0
            }
        {
            i = suffix;
            while i < needle_len
                && c_tolower(*needle.offset(i as isize) as libc::c_int)
                    == c_tolower(
                        *haystack.offset(i.wrapping_add(j) as isize) as libc::c_int,
                    )
            {
                i = i.wrapping_add(1);
                i;
            }
            if needle_len <= i {
                i = suffix.wrapping_sub(1 as libc::c_int as libc::c_ulong);
                while i != 18446744073709551615 as libc::c_ulong
                    && c_tolower(*needle.offset(i as isize) as libc::c_int)
                        == c_tolower(
                            *haystack.offset(i.wrapping_add(j) as isize) as libc::c_int,
                        )
                {
                    i = i.wrapping_sub(1);
                    i;
                }
                if i == 18446744073709551615 as libc::c_ulong {
                    return haystack.offset(j as isize) as *mut libc::c_char;
                }
                j = (j as libc::c_ulong).wrapping_add(period) as size_t as size_t;
            } else {
                j = (j as libc::c_ulong)
                    .wrapping_add(
                        i
                            .wrapping_sub(suffix)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
            }
        }
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn critical_factorization(
    mut needle: *const libc::c_uchar,
    mut needle_len: size_t,
    mut period: *mut size_t,
) -> size_t {
    let mut max_suffix: size_t = 0;
    let mut max_suffix_rev: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut p: size_t = 0;
    let mut a: libc::c_uchar = 0;
    let mut b: libc::c_uchar = 0;
    if needle_len < 3 as libc::c_int as libc::c_ulong {
        *period = 1 as libc::c_int as size_t;
        return needle_len.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    }
    max_suffix = 18446744073709551615 as libc::c_ulong;
    j = 0 as libc::c_int as size_t;
    p = 1 as libc::c_int as size_t;
    k = p;
    while j.wrapping_add(k) < needle_len {
        a = c_tolower(*needle.offset(j.wrapping_add(k) as isize) as libc::c_int)
            as libc::c_uchar;
        b = c_tolower(*needle.offset(max_suffix.wrapping_add(k) as isize) as libc::c_int)
            as libc::c_uchar;
        if (a as libc::c_int) < b as libc::c_int {
            j = (j as libc::c_ulong).wrapping_add(k) as size_t as size_t;
            k = 1 as libc::c_int as size_t;
            p = j.wrapping_sub(max_suffix);
        } else if a as libc::c_int == b as libc::c_int {
            if k != p {
                k = k.wrapping_add(1);
                k;
            } else {
                j = (j as libc::c_ulong).wrapping_add(p) as size_t as size_t;
                k = 1 as libc::c_int as size_t;
            }
        } else {
            let fresh0 = j;
            j = j.wrapping_add(1);
            max_suffix = fresh0;
            p = 1 as libc::c_int as size_t;
            k = p;
        }
    }
    *period = p;
    max_suffix_rev = 18446744073709551615 as libc::c_ulong;
    j = 0 as libc::c_int as size_t;
    p = 1 as libc::c_int as size_t;
    k = p;
    while j.wrapping_add(k) < needle_len {
        a = c_tolower(*needle.offset(j.wrapping_add(k) as isize) as libc::c_int)
            as libc::c_uchar;
        b = c_tolower(
            *needle.offset(max_suffix_rev.wrapping_add(k) as isize) as libc::c_int,
        ) as libc::c_uchar;
        if (b as libc::c_int) < a as libc::c_int {
            j = (j as libc::c_ulong).wrapping_add(k) as size_t as size_t;
            k = 1 as libc::c_int as size_t;
            p = j.wrapping_sub(max_suffix_rev);
        } else if a as libc::c_int == b as libc::c_int {
            if k != p {
                k = k.wrapping_add(1);
                k;
            } else {
                j = (j as libc::c_ulong).wrapping_add(p) as size_t as size_t;
                k = 1 as libc::c_int as size_t;
            }
        } else {
            let fresh1 = j;
            j = j.wrapping_add(1);
            max_suffix_rev = fresh1;
            p = 1 as libc::c_int as size_t;
            k = p;
        }
    }
    if max_suffix_rev.wrapping_add(1 as libc::c_int as libc::c_ulong)
        < max_suffix.wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        return max_suffix.wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    *period = p;
    return max_suffix_rev.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
pub unsafe extern "C" fn c_strcasestr(
    mut haystack_start: *const libc::c_char,
    mut needle_start: *const libc::c_char,
) -> *mut libc::c_char {
    let mut haystack: *const libc::c_char = haystack_start;
    let mut needle: *const libc::c_char = needle_start;
    let mut needle_len: size_t = 0;
    let mut haystack_len: size_t = 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    while *haystack as libc::c_int != 0 && *needle as libc::c_int != 0 {
        let fresh2 = haystack;
        haystack = haystack.offset(1);
        let fresh3 = needle;
        needle = needle.offset(1);
        ok = (ok as libc::c_int
            & (c_tolower(*fresh2 as libc::c_uchar as libc::c_int)
                == c_tolower(*fresh3 as libc::c_uchar as libc::c_int)) as libc::c_int) != 0;
    }
    if *needle != 0 {
        return 0 as *mut libc::c_char;
    }
    if ok {
        return haystack_start as *mut libc::c_char;
    }
    needle_len = needle.offset_from(needle_start) as libc::c_long as size_t;
    haystack = haystack_start.offset(1 as libc::c_int as isize);
    haystack_len = needle_len.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if needle_len < 32 as libc::c_uint as libc::c_ulong {
        return two_way_short_needle(
            haystack as *const libc::c_uchar,
            haystack_len,
            needle_start as *const libc::c_uchar,
            needle_len,
        );
    }
    return two_way_long_needle(
        haystack as *const libc::c_uchar,
        haystack_len,
        needle_start as *const libc::c_uchar,
        needle_len,
    );
}
