use ::libc;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn endpwent();
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn sh_get_home_dir() -> *mut libc::c_char;
    fn sh_get_env_value(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type tilde_hook_func_t = unsafe extern "C" fn(
    *mut libc::c_char,
) -> *mut libc::c_char;
static mut default_prefixes: [*const libc::c_char; 3] = [
    b" ~\0" as *const u8 as *const libc::c_char,
    b"\t~\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_void as *mut libc::c_void as *const libc::c_char,
];
static mut default_suffixes: [*const libc::c_char; 3] = [
    b" \0" as *const u8 as *const libc::c_char,
    b"\n\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_void as *mut libc::c_void as *const libc::c_char,
];
pub static mut tilde_expansion_preexpansion_hook: Option::<tilde_hook_func_t> = unsafe {
    ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<tilde_hook_func_t>,
    >(0 as *const libc::c_void as *mut libc::c_void)
};
pub static mut tilde_expansion_failure_hook: Option::<tilde_hook_func_t> = unsafe {
    ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<tilde_hook_func_t>,
    >(0 as *const libc::c_void as *mut libc::c_void)
};
pub static mut tilde_additional_prefixes: *mut *mut libc::c_char = unsafe {
    default_prefixes.as_ptr() as *mut *mut libc::c_char
};
pub static mut tilde_additional_suffixes: *mut *mut libc::c_char = unsafe {
    default_suffixes.as_ptr() as *mut *mut libc::c_char
};
unsafe extern "C" fn tilde_find_prefix(
    mut string: *const libc::c_char,
    mut len: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut string_len: libc::c_int = 0;
    let mut prefixes: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    prefixes = tilde_additional_prefixes;
    string_len = strlen(string) as libc::c_int;
    *len = 0 as libc::c_int;
    if *string as libc::c_int == '\0' as i32 || *string as libc::c_int == '~' as i32 {
        return 0 as libc::c_int;
    }
    if !prefixes.is_null() {
        i = 0 as libc::c_int;
        while i < string_len {
            j = 0 as libc::c_int;
            while !(*prefixes.offset(j as isize)).is_null() {
                if strncmp(
                    string.offset(i as isize),
                    *prefixes.offset(j as isize),
                    strlen(*prefixes.offset(j as isize)),
                ) == 0 as libc::c_int
                {
                    *len = (strlen(*prefixes.offset(j as isize)))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                    return i + *len;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    return string_len;
}
unsafe extern "C" fn tilde_find_suffix(mut string: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut string_len: libc::c_int = 0;
    let mut suffixes: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    suffixes = tilde_additional_suffixes;
    string_len = strlen(string) as libc::c_int;
    i = 0 as libc::c_int;
    while i < string_len {
        if *string.offset(i as isize) as libc::c_int == '/' as i32 {
            break;
        }
        j = 0 as libc::c_int;
        while !suffixes.is_null() && !(*suffixes.offset(j as isize)).is_null() {
            if strncmp(
                string.offset(i as isize),
                *suffixes.offset(j as isize),
                strlen(*suffixes.offset(j as isize)),
            ) == 0 as libc::c_int
            {
                return i;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return i;
}
pub unsafe extern "C" fn tilde_expand(
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result_size: libc::c_int = 0;
    let mut result_index: libc::c_int = 0;
    result_size = 0 as libc::c_int;
    result_index = result_size;
    result = strchr(string, '~' as i32);
    if !result.is_null() {
        result_size = (strlen(string)).wrapping_add(16 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        result = xmalloc(result_size as size_t) as *mut libc::c_char;
    } else {
        result_size = (strlen(string)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        result = xmalloc(result_size as size_t) as *mut libc::c_char;
    }
    loop {
        let mut start: libc::c_int = 0;
        let mut end: libc::c_int = 0;
        let mut tilde_word: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut expansion: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = 0;
        start = tilde_find_prefix(string, &mut len);
        if result_index + start + 1 as libc::c_int > result_size {
            result_size += start + 20 as libc::c_int;
            result = xrealloc(
                result as *mut libc::c_void,
                (1 as libc::c_int + result_size) as size_t,
            ) as *mut libc::c_char;
        }
        strncpy(result.offset(result_index as isize), string, start as libc::c_ulong);
        result_index += start;
        string = string.offset(start as isize);
        end = tilde_find_suffix(string);
        if start == 0 && end == 0 {
            break;
        }
        tilde_word = xmalloc((1 as libc::c_int + end) as size_t) as *mut libc::c_char;
        strncpy(tilde_word, string, end as libc::c_ulong);
        *tilde_word.offset(end as isize) = '\0' as i32 as libc::c_char;
        string = string.offset(end as isize);
        expansion = tilde_expand_word(tilde_word);
        free(tilde_word as *mut libc::c_void);
        len = strlen(expansion) as libc::c_int;
        if result_index + len + 1 as libc::c_int > result_size {
            result_size += len + 20 as libc::c_int;
            result = xrealloc(
                result as *mut libc::c_void,
                (1 as libc::c_int + result_size) as size_t,
            ) as *mut libc::c_char;
        }
        strcpy(result.offset(result_index as isize), expansion);
        result_index += len;
        free(expansion as *mut libc::c_void);
    }
    *result.offset(result_index as isize) = '\0' as i32 as libc::c_char;
    return result;
}
unsafe extern "C" fn isolate_tilde_prefix(
    mut fname: *const libc::c_char,
    mut lenp: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    ret = xmalloc(strlen(fname)) as *mut libc::c_char;
    i = 1 as libc::c_int;
    while *fname.offset(i as isize) as libc::c_int != 0
        && *fname.offset(i as isize) as libc::c_int != '/' as i32
    {
        *ret.offset((i - 1 as libc::c_int) as isize) = *fname.offset(i as isize);
        i += 1;
        i;
    }
    *ret.offset((i - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    if !lenp.is_null() {
        *lenp = i;
    }
    return ret;
}
unsafe extern "C" fn glue_prefix_and_suffix(
    mut prefix: *mut libc::c_char,
    mut suffix: *const libc::c_char,
    mut suffind: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plen: libc::c_int = 0;
    let mut slen: libc::c_int = 0;
    plen = (if !prefix.is_null() && *prefix as libc::c_int != 0 {
        strlen(prefix)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
    slen = strlen(suffix.offset(suffind as isize)) as libc::c_int;
    ret = xmalloc((plen + slen + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    if plen != 0 {
        strcpy(ret, prefix);
    }
    strcpy(ret.offset(plen as isize), suffix.offset(suffind as isize));
    return ret;
}
pub unsafe extern "C" fn tilde_expand_word(
    mut filename: *const libc::c_char,
) -> *mut libc::c_char {
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expansion: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut username: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user_len: libc::c_int = 0;
    let mut user_entry: *mut passwd = 0 as *mut passwd;
    if filename.is_null() {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if *filename as libc::c_int != '~' as i32 {
        return strcpy(
            xmalloc((1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(filename)))
                as *mut libc::c_char,
            filename,
        );
    }
    if *filename.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        || *filename.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        expansion = sh_get_env_value(b"HOME\0" as *const u8 as *const libc::c_char);
        if expansion.is_null() {
            expansion = sh_get_home_dir();
        }
        return glue_prefix_and_suffix(expansion, filename, 1 as libc::c_int);
    }
    username = isolate_tilde_prefix(filename, &mut user_len);
    if tilde_expansion_preexpansion_hook.is_some() {
        expansion = (Some(tilde_expansion_preexpansion_hook.unwrap()))
            .unwrap()(username);
        if !expansion.is_null() {
            dirname = glue_prefix_and_suffix(expansion, filename, user_len);
            free(username as *mut libc::c_void);
            free(expansion as *mut libc::c_void);
            return dirname;
        }
    }
    dirname = 0 as *mut libc::c_void as *mut libc::c_char;
    user_entry = getpwnam(username);
    if user_entry.is_null() {
        if tilde_expansion_failure_hook.is_some() {
            expansion = (Some(tilde_expansion_failure_hook.unwrap())).unwrap()(username);
            if !expansion.is_null() {
                dirname = glue_prefix_and_suffix(expansion, filename, user_len);
                free(expansion as *mut libc::c_void);
            }
        }
        if dirname.is_null() {
            dirname = strcpy(
                xmalloc(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(filename)),
                ) as *mut libc::c_char,
                filename,
            );
        }
    } else {
        dirname = glue_prefix_and_suffix((*user_entry).pw_dir, filename, user_len);
    }
    free(username as *mut libc::c_void);
    endpwent();
    return dirname;
}
