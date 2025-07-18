use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dep;
    pub type commands;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn concat(_: libc::c_uint, _: ...) -> *const libc::c_char;
    fn error(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn perror_with_name(_: *const libc::c_char, _: *const libc::c_char);
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    static mut db_level: libc::c_int;
    fn lookup_file(name: *const libc::c_char) -> *mut file;
    fn do_variable_definition(
        flocp: *const floc,
        name: *const libc::c_char,
        value: *const libc::c_char,
        origin: variable_origin,
        flavor: variable_flavor,
        target_var: libc::c_int,
    ) -> *mut variable;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type load_func_t = Option::<unsafe extern "C" fn(*const floc) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_list {
    pub next: *mut load_list,
    pub name: *const libc::c_char,
    pub dlp: *mut libc::c_void,
}
static mut loaded_syms: *mut load_list = 0 as *const load_list as *mut load_list;
unsafe extern "C" fn load_object(
    mut flocp: *const floc,
    mut noerror: libc::c_int,
    mut ldname: *const libc::c_char,
    mut symname: *const libc::c_char,
) -> load_func_t {
    static mut global_dl: *mut libc::c_void = 0 as *const libc::c_void
        as *mut libc::c_void;
    let mut symp: load_func_t = None;
    if global_dl.is_null() {
        global_dl = dlopen(
            0 as *const libc::c_char,
            0x2 as libc::c_int | 0x100 as libc::c_int,
        );
        if global_dl.is_null() {
            let mut err: *const libc::c_char = dlerror();
            fatal(
                flocp,
                strlen(err),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to open global symbol table: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                err,
            );
        }
    }
    symp = ::std::mem::transmute::<
        *mut libc::c_void,
        load_func_t,
    >(dlsym(global_dl, symname));
    if symp.is_none() {
        let mut new: *mut load_list = 0 as *mut load_list;
        let mut dlp: *mut libc::c_void = 0 as *mut libc::c_void;
        if (strchr(ldname, '/' as i32)).is_null() {
            dlp = dlopen(
                concat(
                    2 as libc::c_int as libc::c_uint,
                    b"./\0" as *const u8 as *const libc::c_char,
                    ldname,
                ),
                0x1 as libc::c_int | 0x100 as libc::c_int,
            );
        }
        if dlp.is_null() {
            dlp = dlopen(ldname, 0x1 as libc::c_int | 0x100 as libc::c_int);
        }
        if dlp.is_null() {
            let mut err_0: *const libc::c_char = dlerror();
            if noerror != 0 {
                if 0x1 as libc::c_int & db_level != 0 {
                    printf(b"%s\n\0" as *const u8 as *const libc::c_char, err_0);
                    fflush(stdout);
                }
            } else {
                error(
                    flocp,
                    strlen(err_0),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    err_0,
                );
            }
            return None;
        }
        if 0x2 as libc::c_int & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Loaded shared object %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ldname,
            );
            fflush(stdout);
        }
        symp = ::std::mem::transmute::<
            *mut libc::c_void,
            load_func_t,
        >(dlsym(dlp, b"plugin_is_GPL_compatible\0" as *const u8 as *const libc::c_char));
        if symp.is_none() {
            fatal(
                flocp,
                strlen(ldname),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Loaded object %s is not declared to be GPL compatible\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ldname,
            );
        }
        symp = ::std::mem::transmute::<
            *mut libc::c_void,
            load_func_t,
        >(dlsym(dlp, symname));
        if symp.is_none() {
            let mut err_1: *const libc::c_char = dlerror();
            fatal(
                flocp,
                (strlen(symname))
                    .wrapping_add(strlen(ldname))
                    .wrapping_add(strlen(err_1)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to load symbol %s from %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                symname,
                ldname,
                err_1,
            );
        }
        new = xmalloc(::std::mem::size_of::<load_list>() as libc::c_ulong)
            as *mut load_list;
        (*new).name = xstrdup(ldname);
        (*new).dlp = dlp;
        (*new).next = loaded_syms;
        loaded_syms = new;
    }
    return symp;
}
pub unsafe extern "C" fn load_file(
    mut flocp: *const floc,
    mut file: *mut file,
    mut noerror: libc::c_int,
) -> libc::c_int {
    let mut ldname: *const libc::c_char = (*file).name;
    let mut nmlen: size_t = strlen(ldname);
    let mut fresh0 = ::std::vec::from_elem(
        0,
        nmlen
            .wrapping_add(
                (::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    let mut new: *mut libc::c_char = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
    let mut symname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    let mut symp: load_func_t = None;
    fp = strchr(ldname, '(' as i32);
    if !fp.is_null() {
        let mut ep: *const libc::c_char = 0 as *const libc::c_char;
        ep = strchr(fp.offset(1 as libc::c_int as isize), ')' as i32);
        if !ep.is_null()
            && *ep.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            let mut l: size_t = fp.offset_from(ldname) as libc::c_long as size_t;
            fp = fp.offset(1);
            fp;
            if fp == ep {
                fatal(
                    flocp,
                    strlen(ldname),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Empty symbol name for load: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    ldname,
                );
            }
            memcpy(new as *mut libc::c_void, ldname as *const libc::c_void, l);
            *new.offset(l as isize) = '\0' as i32 as libc::c_char;
            ldname = new;
            nmlen = l;
            symname = new.offset(l as isize).offset(1 as libc::c_int as isize);
            memcpy(
                symname as *mut libc::c_void,
                fp as *const libc::c_void,
                ep.offset_from(fp) as libc::c_long as libc::c_ulong,
            );
            *symname
                .offset(
                    ep.offset_from(fp) as libc::c_long as isize,
                ) = '\0' as i32 as libc::c_char;
        }
    }
    (*file).name = strcache_add(ldname);
    ldname = (*file).name;
    file = lookup_file(ldname);
    if !file.is_null() && (*file).loaded() as libc::c_int != 0 {
        return -(1 as libc::c_int);
    }
    if symname.is_null() {
        let mut p: *mut libc::c_char = new;
        fp = strrchr(ldname, '/' as i32);
        if fp.is_null() {
            fp = ldname;
        } else {
            fp = fp.offset(1);
            fp;
        }
        while *(*__ctype_b_loc()).offset(*fp as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            != 0 || *fp as libc::c_int == '_' as i32
        {
            let fresh1 = fp;
            fp = fp.offset(1);
            let fresh2 = p;
            p = p.offset(1);
            *fresh2 = *fresh1;
        }
        strcpy(p, b"_gmk_setup\0" as *const u8 as *const libc::c_char);
        symname = new;
    }
    if 0x2 as libc::c_int & db_level != 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Loading symbol %s from %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            symname,
            ldname,
        );
        fflush(stdout);
    }
    symp = load_object(flocp, noerror, ldname, symname);
    if symp.is_none() {
        return 0 as libc::c_int;
    }
    r = (Some(symp.unwrap())).unwrap()(flocp);
    if r != 0 {
        do_variable_definition(
            flocp,
            b".LOADED\0" as *const u8 as *const libc::c_char,
            ldname,
            o_file,
            f_append_value,
            0 as libc::c_int,
        );
    }
    return r;
}
pub unsafe extern "C" fn unload_file(mut name: *const libc::c_char) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut d: *mut load_list = 0 as *mut load_list;
    d = loaded_syms;
    while !d.is_null() {
        if ((*d).name == name
            || *(*d).name as libc::c_int == *name as libc::c_int
                && (*(*d).name as libc::c_int == '\0' as i32
                    || strcmp(
                        ((*d).name).offset(1 as libc::c_int as isize),
                        name.offset(1 as libc::c_int as isize),
                    ) == 0)) && !((*d).dlp).is_null()
        {
            if 0x2 as libc::c_int & db_level != 0 {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unloading shared object %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                fflush(stdout);
            }
            rc = dlclose((*d).dlp);
            if rc != 0 {
                perror_with_name(
                    b"dlclose: \0" as *const u8 as *const libc::c_char,
                    (*d).name,
                );
            } else {
                (*d).dlp = 0 as *mut libc::c_void;
            }
            break;
        } else {
            d = (*d).next;
        }
    }
    return rc;
}
