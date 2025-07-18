use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_string {
    pub d: *mut libc::c_char,
    pub n: size_t,
    pub i: size_t,
}
unsafe extern "C" fn utstring_findR(
    mut s: *mut UT_string,
    mut P_StartPosition: libc::c_long,
    mut P_Needle: *const libc::c_char,
    mut P_NeedleLen: size_t,
) -> libc::c_long {
    let mut V_StartPosition: libc::c_long = 0;
    let mut V_HaystackLen: libc::c_long = 0;
    let mut V_KMP_Table: *mut libc::c_long = 0 as *mut libc::c_long;
    let mut V_FindPosition: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    if P_StartPosition < 0 as libc::c_int as libc::c_long {
        V_StartPosition = ((*s).i).wrapping_add(P_StartPosition as libc::c_ulong)
            as libc::c_long;
    } else {
        V_StartPosition = P_StartPosition;
    }
    V_HaystackLen = V_StartPosition + 1 as libc::c_int as libc::c_long;
    if V_HaystackLen >= P_NeedleLen as libc::c_long
        && P_NeedleLen > 0 as libc::c_int as libc::c_ulong
    {
        V_KMP_Table = malloc(
            (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_mul(
                    P_NeedleLen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut libc::c_long;
        if !V_KMP_Table.is_null() {
            _utstring_BuildTableR(P_Needle, P_NeedleLen, V_KMP_Table);
            V_FindPosition = _utstring_findR(
                (*s).d,
                V_HaystackLen as size_t,
                P_Needle,
                P_NeedleLen,
                V_KMP_Table,
            );
            free(V_KMP_Table as *mut libc::c_void);
        }
    }
    return V_FindPosition;
}
unsafe extern "C" fn utstring_find(
    mut s: *mut UT_string,
    mut P_StartPosition: libc::c_long,
    mut P_Needle: *const libc::c_char,
    mut P_NeedleLen: size_t,
) -> libc::c_long {
    let mut V_StartPosition: libc::c_long = 0;
    let mut V_HaystackLen: libc::c_long = 0;
    let mut V_KMP_Table: *mut libc::c_long = 0 as *mut libc::c_long;
    let mut V_FindPosition: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    if P_StartPosition < 0 as libc::c_int as libc::c_long {
        V_StartPosition = ((*s).i).wrapping_add(P_StartPosition as libc::c_ulong)
            as libc::c_long;
    } else {
        V_StartPosition = P_StartPosition;
    }
    V_HaystackLen = ((*s).i).wrapping_sub(V_StartPosition as libc::c_ulong)
        as libc::c_long;
    if V_HaystackLen >= P_NeedleLen as libc::c_long
        && P_NeedleLen > 0 as libc::c_int as libc::c_ulong
    {
        V_KMP_Table = malloc(
            (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_mul(
                    P_NeedleLen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut libc::c_long;
        if !V_KMP_Table.is_null() {
            _utstring_BuildTable(P_Needle, P_NeedleLen, V_KMP_Table);
            V_FindPosition = _utstring_find(
                ((*s).d).offset(V_StartPosition as isize),
                V_HaystackLen as size_t,
                P_Needle,
                P_NeedleLen,
                V_KMP_Table,
            );
            if V_FindPosition >= 0 as libc::c_int as libc::c_long {
                V_FindPosition += V_StartPosition;
            }
            free(V_KMP_Table as *mut libc::c_void);
        }
    }
    return V_FindPosition;
}
unsafe extern "C" fn _utstring_findR(
    mut P_Haystack: *const libc::c_char,
    mut P_HaystackLen: size_t,
    mut P_Needle: *const libc::c_char,
    mut P_NeedleLen: size_t,
    mut P_KMP_Table: *mut libc::c_long,
) -> libc::c_long {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut V_FindPosition: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    j = P_HaystackLen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_long;
    i = P_NeedleLen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_long;
    while j >= 0 as libc::c_int as libc::c_long && j >= i {
        while i < P_NeedleLen as libc::c_int as libc::c_long
            && *P_Needle.offset(i as isize) as libc::c_int
                != *P_Haystack.offset(j as isize) as libc::c_int
        {
            i = *P_KMP_Table.offset((i + 1 as libc::c_int as libc::c_long) as isize);
        }
        i -= 1;
        i;
        j -= 1;
        j;
        if !(i < 0 as libc::c_int as libc::c_long) {
            continue;
        }
        V_FindPosition = j + 1 as libc::c_int as libc::c_long;
        break;
    }
    return V_FindPosition;
}
unsafe extern "C" fn _utstring_find(
    mut P_Haystack: *const libc::c_char,
    mut P_HaystackLen: size_t,
    mut P_Needle: *const libc::c_char,
    mut P_NeedleLen: size_t,
    mut P_KMP_Table: *mut libc::c_long,
) -> libc::c_long {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut V_FindPosition: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    j = 0 as libc::c_int as libc::c_long;
    i = j;
    while j < P_HaystackLen as libc::c_int as libc::c_long
        && P_HaystackLen
            .wrapping_sub(j as libc::c_ulong)
            .wrapping_add(i as libc::c_ulong) >= P_NeedleLen
    {
        while i > -(1 as libc::c_int) as libc::c_long
            && *P_Needle.offset(i as isize) as libc::c_int
                != *P_Haystack.offset(j as isize) as libc::c_int
        {
            i = *P_KMP_Table.offset(i as isize);
        }
        i += 1;
        i;
        j += 1;
        j;
        if !(i >= P_NeedleLen as libc::c_int as libc::c_long) {
            continue;
        }
        V_FindPosition = j - i;
        break;
    }
    return V_FindPosition;
}
unsafe extern "C" fn _utstring_BuildTableR(
    mut P_Needle: *const libc::c_char,
    mut P_NeedleLen: size_t,
    mut P_KMP_Table: *mut libc::c_long,
) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    i = P_NeedleLen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_long;
    j = i + 1 as libc::c_int as libc::c_long;
    *P_KMP_Table.offset((i + 1 as libc::c_int as libc::c_long) as isize) = j;
    while i >= 0 as libc::c_int as libc::c_long {
        while j < P_NeedleLen as libc::c_long
            && *P_Needle.offset(i as isize) as libc::c_int
                != *P_Needle.offset(j as isize) as libc::c_int
        {
            j = *P_KMP_Table.offset((j + 1 as libc::c_int as libc::c_long) as isize);
        }
        i -= 1;
        i;
        j -= 1;
        j;
        if i >= 0 as libc::c_int as libc::c_long {
            if *P_Needle.offset(i as isize) as libc::c_int
                == *P_Needle.offset(j as isize) as libc::c_int
            {
                *P_KMP_Table
                    .offset(
                        (i + 1 as libc::c_int as libc::c_long) as isize,
                    ) = *P_KMP_Table
                    .offset((j + 1 as libc::c_int as libc::c_long) as isize);
            } else {
                *P_KMP_Table.offset((i + 1 as libc::c_int as libc::c_long) as isize) = j;
            }
        } else {
            *P_KMP_Table.offset((i + 1 as libc::c_int as libc::c_long) as isize) = j;
        }
    }
}
unsafe extern "C" fn _utstring_BuildTable(
    mut P_Needle: *const libc::c_char,
    mut P_NeedleLen: size_t,
    mut P_KMP_Table: *mut libc::c_long,
) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    i = 0 as libc::c_int as libc::c_long;
    j = i - 1 as libc::c_int as libc::c_long;
    *P_KMP_Table.offset(i as isize) = j;
    while i < P_NeedleLen as libc::c_long {
        while j > -(1 as libc::c_int) as libc::c_long
            && *P_Needle.offset(i as isize) as libc::c_int
                != *P_Needle.offset(j as isize) as libc::c_int
        {
            j = *P_KMP_Table.offset(j as isize);
        }
        i += 1;
        i;
        j += 1;
        j;
        if i < P_NeedleLen as libc::c_long {
            if *P_Needle.offset(i as isize) as libc::c_int
                == *P_Needle.offset(j as isize) as libc::c_int
            {
                *P_KMP_Table.offset(i as isize) = *P_KMP_Table.offset(j as isize);
            } else {
                *P_KMP_Table.offset(i as isize) = j;
            }
        } else {
            *P_KMP_Table.offset(i as isize) = j;
        }
    }
}
unsafe extern "C" fn utstring_printf(
    mut s: *mut UT_string,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    utstring_printf_va(s, fmt, ap.as_va_list());
}
unsafe extern "C" fn utstring_printf_va(
    mut s: *mut UT_string,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut n: libc::c_int = 0;
    let mut cp: ::std::ffi::VaListImpl;
    loop {
        cp = ap.clone();
        n = vsnprintf(
            &mut *((*s).d).offset((*s).i as isize),
            ((*s).n).wrapping_sub((*s).i),
            fmt,
            cp.as_va_list(),
        );
        if n > -(1 as libc::c_int) && (n as size_t) < ((*s).n).wrapping_sub((*s).i) {
            (*s)
                .i = ((*s).i as libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
                as size_t;
            return;
        }
        if n > -(1 as libc::c_int) {
            if ((*s).n).wrapping_sub((*s).i) < (n + 1 as libc::c_int) as size_t {
                let mut utstring_tmp: *mut libc::c_char = realloc(
                    (*s).d as *mut libc::c_void,
                    ((*s).n).wrapping_add((n + 1 as libc::c_int) as libc::c_ulong),
                ) as *mut libc::c_char;
                if utstring_tmp.is_null() {
                    exit(-(1 as libc::c_int));
                }
                (*s).d = utstring_tmp;
                (*s)
                    .n = ((*s).n as libc::c_ulong)
                    .wrapping_add((n + 1 as libc::c_int) as libc::c_ulong) as size_t
                    as size_t;
            }
        } else if ((*s).n).wrapping_sub((*s).i)
            < ((*s).n).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        {
            let mut utstring_tmp_0: *mut libc::c_char = realloc(
                (*s).d as *mut libc::c_void,
                ((*s).n)
                    .wrapping_add(
                        ((*s).n).wrapping_mul(2 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut libc::c_char;
            if utstring_tmp_0.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*s).d = utstring_tmp_0;
            (*s)
                .n = ((*s).n as libc::c_ulong)
                .wrapping_add(((*s).n).wrapping_mul(2 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
        }
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut s: *mut UT_string = 0 as *mut UT_string;
    let mut t: *mut UT_string = 0 as *mut UT_string;
    let mut V_TestStr: [libc::c_char; 56] = *::std::mem::transmute::<
        &[u8; 56],
        &mut [libc::c_char; 56],
    >(b"There are two needle\0s in this \0haystack with needle\0s.\0");
    let mut V_NeedleStr: [libc::c_char; 9] = *::std::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"needle\0s\0");
    let mut V_KMP_Table: *mut libc::c_long = 0 as *mut libc::c_long;
    let mut V_FindPos: libc::c_long = 0;
    let mut V_StartPos: size_t = 0;
    let mut V_FindCnt: size_t = 0 as libc::c_int as size_t;
    s = malloc(::std::mem::size_of::<UT_string>() as libc::c_ulong) as *mut UT_string;
    if s.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*s).n = 0 as libc::c_int as size_t;
    (*s).i = 0 as libc::c_int as size_t;
    (*s).d = 0 as *mut libc::c_char;
    if ((*s).n).wrapping_sub((*s).i) < 100 as libc::c_int as size_t {
        let mut utstring_tmp: *mut libc::c_char = realloc(
            (*s).d as *mut libc::c_void,
            ((*s).n).wrapping_add(100 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if utstring_tmp.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*s).d = utstring_tmp;
        (*s)
            .n = ((*s).n as libc::c_ulong)
            .wrapping_add(100 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    *((*s).d).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    t = malloc(::std::mem::size_of::<UT_string>() as libc::c_ulong) as *mut UT_string;
    if t.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*t).n = 0 as libc::c_int as size_t;
    (*t).i = 0 as libc::c_int as size_t;
    (*t).d = 0 as *mut libc::c_char;
    if ((*t).n).wrapping_sub((*t).i) < 100 as libc::c_int as size_t {
        let mut utstring_tmp_0: *mut libc::c_char = realloc(
            (*t).d as *mut libc::c_void,
            ((*t).n).wrapping_add(100 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if utstring_tmp_0.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*t).d = utstring_tmp_0;
        (*t)
            .n = ((*t).n as libc::c_ulong)
            .wrapping_add(100 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    *((*t).d).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if ((*s).n).wrapping_sub((*s).i)
        < (::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        let mut utstring_tmp_1: *mut libc::c_char = realloc(
            (*s).d as *mut libc::c_void,
            ((*s).n)
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut libc::c_char;
        if utstring_tmp_1.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*s).d = utstring_tmp_1;
        (*s)
            .n = ((*s).n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    if (::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
    {
        memcpy(
            &mut *((*s).d).offset((*s).i as isize) as *mut libc::c_char
                as *mut libc::c_void,
            V_TestStr.as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    (*s)
        .i = ((*s).i as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 56]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    *((*s).d).offset((*s).i as isize) = '\0' as i32 as libc::c_char;
    printf(
        b"\"%s\" len=%u\n\0" as *const u8 as *const libc::c_char,
        (*s).d,
        (*s).i as libc::c_uint,
    );
    if ((*t).n).wrapping_sub((*t).i)
        < (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        let mut utstring_tmp_2: *mut libc::c_char = realloc(
            (*t).d as *mut libc::c_void,
            ((*t).n)
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ),
        ) as *mut libc::c_char;
        if utstring_tmp_2.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*t).d = utstring_tmp_2;
        (*t)
            .n = ((*t).n as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    if (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) != 0
    {
        memcpy(
            &mut *((*t).d).offset((*t).i as isize) as *mut libc::c_char
                as *mut libc::c_void,
            V_NeedleStr.as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    (*t)
        .i = ((*t).i as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    *((*t).d).offset((*t).i as isize) = '\0' as i32 as libc::c_char;
    printf(
        b"\"%s\" len=%u\n\0" as *const u8 as *const libc::c_char,
        (*t).d,
        (*t).i as libc::c_uint,
    );
    V_KMP_Table = malloc(
        (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_mul(((*t).i).wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_long;
    if !V_KMP_Table.is_null() {
        _utstring_BuildTableR((*t).d, (*t).i, V_KMP_Table);
        V_StartPos = ((*s).i).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        loop {
            V_FindPos = _utstring_findR(
                (*s).d,
                V_StartPos.wrapping_add(1 as libc::c_int as libc::c_ulong),
                (*t).d,
                (*t).i,
                V_KMP_Table,
            );
            if V_FindPos >= 0 as libc::c_int as libc::c_long {
                V_FindCnt = V_FindCnt.wrapping_add(1);
                V_FindCnt;
                V_StartPos = (V_FindPos - 1 as libc::c_int as libc::c_long) as size_t;
            }
            printf(
                b"utstring_find()=%ld\n\0" as *const u8 as *const libc::c_char,
                V_FindPos,
            );
            if !(V_FindPos >= 0 as libc::c_int as libc::c_long) {
                break;
            }
        }
        printf(
            b"FindCnt=%u\n\0" as *const u8 as *const libc::c_char,
            V_FindCnt as libc::c_uint,
        );
        free(V_KMP_Table as *mut libc::c_void);
    } else {
        printf(b"malloc() failed...\n\0" as *const u8 as *const libc::c_char);
    }
    if !((*t).d).is_null() {
        free((*t).d as *mut libc::c_void);
    }
    (*t).n = 0 as libc::c_int as size_t;
    free(t as *mut libc::c_void);
    (*s).i = 0 as libc::c_int as size_t;
    *((*s).d).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    utstring_printf(s, b"ABC ABCDAB ABCDABCDABDE\0" as *const u8 as *const libc::c_char);
    let mut o: libc::c_int = 0;
    o = utstring_find(
        s,
        -(9 as libc::c_int) as libc::c_long,
        b"ABC\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) as libc::c_int;
    printf(b"expect 15 %d\n\0" as *const u8 as *const libc::c_char, o);
    o = utstring_find(
        s,
        3 as libc::c_int as libc::c_long,
        b"ABC\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) as libc::c_int;
    printf(b"expect  4 %d\n\0" as *const u8 as *const libc::c_char, o);
    o = utstring_find(
        s,
        16 as libc::c_int as libc::c_long,
        b"ABC\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) as libc::c_int;
    printf(b"expect -1 %d\n\0" as *const u8 as *const libc::c_char, o);
    o = utstring_findR(
        s,
        -(9 as libc::c_int) as libc::c_long,
        b"ABC\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) as libc::c_int;
    printf(b"expect 11 %d\n\0" as *const u8 as *const libc::c_char, o);
    o = utstring_findR(
        s,
        12 as libc::c_int as libc::c_long,
        b"ABC\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) as libc::c_int;
    printf(b"expect  4 %d\n\0" as *const u8 as *const libc::c_char, o);
    o = utstring_findR(
        s,
        13 as libc::c_int as libc::c_long,
        b"ABC\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) as libc::c_int;
    printf(b"expect 11 %d\n\0" as *const u8 as *const libc::c_char, o);
    o = utstring_findR(
        s,
        2 as libc::c_int as libc::c_long,
        b"ABC\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) as libc::c_int;
    printf(b"expect  0 %d\n\0" as *const u8 as *const libc::c_char, o);
    if !((*s).d).is_null() {
        free((*s).d as *mut libc::c_void);
    }
    (*s).n = 0 as libc::c_int as size_t;
    free(s as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
