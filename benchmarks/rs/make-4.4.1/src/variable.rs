use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut make_host: *mut libc::c_char;
    static mut remote_description: *mut libc::c_char;
    static mut version_string: *mut libc::c_char;
    static mut makelevel: libc::c_uint;
    static mut jobserver_auth: *mut libc::c_char;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    static mut cmd_prefix: libc::c_char;
    static mut default_shell: *const libc::c_char;
    fn concat(_: libc::c_uint, _: ...) -> *const libc::c_char;
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn reset_makeflags(origin: variable_origin);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    fn next_token(_: *const libc::c_char) -> *mut libc::c_char;
    static mut reading_file: *const floc;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut env_overrides: libc::c_int;
    static mut warn_undefined_variables_flag: libc::c_int;
    fn hash_init(
        ht: *mut hash_table,
        size: libc::c_ulong,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_delete_at(
        ht: *mut hash_table,
        slot: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const libc::c_void,
        slot: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut *mut libc::c_void;
    fn hash_print_stats(ht: *mut hash_table, out_FILE: *mut FILE);
    fn hash_free(ht: *mut hash_table, free_items: libc::c_int);
    fn hash_map_arg(
        ht: *mut hash_table,
        map: hash_map_arg_func_t,
        arg: *mut libc::c_void,
    );
    fn hash_map(ht: *mut hash_table, map: hash_map_func_t);
    fn jhash(key: *const libc::c_uchar, n: libc::c_int) -> libc::c_uint;
    static mut hash_deleted_item: *mut libc::c_void;
    static mut variable_buffer: *mut libc::c_char;
    static mut shell_var: variable;
    fn variable_buffer_output(
        ptr: *mut libc::c_char,
        string: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn allocated_variable_expand_for_file(
        line: *const libc::c_char,
        file: *mut file,
    ) -> *mut libc::c_char;
    fn install_variable_buffer(bufp: *mut *mut libc::c_char, lenp: *mut size_t);
    fn restore_variable_buffer(buf: *mut libc::c_char, len: size_t);
    fn func_shell_base(
        o: *mut libc::c_char,
        argv: *mut *mut libc::c_char,
        trim_newlines: libc::c_int,
    ) -> *mut libc::c_char;
    fn recursively_expand_for_file(
        v: *mut variable,
        file: *mut file,
    ) -> *mut libc::c_char;
    fn jobserver_get_invalid_auth() -> *const libc::c_char;
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
pub struct pattern_var {
    pub next: *mut pattern_var,
    pub suffix: *const libc::c_char,
    pub target: *const libc::c_char,
    pub len: size_t,
    pub variable: variable,
}
pub type hash_map_arg_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
>;
pub type hash_map_func_t = Option::<unsafe extern "C" fn(*const libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct defined_vars {
    pub name: *const libc::c_char,
    pub len: size_t,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub static mut env_recursion: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
static mut variable_changenum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
static mut pattern_vars: *mut pattern_var = 0 as *const pattern_var as *mut pattern_var;
static mut last_pattern_vars: [*mut pattern_var; 256] = [0 as *const pattern_var
    as *mut pattern_var; 256];
pub unsafe extern "C" fn create_pattern_var(
    mut target: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> *mut pattern_var {
    let mut len: size_t = strlen(target);
    let mut p: *mut pattern_var = xcalloc(
        ::std::mem::size_of::<pattern_var>() as libc::c_ulong,
    ) as *mut pattern_var;
    if !pattern_vars.is_null() {
        if len < 256 as libc::c_int as libc::c_ulong
            && !(last_pattern_vars[len as usize]).is_null()
        {
            (*p).next = (*last_pattern_vars[len as usize]).next;
            (*last_pattern_vars[len as usize]).next = p;
        } else {
            let mut v: *mut *mut pattern_var = 0 as *mut *mut pattern_var;
            v = &mut pattern_vars;
            loop {
                if (*v).is_null() || (**v).len > len {
                    (*p).next = *v;
                    *v = p;
                    break;
                } else {
                    v = &mut (**v).next;
                }
            }
        }
    } else {
        pattern_vars = p;
        (*p).next = 0 as *mut pattern_var;
    }
    (*p).target = target;
    (*p).len = len;
    (*p).suffix = suffix.offset(1 as libc::c_int as isize);
    if len < 256 as libc::c_int as libc::c_ulong {
        last_pattern_vars[len as usize] = p;
    }
    return p;
}
unsafe extern "C" fn lookup_pattern_var(
    mut start: *mut pattern_var,
    mut target: *const libc::c_char,
    mut targlen: size_t,
) -> *mut pattern_var {
    let mut p: *mut pattern_var = 0 as *mut pattern_var;
    p = if !start.is_null() { (*start).next } else { pattern_vars };
    while !p.is_null() {
        let mut stem: *const libc::c_char = 0 as *const libc::c_char;
        let mut stemlen: size_t = 0;
        if !((*p).len > targlen) {
            stem = target
                .offset(
                    (((*p).suffix).offset_from((*p).target) as libc::c_long
                        - 1 as libc::c_int as libc::c_long) as isize,
                );
            stemlen = targlen
                .wrapping_sub((*p).len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            if !(stem > target
                && !(strncmp(
                    (*p).target,
                    target,
                    stem.offset_from(target) as libc::c_long as libc::c_ulong,
                ) == 0 as libc::c_int))
            {
                if *(*p).suffix as libc::c_int
                    == *stem.offset(stemlen as isize) as libc::c_int
                    && (*(*p).suffix as libc::c_int == '\0' as i32
                        || (&*((*p).suffix).offset(1 as libc::c_int as isize)
                            as *const libc::c_char
                            == &*stem
                                .offset(
                                    stemlen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as *const libc::c_char
                            || *((*p).suffix).offset(1 as libc::c_int as isize)
                                as libc::c_int
                                == *stem
                                    .offset(
                                        stemlen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int
                                && (*((*p).suffix).offset(1 as libc::c_int as isize)
                                    as libc::c_int == '\0' as i32
                                    || strcmp(
                                        (&*((*p).suffix).offset(1 as libc::c_int as isize)
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
                    break;
                }
            }
        }
        p = (*p).next;
    }
    return p;
}
unsafe extern "C" fn variable_hash_1(mut keyv: *const libc::c_void) -> libc::c_ulong {
    let mut key: *const variable = keyv as *const variable;
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut _key_: *const libc::c_uchar = (*key).name as *const libc::c_uchar;
    _result_ = _result_
        .wrapping_add(jhash(_key_, (*key).length as libc::c_int) as libc::c_ulong);
    return _result_;
}
unsafe extern "C" fn variable_hash_2(mut keyv: *const libc::c_void) -> libc::c_ulong {
    let mut key: *const variable = keyv as *const variable;
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    return _result_;
}
unsafe extern "C" fn variable_hash_cmp(
    mut xv: *const libc::c_void,
    mut yv: *const libc::c_void,
) -> libc::c_int {
    let mut x: *const variable = xv as *const variable;
    let mut y: *const variable = yv as *const variable;
    let mut result: libc::c_int = ((*x).length).wrapping_sub((*y).length) as libc::c_int;
    if result != 0 {
        return result;
    }
    return if (*x).name == (*y).name {
        0 as libc::c_int
    } else {
        memcmp(
            (*x).name as *const libc::c_void,
            (*y).name as *const libc::c_void,
            (*x).length as libc::c_ulong,
        )
    };
}
static mut global_variable_set: variable_set = variable_set {
    table: hash_table {
        ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
        ht_hash_1: None,
        ht_hash_2: None,
        ht_compare: None,
        ht_size: 0,
        ht_capacity: 0,
        ht_fill: 0,
        ht_empty_slots: 0,
        ht_collisions: 0,
        ht_lookups: 0,
        ht_rehashes: 0,
    },
};
static mut global_setlist: variable_set_list = unsafe {
    {
        let mut init = variable_set_list {
            next: 0 as *const variable_set_list as *mut variable_set_list,
            set: &global_variable_set as *const variable_set as *mut variable_set,
            next_is_parent: 0 as libc::c_int,
        };
        init
    }
};
pub static mut current_variable_set_list: *mut variable_set_list = unsafe {
    &global_setlist as *const variable_set_list as *mut variable_set_list
};
pub unsafe extern "C" fn init_hash_global_variable_set() {
    hash_init(
        &mut global_variable_set.table,
        523 as libc::c_int as libc::c_ulong,
        Some(
            variable_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            variable_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            variable_hash_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn define_variable_in_set(
    mut name: *const libc::c_char,
    mut length: size_t,
    mut value: *const libc::c_char,
    mut origin: variable_origin,
    mut recursive: libc::c_int,
    mut set: *mut variable_set,
    mut flocp: *const floc,
) -> *mut variable {
    let mut v: *mut variable = 0 as *mut variable;
    let mut var_slot: *mut *mut variable = 0 as *mut *mut variable;
    let mut var_key: variable = variable {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        value: 0 as *const libc::c_char as *mut libc::c_char,
        fileinfo: floc {
            filenm: 0 as *const libc::c_char,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    if set.is_null() {
        set = &mut global_variable_set;
    }
    var_key.name = name as *mut libc::c_char;
    var_key.length = length as libc::c_uint;
    var_slot = hash_find_slot(
        &mut (*set).table,
        &mut var_key as *mut variable as *const libc::c_void,
    ) as *mut *mut variable;
    v = *var_slot;
    if env_overrides != 0
        && origin as libc::c_uint == o_env as libc::c_int as libc::c_uint
    {
        origin = o_env_override;
    }
    if !(v.is_null() || v as *mut libc::c_void == hash_deleted_item) {
        if env_overrides != 0 && (*v).origin() as libc::c_int == o_env as libc::c_int {
            (*v).set_origin(o_env_override);
        }
        if origin as libc::c_int >= (*v).origin() as libc::c_int {
            free((*v).value as *mut libc::c_void);
            (*v).value = xstrdup(value);
            if !flocp.is_null() {
                (*v).fileinfo = *flocp;
            } else {
                (*v).fileinfo.filenm = 0 as *const libc::c_char;
            }
            (*v).set_origin(origin);
            (*v).set_recursive(recursive as libc::c_uint);
        }
        return v;
    }
    v = xcalloc(::std::mem::size_of::<variable>() as libc::c_ulong) as *mut variable;
    (*v).name = xstrndup(name, length);
    (*v).length = length as libc::c_uint;
    hash_insert_at(
        &mut (*set).table,
        v as *const libc::c_void,
        var_slot as *const libc::c_void,
    );
    if set == &mut global_variable_set as *mut variable_set {
        variable_changenum = variable_changenum.wrapping_add(1);
        variable_changenum;
    }
    (*v).value = xstrdup(value);
    if !flocp.is_null() {
        (*v).fileinfo = *flocp;
    }
    (*v).set_origin(origin);
    (*v).set_recursive(recursive as libc::c_uint);
    (*v).set_export(v_default);
    (*v).set_exportable(1 as libc::c_int as libc::c_uint);
    name = (*v).name;
    if *name as libc::c_int != '_' as i32
        && ((*name as libc::c_int) < 'A' as i32 || *name as libc::c_int > 'Z' as i32)
        && ((*name as libc::c_int) < 'a' as i32 || *name as libc::c_int > 'z' as i32)
    {
        (*v).set_exportable(0 as libc::c_int as libc::c_uint);
    } else {
        name = name.offset(1);
        name;
        while *name as libc::c_int != '\0' as i32 {
            if *name as libc::c_int != '_' as i32
                && ((*name as libc::c_int) < 'a' as i32
                    || *name as libc::c_int > 'z' as i32)
                && ((*name as libc::c_int) < 'A' as i32
                    || *name as libc::c_int > 'Z' as i32)
                && !((*name as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                    <= 9 as libc::c_int as libc::c_uint)
            {
                break;
            }
            name = name.offset(1);
            name;
        }
        if *name as libc::c_int != '\0' as i32 {
            (*v).set_exportable(0 as libc::c_int as libc::c_uint);
        }
    }
    return v;
}
unsafe extern "C" fn free_variable_name_and_value(mut item: *const libc::c_void) {
    let mut v: *mut variable = item as *mut variable;
    free((*v).name as *mut libc::c_void);
    free((*v).value as *mut libc::c_void);
}
pub unsafe extern "C" fn free_variable_set(mut list: *mut variable_set_list) {
    hash_map(
        &mut (*(*list).set).table,
        Some(
            free_variable_name_and_value
                as unsafe extern "C" fn(*const libc::c_void) -> (),
        ),
    );
    hash_free(&mut (*(*list).set).table, 1 as libc::c_int);
    free((*list).set as *mut libc::c_void);
    free(list as *mut libc::c_void);
}
pub unsafe extern "C" fn undefine_variable_in_set(
    mut name: *const libc::c_char,
    mut length: size_t,
    mut origin: variable_origin,
    mut set: *mut variable_set,
) {
    let mut v: *mut variable = 0 as *mut variable;
    let mut var_slot: *mut *mut variable = 0 as *mut *mut variable;
    let mut var_key: variable = variable {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        value: 0 as *const libc::c_char as *mut libc::c_char,
        fileinfo: floc {
            filenm: 0 as *const libc::c_char,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    if set.is_null() {
        set = &mut global_variable_set;
    }
    var_key.name = name as *mut libc::c_char;
    var_key.length = length as libc::c_uint;
    var_slot = hash_find_slot(
        &mut (*set).table,
        &mut var_key as *mut variable as *const libc::c_void,
    ) as *mut *mut variable;
    if env_overrides != 0
        && origin as libc::c_uint == o_env as libc::c_int as libc::c_uint
    {
        origin = o_env_override;
    }
    v = *var_slot;
    if !(v.is_null() || v as *mut libc::c_void == hash_deleted_item) {
        if env_overrides != 0 && (*v).origin() as libc::c_int == o_env as libc::c_int {
            (*v).set_origin(o_env_override);
        }
        if origin as libc::c_int >= (*v).origin() as libc::c_int {
            hash_delete_at(&mut (*set).table, var_slot as *const libc::c_void);
            free_variable_name_and_value(v as *const libc::c_void);
            free(v as *mut libc::c_void);
            if set == &mut global_variable_set as *mut variable_set {
                variable_changenum = variable_changenum.wrapping_add(1);
                variable_changenum;
            }
        }
    }
}
unsafe extern "C" fn lookup_special_var(mut var: *mut variable) -> *mut variable {
    static mut last_changenum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if variable_changenum != last_changenum
        && ((*var).name
            == b".VARIABLES\0" as *const u8 as *const libc::c_char as *mut libc::c_char
            || *(*var).name as libc::c_int
                == *(b".VARIABLES\0" as *const u8 as *const libc::c_char) as libc::c_int
                && (*(*var).name as libc::c_int == '\0' as i32
                    || strcmp(
                        ((*var).name).offset(1 as libc::c_int as isize),
                        (b".VARIABLES\0" as *const u8 as *const libc::c_char)
                            .offset(1 as libc::c_int as isize),
                    ) == 0))
    {
        let mut max: size_t = (strlen((*var).value))
            .wrapping_div(500 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(500 as libc::c_int as libc::c_ulong);
        let mut len: size_t = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut vp: *mut *mut variable = global_variable_set.table.ht_vec
            as *mut *mut variable;
        let mut end: *mut *mut variable = &mut *vp
            .offset(global_variable_set.table.ht_size as isize) as *mut *mut variable;
        (*var)
            .value = xrealloc((*var).value as *mut libc::c_void, max)
            as *mut libc::c_char;
        p = (*var).value;
        len = 0 as libc::c_int as size_t;
        while vp < end {
            if !((*vp).is_null() || *vp as *mut libc::c_void == hash_deleted_item) {
                let mut v: *mut variable = *vp;
                let mut l: libc::c_int = (*v).length as libc::c_int;
                len = (len as libc::c_ulong)
                    .wrapping_add((l + 1 as libc::c_int) as libc::c_ulong) as size_t
                    as size_t;
                if len > max {
                    let mut off: size_t = p.offset_from((*var).value) as libc::c_long
                        as size_t;
                    max = (max as libc::c_ulong)
                        .wrapping_add(
                            (((l + 1 as libc::c_int) / 500 as libc::c_int
                                + 1 as libc::c_int) * 500 as libc::c_int) as libc::c_ulong,
                        ) as size_t as size_t;
                    (*var)
                        .value = xrealloc((*var).value as *mut libc::c_void, max)
                        as *mut libc::c_char;
                    p = &mut *((*var).value).offset(off as isize) as *mut libc::c_char;
                }
                p = mempcpy(
                    p as *mut libc::c_void,
                    (*v).name as *const libc::c_void,
                    l as libc::c_ulong,
                ) as *mut libc::c_char;
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = ' ' as i32 as libc::c_char;
            }
            vp = vp.offset(1);
            vp;
        }
        *p.offset(-(1 as libc::c_int as isize)) = '\0' as i32 as libc::c_char;
        last_changenum = variable_changenum;
    }
    return var;
}
pub unsafe extern "C" fn lookup_variable(
    mut name: *const libc::c_char,
    mut length: size_t,
) -> *mut variable {
    let mut setlist: *const variable_set_list = 0 as *const variable_set_list;
    let mut var_key: variable = variable {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        value: 0 as *const libc::c_char as *mut libc::c_char,
        fileinfo: floc {
            filenm: 0 as *const libc::c_char,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    let mut is_parent: libc::c_int = 0 as libc::c_int;
    var_key.name = name as *mut libc::c_char;
    var_key.length = length as libc::c_uint;
    setlist = current_variable_set_list;
    while !setlist.is_null() {
        let mut set: *const variable_set = (*setlist).set;
        let mut v: *mut variable = 0 as *mut variable;
        v = hash_find_item(
            &(*set).table as *const hash_table as *mut hash_table,
            &mut var_key as *mut variable as *const libc::c_void,
        ) as *mut variable;
        if !v.is_null() && (is_parent == 0 || (*v).private_var() == 0) {
            return if (*v).special() as libc::c_int != 0 {
                lookup_special_var(v)
            } else {
                v
            };
        }
        is_parent |= (*setlist).next_is_parent;
        setlist = (*setlist).next;
    }
    return 0 as *mut variable;
}
pub unsafe extern "C" fn lookup_variable_for_file(
    mut name: *const libc::c_char,
    mut length: size_t,
    mut file: *mut file,
) -> *mut variable {
    let mut var: *mut variable = 0 as *mut variable;
    let mut savev: *mut variable_set_list = 0 as *mut variable_set_list;
    if file.is_null() {
        return lookup_variable(name, length);
    }
    savev = current_variable_set_list;
    current_variable_set_list = (*file).variables;
    var = lookup_variable(name, length);
    current_variable_set_list = savev;
    return var;
}
pub unsafe extern "C" fn lookup_variable_in_set(
    mut name: *const libc::c_char,
    mut length: size_t,
    mut set: *const variable_set,
) -> *mut variable {
    let mut var_key: variable = variable {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        value: 0 as *const libc::c_char as *mut libc::c_char,
        fileinfo: floc {
            filenm: 0 as *const libc::c_char,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    var_key.name = name as *mut libc::c_char;
    var_key.length = length as libc::c_uint;
    return hash_find_item(
        &(*set).table as *const hash_table as *mut hash_table,
        &mut var_key as *mut variable as *const libc::c_void,
    ) as *mut variable;
}
pub unsafe extern "C" fn initialize_file_variables(
    mut file: *mut file,
    mut reading: libc::c_int,
) {
    let mut l: *mut variable_set_list = (*file).variables;
    if l.is_null() {
        l = xmalloc(::std::mem::size_of::<variable_set_list>() as libc::c_ulong)
            as *mut variable_set_list;
        (*l)
            .set = xmalloc(::std::mem::size_of::<variable_set>() as libc::c_ulong)
            as *mut variable_set;
        hash_init(
            &mut (*(*l).set).table,
            23 as libc::c_int as libc::c_ulong,
            Some(
                variable_hash_1
                    as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
            ),
            Some(
                variable_hash_2
                    as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
            ),
            Some(
                variable_hash_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        (*file).variables = l;
    }
    if !((*file).double_colon).is_null() && (*file).double_colon != file {
        initialize_file_variables((*file).double_colon, reading);
        (*l).next = (*(*file).double_colon).variables;
        (*l).next_is_parent = 0 as libc::c_int;
        return;
    }
    if ((*file).parent).is_null() {
        (*l).next = &mut global_setlist;
    } else {
        initialize_file_variables((*file).parent, reading);
        (*l).next = (*(*file).parent).variables;
    }
    (*l).next_is_parent = 1 as libc::c_int;
    if reading == 0 && (*file).pat_searched() == 0 {
        let mut p: *mut pattern_var = 0 as *mut pattern_var;
        let targlen: size_t = strlen((*file).name);
        p = lookup_pattern_var(0 as *mut pattern_var, (*file).name, targlen);
        if !p.is_null() {
            let mut global: *mut variable_set_list = current_variable_set_list;
            (*file).pat_variables = create_new_variable_set();
            current_variable_set_list = (*file).pat_variables;
            loop {
                let mut v: *mut variable = 0 as *mut variable;
                if ((*p).variable).flavor() as libc::c_int == f_simple as libc::c_int {
                    v = define_variable_in_set(
                        (*p).variable.name,
                        strlen((*p).variable.name),
                        (*p).variable.value,
                        ((*p).variable).origin(),
                        0 as libc::c_int,
                        (*current_variable_set_list).set,
                        &mut (*p).variable.fileinfo,
                    );
                    (*v).set_flavor(f_simple);
                } else {
                    v = do_variable_definition(
                        &mut (*p).variable.fileinfo,
                        (*p).variable.name,
                        (*p).variable.value,
                        ((*p).variable).origin(),
                        ((*p).variable).flavor(),
                        1 as libc::c_int,
                    );
                }
                (*v).set_per_target(((*p).variable).per_target());
                (*v).set_export(((*p).variable).export());
                (*v).set_private_var(((*p).variable).private_var());
                p = lookup_pattern_var(p, (*file).name, targlen);
                if p.is_null() {
                    break;
                }
            }
            current_variable_set_list = global;
        }
        (*file).set_pat_searched(1 as libc::c_int as libc::c_uint);
    }
    if !((*file).pat_variables).is_null() {
        (*(*file).pat_variables).next = (*l).next;
        (*(*file).pat_variables).next_is_parent = (*l).next_is_parent;
        (*l).next = (*file).pat_variables;
        (*l).next_is_parent = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn create_new_variable_set() -> *mut variable_set_list {
    let mut setlist: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut set: *mut variable_set = 0 as *mut variable_set;
    set = xmalloc(::std::mem::size_of::<variable_set>() as libc::c_ulong)
        as *mut variable_set;
    hash_init(
        &mut (*set).table,
        13 as libc::c_int as libc::c_ulong,
        Some(
            variable_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            variable_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            variable_hash_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    setlist = xmalloc(::std::mem::size_of::<variable_set_list>() as libc::c_ulong)
        as *mut variable_set_list;
    (*setlist).set = set;
    (*setlist).next = current_variable_set_list;
    (*setlist).next_is_parent = 0 as libc::c_int;
    return setlist;
}
pub unsafe extern "C" fn push_new_variable_scope() -> *mut variable_set_list {
    current_variable_set_list = create_new_variable_set();
    if (*current_variable_set_list).next == &mut global_setlist as *mut variable_set_list
    {
        let mut set: *mut variable_set = (*current_variable_set_list).set;
        (*current_variable_set_list).set = global_setlist.set;
        global_setlist.set = set;
        (*current_variable_set_list).next = global_setlist.next;
        global_setlist.next = current_variable_set_list;
        current_variable_set_list = &mut global_setlist;
    }
    return current_variable_set_list;
}
pub unsafe extern "C" fn pop_variable_scope() {
    let mut setlist: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut set: *mut variable_set = 0 as *mut variable_set;
    if current_variable_set_list != &mut global_setlist as *mut variable_set_list {
        setlist = current_variable_set_list;
        set = (*setlist).set;
        current_variable_set_list = (*setlist).next;
    } else {
        setlist = global_setlist.next;
        set = global_setlist.set;
        global_setlist.set = (*setlist).set;
        global_setlist.next = (*setlist).next;
        global_setlist.next_is_parent = (*setlist).next_is_parent;
    }
    free(setlist as *mut libc::c_void);
    hash_map(
        &mut (*set).table,
        Some(
            free_variable_name_and_value
                as unsafe extern "C" fn(*const libc::c_void) -> (),
        ),
    );
    hash_free(&mut (*set).table, 1 as libc::c_int);
    free(set as *mut libc::c_void);
}
unsafe extern "C" fn merge_variable_sets(
    mut to_set: *mut variable_set,
    mut from_set: *mut variable_set,
) {
    let mut from_var_slot: *mut *mut variable = (*from_set).table.ht_vec
        as *mut *mut variable;
    let mut from_var_end: *mut *mut variable = from_var_slot
        .offset((*from_set).table.ht_size as isize);
    let mut inc: libc::c_int = if to_set == &mut global_variable_set as *mut variable_set
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    while from_var_slot < from_var_end {
        if !((*from_var_slot).is_null()
            || *from_var_slot as *mut libc::c_void == hash_deleted_item)
        {
            let mut from_var: *mut variable = *from_var_slot;
            let mut to_var_slot: *mut *mut variable = hash_find_slot(
                &mut (*to_set).table,
                *from_var_slot as *const libc::c_void,
            ) as *mut *mut variable;
            if (*to_var_slot).is_null()
                || *to_var_slot as *mut libc::c_void == hash_deleted_item
            {
                hash_insert_at(
                    &mut (*to_set).table,
                    from_var as *const libc::c_void,
                    to_var_slot as *const libc::c_void,
                );
                variable_changenum = variable_changenum
                    .wrapping_add(inc as libc::c_ulong);
            } else {
                free((*from_var).value as *mut libc::c_void);
                free(from_var as *mut libc::c_void);
            }
        }
        from_var_slot = from_var_slot.offset(1);
        from_var_slot;
    }
}
pub unsafe extern "C" fn merge_variable_set_lists(
    mut setlist0: *mut *mut variable_set_list,
    mut setlist1: *mut variable_set_list,
) {
    let mut to: *mut variable_set_list = *setlist0;
    let mut last0: *mut variable_set_list = 0 as *mut variable_set_list;
    if setlist1.is_null() || setlist1 == &mut global_setlist as *mut variable_set_list {
        return;
    }
    if !to.is_null() {
        while to != &mut global_setlist as *mut variable_set_list {
            if to == setlist1 {
                return;
            }
            to = (*to).next;
        }
        to = *setlist0;
        while setlist1 != &mut global_setlist as *mut variable_set_list
            && to != &mut global_setlist as *mut variable_set_list
        {
            let mut from: *mut variable_set_list = setlist1;
            setlist1 = (*setlist1).next;
            merge_variable_sets((*to).set, (*from).set);
            last0 = to;
            to = (*to).next;
        }
    }
    if setlist1 != &mut global_setlist as *mut variable_set_list {
        if last0.is_null() {
            *setlist0 = setlist1;
        } else {
            (*last0).next = setlist1;
        }
    }
}
pub unsafe extern "C" fn define_automatic_variables() {
    let mut v: *mut variable = 0 as *mut variable;
    let mut buf: [libc::c_char; 200] = [0; 200];
    sprintf(buf.as_mut_ptr(), b"%u\0" as *const u8 as *const libc::c_char, makelevel);
    define_variable_in_set(
        b"MAKELEVEL\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        buf.as_mut_ptr(),
        o_env,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    sprintf(
        buf.as_mut_ptr(),
        b"%s%s%s\0" as *const u8 as *const libc::c_char,
        version_string,
        if remote_description.is_null()
            || *remote_description.offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
        {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"-\0" as *const u8 as *const libc::c_char
        },
        if remote_description.is_null()
            || *remote_description.offset(0 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
        {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            remote_description as *const libc::c_char
        },
    );
    define_variable_in_set(
        b"MAKE_VERSION\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        buf.as_mut_ptr(),
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"MAKE_HOST\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        make_host,
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    v = define_variable_in_set(
        b"SHELL\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        default_shell,
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    if *(*v).value as libc::c_int == '\0' as i32
        || (*v).origin() as libc::c_int == o_env as libc::c_int
        || (*v).origin() as libc::c_int == o_env_override as libc::c_int
    {
        free((*v).value as *mut libc::c_void);
        (*v).set_origin(o_file);
        (*v).value = xstrdup(default_shell);
    }
    v = define_variable_in_set(
        b"MAKEFILES\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"\0" as *const u8 as *const libc::c_char,
        o_default,
        0 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    (*v).set_export(v_ifset);
    define_variable_in_set(
        b"@D\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(patsubst %/,%,$(dir $@))\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"%D\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(patsubst %/,%,$(dir $%))\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"*D\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(patsubst %/,%,$(dir $*))\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"<D\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(patsubst %/,%,$(dir $<))\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"?D\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(patsubst %/,%,$(dir $?))\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"^D\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(patsubst %/,%,$(dir $^))\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"+D\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(patsubst %/,%,$(dir $+))\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"@F\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(notdir $@)\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"%F\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(notdir $%)\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"*F\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(notdir $*)\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"<F\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(notdir $<)\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"?F\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(notdir $?)\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"^F\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(notdir $^)\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"+F\0" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"$(notdir $+)\0" as *const u8 as *const libc::c_char,
        o_automatic,
        1 as libc::c_int,
        (*current_variable_set_list).set,
        0 as *mut floc,
    );
}
pub static mut export_all_variables: libc::c_int = 0;
unsafe extern "C" fn should_export(mut v: *const variable) -> libc::c_int {
    match (*v).export() as libc::c_int {
        2 => return 0 as libc::c_int,
        3 => {
            if (*v).origin() as libc::c_int == o_default as libc::c_int {
                return 0 as libc::c_int;
            }
        }
        0 => {
            if (*v).origin() as libc::c_int == o_default as libc::c_int
                || (*v).origin() as libc::c_int == o_automatic as libc::c_int
            {
                return 0 as libc::c_int;
            }
            if (*v).exportable() == 0 {
                return 0 as libc::c_int;
            }
            if export_all_variables == 0
                && (*v).origin() as libc::c_int != o_command as libc::c_int
                && (*v).origin() as libc::c_int != o_env as libc::c_int
                && (*v).origin() as libc::c_int != o_env_override as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
        1 | _ => {}
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn target_environment(
    mut file: *mut file,
    mut recursive: libc::c_int,
) -> *mut *mut libc::c_char {
    let mut set_list: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut s: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut table: hash_table = hash_table {
        ht_vec: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
        ht_hash_1: None,
        ht_hash_2: None,
        ht_compare: None,
        ht_size: 0,
        ht_capacity: 0,
        ht_fill: 0,
        ht_empty_slots: 0,
        ht_collisions: 0,
        ht_lookups: 0,
        ht_rehashes: 0,
    };
    let mut v_slot: *mut *mut variable = 0 as *mut *mut variable;
    let mut v_end: *mut *mut variable = 0 as *mut *mut variable;
    let mut result_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut result: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut invalid: *const libc::c_char = 0 as *const libc::c_char;
    let mut added_SHELL: libc::c_int = (shell_var.value == 0 as *mut libc::c_char)
        as libc::c_int;
    let mut found_makelevel: libc::c_int = 0 as libc::c_int;
    let mut found_mflags: libc::c_int = 0 as libc::c_int;
    let mut found_makeflags: libc::c_int = 0 as libc::c_int;
    if file.is_null() {
        env_recursion = env_recursion.wrapping_add(1);
        env_recursion;
    }
    if recursive == 0 && !jobserver_auth.is_null() {
        invalid = jobserver_get_invalid_auth();
    }
    if !file.is_null() {
        set_list = (*file).variables;
    } else {
        set_list = current_variable_set_list;
    }
    hash_init(
        &mut table,
        523 as libc::c_int as libc::c_ulong,
        Some(
            variable_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            variable_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
        ),
        Some(
            variable_hash_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    s = set_list;
    while !s.is_null() {
        let mut set: *mut variable_set = (*s).set;
        let islocal: libc::c_int = (s == set_list) as libc::c_int;
        let isglobal: libc::c_int = (set
            == &mut global_variable_set as *mut variable_set) as libc::c_int;
        v_slot = (*set).table.ht_vec as *mut *mut variable;
        v_end = v_slot.offset((*set).table.ht_size as isize);
        while v_slot < v_end {
            if !((*v_slot).is_null()
                || *v_slot as *mut libc::c_void == hash_deleted_item)
            {
                let mut evslot: *mut *mut variable = 0 as *mut *mut variable;
                let mut v: *mut variable = *v_slot;
                if !(islocal == 0 && (*v).private_var() as libc::c_int != 0) {
                    evslot = hash_find_slot(&mut table, v as *const libc::c_void)
                        as *mut *mut variable;
                    if (*evslot).is_null()
                        || *evslot as *mut libc::c_void == hash_deleted_item
                    {
                        if isglobal == 0 || should_export(v) != 0 {
                            hash_insert_at(
                                &mut table,
                                v as *const libc::c_void,
                                evslot as *const libc::c_void,
                            );
                        }
                    } else if (**evslot).export() as libc::c_int
                        == v_default as libc::c_int
                    {
                        (**evslot).set_export((*v).export());
                    }
                }
            }
            v_slot = v_slot.offset(1);
            v_slot;
        }
        s = (*s).next;
    }
    result_0 = xmalloc(
        (table.ht_fill)
            .wrapping_add(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    result = result_0;
    v_slot = table.ht_vec as *mut *mut variable;
    v_end = v_slot.offset(table.ht_size as isize);
    while v_slot < v_end {
        if !((*v_slot).is_null() || *v_slot as *mut libc::c_void == hash_deleted_item) {
            let mut v_0: *mut variable = *v_slot;
            let mut value: *mut libc::c_char = (*v_0).value;
            let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
            if !(should_export(v_0) == 0) {
                if (*v_0).recursive() as libc::c_int != 0
                    && ((*v_0).origin() as libc::c_int != o_env as libc::c_int
                        && (*v_0).origin() as libc::c_int
                            != o_env_override as libc::c_int
                        || ((*v_0).name
                            == b"MAKEFLAGS\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char
                            || *(*v_0).name as libc::c_int
                                == *(b"MAKEFLAGS\0" as *const u8 as *const libc::c_char)
                                    as libc::c_int
                                && (*(*v_0).name as libc::c_int == '\0' as i32
                                    || strcmp(
                                        ((*v_0).name).offset(1 as libc::c_int as isize),
                                        (b"MAKEFLAGS\0" as *const u8 as *const libc::c_char)
                                            .offset(1 as libc::c_int as isize),
                                    ) == 0)))
                {
                    cp = recursively_expand_for_file(v_0, file);
                    value = cp;
                }
                if added_SHELL == 0
                    && ((*v_0).name
                        == b"SHELL\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char
                        || *(*v_0).name as libc::c_int
                            == *(b"SHELL\0" as *const u8 as *const libc::c_char)
                                as libc::c_int
                            && (*(*v_0).name as libc::c_int == '\0' as i32
                                || strcmp(
                                    ((*v_0).name).offset(1 as libc::c_int as isize),
                                    (b"SHELL\0" as *const u8 as *const libc::c_char)
                                        .offset(1 as libc::c_int as isize),
                                ) == 0))
                {
                    added_SHELL = 1 as libc::c_int;
                } else if found_makelevel == 0
                    && ((*v_0).name
                        == b"MAKELEVEL\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char
                        || *(*v_0).name as libc::c_int
                            == *(b"MAKELEVEL\0" as *const u8 as *const libc::c_char)
                                as libc::c_int
                            && (*(*v_0).name as libc::c_int == '\0' as i32
                                || strcmp(
                                    ((*v_0).name).offset(1 as libc::c_int as isize),
                                    (b"MAKELEVEL\0" as *const u8 as *const libc::c_char)
                                        .offset(1 as libc::c_int as isize),
                                ) == 0))
                {
                    let mut val: [libc::c_char; 23] = [0; 23];
                    sprintf(
                        val.as_mut_ptr(),
                        b"%u\0" as *const u8 as *const libc::c_char,
                        makelevel.wrapping_add(1 as libc::c_int as libc::c_uint),
                    );
                    free(cp as *mut libc::c_void);
                    cp = xstrdup(val.as_mut_ptr());
                    value = cp;
                    found_makelevel = 1 as libc::c_int;
                } else if !invalid.is_null() {
                    if found_makeflags == 0
                        && ((*v_0).name
                            == b"MAKEFLAGS\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char
                            || *(*v_0).name as libc::c_int
                                == *(b"MAKEFLAGS\0" as *const u8 as *const libc::c_char)
                                    as libc::c_int
                                && (*(*v_0).name as libc::c_int == '\0' as i32
                                    || strcmp(
                                        ((*v_0).name).offset(1 as libc::c_int as isize),
                                        (b"MAKEFLAGS\0" as *const u8 as *const libc::c_char)
                                            .offset(1 as libc::c_int as isize),
                                    ) == 0))
                    {
                        let mut mf: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut vars: *mut libc::c_char = 0 as *mut libc::c_char;
                        found_makeflags = 1 as libc::c_int;
                        if !(strstr(
                            value,
                            b" --jobserver-auth=\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        {
                            vars = strstr(
                                value,
                                b" -- \0" as *const u8 as *const libc::c_char,
                            );
                            if vars.is_null() {
                                mf = xstrdup(
                                    concat(2 as libc::c_int as libc::c_uint, value, invalid),
                                );
                            } else {
                                let mut lf: size_t = vars.offset_from(value) as libc::c_long
                                    as size_t;
                                let mut li: size_t = strlen(invalid);
                                mf = xmalloc(
                                    (strlen(value))
                                        .wrapping_add(li)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) as *mut libc::c_char;
                                strcpy(
                                    mempcpy(
                                        mempcpy(
                                            mf as *mut libc::c_void,
                                            value as *const libc::c_void,
                                            lf,
                                        ),
                                        invalid as *const libc::c_void,
                                        li,
                                    ) as *mut libc::c_char,
                                    vars,
                                );
                            }
                            free(cp as *mut libc::c_void);
                            cp = mf;
                            value = cp;
                            if found_mflags != 0 {
                                invalid = 0 as *const libc::c_char;
                            }
                        }
                    } else if found_mflags == 0
                        && ((*v_0).name
                            == b"MFLAGS\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char
                            || *(*v_0).name as libc::c_int
                                == *(b"MFLAGS\0" as *const u8 as *const libc::c_char)
                                    as libc::c_int
                                && (*(*v_0).name as libc::c_int == '\0' as i32
                                    || strcmp(
                                        ((*v_0).name).offset(1 as libc::c_int as isize),
                                        (b"MFLAGS\0" as *const u8 as *const libc::c_char)
                                            .offset(1 as libc::c_int as isize),
                                    ) == 0))
                    {
                        let mut mf_0: *const libc::c_char = 0 as *const libc::c_char;
                        found_mflags = 1 as libc::c_int;
                        if !(strstr(
                            value,
                            b" --jobserver-auth=\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        {
                            if !((*v_0).origin() as libc::c_int != o_env as libc::c_int)
                            {
                                mf_0 = concat(
                                    2 as libc::c_int as libc::c_uint,
                                    value,
                                    invalid,
                                );
                                free(cp as *mut libc::c_void);
                                cp = xstrdup(mf_0);
                                value = cp;
                                if found_makeflags != 0 {
                                    invalid = 0 as *const libc::c_char;
                                }
                            }
                        }
                    }
                }
                let fresh1 = result;
                result = result.offset(1);
                *fresh1 = xstrdup(
                    concat(
                        3 as libc::c_int as libc::c_uint,
                        (*v_0).name,
                        b"=\0" as *const u8 as *const libc::c_char,
                        value,
                    ),
                );
                free(cp as *mut libc::c_void);
            }
        }
        v_slot = v_slot.offset(1);
        v_slot;
    }
    if added_SHELL == 0 {
        let fresh2 = result;
        result = result.offset(1);
        *fresh2 = xstrdup(
            concat(
                3 as libc::c_int as libc::c_uint,
                shell_var.name,
                b"=\0" as *const u8 as *const libc::c_char,
                shell_var.value,
            ),
        );
    }
    if found_makelevel == 0 {
        let mut val_0: [libc::c_char; 33] = [0; 33];
        sprintf(
            val_0.as_mut_ptr(),
            b"%s=%u\0" as *const u8 as *const libc::c_char,
            b"MAKELEVEL\0" as *const u8 as *const libc::c_char,
            makelevel.wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        let fresh3 = result;
        result = result.offset(1);
        *fresh3 = xstrdup(val_0.as_mut_ptr());
    }
    *result = 0 as *mut libc::c_char;
    hash_free(&mut table, 0 as libc::c_int);
    if file.is_null() {
        env_recursion = env_recursion.wrapping_sub(1);
        env_recursion;
    }
    return result_0;
}
unsafe extern "C" fn set_special_var(
    mut var: *mut variable,
    mut origin: variable_origin,
) -> *mut variable {
    if (*var).name
        == b"MAKEFLAGS\0" as *const u8 as *const libc::c_char as *mut libc::c_char
        || *(*var).name as libc::c_int
            == *(b"MAKEFLAGS\0" as *const u8 as *const libc::c_char) as libc::c_int
            && (*(*var).name as libc::c_int == '\0' as i32
                || strcmp(
                    ((*var).name).offset(1 as libc::c_int as isize),
                    (b"MAKEFLAGS\0" as *const u8 as *const libc::c_char)
                        .offset(1 as libc::c_int as isize),
                ) == 0)
    {
        reset_makeflags(origin);
    } else if (*var).name
        == b".RECIPEPREFIX\0" as *const u8 as *const libc::c_char as *mut libc::c_char
        || *(*var).name as libc::c_int
            == *(b".RECIPEPREFIX\0" as *const u8 as *const libc::c_char) as libc::c_int
            && (*(*var).name as libc::c_int == '\0' as i32
                || strcmp(
                    ((*var).name).offset(1 as libc::c_int as isize),
                    (b".RECIPEPREFIX\0" as *const u8 as *const libc::c_char)
                        .offset(1 as libc::c_int as isize),
                ) == 0)
    {
        cmd_prefix = (if *((*var).value).offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32
        {
            '\t' as i32
        } else {
            *((*var).value).offset(0 as libc::c_int as isize) as libc::c_int
        }) as libc::c_char;
    }
    return var;
}
unsafe extern "C" fn shell_result(mut p: *const libc::c_char) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut args: [*mut libc::c_char; 2] = [0 as *mut libc::c_char; 2];
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    install_variable_buffer(&mut buf, &mut len);
    args[0 as libc::c_int as usize] = p as *mut libc::c_char;
    args[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    variable_buffer_output(
        func_shell_base(variable_buffer, args.as_mut_ptr(), 0 as libc::c_int),
        b"\0\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    result = strdup(variable_buffer);
    restore_variable_buffer(buf, len);
    return result;
}
pub unsafe extern "C" fn do_variable_definition(
    mut flocp: *const floc,
    mut varname: *const libc::c_char,
    mut value: *const libc::c_char,
    mut origin: variable_origin,
    mut flavor: variable_flavor,
    mut target_var: libc::c_int,
) -> *mut variable {
    let mut current_block: u64;
    let mut newval: *const libc::c_char = 0 as *const libc::c_char;
    let mut alloc_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut variable = 0 as *mut variable;
    let mut append: libc::c_int = 0 as libc::c_int;
    let mut conditional: libc::c_int = 0 as libc::c_int;
    match flavor as libc::c_uint {
        1 => {
            alloc_value = allocated_variable_expand_for_file(value, 0 as *mut file);
            newval = alloc_value;
            current_block = 1423531122933789233;
        }
        3 => {
            let mut t: *mut libc::c_char = allocated_variable_expand_for_file(
                value,
                0 as *mut file,
            );
            alloc_value = xmalloc(
                (strlen(t))
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            let mut np: *mut libc::c_char = alloc_value;
            let mut op: *mut libc::c_char = t;
            while *op.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
                if *op.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32 {
                    let fresh4 = np;
                    np = np.offset(1);
                    *fresh4 = '$' as i32 as libc::c_char;
                }
                let fresh5 = op;
                op = op.offset(1);
                let fresh6 = np;
                np = np.offset(1);
                *fresh6 = *fresh5;
            }
            *np = '\0' as i32 as libc::c_char;
            free(t as *mut libc::c_void);
            newval = alloc_value;
            current_block = 1423531122933789233;
        }
        6 => {
            let mut q: *mut libc::c_char = allocated_variable_expand_for_file(
                value,
                0 as *mut file,
            );
            alloc_value = shell_result(q);
            free(q as *mut libc::c_void);
            flavor = f_recursive;
            newval = alloc_value;
            current_block = 1423531122933789233;
        }
        5 => {
            v = lookup_variable(varname, strlen(varname));
            if !v.is_null() {
                current_block = 1923444564214711538;
            } else {
                conditional = 1 as libc::c_int;
                flavor = f_recursive;
                current_block = 10017794567817669253;
            }
        }
        2 => {
            current_block = 10017794567817669253;
        }
        4 | 7 => {
            if target_var != 0 {
                append = 1 as libc::c_int;
                v = lookup_variable_in_set(
                    varname,
                    strlen(varname),
                    (*current_variable_set_list).set,
                );
                if !v.is_null() && (*v).append() == 0 {
                    append = 0 as libc::c_int;
                }
            } else {
                v = lookup_variable(varname, strlen(varname));
            }
            if v.is_null() {
                newval = value;
                flavor = f_recursive;
                current_block = 1423531122933789233;
            } else {
                let mut oldlen: size_t = 0;
                let mut vallen: size_t = 0;
                let mut alloclen: size_t = 0;
                let mut val: *const libc::c_char = 0 as *const libc::c_char;
                let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
                val = value;
                if (*v).recursive() != 0 {
                    flavor = f_recursive;
                } else if flavor as libc::c_uint
                    != f_append_value as libc::c_int as libc::c_uint
                {
                    tp = allocated_variable_expand_for_file(val, 0 as *mut file);
                    val = tp;
                }
                vallen = strlen(val);
                if vallen == 0 {
                    alloc_value = tp;
                    current_block = 1923444564214711538;
                } else {
                    oldlen = strlen((*v).value);
                    alloclen = oldlen
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(vallen)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    alloc_value = xmalloc(alloclen) as *mut libc::c_char;
                    cp = alloc_value;
                    if oldlen != 0 {
                        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
                        if (varname == b"MAKEFLAGS\0" as *const u8 as *const libc::c_char
                            || *varname as libc::c_int
                                == *(b"MAKEFLAGS\0" as *const u8 as *const libc::c_char)
                                    as libc::c_int
                                && (*varname as libc::c_int == '\0' as i32
                                    || strcmp(
                                        varname.offset(1 as libc::c_int as isize),
                                        (b"MAKEFLAGS\0" as *const u8 as *const libc::c_char)
                                            .offset(1 as libc::c_int as isize),
                                    ) == 0))
                            && {
                                s = strstr(
                                    (*v).value,
                                    b" -- \0" as *const u8 as *const libc::c_char,
                                );
                                !s.is_null()
                            }
                        {
                            cp = mempcpy(
                                cp as *mut libc::c_void,
                                (*v).value as *const libc::c_void,
                                s.offset_from((*v).value) as libc::c_long as libc::c_ulong,
                            ) as *mut libc::c_char;
                        } else {
                            cp = mempcpy(
                                cp as *mut libc::c_void,
                                (*v).value as *const libc::c_void,
                                oldlen,
                            ) as *mut libc::c_char;
                        }
                        let fresh7 = cp;
                        cp = cp.offset(1);
                        *fresh7 = ' ' as i32 as libc::c_char;
                    }
                    memcpy(
                        cp as *mut libc::c_void,
                        val as *const libc::c_void,
                        vallen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                    free(tp as *mut libc::c_void);
                    newval = alloc_value;
                    current_block = 1423531122933789233;
                }
            }
        }
        0 | _ => {
            abort();
        }
    }
    match current_block {
        10017794567817669253 => {
            newval = value;
            current_block = 1423531122933789233;
        }
        _ => {}
    }
    match current_block {
        1423531122933789233 => {
            v = define_variable_in_set(
                varname,
                strlen(varname),
                newval,
                origin,
                (flavor as libc::c_uint == f_recursive as libc::c_int as libc::c_uint
                    || flavor as libc::c_uint == f_expand as libc::c_int as libc::c_uint)
                    as libc::c_int,
                if target_var != 0 {
                    (*current_variable_set_list).set
                } else {
                    0 as *mut variable_set
                },
                flocp,
            );
            (*v).set_append(append as libc::c_uint);
            (*v).set_conditional(conditional as libc::c_uint);
        }
        _ => {}
    }
    free(alloc_value as *mut libc::c_void);
    return if (*v).special() as libc::c_int != 0 {
        set_special_var(v, origin)
    } else {
        v
    };
}
pub unsafe extern "C" fn parse_variable_definition(
    mut str: *const libc::c_char,
    mut var: *mut variable,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = str;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    while *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize) as libc::c_int
        & (0x2 as libc::c_int | 0x4 as libc::c_int) != 0 as libc::c_int
    {
        p = p.offset(1);
        p;
    }
    (*var).name = p as *mut libc::c_char;
    (*var).length = 0 as libc::c_int as libc::c_uint;
    let mut current_block_38: u64;
    loop {
        let fresh8 = p;
        p = p.offset(1);
        let mut c: libc::c_int = *fresh8 as libc::c_int;
        if *stopchar_map.as_mut_ptr().offset(c as libc::c_uchar as isize) as libc::c_int
            & (0x8 as libc::c_int | 0x1 as libc::c_int) != 0 as libc::c_int
        {
            return 0 as *mut libc::c_char;
        }
        if *stopchar_map.as_mut_ptr().offset(c as libc::c_uchar as isize) as libc::c_int
            & 0x2 as libc::c_int != 0 as libc::c_int
        {
            if !end.is_null() {
                return 0 as *mut libc::c_char;
            }
            end = p.offset(-(1 as libc::c_int as isize));
            while *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
                as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
                != 0 as libc::c_int
            {
                p = p.offset(1);
                p;
            }
        } else if c == '=' as i32 {
            if end.is_null() {
                end = p.offset(-(1 as libc::c_int as isize));
            }
            (*var).set_flavor(f_recursive);
            break;
        } else if c == ':' as i32 {
            if end.is_null() {
                end = p.offset(-(1 as libc::c_int as isize));
            }
            let fresh9 = p;
            p = p.offset(1);
            c = *fresh9 as libc::c_int;
            if c == '=' as i32 {
                (*var).set_flavor(f_simple);
                break;
            } else {
                if c == ':' as i32 {
                    let fresh10 = p;
                    p = p.offset(1);
                    c = *fresh10 as libc::c_int;
                    if c == '=' as i32 {
                        (*var).set_flavor(f_simple);
                        break;
                    } else if c == ':' as i32
                        && {
                            let fresh11 = p;
                            p = p.offset(1);
                            *fresh11 as libc::c_int == '=' as i32
                        }
                    {
                        (*var).set_flavor(f_expand);
                        break;
                    }
                }
                return 0 as *mut libc::c_char;
            }
        } else {
            if *p as libc::c_int == '=' as i32 {
                match c {
                    43 => {
                        current_block_38 = 156688694994682691;
                        match current_block_38 {
                            5344513092143988952 => {
                                (*var).set_flavor(f_shell);
                            }
                            5305977062421079247 => {
                                (*var).set_flavor(f_conditional);
                            }
                            _ => {
                                (*var).set_flavor(f_append);
                            }
                        }
                        if end.is_null() {
                            end = p.offset(-(1 as libc::c_int as isize));
                        }
                        p = p.offset(1);
                        p;
                        break;
                    }
                    63 => {
                        current_block_38 = 5305977062421079247;
                        match current_block_38 {
                            5344513092143988952 => {
                                (*var).set_flavor(f_shell);
                            }
                            5305977062421079247 => {
                                (*var).set_flavor(f_conditional);
                            }
                            _ => {
                                (*var).set_flavor(f_append);
                            }
                        }
                        if end.is_null() {
                            end = p.offset(-(1 as libc::c_int as isize));
                        }
                        p = p.offset(1);
                        p;
                        break;
                    }
                    33 => {
                        current_block_38 = 5344513092143988952;
                        match current_block_38 {
                            5344513092143988952 => {
                                (*var).set_flavor(f_shell);
                            }
                            5305977062421079247 => {
                                (*var).set_flavor(f_conditional);
                            }
                            _ => {
                                (*var).set_flavor(f_append);
                            }
                        }
                        if end.is_null() {
                            end = p.offset(-(1 as libc::c_int as isize));
                        }
                        p = p.offset(1);
                        p;
                        break;
                    }
                    _ => {}
                }
            }
            if !end.is_null() {
                return 0 as *mut libc::c_char;
            }
            if !(c == '$' as i32) {
                continue;
            }
            let mut closeparen: libc::c_char = 0;
            let mut count: libc::c_uint = 0;
            let fresh12 = p;
            p = p.offset(1);
            c = *fresh12 as libc::c_int;
            match c {
                40 => {
                    closeparen = ')' as i32 as libc::c_char;
                }
                123 => {
                    closeparen = '}' as i32 as libc::c_char;
                }
                0 => return 0 as *mut libc::c_char,
                _ => {
                    continue;
                }
            }
            count = 1 as libc::c_int as libc::c_uint;
            while *p as libc::c_int != '\0' as i32 {
                if *p as libc::c_int == closeparen as libc::c_int
                    && {
                        count = count.wrapping_sub(1);
                        count == 0 as libc::c_int as libc::c_uint
                    }
                {
                    p = p.offset(1);
                    p;
                    break;
                } else {
                    if *p as libc::c_int == c {
                        count = count.wrapping_add(1);
                        count;
                    }
                    p = p.offset(1);
                    p;
                }
            }
        }
    }
    (*var).length = end.offset_from((*var).name) as libc::c_long as libc::c_uint;
    (*var).value = next_token(p);
    return p as *mut libc::c_char;
}
pub unsafe extern "C" fn assign_variable_definition(
    mut v: *mut variable,
    mut line: *const libc::c_char,
) -> *mut variable {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if (parse_variable_definition(line, v)).is_null() {
        return 0 as *mut variable;
    }
    let mut fresh13 = ::std::vec::from_elem(
        0,
        ((*v).length).wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong
            as usize,
    );
    name = fresh13.leak().as_mut_ptr() as *mut libc::c_char;
    memcpy(
        name as *mut libc::c_void,
        (*v).name as *const libc::c_void,
        (*v).length as libc::c_ulong,
    );
    *name.offset((*v).length as isize) = '\0' as i32 as libc::c_char;
    (*v).name = allocated_variable_expand_for_file(name, 0 as *mut file);
    if *((*v).name).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        fatal(
            &mut (*v).fileinfo as *mut floc,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b"empty variable name\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    return v;
}
pub unsafe extern "C" fn try_variable_definition(
    mut flocp: *const floc,
    mut line: *const libc::c_char,
    mut origin: variable_origin,
    mut target_var: libc::c_int,
) -> *mut variable {
    let mut v: variable = variable {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        value: 0 as *const libc::c_char as *mut libc::c_char,
        fileinfo: floc {
            filenm: 0 as *const libc::c_char,
            lineno: 0,
            offset: 0,
        },
        length: 0,
        recursive_append_conditional_per_target_special_exportable_expanding_private_var_exp_count_flavor_origin_export: [0; 4],
    };
    let mut vp: *mut variable = 0 as *mut variable;
    if !flocp.is_null() {
        v.fileinfo = *flocp;
    } else {
        v.fileinfo.filenm = 0 as *const libc::c_char;
    }
    if (assign_variable_definition(&mut v, line)).is_null() {
        return 0 as *mut variable;
    }
    vp = do_variable_definition(flocp, v.name, v.value, origin, v.flavor(), target_var);
    free(v.name as *mut libc::c_void);
    return vp;
}
static mut defined_vars: [defined_vars; 11] = [defined_vars {
    name: 0 as *const libc::c_char,
    len: 0,
}; 11];
pub unsafe extern "C" fn warn_undefined(mut name: *const libc::c_char, mut len: size_t) {
    if warn_undefined_variables_flag != 0 {
        let mut dp: *const defined_vars = 0 as *const defined_vars;
        dp = defined_vars.as_ptr();
        while !((*dp).name).is_null() {
            if (*dp).len == len
                && memcmp(
                    (*dp).name as *const libc::c_void,
                    name as *const libc::c_void,
                    len,
                ) == 0 as libc::c_int
            {
                return;
            }
            dp = dp.offset(1);
            dp;
        }
        error(
            reading_file,
            len,
            dcgettext(
                0 as *const libc::c_char,
                b"warning: undefined variable '%.*s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            len as libc::c_int,
            name,
        );
    }
}
unsafe extern "C" fn print_variable(
    mut item: *const libc::c_void,
    mut arg: *mut libc::c_void,
) {
    let mut v: *const variable = item as *const variable;
    let mut prefix: *const libc::c_char = arg as *const libc::c_char;
    let mut origin: *const libc::c_char = 0 as *const libc::c_char;
    match (*v).origin() as libc::c_int {
        6 => {
            origin = dcgettext(
                0 as *const libc::c_char,
                b"automatic\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        0 => {
            origin = dcgettext(
                0 as *const libc::c_char,
                b"default\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        1 => {
            origin = dcgettext(
                0 as *const libc::c_char,
                b"environment\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        2 => {
            origin = dcgettext(
                0 as *const libc::c_char,
                b"makefile\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        3 => {
            origin = dcgettext(
                0 as *const libc::c_char,
                b"environment under -e\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        4 => {
            origin = dcgettext(
                0 as *const libc::c_char,
                b"command line\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        5 => {
            origin = dcgettext(
                0 as *const libc::c_char,
                b"'override' directive\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        7 => {
            abort();
        }
        _ => {}
    }
    fputs(b"# \0" as *const u8 as *const libc::c_char, stdout);
    fputs(origin, stdout);
    if (*v).private_var() != 0 {
        fputs(b" private\0" as *const u8 as *const libc::c_char, stdout);
    }
    if !((*v).fileinfo.filenm).is_null() {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b" (from '%s', line %lu)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*v).fileinfo.filenm,
            ((*v).fileinfo.lineno).wrapping_add((*v).fileinfo.offset),
        );
    }
    putchar('\n' as i32);
    fputs(prefix, stdout);
    if (*v).recursive() as libc::c_int != 0
        && !(strchr((*v).value, '\n' as i32)).is_null()
    {
        printf(
            b"define %s\n%s\nendef\n\0" as *const u8 as *const libc::c_char,
            (*v).name,
            (*v).value,
        );
    } else {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        printf(
            b"%s %s= \0" as *const u8 as *const libc::c_char,
            (*v).name,
            if (*v).recursive() as libc::c_int != 0 {
                if (*v).append() as libc::c_int != 0 {
                    b"+\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                }
            } else {
                b":\0" as *const u8 as *const libc::c_char
            },
        );
        p = next_token((*v).value);
        if p != (*v).value && *p as libc::c_int == '\0' as i32 {
            printf(b"$(subst ,,%s)\0" as *const u8 as *const libc::c_char, (*v).value);
        } else if (*v).recursive() != 0 {
            fputs((*v).value, stdout);
        } else {
            p = (*v).value;
            while *p as libc::c_int != '\0' as i32 {
                if *p as libc::c_int == '$' as i32 {
                    putchar('$' as i32);
                }
                putchar(*p as libc::c_int);
                p = p.offset(1);
                p;
            }
        }
        putchar('\n' as i32);
    };
}
unsafe extern "C" fn print_auto_variable(
    mut item: *const libc::c_void,
    mut arg: *mut libc::c_void,
) {
    let mut v: *const variable = item as *const variable;
    if (*v).origin() as libc::c_int == o_automatic as libc::c_int {
        print_variable(item, arg);
    }
}
unsafe extern "C" fn print_noauto_variable(
    mut item: *const libc::c_void,
    mut arg: *mut libc::c_void,
) {
    let mut v: *const variable = item as *const variable;
    if (*v).origin() as libc::c_int != o_automatic as libc::c_int {
        print_variable(item, arg);
    }
}
unsafe extern "C" fn print_variable_set(
    mut set: *mut variable_set,
    mut prefix: *const libc::c_char,
    mut pauto: libc::c_int,
) {
    hash_map_arg(
        &mut (*set).table,
        if pauto != 0 {
            Some(
                print_auto_variable
                    as unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
            )
        } else {
            Some(
                print_variable
                    as unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
            )
        },
        prefix as *mut libc::c_void,
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"# variable set hash-table stats:\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    fputs(b"# \0" as *const u8 as *const libc::c_char, stdout);
    hash_print_stats(&mut (*set).table, stdout);
    putc('\n' as i32, stdout);
}
pub unsafe extern "C" fn print_variable_data_base() {
    puts(
        dcgettext(
            0 as *const libc::c_char,
            b"\n# Variables\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    print_variable_set(
        &mut global_variable_set,
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    puts(
        dcgettext(
            0 as *const libc::c_char,
            b"\n# Pattern-specific Variable Values\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    let mut p: *mut pattern_var = 0 as *mut pattern_var;
    let mut rules: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    p = pattern_vars;
    while !p.is_null() {
        rules = rules.wrapping_add(1);
        rules;
        printf(b"\n%s :\n\0" as *const u8 as *const libc::c_char, (*p).target);
        print_variable(
            &mut (*p).variable as *mut variable as *const libc::c_void,
            b"# \0" as *const u8 as *const libc::c_char as *mut libc::c_void,
        );
        p = (*p).next;
    }
    if rules == 0 as libc::c_int as libc::c_uint {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"\n# No pattern-specific variable values.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\n# %u pattern-specific variable values\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            rules,
        );
    };
}
pub unsafe extern "C" fn print_file_variables(mut file: *const file) {
    if !((*file).variables).is_null() {
        print_variable_set(
            (*(*file).variables).set,
            b"# \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn print_target_variables(mut file: *const file) {
    if !((*file).variables).is_null() {
        let mut l: size_t = strlen((*file).name);
        let mut fresh14 = ::std::vec::from_elem(
            0,
            l.wrapping_add(3 as libc::c_int as libc::c_ulong) as usize,
        );
        let mut t: *mut libc::c_char = fresh14.leak().as_mut_ptr() as *mut libc::c_char;
        memcpy(t as *mut libc::c_void, (*file).name as *const libc::c_void, l);
        *t.offset(l as isize) = ':' as i32 as libc::c_char;
        *t
            .offset(
                l.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = ' ' as i32 as libc::c_char;
        *t
            .offset(
                l.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        hash_map_arg(
            &mut (*(*(*file).variables).set).table,
            Some(
                print_noauto_variable
                    as unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
            ),
            t as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn run_static_initializers() {
    defined_vars = [
        {
            let mut init = defined_vars {
                name: b"MAKECMDGOALS\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"MAKE_RESTARTS\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"MAKE_TERMOUT\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"MAKE_TERMERR\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"MAKEOVERRIDES\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b".DEFAULT\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"-*-command-variables-*-\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"-*-eval-flags-*-\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"VPATH\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: b"GPATH\0" as *const u8 as *const libc::c_char,
                len: (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            };
            init
        },
        {
            let mut init = defined_vars {
                name: 0 as *const libc::c_char,
                len: 0 as libc::c_int as size_t,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
