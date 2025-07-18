use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    pub type wordsplit_node;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn fatal_exit() -> !;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn __strtoul_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_ulong;
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
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    fn open_fatal(_: *const libc::c_char) -> !;
    static mut group_name_option: *const libc::c_char;
    static mut group_option: gid_t;
    static mut owner_name_option: *const libc::c_char;
    static mut owner_option: uid_t;
    fn wordsplit(
        s: *const libc::c_char,
        ws: *mut wordsplit_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn wordsplit_free(ws: *mut wordsplit_t);
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const libc::c_char;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
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
pub type __uintmax_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uintmax_t = __uintmax_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
pub type Hash_table = hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut libc::c_char,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: libc::c_uint,
    pub ws_options: libc::c_uint,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const libc::c_char,
    pub ws_comment: *const libc::c_char,
    pub ws_escape: [*const libc::c_char; 2],
    pub ws_alloc_die: Option::<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_debug: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_env: *mut *const libc::c_char,
    pub ws_envbuf: *mut *mut libc::c_char,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_getvar: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_closure: *mut libc::c_void,
    pub ws_command: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut *mut libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_input: *const libc::c_char,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: libc::c_int,
    pub ws_usererr: *mut libc::c_char,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_lvl: libc::c_int,
}
pub type wordsplit_t = wordsplit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapentry {
    pub orig_id: uintmax_t,
    pub new_id: uintmax_t,
    pub new_name: *mut libc::c_char,
}
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
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn strtoumax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
}
unsafe extern "C" fn map_hash(
    mut entry: *const libc::c_void,
    mut nbuckets: size_t,
) -> size_t {
    let mut map: *const mapentry = entry as *const mapentry;
    return ((*map).orig_id).wrapping_rem(nbuckets);
}
unsafe extern "C" fn map_compare(
    mut entry1: *const libc::c_void,
    mut entry2: *const libc::c_void,
) -> bool {
    let mut map1: *const mapentry = entry1 as *const mapentry;
    let mut map2: *const mapentry = entry2 as *const mapentry;
    return (*map1).orig_id == (*map2).orig_id;
}
unsafe extern "C" fn parse_id(
    mut retval: *mut uintmax_t,
    mut arg: *const libc::c_char,
    mut what: *const libc::c_char,
    mut maxval: uintmax_t,
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
) -> libc::c_int {
    let mut v: uintmax_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    v = strtoumax(arg, &mut p, 10 as libc::c_int);
    if *p as libc::c_int != 0 || *__errno_location() != 0 {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%u: invalid %s: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            what,
            arg,
        );
        return -(1 as libc::c_int);
    }
    if v > maxval {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s:%u: %s out of range: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            file,
            line,
            what,
            arg,
        );
        return -(1 as libc::c_int);
    }
    *retval = v;
    return 0 as libc::c_int;
}
unsafe extern "C" fn map_read(
    mut ptab: *mut *mut Hash_table,
    mut file: *const libc::c_char,
    mut name_to_id: Option::<unsafe extern "C" fn(*const libc::c_char) -> uintmax_t>,
    mut what: *const libc::c_char,
    mut maxval: uintmax_t,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut n: ssize_t = 0;
    let mut ws: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: 0 as *mut *mut libc::c_char,
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: 0 as *const libc::c_char,
        ws_comment: 0 as *const libc::c_char,
        ws_escape: [0 as *const libc::c_char; 2],
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: 0 as *mut *const libc::c_char,
        ws_envbuf: 0 as *mut *mut libc::c_char,
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_getvar: None,
        ws_closure: 0 as *mut libc::c_void,
        ws_command: None,
        ws_input: 0 as *const libc::c_char,
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: 0 as *mut libc::c_char,
        ws_head: 0 as *mut wordsplit_node,
        ws_tail: 0 as *mut wordsplit_node,
        ws_lvl: 0,
    };
    let mut wsopt: libc::c_int = 0;
    let mut line: libc::c_uint = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    fp = fopen(file, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        open_fatal(file);
    }
    ws.ws_comment = b"#\0" as *const u8 as *const libc::c_char;
    wsopt = 0x8000 as libc::c_int | 0x40 as libc::c_int | 0x4 as libc::c_int
        | 0x800 as libc::c_int | (0x200 as libc::c_int | 0x400 as libc::c_int);
    line = 0 as libc::c_int as libc::c_uint;
    loop {
        n = getline(&mut buf, &mut bufsize, fp);
        if !(n > 0 as libc::c_int as libc::c_long) {
            break;
        }
        let mut ent: *mut mapentry = 0 as *mut mapentry;
        let mut orig_id: uintmax_t = 0;
        let mut new_id: uintmax_t = 0;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
        line = line.wrapping_add(1);
        line;
        if wordsplit(buf, &mut ws, wsopt as libc::c_uint) != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s:%u: cannot split line: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file,
                line,
                wordsplit_strerror(&mut ws),
            );
            fatal_exit();
        }
        wsopt |= 0x8 as libc::c_int;
        if ws.ws_wordc == 0 as libc::c_int as libc::c_ulong {
            continue;
        }
        if ws.ws_wordc != 2 as libc::c_int as libc::c_ulong {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s:%u: malformed line\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file,
                line,
            );
            err = 1 as libc::c_int;
        } else {
            if *(*(ws.ws_wordv).offset(0 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
            {
                if parse_id(
                    &mut orig_id,
                    (*(ws.ws_wordv).offset(0 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize),
                    what,
                    maxval,
                    file,
                    line,
                ) != 0
                {
                    err = 1 as libc::c_int;
                    continue;
                }
            } else if name_to_id.is_some() {
                orig_id = name_to_id
                    .unwrap()(*(ws.ws_wordv).offset(0 as libc::c_int as isize));
                if orig_id == 18446744073709551615 as libc::c_ulong {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%u: can't obtain %s of %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        file,
                        line,
                        what,
                        *(ws.ws_wordv).offset(0 as libc::c_int as isize),
                    );
                    err = 1 as libc::c_int;
                    continue;
                }
            }
            colon = strchr(*(ws.ws_wordv).offset(1 as libc::c_int as isize), ':' as i32);
            if !colon.is_null() {
                if colon > *(ws.ws_wordv).offset(1 as libc::c_int as isize) {
                    name = *(ws.ws_wordv).offset(1 as libc::c_int as isize);
                }
                let fresh0 = colon;
                colon = colon.offset(1);
                *fresh0 = 0 as libc::c_int as libc::c_char;
                if parse_id(&mut new_id, colon, what, maxval, file, line) != 0 {
                    err = 1 as libc::c_int;
                    continue;
                }
            } else if *(*(ws.ws_wordv).offset(1 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
            {
                if parse_id(
                    &mut new_id,
                    *(ws.ws_wordv).offset(1 as libc::c_int as isize),
                    what,
                    maxval,
                    file,
                    line,
                ) != 0
                {
                    err = 1 as libc::c_int;
                    continue;
                }
            } else {
                name = *(ws.ws_wordv).offset(1 as libc::c_int as isize);
                new_id = name_to_id
                    .unwrap()(*(ws.ws_wordv).offset(1 as libc::c_int as isize));
                if new_id == 18446744073709551615 as libc::c_ulong {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s:%u: can't obtain %s of %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        file,
                        line,
                        what,
                        *(ws.ws_wordv).offset(1 as libc::c_int as isize),
                    );
                    err = 1 as libc::c_int;
                    continue;
                }
            }
            ent = xmalloc(::std::mem::size_of::<mapentry>() as libc::c_ulong)
                as *mut mapentry;
            (*ent).orig_id = orig_id;
            (*ent).new_id = new_id;
            (*ent)
                .new_name = if !name.is_null() {
                xstrdup(name)
            } else {
                0 as *mut libc::c_char
            };
            if !((!(*ptab).is_null()
                || {
                    *ptab = hash_initialize(
                        0 as libc::c_int as size_t,
                        0 as *const Hash_tuning,
                        Some(
                            map_hash
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    size_t,
                                ) -> size_t,
                        ),
                        Some(
                            map_compare
                                as unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *const libc::c_void,
                                ) -> bool,
                        ),
                        None,
                    );
                    !(*ptab).is_null()
                }) && !(hash_insert(*ptab, ent as *const libc::c_void)).is_null())
            {
                xalloc_die();
            }
        }
    }
    if wsopt & 0x8 as libc::c_int != 0 {
        wordsplit_free(&mut ws);
    }
    fclose(fp);
    if err != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"errors reading map file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        fatal_exit();
    }
}
static mut owner_map: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn name_to_uid(mut name: *const libc::c_char) -> uintmax_t {
    let mut pw: *mut passwd = getpwnam(name);
    return if !pw.is_null() {
        (*pw).pw_uid as libc::c_ulong
    } else {
        18446744073709551615 as libc::c_ulong
    };
}
pub unsafe extern "C" fn owner_map_read(mut file: *const libc::c_char) {
    map_read(
        &mut owner_map,
        file,
        Some(name_to_uid as unsafe extern "C" fn(*const libc::c_char) -> uintmax_t),
        b"UID\0" as *const u8 as *const libc::c_char,
        (if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t {
            -(1 as libc::c_int) as uid_t
        } else {
            ((1 as libc::c_int as uid_t)
                << (::std::mem::size_of::<uid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
    );
}
pub unsafe extern "C" fn owner_map_translate(
    mut uid: uid_t,
    mut new_uid: *mut uid_t,
    mut new_name: *mut *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 1 as libc::c_int;
    if !owner_map.is_null() {
        let mut ent: mapentry = mapentry {
            orig_id: 0,
            new_id: 0,
            new_name: 0 as *mut libc::c_char,
        };
        let mut res: *mut mapentry = 0 as *mut mapentry;
        ent.orig_id = uid as uintmax_t;
        res = hash_lookup(owner_map, &mut ent as *mut mapentry as *const libc::c_void)
            as *mut mapentry;
        if !res.is_null() {
            *new_uid = (*res).new_id as uid_t;
            *new_name = (*res).new_name;
            return 0 as libc::c_int;
        }
    }
    if owner_option != -(1 as libc::c_int) as uid_t {
        *new_uid = owner_option;
        rc = 0 as libc::c_int;
    }
    if !owner_name_option.is_null() {
        *new_name = owner_name_option;
        rc = 0 as libc::c_int;
    }
    return rc;
}
static mut group_map: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn name_to_gid(mut name: *const libc::c_char) -> uintmax_t {
    let mut gr: *mut group = getgrnam(name);
    return if !gr.is_null() {
        (*gr).gr_gid as libc::c_ulong
    } else {
        18446744073709551615 as libc::c_ulong
    };
}
pub unsafe extern "C" fn group_map_read(mut file: *const libc::c_char) {
    map_read(
        &mut group_map,
        file,
        Some(name_to_gid as unsafe extern "C" fn(*const libc::c_char) -> uintmax_t),
        b"GID\0" as *const u8 as *const libc::c_char,
        (if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
            -(1 as libc::c_int) as gid_t
        } else {
            ((1 as libc::c_int as gid_t)
                << (::std::mem::size_of::<gid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
    );
}
pub unsafe extern "C" fn group_map_translate(
    mut gid: gid_t,
    mut new_gid: *mut gid_t,
    mut new_name: *mut *const libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 1 as libc::c_int;
    if !group_map.is_null() {
        let mut ent: mapentry = mapentry {
            orig_id: 0,
            new_id: 0,
            new_name: 0 as *mut libc::c_char,
        };
        let mut res: *mut mapentry = 0 as *mut mapentry;
        ent.orig_id = gid as uintmax_t;
        res = hash_lookup(group_map, &mut ent as *mut mapentry as *const libc::c_void)
            as *mut mapentry;
        if !res.is_null() {
            *new_gid = (*res).new_id as gid_t;
            *new_name = (*res).new_name;
            return 0 as libc::c_int;
        }
    }
    if group_option != -(1 as libc::c_int) as uid_t {
        *new_gid = group_option;
        rc = 0 as libc::c_int;
    }
    if !group_name_option.is_null() {
        *new_name = group_name_option;
        rc = 0 as libc::c_int;
    }
    return rc;
}
