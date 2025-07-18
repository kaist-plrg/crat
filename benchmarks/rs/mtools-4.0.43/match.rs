use ::libc;
extern "C" {
    fn towlower(__wc: wint_t) -> wint_t;
    fn towupper(__wc: wint_t) -> wint_t;
}
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
unsafe extern "C" fn casecmp(mut a: wchar_t, mut b: wchar_t) -> libc::c_int {
    return (towupper(a as wint_t) == towupper(b as wint_t)) as libc::c_int;
}
unsafe extern "C" fn exactcmp(mut a: wchar_t, mut b: wchar_t) -> libc::c_int {
    return (a == b) as libc::c_int;
}
unsafe extern "C" fn is_in_range(
    mut ch: wchar_t,
    mut p: *mut *const wchar_t,
    mut reverse: *mut libc::c_int,
) -> libc::c_int {
    let mut first: wchar_t = 0;
    let mut last: wchar_t = 0;
    let mut found: libc::c_int = 0 as libc::c_int;
    if **p == '^' as i32 {
        *reverse = 1 as libc::c_int;
        *p = (*p).offset(1);
        *p;
    } else {
        *reverse = 0 as libc::c_int;
    }
    loop {
        first = **p;
        if !(first != ']' as i32) {
            break;
        }
        if first == 0 {
            return 0 as libc::c_int;
        }
        *p = (*p).offset(1);
        if **p == '-' as i32 {
            *p = (*p).offset(1);
            last = **p;
            if last == ']' as i32 {
                if ch == first || ch == '-' as i32 {
                    found = 1 as libc::c_int;
                }
                break;
            } else {
                *p = (*p).offset(1);
                *p;
                if ch >= first && ch <= last {
                    found = 1 as libc::c_int;
                }
            }
        } else if ch == first {
            found = 1 as libc::c_int;
        }
    }
    return found;
}
unsafe extern "C" fn parse_range(
    mut p: *mut *const wchar_t,
    mut s: *const wchar_t,
    mut out: *mut wchar_t,
    mut compfn: Option::<unsafe extern "C" fn(wchar_t, wchar_t) -> libc::c_int>,
) -> libc::c_int {
    let mut reverse: libc::c_int = 0;
    let mut p0: *const wchar_t = *p;
    let mut p1: *const wchar_t = *p;
    if !out.is_null() {
        *out = *s;
    }
    if is_in_range(*s, p, &mut reverse) != 0 {
        return 1 as libc::c_int ^ reverse;
    }
    if compfn == Some(exactcmp as unsafe extern "C" fn(wchar_t, wchar_t) -> libc::c_int)
    {
        return reverse;
    }
    if is_in_range(towlower(*s as wint_t) as wchar_t, &mut p0, &mut reverse) != 0 {
        if !out.is_null() {
            *out = towlower(*s as wint_t) as wchar_t;
        }
        return 1 as libc::c_int ^ reverse;
    }
    if is_in_range(towupper(*s as wint_t) as wchar_t, &mut p1, &mut reverse) != 0 {
        if !out.is_null() {
            *out = towupper(*s as wint_t) as wchar_t;
        }
        return 1 as libc::c_int ^ reverse;
    }
    return reverse;
}
unsafe extern "C" fn _match(
    mut s: *const wchar_t,
    mut p: *const wchar_t,
    mut out: *mut wchar_t,
    mut Case: libc::c_int,
    mut length: libc::c_int,
    mut compfn: Option::<unsafe extern "C" fn(wchar_t, wchar_t) -> libc::c_int>,
) -> libc::c_int {
    let mut current_block_27: u64;
    while *p != '\0' as i32 && length != 0 {
        match *p {
            63 => {
                if *s == '\0' as i32 {
                    return 0 as libc::c_int;
                }
                if !out.is_null() {
                    let fresh0 = out;
                    out = out.offset(1);
                    *fresh0 = *s;
                }
                current_block_27 = 10043043949733653460;
            }
            42 => {
                while *p == '*' as i32 && length != 0 {
                    p = p.offset(1);
                    p;
                    length -= 1;
                    length;
                }
                while *s != 0 {
                    if _match(s, p, out, Case, length, compfn) != 0 {
                        return 1 as libc::c_int;
                    }
                    if !out.is_null() {
                        let fresh1 = out;
                        out = out.offset(1);
                        *fresh1 = *s;
                    }
                    s = s.offset(1);
                    s;
                }
                continue;
            }
            91 => {
                p = p.offset(1);
                p;
                length -= 1;
                length;
                let fresh2 = out;
                out = out.offset(1);
                if parse_range(&mut p, s, fresh2, compfn) == 0 {
                    return 0 as libc::c_int;
                }
                current_block_27 = 10043043949733653460;
            }
            92 => {
                p = p.offset(1);
                p;
                length -= 1;
                length;
                current_block_27 = 3831307367355215266;
            }
            _ => {
                current_block_27 = 3831307367355215266;
            }
        }
        match current_block_27 {
            3831307367355215266 => {
                if compfn.unwrap()(*s, *p) == 0 {
                    return 0 as libc::c_int;
                }
                if !out.is_null() {
                    let fresh3 = out;
                    out = out.offset(1);
                    *fresh3 = *p;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
        length -= 1;
        length;
        s = s.offset(1);
        s;
    }
    if !out.is_null() {
        *out = '\0' as i32;
    }
    if *s != '\0' as i32 { return 0 as libc::c_int } else { return 1 as libc::c_int };
}
#[export_name = "match"]
pub unsafe extern "C" fn match_0(
    mut s: *const wchar_t,
    mut p: *const wchar_t,
    mut out: *mut wchar_t,
    mut Case: libc::c_int,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut compfn: Option::<unsafe extern "C" fn(wchar_t, wchar_t) -> libc::c_int> = None;
    if Case != 0 {
        compfn = Some(casecmp as unsafe extern "C" fn(wchar_t, wchar_t) -> libc::c_int);
    } else {
        compfn = Some(casecmp as unsafe extern "C" fn(wchar_t, wchar_t) -> libc::c_int);
    }
    return _match(s, p, out, Case, length, compfn);
}
