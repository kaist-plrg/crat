use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn perror_with_name(_: *const libc::c_char, _: *const libc::c_char);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn end_of_token(_: *const libc::c_char) -> *mut libc::c_char;
    fn find_percent(_: *mut libc::c_char) -> *mut libc::c_char;
    fn strcache_iscached(str: *const libc::c_char) -> libc::c_int;
    fn strcache_add_len(str: *const libc::c_char, len: size_t) -> *const libc::c_char;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut just_print_flag: libc::c_int;
    static mut run_silent: libc::c_int;
    static mut ignore_errors_flag: libc::c_int;
    static mut question_flag: libc::c_int;
    static mut touch_flag: libc::c_int;
    static mut no_builtin_rules_flag: libc::c_int;
    static mut not_parallel: libc::c_int;
    static mut second_expansion: libc::c_int;
    static mut cmd_prefix: libc::c_char;
    static mut no_intermediates: libc::c_uint;
    fn hash_delete(ht: *mut hash_table, item: *const libc::c_void) -> *mut libc::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const libc::c_void,
        slot: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut *mut libc::c_void;
    fn hash_init(
        ht: *mut hash_table,
        size: libc::c_ulong,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_map(ht: *mut hash_table, map: hash_map_func_t);
    fn hash_map_arg(
        ht: *mut hash_table,
        map: hash_map_arg_func_t,
        arg: *mut libc::c_void,
    );
    fn hash_print_stats(ht: *mut hash_table, out_FILE: *mut FILE);
    fn jhash_string(key: *const libc::c_uchar) -> libc::c_uint;
    static mut hash_deleted_item: *mut libc::c_void;
    fn parse_file_seq(
        stringp: *mut *mut libc::c_char,
        size: size_t,
        stopmap: libc::c_int,
        prefix: *const libc::c_char,
        flags: libc::c_int,
    ) -> *mut libc::c_void;
    fn free_ns_chain(n: *mut nameseq);
    fn copy_dep_chain(d: *const dep) -> *mut dep;
    fn print_commands(cmds: *const commands);
    fn set_file_variables(file: *mut file, stem: *const libc::c_char);
    static mut variable_buffer: *mut libc::c_char;
    fn variable_buffer_output(
        ptr: *mut libc::c_char,
        string: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn variable_expand(line: *const libc::c_char) -> *mut libc::c_char;
    fn variable_expand_for_file(
        line: *const libc::c_char,
        file: *mut file,
    ) -> *mut libc::c_char;
    fn patsubst_expand_pat(
        o: *mut libc::c_char,
        text: *const libc::c_char,
        pattern: *const libc::c_char,
        replace: *const libc::c_char,
        pattern_percent: *const libc::c_char,
        replace_percent: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn initialize_file_variables(file: *mut file, reading: libc::c_int);
    fn print_file_variables(file: *const file);
    fn print_target_variables(file: *const file);
    fn merge_variable_set_lists(
        to_list: *mut *mut variable_set_list,
        from_list: *mut variable_set_list,
    );
    fn lookup_variable(name: *const libc::c_char, length: size_t) -> *mut variable;
    fn lookup_variable_in_set(
        name: *const libc::c_char,
        length: size_t,
        set: *const variable_set,
    ) -> *mut variable;
    static mut export_all_variables: libc::c_int;
    static mut db_level: libc::c_int;
    fn shuffle_deps_recursive(g: *mut dep);
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type intmax_t = __intmax_t;
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
pub type hash_map_func_t = Option::<unsafe extern "C" fn(*const libc::c_void) -> ()>;
pub type hash_map_arg_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub static mut snapped_deps: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn file_hash_1(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut _key_: *const libc::c_uchar = (*(key as *const file)).hname
        as *const libc::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as libc::c_ulong);
    return _result_;
}
unsafe extern "C" fn file_hash_2(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    return _result_;
}
unsafe extern "C" fn file_hash_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> libc::c_int {
    return if (*(x as *const file)).hname == (*(y as *const file)).hname {
        0 as libc::c_int
    } else {
        strcmp((*(x as *const file)).hname, (*(y as *const file)).hname)
    };
}
static mut files: hash_table = hash_table {
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
static mut all_secondary: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn lookup_file(mut name: *const libc::c_char) -> *mut file {
    let mut f: *mut file = 0 as *mut file;
    let mut file_key: file = file {
        name: 0 as *const libc::c_char,
        hname: 0 as *const libc::c_char,
        vpath: 0 as *const libc::c_char,
        deps: 0 as *mut dep,
        cmds: 0 as *mut commands,
        stem: 0 as *const libc::c_char,
        also_make: 0 as *mut dep,
        prev: 0 as *mut file,
        last: 0 as *mut file,
        renamed: 0 as *mut file,
        variables: 0 as *mut variable_set_list,
        pat_variables: 0 as *mut variable_set_list,
        parent: 0 as *mut file,
        double_colon: 0 as *mut file,
        last_mtime: 0,
        mtime_before_update: 0,
        considered: 0,
        command_flags: 0,
        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [0; 4],
        c2rust_padding: [0; 4],
    };
    while *name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *stopchar_map
            .as_mut_ptr()
            .offset(*name.offset(1 as libc::c_int as isize) as libc::c_uchar as isize)
            as libc::c_int & 0x8000 as libc::c_int != 0 as libc::c_int
        && *name.offset(2 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        name = name.offset(2 as libc::c_int as isize);
        while *stopchar_map.as_mut_ptr().offset(*name as libc::c_uchar as isize)
            as libc::c_int & 0x8000 as libc::c_int != 0 as libc::c_int
        {
            name = name.offset(1);
            name;
        }
    }
    if *name as libc::c_int == '\0' as i32 {
        name = b"./\0" as *const u8 as *const libc::c_char;
    }
    file_key.hname = name;
    f = hash_find_item(&mut files, &mut file_key as *mut file as *const libc::c_void)
        as *mut file;
    return f;
}
pub unsafe extern "C" fn enter_file(mut name: *const libc::c_char) -> *mut file {
    let mut f: *mut file = 0 as *mut file;
    let mut new: *mut file = 0 as *mut file;
    let mut file_slot: *mut *mut file = 0 as *mut *mut file;
    let mut file_key: file = file {
        name: 0 as *const libc::c_char,
        hname: 0 as *const libc::c_char,
        vpath: 0 as *const libc::c_char,
        deps: 0 as *mut dep,
        cmds: 0 as *mut commands,
        stem: 0 as *const libc::c_char,
        also_make: 0 as *mut dep,
        prev: 0 as *mut file,
        last: 0 as *mut file,
        renamed: 0 as *mut file,
        variables: 0 as *mut variable_set_list,
        pat_variables: 0 as *mut variable_set_list,
        parent: 0 as *mut file,
        double_colon: 0 as *mut file,
        last_mtime: 0,
        mtime_before_update: 0,
        considered: 0,
        command_flags: 0,
        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [0; 4],
        c2rust_padding: [0; 4],
    };
    file_key.hname = name;
    file_slot = hash_find_slot(
        &mut files,
        &mut file_key as *mut file as *const libc::c_void,
    ) as *mut *mut file;
    f = *file_slot;
    if !(f.is_null() || f as *mut libc::c_void == hash_deleted_item)
        && ((*f).double_colon).is_null()
    {
        (*f).set_builtin(0 as libc::c_int as libc::c_uint);
        return f;
    }
    new = xcalloc(::std::mem::size_of::<file>() as libc::c_ulong) as *mut file;
    (*new).hname = name;
    (*new).name = (*new).hname;
    (*new).set_update_status(us_none);
    if f.is_null() || f as *mut libc::c_void == hash_deleted_item {
        (*new).last = new;
        hash_insert_at(
            &mut files,
            new as *const libc::c_void,
            file_slot as *const libc::c_void,
        );
    } else {
        (*new).double_colon = f;
        (*(*f).last).prev = new;
        (*f).last = new;
    }
    return new;
}
pub unsafe extern "C" fn rehash_file(
    mut from_file: *mut file,
    mut to_hname: *const libc::c_char,
) {
    let mut file_key: file = file {
        name: 0 as *const libc::c_char,
        hname: 0 as *const libc::c_char,
        vpath: 0 as *const libc::c_char,
        deps: 0 as *mut dep,
        cmds: 0 as *mut commands,
        stem: 0 as *const libc::c_char,
        also_make: 0 as *mut dep,
        prev: 0 as *mut file,
        last: 0 as *mut file,
        renamed: 0 as *mut file,
        variables: 0 as *mut variable_set_list,
        pat_variables: 0 as *mut variable_set_list,
        parent: 0 as *mut file,
        double_colon: 0 as *mut file,
        last_mtime: 0,
        mtime_before_update: 0,
        considered: 0,
        command_flags: 0,
        update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [0; 4],
        c2rust_padding: [0; 4],
    };
    let mut file_slot: *mut *mut file = 0 as *mut *mut file;
    let mut to_file: *mut file = 0 as *mut file;
    let mut deleted_file: *mut file = 0 as *mut file;
    let mut f: *mut file = 0 as *mut file;
    (*from_file).set_builtin(0 as libc::c_int as libc::c_uint);
    file_key.hname = to_hname;
    if file_hash_cmp(
        from_file as *const libc::c_void,
        &mut file_key as *mut file as *const libc::c_void,
    ) == 0
    {
        return;
    }
    file_key.hname = (*from_file).hname;
    while !((*from_file).renamed).is_null() {
        from_file = (*from_file).renamed;
    }
    if file_hash_cmp(
        from_file as *const libc::c_void,
        &mut file_key as *mut file as *const libc::c_void,
    ) != 0
    {
        abort();
    }
    deleted_file = hash_delete(&mut files, from_file as *const libc::c_void)
        as *mut file;
    if deleted_file != from_file {
        abort();
    }
    file_key.hname = to_hname;
    file_slot = hash_find_slot(
        &mut files,
        &mut file_key as *mut file as *const libc::c_void,
    ) as *mut *mut file;
    to_file = *file_slot;
    (*from_file).hname = to_hname;
    f = (*from_file).double_colon;
    while !f.is_null() {
        (*f).hname = to_hname;
        f = (*f).prev;
    }
    if to_file.is_null() || to_file as *mut libc::c_void == hash_deleted_item {
        hash_insert_at(
            &mut files,
            from_file as *const libc::c_void,
            file_slot as *const libc::c_void,
        );
        return;
    }
    if !((*from_file).cmds).is_null() {
        if ((*to_file).cmds).is_null() {
            (*to_file).cmds = (*from_file).cmds;
        } else if (*from_file).cmds != (*to_file).cmds {
            let mut l: size_t = strlen((*from_file).name);
            if !((*(*to_file).cmds).fileinfo.filenm).is_null() {
                error(
                    &mut (*(*from_file).cmds).fileinfo as *mut floc,
                    l
                        .wrapping_add(strlen((*(*to_file).cmds).fileinfo.filenm))
                        .wrapping_add(
                            (53 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                                )
                                .wrapping_div(22 as libc::c_int as libc::c_ulong)
                                .wrapping_add(3 as libc::c_int as libc::c_ulong),
                        ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Recipe was specified for file '%s' at %s:%lu,\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*from_file).name,
                    (*(*from_file).cmds).fileinfo.filenm,
                    (*(*from_file).cmds).fileinfo.lineno,
                );
            } else {
                error(
                    &mut (*(*from_file).cmds).fileinfo as *mut floc,
                    l,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Recipe for file '%s' was found by implicit rule search,\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*from_file).name,
                );
            }
            l = (l as libc::c_ulong).wrapping_add(strlen(to_hname)) as size_t as size_t;
            error(
                &mut (*(*from_file).cmds).fileinfo as *mut floc,
                l,
                dcgettext(
                    0 as *const libc::c_char,
                    b"but '%s' is now considered the same file as '%s'.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*from_file).name,
                to_hname,
            );
            error(
                &mut (*(*from_file).cmds).fileinfo as *mut floc,
                l,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Recipe for '%s' will be ignored in favor of the one for '%s'.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*from_file).name,
                to_hname,
            );
        }
    }
    if ((*to_file).deps).is_null() {
        (*to_file).deps = (*from_file).deps;
    } else {
        let mut deps: *mut dep = (*to_file).deps;
        while !((*deps).next).is_null() {
            deps = (*deps).next;
        }
        (*deps).next = (*from_file).deps;
    }
    merge_variable_set_lists(&mut (*to_file).variables, (*from_file).variables);
    if !((*to_file).double_colon).is_null()
        && (*from_file).is_target() as libc::c_int != 0
        && ((*from_file).double_colon).is_null()
    {
        fatal(
            0 as *mut floc,
            (strlen((*from_file).name)).wrapping_add(strlen(to_hname)),
            dcgettext(
                0 as *const libc::c_char,
                b"can't rename single-colon '%s' to double-colon '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*from_file).name,
            to_hname,
        );
    }
    if ((*to_file).double_colon).is_null() && !((*from_file).double_colon).is_null() {
        if (*to_file).is_target() != 0 {
            fatal(
                0 as *mut floc,
                (strlen((*from_file).name)).wrapping_add(strlen(to_hname)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"can't rename double-colon '%s' to single-colon '%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*from_file).name,
                to_hname,
            );
        } else {
            (*to_file).double_colon = (*from_file).double_colon;
        }
    }
    if (*from_file).last_mtime > (*to_file).last_mtime {
        (*to_file).last_mtime = (*from_file).last_mtime;
    }
    (*to_file).mtime_before_update = (*from_file).mtime_before_update;
    (*to_file)
        .set_precious(
            (*to_file).precious()
                | (*from_file).precious() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_loaded(
            (*to_file).loaded() | (*from_file).loaded() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_tried_implicit(
            (*to_file).tried_implicit()
                | (*from_file).tried_implicit() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_updating(
            (*to_file).updating()
                | (*from_file).updating() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_updated(
            (*to_file).updated() | (*from_file).updated() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_is_target(
            (*to_file).is_target()
                | (*from_file).is_target() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_cmd_target(
            (*to_file).cmd_target()
                | (*from_file).cmd_target() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_phony(
            (*to_file).phony() | (*from_file).phony() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_is_explicit(
            (*to_file).is_explicit()
                | (*from_file).is_explicit() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_secondary(
            (*to_file).secondary()
                | (*from_file).secondary() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_notintermediate(
            (*to_file).notintermediate()
                | (*from_file).notintermediate() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_ignore_vpath(
            (*to_file).ignore_vpath()
                | (*from_file).ignore_vpath() as libc::c_int as libc::c_uint,
        );
    (*to_file)
        .set_snapped(
            (*to_file).snapped() | (*from_file).snapped() as libc::c_int as libc::c_uint,
        );
    (*to_file).set_builtin(0 as libc::c_int as libc::c_uint);
    (*from_file).renamed = to_file;
}
pub unsafe extern "C" fn rename_file(
    mut from_file: *mut file,
    mut to_hname: *const libc::c_char,
) {
    rehash_file(from_file, to_hname);
    while !from_file.is_null() {
        (*from_file).name = (*from_file).hname;
        from_file = (*from_file).prev;
    }
}
pub unsafe extern "C" fn remove_intermediates(mut sig: libc::c_int) {
    let mut file_slot: *mut *mut file = 0 as *mut *mut file;
    let mut file_end: *mut *mut file = 0 as *mut *mut file;
    let mut doneany: libc::c_int = 0 as libc::c_int;
    if question_flag != 0 || touch_flag != 0 || all_secondary != 0
        || no_intermediates != 0
    {
        return;
    }
    if sig != 0 && just_print_flag != 0 {
        return;
    }
    file_slot = files.ht_vec as *mut *mut file;
    file_end = file_slot.offset(files.ht_size as isize);
    let mut current_block_32: u64;
    while file_slot < file_end {
        if !((*file_slot).is_null()
            || *file_slot as *mut libc::c_void == hash_deleted_item)
        {
            let mut f: *mut file = *file_slot;
            if (*f).intermediate() as libc::c_int != 0
                && ((*f).dontcare() as libc::c_int != 0 || (*f).precious() == 0)
                && (*f).secondary() == 0 && (*f).notintermediate() == 0
                && (*f).cmd_target() == 0
            {
                let mut status: libc::c_int = 0;
                if !((*f).update_status() as libc::c_int == us_none as libc::c_int) {
                    if just_print_flag != 0 {
                        status = 0 as libc::c_int;
                        current_block_32 = 2979737022853876585;
                    } else {
                        status = unlink((*f).name);
                        if status < 0 as libc::c_int
                            && *__errno_location() == 2 as libc::c_int
                        {
                            current_block_32 = 6873731126896040597;
                        } else {
                            current_block_32 = 2979737022853876585;
                        }
                    }
                    match current_block_32 {
                        6873731126896040597 => {}
                        _ => {
                            if (*f).dontcare() == 0 {
                                if sig != 0 {
                                    error(
                                        0 as *mut floc,
                                        strlen((*f).name),
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"*** Deleting intermediate file '%s'\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        (*f).name,
                                    );
                                } else {
                                    if doneany == 0 {
                                        if 0x1 as libc::c_int & db_level != 0 {
                                            printf(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Removing intermediate files...\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            fflush(stdout);
                                        }
                                    }
                                    if run_silent == 0 {
                                        if doneany == 0 {
                                            fputs(b"rm \0" as *const u8 as *const libc::c_char, stdout);
                                            doneany = 1 as libc::c_int;
                                        } else {
                                            putchar(' ' as i32);
                                        }
                                        fputs((*f).name, stdout);
                                        fflush(stdout);
                                    }
                                }
                                if status < 0 as libc::c_int {
                                    perror_with_name(
                                        b"\nunlink: \0" as *const u8 as *const libc::c_char,
                                        (*f).name,
                                    );
                                    doneany = 0 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
        file_slot = file_slot.offset(1);
        file_slot;
    }
    if doneany != 0 && sig == 0 {
        putchar('\n' as i32);
        fflush(stdout);
    }
}
pub unsafe extern "C" fn split_prereqs(mut p: *mut libc::c_char) -> *mut dep {
    let mut new: *mut dep = parse_file_seq(
        &mut p,
        ::std::mem::size_of::<dep>() as libc::c_ulong,
        0x100 as libc::c_int,
        0 as *const libc::c_char,
        0x40 as libc::c_int,
    ) as *mut dep;
    if *p != 0 {
        let mut ood: *mut dep = 0 as *mut dep;
        p = p.offset(1);
        p;
        ood = parse_file_seq(
            &mut p,
            ::std::mem::size_of::<dep>() as libc::c_ulong,
            0x1 as libc::c_int,
            0 as *const libc::c_char,
            0x40 as libc::c_int,
        ) as *mut dep;
        if new.is_null() {
            new = ood;
        } else {
            let mut dp: *mut dep = 0 as *mut dep;
            dp = new;
            while !((*dp).next).is_null() {
                dp = (*dp).next;
            }
            (*dp).next = ood;
        }
        while !ood.is_null() {
            (*ood).set_ignore_mtime(1 as libc::c_int as libc::c_uint);
            ood = (*ood).next;
        }
    }
    return new;
}
pub unsafe extern "C" fn enter_prereqs(
    mut deps: *mut dep,
    mut stem: *const libc::c_char,
) -> *mut dep {
    let mut d1: *mut dep = 0 as *mut dep;
    if deps.is_null() {
        return 0 as *mut dep;
    }
    if !stem.is_null() {
        let mut pattern: *const libc::c_char = b"%\0" as *const u8
            as *const libc::c_char;
        let mut dp: *mut dep = deps;
        let mut dl: *mut dep = 0 as *mut dep;
        while !dp.is_null() {
            let mut percent: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut nl: size_t = (strlen((*dp).name))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            let mut fresh0 = ::std::vec::from_elem(0, nl as usize);
            let mut nm: *mut libc::c_char = fresh0.leak().as_mut_ptr()
                as *mut libc::c_char;
            memcpy(nm as *mut libc::c_void, (*dp).name as *const libc::c_void, nl);
            percent = find_percent(nm);
            if !percent.is_null() {
                let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
                if *stem.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
                {
                    memmove(
                        percent as *mut libc::c_void,
                        percent.offset(1 as libc::c_int as isize) as *const libc::c_void,
                        strlen(percent),
                    );
                    o = variable_buffer_output(
                        variable_buffer,
                        nm,
                        (strlen(nm)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                } else {
                    o = patsubst_expand_pat(
                        variable_buffer,
                        stem,
                        pattern,
                        nm,
                        pattern.offset(1 as libc::c_int as isize),
                        percent.offset(1 as libc::c_int as isize),
                    );
                }
                if *variable_buffer.offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
                {
                    let mut df: *mut dep = dp;
                    if dp == deps {
                        deps = (*deps).next;
                        dp = deps;
                    } else {
                        (*dl).next = (*dp).next;
                        dp = (*dl).next;
                    }
                    free(df as *mut libc::c_void);
                    continue;
                } else {
                    (*dp)
                        .name = strcache_add_len(
                        variable_buffer,
                        o.offset_from(variable_buffer) as libc::c_long as size_t,
                    );
                }
            }
            (*dp).stem = stem;
            (*dp).set_staticpattern(1 as libc::c_int as libc::c_uint);
            dl = dp;
            dp = (*dp).next;
        }
    }
    d1 = deps;
    while !d1.is_null() {
        if !((*d1).need_2nd_expansion() != 0) {
            (*d1).file = lookup_file((*d1).name);
            if ((*d1).file).is_null() {
                (*d1).file = enter_file((*d1).name);
            }
            (*d1).set_staticpattern(0 as libc::c_int as libc::c_uint);
            (*d1).name = 0 as *const libc::c_char;
            if stem.is_null() {
                (*(*d1).file).set_is_explicit(1 as libc::c_int as libc::c_uint);
            }
        }
        d1 = (*d1).next;
    }
    return deps;
}
pub unsafe extern "C" fn expand_deps(mut f: *mut file) {
    let mut d: *mut dep = 0 as *mut dep;
    let mut dp: *mut *mut dep = 0 as *mut *mut dep;
    let mut fstem: *const libc::c_char = 0 as *const libc::c_char;
    let mut initialized: libc::c_int = 0 as libc::c_int;
    let mut changed_dep: libc::c_int = 0 as libc::c_int;
    if (*f).snapped() != 0 {
        return;
    }
    (*f).set_snapped(1 as libc::c_int as libc::c_uint);
    dp = &mut (*f).deps;
    d = (*f).deps;
    while !d.is_null() {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut new: *mut dep = 0 as *mut dep;
        let mut next: *mut dep = 0 as *mut dep;
        if ((*d).name).is_null() || (*d).need_2nd_expansion() == 0 {
            dp = &mut (*d).next;
            d = (*d).next;
        } else {
            if (*d).staticpattern() != 0 {
                let mut cs: *const libc::c_char = (*d).name;
                let mut nperc: size_t = 0 as libc::c_int as size_t;
                loop {
                    cs = strchr(cs, '%' as i32);
                    if cs.is_null() {
                        break;
                    }
                    nperc = nperc.wrapping_add(1);
                    nperc;
                    cs = cs.offset(1);
                    cs;
                }
                if nperc != 0 {
                    let mut slen: size_t = (strlen((*d).name))
                        .wrapping_add(nperc)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    let mut pcs: *const libc::c_char = (*d).name;
                    let mut name: *mut libc::c_char = xmalloc(slen) as *mut libc::c_char;
                    let mut s: *mut libc::c_char = name;
                    cs = strchr(pcs, '%' as i32);
                    while !cs.is_null() {
                        s = mempcpy(
                            s as *mut libc::c_void,
                            pcs as *const libc::c_void,
                            cs.offset_from(pcs) as libc::c_long as libc::c_ulong,
                        ) as *mut libc::c_char;
                        let fresh1 = s;
                        s = s.offset(1);
                        *fresh1 = '$' as i32 as libc::c_char;
                        let fresh2 = s;
                        s = s.offset(1);
                        *fresh2 = '*' as i32 as libc::c_char;
                        cs = cs.offset(1);
                        pcs = cs;
                        cs = strchr(end_of_token(cs), '%' as i32);
                    }
                    strcpy(s, pcs);
                    free((*d).name as *mut libc::c_char as *mut libc::c_void);
                    (*d).name = name;
                }
            }
            if initialized == 0 {
                initialize_file_variables(f, 0 as libc::c_int);
                initialized = 1 as libc::c_int;
            }
            set_file_variables(
                f,
                if !((*d).stem).is_null() { (*d).stem } else { (*f).stem },
            );
            p = variable_expand_for_file((*d).name, f);
            free((*d).name as *mut libc::c_char as *mut libc::c_void);
            new = split_prereqs(p);
            if new.is_null() {
                *dp = (*d).next;
                changed_dep = 1 as libc::c_int;
                free(d as *mut libc::c_void);
                d = *dp;
            } else {
                fstem = (*d).stem;
                next = (*d).next;
                changed_dep = 1 as libc::c_int;
                free(d as *mut libc::c_void);
                *dp = new;
                dp = &mut new;
                d = new;
                while !d.is_null() {
                    (*d).file = lookup_file((*d).name);
                    if ((*d).file).is_null() {
                        (*d).file = enter_file((*d).name);
                    }
                    (*d).name = 0 as *const libc::c_char;
                    (*d).stem = fstem;
                    if fstem.is_null() {
                        (*(*d).file).set_is_explicit(1 as libc::c_int as libc::c_uint);
                    }
                    dp = &mut (*d).next;
                    d = (*d).next;
                }
                *dp = next;
                d = *dp;
            }
        }
    }
    if changed_dep != 0 {
        shuffle_deps_recursive((*f).deps);
    }
}
pub unsafe extern "C" fn expand_extra_prereqs(mut extra: *const variable) -> *mut dep {
    let mut d: *mut dep = 0 as *mut dep;
    let mut prereqs: *mut dep = if !extra.is_null() {
        split_prereqs(variable_expand((*extra).value))
    } else {
        0 as *mut dep
    };
    d = prereqs;
    while !d.is_null() {
        (*d).file = lookup_file((*d).name);
        if ((*d).file).is_null() {
            (*d).file = enter_file((*d).name);
        }
        (*d).name = 0 as *const libc::c_char;
        (*d).set_ignore_automatic_vars(1 as libc::c_int as libc::c_uint);
        d = (*d).next;
    }
    return prereqs;
}
unsafe extern "C" fn snap_file(
    mut item: *const libc::c_void,
    mut arg: *mut libc::c_void,
) {
    let mut f: *mut file = item as *mut file;
    let mut prereqs: *mut dep = 0 as *mut dep;
    if second_expansion == 0 {
        (*f).set_updating(0 as libc::c_int as libc::c_uint);
    }
    if all_secondary != 0 && (*f).notintermediate() == 0 {
        (*f).set_intermediate(1 as libc::c_int as libc::c_uint);
    }
    if no_intermediates != 0 && (*f).intermediate() == 0 && (*f).secondary() == 0 {
        (*f).set_notintermediate(1 as libc::c_int as libc::c_uint);
    }
    if !((*f).variables).is_null() {
        prereqs = expand_extra_prereqs(
            lookup_variable_in_set(
                b".EXTRA_PREREQS\0" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                (*(*f).variables).set,
            ),
        );
    } else if (*f).is_target() != 0 {
        prereqs = copy_dep_chain(arg as *const dep);
    }
    if !prereqs.is_null() {
        let mut d: *mut dep = 0 as *mut dep;
        d = prereqs;
        while !d.is_null() {
            if (*f).name
                == (if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name })
                || *(*f).name as libc::c_int
                    == *(if !((*d).name).is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as libc::c_int
                    && (*(*f).name as libc::c_int == '\0' as i32
                        || strcmp(
                            ((*f).name).offset(1 as libc::c_int as isize),
                            (if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            })
                                .offset(1 as libc::c_int as isize),
                        ) == 0)
            {
                break;
            }
            d = (*d).next;
        }
        if !d.is_null() {
            free_ns_chain(prereqs as *mut nameseq);
        } else if ((*f).deps).is_null() {
            (*f).deps = prereqs;
        } else {
            d = (*f).deps;
            while !((*d).next).is_null() {
                d = (*d).next;
            }
            (*d).next = prereqs;
        }
    }
}
pub unsafe extern "C" fn snap_deps() {
    let mut f: *mut file = 0 as *mut file;
    let mut f2: *mut file = 0 as *mut file;
    let mut d: *mut dep = 0 as *mut dep;
    snapped_deps = 1 as libc::c_int;
    f = lookup_file(b".PRECIOUS\0" as *const u8 as *const libc::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_precious(1 as libc::c_int as libc::c_uint);
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".LOW_RESOLUTION_TIME\0" as *const u8 as *const libc::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_low_resolution_time(1 as libc::c_int as libc::c_uint);
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".PHONY\0" as *const u8 as *const libc::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                (*f2).set_phony(1 as libc::c_int as libc::c_uint);
                (*f2).set_is_target(1 as libc::c_int as libc::c_uint);
                (*f2).last_mtime = 1 as libc::c_int as uintmax_t;
                (*f2).mtime_before_update = 1 as libc::c_int as uintmax_t;
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".NOTINTERMEDIATE\0" as *const u8 as *const libc::c_char);
    while !f.is_null() {
        if !((*f).deps).is_null() {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).set_notintermediate(1 as libc::c_int as libc::c_uint);
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        } else {
            no_intermediates = 1 as libc::c_int as libc::c_uint;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".INTERMEDIATE\0" as *const u8 as *const libc::c_char);
    while !f.is_null() {
        d = (*f).deps;
        while !d.is_null() {
            f2 = (*d).file;
            while !f2.is_null() {
                if (*f2).notintermediate() != 0 {
                    fatal(
                        0 as *mut floc,
                        strlen((*f2).name),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s cannot be both .NOTINTERMEDIATE and .INTERMEDIATE\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*f2).name,
                    );
                } else {
                    (*f2).set_intermediate(1 as libc::c_int as libc::c_uint);
                }
                f2 = (*f2).prev;
            }
            d = (*d).next;
        }
        f = (*f).prev;
    }
    f = lookup_file(b".SECONDARY\0" as *const u8 as *const libc::c_char);
    while !f.is_null() {
        if !((*f).deps).is_null() {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    if (*f2).notintermediate() != 0 {
                        fatal(
                            0 as *mut floc,
                            strlen((*f2).name),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s cannot be both .NOTINTERMEDIATE and .SECONDARY\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*f2).name,
                        );
                    } else {
                        (*f2).set_secondary(1 as libc::c_int as libc::c_uint);
                        (*f2).set_intermediate((*f2).secondary());
                    }
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        } else {
            all_secondary = 1 as libc::c_int;
        }
        f = (*f).prev;
    }
    if no_intermediates != 0 && all_secondary != 0 {
        fatal(
            0 as *mut floc,
            0 as libc::c_int as size_t,
            dcgettext(
                0 as *const libc::c_char,
                b".NOTINTERMEDIATE and .SECONDARY are mutually exclusive\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    f = lookup_file(b".EXPORT_ALL_VARIABLES\0" as *const u8 as *const libc::c_char);
    if !f.is_null() && (*f).is_target() as libc::c_int != 0 {
        export_all_variables = 1 as libc::c_int;
    }
    f = lookup_file(b".IGNORE\0" as *const u8 as *const libc::c_char);
    if !f.is_null() && (*f).is_target() as libc::c_int != 0 {
        if ((*f).deps).is_null() {
            ignore_errors_flag = 1 as libc::c_int;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= 4 as libc::c_int;
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    f = lookup_file(b".SILENT\0" as *const u8 as *const libc::c_char);
    if !f.is_null() && (*f).is_target() as libc::c_int != 0 {
        if ((*f).deps).is_null() {
            run_silent = 1 as libc::c_int;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    (*f2).command_flags |= 2 as libc::c_int;
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    f = lookup_file(b".NOTPARALLEL\0" as *const u8 as *const libc::c_char);
    if !f.is_null() && (*f).is_target() as libc::c_int != 0 {
        let mut d2: *mut dep = 0 as *mut dep;
        if ((*f).deps).is_null() {
            not_parallel = 1 as libc::c_int;
        } else {
            d = (*f).deps;
            while !d.is_null() {
                f2 = (*d).file;
                while !f2.is_null() {
                    if !((*f2).deps).is_null() {
                        d2 = (*(*f2).deps).next;
                        while !d2.is_null() {
                            (*d2).set_wait_here(1 as libc::c_int as libc::c_uint);
                            d2 = (*d2).next;
                        }
                    }
                    f2 = (*f2).prev;
                }
                d = (*d).next;
            }
        }
    }
    let mut prereqs: *mut dep = expand_extra_prereqs(
        lookup_variable(
            b".EXTRA_PREREQS\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ),
    );
    hash_map_arg(
        &mut files,
        Some(
            snap_file
                as unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> (),
        ),
        prereqs as *mut libc::c_void,
    );
    free_ns_chain(prereqs as *mut nameseq);
}
pub unsafe extern "C" fn set_command_state(mut file: *mut file, mut state: cmd_state) {
    let mut d: *mut dep = 0 as *mut dep;
    (*file).set_command_state(state);
    d = (*file).also_make;
    while !d.is_null() {
        if state as libc::c_uint > (*(*d).file).command_state() as libc::c_uint {
            (*(*d).file).set_command_state(state);
        }
        d = (*d).next;
    }
}
pub unsafe extern "C" fn file_timestamp_cons(
    mut fname: *const libc::c_char,
    mut stamp: time_t,
    mut ns: libc::c_long,
) -> uintmax_t {
    let mut offset: libc::c_int = ((2 as libc::c_int + 1 as libc::c_int) as libc::c_long
        + (if 1 as libc::c_int != 0 { ns } else { 0 as libc::c_int as libc::c_long }))
        as libc::c_int;
    let mut s: uintmax_t = stamp as uintmax_t;
    let mut product: uintmax_t = s
        << (if 1 as libc::c_int != 0 { 30 as libc::c_int } else { 0 as libc::c_int });
    let mut ts: uintmax_t = product.wrapping_add(offset as libc::c_ulong);
    if !(s
        <= ((!(0 as libc::c_int as uintmax_t))
            .wrapping_sub(
                (if !(-(1 as libc::c_int) as uintmax_t
                    <= 0 as libc::c_int as libc::c_ulong)
                {
                    0 as libc::c_int as uintmax_t
                } else {
                    !(0 as libc::c_int as uintmax_t)
                        << (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                }),
            )
            .wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            >> (if 1 as libc::c_int != 0 { 30 as libc::c_int } else { 0 as libc::c_int })
            << (if 1 as libc::c_int != 0 {
                30 as libc::c_int
            } else {
                0 as libc::c_int
            }))
            .wrapping_add((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_add(
                (if 1 as libc::c_int != 0 {
                    1000000000 as libc::c_int
                } else {
                    1 as libc::c_int
                }) as libc::c_ulong,
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            >> (if 1 as libc::c_int != 0 { 30 as libc::c_int } else { 0 as libc::c_int })
        && product <= ts
        && ts
            <= ((!(0 as libc::c_int as uintmax_t))
                .wrapping_sub(
                    (if !(-(1 as libc::c_int) as uintmax_t
                        <= 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int as uintmax_t
                    } else {
                        !(0 as libc::c_int as uintmax_t)
                            << (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    }),
                )
                .wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                >> (if 1 as libc::c_int != 0 {
                    30 as libc::c_int
                } else {
                    0 as libc::c_int
                })
                << (if 1 as libc::c_int != 0 {
                    30 as libc::c_int
                } else {
                    0 as libc::c_int
                }))
                .wrapping_add((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_add(
                    (if 1 as libc::c_int != 0 {
                        1000000000 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as libc::c_ulong,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
    {
        let mut buf: [libc::c_char; 43] = [0; 43];
        let mut f: *const libc::c_char = if !fname.is_null() {
            fname
        } else {
            dcgettext(
                0 as *const libc::c_char,
                b"Current time\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ) as *const libc::c_char
        };
        ts = if s <= 2 as libc::c_int as libc::c_ulong {
            (2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
        } else {
            ((!(0 as libc::c_int as uintmax_t))
                .wrapping_sub(
                    (if !(-(1 as libc::c_int) as uintmax_t
                        <= 0 as libc::c_int as libc::c_ulong)
                    {
                        0 as libc::c_int as uintmax_t
                    } else {
                        !(0 as libc::c_int as uintmax_t)
                            << (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    }),
                )
                .wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                >> (if 1 as libc::c_int != 0 {
                    30 as libc::c_int
                } else {
                    0 as libc::c_int
                })
                << (if 1 as libc::c_int != 0 {
                    30 as libc::c_int
                } else {
                    0 as libc::c_int
                }))
                .wrapping_add((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_add(
                    (if 1 as libc::c_int != 0 {
                        1000000000 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as libc::c_ulong,
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        };
        file_timestamp_sprintf(buf.as_mut_ptr(), ts);
        error(
            0 as *mut floc,
            (strlen(f)).wrapping_add(strlen(buf.as_mut_ptr())),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Timestamp out of range; substituting %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            f,
            buf.as_mut_ptr(),
        );
    }
    return ts;
}
pub unsafe extern "C" fn file_timestamp_now(
    mut resolution: *mut libc::c_int,
) -> uintmax_t {
    let mut r: libc::c_int = 0;
    let mut s: time_t = 0;
    let mut ns: libc::c_int = 0;
    let mut timespec: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if clock_gettime(0 as libc::c_int, &mut timespec) == 0 as libc::c_int {
        r = 1 as libc::c_int;
        s = timespec.tv_sec;
        ns = timespec.tv_nsec as libc::c_int;
    } else {
        let mut timeval: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        if gettimeofday(&mut timeval, 0 as *mut libc::c_void) == 0 as libc::c_int {
            r = 1000 as libc::c_int;
            s = timeval.tv_sec;
            ns = (timeval.tv_usec * 1000 as libc::c_int as libc::c_long) as libc::c_int;
        } else {
            r = 1000000000 as libc::c_int;
            s = time(0 as *mut time_t);
            ns = 0 as libc::c_int;
        }
    }
    *resolution = r;
    return file_timestamp_cons(0 as *const libc::c_char, s, ns as libc::c_long);
}
pub unsafe extern "C" fn file_timestamp_sprintf(
    mut p: *mut libc::c_char,
    mut ts: uintmax_t,
) {
    let mut t: time_t = (ts
        .wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
        >> (if 1 as libc::c_int != 0 { 30 as libc::c_int } else { 0 as libc::c_int }))
        as time_t;
    let mut tm: *mut tm = localtime(&mut t);
    if !tm.is_null() {
        let mut year: intmax_t = (*tm).tm_year as intmax_t;
        sprintf(
            p,
            b"%04ld-%02d-%02d %02d:%02d:%02d\0" as *const u8 as *const libc::c_char,
            year + 1900 as libc::c_int as libc::c_long,
            (*tm).tm_mon + 1 as libc::c_int,
            (*tm).tm_mday,
            (*tm).tm_hour,
            (*tm).tm_min,
            (*tm).tm_sec,
        );
    } else if t < 0 as libc::c_int as libc::c_long {
        sprintf(p, b"%ld\0" as *const u8 as *const libc::c_char, t);
    } else {
        sprintf(p, b"%lu\0" as *const u8 as *const libc::c_char, t as uintmax_t);
    }
    p = p.offset(strlen(p) as isize);
    sprintf(
        p,
        b".%09d\0" as *const u8 as *const libc::c_char,
        (ts.wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            & (((1 as libc::c_int)
                << (if 1 as libc::c_int != 0 {
                    30 as libc::c_int
                } else {
                    0 as libc::c_int
                })) - 1 as libc::c_int) as libc::c_ulong) as libc::c_int,
    );
    p = p.offset((strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    while *p as libc::c_int == '0' as i32 {
        p = p.offset(-1);
        p;
    }
    p = p.offset((*p as libc::c_int != '.' as i32) as libc::c_int as isize);
    *p = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn print_prereqs(mut deps: *const dep) {
    let mut ood: *const dep = 0 as *const dep;
    while !deps.is_null() {
        if (*deps).ignore_mtime() == 0 {
            printf(
                b" %s%s\0" as *const u8 as *const libc::c_char,
                if (*deps).wait_here() as libc::c_int != 0 {
                    b".WAIT \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if !((*deps).name).is_null() {
                    (*deps).name
                } else {
                    (*(*deps).file).name
                },
            );
        } else if ood.is_null() {
            ood = deps;
        }
        deps = (*deps).next;
    }
    if !ood.is_null() {
        printf(
            b" | %s%s\0" as *const u8 as *const libc::c_char,
            if (*ood).wait_here() as libc::c_int != 0 {
                b".WAIT \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if !((*ood).name).is_null() { (*ood).name } else { (*(*ood).file).name },
        );
        ood = (*ood).next;
        while !ood.is_null() {
            if (*ood).ignore_mtime() != 0 {
                printf(
                    b" %s%s\0" as *const u8 as *const libc::c_char,
                    if (*ood).wait_here() as libc::c_int != 0 {
                        b".WAIT \0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    if !((*ood).name).is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    },
                );
            }
            ood = (*ood).next;
        }
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn print_file(mut item: *const libc::c_void) {
    let mut f: *const file = item as *const file;
    if no_builtin_rules_flag != 0 && (*f).builtin() as libc::c_int != 0 {
        return;
    }
    putchar('\n' as i32);
    if !((*f).cmds).is_null()
        && (*(*f).cmds).recipe_prefix as libc::c_int != cmd_prefix as libc::c_int
    {
        fputs(b".RECIPEPREFIX = \0" as *const u8 as *const libc::c_char, stdout);
        cmd_prefix = (*(*f).cmds).recipe_prefix;
        if cmd_prefix as libc::c_int != '\t' as i32 {
            putchar(cmd_prefix as libc::c_int);
        }
        putchar('\n' as i32);
    }
    if !((*f).variables).is_null() {
        print_target_variables(f);
    }
    if (*f).is_target() == 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"# Not a target:\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    printf(
        b"%s:%s\0" as *const u8 as *const libc::c_char,
        (*f).name,
        if !((*f).double_colon).is_null() {
            b":\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    print_prereqs((*f).deps);
    if (*f).precious() != 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  Precious file (prerequisite of .PRECIOUS).\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (*f).phony() != 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  Phony target (prerequisite of .PHONY).\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (*f).cmd_target() != 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  Command line target.\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (*f).dontcare() != 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  A default, MAKEFILES, or -include/sinclude makefile.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (*f).builtin() != 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  Builtin rule\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    puts(
        if (*f).tried_implicit() as libc::c_int != 0 {
            dcgettext(
                0 as *const libc::c_char,
                b"#  Implicit rule search has been done.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            )
        } else {
            dcgettext(
                0 as *const libc::c_char,
                b"#  Implicit rule search has not been done.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            )
        },
    );
    if !((*f).stem).is_null() {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"#  Implicit/static pattern stem: '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*f).stem,
        );
    }
    if (*f).intermediate() != 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  File is an intermediate prerequisite.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (*f).notintermediate() != 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  File is a prerequisite of .NOTINTERMEDIATE.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (*f).secondary() != 0 {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  File is secondary (prerequisite of .SECONDARY).\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !((*f).also_make).is_null() {
        let mut d: *const dep = 0 as *const dep;
        fputs(
            dcgettext(
                0 as *const libc::c_char,
                b"#  Also makes:\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        d = (*f).also_make;
        while !d.is_null() {
            printf(
                b" %s\0" as *const u8 as *const libc::c_char,
                if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
            );
            d = (*d).next;
        }
        putchar('\n' as i32);
    }
    if (*f).last_mtime == 0 as libc::c_int as libc::c_ulong {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  Modification time never checked.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if (*f).last_mtime == 1 as libc::c_int as libc::c_ulong {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  File does not exist.\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else if (*f).last_mtime == 2 as libc::c_int as libc::c_ulong {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"#  File is very old.\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        let mut buf: [libc::c_char; 43] = [0; 43];
        file_timestamp_sprintf(buf.as_mut_ptr(), (*f).last_mtime);
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"#  Last modified %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            buf.as_mut_ptr(),
        );
    }
    puts(
        if (*f).updated() as libc::c_int != 0 {
            dcgettext(
                0 as *const libc::c_char,
                b"#  File has been updated.\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        } else {
            dcgettext(
                0 as *const libc::c_char,
                b"#  File has not been updated.\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            )
        },
    );
    match (*f).command_state() as libc::c_int {
        2 => {
            puts(
                dcgettext(
                    0 as *const libc::c_char,
                    b"#  Recipe currently running (THIS IS A BUG).\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        1 => {
            puts(
                dcgettext(
                    0 as *const libc::c_char,
                    b"#  Dependencies recipe running (THIS IS A BUG).\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        0 | 3 => {
            match (*f).update_status() as libc::c_int {
                0 => {
                    puts(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"#  Successfully updated.\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                2 => {
                    puts(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"#  Needs to be updated (-q is set).\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                3 => {
                    puts(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"#  Failed to be updated.\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                1 | _ => {}
            }
        }
        _ => {
            puts(
                dcgettext(
                    0 as *const libc::c_char,
                    b"#  Invalid value in 'command_state' member!\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fflush(stdout);
            fflush(stderr);
            abort();
        }
    }
    if !((*f).variables).is_null() {
        print_file_variables(f);
    }
    if !((*f).cmds).is_null() {
        print_commands((*f).cmds);
    }
    if !((*f).prev).is_null() {
        print_file((*f).prev as *const libc::c_void);
    }
}
pub unsafe extern "C" fn print_file_data_base() {
    puts(
        dcgettext(
            0 as *const libc::c_char,
            b"\n# Files\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    hash_map(
        &mut files,
        Some(print_file as unsafe extern "C" fn(*const libc::c_void) -> ()),
    );
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"\n# files hash-table stats:\n# \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    hash_print_stats(&mut files, stdout);
}
unsafe extern "C" fn verify_file(mut item: *const libc::c_void) {
    let mut f: *const file = item as *const file;
    let mut d: *const dep = 0 as *const dep;
    if !((*f).name).is_null()
        && *((*f).name).offset(0 as libc::c_int as isize) as libc::c_int != 0
        && strcache_iscached((*f).name) == 0
    {
        error(
            0 as *const floc,
            (strlen((*f).name))
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(strlen((*f).name)),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Field '%s' not cached: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*f).name,
            b"name\0" as *const u8 as *const libc::c_char,
            (*f).name,
        );
    }
    if !((*f).hname).is_null()
        && *((*f).hname).offset(0 as libc::c_int as isize) as libc::c_int != 0
        && strcache_iscached((*f).hname) == 0
    {
        error(
            0 as *const floc,
            (strlen((*f).name))
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(strlen((*f).hname)),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Field '%s' not cached: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*f).name,
            b"hname\0" as *const u8 as *const libc::c_char,
            (*f).hname,
        );
    }
    if !((*f).vpath).is_null()
        && *((*f).vpath).offset(0 as libc::c_int as isize) as libc::c_int != 0
        && strcache_iscached((*f).vpath) == 0
    {
        error(
            0 as *const floc,
            (strlen((*f).name))
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(strlen((*f).vpath)),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Field '%s' not cached: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*f).name,
            b"vpath\0" as *const u8 as *const libc::c_char,
            (*f).vpath,
        );
    }
    if !((*f).stem).is_null()
        && *((*f).stem).offset(0 as libc::c_int as isize) as libc::c_int != 0
        && strcache_iscached((*f).stem) == 0
    {
        error(
            0 as *const floc,
            (strlen((*f).name))
                .wrapping_add(
                    (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(strlen((*f).stem)),
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Field '%s' not cached: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*f).name,
            b"stem\0" as *const u8 as *const libc::c_char,
            (*f).stem,
        );
    }
    d = (*f).deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0 {
            if !((*d).name).is_null()
                && *((*d).name).offset(0 as libc::c_int as isize) as libc::c_int != 0
                && strcache_iscached((*d).name) == 0
            {
                error(
                    0 as *const floc,
                    (strlen((*d).name))
                        .wrapping_add(
                            (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_add(strlen((*d).name)),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Field '%s' not cached: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*d).name,
                    b"name\0" as *const u8 as *const libc::c_char,
                    (*d).name,
                );
            }
        }
        if !((*d).stem).is_null()
            && *((*d).stem).offset(0 as libc::c_int as isize) as libc::c_int != 0
            && strcache_iscached((*d).stem) == 0
        {
            error(
                0 as *const floc,
                (strlen((*d).name))
                    .wrapping_add(
                        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(strlen((*d).stem)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Field '%s' not cached: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*d).name,
                b"stem\0" as *const u8 as *const libc::c_char,
                (*d).stem,
            );
        }
        d = (*d).next;
    }
}
pub unsafe extern "C" fn verify_file_data_base() {
    hash_map(
        &mut files,
        Some(verify_file as unsafe extern "C" fn(*const libc::c_void) -> ()),
    );
}
pub unsafe extern "C" fn build_target_list(
    mut value: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut last_targ_count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if files.ht_fill != last_targ_count {
        let mut max: size_t = (strlen(value))
            .wrapping_div(500 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(500 as libc::c_int as libc::c_ulong);
        let mut len: size_t = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut fp: *mut *mut file = files.ht_vec as *mut *mut file;
        let mut end: *mut *mut file = &mut *fp.offset(files.ht_size as isize)
            as *mut *mut file;
        value = xrealloc(value as *mut libc::c_void, max) as *mut libc::c_char;
        p = value;
        len = 0 as libc::c_int as size_t;
        while fp < end {
            if !((*fp).is_null() || *fp as *mut libc::c_void == hash_deleted_item)
                && (**fp).is_target() as libc::c_int != 0
            {
                let mut f: *mut file = *fp;
                let mut l: size_t = strlen((*f).name);
                len = (len as libc::c_ulong)
                    .wrapping_add(l.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
                if len > max {
                    let mut off: size_t = p.offset_from(value) as libc::c_long as size_t;
                    max = (max as libc::c_ulong)
                        .wrapping_add(
                            l
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                .wrapping_div(500 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(500 as libc::c_int as libc::c_ulong),
                        ) as size_t as size_t;
                    value = xrealloc(value as *mut libc::c_void, max)
                        as *mut libc::c_char;
                    p = &mut *value.offset(off as isize) as *mut libc::c_char;
                }
                p = mempcpy(p as *mut libc::c_void, (*f).name as *const libc::c_void, l)
                    as *mut libc::c_char;
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = ' ' as i32 as libc::c_char;
            }
            fp = fp.offset(1);
            fp;
        }
        *p.offset(-(1 as libc::c_int as isize)) = '\0' as i32 as libc::c_char;
        last_targ_count = files.ht_fill;
    }
    return value;
}
pub unsafe extern "C" fn init_hash_files() {
    hash_init(
        &mut files,
        1000 as libc::c_int as libc::c_ulong,
        Some(file_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong),
        Some(file_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong),
        Some(
            file_hash_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
