use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut no_intermediates: libc::c_uint;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut stopchar_map: [libc::c_ushort; 0];
    fn strcache_add_len(str: *const libc::c_char, len: size_t) -> *const libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn lindex(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn print_spaces(_: libc::c_uint);
    fn ar_name(_: *const libc::c_char) -> libc::c_int;
    fn file_exists_p(_: *const libc::c_char) -> libc::c_int;
    fn file_impossible_p(_: *const libc::c_char) -> libc::c_int;
    fn file_impossible(_: *const libc::c_char);
    fn vpath_search(
        file: *const libc::c_char,
        mtime_ptr: *mut uintmax_t,
        vpath_index: *mut libc::c_uint,
        path_index: *mut libc::c_uint,
    ) -> *const libc::c_char;
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn lookup_file(name: *const libc::c_char) -> *mut file;
    fn enter_file(name: *const libc::c_char) -> *mut file;
    static mut pattern_rules: *mut rule;
    static mut num_pattern_rules: libc::c_uint;
    static mut max_pattern_deps: libc::c_uint;
    static mut max_pattern_targets: libc::c_uint;
    static mut max_pattern_dep_length: size_t;
    fn get_rule_defn(rule: *mut rule) -> *const libc::c_char;
    fn parse_file_seq(
        stringp: *mut *mut libc::c_char,
        size: size_t,
        stopmap: libc::c_int,
        prefix: *const libc::c_char,
        flags: libc::c_int,
    ) -> *mut libc::c_void;
    fn free_ns_chain(n: *mut nameseq);
    static mut db_level: libc::c_int;
    fn variable_expand_for_file(
        line: *const libc::c_char,
        file: *mut file,
    ) -> *mut libc::c_char;
    fn free_variable_set(_: *mut variable_set_list);
    fn initialize_file_variables(file: *mut file, reading: libc::c_int);
    fn merge_variable_set_lists(
        to_list: *mut *mut variable_set_list,
        from_list: *mut variable_set_list,
    );
    fn define_variable_in_set(
        name: *const libc::c_char,
        length: size_t,
        value: *const libc::c_char,
        origin: variable_origin,
        recursive: libc::c_int,
        set: *mut variable_set,
        flocp: *const floc,
    ) -> *mut variable;
    fn set_file_variables(file: *mut file, stem: *const libc::c_char);
    fn shuffle_deps_recursive(g: *mut dep);
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const libc::c_char,
    pub hname: *const libc::c_char,
    pub vpath: *const libc::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const libc::c_char,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: libc::c_uint,
    pub command_flags: libc::c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type cmd_state = libc::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = libc::c_uint;
pub const us_failed: update_status = 3;
pub const us_question: update_status = 2;
pub const us_none: update_status = 1;
pub const us_success: update_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: libc::c_ulong,
    pub ht_capacity: libc::c_ulong,
    pub ht_fill: libc::c_ulong,
    pub ht_empty_slots: libc::c_ulong,
    pub ht_collisions: libc::c_ulong,
    pub ht_lookups: libc::c_ulong,
    pub ht_rehashes: libc::c_uint,
}
pub type hash_cmp_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type hash_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct dep {
    pub next: *mut dep,
    pub name: *const libc::c_char,
    pub file: *mut file,
    pub shuf: *mut dep,
    pub stem: *const libc::c_char,
    #[bitfield(name = "flags", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "changed", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "staticpattern", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "need_2nd_expansion", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "14..=14")]
    pub flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct commands {
    pub fileinfo: floc,
    pub commands: *mut libc::c_char,
    pub command_lines: *mut *mut libc::c_char,
    pub lines_flags: *mut libc::c_uchar,
    pub ncommand_lines: libc::c_ushort,
    pub recipe_prefix: libc::c_char,
    #[bitfield(name = "any_recurse", ty = "libc::c_uint", bits = "0..=0")]
    pub any_recurse: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
pub const o_invalid: variable_origin = 7;
pub const o_automatic: variable_origin = 6;
pub const o_override: variable_origin = 5;
pub const o_command: variable_origin = 4;
pub const o_env_override: variable_origin = 3;
pub const o_file: variable_origin = 2;
pub const o_env: variable_origin = 1;
pub const o_default: variable_origin = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct variable {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub fileinfo: floc,
    pub length: libc::c_uint,
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "append", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "conditional", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "per_target", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "special", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "exportable", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "expanding", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "private_var", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "exp_count", ty = "libc::c_uint", bits = "8..=22")]
    #[bitfield(name = "flavor", ty = "variable_flavor", bits = "23..=25")]
    #[bitfield(name = "origin", ty = "variable_origin", bits = "26..=28")]
    #[bitfield(name = "export", ty = "variable_export", bits = "29..=30")]
    pub recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [u8; 4],
}
pub type variable_export = libc::c_uint;
pub const v_ifset: variable_export = 3;
pub const v_noexport: variable_export = 2;
pub const v_export: variable_export = 1;
pub const v_default: variable_export = 0;
pub type variable_origin = libc::c_uint;
pub type variable_flavor = libc::c_uint;
pub const f_append_value: variable_flavor = 7;
pub const f_shell: variable_flavor = 6;
pub const f_conditional: variable_flavor = 5;
pub const f_append: variable_flavor = 4;
pub const f_expand: variable_flavor = 3;
pub const f_recursive: variable_flavor = 2;
pub const f_simple: variable_flavor = 1;
pub const f_bogus: variable_flavor = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rule {
    pub next: *mut rule,
    pub targets: *mut *const libc::c_char,
    pub lens: *mut libc::c_uint,
    pub suffixes: *mut *const libc::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub _defn: *mut libc::c_char,
    pub num: libc::c_ushort,
    pub terminal: libc::c_char,
    pub in_use: libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct patdeps {
    pub name: *const libc::c_char,
    pub pattern: *const libc::c_char,
    pub file: *mut file,
    #[bitfield(name = "ignore_mtime", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "ignore_automatic_vars", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "wait_here", ty = "libc::c_uint", bits = "3..=3")]
    pub ignore_mtime_ignore_automatic_vars_is_explicit_wait_here: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tryrule {
    pub rule: *mut rule,
    pub stemlen: size_t,
    pub matches: libc::c_uint,
    pub order: libc::c_uint,
    pub checked_lastslash: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const libc::c_char,
}
pub unsafe extern "C" fn try_implicit_rule(
    mut file: *mut file,
    mut depth: libc::c_uint,
) -> libc::c_int {
    if 0x8 as libc::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Looking for an implicit rule for '%s'.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file).name,
        );
        fflush(stdout);
    }
    if pattern_search(
        file,
        0 as libc::c_int,
        depth,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    if ar_name((*file).name) != 0 {
        if 0x8 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Looking for archive-member implicit rule for '%s'.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        if pattern_search(
            file,
            1 as libc::c_int,
            depth,
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int,
        ) != 0
        {
            return 1 as libc::c_int;
        }
        if 0x8 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"No archive-member implicit rule found for '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_next_word(
    mut buffer: *const libc::c_char,
    mut length: *mut size_t,
) -> *const libc::c_char {
    let mut current_block: u64;
    let mut p: *const libc::c_char = buffer;
    let mut beg: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_char = 0;
    while *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize) as libc::c_int
        & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 as libc::c_int
    {
        p = p.offset(1);
        p;
    }
    beg = p;
    let fresh0 = p;
    p = p.offset(1);
    c = *fresh0;
    if c as libc::c_int == '\0' as i32 {
        return 0 as *const libc::c_char;
    }
    loop {
        let mut closeparen: libc::c_char = 0;
        let mut count: libc::c_int = 0;
        match c as libc::c_int {
            0 | 32 | 9 => {
                current_block = 13804772961564280491;
                break;
            }
            36 => {
                let fresh1 = p;
                p = p.offset(1);
                c = *fresh1;
                if !(c as libc::c_int == '$' as i32) {
                    if c as libc::c_int == '(' as i32 {
                        closeparen = ')' as i32 as libc::c_char;
                        current_block = 10599921512955367680;
                    } else if c as libc::c_int == '{' as i32 {
                        closeparen = '}' as i32 as libc::c_char;
                        current_block = 10599921512955367680;
                    } else {
                        current_block = 2668756484064249700;
                    }
                    match current_block {
                        2668756484064249700 => {}
                        _ => {
                            count = 0 as libc::c_int;
                            while *p as libc::c_int != '\0' as i32 {
                                if *p as libc::c_int == c as libc::c_int {
                                    count += 1;
                                    count;
                                } else if *p as libc::c_int == closeparen as libc::c_int
                                    && {
                                        count -= 1;
                                        count < 0 as libc::c_int
                                    }
                                {
                                    p = p.offset(1);
                                    p;
                                    break;
                                }
                                p = p.offset(1);
                                p;
                            }
                        }
                    }
                }
            }
            124 => {
                current_block = 7692805958183212788;
                break;
            }
            _ => {}
        }
        let fresh2 = p;
        p = p.offset(1);
        c = *fresh2;
    }
    match current_block {
        13804772961564280491 => {
            p = p.offset(-1);
            p;
        }
        _ => {}
    }
    if !length.is_null() {
        *length = p.offset_from(beg) as libc::c_long as size_t;
    }
    return beg;
}
pub unsafe extern "C" fn stemlen_compare(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> libc::c_int {
    let mut r1: *const tryrule = v1 as *const tryrule;
    let mut r2: *const tryrule = v2 as *const tryrule;
    let mut r: libc::c_int = ((*r1).stemlen).wrapping_sub((*r2).stemlen) as libc::c_int;
    return if r != 0 as libc::c_int {
        r
    } else {
        ((*r1).order).wrapping_sub((*r2).order) as libc::c_int
    };
}
unsafe extern "C" fn pattern_search(
    mut file: *mut file,
    mut archive: libc::c_int,
    mut depth: libc::c_uint,
    mut recursions: libc::c_uint,
    mut allow_compat_rules: libc::c_int,
) -> libc::c_int {
    let mut filename: *const libc::c_char = if archive != 0 {
        strchr((*file).name, '(' as i32) as *const libc::c_char
    } else {
        (*file).name
    };
    let mut namelen: size_t = strlen(filename);
    let mut lastslash: *const libc::c_char = 0 as *const libc::c_char;
    let mut int_file: *mut file = 0 as *mut file;
    let mut max_deps: libc::c_uint = max_pattern_deps;
    let mut deplist: *mut patdeps = xmalloc(
        (max_deps as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<patdeps>() as libc::c_ulong),
    ) as *mut patdeps;
    let mut pat: *mut patdeps = deplist;
    let mut deplen: size_t = namelen
        .wrapping_add(max_pattern_dep_length)
        .wrapping_add(4 as libc::c_int as libc::c_ulong);
    let mut fresh3 = ::std::vec::from_elem(0, deplen as usize);
    let mut depname: *mut libc::c_char = fresh3.leak().as_mut_ptr() as *mut libc::c_char;
    let mut stem: *const libc::c_char = 0 as *const libc::c_char;
    let mut stemlen: size_t = 0 as libc::c_int as size_t;
    let mut fullstemlen: size_t = 0 as libc::c_int as size_t;
    let mut tryrules: *mut tryrule = xmalloc(
        (num_pattern_rules.wrapping_mul(max_pattern_targets) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<tryrule>() as libc::c_ulong),
    ) as *mut tryrule;
    let mut nrules: libc::c_uint = 0;
    let mut foundrule: libc::c_uint = 0;
    let mut intermed_ok: libc::c_int = 0;
    let mut file_vars_initialized: libc::c_int = 0 as libc::c_int;
    let mut specific_rule_matched: libc::c_int = 0 as libc::c_int;
    let mut ri: libc::c_uint = 0;
    let mut found_compat_rule: libc::c_int = 0 as libc::c_int;
    let mut rule: *mut rule = 0 as *mut rule;
    let mut pathdir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pathlen: size_t = 0;
    let mut stem_str: [libc::c_char; 4097] = [0; 4097];
    depth = depth.wrapping_add(1);
    depth;
    if archive != 0 || ar_name(filename) != 0 {
        lastslash = 0 as *const libc::c_char;
    } else {
        lastslash = memrchr(
            filename as *const libc::c_void,
            '/' as i32,
            namelen.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as *const libc::c_char;
    }
    pathlen = (if !lastslash.is_null() {
        lastslash.offset_from(filename) as libc::c_long
            + 1 as libc::c_int as libc::c_long
    } else {
        0 as libc::c_int as libc::c_long
    }) as size_t;
    nrules = 0 as libc::c_int as libc::c_uint;
    rule = pattern_rules;
    while !rule.is_null() {
        let mut ti: libc::c_uint = 0;
        if !(!((*rule).deps).is_null() && ((*rule).cmds).is_null()) {
            if (*rule).in_use != 0 {
                if 0x8 as libc::c_int & db_level != 0 {
                    print_spaces(depth);
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Avoiding implicit rule recursion for rule '%s'.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        get_rule_defn(rule),
                    );
                    fflush(stdout);
                }
            } else {
                let mut current_block_31: u64;
                ti = 0 as libc::c_int as libc::c_uint;
                while ti < (*rule).num as libc::c_uint {
                    let mut target: *const libc::c_char = *((*rule).targets)
                        .offset(ti as isize);
                    let mut suffix: *const libc::c_char = *((*rule).suffixes)
                        .offset(ti as isize);
                    let mut check_lastslash: libc::c_char = 0;
                    if !(recursions > 0 as libc::c_int as libc::c_uint
                        && *target.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32 && (*rule).terminal == 0)
                    {
                        if !(*((*rule).lens).offset(ti as isize) as libc::c_ulong
                            > namelen)
                        {
                            stem = filename
                                .offset(
                                    (suffix.offset_from(target) as libc::c_long
                                        - 1 as libc::c_int as libc::c_long) as isize,
                                );
                            stemlen = namelen
                                .wrapping_sub(
                                    *((*rule).lens).offset(ti as isize) as libc::c_ulong,
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_ulong);
                            check_lastslash = 0 as libc::c_int as libc::c_char;
                            if !lastslash.is_null() {
                                check_lastslash = (strchr(target, '/' as i32)
                                    == 0 as *mut libc::c_char) as libc::c_int as libc::c_char;
                            }
                            if check_lastslash != 0 {
                                if pathlen > stemlen {
                                    current_block_31 = 16203760046146113240;
                                } else {
                                    stemlen = (stemlen as libc::c_ulong).wrapping_sub(pathlen)
                                        as size_t as size_t;
                                    stem = stem.offset(pathlen as isize);
                                    current_block_31 = 8845338526596852646;
                                }
                            } else {
                                current_block_31 = 8845338526596852646;
                            }
                            match current_block_31 {
                                16203760046146113240 => {}
                                _ => {
                                    if check_lastslash != 0 {
                                        if stem > lastslash.offset(1 as libc::c_int as isize)
                                            && !(strncmp(
                                                target,
                                                lastslash.offset(1 as libc::c_int as isize),
                                                (stem.offset_from(lastslash) as libc::c_long
                                                    - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                                            ) == 0 as libc::c_int)
                                        {
                                            current_block_31 = 16203760046146113240;
                                        } else {
                                            current_block_31 = 10891380440665537214;
                                        }
                                    } else if stem > filename
                                        && !(strncmp(
                                            target,
                                            filename,
                                            stem.offset_from(filename) as libc::c_long as libc::c_ulong,
                                        ) == 0 as libc::c_int)
                                    {
                                        current_block_31 = 16203760046146113240;
                                    } else {
                                        current_block_31 = 10891380440665537214;
                                    }
                                    match current_block_31 {
                                        16203760046146113240 => {}
                                        _ => {
                                            if !(*suffix as libc::c_int
                                                != *stem.offset(stemlen as isize) as libc::c_int
                                                || *suffix as libc::c_int != '\0' as i32
                                                    && !(&*suffix.offset(1 as libc::c_int as isize)
                                                        as *const libc::c_char
                                                        == &*stem
                                                            .offset(
                                                                stemlen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                                    as isize,
                                                            ) as *const libc::c_char
                                                        || *suffix.offset(1 as libc::c_int as isize) as libc::c_int
                                                            == *stem
                                                                .offset(
                                                                    stemlen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                                        as isize,
                                                                ) as libc::c_int
                                                            && (*suffix.offset(1 as libc::c_int as isize) as libc::c_int
                                                                == '\0' as i32
                                                                || strcmp(
                                                                    (&*suffix.offset(1 as libc::c_int as isize)
                                                                        as *const libc::c_char)
                                                                        .offset(1 as libc::c_int as isize),
                                                                    (&*stem
                                                                        .offset(
                                                                            stemlen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                                                as isize,
                                                                        ) as *const libc::c_char)
                                                                        .offset(1 as libc::c_int as isize),
                                                                ) == 0)))
                                            {
                                                if *target.offset(1 as libc::c_int as isize) as libc::c_int
                                                    != '\0' as i32
                                                {
                                                    specific_rule_matched = 1 as libc::c_int;
                                                }
                                                if !(((*rule).deps).is_null() && ((*rule).cmds).is_null()) {
                                                    let ref mut fresh4 = (*tryrules.offset(nrules as isize))
                                                        .rule;
                                                    *fresh4 = rule;
                                                    (*tryrules.offset(nrules as isize)).matches = ti;
                                                    (*tryrules.offset(nrules as isize))
                                                        .stemlen = stemlen
                                                        .wrapping_add(
                                                            (if check_lastslash as libc::c_int != 0 {
                                                                pathlen
                                                            } else {
                                                                0 as libc::c_int as libc::c_ulong
                                                            }),
                                                        );
                                                    (*tryrules.offset(nrules as isize)).order = nrules;
                                                    (*tryrules.offset(nrules as isize))
                                                        .checked_lastslash = check_lastslash;
                                                    nrules = nrules.wrapping_add(1);
                                                    nrules;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ti = ti.wrapping_add(1);
                    ti;
                }
            }
        }
        rule = (*rule).next;
    }
    if !(nrules == 0 as libc::c_int as libc::c_uint) {
        if nrules > 1 as libc::c_int as libc::c_uint {
            qsort(
                tryrules as *mut libc::c_void,
                nrules as size_t,
                ::std::mem::size_of::<tryrule>() as libc::c_ulong,
                Some(
                    stemlen_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
        }
        if specific_rule_matched != 0 {
            ri = 0 as libc::c_int as libc::c_uint;
            while ri < nrules {
                if (*(*tryrules.offset(ri as isize)).rule).terminal == 0 {
                    let mut j: libc::c_uint = 0;
                    j = 0 as libc::c_int as libc::c_uint;
                    while j < (*(*tryrules.offset(ri as isize)).rule).num as libc::c_uint
                    {
                        if *(*((*(*tryrules.offset(ri as isize)).rule).targets)
                            .offset(j as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                        {
                            let ref mut fresh5 = (*tryrules.offset(ri as isize)).rule;
                            *fresh5 = 0 as *mut rule;
                            break;
                        } else {
                            j = j.wrapping_add(1);
                            j;
                        }
                    }
                }
                ri = ri.wrapping_add(1);
                ri;
            }
        }
        intermed_ok = 0 as libc::c_int;
        while intermed_ok < 2 as libc::c_int {
            pat = deplist;
            if intermed_ok != 0 {
                if 0x8 as libc::c_int & db_level != 0 {
                    print_spaces(depth);
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Trying harder.\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fflush(stdout);
                }
            }
            ri = 0 as libc::c_int as libc::c_uint;
            while ri < nrules {
                let mut dep: *mut dep = 0 as *mut dep;
                let mut check_lastslash_0: libc::c_char = 0;
                let mut failed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut file_variables_set: libc::c_int = 0 as libc::c_int;
                let mut deps_found: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut nptr: *const libc::c_char = 0 as *const libc::c_char;
                let mut order_only: libc::c_int = 0 as libc::c_int;
                let mut matches: libc::c_uint = 0;
                rule = (*tryrules.offset(ri as isize)).rule;
                if !rule.is_null() {
                    if !(intermed_ok != 0 && (*rule).terminal as libc::c_int != 0) {
                        matches = (*tryrules.offset(ri as isize)).matches;
                        stem = filename
                            .offset(
                                (*((*rule).suffixes).offset(matches as isize))
                                    .offset_from(*((*rule).targets).offset(matches as isize))
                                    as libc::c_long as isize,
                            )
                            .offset(-(1 as libc::c_int as isize));
                        stemlen = namelen
                            .wrapping_sub(
                                *((*rule).lens).offset(matches as isize) as libc::c_ulong,
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong);
                        check_lastslash_0 = (*tryrules.offset(ri as isize))
                            .checked_lastslash;
                        if check_lastslash_0 != 0 {
                            stem = stem.offset(pathlen as isize);
                            stemlen = (stemlen as libc::c_ulong).wrapping_sub(pathlen)
                                as size_t as size_t;
                            if pathdir.is_null() {
                                let mut fresh6 = ::std::vec::from_elem(
                                    0,
                                    pathlen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as usize,
                                );
                                pathdir = fresh6.leak().as_mut_ptr() as *mut libc::c_char;
                                memcpy(
                                    pathdir as *mut libc::c_void,
                                    filename as *const libc::c_void,
                                    pathlen,
                                );
                                *pathdir
                                    .offset(pathlen as isize) = '\0' as i32 as libc::c_char;
                            }
                        }
                        if 0x8 as libc::c_int & db_level != 0 {
                            print_spaces(depth);
                            printf(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Trying pattern rule '%s' with stem '%.*s'.\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                get_rule_defn(rule),
                                stemlen as libc::c_int,
                                stem,
                            );
                            fflush(stdout);
                        }
                        if stemlen
                            .wrapping_add(
                                (if check_lastslash_0 as libc::c_int != 0 {
                                    pathlen
                                } else {
                                    0 as libc::c_int as libc::c_ulong
                                }),
                            ) > 4096 as libc::c_int as libc::c_ulong
                        {
                            if 0x8 as libc::c_int & db_level != 0 {
                                print_spaces(depth);
                                printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Stem too long: '%s%.*s'.\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    if check_lastslash_0 as libc::c_int != 0 {
                                        pathdir as *const libc::c_char
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                    },
                                    stemlen as libc::c_int,
                                    stem,
                                );
                                fflush(stdout);
                            }
                        } else {
                            if check_lastslash_0 == 0 {
                                memcpy(
                                    stem_str.as_mut_ptr() as *mut libc::c_void,
                                    stem as *const libc::c_void,
                                    stemlen,
                                );
                                stem_str[stemlen as usize] = '\0' as i32 as libc::c_char;
                            } else {
                                memcpy(
                                    stem_str.as_mut_ptr() as *mut libc::c_void,
                                    filename as *const libc::c_void,
                                    pathlen,
                                );
                                memcpy(
                                    stem_str.as_mut_ptr().offset(pathlen as isize)
                                        as *mut libc::c_void,
                                    stem as *const libc::c_void,
                                    stemlen,
                                );
                                stem_str[pathlen.wrapping_add(stemlen)
                                    as usize] = '\0' as i32 as libc::c_char;
                            }
                            if ((*rule).deps).is_null() {
                                break;
                            }
                            (*rule).in_use = 1 as libc::c_int as libc::c_char;
                            pat = deplist;
                            dep = (*rule).deps;
                            nptr = if !((*dep).name).is_null() {
                                (*dep).name
                            } else {
                                (*(*dep).file).name
                            };
                            loop {
                                let mut dl: *mut dep = 0 as *mut dep;
                                let mut d: *mut dep = 0 as *mut dep;
                                if nptr.is_null() {
                                    dep = (*dep).next;
                                    if dep.is_null() {
                                        break;
                                    }
                                    nptr = if !((*dep).name).is_null() {
                                        (*dep).name
                                    } else {
                                        (*(*dep).file).name
                                    };
                                }
                                if (*dep).need_2nd_expansion() == 0 {
                                    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                                    let mut is_explicit: libc::c_int = 1 as libc::c_int;
                                    let mut cp: *const libc::c_char = strchr(nptr, '%' as i32);
                                    if cp.is_null() {
                                        strcpy(depname, nptr);
                                    } else {
                                        let mut o: *mut libc::c_char = depname;
                                        if check_lastslash_0 != 0 {
                                            o = mempcpy(
                                                o as *mut libc::c_void,
                                                filename as *const libc::c_void,
                                                pathlen,
                                            ) as *mut libc::c_char;
                                        }
                                        o = mempcpy(
                                            o as *mut libc::c_void,
                                            nptr as *const libc::c_void,
                                            cp.offset_from(nptr) as libc::c_long as libc::c_ulong,
                                        ) as *mut libc::c_char;
                                        o = mempcpy(
                                            o as *mut libc::c_void,
                                            stem as *const libc::c_void,
                                            stemlen,
                                        ) as *mut libc::c_char;
                                        strcpy(o, cp.offset(1 as libc::c_int as isize));
                                        is_explicit = 0 as libc::c_int;
                                    }
                                    p = depname;
                                    dl = parse_file_seq(
                                        &mut p,
                                        ::std::mem::size_of::<dep>() as libc::c_ulong,
                                        0x1 as libc::c_int,
                                        0 as *const libc::c_char,
                                        0x20 as libc::c_int | 0x40 as libc::c_int,
                                    ) as *mut dep;
                                    d = dl;
                                    while !d.is_null() {
                                        deps_found = deps_found.wrapping_add(1);
                                        deps_found;
                                        (*d).set_ignore_mtime((*dep).ignore_mtime());
                                        (*d)
                                            .set_ignore_automatic_vars((*dep).ignore_automatic_vars());
                                        (*d)
                                            .set_wait_here(
                                                (*d).wait_here()
                                                    | (*dep).wait_here() as libc::c_int as libc::c_uint,
                                            );
                                        (*d).set_is_explicit(is_explicit as libc::c_uint);
                                        d = (*d).next;
                                    }
                                    nptr = 0 as *const libc::c_char;
                                } else {
                                    let mut add_dir: libc::c_int = 0 as libc::c_int;
                                    let mut len: size_t = 0;
                                    let mut end: *const libc::c_char = 0 as *const libc::c_char;
                                    let mut dptr: *mut *mut dep = 0 as *mut *mut dep;
                                    let mut is_explicit_0: libc::c_int = 0;
                                    let mut cp_0: *const libc::c_char = 0
                                        as *const libc::c_char;
                                    let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
                                    nptr = get_next_word(nptr, &mut len);
                                    if nptr.is_null() {
                                        continue;
                                    }
                                    end = nptr.offset(len as isize);
                                    if order_only == 0
                                        && len == 1 as libc::c_int as libc::c_ulong
                                        && *nptr.offset(0 as libc::c_int as isize) as libc::c_int
                                            == '|' as i32
                                    {
                                        order_only = 1 as libc::c_int;
                                        nptr = end;
                                        continue;
                                    } else {
                                        cp_0 = lindex(nptr, end, '%' as i32);
                                        if cp_0.is_null() {
                                            memcpy(
                                                depname as *mut libc::c_void,
                                                nptr as *const libc::c_void,
                                                len,
                                            );
                                            *depname.offset(len as isize) = '\0' as i32 as libc::c_char;
                                            is_explicit_0 = 1 as libc::c_int;
                                        } else {
                                            let mut o_0: *mut libc::c_char = depname;
                                            is_explicit_0 = 0 as libc::c_int;
                                            loop {
                                                let mut i: size_t = cp_0.offset_from(nptr) as libc::c_long
                                                    as size_t;
                                                o_0 = mempcpy(
                                                    o_0 as *mut libc::c_void,
                                                    nptr as *const libc::c_void,
                                                    i,
                                                ) as *mut libc::c_char;
                                                if check_lastslash_0 != 0 {
                                                    add_dir = 1 as libc::c_int;
                                                    o_0 = mempcpy(
                                                        o_0 as *mut libc::c_void,
                                                        b"$(*F)\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        5 as libc::c_int as libc::c_ulong,
                                                    ) as *mut libc::c_char;
                                                } else {
                                                    o_0 = mempcpy(
                                                        o_0 as *mut libc::c_void,
                                                        b"$*\0" as *const u8 as *const libc::c_char
                                                            as *const libc::c_void,
                                                        2 as libc::c_int as libc::c_ulong,
                                                    ) as *mut libc::c_char;
                                                }
                                                cp_0 = cp_0.offset(1);
                                                cp_0;
                                                nptr = cp_0;
                                                if nptr == end {
                                                    break;
                                                }
                                                while cp_0 < end
                                                    && !(*stopchar_map
                                                        .as_mut_ptr()
                                                        .offset(*cp_0 as libc::c_uchar as isize) as libc::c_int
                                                        & (0x2 as libc::c_int | 0x4 as libc::c_int
                                                            | 0x1 as libc::c_int) != 0 as libc::c_int)
                                                {
                                                    cp_0 = cp_0.offset(1);
                                                    cp_0;
                                                }
                                                cp_0 = lindex(cp_0, end, '%' as i32);
                                                if cp_0.is_null() {
                                                    break;
                                                }
                                            }
                                            len = end.offset_from(nptr) as libc::c_long as size_t;
                                            memcpy(
                                                o_0 as *mut libc::c_void,
                                                nptr as *const libc::c_void,
                                                len,
                                            );
                                            *o_0.offset(len as isize) = '\0' as i32 as libc::c_char;
                                        }
                                        nptr = end;
                                        if file_vars_initialized == 0 {
                                            initialize_file_variables(file, 0 as libc::c_int);
                                            set_file_variables(file, stem_str.as_mut_ptr());
                                            file_vars_initialized = 1 as libc::c_int;
                                        } else if file_variables_set == 0 {
                                            define_variable_in_set(
                                                b"*\0" as *const u8 as *const libc::c_char,
                                                1 as libc::c_int as size_t,
                                                stem_str.as_mut_ptr(),
                                                o_automatic,
                                                0 as libc::c_int,
                                                (*(*file).variables).set,
                                                0 as *mut floc,
                                            );
                                            file_variables_set = 1 as libc::c_int;
                                        }
                                        p_0 = variable_expand_for_file(depname, file);
                                        dptr = &mut dl;
                                        loop {
                                            let mut dp: *mut dep = parse_file_seq(
                                                &mut p_0,
                                                ::std::mem::size_of::<dep>() as libc::c_ulong,
                                                if order_only != 0 {
                                                    0x1 as libc::c_int
                                                } else {
                                                    0x100 as libc::c_int
                                                },
                                                if add_dir != 0 { pathdir } else { 0 as *mut libc::c_char },
                                                0x40 as libc::c_int,
                                            ) as *mut dep;
                                            *dptr = dp;
                                            d = dp;
                                            while !d.is_null() {
                                                deps_found = deps_found.wrapping_add(1);
                                                deps_found;
                                                if order_only != 0 {
                                                    (*d).set_ignore_mtime(1 as libc::c_int as libc::c_uint);
                                                }
                                                (*d).set_is_explicit(is_explicit_0 as libc::c_uint);
                                                dptr = &mut (*d).next;
                                                d = (*d).next;
                                            }
                                            if *p_0 as libc::c_int == '|' as i32 {
                                                order_only = 1 as libc::c_int;
                                                p_0 = p_0.offset(1);
                                                p_0;
                                            }
                                            if !(*p_0 as libc::c_int != '\0' as i32) {
                                                break;
                                            }
                                        }
                                    }
                                }
                                if deps_found > max_deps {
                                    let mut l: size_t = pat.offset_from(deplist) as libc::c_long
                                        as size_t;
                                    max_pattern_deps = if max_pattern_deps > deps_found {
                                        max_pattern_deps
                                    } else {
                                        deps_found
                                    };
                                    max_deps = max_pattern_deps;
                                    deplist = xrealloc(
                                        deplist as *mut libc::c_void,
                                        (max_deps as libc::c_ulong)
                                            .wrapping_mul(
                                                ::std::mem::size_of::<patdeps>() as libc::c_ulong,
                                            ),
                                    ) as *mut patdeps;
                                    pat = deplist.offset(l as isize);
                                }
                                let mut current_block_294: u64;
                                d = dl;
                                while !d.is_null() {
                                    let mut df: *mut file = 0 as *mut file;
                                    let mut is_rule: libc::c_int = ((*d).name
                                        == (if !((*dep).name).is_null() {
                                            (*dep).name
                                        } else {
                                            (*(*dep).file).name
                                        })) as libc::c_int;
                                    let mut explicit: libc::c_int = 0 as libc::c_int;
                                    let mut dp_0: *mut dep = 0 as *mut dep;
                                    if file_impossible_p((*d).name) != 0 {
                                        if 0x8 as libc::c_int & db_level != 0 {
                                            print_spaces(depth);
                                            printf(
                                                if is_rule != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Rejecting rule '%s' due to impossible rule prerequisite '%s'.\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Rejecting rule '%s' due to impossible implicit prerequisite '%s'.\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                get_rule_defn(rule),
                                                (*d).name,
                                            );
                                            fflush(stdout);
                                        }
                                        let ref mut fresh7 = (*tryrules.offset(ri as isize)).rule;
                                        *fresh7 = 0 as *mut rule;
                                        failed = 1 as libc::c_int as libc::c_uint;
                                        break;
                                    } else {
                                        memset(
                                            pat as *mut libc::c_void,
                                            '\0' as i32,
                                            ::std::mem::size_of::<patdeps>() as libc::c_ulong,
                                        );
                                        (*pat).set_ignore_mtime((*d).ignore_mtime());
                                        (*pat)
                                            .set_ignore_automatic_vars((*d).ignore_automatic_vars());
                                        (*pat).set_wait_here((*d).wait_here());
                                        (*pat).set_is_explicit((*d).is_explicit());
                                        if 0x8 as libc::c_int & db_level != 0 {
                                            print_spaces(depth);
                                            printf(
                                                if is_rule != 0 {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Trying rule prerequisite '%s'.\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                } else {
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Trying implicit prerequisite '%s'.\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    )
                                                },
                                                (*d).name,
                                            );
                                            fflush(stdout);
                                        }
                                        df = lookup_file((*d).name);
                                        if !df.is_null() && (*df).is_explicit() as libc::c_int != 0
                                        {
                                            (*pat).set_is_explicit(1 as libc::c_int as libc::c_uint);
                                        }
                                        if !df.is_null() && (*df).is_explicit() == 0
                                            && (*d).is_explicit() == 0
                                        {
                                            (*df).set_intermediate(1 as libc::c_int as libc::c_uint);
                                        }
                                        if !df.is_null() && (*df).is_target() as libc::c_int != 0 {
                                            explicit = 1 as libc::c_int;
                                        } else {
                                            dp_0 = (*file).deps;
                                            while !dp_0.is_null() {
                                                if (*d).name
                                                    == (if !((*dp_0).name).is_null() {
                                                        (*dp_0).name
                                                    } else {
                                                        (*(*dp_0).file).name
                                                    })
                                                    || *(*d).name as libc::c_int
                                                        == *(if !((*dp_0).name).is_null() {
                                                            (*dp_0).name
                                                        } else {
                                                            (*(*dp_0).file).name
                                                        }) as libc::c_int
                                                        && (*(*d).name as libc::c_int == '\0' as i32
                                                            || strcmp(
                                                                ((*d).name).offset(1 as libc::c_int as isize),
                                                                (if !((*dp_0).name).is_null() {
                                                                    (*dp_0).name
                                                                } else {
                                                                    (*(*dp_0).file).name
                                                                })
                                                                    .offset(1 as libc::c_int as isize),
                                                            ) == 0)
                                                {
                                                    break;
                                                }
                                                dp_0 = (*dp_0).next;
                                            }
                                        }
                                        if explicit != 0 || !dp_0.is_null() {
                                            let fresh8 = pat;
                                            pat = pat.offset(1);
                                            (*fresh8).name = (*d).name;
                                            if 0x8 as libc::c_int & db_level != 0 {
                                                print_spaces(depth);
                                                printf(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"'%s' ought to exist.\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    (*d).name,
                                                );
                                                fflush(stdout);
                                            }
                                        } else if file_exists_p((*d).name) != 0 {
                                            let fresh9 = pat;
                                            pat = pat.offset(1);
                                            (*fresh9).name = (*d).name;
                                            if 0x8 as libc::c_int & db_level != 0 {
                                                print_spaces(depth);
                                                printf(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Found '%s'.\n\0" as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    (*d).name,
                                                );
                                                fflush(stdout);
                                            }
                                        } else if !df.is_null() && allow_compat_rules != 0 {
                                            let fresh10 = pat;
                                            pat = pat.offset(1);
                                            (*fresh10).name = (*d).name;
                                            if 0x8 as libc::c_int & db_level != 0 {
                                                print_spaces(depth);
                                                printf(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Using compatibility rule '%s' due to '%s'.\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    get_rule_defn(rule),
                                                    (*d).name,
                                                );
                                                fflush(stdout);
                                            }
                                        } else {
                                            if !df.is_null() {
                                                if 0x8 as libc::c_int & db_level != 0 {
                                                    print_spaces(depth);
                                                    printf(
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"Prerequisite '%s' of rule '%s' does not qualify as ought to exist.\n\0"
                                                                as *const u8 as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                        (*d).name,
                                                        get_rule_defn(rule),
                                                    );
                                                    fflush(stdout);
                                                }
                                                found_compat_rule = 1 as libc::c_int;
                                            }
                                            let mut vname: *const libc::c_char = vpath_search(
                                                (*d).name,
                                                0 as *mut uintmax_t,
                                                0 as *mut libc::c_uint,
                                                0 as *mut libc::c_uint,
                                            );
                                            if !vname.is_null() {
                                                if 0x8 as libc::c_int & db_level != 0 {
                                                    print_spaces(depth);
                                                    printf(
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"Found prerequisite '%s' as VPATH '%s'.\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                        (*d).name,
                                                        vname,
                                                    );
                                                    fflush(stdout);
                                                }
                                                let fresh11 = pat;
                                                pat = pat.offset(1);
                                                (*fresh11).name = (*d).name;
                                            } else {
                                                if intermed_ok != 0 {
                                                    if 0x8 as libc::c_int & db_level != 0 {
                                                        print_spaces(depth);
                                                        printf(
                                                            if (*d).is_explicit() as libc::c_int != 0
                                                                || !df.is_null() && (*df).is_explicit() as libc::c_int != 0
                                                            {
                                                                dcgettext(
                                                                    0 as *const libc::c_char,
                                                                    b"Looking for a rule with explicit file '%s'.\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    5 as libc::c_int,
                                                                )
                                                            } else {
                                                                dcgettext(
                                                                    0 as *const libc::c_char,
                                                                    b"Looking for a rule with intermediate file '%s'.\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    5 as libc::c_int,
                                                                )
                                                            },
                                                            (*d).name,
                                                        );
                                                        fflush(stdout);
                                                    }
                                                    if int_file.is_null() {
                                                        let mut fresh12 = ::std::vec::from_elem(
                                                            0,
                                                            ::std::mem::size_of::<file>() as libc::c_ulong as usize,
                                                        );
                                                        int_file = fresh12.leak().as_mut_ptr() as *mut file;
                                                    }
                                                    memset(
                                                        int_file as *mut libc::c_void,
                                                        '\0' as i32,
                                                        ::std::mem::size_of::<file>() as libc::c_ulong,
                                                    );
                                                    (*int_file).name = (*d).name;
                                                    if pattern_search(
                                                        int_file,
                                                        0 as libc::c_int,
                                                        depth,
                                                        recursions.wrapping_add(1 as libc::c_int as libc::c_uint),
                                                        allow_compat_rules,
                                                    ) != 0
                                                    {
                                                        (*pat).pattern = (*int_file).name;
                                                        (*int_file).name = (*d).name;
                                                        (*pat).file = int_file;
                                                        int_file = 0 as *mut file;
                                                        let fresh13 = pat;
                                                        pat = pat.offset(1);
                                                        (*fresh13).name = (*d).name;
                                                        current_block_294 = 2277602629737488951;
                                                    } else {
                                                        if !((*int_file).variables).is_null() {
                                                            free_variable_set((*int_file).variables);
                                                        }
                                                        if !((*int_file).pat_variables).is_null() {
                                                            free_variable_set((*int_file).pat_variables);
                                                        }
                                                        if df.is_null() {
                                                            file_impossible((*d).name);
                                                        }
                                                        current_block_294 = 7344615536999694015;
                                                    }
                                                } else {
                                                    current_block_294 = 7344615536999694015;
                                                }
                                                match current_block_294 {
                                                    2277602629737488951 => {}
                                                    _ => {
                                                        if intermed_ok != 0 {
                                                            if 0x8 as libc::c_int & db_level != 0 {
                                                                print_spaces(depth);
                                                                printf(
                                                                    dcgettext(
                                                                        0 as *const libc::c_char,
                                                                        b"Rejecting rule '%s' due to impossible prerequisite '%s'.\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        5 as libc::c_int,
                                                                    ),
                                                                    get_rule_defn(rule),
                                                                    (*d).name,
                                                                );
                                                                fflush(stdout);
                                                            }
                                                        } else if 0x8 as libc::c_int & db_level != 0 {
                                                            print_spaces(depth);
                                                            printf(
                                                                dcgettext(
                                                                    0 as *const libc::c_char,
                                                                    b"Not found '%s'.\n\0" as *const u8 as *const libc::c_char,
                                                                    5 as libc::c_int,
                                                                ),
                                                                (*d).name,
                                                            );
                                                            fflush(stdout);
                                                        }
                                                        failed = 1 as libc::c_int as libc::c_uint;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        d = (*d).next;
                                    }
                                }
                                free_ns_chain(dl as *mut nameseq);
                                if failed != 0 {
                                    break;
                                }
                            }
                            (*rule).in_use = 0 as libc::c_int as libc::c_char;
                            if failed == 0 {
                                break;
                            }
                        }
                    }
                }
                ri = ri.wrapping_add(1);
                ri;
            }
            if ri < nrules {
                break;
            }
            rule = 0 as *mut rule;
            intermed_ok += 1;
            intermed_ok;
        }
        if !rule.is_null() {
            foundrule = ri;
            if recursions > 0 as libc::c_int as libc::c_uint {
                (*file)
                    .name = *((*rule).targets)
                    .offset((*tryrules.offset(foundrule as isize)).matches as isize);
            }
            loop {
                let fresh14 = pat;
                pat = pat.offset(-1);
                if !(fresh14 > deplist) {
                    break;
                }
                let mut dep_0: *mut dep = 0 as *mut dep;
                let mut s: *const libc::c_char = 0 as *const libc::c_char;
                if !((*pat).file).is_null() {
                    let mut imf: *mut file = (*pat).file;
                    let mut f: *mut file = lookup_file((*imf).name);
                    if f.is_null() {
                        f = enter_file((*imf).name);
                    }
                    (*f).deps = (*imf).deps;
                    (*f).cmds = (*imf).cmds;
                    (*f).stem = (*imf).stem;
                    merge_variable_set_lists(&mut (*f).variables, (*imf).variables);
                    (*f).pat_variables = (*imf).pat_variables;
                    (*f).set_pat_searched((*imf).pat_searched());
                    (*f).also_make = (*imf).also_make;
                    (*f).set_is_target(1 as libc::c_int as libc::c_uint);
                    (*f)
                        .set_is_explicit(
                            (*f).is_explicit()
                                | ((*imf).is_explicit() as libc::c_int != 0
                                    || (*pat).is_explicit() as libc::c_int != 0) as libc::c_int
                                    as libc::c_uint,
                        );
                    (*f)
                        .set_notintermediate(
                            (*f).notintermediate()
                                | ((*imf).notintermediate() as libc::c_int != 0
                                    || no_intermediates != 0) as libc::c_int as libc::c_uint,
                        );
                    (*f)
                        .set_intermediate(
                            (*f).intermediate()
                                | ((*f).is_explicit() == 0 && (*f).notintermediate() == 0)
                                    as libc::c_int as libc::c_uint,
                        );
                    (*f).set_tried_implicit(1 as libc::c_int as libc::c_uint);
                    imf = lookup_file((*pat).pattern);
                    if !imf.is_null() && (*imf).precious() as libc::c_int != 0 {
                        (*f).set_precious(1 as libc::c_int as libc::c_uint);
                    }
                    dep_0 = (*f).deps;
                    while !dep_0.is_null() {
                        (*dep_0).file = enter_file((*dep_0).name);
                        (*dep_0).name = 0 as *const libc::c_char;
                        (*(*dep_0).file)
                            .set_tried_implicit(
                                (*(*dep_0).file).tried_implicit()
                                    | (*dep_0).changed() as libc::c_int as libc::c_uint,
                            );
                        dep_0 = (*dep_0).next;
                    }
                }
                dep_0 = xcalloc(::std::mem::size_of::<dep>() as libc::c_ulong)
                    as *mut dep;
                (*dep_0).set_ignore_mtime((*pat).ignore_mtime());
                (*dep_0).set_is_explicit((*pat).is_explicit());
                (*dep_0).set_ignore_automatic_vars((*pat).ignore_automatic_vars());
                (*dep_0).set_wait_here((*pat).wait_here());
                s = strcache_add((*pat).name);
                if recursions != 0 {
                    (*dep_0).name = s;
                } else {
                    (*dep_0).file = lookup_file(s);
                    if ((*dep_0).file).is_null() {
                        (*dep_0).file = enter_file(s);
                    }
                }
                if ((*pat).file).is_null()
                    && (*(*tryrules.offset(foundrule as isize)).rule).terminal
                        as libc::c_int != 0
                {
                    if ((*dep_0).file).is_null() {
                        (*dep_0).set_changed(1 as libc::c_int as libc::c_uint);
                    } else {
                        (*(*dep_0).file)
                            .set_tried_implicit(1 as libc::c_int as libc::c_uint);
                    }
                }
                (*dep_0).next = (*file).deps;
                (*file).deps = dep_0;
                (*file).set_was_shuffled(0 as libc::c_int as libc::c_uint);
            }
            if (*file).was_shuffled() == 0 {
                shuffle_deps_recursive((*file).deps);
            }
            if (*tryrules.offset(foundrule as isize)).checked_lastslash == 0 {
                (*file).stem = strcache_add_len(stem, stemlen);
                fullstemlen = stemlen;
            } else {
                fullstemlen = pathlen.wrapping_add(stemlen);
                memcpy(
                    stem_str.as_mut_ptr() as *mut libc::c_void,
                    filename as *const libc::c_void,
                    pathlen,
                );
                memcpy(
                    stem_str.as_mut_ptr().offset(pathlen as isize) as *mut libc::c_void,
                    stem as *const libc::c_void,
                    stemlen,
                );
                stem_str[fullstemlen as usize] = '\0' as i32 as libc::c_char;
                (*file).stem = strcache_add(stem_str.as_mut_ptr());
            }
            (*file).cmds = (*rule).cmds;
            (*file).set_is_target(1 as libc::c_int as libc::c_uint);
            let mut f_0: *mut file = lookup_file(
                *((*rule).targets)
                    .offset((*tryrules.offset(foundrule as isize)).matches as isize),
            );
            if !f_0.is_null() {
                if (*f_0).precious() != 0 {
                    (*file).set_precious(1 as libc::c_int as libc::c_uint);
                }
                if (*f_0).notintermediate() as libc::c_int != 0 || no_intermediates != 0
                {
                    (*file).set_notintermediate(1 as libc::c_int as libc::c_uint);
                }
            }
            if (*rule).num as libc::c_int > 1 as libc::c_int {
                ri = 0 as libc::c_int as libc::c_uint;
                while ri < (*rule).num as libc::c_uint {
                    if ri != (*tryrules.offset(foundrule as isize)).matches {
                        let mut fresh15 = ::std::vec::from_elem(
                            0,
                            (*((*rule).lens).offset(ri as isize) as libc::c_ulong)
                                .wrapping_add(fullstemlen)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
                        );
                        let mut nm: *mut libc::c_char = fresh15.leak().as_mut_ptr()
                            as *mut libc::c_char;
                        let mut p_1: *mut libc::c_char = nm;
                        let mut f_1: *mut file = 0 as *mut file;
                        let mut new: *mut dep = xcalloc(
                            ::std::mem::size_of::<dep>() as libc::c_ulong,
                        ) as *mut dep;
                        p_1 = mempcpy(
                            p_1 as *mut libc::c_void,
                            *((*rule).targets).offset(ri as isize)
                                as *const libc::c_void,
                            ((*((*rule).suffixes).offset(ri as isize))
                                .offset_from(*((*rule).targets).offset(ri as isize))
                                as libc::c_long - 1 as libc::c_int as libc::c_long)
                                as libc::c_ulong,
                        ) as *mut libc::c_char;
                        p_1 = mempcpy(
                            p_1 as *mut libc::c_void,
                            (*file).stem as *const libc::c_void,
                            fullstemlen,
                        ) as *mut libc::c_char;
                        memcpy(
                            p_1 as *mut libc::c_void,
                            *((*rule).suffixes).offset(ri as isize)
                                as *const libc::c_void,
                            (*((*rule).lens).offset(ri as isize) as libc::c_long
                                - (*((*rule).suffixes).offset(ri as isize))
                                    .offset_from(*((*rule).targets).offset(ri as isize))
                                    as libc::c_long + 1 as libc::c_int as libc::c_long)
                                as libc::c_ulong,
                        );
                        (*new).name = strcache_add(nm);
                        (*new).file = enter_file((*new).name);
                        (*new).next = (*file).also_make;
                        f_1 = lookup_file(*((*rule).targets).offset(ri as isize));
                        if !f_1.is_null() {
                            if (*f_1).precious() != 0 {
                                (*(*new).file)
                                    .set_precious(1 as libc::c_int as libc::c_uint);
                            }
                            if (*f_1).notintermediate() as libc::c_int != 0
                                || no_intermediates != 0
                            {
                                (*(*new).file)
                                    .set_notintermediate(1 as libc::c_int as libc::c_uint);
                            }
                        }
                        (*(*new).file).set_is_target(1 as libc::c_int as libc::c_uint);
                        (*file).also_make = new;
                    }
                    ri = ri.wrapping_add(1);
                    ri;
                }
            }
        }
    }
    free(tryrules as *mut libc::c_void);
    free(deplist as *mut libc::c_void);
    depth = depth.wrapping_sub(1);
    depth;
    if !rule.is_null() {
        if 0x8 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Found implicit rule '%s' for '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                get_rule_defn(rule),
                filename,
            );
            fflush(stdout);
        }
        return 1 as libc::c_int;
    }
    if found_compat_rule != 0 {
        if 0x8 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Searching for a compatibility rule for '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
            fflush(stdout);
        }
        return pattern_search(file, archive, depth, recursions, 1 as libc::c_int);
    }
    if 0x8 as libc::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"No implicit rule found for '%s'.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            filename,
        );
        fflush(stdout);
    }
    return 0 as libc::c_int;
}
