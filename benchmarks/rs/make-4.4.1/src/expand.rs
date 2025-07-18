use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dep;
    static mut environ: *mut *mut libc::c_char;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xstrndup(_: *const libc::c_char, _: size_t) -> *mut libc::c_char;
    fn lindex(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    fn find_percent(_: *mut libc::c_char) -> *mut libc::c_char;
    static mut reading_file: *const floc;
    static mut stopchar_map: [libc::c_ushort; 0];
    static mut db_level: libc::c_int;
    static mut env_recursion: libc::c_ulonglong;
    static mut current_variable_set_list: *mut variable_set_list;
    fn handle_function(
        op: *mut *mut libc::c_char,
        stringp: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn patsubst_expand_pat(
        o: *mut libc::c_char,
        text: *const libc::c_char,
        pattern: *const libc::c_char,
        replace: *const libc::c_char,
        pattern_percent: *const libc::c_char,
        replace_percent: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn lookup_variable(name: *const libc::c_char, length: size_t) -> *mut variable;
    fn lookup_variable_in_set(
        name: *const libc::c_char,
        length: size_t,
        set: *const variable_set,
    ) -> *mut variable;
    fn warn_undefined(name: *const libc::c_char, length: size_t);
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
pub static mut expanding_var: *mut *const floc = unsafe {
    &reading_file as *const *const floc as *mut *const floc
};
static mut variable_buffer_length: size_t = 0;
pub static mut variable_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn variable_buffer_output(
    mut ptr: *mut libc::c_char,
    mut string: *const libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    let mut newlen: size_t = length
        .wrapping_add(ptr.offset_from(variable_buffer) as libc::c_long as libc::c_ulong);
    if newlen.wrapping_add(5 as libc::c_int as libc::c_ulong) > variable_buffer_length {
        let mut offset: size_t = ptr.offset_from(variable_buffer) as libc::c_long
            as size_t;
        variable_buffer_length = if newlen
            .wrapping_add(100 as libc::c_int as libc::c_ulong)
            > (2 as libc::c_int as libc::c_ulong).wrapping_mul(variable_buffer_length)
        {
            newlen.wrapping_add(100 as libc::c_int as libc::c_ulong)
        } else {
            (2 as libc::c_int as libc::c_ulong).wrapping_mul(variable_buffer_length)
        };
        variable_buffer = xrealloc(
            variable_buffer as *mut libc::c_void,
            variable_buffer_length,
        ) as *mut libc::c_char;
        ptr = variable_buffer.offset(offset as isize);
    }
    return mempcpy(ptr as *mut libc::c_void, string as *const libc::c_void, length)
        as *mut libc::c_char;
}
pub unsafe extern "C" fn initialize_variable_output() -> *mut libc::c_char {
    if variable_buffer.is_null() {
        variable_buffer_length = 200 as libc::c_int as size_t;
        variable_buffer = xmalloc(variable_buffer_length) as *mut libc::c_char;
        *variable_buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    return variable_buffer;
}
pub unsafe extern "C" fn recursively_expand_for_file(
    mut v: *mut variable,
    mut file: *mut file,
) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut this_var: *const floc = 0 as *const floc;
    let mut saved_varp: *mut *const floc = 0 as *mut *const floc;
    let mut save: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut set_reading: libc::c_int = 0 as libc::c_int;
    if (*v).expanding() as libc::c_int != 0 && env_recursion != 0 {
        let mut nl: size_t = strlen((*v).name);
        let mut ep: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        if 0x2 as libc::c_int & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s:%lu: not recursively expanding %s to export to shell function\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*v).fileinfo.filenm,
                (*v).fileinfo.lineno,
                (*v).name,
            );
            fflush(stdout);
        }
        ep = environ;
        while !(*ep).is_null() {
            if *(*ep).offset(nl as isize) as libc::c_int == '=' as i32
                && strncmp(*ep, (*v).name, nl) == 0 as libc::c_int
            {
                return xstrdup(
                    (*ep).offset(nl as isize).offset(1 as libc::c_int as isize),
                );
            }
            ep = ep.offset(1);
            ep;
        }
        return xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    saved_varp = expanding_var;
    if !((*v).fileinfo.filenm).is_null() {
        this_var = &mut (*v).fileinfo;
        expanding_var = &mut this_var;
    }
    if reading_file.is_null() {
        set_reading = 1 as libc::c_int;
        reading_file = &mut (*v).fileinfo;
    }
    if (*v).expanding() != 0 {
        if (*v).exp_count() == 0 {
            fatal(
                *expanding_var,
                strlen((*v).name),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Recursive variable '%s' references itself (eventually)\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*v).name,
            );
        }
        (*v).set_exp_count((*v).exp_count() - 1);
        (*v).exp_count();
    }
    if !file.is_null() {
        save = current_variable_set_list;
        current_variable_set_list = (*file).variables;
    }
    (*v).set_expanding(1 as libc::c_int as libc::c_uint);
    if (*v).append() != 0 {
        value = allocated_variable_append(v);
    } else {
        value = allocated_variable_expand_for_file((*v).value, 0 as *mut file);
    }
    (*v).set_expanding(0 as libc::c_int as libc::c_uint);
    if set_reading != 0 {
        reading_file = 0 as *const floc;
    }
    if !file.is_null() {
        current_variable_set_list = save;
    }
    expanding_var = saved_varp;
    return value;
}
#[inline]
unsafe extern "C" fn reference_variable(
    mut o: *mut libc::c_char,
    mut name: *const libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    let mut v: *mut variable = 0 as *mut variable;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    v = lookup_variable(name, length);
    if v.is_null() {
        warn_undefined(name, length);
    }
    if v.is_null() || *(*v).value as libc::c_int == '\0' as i32 && (*v).append() == 0 {
        return o;
    }
    value = if (*v).recursive() as libc::c_int != 0 {
        recursively_expand_for_file(v, 0 as *mut file)
    } else {
        (*v).value
    };
    o = variable_buffer_output(o, value, strlen(value));
    if (*v).recursive() != 0 {
        free(value as *mut libc::c_void);
    }
    return o;
}
pub unsafe extern "C" fn variable_expand_string(
    mut line: *mut libc::c_char,
    mut string: *const libc::c_char,
    mut length: size_t,
) -> *mut libc::c_char {
    let mut v: *mut variable = 0 as *mut variable;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_offset: size_t = 0;
    if line.is_null() {
        line = initialize_variable_output();
    }
    o = line;
    line_offset = line.offset_from(variable_buffer) as libc::c_long as size_t;
    if length == 0 as libc::c_int as libc::c_ulong {
        variable_buffer_output(
            o,
            b"\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        return variable_buffer;
    }
    save = if length == 18446744073709551615 as libc::c_ulong {
        xstrdup(string)
    } else {
        xstrndup(string, length)
    };
    p = save;
    loop {
        p1 = strchr(p, '$' as i32);
        o = variable_buffer_output(
            o,
            p,
            if !p1.is_null() {
                p1.offset_from(p) as libc::c_long as size_t
            } else {
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            },
        );
        if p1.is_null() {
            break;
        }
        p = p1.offset(1 as libc::c_int as isize);
        match *p as libc::c_int {
            36 | 0 => {
                o = variable_buffer_output(o, p1, 1 as libc::c_int as size_t);
            }
            40 | 123 => {
                let mut openparen: libc::c_char = *p;
                let mut closeparen: libc::c_char = (if openparen as libc::c_int
                    == '(' as i32
                {
                    ')' as i32
                } else {
                    '}' as i32
                }) as libc::c_char;
                let mut begp: *const libc::c_char = 0 as *const libc::c_char;
                let mut beg: *const libc::c_char = p.offset(1 as libc::c_int as isize);
                let mut op: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut abeg: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut end: *const libc::c_char = 0 as *const libc::c_char;
                let mut colon: *const libc::c_char = 0 as *const libc::c_char;
                op = o;
                begp = p;
                if handle_function(&mut op, &mut begp) != 0 {
                    o = op;
                    p = begp;
                } else {
                    end = strchr(beg, closeparen as libc::c_int);
                    if end.is_null() {
                        fatal(
                            *expanding_var,
                            0 as libc::c_int as size_t,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"unterminated variable reference\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                    p1 = lindex(beg, end, '$' as i32);
                    if !p1.is_null() {
                        let mut count: libc::c_int = 0 as libc::c_int;
                        p = beg;
                        while *p as libc::c_int != '\0' as i32 {
                            if *p as libc::c_int == openparen as libc::c_int {
                                count += 1;
                                count;
                            } else if *p as libc::c_int == closeparen as libc::c_int
                                && {
                                    count -= 1;
                                    count < 0 as libc::c_int
                                }
                            {
                                break;
                            }
                            p = p.offset(1);
                            p;
                        }
                        if count < 0 as libc::c_int {
                            abeg = expand_argument(beg, p);
                            beg = abeg;
                            end = strchr(beg, '\0' as i32);
                        }
                    } else {
                        p = end;
                    }
                    colon = lindex(beg, end, ':' as i32);
                    if !colon.is_null() {
                        let mut subst_beg: *const libc::c_char = colon
                            .offset(1 as libc::c_int as isize);
                        let mut subst_end: *const libc::c_char = lindex(
                            subst_beg,
                            end,
                            '=' as i32,
                        );
                        if subst_end.is_null() {
                            colon = 0 as *const libc::c_char;
                        } else {
                            let mut replace_beg: *const libc::c_char = subst_end
                                .offset(1 as libc::c_int as isize);
                            let mut replace_end: *const libc::c_char = end;
                            v = lookup_variable(
                                beg,
                                colon.offset_from(beg) as libc::c_long as size_t,
                            );
                            if v.is_null() {
                                warn_undefined(
                                    beg,
                                    colon.offset_from(beg) as libc::c_long as size_t,
                                );
                            }
                            if !v.is_null() && *(*v).value as libc::c_int != '\0' as i32
                            {
                                let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut replace: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut ppercent: *mut libc::c_char = 0
                                    as *mut libc::c_char;
                                let mut rpercent: *mut libc::c_char = 0
                                    as *mut libc::c_char;
                                let mut value: *mut libc::c_char = if (*v).recursive()
                                    as libc::c_int != 0
                                {
                                    recursively_expand_for_file(v, 0 as *mut file)
                                } else {
                                    (*v).value
                                };
                                let mut fresh0 = ::std::vec::from_elem(
                                    0,
                                    (subst_end.offset_from(subst_beg) as libc::c_long
                                        + 2 as libc::c_int as libc::c_long) as libc::c_ulong
                                        as usize,
                                );
                                pattern = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
                                let fresh1 = pattern;
                                pattern = pattern.offset(1);
                                *fresh1 = '%' as i32 as libc::c_char;
                                memcpy(
                                    pattern as *mut libc::c_void,
                                    subst_beg as *const libc::c_void,
                                    subst_end.offset_from(subst_beg) as libc::c_long
                                        as libc::c_ulong,
                                );
                                *pattern
                                    .offset(
                                        subst_end.offset_from(subst_beg) as libc::c_long as isize,
                                    ) = '\0' as i32 as libc::c_char;
                                let mut fresh2 = ::std::vec::from_elem(
                                    0,
                                    (replace_end.offset_from(replace_beg) as libc::c_long
                                        + 2 as libc::c_int as libc::c_long) as libc::c_ulong
                                        as usize,
                                );
                                replace = fresh2.leak().as_mut_ptr() as *mut libc::c_char;
                                let fresh3 = replace;
                                replace = replace.offset(1);
                                *fresh3 = '%' as i32 as libc::c_char;
                                memcpy(
                                    replace as *mut libc::c_void,
                                    replace_beg as *const libc::c_void,
                                    replace_end.offset_from(replace_beg) as libc::c_long
                                        as libc::c_ulong,
                                );
                                *replace
                                    .offset(
                                        replace_end.offset_from(replace_beg) as libc::c_long
                                            as isize,
                                    ) = '\0' as i32 as libc::c_char;
                                ppercent = find_percent(pattern);
                                if !ppercent.is_null() {
                                    ppercent = ppercent.offset(1);
                                    ppercent;
                                    rpercent = find_percent(replace);
                                    if !rpercent.is_null() {
                                        rpercent = rpercent.offset(1);
                                        rpercent;
                                    }
                                } else {
                                    ppercent = pattern;
                                    rpercent = replace;
                                    pattern = pattern.offset(-1);
                                    pattern;
                                    replace = replace.offset(-1);
                                    replace;
                                }
                                o = patsubst_expand_pat(
                                    o,
                                    value,
                                    pattern,
                                    replace,
                                    ppercent,
                                    rpercent,
                                );
                                if (*v).recursive() != 0 {
                                    free(value as *mut libc::c_void);
                                }
                            }
                        }
                    }
                    if colon.is_null() {
                        o = reference_variable(
                            o,
                            beg,
                            end.offset_from(beg) as libc::c_long as size_t,
                        );
                    }
                    free(abeg as *mut libc::c_void);
                }
            }
            _ => {
                if !(*stopchar_map
                    .as_mut_ptr()
                    .offset(
                        *p.offset(-(1 as libc::c_int) as isize) as libc::c_uchar as isize,
                    ) as libc::c_int & (0x2 as libc::c_int | 0x4 as libc::c_int)
                    != 0 as libc::c_int)
                {
                    o = reference_variable(o, p, 1 as libc::c_int as size_t);
                }
            }
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        p = p.offset(1);
        p;
    }
    free(save as *mut libc::c_void);
    variable_buffer_output(
        o,
        b"\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    return variable_buffer.offset(line_offset as isize);
}
pub unsafe extern "C" fn variable_expand(
    mut line: *const libc::c_char,
) -> *mut libc::c_char {
    return variable_expand_string(
        0 as *mut libc::c_char,
        line,
        18446744073709551615 as libc::c_ulong,
    );
}
pub unsafe extern "C" fn expand_argument(
    mut str: *const libc::c_char,
    mut end: *const libc::c_char,
) -> *mut libc::c_char {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alloc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    if str == end {
        return xstrdup(b"\0" as *const u8 as *const libc::c_char);
    }
    if end.is_null() || *end as libc::c_int == '\0' as i32 {
        return allocated_variable_expand_for_file(str, 0 as *mut file);
    }
    if end.offset_from(str) as libc::c_long + 1 as libc::c_int as libc::c_long
        > 1000 as libc::c_int as libc::c_long
    {
        alloc = xmalloc(
            (end.offset_from(str) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        ) as *mut libc::c_char;
        tmp = alloc;
    } else {
        let mut fresh4 = ::std::vec::from_elem(
            0,
            (end.offset_from(str) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong as usize,
        );
        tmp = fresh4.leak().as_mut_ptr() as *mut libc::c_char;
    }
    memcpy(
        tmp as *mut libc::c_void,
        str as *const libc::c_void,
        end.offset_from(str) as libc::c_long as libc::c_ulong,
    );
    *tmp
        .offset(
            end.offset_from(str) as libc::c_long as isize,
        ) = '\0' as i32 as libc::c_char;
    r = allocated_variable_expand_for_file(tmp, 0 as *mut file);
    free(alloc as *mut libc::c_void);
    return r;
}
pub unsafe extern "C" fn variable_expand_for_file(
    mut line: *const libc::c_char,
    mut file: *mut file,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut savev: *mut variable_set_list = 0 as *mut variable_set_list;
    let mut savef: *const floc = 0 as *const floc;
    if file.is_null() {
        return variable_expand(line);
    }
    savev = current_variable_set_list;
    current_variable_set_list = (*file).variables;
    savef = reading_file;
    if !((*file).cmds).is_null() && !((*(*file).cmds).fileinfo.filenm).is_null() {
        reading_file = &mut (*(*file).cmds).fileinfo;
    } else {
        reading_file = 0 as *const floc;
    }
    result = variable_expand(line);
    current_variable_set_list = savev;
    reading_file = savef;
    return result;
}
unsafe extern "C" fn variable_append(
    mut name: *const libc::c_char,
    mut length: size_t,
    mut set: *const variable_set_list,
    mut local: libc::c_int,
) -> *mut libc::c_char {
    let mut v: *const variable = 0 as *const variable;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nextlocal: libc::c_int = 0;
    if set.is_null() {
        return initialize_variable_output();
    }
    nextlocal = (local != 0 && (*set).next_is_parent == 0 as libc::c_int) as libc::c_int;
    v = lookup_variable_in_set(name, length, (*set).set);
    if v.is_null() || local == 0 && (*v).private_var() as libc::c_int != 0 {
        return variable_append(name, length, (*set).next, nextlocal);
    }
    if (*v).append() != 0 {
        buf = variable_append(name, length, (*set).next, nextlocal);
    } else {
        buf = initialize_variable_output();
    }
    if buf > variable_buffer {
        buf = variable_buffer_output(
            buf,
            b" \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
    }
    if (*v).recursive() == 0 {
        return variable_buffer_output(buf, (*v).value, strlen((*v).value));
    }
    buf = variable_expand_string(buf, (*v).value, strlen((*v).value));
    return buf.offset(strlen(buf) as isize);
}
unsafe extern "C" fn allocated_variable_append(
    mut v: *const variable,
) -> *mut libc::c_char {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obuf: *mut libc::c_char = variable_buffer;
    let mut olen: size_t = variable_buffer_length;
    variable_buffer = 0 as *mut libc::c_char;
    val = variable_append(
        (*v).name,
        strlen((*v).name),
        current_variable_set_list,
        1 as libc::c_int,
    );
    variable_buffer_output(
        val,
        b"\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    val = variable_buffer;
    variable_buffer = obuf;
    variable_buffer_length = olen;
    return val;
}
pub unsafe extern "C" fn allocated_variable_expand_for_file(
    mut line: *const libc::c_char,
    mut file: *mut file,
) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obuf: *mut libc::c_char = variable_buffer;
    let mut olen: size_t = variable_buffer_length;
    variable_buffer = 0 as *mut libc::c_char;
    value = variable_expand_for_file(line, file);
    variable_buffer = obuf;
    variable_buffer_length = olen;
    return value;
}
pub unsafe extern "C" fn install_variable_buffer(
    mut bufp: *mut *mut libc::c_char,
    mut lenp: *mut size_t,
) {
    *bufp = variable_buffer;
    *lenp = variable_buffer_length;
    variable_buffer = 0 as *mut libc::c_char;
    initialize_variable_output();
}
pub unsafe extern "C" fn restore_variable_buffer(
    mut buf: *mut libc::c_char,
    mut len: size_t,
) {
    free(variable_buffer as *mut libc::c_void);
    variable_buffer = buf;
    variable_buffer_length = len;
}
