use ::libc;
extern "C" {
    pub type doscp_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn wcscmp(_: *const libc::c_int, _: *const libc::c_int) -> libc::c_int;
    fn iswcntrl(__wc: wint_t) -> libc::c_int;
    fn iswlower(__wc: wint_t) -> libc::c_int;
    fn iswupper(__wc: wint_t) -> libc::c_int;
    fn towupper(__wc: wint_t) -> wint_t;
    fn _basename(filename: *const libc::c_char) -> *const libc::c_char;
    static mut mtools_ignore_short_case: libc::c_uint;
    static mut mtools_no_vfat: libc::c_uint;
    fn autorename_short(_: *mut dos_name_t, _: libc::c_int);
    fn dos_to_wchar(
        fromDos: *mut doscp_t,
        dos: *const libc::c_char,
        wchar: *mut wchar_t,
        len: size_t,
    ) -> size_t;
    fn wchar_to_dos(
        toDos: *mut doscp_t,
        wchar: *mut wchar_t,
        dos: *mut libc::c_char,
        len: size_t,
        mangled: *mut libc::c_int,
    );
    fn wchar_to_native(
        wchar: *const wchar_t,
        native: *mut libc::c_char,
        len: size_t,
        out_len: size_t,
    ) -> size_t;
    fn native_to_wchar(
        native: *const libc::c_char,
        wchar: *mut wchar_t,
        len: size_t,
        end: *const libc::c_char,
        mangled: *mut libc::c_int,
    ) -> size_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type size_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type wchar_t = libc::c_int;
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dos_name_t {
    pub base: [libc::c_char; 8],
    pub ext: [libc::c_char; 3],
    pub sentinel: libc::c_char,
}
pub const LOWER: Case_l = 2;
pub type Case_t = Case_l;
pub type Case_l = libc::c_uint;
pub const UPPER: Case_l = 1;
pub const NONE: Case_l = 0;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn ch_tolower(mut ch: libc::c_char) -> libc::c_char {
    return ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = ch as libc::c_uchar as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(ch as libc::c_uchar as libc::c_int);
            }
        } else {
            __res = *(*__ctype_tolower_loc())
                .offset(ch as libc::c_uchar as libc::c_int as isize);
        }
        __res
    }) as libc::c_char;
}
#[inline]
unsafe extern "C" fn ch_towupper(mut ch: wchar_t) -> wchar_t {
    return towupper(ch as wint_t) as wchar_t;
}
pub unsafe extern "C" fn unix_normalize(
    mut cp: *mut doscp_t,
    mut ans: *mut libc::c_char,
    mut dn: *mut dos_name_t,
    mut ans_size: size_t,
) -> *mut libc::c_char {
    let mut buffer: [libc::c_char; 13] = [0; 13];
    let mut wbuffer: [wchar_t; 13] = [0; 13];
    let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    a = buffer.as_mut_ptr();
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int && (*dn).base[j as usize] as libc::c_int > ' ' as i32 {
        *a = (*dn).base[j as usize];
        j += 1;
        j;
        a = a.offset(1);
        a;
    }
    if (*dn).ext[0 as libc::c_int as usize] as libc::c_int > ' ' as i32 {
        let fresh0 = a;
        a = a.offset(1);
        *fresh0 = '.' as i32 as libc::c_char;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int && (*dn).ext[j as usize] as libc::c_int > ' ' as i32 {
            *a = (*dn).ext[j as usize];
            j += 1;
            j;
            a = a.offset(1);
            a;
        }
    }
    let fresh1 = a;
    a = a.offset(1);
    *fresh1 = '\0' as i32 as libc::c_char;
    dos_to_wchar(
        cp,
        buffer.as_mut_ptr(),
        wbuffer.as_mut_ptr(),
        13 as libc::c_int as size_t,
    );
    wchar_to_native(wbuffer.as_mut_ptr(), ans, 13 as libc::c_int as size_t, ans_size);
    return ans;
}
unsafe extern "C" fn TranslateToDos(
    mut toDos: *mut doscp_t,
    mut in_0: *const libc::c_char,
    mut out: *mut libc::c_char,
    mut count: size_t,
    mut end: *mut libc::c_char,
    mut Case: *mut Case_t,
    mut mangled: *mut libc::c_int,
) {
    let mut buffer: [wchar_t; 12] = [0; 12];
    let mut s: *mut wchar_t = buffer.as_mut_ptr();
    let mut t_idx: size_t = 0 as libc::c_int as size_t;
    native_to_wchar(in_0, buffer.as_mut_ptr(), count, end, mangled);
    buffer[count as usize] = '\0' as i32;
    *Case = NONE;
    while *s != 0 {
        if *s == ' ' as i32 || *s == '.' as i32 {
            *mangled |= 3 as libc::c_int;
        } else {
            if iswcntrl(*s as wint_t) != 0 {
                *mangled |= 3 as libc::c_int;
                buffer[t_idx as usize] = '_' as i32;
            } else if iswlower(*s as wint_t) != 0 {
                buffer[t_idx as usize] = ch_towupper(*s);
                if *Case as libc::c_uint == UPPER as libc::c_int as libc::c_uint
                    && mtools_no_vfat == 0
                {
                    *mangled |= 1 as libc::c_int;
                } else {
                    *Case = LOWER;
                }
            } else if iswupper(*s as wint_t) != 0 {
                buffer[t_idx as usize] = *s;
                if *Case as libc::c_uint == LOWER as libc::c_int as libc::c_uint
                    && mtools_no_vfat == 0
                {
                    *mangled |= 1 as libc::c_int;
                } else {
                    *Case = UPPER;
                }
            } else {
                buffer[t_idx as usize] = *s;
            }
            t_idx = t_idx.wrapping_add(1);
            t_idx;
        }
        s = s.offset(1);
        s;
    }
    wchar_to_dos(toDos, buffer.as_mut_ptr(), out, t_idx, mangled);
}
pub unsafe extern "C" fn dos_name(
    mut toDos: *mut doscp_t,
    mut name: *const libc::c_char,
    mut verbose: libc::c_int,
    mut mangled: *mut libc::c_int,
    mut dn: *mut dos_name_t,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut BaseCase: Case_t = NONE;
    let mut ExtCase: Case_t = UPPER;
    *mangled = 0 as libc::c_int;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int != 0
        && *name.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
    {
        name = &*name.offset(2 as libc::c_int as isize) as *const libc::c_char;
    }
    name = _basename(name);
    s = strrchr(name, '\\' as i32);
    if !s.is_null() {
        name = s.offset(1 as libc::c_int as isize);
    }
    memset(dn as *mut libc::c_void, ' ' as i32, 11 as libc::c_int as libc::c_ulong);
    i = strspn(name, b". \0" as *const u8 as *const libc::c_char);
    if i != 0 {
        name = name.offset(i as isize);
        *mangled = 3 as libc::c_int;
    }
    ext = strrchr(name, '.' as i32);
    TranslateToDos(
        toDos,
        name,
        ((*dn).base).as_mut_ptr(),
        8 as libc::c_int as size_t,
        ext,
        &mut BaseCase,
        mangled,
    );
    if !ext.is_null() {
        TranslateToDos(
            toDos,
            ext.offset(1 as libc::c_int as isize),
            ((*dn).ext).as_mut_ptr(),
            3 as libc::c_int as size_t,
            0 as *mut libc::c_char,
            &mut ExtCase,
            mangled,
        );
    }
    if *mangled & 2 as libc::c_int != 0 {
        autorename_short(dn, 0 as libc::c_int);
    }
    if *mangled == 0 {
        if BaseCase as libc::c_uint == LOWER as libc::c_int as libc::c_uint {
            *mangled |= 0x8 as libc::c_int;
        }
        if ExtCase as libc::c_uint == LOWER as libc::c_int as libc::c_uint {
            *mangled |= 0x10 as libc::c_int;
        }
    }
}
pub unsafe extern "C" fn unix_name(
    mut dosCp: *mut doscp_t,
    mut base: *const libc::c_char,
    mut ext: *const libc::c_char,
    mut Case: uint8_t,
    mut ret: *mut wchar_t,
) -> *mut wchar_t {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tname: [libc::c_char; 9] = [0; 9];
    let mut text: [libc::c_char; 4] = [0; 4];
    let mut ans: [libc::c_char; 13] = [0; 13];
    let mut i: libc::c_int = 0;
    strncpy(tname.as_mut_ptr(), base, 8 as libc::c_int as libc::c_ulong);
    tname[8 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    s = strchr(tname.as_mut_ptr(), ' ' as i32);
    if !s.is_null() {
        *s = '\0' as i32 as libc::c_char;
    }
    if tname[0 as libc::c_int as usize] as libc::c_int == '\u{5}' as i32 {
        tname[0 as libc::c_int as usize] = -27i32 as libc::c_char;
    }
    if Case as libc::c_int & (0x8 as libc::c_int | 0x10 as libc::c_int) == 0
        && mtools_ignore_short_case != 0
    {
        Case = (Case as libc::c_int | (0x8 as libc::c_int | 0x10 as libc::c_int))
            as uint8_t;
    }
    if Case as libc::c_int & 0x8 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int && tname[i as usize] as libc::c_int != 0 {
            tname[i as usize] = ch_tolower(tname[i as usize]);
            i += 1;
            i;
        }
    }
    strncpy(text.as_mut_ptr(), ext, 3 as libc::c_int as libc::c_ulong);
    text[3 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    s = strchr(text.as_mut_ptr(), ' ' as i32);
    if !s.is_null() {
        *s = '\0' as i32 as libc::c_char;
    }
    if Case as libc::c_int & 0x10 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int && text[i as usize] as libc::c_int != 0 {
            text[i as usize] = ch_tolower(text[i as usize]);
            i += 1;
            i;
        }
    }
    if *text.as_mut_ptr() != 0 {
        strcpy(ans.as_mut_ptr(), tname.as_mut_ptr());
        strcat(ans.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
        strcat(ans.as_mut_ptr(), text.as_mut_ptr());
    } else {
        strcpy(ans.as_mut_ptr(), tname.as_mut_ptr());
    }
    dos_to_wchar(dosCp, ans.as_mut_ptr(), ret, 12 as libc::c_int as size_t);
    return ret;
}
pub unsafe extern "C" fn isSpecial(mut name: *const libc::c_char) -> libc::c_int {
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return 1 as libc::c_int;
    }
    if strcmp(name, b".\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int;
    }
    if strcmp(name, b"..\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn isSpecialW(mut name: *const wchar_t) -> libc::c_int {
    if *name.offset(0 as libc::c_int as isize) == '\0' as i32 {
        return 1 as libc::c_int;
    }
    if wcscmp(
        name,
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_int; 2]>(b".\0\0\0\0\0\0\0"))
            .as_ptr(),
    ) == 0
    {
        return 1 as libc::c_int;
    }
    if wcscmp(
        name,
        (*::std::mem::transmute::<
            &[u8; 12],
            &[libc::c_int; 3],
        >(b".\0\0\0.\0\0\0\0\0\0\0"))
            .as_ptr(),
    ) == 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
