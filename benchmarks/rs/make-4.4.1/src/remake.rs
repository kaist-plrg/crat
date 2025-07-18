use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut no_intermediates: libc::c_uint;
    fn message(prefix: libc::c_int, length: size_t, fmt: *const libc::c_char, _: ...);
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    static mut command_count: libc::c_ulong;
    fn perror_with_name(_: *const libc::c_char, _: *const libc::c_char);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn find_next_token(_: *mut *const libc::c_char, _: *mut size_t) -> *mut libc::c_char;
    fn print_spaces(_: libc::c_uint);
    fn find_percent(_: *mut libc::c_char) -> *mut libc::c_char;
    fn ar_name(_: *const libc::c_char) -> libc::c_int;
    fn ar_parse_name(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: *mut *mut libc::c_char,
    );
    fn ar_touch(_: *const libc::c_char) -> libc::c_int;
    fn ar_member_date(_: *const libc::c_char) -> time_t;
    fn vpath_search(
        file: *const libc::c_char,
        mtime_ptr: *mut uintmax_t,
        vpath_index: *mut libc::c_uint,
        path_index: *mut libc::c_uint,
    ) -> *const libc::c_char;
    fn gpath_search(file: *const libc::c_char, len: size_t) -> libc::c_int;
    static mut rebuilding_makefiles: libc::c_int;
    static mut clock_skew_detected: libc::c_int;
    static mut second_expansion: libc::c_int;
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    static mut just_print_flag: libc::c_int;
    static mut run_silent: libc::c_int;
    static mut keep_going_flag: libc::c_int;
    static mut question_flag: libc::c_int;
    static mut touch_flag: libc::c_int;
    static mut always_make_flag: libc::c_int;
    static mut check_symlink_flag: libc::c_int;
    static mut default_file: *mut file;
    fn lookup_file(name: *const libc::c_char) -> *mut file;
    fn enter_file(name: *const libc::c_char) -> *mut file;
    fn expand_deps(f: *mut file);
    fn rename_file(file: *mut file, name: *const libc::c_char);
    fn rehash_file(file: *mut file, name: *const libc::c_char);
    fn set_command_state(file: *mut file, state: cmd_state);
    fn try_implicit_rule(file: *mut file, depth: libc::c_uint) -> libc::c_int;
    fn file_timestamp_now(_: *mut libc::c_int) -> uintmax_t;
    fn file_timestamp_cons(
        _: *const libc::c_char,
        _: time_t,
        _: libc::c_long,
    ) -> uintmax_t;
    fn reap_children(block: libc::c_int, err: libc::c_int);
    fn start_waiting_jobs();
    fn execute_file_commands(file: *mut file);
    fn chop_commands(cmds: *mut commands);
    fn free_ns_chain(n: *mut nameseq);
    fn copy_dep_chain(d: *const dep) -> *mut dep;
    static mut variable_buffer: *mut libc::c_char;
    fn variable_buffer_output(
        ptr: *mut libc::c_char,
        string: *const libc::c_char,
        length: size_t,
    ) -> *mut libc::c_char;
    fn variable_expand(line: *const libc::c_char) -> *mut libc::c_char;
    static mut db_level: libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct goaldep {
    pub next: *mut goaldep,
    pub name: *const libc::c_char,
    pub file: *mut file,
    pub shuf: *mut goaldep,
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
    pub c2rust_padding: [u8; 2],
    pub error: libc::c_int,
    pub floc: floc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut commands_started: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut goal_list: *mut goaldep = 0 as *const goaldep as *mut goaldep;
static mut goal_dep: *mut dep = 0 as *const dep as *mut dep;
static mut considered: libc::c_uint = 0 as libc::c_int as libc::c_uint;
unsafe extern "C" fn check_also_make(mut file: *const file) {
    let mut ad: *mut dep = 0 as *mut dep;
    let mut mtime: uintmax_t = (*file).last_mtime;
    if mtime == 0 as libc::c_int as libc::c_ulong {
        mtime = name_mtime((*file).name);
    }
    if mtime >= (2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
        && mtime
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
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && mtime > (*file).mtime_before_update
    {
        ad = (*file).also_make;
        while !ad.is_null() {
            if (*(*ad).file).last_mtime == 1 as libc::c_int as libc::c_ulong {
                error(
                    if !((*file).cmds).is_null() {
                        &mut (*(*file).cmds).fileinfo as *mut floc
                    } else {
                        0 as *mut floc
                    },
                    strlen((*(*ad).file).name),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"warning: pattern recipe did not update peer target '%s'.\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*(*ad).file).name,
                );
            }
            ad = (*ad).next;
        }
    }
}
pub unsafe extern "C" fn update_goal_chain(mut goaldeps: *mut goaldep) -> update_status {
    let mut last_cmd_count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut t: libc::c_int = touch_flag;
    let mut q: libc::c_int = question_flag;
    let mut n: libc::c_int = just_print_flag;
    let mut status: update_status = us_none;
    let mut goals_orig: *mut dep = copy_dep_chain(goaldeps as *mut dep);
    let mut goals: *mut dep = goals_orig;
    goal_list = if rebuilding_makefiles != 0 { goaldeps } else { 0 as *mut goaldep };
    considered = considered.wrapping_add(1);
    considered;
    while !goals.is_null() {
        let mut gu: *mut dep = 0 as *mut dep;
        let mut g: *mut dep = 0 as *mut dep;
        let mut lastgoal: *mut dep = 0 as *mut dep;
        start_waiting_jobs();
        reap_children(
            (last_cmd_count == command_count) as libc::c_int,
            0 as libc::c_int,
        );
        last_cmd_count = command_count;
        lastgoal = 0 as *mut dep;
        gu = goals;
        while !gu.is_null() {
            let mut file: *mut file = 0 as *mut file;
            let mut stop: libc::c_int = 0 as libc::c_int;
            let mut any_not_updated: libc::c_int = 0 as libc::c_int;
            g = if !((*gu).shuf).is_null() { (*gu).shuf } else { gu };
            goal_dep = g;
            file = if !((*(*g).file).double_colon).is_null() {
                (*(*g).file).double_colon
            } else {
                (*g).file
            };
            while !file.is_null() {
                let mut ocommands_started: libc::c_uint = 0;
                let mut fail: update_status = us_success;
                (*file)
                    .set_dontcare(
                        ((*g).flags() as libc::c_int
                            & (1 as libc::c_int) << 2 as libc::c_int != 0 as libc::c_int)
                            as libc::c_int as libc::c_uint,
                    );
                while !((*file).renamed).is_null() {
                    file = (*file).renamed;
                }
                if rebuilding_makefiles != 0 {
                    if (*file).cmd_target() != 0 {
                        touch_flag = t;
                        question_flag = q;
                        just_print_flag = n;
                    } else {
                        just_print_flag = 0 as libc::c_int;
                        question_flag = just_print_flag;
                        touch_flag = question_flag;
                    }
                }
                ocommands_started = commands_started;
                fail = update_file(
                    file,
                    (if rebuilding_makefiles != 0 {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint,
                );
                while !((*file).renamed).is_null() {
                    file = (*file).renamed;
                }
                if commands_started > ocommands_started {
                    (*g).set_changed(1 as libc::c_int as libc::c_uint);
                }
                stop = 0 as libc::c_int;
                if (fail as libc::c_uint != 0 || (*file).updated() as libc::c_int != 0)
                    && (status as libc::c_uint)
                        < us_question as libc::c_int as libc::c_uint
                {
                    if (*file).update_status() as u64 != 0 {
                        status = (*file).update_status();
                        stop = (question_flag != 0 && keep_going_flag == 0
                            && rebuilding_makefiles == 0) as libc::c_int;
                    } else {
                        let mut mtime: uintmax_t = if rebuilding_makefiles != 0 {
                            if (*file).last_mtime == 0 as libc::c_int as libc::c_ulong {
                                f_mtime(file, 0 as libc::c_int)
                            } else {
                                (*file).last_mtime
                            }
                        } else if (*file).last_mtime == 0 as libc::c_int as libc::c_ulong
                        {
                            f_mtime(file, 1 as libc::c_int)
                        } else {
                            (*file).last_mtime
                        };
                        while !((*file).renamed).is_null() {
                            file = (*file).renamed;
                        }
                        if (*file).updated() as libc::c_int != 0
                            && mtime != (*file).mtime_before_update
                        {
                            if rebuilding_makefiles == 0
                                || just_print_flag == 0 && question_flag == 0
                            {
                                status = us_success;
                            }
                            if rebuilding_makefiles != 0
                                && (*file).dontcare() as libc::c_int != 0
                            {
                                stop = 1 as libc::c_int;
                            }
                        }
                    }
                }
                any_not_updated |= ((*file).updated() == 0) as libc::c_int;
                (*file).set_dontcare(0 as libc::c_int as libc::c_uint);
                if stop != 0 {
                    break;
                }
                file = (*file).prev;
            }
            file = (*g).file;
            if stop != 0 || any_not_updated == 0 {
                if rebuilding_makefiles == 0
                    && (*file).update_status() as libc::c_int
                        == us_success as libc::c_int && (*g).changed() == 0
                    && run_silent == 0 && question_flag == 0
                {
                    message(
                        1 as libc::c_int,
                        strlen((*file).name),
                        if (*file).phony() as libc::c_int != 0
                            || ((*file).cmds).is_null()
                        {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Nothing to be done for '%s'.\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        } else {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"'%s' is up to date.\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        },
                        (*file).name,
                    );
                }
                if lastgoal.is_null() {
                    goals = (*gu).next;
                } else {
                    (*lastgoal).next = (*gu).next;
                }
                gu = if lastgoal.is_null() { goals } else { (*lastgoal).next };
                if stop != 0 {
                    break;
                }
            } else {
                lastgoal = gu;
                gu = (*gu).next;
            }
        }
        if gu.is_null() {
            considered = considered.wrapping_add(1);
            considered;
        }
    }
    free_ns_chain(goals_orig as *mut nameseq);
    if rebuilding_makefiles != 0 {
        touch_flag = t;
        question_flag = q;
        just_print_flag = n;
    }
    return status;
}
pub unsafe extern "C" fn show_goal_error() {
    let mut goal: *mut goaldep = 0 as *mut goaldep;
    if (*goal_dep).flags() as libc::c_int
        & ((1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int)
        != (1 as libc::c_int) << 1 as libc::c_int
    {
        return;
    }
    goal = goal_list;
    while !goal.is_null() {
        if (*goal_dep).file == (*goal).file {
            if (*goal).error != 0 {
                error(
                    &mut (*goal).floc as *mut floc,
                    (strlen((*(*goal).file).name))
                        .wrapping_add(strlen(strerror((*goal).error))),
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    (*(*goal).file).name,
                    strerror((*goal).error),
                );
                (*goal).error = 0 as libc::c_int;
            }
            return;
        }
        goal = (*goal).next;
    }
}
unsafe extern "C" fn update_file(
    mut file: *mut file,
    mut depth: libc::c_uint,
) -> update_status {
    let mut status: update_status = us_success;
    let mut f: *mut file = 0 as *mut file;
    f = if !((*file).double_colon).is_null() { (*file).double_colon } else { file };
    if (*f).considered == considered {
        if !((*f).updated() as libc::c_int != 0
            && (*f).update_status() as libc::c_int > us_none as libc::c_int
            && (*f).dontcare() == 0 && (*f).no_diag() as libc::c_int != 0)
        {
            if 0x2 as libc::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Pruning file '%s'.\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            return (if (*f).command_state() as libc::c_int == cs_finished as libc::c_int
            {
                (*f).update_status() as libc::c_int
            } else {
                us_success as libc::c_int
            }) as update_status;
        }
    }
    while !f.is_null() {
        let mut new: update_status = us_success;
        (*f).considered = considered;
        new = update_file_1(f, depth);
        while !((*f).renamed).is_null() {
            f = (*f).renamed;
        }
        if new as libc::c_uint != 0 && keep_going_flag == 0 {
            return new;
        }
        if (*f).command_state() as libc::c_int == cs_running as libc::c_int
            || (*f).command_state() as libc::c_int == cs_deps_running as libc::c_int
        {
            return us_success;
        }
        if new as libc::c_uint > status as libc::c_uint {
            status = new;
        }
        f = (*f).prev;
    }
    return status;
}
unsafe extern "C" fn complain(mut file: *mut file) {
    let mut d: *mut dep = 0 as *mut dep;
    d = (*file).deps;
    while !d.is_null() {
        if (*(*d).file).updated() as libc::c_int != 0
            && (*(*d).file).update_status() as libc::c_int > us_none as libc::c_int
            && (*file).no_diag() as libc::c_int != 0
        {
            complain((*d).file);
            break;
        } else {
            d = (*d).next;
        }
    }
    if d.is_null() {
        show_goal_error();
        if !((*file).parent).is_null() {
            let mut l: size_t = (strlen((*file).name))
                .wrapping_add(strlen((*(*file).parent).name))
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
            let mut m: *const libc::c_char = dcgettext(
                0 as *const libc::c_char,
                b"%sNo rule to make target '%s', needed by '%s'%s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
            if keep_going_flag == 0 {
                fatal(
                    0 as *mut floc,
                    l,
                    m,
                    b"\0" as *const u8 as *const libc::c_char,
                    (*file).name,
                    (*(*file).parent).name,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            error(
                0 as *mut floc,
                l,
                m,
                b"*** \0" as *const u8 as *const libc::c_char,
                (*file).name,
                (*(*file).parent).name,
                b".\0" as *const u8 as *const libc::c_char,
            );
        } else {
            let mut l_0: size_t = (strlen((*file).name))
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
            let mut m_0: *const libc::c_char = dcgettext(
                0 as *const libc::c_char,
                b"%sNo rule to make target '%s'%s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
            if keep_going_flag == 0 {
                fatal(
                    0 as *mut floc,
                    l_0,
                    m_0,
                    b"\0" as *const u8 as *const libc::c_char,
                    (*file).name,
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            error(
                0 as *mut floc,
                l_0,
                m_0,
                b"*** \0" as *const u8 as *const libc::c_char,
                (*file).name,
                b".\0" as *const u8 as *const libc::c_char,
            );
        }
        (*file).set_no_diag(0 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn update_file_1(
    mut file: *mut file,
    mut depth: libc::c_uint,
) -> update_status {
    let mut dep_status: update_status = us_success;
    let mut this_mtime: uintmax_t = 0;
    let mut noexist: libc::c_int = 0;
    let mut must_make: libc::c_int = 0;
    let mut deps_changed: libc::c_int = 0;
    let mut ofile: *mut file = 0 as *mut file;
    let mut du: *mut dep = 0 as *mut dep;
    let mut d: *mut dep = 0 as *mut dep;
    let mut ad: *mut dep = 0 as *mut dep;
    let mut amake: dep = dep {
        next: 0 as *mut dep,
        name: 0 as *const libc::c_char,
        file: 0 as *mut file,
        shuf: 0 as *mut dep,
        stem: 0 as *const libc::c_char,
        flags_changed_ignore_mtime_staticpattern_need_2nd_expansion_ignore_automatic_vars_is_explicit_wait_here: [0; 2],
        c2rust_padding: [0; 6],
    };
    let mut running: libc::c_int = 0 as libc::c_int;
    if 0x2 as libc::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Considering target file '%s'.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file).name,
        );
        fflush(stdout);
    }
    if (*file).updated() != 0 {
        if (*file).update_status() as libc::c_int > us_none as libc::c_int {
            if 0x2 as libc::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Recently tried and failed to update file '%s'.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            if (*file).no_diag() as libc::c_int != 0 && (*file).dontcare() == 0 {
                complain(file);
            }
            return (*file).update_status();
        }
        if 0x2 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"File '%s' was considered already.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        return us_success;
    }
    match (*file).command_state() as libc::c_int {
        0 | 1 => {}
        2 => {
            if 0x2 as libc::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Still updating file '%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            return us_success;
        }
        3 => {
            if 0x2 as libc::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Finished updating file '%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            return (*file).update_status();
        }
        _ => {
            abort();
        }
    }
    (*file).set_no_diag((*file).dontcare());
    let ref mut fresh0 = *if !((*file).double_colon).is_null() {
        (*file).double_colon
    } else {
        file
    };
    (*fresh0).set_updating(1 as libc::c_int as libc::c_uint);
    ofile = file;
    depth = depth.wrapping_add(1);
    depth;
    this_mtime = if (*file).last_mtime == 0 as libc::c_int as libc::c_ulong {
        f_mtime(file, 1 as libc::c_int)
    } else {
        (*file).last_mtime
    };
    while !((*file).renamed).is_null() {
        file = (*file).renamed;
    }
    noexist = (this_mtime == 1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if noexist != 0 {
        if 0x1 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"File '%s' does not exist.\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    } else if this_mtime >= (2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
        && this_mtime
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
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        && (*file).low_resolution_time() as libc::c_int != 0
    {
        let mut ns: libc::c_int = (this_mtime
            .wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            & (((1 as libc::c_int)
                << (if 1 as libc::c_int != 0 {
                    30 as libc::c_int
                } else {
                    0 as libc::c_int
                })) - 1 as libc::c_int) as libc::c_ulong) as libc::c_int;
        if ns != 0 as libc::c_int {
            error(
                0 as *mut floc,
                strlen((*file).name),
                dcgettext(
                    0 as *const libc::c_char,
                    b"*** Warning: .LOW_RESOLUTION_TIME file '%s' has a high resolution time stamp\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
        }
        this_mtime = (this_mtime as libc::c_ulong)
            .wrapping_add(
                ((if 1 as libc::c_int != 0 {
                    1000000000 as libc::c_int
                } else {
                    1 as libc::c_int
                }) - 1 as libc::c_int - ns) as libc::c_ulong,
            ) as uintmax_t as uintmax_t;
    }
    ad = (*file).also_make;
    while !ad.is_null() && noexist == 0 {
        let mut adfile: *mut file = (*ad).file;
        let mut fmtime: uintmax_t = if (*adfile).last_mtime
            == 0 as libc::c_int as libc::c_ulong
        {
            f_mtime(adfile, 1 as libc::c_int)
        } else {
            (*adfile).last_mtime
        };
        noexist = (fmtime == 1 as libc::c_int as libc::c_ulong) as libc::c_int;
        if noexist != 0 {
            while !((*adfile).renamed).is_null() {
                adfile = (*adfile).renamed;
            }
            if 0x1 as libc::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Grouped target peer '%s' of file '%s' does not exist.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*adfile).name,
                    (*file).name,
                );
                fflush(stdout);
            }
        } else if fmtime < this_mtime {
            this_mtime = fmtime;
        }
        ad = (*ad).next;
    }
    must_make = noexist;
    if (*file).phony() == 0 && ((*file).cmds).is_null() && (*file).tried_implicit() == 0
    {
        try_implicit_rule(file, depth);
        (*file).set_tried_implicit(1 as libc::c_int as libc::c_uint);
    }
    if ((*file).cmds).is_null() && (*file).is_target() == 0 && !default_file.is_null()
        && !((*default_file).cmds).is_null()
    {
        if 0x8 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Using default recipe for '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        (*file).cmds = (*default_file).cmds;
    }
    amake.file = file;
    amake.next = (*file).also_make;
    ad = &mut amake;
    while !ad.is_null() {
        let mut lastd: *mut dep = 0 as *mut dep;
        if second_expansion != 0 {
            expand_deps((*ad).file);
        }
        du = (*(*ad).file).deps;
        ad = (*ad).next;
        while !du.is_null() {
            let mut new: update_status = us_success;
            let mut mtime: uintmax_t = 0;
            let mut maybe_make: libc::c_int = 0;
            let mut dontcare: libc::c_int = 0 as libc::c_int;
            d = if !((*du).shuf).is_null() { (*du).shuf } else { du };
            if (*d).wait_here() as libc::c_int != 0 && running != 0 {
                break;
            }
            while !((*(*d).file).renamed).is_null() {
                (*d).file = (*(*d).file).renamed;
            }
            mtime = if (*(*d).file).last_mtime == 0 as libc::c_int as libc::c_ulong {
                f_mtime((*d).file, 1 as libc::c_int)
            } else {
                (*(*d).file).last_mtime
            };
            while !((*(*d).file).renamed).is_null() {
                (*d).file = (*(*d).file).renamed;
            }
            if (*if !((*(*d).file).double_colon).is_null() {
                (*(*d).file).double_colon
            } else {
                (*d).file
            })
                .updating() != 0
            {
                error(
                    0 as *mut floc,
                    (strlen((*file).name)).wrapping_add(strlen((*(*d).file).name)),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Circular %s <- %s dependency dropped.\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                    (*(*d).file).name,
                );
                if lastd.is_null() {
                    (*file).deps = (*du).next;
                } else {
                    (*lastd).next = (*du).next;
                }
                du = (*du).next;
            } else {
                (*(*d).file).parent = file;
                maybe_make = must_make;
                if rebuilding_makefiles != 0 {
                    dontcare = (*(*d).file).dontcare() as libc::c_int;
                    (*(*d).file).set_dontcare((*file).dontcare());
                }
                new = check_dep((*d).file, depth, this_mtime, &mut maybe_make);
                if new as libc::c_uint > dep_status as libc::c_uint {
                    dep_status = new;
                }
                if rebuilding_makefiles != 0 {
                    (*(*d).file).set_dontcare(dontcare as libc::c_uint);
                }
                if (*d).ignore_mtime() == 0 {
                    must_make = maybe_make;
                }
                while !((*(*d).file).renamed).is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                let mut f: *mut file = (*d).file;
                if !((*f).double_colon).is_null() {
                    f = (*f).double_colon;
                }
                loop {
                    running
                        |= ((*f).command_state() as libc::c_int
                            == cs_running as libc::c_int
                            || (*f).command_state() as libc::c_int
                                == cs_deps_running as libc::c_int) as libc::c_int;
                    f = (*f).prev;
                    if f.is_null() {
                        break;
                    }
                }
                if dep_status as libc::c_uint != 0 && keep_going_flag == 0 {
                    break;
                }
                if running == 0 {
                    (*d)
                        .set_changed(
                            ((if (*(*d).file).last_mtime
                                == 0 as libc::c_int as libc::c_ulong
                            {
                                f_mtime((*d).file, 1 as libc::c_int)
                            } else {
                                (*(*d).file).last_mtime
                            }) != mtime || mtime == 1 as libc::c_int as libc::c_ulong)
                                as libc::c_int as libc::c_uint,
                        );
                }
                lastd = du;
                du = (*du).next;
            }
        }
    }
    if must_make != 0 || always_make_flag != 0 {
        du = (*file).deps;
        while !du.is_null() {
            d = if !((*du).shuf).is_null() { (*du).shuf } else { du };
            if (*d).wait_here() as libc::c_int != 0 && running != 0 {
                break;
            }
            if (*(*d).file).intermediate() != 0 {
                let mut new_0: update_status = us_success;
                let mut dontcare_0: libc::c_int = 0 as libc::c_int;
                let mut mtime_0: uintmax_t = if (*(*d).file).last_mtime
                    == 0 as libc::c_int as libc::c_ulong
                {
                    f_mtime((*d).file, 1 as libc::c_int)
                } else {
                    (*(*d).file).last_mtime
                };
                while !((*(*d).file).renamed).is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                (*(*d).file).parent = file;
                if rebuilding_makefiles != 0 {
                    dontcare_0 = (*(*d).file).dontcare() as libc::c_int;
                    (*(*d).file).set_dontcare((*file).dontcare());
                }
                (*(*d).file).considered = 0 as libc::c_int as libc::c_uint;
                new_0 = update_file((*d).file, depth);
                if new_0 as libc::c_uint > dep_status as libc::c_uint {
                    dep_status = new_0;
                }
                if rebuilding_makefiles != 0 {
                    (*(*d).file).set_dontcare(dontcare_0 as libc::c_uint);
                }
                while !((*(*d).file).renamed).is_null() {
                    (*d).file = (*(*d).file).renamed;
                }
                let mut f_0: *mut file = (*d).file;
                if !((*f_0).double_colon).is_null() {
                    f_0 = (*f_0).double_colon;
                }
                loop {
                    running
                        |= ((*f_0).command_state() as libc::c_int
                            == cs_running as libc::c_int
                            || (*f_0).command_state() as libc::c_int
                                == cs_deps_running as libc::c_int) as libc::c_int;
                    f_0 = (*f_0).prev;
                    if f_0.is_null() {
                        break;
                    }
                }
                if dep_status as libc::c_uint != 0 && keep_going_flag == 0 {
                    break;
                }
                if running == 0 {
                    (*d)
                        .set_changed(
                            ((*file).phony() as libc::c_int != 0
                                && !((*file).cmds).is_null()
                                || (if (*(*d).file).last_mtime
                                    == 0 as libc::c_int as libc::c_ulong
                                {
                                    f_mtime((*d).file, 1 as libc::c_int)
                                } else {
                                    (*(*d).file).last_mtime
                                }) != mtime_0) as libc::c_int as libc::c_uint,
                        );
                }
            }
            du = (*du).next;
        }
    }
    let ref mut fresh1 = *if !((*file).double_colon).is_null() {
        (*file).double_colon
    } else {
        file
    };
    (*fresh1).set_updating(0 as libc::c_int as libc::c_uint);
    let ref mut fresh2 = *if !((*ofile).double_colon).is_null() {
        (*ofile).double_colon
    } else {
        ofile
    };
    (*fresh2).set_updating(0 as libc::c_int as libc::c_uint);
    depth = depth.wrapping_sub(1);
    depth;
    if running != 0 {
        set_command_state(file, cs_deps_running);
        if 0x2 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"The prerequisites of '%s' are being made.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        return us_success;
    }
    if 0x2 as libc::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Finished prerequisites of target file '%s'.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file).name,
        );
        fflush(stdout);
    }
    if dep_status as u64 != 0 {
        (*file)
            .set_update_status(
                (if dep_status as libc::c_uint == us_none as libc::c_int as libc::c_uint
                {
                    us_failed as libc::c_int as libc::c_uint
                } else {
                    dep_status as libc::c_uint
                }) as update_status,
            );
        notice_finished_file(file);
        if 0x2 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Giving up on target file '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        if depth == 0 as libc::c_int as libc::c_uint && keep_going_flag != 0
            && just_print_flag == 0 && question_flag == 0
        {
            error(
                0 as *mut floc,
                strlen((*file).name),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Target '%s' not remade because of errors.\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
        }
        return dep_status;
    }
    if (*file).command_state() as libc::c_int == cs_deps_running as libc::c_int {
        set_command_state(file, cs_not_started);
    }
    deps_changed = 0 as libc::c_int;
    d = (*file).deps;
    while !d.is_null() {
        let mut d_mtime: uintmax_t = if (*(*d).file).last_mtime
            == 0 as libc::c_int as libc::c_ulong
        {
            f_mtime((*d).file, 1 as libc::c_int)
        } else {
            (*(*d).file).last_mtime
        };
        while !((*(*d).file).renamed).is_null() {
            (*d).file = (*(*d).file).renamed;
        }
        if (*d).ignore_mtime() == 0 {
            if d_mtime == 1 as libc::c_int as libc::c_ulong
                && (*(*d).file).intermediate() == 0
            {
                must_make = 1 as libc::c_int;
            }
            deps_changed |= (*d).changed() as libc::c_int;
        }
        (*d)
            .set_changed(
                (*d).changed()
                    | (noexist != 0 || d_mtime > this_mtime) as libc::c_int
                        as libc::c_uint,
            );
        if noexist == 0 && (0x1 as libc::c_int | 0x2 as libc::c_int) & db_level != 0 {
            let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
            if (*d).ignore_mtime() != 0 {
                if 0x2 as libc::c_int & db_level != 0 {
                    fmt = dcgettext(
                        0 as *const libc::c_char,
                        b"Prerequisite '%s' is order-only for target '%s'.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                }
            } else if d_mtime == 1 as libc::c_int as libc::c_ulong {
                if 0x1 as libc::c_int & db_level != 0 {
                    fmt = dcgettext(
                        0 as *const libc::c_char,
                        b"Prerequisite '%s' of target '%s' does not exist.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    );
                }
            } else if (*d).changed() != 0 {
                if 0x1 as libc::c_int & db_level != 0 {
                    fmt = dcgettext(
                        0 as *const libc::c_char,
                        b"Prerequisite '%s' is newer than target '%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    );
                }
            } else if 0x2 as libc::c_int & db_level != 0 {
                fmt = dcgettext(
                    0 as *const libc::c_char,
                    b"Prerequisite '%s' is older than target '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
            if !fmt.is_null() {
                print_spaces(depth.wrapping_add(1 as libc::c_int as libc::c_uint));
                printf(
                    fmt,
                    if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        d = (*d).next;
    }
    if !((*file).double_colon).is_null() && ((*file).deps).is_null() {
        must_make = 1 as libc::c_int;
        if 0x1 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Target '%s' is double-colon and has no prerequisites.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    } else if noexist == 0 && (*file).is_target() as libc::c_int != 0
        && deps_changed == 0 && ((*file).cmds).is_null() && always_make_flag == 0
    {
        must_make = 0 as libc::c_int;
        if 0x2 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"No recipe for '%s' and no prerequisites actually changed.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    } else if must_make == 0 && !((*file).cmds).is_null() && always_make_flag != 0 {
        must_make = 1 as libc::c_int;
        if 0x2 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Making '%s' due to always-make flag.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
    }
    if must_make == 0 {
        if 0x2 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"No need to remake target '%s'\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            if !((*file).name == (*file).hname
                || *(*file).name as libc::c_int == *(*file).hname as libc::c_int
                    && (*(*file).name as libc::c_int == '\0' as i32
                        || strcmp(
                            ((*file).name).offset(1 as libc::c_int as isize),
                            ((*file).hname).offset(1 as libc::c_int as isize),
                        ) == 0))
            {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"; using VPATH name '%s'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).hname,
                );
            }
            puts(b".\0" as *const u8 as *const libc::c_char);
            fflush(stdout);
        }
        if (*file).notintermediate() == 0
            && no_intermediates == 0 as libc::c_int as libc::c_uint
        {
            (*file).set_secondary(1 as libc::c_int as libc::c_uint);
        }
        notice_finished_file(file);
        while !file.is_null() {
            (*file).name = (*file).hname;
            file = (*file).prev;
        }
        return us_success;
    }
    if 0x1 as libc::c_int & db_level != 0 {
        print_spaces(depth);
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Must remake target '%s'.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*file).name,
        );
        fflush(stdout);
    }
    if !((*file).name == (*file).hname
        || *(*file).name as libc::c_int == *(*file).hname as libc::c_int
            && (*(*file).name as libc::c_int == '\0' as i32
                || strcmp(
                    ((*file).name).offset(1 as libc::c_int as isize),
                    ((*file).hname).offset(1 as libc::c_int as isize),
                ) == 0))
    {
        if 0x1 as libc::c_int & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"  Ignoring VPATH name '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).hname,
            );
            fflush(stdout);
        }
        (*file).set_ignore_vpath(1 as libc::c_int as libc::c_uint);
    }
    remake_file(file);
    if (*file).command_state() as libc::c_int != cs_finished as libc::c_int {
        if 0x2 as libc::c_int & db_level != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Recipe of '%s' is being run.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*file).name,
            );
            fflush(stdout);
        }
        return us_success;
    }
    match (*file).update_status() as libc::c_int {
        3 => {
            if 0x1 as libc::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to remake target file '%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        0 => {
            if 0x1 as libc::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Successfully remade target file '%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        2 => {
            if 0x1 as libc::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Target file '%s' needs to be remade under -q.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
        }
        1 | _ => {}
    }
    (*file).set_updated(1 as libc::c_int as libc::c_uint);
    return (*file).update_status();
}
pub unsafe extern "C" fn notice_finished_file(mut file: *mut file) {
    let mut d: *mut dep = 0 as *mut dep;
    let mut ran: libc::c_int = ((*file).command_state() as libc::c_int
        == cs_running as libc::c_int) as libc::c_int;
    let mut touched: libc::c_int = 0 as libc::c_int;
    (*file).set_command_state(cs_finished);
    (*file).set_updated(1 as libc::c_int as libc::c_uint);
    if touch_flag != 0
        && (*file).update_status() as libc::c_int == us_success as libc::c_int
    {
        let mut current_block_9: u64;
        if !((*file).cmds).is_null() && (*(*file).cmds).any_recurse() as libc::c_int != 0
        {
            let mut i: libc::c_uint = 0;
            i = 0 as libc::c_int as libc::c_uint;
            loop {
                if !(i < (*(*file).cmds).ncommand_lines as libc::c_uint) {
                    current_block_9 = 3512920355445576850;
                    break;
                }
                if !(*((*(*file).cmds).lines_flags).offset(i as isize) as libc::c_int
                    & 1 as libc::c_int != 0 as libc::c_int)
                {
                    current_block_9 = 5630070708772550801;
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            current_block_9 = 5630070708772550801;
        }
        match current_block_9 {
            5630070708772550801 => {
                if (*file).phony() != 0 {
                    (*file).set_update_status(us_success);
                } else if !((*file).cmds).is_null() {
                    (*file).set_update_status(touch_file(file));
                    commands_started = commands_started.wrapping_add(1);
                    commands_started;
                    touched = 1 as libc::c_int;
                }
            }
            _ => {}
        }
    }
    if (*file).mtime_before_update == 0 as libc::c_int as libc::c_ulong {
        (*file).mtime_before_update = (*file).last_mtime;
    }
    if ran != 0 && (*file).phony() == 0 || touched != 0 {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        if (question_flag != 0 || just_print_flag != 0 || touch_flag != 0)
            && !((*file).cmds).is_null()
        {
            i_0 = (*(*file).cmds).ncommand_lines as libc::c_int;
            while i_0 > 0 as libc::c_int {
                if !(*((*(*file).cmds).lines_flags)
                    .offset((i_0 - 1 as libc::c_int) as isize) as libc::c_int
                    & 1 as libc::c_int != 0 as libc::c_int)
                {
                    break;
                }
                i_0 -= 1;
                i_0;
            }
        } else if (*file).is_target() as libc::c_int != 0 && ((*file).cmds).is_null() {
            i_0 = 1 as libc::c_int;
        }
        (*file)
            .last_mtime = if i_0 == 0 as libc::c_int {
            0 as libc::c_int as libc::c_ulong
        } else {
            (!(0 as libc::c_int as uintmax_t))
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
        };
    }
    if !((*file).double_colon).is_null() {
        let mut f: *mut file = 0 as *mut file;
        let mut max_mtime: uintmax_t = (*file).last_mtime;
        f = (*file).double_colon;
        while !f.is_null() && (*f).updated() as libc::c_int != 0 {
            if max_mtime != 0 as libc::c_int as libc::c_ulong
                && ((*f).last_mtime == 0 as libc::c_int as libc::c_ulong
                    || (*f).last_mtime > max_mtime)
            {
                max_mtime = (*f).last_mtime;
            }
            f = (*f).prev;
        }
        if f.is_null() {
            f = (*file).double_colon;
            while !f.is_null() {
                (*f).last_mtime = max_mtime;
                f = (*f).prev;
            }
        }
    }
    if ran != 0 && (*file).update_status() as libc::c_int != us_none as libc::c_int {
        d = (*file).also_make;
        while !d.is_null() {
            (*(*d).file).set_command_state(cs_finished);
            (*(*d).file).set_updated(1 as libc::c_int as libc::c_uint);
            (*(*d).file).set_update_status((*file).update_status());
            if ran != 0 && (*(*d).file).phony() == 0 {
                f_mtime((*d).file, 0 as libc::c_int);
            }
            d = (*d).next;
        }
        if (*file).tried_implicit() as libc::c_int != 0 && !((*file).also_make).is_null()
        {
            check_also_make(file);
        }
    } else if (*file).update_status() as libc::c_int == us_none as libc::c_int {
        (*file).set_update_status(us_success);
    }
}
unsafe extern "C" fn check_dep(
    mut file: *mut file,
    mut depth: libc::c_uint,
    mut this_mtime: uintmax_t,
    mut must_make_ptr: *mut libc::c_int,
) -> update_status {
    let mut ofile: *mut file = 0 as *mut file;
    let mut d: *mut dep = 0 as *mut dep;
    let mut dep_status: update_status = us_success;
    let ref mut fresh3 = *if !((*file).double_colon).is_null() {
        (*file).double_colon
    } else {
        file
    };
    (*fresh3).set_updating(1 as libc::c_int as libc::c_uint);
    ofile = file;
    if (*file).phony() as libc::c_int != 0 || (*file).intermediate() == 0 {
        let mut mtime: uintmax_t = 0;
        dep_status = update_file(file, depth);
        while !((*file).renamed).is_null() {
            file = (*file).renamed;
        }
        mtime = if (*file).last_mtime == 0 as libc::c_int as libc::c_ulong {
            f_mtime(file, 1 as libc::c_int)
        } else {
            (*file).last_mtime
        };
        while !((*file).renamed).is_null() {
            file = (*file).renamed;
        }
        if mtime == 1 as libc::c_int as libc::c_ulong || mtime > this_mtime {
            *must_make_ptr = 1 as libc::c_int;
        }
    } else {
        let mut mtime_0: uintmax_t = 0;
        if (*file).phony() == 0 && ((*file).cmds).is_null()
            && (*file).tried_implicit() == 0
        {
            try_implicit_rule(file, depth);
            (*file).set_tried_implicit(1 as libc::c_int as libc::c_uint);
        }
        if ((*file).cmds).is_null() && (*file).is_target() == 0
            && !default_file.is_null() && !((*default_file).cmds).is_null()
        {
            if 0x8 as libc::c_int & db_level != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Using default commands for '%s'.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
            (*file).cmds = (*default_file).cmds;
        }
        while !((*file).renamed).is_null() {
            file = (*file).renamed;
        }
        mtime_0 = if (*file).last_mtime == 0 as libc::c_int as libc::c_ulong {
            f_mtime(file, 1 as libc::c_int)
        } else {
            (*file).last_mtime
        };
        while !((*file).renamed).is_null() {
            file = (*file).renamed;
        }
        if mtime_0 != 1 as libc::c_int as libc::c_ulong && mtime_0 > this_mtime {
            *must_make_ptr = 1 as libc::c_int;
        } else {
            let mut ld: *mut dep = 0 as *mut dep;
            let mut deps_running: libc::c_int = 0 as libc::c_int;
            if (*file).command_state() as libc::c_int != cs_running as libc::c_int {
                if (*file).command_state() as libc::c_int
                    == cs_deps_running as libc::c_int
                {
                    (*file).considered = 0 as libc::c_int as libc::c_uint;
                }
                set_command_state(file, cs_not_started);
            }
            ld = 0 as *mut dep;
            if second_expansion != 0 {
                expand_deps(file);
            }
            d = (*file).deps;
            while !d.is_null() {
                let mut new: update_status = us_success;
                let mut maybe_make: libc::c_int = 0;
                if (*if !((*(*d).file).double_colon).is_null() {
                    (*(*d).file).double_colon
                } else {
                    (*d).file
                })
                    .updating() != 0
                {
                    error(
                        0 as *mut floc,
                        (strlen((*file).name)).wrapping_add(strlen((*(*d).file).name)),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Circular %s <- %s dependency dropped.\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*file).name,
                        (*(*d).file).name,
                    );
                    if ld.is_null() {
                        (*file).deps = (*d).next;
                        free(d as *mut libc::c_void);
                        d = (*file).deps;
                    } else {
                        (*ld).next = (*d).next;
                        free(d as *mut libc::c_void);
                        d = (*ld).next;
                    }
                } else {
                    (*(*d).file).parent = file;
                    maybe_make = *must_make_ptr;
                    new = check_dep(
                        (*d).file,
                        depth.wrapping_add(1 as libc::c_int as libc::c_uint),
                        this_mtime,
                        &mut maybe_make,
                    );
                    if new as libc::c_uint > dep_status as libc::c_uint {
                        dep_status = new;
                    }
                    if (*d).ignore_mtime() == 0 {
                        *must_make_ptr = maybe_make;
                    }
                    while !((*(*d).file).renamed).is_null() {
                        (*d).file = (*(*d).file).renamed;
                    }
                    if dep_status as libc::c_uint != 0 && keep_going_flag == 0 {
                        break;
                    }
                    if (*(*d).file).command_state() as libc::c_int
                        == cs_running as libc::c_int
                        || (*(*d).file).command_state() as libc::c_int
                            == cs_deps_running as libc::c_int
                    {
                        deps_running = 1 as libc::c_int;
                    }
                    ld = d;
                    d = (*d).next;
                }
            }
            if deps_running != 0 {
                set_command_state(file, cs_deps_running);
            }
        }
    }
    let ref mut fresh4 = *if !((*file).double_colon).is_null() {
        (*file).double_colon
    } else {
        file
    };
    (*fresh4).set_updating(0 as libc::c_int as libc::c_uint);
    let ref mut fresh5 = *if !((*ofile).double_colon).is_null() {
        (*ofile).double_colon
    } else {
        ofile
    };
    (*fresh5).set_updating(0 as libc::c_int as libc::c_uint);
    return dep_status;
}
unsafe extern "C" fn touch_file(mut file: *mut file) -> update_status {
    if run_silent == 0 {
        message(
            0 as libc::c_int,
            strlen((*file).name),
            b"touch %s\0" as *const u8 as *const libc::c_char,
            (*file).name,
        );
    }
    if just_print_flag != 0 {
        return us_success;
    }
    if ar_name((*file).name) != 0 {
        return (if ar_touch((*file).name) != 0 {
            us_failed as libc::c_int
        } else {
            us_success as libc::c_int
        }) as update_status
    } else {
        let mut fd: libc::c_int = 0;
        loop {
            fd = open(
                (*file).name,
                0o2 as libc::c_int | 0o100 as libc::c_int,
                0o666 as libc::c_int,
            );
            if !(fd == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
                break;
            }
        }
        if fd < 0 as libc::c_int {
            perror_with_name(
                b"touch: open: \0" as *const u8 as *const libc::c_char,
                (*file).name,
            );
            return us_failed;
        } else {
            let mut statbuf: stat = stat {
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
            let mut buf: libc::c_char = 'x' as i32 as libc::c_char;
            let mut e: libc::c_int = 0;
            loop {
                e = fstat(fd, &mut statbuf);
                if !(e == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if e < 0 as libc::c_int {
                perror_with_name(
                    b"touch: fstat: \0" as *const u8 as *const libc::c_char,
                    (*file).name,
                );
                return us_failed;
            }
            loop {
                e = read(
                    fd,
                    &mut buf as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                ) as libc::c_int;
                if !(e == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if e < 0 as libc::c_int {
                perror_with_name(
                    b"touch: read: \0" as *const u8 as *const libc::c_char,
                    (*file).name,
                );
                return us_failed;
            }
            let mut o: off_t = 0;
            loop {
                o = lseek(fd, 0 as libc::c_long, 0 as libc::c_int);
                if !(o == -(1 as libc::c_int) as libc::c_long
                    && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if o < 0 as libc::c_long {
                perror_with_name(
                    b"touch: lseek: \0" as *const u8 as *const libc::c_char,
                    (*file).name,
                );
                return us_failed;
            }
            loop {
                e = write(
                    fd,
                    &mut buf as *mut libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                ) as libc::c_int;
                if !(e == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if e < 0 as libc::c_int {
                perror_with_name(
                    b"touch: write: \0" as *const u8 as *const libc::c_char,
                    (*file).name,
                );
                return us_failed;
            }
            if statbuf.st_size == 0 as libc::c_int as libc::c_long {
                close(fd);
                loop {
                    fd = open(
                        (*file).name,
                        0o2 as libc::c_int | 0o1000 as libc::c_int,
                        0o666 as libc::c_int,
                    );
                    if !(fd == -(1 as libc::c_int)
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        break;
                    }
                }
                if fd < 0 as libc::c_int {
                    perror_with_name(
                        b"touch: open: \0" as *const u8 as *const libc::c_char,
                        (*file).name,
                    );
                    return us_failed;
                }
            }
            close(fd);
        }
    }
    return us_success;
}
unsafe extern "C" fn remake_file(mut file: *mut file) {
    if ((*file).cmds).is_null() {
        if (*file).phony() != 0 {
            (*file).set_update_status(us_success);
        } else if (*file).is_target() != 0 {
            (*file).set_update_status(us_success);
        } else {
            if rebuilding_makefiles == 0 || (*file).dontcare() == 0 {
                complain(file);
            }
            (*file).set_update_status(us_failed);
        }
    } else {
        chop_commands((*file).cmds);
        if touch_flag == 0 || (*(*file).cmds).any_recurse() as libc::c_int != 0 {
            execute_file_commands(file);
            return;
        }
        (*file).set_update_status(us_success);
    }
    notice_finished_file(file);
}
pub unsafe extern "C" fn f_mtime(
    mut file: *mut file,
    mut search: libc::c_int,
) -> uintmax_t {
    let mut mtime: uintmax_t = 0;
    let mut propagate_timestamp: libc::c_uint = 0;
    if ar_name((*file).name) != 0 {
        let mut memmtime: uintmax_t = 0;
        let mut arname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut memname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut arfile: *mut file = 0 as *mut file;
        let mut member_date: time_t = 0;
        ar_parse_name((*file).name, &mut arname, &mut memname);
        memmtime = name_mtime(memname);
        arfile = lookup_file(arname);
        if arfile.is_null() {
            arfile = enter_file(strcache_add(arname));
        }
        mtime = f_mtime(arfile, search);
        while !((*arfile).renamed).is_null() {
            arfile = (*arfile).renamed;
        }
        if search != 0 && strcmp((*arfile).hname, arname) != 0 {
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut arlen: size_t = 0;
            let mut memlen: size_t = 0;
            arlen = strlen((*arfile).hname);
            memlen = strlen(memname);
            let mut fresh6 = ::std::vec::from_elem(
                0,
                arlen
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(memlen)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as usize,
            );
            name = fresh6.leak().as_mut_ptr() as *mut libc::c_char;
            memcpy(
                name as *mut libc::c_void,
                (*arfile).hname as *const libc::c_void,
                arlen,
            );
            *name.offset(arlen as isize) = '(' as i32 as libc::c_char;
            memcpy(
                name.offset(arlen as isize).offset(1 as libc::c_int as isize)
                    as *mut libc::c_void,
                memname as *const libc::c_void,
                memlen,
            );
            *name
                .offset(
                    arlen
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(memlen) as isize,
                ) = ')' as i32 as libc::c_char;
            *name
                .offset(
                    arlen
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        .wrapping_add(memlen)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = '\0' as i32 as libc::c_char;
            if (*arfile).name == (*arfile).hname {
                rename_file(file, strcache_add(name));
            } else {
                rehash_file(file, strcache_add(name));
            }
            while !((*file).renamed).is_null() {
                file = (*file).renamed;
            }
        }
        free(arname as *mut libc::c_void);
        (*file).set_low_resolution_time(1 as libc::c_int as libc::c_uint);
        if mtime == 1 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int as uintmax_t;
        }
        member_date = ar_member_date((*file).hname);
        if member_date == -(1 as libc::c_int) as time_t
            || memmtime != 1 as libc::c_int as libc::c_ulong
                && (memmtime
                    .wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                    >> (if 1 as libc::c_int != 0 {
                        30 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })) as time_t > member_date
        {
            mtime = 1 as libc::c_int as uintmax_t;
        } else {
            mtime = file_timestamp_cons(
                (*file).hname,
                member_date,
                0 as libc::c_int as libc::c_long,
            );
        }
    } else {
        mtime = name_mtime((*file).name);
        if mtime == 1 as libc::c_int as libc::c_ulong && search != 0
            && (*file).ignore_vpath() == 0
        {
            let mut name_0: *const libc::c_char = vpath_search(
                (*file).name,
                &mut mtime,
                0 as *mut libc::c_uint,
                0 as *mut libc::c_uint,
            );
            if !name_0.is_null()
                || *((*file).name).offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
                    && *((*file).name).offset(1 as libc::c_int as isize) as libc::c_int
                        == 'l' as i32
                    && {
                        name_0 = library_search((*file).name, &mut mtime);
                        !name_0.is_null()
                    }
            {
                let mut name_len: size_t = 0;
                if mtime != 0 as libc::c_int as libc::c_ulong {
                    (*file).last_mtime = mtime;
                }
                name_len = (strlen(name_0))
                    .wrapping_sub(strlen((*file).name))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                if gpath_search(name_0, name_len) != 0 {
                    rename_file(file, name_0);
                    while !((*file).renamed).is_null() {
                        file = (*file).renamed;
                    }
                    return if (*file).last_mtime == 0 as libc::c_int as libc::c_ulong {
                        f_mtime(file, 1 as libc::c_int)
                    } else {
                        (*file).last_mtime
                    };
                }
                rehash_file(file, name_0);
                while !((*file).renamed).is_null() {
                    file = (*file).renamed;
                }
                if mtime != 2 as libc::c_int as libc::c_ulong
                    && mtime
                        != (!(0 as libc::c_int as uintmax_t))
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
                {
                    mtime = name_mtime(name_0);
                }
            }
        }
    }
    if clock_skew_detected == 0 && mtime != 1 as libc::c_int as libc::c_ulong
        && mtime
            != (!(0 as libc::c_int as uintmax_t))
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
                ) && (*file).updated() == 0
    {
        static mut adjusted_now: uintmax_t = 0;
        let mut adjusted_mtime: uintmax_t = mtime;
        if adjusted_now < adjusted_mtime {
            let mut resolution: libc::c_int = 0;
            let mut now: uintmax_t = file_timestamp_now(&mut resolution);
            adjusted_now = now
                .wrapping_add((resolution - 1 as libc::c_int) as libc::c_ulong);
            if adjusted_now < adjusted_mtime {
                let mut from_now: libc::c_double = (mtime
                    .wrapping_sub((2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
                    >> (if 1 as libc::c_int != 0 {
                        30 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }))
                    .wrapping_sub(
                        now
                            .wrapping_sub(
                                (2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                            )
                            >> (if 1 as libc::c_int != 0 {
                                30 as libc::c_int
                            } else {
                                0 as libc::c_int
                            }),
                    ) as libc::c_double
                    + ((mtime
                        .wrapping_sub(
                            (2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                        )
                        & (((1 as libc::c_int)
                            << (if 1 as libc::c_int != 0 {
                                30 as libc::c_int
                            } else {
                                0 as libc::c_int
                            })) - 1 as libc::c_int) as libc::c_ulong) as libc::c_int
                        - (now
                            .wrapping_sub(
                                (2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                            )
                            & (((1 as libc::c_int)
                                << (if 1 as libc::c_int != 0 {
                                    30 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                })) - 1 as libc::c_int) as libc::c_ulong) as libc::c_int)
                        as libc::c_double / 1e9f64;
                let mut from_now_string: [libc::c_char; 100] = [0; 100];
                if from_now >= 100.0f64
                    && from_now
                        < (9223372036854775807 as libc::c_long as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong)
                            .wrapping_add(1 as libc::c_ulong) as libc::c_double
                {
                    sprintf(
                        from_now_string.as_mut_ptr(),
                        b"%lu\0" as *const u8 as *const libc::c_char,
                        from_now as libc::c_ulong,
                    );
                } else {
                    sprintf(
                        from_now_string.as_mut_ptr(),
                        b"%.2g\0" as *const u8 as *const libc::c_char,
                        from_now,
                    );
                }
                error(
                    0 as *mut floc,
                    (strlen((*file).name))
                        .wrapping_add(strlen(from_now_string.as_mut_ptr())),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Warning: File '%s' has modification time %s s in the future\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*file).name,
                    from_now_string.as_mut_ptr(),
                );
                clock_skew_detected = 1 as libc::c_int;
            }
        }
    }
    if !((*file).double_colon).is_null() {
        file = (*file).double_colon;
    }
    propagate_timestamp = (*file).updated();
    loop {
        if mtime != 1 as libc::c_int as libc::c_ulong
            && (*file).command_state() as libc::c_int == cs_not_started as libc::c_int
            && (*file).tried_implicit() == 0
            && (*file).intermediate() as libc::c_int != 0
        {
            (*file).set_intermediate(0 as libc::c_int as libc::c_uint);
        }
        if (*file).updated() == propagate_timestamp {
            (*file).last_mtime = mtime;
        }
        file = (*file).prev;
        if file.is_null() {
            break;
        }
    }
    return mtime;
}
unsafe extern "C" fn name_mtime(mut name: *const libc::c_char) -> uintmax_t {
    let mut mtime: uintmax_t = 0;
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
    loop {
        e = stat(name, &mut st);
        if !(e == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if e == 0 as libc::c_int {
        mtime = file_timestamp_cons(name, st.st_mtim.tv_sec, st.st_mtim.tv_nsec);
    } else if *__errno_location() == 2 as libc::c_int
        || *__errno_location() == 20 as libc::c_int
    {
        mtime = 1 as libc::c_int as uintmax_t;
    } else {
        perror_with_name(b"stat: \0" as *const u8 as *const libc::c_char, name);
        return 1 as libc::c_int as uintmax_t;
    }
    if check_symlink_flag != 0 && strlen(name) <= 4096 as libc::c_int as libc::c_ulong {
        let mut lpath: [libc::c_char; 4097] = [0; 4097];
        strcpy(lpath.as_mut_ptr(), name);
        loop {
            let mut ltime: uintmax_t = 0;
            let mut lbuf: [libc::c_char; 4097] = [0; 4097];
            let mut llen: libc::c_long = 0;
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            loop {
                e = lstat(lpath.as_mut_ptr(), &mut st);
                if !(e == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if e != 0 {
                if *__errno_location() != 2 as libc::c_int
                    && *__errno_location() != 20 as libc::c_int
                {
                    perror_with_name(
                        b"lstat: \0" as *const u8 as *const libc::c_char,
                        lpath.as_mut_ptr(),
                    );
                }
                break;
            } else {
                if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o120000 as libc::c_int as libc::c_uint)
                {
                    break;
                }
                ltime = file_timestamp_cons(
                    lpath.as_mut_ptr(),
                    st.st_mtim.tv_sec,
                    st.st_mtim.tv_nsec,
                );
                if ltime > mtime {
                    mtime = ltime;
                }
                loop {
                    llen = readlink(
                        lpath.as_mut_ptr(),
                        lbuf.as_mut_ptr(),
                        (4096 as libc::c_int - 1 as libc::c_int) as size_t,
                    );
                    if !(llen == -(1 as libc::c_int) as libc::c_long
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        break;
                    }
                }
                if llen < 0 as libc::c_int as libc::c_long {
                    perror_with_name(
                        b"readlink: \0" as *const u8 as *const libc::c_char,
                        lpath.as_mut_ptr(),
                    );
                    break;
                } else {
                    lbuf[llen as usize] = '\0' as i32 as libc::c_char;
                    if lbuf[0 as libc::c_int as usize] as libc::c_int == '/' as i32
                        || {
                            p = strrchr(lpath.as_mut_ptr(), '/' as i32);
                            p.is_null()
                        }
                    {
                        strcpy(lpath.as_mut_ptr(), lbuf.as_mut_ptr());
                    } else {
                        if p.offset_from(lpath.as_mut_ptr()) as libc::c_long + llen
                            + 2 as libc::c_int as libc::c_long
                            > 4096 as libc::c_int as libc::c_long
                        {
                            break;
                        }
                        strcpy(p.offset(1 as libc::c_int as isize), lbuf.as_mut_ptr());
                    }
                }
            }
        }
    }
    return mtime;
}
unsafe extern "C" fn library_search(
    mut lib: *const libc::c_char,
    mut mtime_ptr: *mut uintmax_t,
) -> *const libc::c_char {
    static mut dirs: [*const libc::c_char; 4] = [
        b"/lib\0" as *const u8 as *const libc::c_char,
        b"/usr/lib\0" as *const u8 as *const libc::c_char,
        b"/usr/local/lib\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut file: *const libc::c_char = 0 as *const libc::c_char;
    let mut libpatterns: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mtime: uintmax_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p2: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut liblen: size_t = 0;
    let mut best_vpath: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut best_path: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    libpatterns = xstrdup(
        variable_expand(b"$(.LIBPATTERNS)\0" as *const u8 as *const libc::c_char),
    );
    lib = lib.offset(2 as libc::c_int as isize);
    liblen = strlen(lib);
    p2 = libpatterns;
    loop {
        p = find_next_token(&mut p2, &mut len);
        if p.is_null() {
            break;
        }
        static mut buf: *mut libc::c_char = 0 as *const libc::c_char
            as *mut libc::c_char;
        static mut buflen: size_t = 0 as libc::c_int as size_t;
        static mut libdir_maxlen: size_t = 0 as libc::c_int as size_t;
        static mut std_dirs: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut libbuf: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: libc::c_char = *p.offset(len as isize);
        let mut p3: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p4: *mut libc::c_char = 0 as *mut libc::c_char;
        *p.offset(len as isize) = '\0' as i32 as libc::c_char;
        p3 = find_percent(p);
        if p3.is_null() {
            error(
                0 as *mut floc,
                strlen(p),
                dcgettext(
                    0 as *const libc::c_char,
                    b".LIBPATTERNS element '%s' is not a pattern\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                p,
            );
            *p.offset(len as isize) = c;
        } else {
            p4 = variable_buffer_output(
                variable_buffer,
                p,
                p3.offset_from(p) as libc::c_long as size_t,
            );
            p4 = variable_buffer_output(p4, lib, liblen);
            p4 = variable_buffer_output(
                p4,
                p3.offset(1 as libc::c_int as isize),
                len.wrapping_sub(p3.offset_from(p) as libc::c_long as libc::c_ulong),
            );
            *p.offset(len as isize) = c;
            libbuf = variable_buffer;
            mtime = name_mtime(libbuf);
            if mtime != 1 as libc::c_int as libc::c_ulong {
                if !mtime_ptr.is_null() {
                    *mtime_ptr = mtime;
                }
                file = strcache_add(libbuf);
                break;
            } else {
                let mut vpath_index: libc::c_uint = 0;
                let mut path_index: libc::c_uint = 0;
                let mut f: *const libc::c_char = vpath_search(
                    libbuf,
                    if !mtime_ptr.is_null() { &mut mtime } else { 0 as *mut uintmax_t },
                    &mut vpath_index,
                    &mut path_index,
                );
                if !f.is_null() {
                    if file.is_null() || vpath_index < best_vpath
                        || vpath_index == best_vpath && path_index < best_path
                    {
                        file = f;
                        best_vpath = vpath_index;
                        best_path = path_index;
                        if !mtime_ptr.is_null() {
                            *mtime_ptr = mtime;
                        }
                    }
                }
                if buflen == 0 {
                    dp = dirs.as_mut_ptr();
                    while !(*dp).is_null() {
                        let mut l: size_t = strlen(*dp);
                        if l > libdir_maxlen {
                            libdir_maxlen = l;
                        }
                        std_dirs = std_dirs.wrapping_add(1);
                        std_dirs;
                        dp = dp.offset(1);
                        dp;
                    }
                    buflen = strlen(libbuf);
                    buf = xmalloc(
                        libdir_maxlen
                            .wrapping_add(buflen)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                } else if buflen < strlen(libbuf) {
                    buflen = strlen(libbuf);
                    buf = xrealloc(
                        buf as *mut libc::c_void,
                        libdir_maxlen
                            .wrapping_add(buflen)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                }
                let mut vpath_index_0: libc::c_uint = (!(0 as libc::c_int
                    as libc::c_uint))
                    .wrapping_sub(std_dirs);
                dp = dirs.as_mut_ptr();
                while !(*dp).is_null() {
                    sprintf(
                        buf,
                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                        *dp,
                        libbuf,
                    );
                    mtime = name_mtime(buf);
                    if mtime != 1 as libc::c_int as libc::c_ulong {
                        if file.is_null() || vpath_index_0 < best_vpath {
                            file = strcache_add(buf);
                            best_vpath = vpath_index_0;
                            if !mtime_ptr.is_null() {
                                *mtime_ptr = mtime;
                            }
                        }
                    }
                    vpath_index_0 = vpath_index_0.wrapping_add(1);
                    vpath_index_0;
                    dp = dp.offset(1);
                    dp;
                }
            }
        }
    }
    free(libpatterns as *mut libc::c_void);
    return file;
}
