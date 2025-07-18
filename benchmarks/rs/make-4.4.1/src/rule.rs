use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut posix_pedantic: libc::c_int;
    fn strcache_add_len(str: *const libc::c_char, len: size_t) -> *const libc::c_char;
    fn dir_file_exists_p(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn find_percent_cached(_: *mut *const libc::c_char) -> *const libc::c_char;
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn lookup_file(name: *const libc::c_char) -> *mut file;
    fn expand_extra_prereqs(extra: *const variable) -> *mut dep;
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
    fn lookup_variable(name: *const libc::c_char, length: size_t) -> *mut variable;
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
pub struct nameseq {
    pub next: *mut nameseq,
    pub name: *const libc::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pspec {
    pub target: *const libc::c_char,
    pub dep: *const libc::c_char,
    pub commands: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub static mut pattern_rules: *mut rule = 0 as *const rule as *mut rule;
pub static mut last_pattern_rule: *mut rule = 0 as *const rule as *mut rule;
pub static mut num_pattern_rules: libc::c_uint = 0;
pub static mut max_pattern_targets: libc::c_uint = 0;
pub static mut max_pattern_deps: libc::c_uint = 0;
pub static mut max_pattern_dep_length: size_t = 0;
pub static mut suffix_file: *mut file = 0 as *const file as *mut file;
static mut maxsuffix: size_t = 0;
pub unsafe extern "C" fn get_rule_defn(mut r: *mut rule) -> *const libc::c_char {
    if ((*r)._defn).is_null() {
        let mut len: size_t = 8 as libc::c_int as size_t;
        let mut k: libc::c_uint = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut sep: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
        let mut dep: *const dep = 0 as *const dep;
        let mut ood: *const dep = 0 as *const dep;
        k = 0 as libc::c_int as libc::c_uint;
        while k < (*r).num as libc::c_uint {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (*((*r).lens).offset(k as isize))
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
                ) as size_t as size_t;
            k = k.wrapping_add(1);
            k;
        }
        dep = (*r).deps;
        while !dep.is_null() {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen(
                        (if !((*dep).name).is_null() {
                            (*dep).name
                        } else {
                            (*(*dep).file).name
                        }),
                    ))
                        .wrapping_add(
                            (if (*dep).wait_here() as libc::c_int != 0 {
                                (::std::mem::size_of::<[libc::c_char; 7]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                0 as libc::c_int as libc::c_ulong
                            }),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
            dep = (*dep).next;
        }
        (*r)._defn = xmalloc(len) as *mut libc::c_char;
        p = (*r)._defn;
        k = 0 as libc::c_int as libc::c_uint;
        while k < (*r).num as libc::c_uint {
            p = mempcpy(
                mempcpy(p as *mut libc::c_void, sep as *const libc::c_void, strlen(sep)),
                *((*r).targets).offset(k as isize) as *const libc::c_void,
                *((*r).lens).offset(k as isize) as libc::c_ulong,
            ) as *mut libc::c_char;
            k = k.wrapping_add(1);
            k;
            sep = b" \0" as *const u8 as *const libc::c_char;
        }
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = ':' as i32 as libc::c_char;
        if (*r).terminal != 0 {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = ':' as i32 as libc::c_char;
        }
        dep = (*r).deps;
        while !dep.is_null() {
            if (*dep).ignore_mtime() as libc::c_int == 0 as libc::c_int {
                if (*dep).wait_here() != 0 {
                    p = mempcpy(
                        p as *mut libc::c_void,
                        b" .WAIT\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                }
                p = mempcpy(
                    mempcpy(
                        p as *mut libc::c_void,
                        b" \0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                    ),
                    (if !((*dep).name).is_null() {
                        (*dep).name
                    } else {
                        (*(*dep).file).name
                    }) as *const libc::c_void,
                    strlen(
                        if !((*dep).name).is_null() {
                            (*dep).name
                        } else {
                            (*(*dep).file).name
                        },
                    ),
                ) as *mut libc::c_char;
            } else if ood.is_null() {
                ood = dep;
            }
            dep = (*dep).next;
        }
        sep = b" | \0" as *const u8 as *const libc::c_char;
        while !ood.is_null() {
            if (*ood).ignore_mtime() != 0 {
                p = mempcpy(
                    p as *mut libc::c_void,
                    sep as *const libc::c_void,
                    strlen(sep),
                ) as *mut libc::c_char;
                if (*ood).wait_here() != 0 {
                    p = mempcpy(
                        p as *mut libc::c_void,
                        b".WAIT \0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                }
                p = mempcpy(
                    p as *mut libc::c_void,
                    (if !((*ood).name).is_null() {
                        (*ood).name
                    } else {
                        (*(*ood).file).name
                    }) as *const libc::c_void,
                    strlen(
                        if !((*ood).name).is_null() {
                            (*ood).name
                        } else {
                            (*(*ood).file).name
                        },
                    ),
                ) as *mut libc::c_char;
            }
            ood = (*ood).next;
            sep = b" \0" as *const u8 as *const libc::c_char;
        }
        *p = '\0' as i32 as libc::c_char;
    }
    return (*r)._defn;
}
pub unsafe extern "C" fn snap_implicit_rules() {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut namelen: size_t = 0 as libc::c_int as size_t;
    let mut rule: *mut rule = 0 as *mut rule;
    let mut dep: *mut dep = 0 as *mut dep;
    let mut prereqs: *mut dep = expand_extra_prereqs(
        lookup_variable(
            b".EXTRA_PREREQS\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ),
    );
    let mut pre_deps: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    max_pattern_dep_length = 0 as libc::c_int as size_t;
    dep = prereqs;
    while !dep.is_null() {
        let mut d: *const libc::c_char = if !((*dep).name).is_null() {
            (*dep).name
        } else {
            (*(*dep).file).name
        };
        let mut l: size_t = strlen(d);
        if (*dep).need_2nd_expansion() != 0 {
            loop {
                d = strchr(d, '%' as i32);
                if d.is_null() {
                    break;
                }
                l = (l as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                d = d.offset(1);
                d;
            }
        }
        if l > max_pattern_dep_length {
            max_pattern_dep_length = l;
        }
        pre_deps = pre_deps.wrapping_add(1);
        pre_deps;
        dep = (*dep).next;
    }
    max_pattern_deps = 0 as libc::c_int as libc::c_uint;
    max_pattern_targets = max_pattern_deps;
    num_pattern_rules = max_pattern_targets;
    rule = pattern_rules;
    while !rule.is_null() {
        let mut ndeps: libc::c_uint = pre_deps;
        let mut lastdep: *mut dep = 0 as *mut dep;
        num_pattern_rules = num_pattern_rules.wrapping_add(1);
        num_pattern_rules;
        if (*rule).num as libc::c_uint > max_pattern_targets {
            max_pattern_targets = (*rule).num as libc::c_uint;
        }
        dep = (*rule).deps;
        while !dep.is_null() {
            let mut dname: *const libc::c_char = if !((*dep).name).is_null() {
                (*dep).name
            } else {
                (*(*dep).file).name
            };
            let mut len: size_t = strlen(dname);
            let mut p: *const libc::c_char = strrchr(dname, '/' as i32);
            let mut p2: *const libc::c_char = if !p.is_null() {
                strchr(p, '%' as i32)
            } else {
                0 as *mut libc::c_char
            };
            ndeps = ndeps.wrapping_add(1);
            ndeps;
            if len > max_pattern_dep_length {
                max_pattern_dep_length = len;
            }
            if ((*dep).next).is_null() {
                lastdep = dep;
            }
            if !p2.is_null() {
                if p == dname {
                    p = p.offset(1);
                    p;
                }
                if p.offset_from(dname) as libc::c_long as size_t > namelen {
                    namelen = p.offset_from(dname) as libc::c_long as size_t;
                    name = xrealloc(
                        name as *mut libc::c_void,
                        namelen.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as *mut libc::c_char;
                }
                memcpy(
                    name as *mut libc::c_void,
                    dname as *const libc::c_void,
                    p.offset_from(dname) as libc::c_long as libc::c_ulong,
                );
                *name
                    .offset(
                        p.offset_from(dname) as libc::c_long as isize,
                    ) = '\0' as i32 as libc::c_char;
                (*dep)
                    .set_changed(
                        (dir_file_exists_p(
                            name,
                            b"\0" as *const u8 as *const libc::c_char,
                        ) == 0) as libc::c_int as libc::c_uint,
                    );
            } else {
                (*dep).set_changed(0 as libc::c_int as libc::c_uint);
            }
            dep = (*dep).next;
        }
        if !prereqs.is_null() {
            if !lastdep.is_null() {
                (*lastdep).next = copy_dep_chain(prereqs);
            } else {
                (*rule).deps = copy_dep_chain(prereqs);
            }
        }
        if ndeps > max_pattern_deps {
            max_pattern_deps = ndeps;
        }
        rule = (*rule).next;
    }
    free(name as *mut libc::c_void);
    free_ns_chain(prereqs as *mut nameseq);
}
unsafe extern "C" fn convert_suffix_rule(
    mut target: *const libc::c_char,
    mut source: *const libc::c_char,
    mut cmds: *mut commands,
) {
    let mut names: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut percents: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut deps: *mut dep = 0 as *mut dep;
    names = xmalloc(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        as *mut *const libc::c_char;
    percents = xmalloc(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        as *mut *const libc::c_char;
    if target.is_null() {
        *names = strcache_add_len(
            b"(%.o)\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
        *percents = (*names).offset(1 as libc::c_int as isize);
    } else {
        let mut len: size_t = strlen(target);
        let mut fresh2 = ::std::vec::from_elem(
            0,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(len)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        );
        let mut p: *mut libc::c_char = fresh2.leak().as_mut_ptr() as *mut libc::c_char;
        *p.offset(0 as libc::c_int as isize) = '%' as i32 as libc::c_char;
        memcpy(
            p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            target as *const libc::c_void,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        *names = strcache_add_len(
            p,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        *percents = *names;
    }
    if source.is_null() {
        deps = 0 as *mut dep;
    } else {
        let mut len_0: size_t = strlen(source);
        let mut fresh3 = ::std::vec::from_elem(
            0,
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(len_0)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
        );
        let mut p_0: *mut libc::c_char = fresh3.leak().as_mut_ptr() as *mut libc::c_char;
        *p_0.offset(0 as libc::c_int as isize) = '%' as i32 as libc::c_char;
        memcpy(
            p_0.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            source as *const libc::c_void,
            len_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        deps = xcalloc(::std::mem::size_of::<dep>() as libc::c_ulong) as *mut dep;
        (*deps)
            .name = strcache_add_len(
            p_0,
            len_0.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    create_pattern_rule(
        names,
        percents,
        1 as libc::c_int as libc::c_ushort,
        0 as libc::c_int,
        deps,
        cmds,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn convert_to_pattern() {
    let mut d: *mut dep = 0 as *mut dep;
    let mut d2: *mut dep = 0 as *mut dep;
    let mut rulename: *mut libc::c_char = 0 as *mut libc::c_char;
    maxsuffix = 0 as libc::c_int as size_t;
    d = (*suffix_file).deps;
    while !d.is_null() {
        let mut l: size_t = strlen(
            if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
        );
        if l > maxsuffix {
            maxsuffix = l;
        }
        d = (*d).next;
    }
    let mut fresh4 = ::std::vec::from_elem(
        0,
        maxsuffix
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    rulename = fresh4.leak().as_mut_ptr() as *mut libc::c_char;
    d = (*suffix_file).deps;
    while !d.is_null() {
        let mut slen: size_t = 0;
        convert_suffix_rule(
            if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
            0 as *const libc::c_char,
            0 as *mut commands,
        );
        if !((*(*d).file).cmds).is_null() {
            convert_suffix_rule(
                b"\0" as *const u8 as *const libc::c_char,
                if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
                (*(*d).file).cmds,
            );
        }
        slen = strlen(
            if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name },
        );
        memcpy(
            rulename as *mut libc::c_void,
            (if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name })
                as *const libc::c_void,
            slen,
        );
        let mut current_block_20: u64;
        d2 = (*suffix_file).deps;
        while !d2.is_null() {
            let mut f: *mut file = 0 as *mut file;
            let mut s2len: size_t = 0;
            s2len = strlen(
                if !((*d2).name).is_null() { (*d2).name } else { (*(*d2).file).name },
            );
            if !(slen == s2len
                && ((if !((*d).name).is_null() { (*d).name } else { (*(*d).file).name })
                    == (if !((*d2).name).is_null() {
                        (*d2).name
                    } else {
                        (*(*d2).file).name
                    })
                    || *(if !((*d).name).is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    }) as libc::c_int
                        == *(if !((*d2).name).is_null() {
                            (*d2).name
                        } else {
                            (*(*d2).file).name
                        }) as libc::c_int
                        && (*(if !((*d).name).is_null() {
                            (*d).name
                        } else {
                            (*(*d).file).name
                        }) as libc::c_int == '\0' as i32
                            || strcmp(
                                (if !((*d).name).is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                })
                                    .offset(1 as libc::c_int as isize),
                                (if !((*d2).name).is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                })
                                    .offset(1 as libc::c_int as isize),
                            ) == 0)))
            {
                memcpy(
                    rulename.offset(slen as isize) as *mut libc::c_void,
                    (if !((*d2).name).is_null() {
                        (*d2).name
                    } else {
                        (*(*d2).file).name
                    }) as *const libc::c_void,
                    s2len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                f = lookup_file(rulename);
                if !(f.is_null() || ((*f).cmds).is_null()) {
                    if !((*f).deps).is_null() {
                        if posix_pedantic != 0 {
                            current_block_20 = 13586036798005543211;
                        } else {
                            error(
                                &mut (*(*f).cmds).fileinfo as *mut floc,
                                0 as libc::c_int as size_t,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"warning: ignoring prerequisites on suffix rule definition\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                            current_block_20 = 5783071609795492627;
                        }
                    } else {
                        current_block_20 = 5783071609795492627;
                    }
                    match current_block_20 {
                        13586036798005543211 => {}
                        _ => {
                            if s2len == 2 as libc::c_int as libc::c_ulong
                                && *rulename.offset(slen as isize) as libc::c_int
                                    == '.' as i32
                                && *rulename
                                    .offset(
                                        slen.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    ) as libc::c_int == 'a' as i32
                            {
                                convert_suffix_rule(
                                    0 as *const libc::c_char,
                                    if !((*d).name).is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    },
                                    (*f).cmds,
                                );
                            }
                            convert_suffix_rule(
                                if !((*d2).name).is_null() {
                                    (*d2).name
                                } else {
                                    (*(*d2).file).name
                                },
                                if !((*d).name).is_null() {
                                    (*d).name
                                } else {
                                    (*(*d).file).name
                                },
                                (*f).cmds,
                            );
                        }
                    }
                }
            }
            d2 = (*d2).next;
        }
        d = (*d).next;
    }
}
unsafe extern "C" fn new_pattern_rule(
    mut rule: *mut rule,
    mut override_0: libc::c_int,
) -> libc::c_int {
    let mut r: *mut rule = 0 as *mut rule;
    let mut lastrule: *mut rule = 0 as *mut rule;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    (*rule).in_use = 0 as libc::c_int as libc::c_char;
    (*rule).terminal = 0 as libc::c_int as libc::c_char;
    (*rule).next = 0 as *mut rule;
    lastrule = 0 as *mut rule;
    r = pattern_rules;
    's_18: while !r.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while i < (*rule).num as libc::c_uint {
            j = 0 as libc::c_int as libc::c_uint;
            while j < (*r).num as libc::c_uint {
                if !(*((*rule).targets).offset(i as isize)
                    == *((*r).targets).offset(j as isize)
                    || **((*rule).targets).offset(i as isize) as libc::c_int
                        == **((*r).targets).offset(j as isize) as libc::c_int
                        && (**((*rule).targets).offset(i as isize) as libc::c_int
                            == '\0' as i32
                            || strcmp(
                                (*((*rule).targets).offset(i as isize))
                                    .offset(1 as libc::c_int as isize),
                                (*((*r).targets).offset(j as isize))
                                    .offset(1 as libc::c_int as isize),
                            ) == 0))
                {
                    break;
                }
                j = j.wrapping_add(1);
                j;
            }
            if j == (*r).num as libc::c_uint {
                let mut d: *mut dep = 0 as *mut dep;
                let mut d2: *mut dep = 0 as *mut dep;
                d = (*rule).deps;
                d2 = (*r).deps;
                while !d.is_null() && !d2.is_null() {
                    if !((if !((*d).name).is_null() {
                        (*d).name
                    } else {
                        (*(*d).file).name
                    })
                        == (if !((*d2).name).is_null() {
                            (*d2).name
                        } else {
                            (*(*d2).file).name
                        })
                        || *(if !((*d).name).is_null() {
                            (*d).name
                        } else {
                            (*(*d).file).name
                        }) as libc::c_int
                            == *(if !((*d2).name).is_null() {
                                (*d2).name
                            } else {
                                (*(*d2).file).name
                            }) as libc::c_int
                            && (*(if !((*d).name).is_null() {
                                (*d).name
                            } else {
                                (*(*d).file).name
                            }) as libc::c_int == '\0' as i32
                                || strcmp(
                                    (if !((*d).name).is_null() {
                                        (*d).name
                                    } else {
                                        (*(*d).file).name
                                    })
                                        .offset(1 as libc::c_int as isize),
                                    (if !((*d2).name).is_null() {
                                        (*d2).name
                                    } else {
                                        (*(*d2).file).name
                                    })
                                        .offset(1 as libc::c_int as isize),
                                ) == 0))
                    {
                        break;
                    }
                    d = (*d).next;
                    d2 = (*d2).next;
                }
                if d.is_null() && d2.is_null() {
                    if override_0 != 0 {
                        freerule(r, lastrule);
                        if pattern_rules.is_null() {
                            pattern_rules = rule;
                        } else {
                            (*last_pattern_rule).next = rule;
                        }
                        last_pattern_rule = rule;
                        break 's_18;
                    } else {
                        freerule(rule, 0 as *mut rule);
                        return 0 as libc::c_int;
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        lastrule = r;
        r = (*r).next;
    }
    if r.is_null() {
        if pattern_rules.is_null() {
            pattern_rules = rule;
        } else {
            (*last_pattern_rule).next = rule;
        }
        last_pattern_rule = rule;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn install_pattern_rule(
    mut p: *mut pspec,
    mut terminal: libc::c_int,
) {
    let mut r: *mut rule = 0 as *mut rule;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    r = xmalloc(::std::mem::size_of::<rule>() as libc::c_ulong) as *mut rule;
    (*r).num = 1 as libc::c_int as libc::c_ushort;
    (*r)
        .targets = xmalloc(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        as *mut *const libc::c_char;
    (*r)
        .suffixes = xmalloc(
        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
    ) as *mut *const libc::c_char;
    (*r)
        .lens = xmalloc(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
        as *mut libc::c_uint;
    (*r)._defn = 0 as *mut libc::c_char;
    *((*r).lens).offset(0 as libc::c_int as isize) = strlen((*p).target) as libc::c_uint;
    let ref mut fresh5 = *((*r).targets).offset(0 as libc::c_int as isize);
    *fresh5 = (*p).target;
    let ref mut fresh6 = *((*r).suffixes).offset(0 as libc::c_int as isize);
    *fresh6 = find_percent_cached(
        &mut *((*r).targets).offset(0 as libc::c_int as isize),
    );
    let ref mut fresh7 = *((*r).suffixes).offset(0 as libc::c_int as isize);
    *fresh7 = (*fresh7).offset(1);
    *fresh7;
    ptr = (*p).dep;
    (*r)
        .deps = parse_file_seq(
        &mut ptr as *mut *const libc::c_char as *mut *mut libc::c_char,
        ::std::mem::size_of::<dep>() as libc::c_ulong,
        0x1 as libc::c_int,
        0 as *const libc::c_char,
        0 as libc::c_int,
    ) as *mut dep;
    if new_pattern_rule(r, 0 as libc::c_int) != 0 {
        (*r)
            .terminal = (if terminal != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
            as libc::c_char;
        (*r)
            .cmds = xmalloc(::std::mem::size_of::<commands>() as libc::c_ulong)
            as *mut commands;
        (*(*r).cmds).fileinfo.filenm = 0 as *const libc::c_char;
        (*(*r).cmds).fileinfo.lineno = 0 as libc::c_int as libc::c_ulong;
        (*(*r).cmds).fileinfo.offset = 0 as libc::c_int as libc::c_ulong;
        (*(*r).cmds).commands = xstrdup((*p).commands);
        (*(*r).cmds).command_lines = 0 as *mut *mut libc::c_char;
        (*(*r).cmds).recipe_prefix = '\t' as i32 as libc::c_char;
    }
}
unsafe extern "C" fn freerule(mut rule: *mut rule, mut lastrule: *mut rule) {
    let mut next: *mut rule = (*rule).next;
    free_ns_chain((*rule).deps as *mut nameseq);
    free((*rule).targets as *mut libc::c_void);
    free((*rule).suffixes as *mut libc::c_void);
    free((*rule).lens as *mut libc::c_void);
    free((*rule)._defn as *mut libc::c_void);
    free(rule as *mut libc::c_void);
    if pattern_rules == rule {
        if !lastrule.is_null() {
            abort();
        } else {
            pattern_rules = next;
        }
    } else if !lastrule.is_null() {
        (*lastrule).next = next;
    }
    if last_pattern_rule == rule {
        last_pattern_rule = lastrule;
    }
}
pub unsafe extern "C" fn create_pattern_rule(
    mut targets: *mut *const libc::c_char,
    mut target_percents: *mut *const libc::c_char,
    mut n: libc::c_ushort,
    mut terminal: libc::c_int,
    mut deps: *mut dep,
    mut commands: *mut commands,
    mut override_0: libc::c_int,
) {
    let mut i: libc::c_uint = 0;
    let mut r: *mut rule = xmalloc(::std::mem::size_of::<rule>() as libc::c_ulong)
        as *mut rule;
    (*r).num = n;
    (*r).cmds = commands;
    (*r).deps = deps;
    (*r).targets = targets;
    (*r).suffixes = target_percents;
    (*r)
        .lens = xmalloc(
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    (*r)._defn = 0 as *mut libc::c_char;
    i = 0 as libc::c_int as libc::c_uint;
    while i < n as libc::c_uint {
        *((*r).lens)
            .offset(i as isize) = strlen(*targets.offset(i as isize)) as libc::c_uint;
        let ref mut fresh8 = *((*r).suffixes).offset(i as isize);
        *fresh8 = (*fresh8).offset(1);
        *fresh8;
        i = i.wrapping_add(1);
        i;
    }
    if new_pattern_rule(r, override_0) != 0 {
        (*r)
            .terminal = (if terminal != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
            as libc::c_char;
    }
}
unsafe extern "C" fn print_rule(mut r: *mut rule) {
    fputs(get_rule_defn(r), stdout);
    putchar('\n' as i32);
    if !((*r).cmds).is_null() {
        print_commands((*r).cmds);
    }
}
pub unsafe extern "C" fn print_rule_data_base() {
    let mut rules: libc::c_uint = 0;
    let mut terminal: libc::c_uint = 0;
    let mut r: *mut rule = 0 as *mut rule;
    puts(
        dcgettext(
            0 as *const libc::c_char,
            b"\n# Implicit Rules\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    terminal = 0 as libc::c_int as libc::c_uint;
    rules = terminal;
    r = pattern_rules;
    while !r.is_null() {
        rules = rules.wrapping_add(1);
        rules;
        putchar('\n' as i32);
        print_rule(r);
        if (*r).terminal != 0 {
            terminal = terminal.wrapping_add(1);
            terminal;
        }
        r = (*r).next;
    }
    if rules == 0 as libc::c_int as libc::c_uint {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"\n# No implicit rules.\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\n# %u implicit rules, %u (%.1f%%) terminal.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            rules,
            terminal,
            terminal as libc::c_double / rules as libc::c_double * 100.0f64,
        );
    }
    if num_pattern_rules != rules {
        if num_pattern_rules != 0 as libc::c_int as libc::c_uint {
            fatal(
                0 as *mut floc,
                (53 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_div(22 as libc::c_int as libc::c_ulong)
                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong),
                dcgettext(
                    0 as *const libc::c_char,
                    b"BUG: num_pattern_rules is wrong!  %u != %u\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                num_pattern_rules,
                rules,
            );
        }
    }
}
