use ::libc;
extern "C" {
    pub type hash_table;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    fn rpl_free(ptr: *mut libc::c_void);
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn hash_get_n_entries(table: *const Hash_table) -> size_t;
    fn hash_string(string: *const libc::c_char, n_buckets: size_t) -> size_t;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type Hash_table = hash_table;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
unsafe extern "C" fn hash_string_hasher(
    mut name: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    return hash_string(name as *const libc::c_char, n_buckets);
}
unsafe extern "C" fn hash_string_compare(
    mut name1: *const libc::c_void,
    mut name2: *const libc::c_void,
) -> bool {
    return strcmp(name1 as *const libc::c_char, name2 as *const libc::c_char)
        == 0 as libc::c_int;
}
unsafe extern "C" fn hash_string_insert_prefix(
    mut table: *mut *mut Hash_table,
    mut string: *const libc::c_char,
    mut len: size_t,
    mut return_prefix: *mut *const libc::c_char,
) -> bool {
    let mut t: *mut Hash_table = *table;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    if len != 0 {
        s = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        memcpy(s as *mut libc::c_void, string as *const libc::c_void, len);
        *s.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        s = xstrdup(string);
    }
    if !((!t.is_null()
        || {
            t = hash_initialize(
                0 as libc::c_int as size_t,
                0 as *const Hash_tuning,
                Some(
                    hash_string_hasher
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    hash_string_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                None,
            );
            *table = t;
            !(*table).is_null()
        })
        && {
            e = hash_insert(t, s as *const libc::c_void) as *mut libc::c_char;
            !e.is_null()
        })
    {
        xalloc_die();
    }
    if e == s {
        if !return_prefix.is_null() {
            *return_prefix = s;
        }
        return 1 as libc::c_int != 0;
    } else {
        rpl_free(s as *mut libc::c_void);
        return 0 as libc::c_int != 0;
    };
}
static mut prefix_table: [*mut Hash_table; 2] = [0 as *const Hash_table
    as *mut Hash_table; 2];
pub unsafe extern "C" fn removed_prefixes_p() -> bool {
    return !(prefix_table[0 as libc::c_int as usize]).is_null()
        && hash_get_n_entries(prefix_table[0 as libc::c_int as usize])
            != 0 as libc::c_int as libc::c_ulong
        || !(prefix_table[1 as libc::c_int as usize]).is_null()
            && hash_get_n_entries(prefix_table[1 as libc::c_int as usize])
                != 0 as libc::c_int as libc::c_ulong;
}
pub unsafe extern "C" fn safer_name_suffix(
    mut file_name: *const libc::c_char,
    mut link_target: bool,
    mut absolute_names: bool,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if absolute_names {
        p = file_name;
    } else {
        let mut prefix_len: size_t = 0 as libc::c_int as size_t;
        p = file_name.offset(prefix_len as isize);
        while *p != 0 {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && (*p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                    || *p.offset(2 as libc::c_int as isize) == 0)
            {
                prefix_len = p.offset(2 as libc::c_int as isize).offset_from(file_name)
                    as libc::c_long as size_t;
            }
            loop {
                let fresh0 = p;
                p = p.offset(1);
                let mut c: libc::c_char = *fresh0;
                if c as libc::c_int == '/' as i32 {
                    break;
                }
                if !(*p != 0) {
                    break;
                }
            }
        }
        p = file_name.offset(prefix_len as isize);
        while *p as libc::c_int == '/' as i32 {
            p = p.offset(1);
            p;
        }
        prefix_len = p.offset_from(file_name) as libc::c_long as size_t;
        if prefix_len != 0 {
            let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
            if hash_string_insert_prefix(
                &mut *prefix_table.as_mut_ptr().offset(link_target as isize),
                file_name,
                prefix_len,
                &mut prefix,
            ) {
                static mut diagnostic: [*const libc::c_char; 2] = [
                    b"Removing leading `%s' from member names\0" as *const u8
                        as *const libc::c_char,
                    b"Removing leading `%s' from hard link targets\0" as *const u8
                        as *const libc::c_char,
                ];
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        diagnostic[link_target as usize],
                        5 as libc::c_int,
                    ),
                    prefix,
                );
            }
        }
    }
    if *p == 0 {
        if p == file_name {
            static mut diagnostic_0: [*const libc::c_char; 2] = [
                b"Substituting `.' for empty member name\0" as *const u8
                    as *const libc::c_char,
                b"Substituting `.' for empty hard link target\0" as *const u8
                    as *const libc::c_char,
            ];
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                dcgettext(
                    0 as *const libc::c_char,
                    diagnostic_0[link_target as usize],
                    5 as libc::c_int,
                ),
            );
        }
        p = b".\0" as *const u8 as *const libc::c_char;
    }
    return p as *mut libc::c_char;
}
