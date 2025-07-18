use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn remote_kill(id: pid_t, sig: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    static mut one_shell: libc::c_int;
    fn temp_stdin_unlink();
    fn pfatal_with_name(_: *const libc::c_char) -> !;
    fn perror_with_name(_: *const libc::c_char, _: *const libc::c_char);
    fn make_pid() -> pid_t;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    fn ar_name(_: *const libc::c_char) -> libc::c_int;
    fn ar_member_date(_: *const libc::c_char) -> time_t;
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    fn strcache_add_len(str: *const libc::c_char, len: size_t) -> *const libc::c_char;
    fn unload_file(name: *const libc::c_char) -> libc::c_int;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut always_make_flag: libc::c_int;
    static mut default_file: *mut file;
    fn enter_file(name: *const libc::c_char) -> *mut file;
    fn remove_intermediates(sig: libc::c_int);
    fn set_command_state(file: *mut file, state: cmd_state);
    fn notice_finished_file(file: *mut file);
    fn file_timestamp_cons(
        _: *const libc::c_char,
        _: time_t,
        _: libc::c_long,
    ) -> uintmax_t;
    fn hash_init(
        ht: *mut hash_table,
        size: libc::c_ulong,
        hash_1: hash_func_t,
        hash_2: hash_func_t,
        hash_cmp: hash_cmp_func_t,
    );
    fn hash_find_slot(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut *mut libc::c_void;
    fn hash_find_item(
        ht: *mut hash_table,
        key: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_insert_at(
        ht: *mut hash_table,
        item: *const libc::c_void,
        slot: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_free(ht: *mut hash_table, free_items: libc::c_int);
    fn jhash_string(key: *const libc::c_uchar) -> libc::c_uint;
    static mut hash_deleted_item: *mut libc::c_void;
    fn jobserver_clear();
    fn osync_clear();
    fn initialize_file_variables(file: *mut file, reading: libc::c_int);
    fn define_variable_in_set(
        name: *const libc::c_char,
        length: size_t,
        value: *const libc::c_char,
        origin: variable_origin,
        recursive: libc::c_int,
        set: *mut variable_set,
        flocp: *const floc,
    ) -> *mut variable;
    static mut job_slots_used: libc::c_uint;
    static mut children: *mut child;
    fn new_job(file: *mut file);
    fn reap_children(block: libc::c_int, err: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct output {
    pub out: libc::c_int,
    pub err: libc::c_int,
    #[bitfield(name = "syncout", ty = "libc::c_uint", bits = "0..=0")]
    pub syncout: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct child {
    pub cmd_name: *mut libc::c_char,
    pub environment: *mut *mut libc::c_char,
    pub output: output,
    pub next: *mut child,
    pub file: *mut file,
    pub sh_batch_file: *mut libc::c_char,
    pub command_lines: *mut *mut libc::c_char,
    pub command_ptr: *mut libc::c_char,
    pub command_line: libc::c_uint,
    pub pid: pid_t,
    #[bitfield(name = "remote", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "noerror", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "good_stdin", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "deleted", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "recursive", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "jobslot", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "6..=6")]
    pub remote_noerror_good_stdin_deleted_recursive_jobslot_dontcare: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn dep_hash_1(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut d: *const dep = key as *const dep;
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut _key_: *const libc::c_uchar = (if !((*d).name).is_null() {
        (*d).name
    } else {
        (*(*d).file).name
    }) as *const libc::c_uchar;
    _result_ = _result_.wrapping_add(jhash_string(_key_) as libc::c_ulong);
    return _result_;
}
unsafe extern "C" fn dep_hash_2(mut key: *const libc::c_void) -> libc::c_ulong {
    let mut d: *const dep = key as *const dep;
    let mut _result_: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if !((*d).name).is_null() {} else {};
    return _result_;
}
unsafe extern "C" fn dep_hash_cmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> libc::c_int {
    let mut dx: *const dep = x as *const dep;
    let mut dy: *const dep = y as *const dep;
    return strcmp(
        if !((*dx).name).is_null() { (*dx).name } else { (*(*dx).file).name },
        if !((*dy).name).is_null() { (*dy).name } else { (*(*dy).file).name },
    );
}
pub unsafe extern "C" fn set_file_variables(
    mut file: *mut file,
    mut stem: *const libc::c_char,
) {
    let mut d: *mut dep = 0 as *mut dep;
    let mut at: *const libc::c_char = 0 as *const libc::c_char;
    let mut percent: *const libc::c_char = 0 as *const libc::c_char;
    let mut star: *const libc::c_char = 0 as *const libc::c_char;
    let mut less: *const libc::c_char = 0 as *const libc::c_char;
    if ar_name((*file).name) != 0 {
        let mut len: size_t = 0;
        let mut cp: *const libc::c_char = 0 as *const libc::c_char;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        cp = strchr((*file).name, '(' as i32);
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (cp.offset_from((*file).name) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong as usize,
        );
        p = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
        memcpy(
            p as *mut libc::c_void,
            (*file).name as *const libc::c_void,
            cp.offset_from((*file).name) as libc::c_long as libc::c_ulong,
        );
        *p
            .offset(
                cp.offset_from((*file).name) as libc::c_long as isize,
            ) = '\0' as i32 as libc::c_char;
        at = p;
        len = strlen(cp.offset(1 as libc::c_int as isize));
        let mut fresh1 = ::std::vec::from_elem(0, len as usize);
        p = fresh1.leak().as_mut_ptr() as *mut libc::c_char;
        memcpy(
            p as *mut libc::c_void,
            cp.offset(1 as libc::c_int as isize) as *const libc::c_void,
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        *p
            .offset(
                len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        percent = p;
    } else {
        at = (*file).name;
        percent = b"\0" as *const u8 as *const libc::c_char;
    }
    if stem.is_null() {
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut len_0: size_t = 0;
        if ar_name((*file).name) != 0 {
            name = (strchr((*file).name, '(' as i32)).offset(1 as libc::c_int as isize);
            len_0 = (strlen(name)).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        } else {
            name = (*file).name;
            len_0 = strlen(name);
        }
        d = (*enter_file(
            strcache_add(b".SUFFIXES\0" as *const u8 as *const libc::c_char),
        ))
            .deps;
        while !d.is_null() {
            let mut dn: *const libc::c_char = if !((*d).name).is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            let mut slen: size_t = strlen(dn);
            if len_0 > slen
                && memcmp(
                    dn as *const libc::c_void,
                    name.offset(len_0.wrapping_sub(slen) as isize)
                        as *const libc::c_void,
                    slen,
                ) == 0 as libc::c_int
            {
                stem = strcache_add_len(name, len_0.wrapping_sub(slen));
                (*file).stem = stem;
                break;
            } else {
                d = (*d).next;
            }
        }
        if d.is_null() {
            stem = b"\0" as *const u8 as *const libc::c_char;
            (*file).stem = stem;
        }
    }
    star = stem;
    less = b"\0" as *const u8 as *const libc::c_char;
    d = (*file).deps;
    while !d.is_null() {
        if (*d).ignore_mtime() == 0 && (*d).ignore_automatic_vars() == 0
            && (*d).need_2nd_expansion() == 0
        {
            less = if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name };
            break;
        } else {
            d = (*d).next;
        }
    }
    if !((*file).cmds).is_null() && (*file).cmds == (*default_file).cmds {
        less = at;
    }
    define_variable_in_set(
        b"<\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        less,
        o_automatic,
        0 as libc::c_int,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"*\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        star,
        o_automatic,
        0 as libc::c_int,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"@\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        at,
        o_automatic,
        0 as libc::c_int,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    define_variable_in_set(
        b"%\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        percent,
        o_automatic,
        0 as libc::c_int,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    static mut plus_value: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut bar_value: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut qmark_value: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut plus_max: size_t = 0 as libc::c_int as size_t;
    static mut bar_max: size_t = 0 as libc::c_int as size_t;
    static mut qmark_max: size_t = 0 as libc::c_int as size_t;
    let mut qmark_len: size_t = 0;
    let mut plus_len: size_t = 0;
    let mut bar_len: size_t = 0;
    let mut cp_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut caret_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len_1: size_t = 0;
    let mut dep_hash: hash_table = hash_table {
        ht_vec: 0 as *mut *mut libc::c_void,
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
    let mut slot: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    plus_len = 0 as libc::c_int as size_t;
    bar_len = 0 as libc::c_int as size_t;
    d = (*file).deps;
    while !d.is_null() {
        if (*d).need_2nd_expansion() == 0 && (*d).ignore_automatic_vars() == 0 {
            if (*d).ignore_mtime() != 0 {
                bar_len = (bar_len as libc::c_ulong)
                    .wrapping_add(
                        (strlen(
                            (if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            }),
                        ))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
            } else {
                plus_len = (plus_len as libc::c_ulong)
                    .wrapping_add(
                        (strlen(
                            (if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            }),
                        ))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as size_t as size_t;
            }
        }
        d = (*d).next;
    }
    if bar_len == 0 as libc::c_int as libc::c_ulong {
        bar_len = bar_len.wrapping_add(1);
        bar_len;
    }
    if plus_len == 0 as libc::c_int as libc::c_ulong {
        plus_len = plus_len.wrapping_add(1);
        plus_len;
    }
    if plus_len > plus_max {
        plus_max = plus_len;
        plus_value = xrealloc(plus_value as *mut libc::c_void, plus_max)
            as *mut libc::c_char;
    }
    cp_0 = plus_value;
    qmark_len = plus_len.wrapping_add(1 as libc::c_int as libc::c_ulong);
    d = (*file).deps;
    while !d.is_null() {
        if (*d).ignore_mtime() == 0 && (*d).need_2nd_expansion() == 0
            && (*d).ignore_automatic_vars() == 0
        {
            let mut c: *const libc::c_char = if !((*d).name).is_null() {
                (*d).name
            } else {
                (*(*d).file).name
            };
            if ar_name(c) != 0 {
                c = (strchr(c, '(' as i32)).offset(1 as libc::c_int as isize);
                len_1 = (strlen(c)).wrapping_sub(1 as libc::c_int as libc::c_ulong);
            } else {
                len_1 = strlen(c);
            }
            cp_0 = mempcpy(cp_0 as *mut libc::c_void, c as *const libc::c_void, len_1)
                as *mut libc::c_char;
            let fresh2 = cp_0;
            cp_0 = cp_0.offset(1);
            *fresh2 = ' ' as i32 as libc::c_char;
            if !((*d).changed() as libc::c_int != 0 || always_make_flag != 0) {
                qmark_len = (qmark_len as libc::c_ulong)
                    .wrapping_sub(len_1.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as size_t as size_t;
            }
        }
        d = (*d).next;
    }
    *cp_0
        .offset(
            (if cp_0 > plus_value { -(1 as libc::c_int) } else { 0 as libc::c_int })
                as isize,
        ) = '\0' as i32 as libc::c_char;
    define_variable_in_set(
        b"+\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        plus_value,
        o_automatic,
        0 as libc::c_int,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    caret_value = plus_value;
    cp_0 = caret_value;
    if qmark_len > qmark_max {
        qmark_max = qmark_len;
        qmark_value = xrealloc(qmark_value as *mut libc::c_void, qmark_max)
            as *mut libc::c_char;
    }
    qp = qmark_value;
    if bar_len > bar_max {
        bar_max = bar_len;
        bar_value = xrealloc(bar_value as *mut libc::c_void, bar_max)
            as *mut libc::c_char;
    }
    bp = bar_value;
    hash_init(
        &mut dep_hash,
        500 as libc::c_int as libc::c_ulong,
        Some(dep_hash_1 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong),
        Some(dep_hash_2 as unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong),
        Some(
            dep_hash_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    d = (*file).deps;
    while !d.is_null() {
        if !((*d).need_2nd_expansion() as libc::c_int != 0
            || (*d).ignore_automatic_vars() as libc::c_int != 0)
        {
            slot = hash_find_slot(&mut dep_hash, d as *const libc::c_void);
            if (*slot).is_null() || *slot == hash_deleted_item {
                hash_insert_at(
                    &mut dep_hash,
                    d as *const libc::c_void,
                    slot as *const libc::c_void,
                );
            } else {
                let mut hd: *mut dep = *slot as *mut dep;
                if (*d).ignore_mtime() as libc::c_int
                    != (*hd).ignore_mtime() as libc::c_int
                {
                    (*hd).set_ignore_mtime(0 as libc::c_int as libc::c_uint);
                    (*d).set_ignore_mtime((*hd).ignore_mtime());
                }
            }
        }
        d = (*d).next;
    }
    d = (*file).deps;
    while !d.is_null() {
        let mut c_0: *const libc::c_char = 0 as *const libc::c_char;
        if !((*d).need_2nd_expansion() as libc::c_int != 0
            || (*d).ignore_automatic_vars() as libc::c_int != 0
            || hash_find_item(&mut dep_hash, d as *const libc::c_void)
                != d as *mut libc::c_void)
        {
            c_0 = if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name };
            if ar_name(c_0) != 0 {
                c_0 = (strchr(c_0, '(' as i32)).offset(1 as libc::c_int as isize);
                len_1 = (strlen(c_0)).wrapping_sub(1 as libc::c_int as libc::c_ulong);
            } else {
                len_1 = strlen(c_0);
            }
            if (*d).ignore_mtime() != 0 {
                bp = mempcpy(bp as *mut libc::c_void, c_0 as *const libc::c_void, len_1)
                    as *mut libc::c_char;
                let fresh3 = bp;
                bp = bp.offset(1);
                *fresh3 = ' ' as i32 as libc::c_char;
            } else {
                cp_0 = mempcpy(
                    cp_0 as *mut libc::c_void,
                    c_0 as *const libc::c_void,
                    len_1,
                ) as *mut libc::c_char;
                let fresh4 = cp_0;
                cp_0 = cp_0.offset(1);
                *fresh4 = ' ' as i32 as libc::c_char;
                if (*d).changed() as libc::c_int != 0 || always_make_flag != 0 {
                    qp = mempcpy(
                        qp as *mut libc::c_void,
                        c_0 as *const libc::c_void,
                        len_1,
                    ) as *mut libc::c_char;
                    let fresh5 = qp;
                    qp = qp.offset(1);
                    *fresh5 = ' ' as i32 as libc::c_char;
                }
            }
        }
        d = (*d).next;
    }
    hash_free(&mut dep_hash, 0 as libc::c_int);
    *cp_0
        .offset(
            (if cp_0 > caret_value { -(1 as libc::c_int) } else { 0 as libc::c_int })
                as isize,
        ) = '\0' as i32 as libc::c_char;
    define_variable_in_set(
        b"^\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        caret_value,
        o_automatic,
        0 as libc::c_int,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    *qp
        .offset(
            (if qp > qmark_value { -(1 as libc::c_int) } else { 0 as libc::c_int })
                as isize,
        ) = '\0' as i32 as libc::c_char;
    define_variable_in_set(
        b"?\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        qmark_value,
        o_automatic,
        0 as libc::c_int,
        (*(*file).variables).set,
        0 as *mut floc,
    );
    *bp
        .offset(
            (if bp > bar_value { -(1 as libc::c_int) } else { 0 as libc::c_int })
                as isize,
        ) = '\0' as i32 as libc::c_char;
    define_variable_in_set(
        b"|\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        bar_value,
        o_automatic,
        0 as libc::c_int,
        (*(*file).variables).set,
        0 as *mut floc,
    );
}
pub unsafe extern "C" fn chop_commands(mut cmds: *mut commands) {
    let mut nlines: libc::c_ushort = 0;
    let mut i: libc::c_ushort = 0;
    let mut lines: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if cmds.is_null() || !((*cmds).command_lines).is_null() {
        return;
    }
    if one_shell != 0 {
        let mut l: size_t = strlen((*cmds).commands);
        nlines = 1 as libc::c_int as libc::c_ushort;
        lines = xmalloc(
            (nlines as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        let ref mut fresh6 = *lines.offset(0 as libc::c_int as isize);
        *fresh6 = xstrdup((*cmds).commands);
        if l > 0 as libc::c_int as libc::c_ulong
            && *(*lines.offset(0 as libc::c_int as isize))
                .offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int == '\n' as i32
        {
            *(*lines.offset(0 as libc::c_int as isize))
                .offset(
                    l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
    } else {
        let mut p: *const libc::c_char = (*cmds).commands;
        let mut max: size_t = 5 as libc::c_int as size_t;
        nlines = 0 as libc::c_int as libc::c_ushort;
        lines = xmalloc(
            max.wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        ) as *mut *mut libc::c_char;
        while *p as libc::c_int != '\0' as i32 {
            let mut end: *const libc::c_char = p;
            loop {
                end = strchr(end, '\n' as i32);
                if end.is_null() {
                    end = p.offset(strlen(p) as isize);
                    break;
                } else {
                    if !(end > p
                        && *end.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '\\' as i32)
                    {
                        break;
                    }
                    let mut backslash: libc::c_int = 1 as libc::c_int;
                    if end > p.offset(1 as libc::c_int as isize) {
                        let mut b: *const libc::c_char = 0 as *const libc::c_char;
                        b = end.offset(-(2 as libc::c_int as isize));
                        while b >= p && *b as libc::c_int == '\\' as i32 {
                            backslash = (backslash == 0) as libc::c_int;
                            b = b.offset(-1);
                            b;
                        }
                    }
                    if !(backslash != 0) {
                        break;
                    }
                    end = end.offset(1);
                    end;
                }
            }
            if nlines as libc::c_int
                == 32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
            {
                fatal(
                    &mut (*cmds).fileinfo as *mut floc,
                    (53 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<uintmax_t>() as libc::c_ulong,
                        )
                        .wrapping_div(22 as libc::c_int as libc::c_ulong)
                        .wrapping_add(3 as libc::c_int as libc::c_ulong),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Recipe has too many lines (limit %hu)\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    nlines as libc::c_int,
                );
            }
            if nlines as libc::c_ulong == max {
                max = (max as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                lines = xrealloc(
                    lines as *mut libc::c_void,
                    max
                        .wrapping_mul(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as *mut *mut libc::c_char;
            }
            let fresh7 = nlines;
            nlines = nlines.wrapping_add(1);
            let ref mut fresh8 = *lines.offset(fresh7 as isize);
            *fresh8 = xstrndup(p, end.offset_from(p) as libc::c_long as size_t);
            p = end;
            if *p as libc::c_int != '\0' as i32 {
                p = p.offset(1);
                p;
            }
        }
    }
    (*cmds).ncommand_lines = nlines;
    (*cmds).command_lines = lines;
    (*cmds).set_any_recurse(0 as libc::c_int as libc::c_uint);
    (*cmds).lines_flags = xmalloc(nlines as size_t) as *mut libc::c_uchar;
    i = 0 as libc::c_int as libc::c_ushort;
    while (i as libc::c_int) < nlines as libc::c_int {
        let mut flags: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
        let mut p_0: *const libc::c_char = *lines.offset(i as isize);
        while *stopchar_map.as_mut_ptr().offset(*p_0 as libc::c_uchar as isize)
            as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
            || *p_0 as libc::c_int == '-' as i32 || *p_0 as libc::c_int == '@' as i32
            || *p_0 as libc::c_int == '+' as i32
        {
            let fresh9 = p_0;
            p_0 = p_0.offset(1);
            match *fresh9 as libc::c_int {
                43 => {
                    flags = (flags as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
                }
                64 => {
                    flags = (flags as libc::c_int | 2 as libc::c_int) as libc::c_uchar;
                }
                45 => {
                    flags = (flags as libc::c_int | 4 as libc::c_int) as libc::c_uchar;
                }
                _ => {}
            }
        }
        if !(flags as libc::c_int & 1 as libc::c_int != 0 as libc::c_int)
            && (!(strstr(p_0, b"$(MAKE)\0" as *const u8 as *const libc::c_char))
                .is_null()
                || !(strstr(p_0, b"${MAKE}\0" as *const u8 as *const libc::c_char))
                    .is_null())
        {
            flags = (flags as libc::c_int | 1 as libc::c_int) as libc::c_uchar;
        }
        *((*cmds).lines_flags).offset(i as isize) = flags;
        (*cmds)
            .set_any_recurse(
                (*cmds).any_recurse()
                    | (if flags as libc::c_int & 1 as libc::c_int != 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint,
            );
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn execute_file_commands(mut file: *mut file) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = (*(*file).cmds).commands;
    while *p as libc::c_int != '\0' as i32 {
        if !(*stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
            as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
            != 0 as libc::c_int) && *p as libc::c_int != '-' as i32
            && *p as libc::c_int != '@' as i32 && *p as libc::c_int != '+' as i32
        {
            break;
        }
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '\0' as i32 {
        set_command_state(file, cs_running);
        (*file).set_update_status(us_success);
        notice_finished_file(file);
        return;
    }
    initialize_file_variables(file, 0 as libc::c_int);
    set_file_variables(file, (*file).stem);
    if (*file).loaded() as libc::c_int != 0
        && unload_file((*file).name) == 0 as libc::c_int
    {
        (*file).set_loaded(0 as libc::c_int as libc::c_uint);
        (*file).set_unloaded(1 as libc::c_int as libc::c_uint);
    }
    new_job(file);
}
pub static mut handling_fatal_signal: sig_atomic_t = 0 as libc::c_int;
pub unsafe extern "C" fn fatal_error_signal(mut sig: libc::c_int) {
    ::std::ptr::write_volatile(
        &mut handling_fatal_signal as *mut sig_atomic_t,
        1 as libc::c_int,
    );
    signal(sig, None);
    temp_stdin_unlink();
    osync_clear();
    jobserver_clear();
    if sig == 15 as libc::c_int {
        let mut c: *mut child = 0 as *mut child;
        c = children;
        while !c.is_null() {
            if (*c).remote() == 0 && (*c).pid > 0 as libc::c_int {
                kill((*c).pid, 15 as libc::c_int);
            }
            c = (*c).next;
        }
    }
    if sig == 15 as libc::c_int || sig == 2 as libc::c_int || sig == 1 as libc::c_int
        || sig == 3 as libc::c_int
    {
        let mut c_0: *mut child = 0 as *mut child;
        c_0 = children;
        while !c_0.is_null() {
            if (*c_0).remote() as libc::c_int != 0 && (*c_0).pid > 0 as libc::c_int {
                remote_kill((*c_0).pid, sig);
            }
            c_0 = (*c_0).next;
        }
        c_0 = children;
        while !c_0.is_null() {
            delete_child_targets(c_0);
            c_0 = (*c_0).next;
        }
        while job_slots_used > 0 as libc::c_int as libc::c_uint {
            reap_children(1 as libc::c_int, 0 as libc::c_int);
        }
    } else {
        while job_slots_used > 0 as libc::c_int as libc::c_uint {
            reap_children(1 as libc::c_int, 1 as libc::c_int);
        }
    }
    remove_intermediates(1 as libc::c_int);
    if sig == 3 as libc::c_int {
        exit(1 as libc::c_int);
    }
    if kill(make_pid(), sig) < 0 as libc::c_int {
        pfatal_with_name(b"kill\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn delete_target(
    mut file: *mut file,
    mut on_behalf_of: *const libc::c_char,
) {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut e: libc::c_int = 0;
    if (*file).precious() as libc::c_int != 0 || (*file).phony() as libc::c_int != 0 {
        return;
    }
    if ar_name((*file).name) != 0 {
        let mut file_date: time_t = if (*file).last_mtime
            == 1 as libc::c_int as libc::c_ulong
        {
            -(1 as libc::c_int) as time_t
        } else {
            (((*file).last_mtime)
                .wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                >> (if 1 as libc::c_int != 0 {
                    30 as libc::c_int
                } else {
                    0 as libc::c_int
                })) as time_t
        };
        if ar_member_date((*file).name) != file_date {
            if !on_behalf_of.is_null() {
                error(
                    0 as *mut floc,
                    (strlen(on_behalf_of)).wrapping_add(strlen((*file).name)),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"*** [%s] Archive member '%s' may be bogus; not deleted\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    on_behalf_of,
                    (*file).name,
                );
            } else {
                error(
                    0 as *mut floc,
                    strlen((*file).name),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"*** Archive member '%s' may be bogus; not deleted\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                );
            }
        }
        return;
    }
    loop {
        e = stat((*file).name, &mut st);
        if !(e == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if e == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        && file_timestamp_cons((*file).name, st.st_mtim.tv_sec, st.st_mtim.tv_nsec)
            != (*file).last_mtime
    {
        if !on_behalf_of.is_null() {
            error(
                0 as *mut floc,
                (strlen(on_behalf_of)).wrapping_add(strlen((*file).name)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"*** [%s] Deleting file '%s'\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                on_behalf_of,
                (*file).name,
            );
        } else {
            error(
                0 as *mut floc,
                strlen((*file).name),
                dcgettext(
                    0 as *const libc::c_char,
                    b"*** Deleting file '%s'\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
        }
        if unlink((*file).name) < 0 as libc::c_int
            && *__errno_location() != 2 as libc::c_int
        {
            perror_with_name(
                b"unlink: \0" as *const u8 as *const libc::c_char,
                (*file).name,
            );
        }
    }
}
pub unsafe extern "C" fn delete_child_targets(mut child: *mut child) {
    let mut d: *mut dep = 0 as *mut dep;
    if (*child).deleted() as libc::c_int != 0 || (*child).pid < 0 as libc::c_int {
        return;
    }
    delete_target((*child).file, 0 as *const libc::c_char);
    d = (*(*child).file).also_make;
    while !d.is_null() {
        delete_target((*d).file, (*(*child).file).name);
        d = (*d).next;
    }
    (*child).set_deleted(1 as libc::c_int as libc::c_uint);
}
pub unsafe extern "C" fn print_commands(mut cmds: *const commands) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    fputs(
        dcgettext(
            0 as *const libc::c_char,
            b"#  recipe to execute\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        stdout,
    );
    if ((*cmds).fileinfo.filenm).is_null() {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b" (built-in):\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b" (from '%s', line %lu):\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*cmds).fileinfo.filenm,
            (*cmds).fileinfo.lineno,
        );
    }
    s = (*cmds).commands;
    while *s as libc::c_int != '\0' as i32 {
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        let mut bs: libc::c_int = 0;
        end = s;
        bs = 0 as libc::c_int;
        while *end as libc::c_int != '\0' as i32 {
            if *end as libc::c_int == '\n' as i32 && bs == 0 {
                break;
            }
            bs = if *end as libc::c_int == '\\' as i32 {
                (bs == 0) as libc::c_int
            } else {
                0 as libc::c_int
            };
            end = end.offset(1);
            end;
        }
        printf(
            b"%c%.*s\n\0" as *const u8 as *const libc::c_char,
            cmd_prefix as libc::c_int,
            end.offset_from(s) as libc::c_long as libc::c_int,
            s,
        );
        s = end
            .offset(
                (*end.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32)
                    as libc::c_int as isize,
            );
    }
}
